use base64::{decode, encode};
use rsa::{
    pkcs1::{DecodeRsaPublicKey, EncodeRsaPublicKey},
    RsaPublicKey,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SerializablePublicKey {
    key_data: Vec<u8>, // Raw DER-encoded bytes
}

impl SerializablePublicKey {
    /// Convert an `RsaPublicKey` into a serializable struct.
    pub fn from_rsa_key(public_key: &RsaPublicKey) -> Self {
        let der_encoded = public_key.to_pkcs1_der().unwrap().as_ref().to_vec();
        Self {
            key_data: der_encoded,
        }
    }

    /// Convert back to an `RsaPublicKey`.
    pub fn to_rsa_key(&self) -> RsaPublicKey {
        RsaPublicKey::from_pkcs1_der(&self.key_data).expect("Failed to parse public key")
    }
}
