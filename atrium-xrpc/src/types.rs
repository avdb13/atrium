use http::header::{AUTHORIZATION, CONTENT_TYPE};
use http::{HeaderName, Method};
use serde::{de::DeserializeOwned, Serialize};

pub(crate) const NSID_REFRESH_SESSION: &str = "com.atproto.server.refreshSession";

/// Supported token types which can be used for authentication.
pub enum JwtTokenType {
    Bearer,
    DPoP,
}

impl std::fmt::Display for JwtTokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            JwtTokenType::Bearer => "Bearer",
            JwtTokenType::DPoP => "DPoP",
        })
    }
}

/// HTTP headers which can be used in XPRC requests.
pub enum Header {
    ContentType,
    Authorization,
    AtprotoProxy,
    AtprotoAcceptLabelers,
}

impl From<Header> for HeaderName {
    fn from(value: Header) -> Self {
        match value {
            Header::ContentType => CONTENT_TYPE,
            Header::Authorization => AUTHORIZATION,
            Header::AtprotoProxy => HeaderName::from_static("atproto-proxy"),
            Header::AtprotoAcceptLabelers => HeaderName::from_static("atproto-accept-labelers"),
        }
    }
}

/// A request which can be executed with [`XrpcClient::send_xrpc()`](crate::XrpcClient::send_xrpc).
pub struct XrpcRequest<P, I>
where
    I: Serialize,
{
    pub method: Method,
    pub nsid: String,
    pub parameters: Option<P>,
    pub input: Option<InputDataOrBytes<I>>,
    pub encoding: Option<String>,
}

/// A type which can be used as a parameter of [`XrpcRequest`].
///
/// JSON serializable data or raw bytes.
pub enum InputDataOrBytes<T>
where
    T: Serialize,
{
    Data(T),
    Bytes(Vec<u8>),
}

/// A type which can be used as a return value of [`XrpcClient::send_xrpc()`](crate::XrpcClient::send_xrpc).
///
/// JSON deserializable data or raw bytes.
pub enum OutputDataOrBytes<T>
where
    T: DeserializeOwned,
{
    Data(T),
    Bytes(Vec<u8>),
}
