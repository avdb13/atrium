use crate::jose::create_signed_jwt;
use crate::jose::jws::RegisteredHeader;
use crate::jose::jwt::{Claims, PublicClaims, RegisteredClaims};
use atrium_common::store::memory::MemoryMapStore;
use atrium_common::store::MapStore;
use atrium_xrpc::http::{Request, Response};
use atrium_xrpc::HttpClient;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use chrono::Utc;
use jose_jwa::{Algorithm, Signing};
use jose_jwk::{crypto, EcCurves, Jwk, Key};
use rand::rngs::SmallRng;
use rand::{RngCore, SeedableRng};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::sync::Arc;
use thiserror::Error;

const JWT_HEADER_TYP_DPOP: &str = "dpop+jwt";

#[derive(Deserialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("crypto error: {0:?}")]
    JwkCrypto(crypto::Error),
    #[error("key does not match any alg supported by the server")]
    UnsupportedKey,
    #[error("nonce store error: {0}")]
    Nonces(Box<dyn std::error::Error + Send + Sync + 'static>),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
}

type Result<T> = core::result::Result<T, Error>;

pub struct DpopClient<T, S = MemoryMapStore<String, String>>
where
    S: MapStore<String, String>,
{
    inner: Arc<T>,
    pub(crate) key: Key,
    nonces: S,
}

impl<T> DpopClient<T> {
    pub fn new(
        key: Key,
        http_client: Arc<T>,
        supported_algs: &Option<Vec<String>>,
    ) -> Result<Self> {
        if let Some(algs) = supported_algs {
            let alg = String::from(match &key {
                Key::Ec(ec) => match &ec.crv {
                    EcCurves::P256 => "ES256",
                    _ => unimplemented!(),
                },
                _ => unimplemented!(),
            });
            if !algs.contains(&alg) {
                return Err(Error::UnsupportedKey);
            }
        }
        let nonces = MemoryMapStore::<String, String>::default();
        Ok(Self { inner: http_client, key, iss, nonces })
    }
}

impl<T, S> DpopClient<T, S>
where
    S: MapStore<String, String>,
{
    fn build_proof(
        &self,
        htm: String,
        htu: String,
        ath: Option<String>,
        nonce: Option<String>,
    ) -> Result<String> {
        match crypto::Key::try_from(&self.key).map_err(Error::JwkCrypto)? {
            crypto::Key::P256(crypto::Kind::Secret(secret_key)) => {
                let mut header = RegisteredHeader::from(Algorithm::Signing(Signing::Es256));
                header.typ = Some(JWT_HEADER_TYP_DPOP.into());
                header.jwk = Some(Jwk {
                    key: Key::from(&crypto::Key::from(secret_key.public_key())),
                    prm: Default::default(),
                });
                let claims = Claims {
                    registered: RegisteredClaims {
                        jti: Some(Self::generate_jti()),
                        iat: Some(Utc::now().timestamp()),
                        ..Default::default()
                    },
                    public: PublicClaims { htm: Some(htm), htu: Some(htu), ath, nonce },
                };
                Ok(create_signed_jwt(secret_key.into(), header.into(), claims)?)
            }
            _ => unimplemented!(),
        }
    }
    fn is_use_dpop_nonce_error(&self, response: &Response<Vec<u8>>, is_auth_server: bool) -> bool {
        // https://datatracker.ietf.org/doc/html/rfc9449#name-authorization-server-provid
        if is_auth_server && response.status() == 400 {
            if let Ok(res) = serde_json::from_slice::<ErrorResponse>(response.body()) {
                return res.error == "use_dpop_nonce";
            };
        }
        // https://datatracker.ietf.org/doc/html/rfc9449#name-resource-server-provided-no
        if !is_auth_server && response.status() == 401 {
            // https://datatracker.ietf.org/doc/html/rfc6750#section-3
            if let Some(www_auth) =
                response.headers().get("WWW-Authenticate").and_then(|v| v.to_str().ok())
            {
                return www_auth.starts_with("DPoP")
                    && www_auth.contains(r#"error="use_dpop_nonce""#);
            }
        }
        false
    }
    // https://datatracker.ietf.org/doc/html/rfc9449#section-4.2
    fn generate_jti() -> String {
        let mut rng = SmallRng::from_entropy();
        let mut bytes = [0u8; 12];
        rng.fill_bytes(&mut bytes);
        URL_SAFE_NO_PAD.encode(bytes)
    }
}

impl<T, S> HttpClient for DpopClient<T, S>
where
    T: HttpClient + Send + Sync + 'static,
    S: MapStore<String, String> + Send + Sync + 'static,
    S::Error: Send + Sync + 'static,
{
    async fn send_http(
        &self,
        mut request: Request<Vec<u8>>,
    ) -> core::result::Result<Response<Vec<u8>>, Box<dyn std::error::Error + Send + Sync + 'static>>
    {
        let uri = request.uri();
        let nonce_key = uri.authority().unwrap().to_string();
        let htm = request.method().to_string();
        let htu = uri.to_string();

        let is_auth_server = uri.path().starts_with("/oauth");
        let ath = match request.headers().get("Authorization").and_then(|v| v.to_str().ok()) {
            Some(s) if s.starts_with("DPoP ") => {
                Some(URL_SAFE_NO_PAD.encode(Sha256::digest(s.strip_prefix("DPoP ").unwrap())))
            }
            _ => None,
        };

        let init_nonce =
            self.nonces.get(&nonce_key).await.map_err(|e| Error::Nonces(Box::new(e)))?;
        let init_proof =
            self.build_proof(htm.clone(), htu.clone(), ath.clone(), init_nonce.clone())?;
        request.headers_mut().insert("DPoP", init_proof.parse()?);
        let response = self.inner.send_http(request.clone()).await?;

        let next_nonce =
            response.headers().get("DPoP-Nonce").and_then(|v| v.to_str().ok()).map(String::from);
        match &next_nonce {
            Some(s) if next_nonce != init_nonce => {
                // Store the fresh nonce for future requests
                self.nonces
                    .set(nonce_key, s.clone())
                    .await
                    .map_err(|e| Error::Nonces(Box::new(e)))?;
            }
            _ => {
                // No nonce was returned or it is the same as the one we sent. No need to
                // update the nonce store, or retry the request.
                return Ok(response);
            }
        }

        if !self.is_use_dpop_nonce_error(&response, is_auth_server) {
            return Ok(response);
        }
        let next_proof = self.build_proof(htm, htu, ath, next_nonce)?;
        request.headers_mut().insert("DPoP", next_proof.parse()?);
        let response = self.inner.send_http(request).await?;
        Ok(response)
    }
}
