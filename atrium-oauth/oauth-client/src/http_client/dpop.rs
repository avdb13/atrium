use crate::jose::create_signed_jwt;
use crate::jose::jws::RegisteredHeader;
use crate::jose::jwt::{Claims, PublicClaims, RegisteredClaims};
use atrium_common::store::memory::MemorySimpleStore;
use atrium_common::store::SimpleStore;
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
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
}

type Result<T> = core::result::Result<T, Error>;

pub struct DpopClient<T, S = MemorySimpleStore<String, String>>
where
    S: SimpleStore<String, String>,
{
    inner: Arc<T>,
    key: Key,
    #[allow(dead_code)]
    iss: String,
    nonces: S,
}

impl<T> DpopClient<T> {
    pub fn new(
        key: Key,
        iss: String,
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
        let nonces = MemorySimpleStore::<String, String>::default();
        Ok(Self { inner: http_client, key, iss, nonces })
    }
    fn build_proof(&self, htm: String, htu: String, nonce: Option<String>) -> Result<String> {
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
                    public: PublicClaims {
                        htm: Some(htm),
                        htu: Some(htu),
                        nonce,
                        ..Default::default()
                    },
                };
                Ok(create_signed_jwt(secret_key.into(), header.into(), claims)?)
            }
            _ => unimplemented!(),
        }
    }
    fn is_use_dpop_nonce_error(&self, response: &Response<Vec<u8>>) -> bool {
        // is auth server?
        if response.status() == 400 {
            if let Ok(res) = serde_json::from_slice::<ErrorResponse>(response.body()) {
                return res.error == "use_dpop_nonce";
            };
        }
        // is resource server?

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

impl<T> HttpClient for DpopClient<T>
where
    T: HttpClient + Send + Sync + 'static,
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

        let init_nonce = self.nonces.get(&nonce_key).await?;
        let init_proof = self.build_proof(htm.clone(), htu.clone(), init_nonce.clone())?;
        request.headers_mut().insert("DPoP", init_proof.parse()?);
        let response = self.inner.send_http(request.clone()).await?;

        let next_nonce =
            response.headers().get("DPoP-Nonce").and_then(|v| v.to_str().ok()).map(String::from);
        match &next_nonce {
            Some(s) if next_nonce != init_nonce => {
                // Store the fresh nonce for future requests
                self.nonces.set(nonce_key, s.clone()).await?;
            }
            _ => {
                // No nonce was returned or it is the same as the one we sent. No need to
                // update the nonce store, or retry the request.
                return Ok(response);
            }
        }

        if !self.is_use_dpop_nonce_error(&response) {
            return Ok(response);
        }
        let next_proof = self.build_proof(htm, htu, next_nonce)?;
        request.headers_mut().insert("DPoP", next_proof.parse()?);
        let response = self.inner.send_http(request).await?;
        Ok(response)
    }
}
