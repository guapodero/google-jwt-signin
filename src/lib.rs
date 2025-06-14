#[cfg(test)]
mod test;

mod algorithm;
mod client;
mod error;
mod header;
mod http_client;
mod jwk;
mod key_provider;
mod token;
mod unverified_token;

#[cfg(feature = "blocking")]
pub use crate::client::Client;
#[cfg(feature = "async")]
pub use crate::client::TokioClient;
pub use crate::token::{IdPayload, RequiredClaims, Token};
pub use error::Error;

fn base64_decode(input: &str) -> Result<Vec<u8>, base64::DecodeError> {
    use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
    URL_SAFE_NO_PAD.decode(input)
}
