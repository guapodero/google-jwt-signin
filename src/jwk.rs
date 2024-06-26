use crate::algorithm::Algorithm;
use crate::base64_decode;
use crate::error::Error;
use serde_derive::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct JsonWebKeySet {
    keys: Vec<JsonWebKey>,
}

impl JsonWebKeySet {
    pub fn get_key(&self, id: &str) -> Option<JsonWebKey> {
        self.keys.iter().find(|key| key.id == id).cloned()
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct JsonWebKey {
    #[serde(rename = "alg")]
    algorithm: Algorithm,
    #[serde(rename = "kid")]
    id: String,
    n: String,
    e: String,
}

impl JsonWebKey {
    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn verify(&self, body: &[u8], signature: &[u8]) -> Result<(), Error> {
        match self.algorithm {
            Algorithm::RS256 => ring::rsa::PublicKeyComponents {
                n: base64_decode(&self.n)?,
                e: base64_decode(&self.e)?,
            }
            .verify(
                &ring::signature::RSA_PKCS1_2048_8192_SHA256,
                body,
                signature,
            )
            .map_err(Error::from),
            _ => Err(Error::UnsupportedAlgorithm(self.algorithm)),
        }
    }
}
