use std::convert::{TryFrom, TryInto};

use anyhow::{Context, Result};
use rand::{rngs::OsRng, Rng as _};
use rsa::{PublicKey as _, PublicKeyEncoding, RSAPrivateKey, RSAPublicKey};

pub struct PublicKey(RSAPublicKey);

pub struct PrivateKey(RSAPrivateKey);

/// Generate a public and private key for asymmetric encryption.
pub fn keypair() -> Result<(PublicKey, PrivateKey)> {
    let mut rng = OsRng;
    let bits = 1024;
    let private_key = RSAPrivateKey::new(&mut rng, bits)?;
    let public_key = RSAPublicKey::from(&private_key);
    Ok((PublicKey(public_key), PrivateKey(private_key)))
}

/// Generate a random 64-character base64 string.
pub fn random_token() -> String {
    let mut rng = OsRng;
    let mut token_bytes = [0; 48];
    for byte in token_bytes.iter_mut() {
        *byte = rng.gen();
    }
    base64::encode(&token_bytes)
}

impl PublicKey {
    /// Convert a string to a base64-encoded string that can only be decoded with the corresponding
    /// private key.
    pub fn encrypt_string(&self, string: &str) -> Result<String> {
        let mut rng = OsRng;
        let bytes = string.as_bytes();
        let encrypted_bytes = self
            .0
            .encrypt(&mut rng, PADDING_SCHEME, bytes)
            .context("failed to encrypt string with public key")?;
        let encrypted_string = base64::encode(&encrypted_bytes);
        Ok(encrypted_string)
    }
}

impl PrivateKey {
    /// Decrypt a base64-encoded string that was encrypted by the correspoding public key.
    pub fn decrypt_string(&self, encrypted_string: &str) -> Result<String> {
        let encrypted_bytes =
            base64::decode(encrypted_string).context("failed to base64-decode encrypted string")?;
        let bytes = self
            .0
            .decrypt(PADDING_SCHEME, &encrypted_bytes)
            .context("failed to decrypt string with private key")?;
        let string = String::from_utf8(bytes).context("decrypted content was not valid utf8")?;
        Ok(string)
    }
}

impl TryInto<String> for PublicKey {
    type Error = anyhow::Error;
    fn try_into(self) -> Result<String> {
        let bytes = self
            .0
            .to_pkcs1()
            .context("failed to serialize public key")?;
        let string = base64::encode(&bytes);
        Ok(string)
    }
}

impl TryFrom<String> for PublicKey {
    type Error = anyhow::Error;
    fn try_from(value: String) -> Result<Self> {
        let bytes = base64::decode(&value).context("failed to base64-decode public key string")?;
        let key = Self(RSAPublicKey::from_pkcs1(&bytes).context("failed to parse public key")?);
        Ok(key)
    }
}

const PADDING_SCHEME: rsa::PaddingScheme = rsa::PaddingScheme::PKCS1v15Encrypt;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_encrypt_and_decrypt_token() {
        // CLIENT:
        // * generate a keypair for asymmetric encryption
        // * serialize the public key to send it to the server.
        let (public, private) = keypair().unwrap();
        let public_string: String = public.try_into().unwrap();

        // SERVER:
        // * parse the public key
        // * generate a random token.
        // * encrypt the token using the public key.
        let public: PublicKey = public_string.try_into().unwrap();
        let token = random_token();
        let encrypted_token = public.encrypt_string(&token).unwrap();
        assert_eq!(token.len(), 64);
        assert_ne!(encrypted_token, token);
        assert_printable(&token);
        assert_printable(&encrypted_token);

        // CLIENT:
        // * decrypt the token using the private key.
        let decrypted_token = private.decrypt_string(&encrypted_token).unwrap();
        assert_eq!(decrypted_token, token);
    }

    fn assert_printable(token: &str) {
        for c in token.chars() {
            assert!(
                c.is_ascii_graphic(),
                "token {:?} has non-printable char {}",
                token,
                c
            );
        }
    }
}
