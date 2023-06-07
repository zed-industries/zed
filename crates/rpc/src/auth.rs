use anyhow::{Context, Result};
use rand::{thread_rng, Rng as _};
use rsa::{PublicKey as _, PublicKeyEncoding, RSAPrivateKey, RSAPublicKey};
use std::convert::TryFrom;

pub struct PublicKey(RSAPublicKey);

pub struct PrivateKey(RSAPrivateKey);

/// Generate a public and private key for asymmetric encryption.
pub fn keypair() -> Result<(PublicKey, PrivateKey)> {
    let mut rng = thread_rng();
    let bits = 1024;
    let private_key = RSAPrivateKey::new(&mut rng, bits)?;
    let public_key = RSAPublicKey::from(&private_key);
    Ok((PublicKey(public_key), PrivateKey(private_key)))
}

/// Generate a random 64-character base64 string.
pub fn random_token() -> String {
    let mut rng = thread_rng();
    let mut token_bytes = [0; 48];
    for byte in token_bytes.iter_mut() {
        *byte = rng.gen();
    }
    base64::encode_config(token_bytes, base64::URL_SAFE)
}

impl PublicKey {
    /// Convert a string to a base64-encoded string that can only be decoded with the corresponding
    /// private key.
    pub fn encrypt_string(&self, string: &str) -> Result<String> {
        let mut rng = thread_rng();
        let bytes = string.as_bytes();
        let encrypted_bytes = self
            .0
            .encrypt(&mut rng, PADDING_SCHEME, bytes)
            .context("failed to encrypt string with public key")?;
        let encrypted_string = base64::encode_config(&encrypted_bytes, base64::URL_SAFE);
        Ok(encrypted_string)
    }
}

impl PrivateKey {
    /// Decrypt a base64-encoded string that was encrypted by the corresponding public key.
    pub fn decrypt_string(&self, encrypted_string: &str) -> Result<String> {
        let encrypted_bytes = base64::decode_config(encrypted_string, base64::URL_SAFE)
            .context("failed to base64-decode encrypted string")?;
        let bytes = self
            .0
            .decrypt(PADDING_SCHEME, &encrypted_bytes)
            .context("failed to decrypt string with private key")?;
        let string = String::from_utf8(bytes).context("decrypted content was not valid utf8")?;
        Ok(string)
    }
}

impl TryFrom<PublicKey> for String {
    type Error = anyhow::Error;
    fn try_from(key: PublicKey) -> Result<Self> {
        let bytes = key.0.to_pkcs1().context("failed to serialize public key")?;
        let string = base64::encode_config(&bytes, base64::URL_SAFE);
        Ok(string)
    }
}

impl TryFrom<String> for PublicKey {
    type Error = anyhow::Error;
    fn try_from(value: String) -> Result<Self> {
        let bytes = base64::decode_config(&value, base64::URL_SAFE)
            .context("failed to base64-decode public key string")?;
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
        let public_string = String::try_from(public).unwrap();
        assert_printable(&public_string);

        // SERVER:
        // * parse the public key
        // * generate a random token.
        // * encrypt the token using the public key.
        let public = PublicKey::try_from(public_string).unwrap();
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

    #[test]
    fn test_tokens_are_always_url_safe() {
        for _ in 0..5 {
            let token = random_token();
            let (public_key, _) = keypair().unwrap();
            let encrypted_token = public_key.encrypt_string(&token).unwrap();
            let public_key_str = String::try_from(public_key).unwrap();

            assert_printable(&token);
            assert_printable(&public_key_str);
            assert_printable(&encrypted_token);
        }
    }

    fn assert_printable(token: &str) {
        for c in token.chars() {
            assert!(
                c.is_ascii_graphic(),
                "token {:?} has non-printable char {}",
                token,
                c
            );
            assert_ne!(c, '/', "token {:?} is not URL-safe", token);
            assert_ne!(c, '&', "token {:?} is not URL-safe", token);
        }
    }
}
