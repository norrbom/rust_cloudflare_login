use reqwest::blocking::Client;
use reqwest::StatusCode;
use reqwest::Error as ReqwestError;
use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    /// Public keys in JWK format, the current key used to sign all new tokens, and the previous key that has been rotated out
    pub keys: Vec<Key>,
    #[serde(rename = "public_cert")]
    /// The current public key current key in PEM format
    pub public_cert: PublicCert,
    #[serde(rename = "public_certs")]
    /// Public keys in PEM format, the current key used to sign all new tokens, and the previous key that has been rotated out
    pub public_certs: Vec<PublicCerts>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Key {
    pub kid: String,
    pub kty: String,
    pub alg: String,
    #[serde(rename = "use")]
    pub use_field: String,
    pub e: String,
    pub n: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicCert {
    pub kid: String,
    pub cert: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicCerts {
    pub kid: String,
    pub cert: String,
}

pub fn keys(url: &str) -> Result<Root, reqwest::Error> {
    let client = Client::new();
    let response = client.get(url).send()?;

    match response.status() {
        StatusCode::OK => {
            let json_response: Root = response.json()?;
            Ok(json_response)
        }
        _ => Err(ReqwestError::from(response.error_for_status().unwrap_err())),
    }
}
