use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::{crypto, file};

/// A key.
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct Key {
    key: Vec<u8>,
    #[zeroize(skip)]
    strength: Result<(), file::WeakKeyError>,
}

impl std::fmt::Debug for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Key {{ key: [REDACTED], strength: {:?} }}",
            self.strength
        )
    }
}

impl AsRef<[u8]> for Key {
    fn as_ref(&self) -> &[u8] {
        self.key.as_slice()
    }
}

impl AsMut<[u8]> for Key {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.key
    }
}

impl Key {
    pub const fn new(key: Vec<u8>) -> Self {
        Self::new_with_strength(key, Err(file::WeakKeyError::StrengthUnknown))
    }

    pub(crate) const fn check_strength(&self) -> Result<(), file::WeakKeyError> {
        self.strength
    }

    pub(crate) const fn new_with_strength(
        key: Vec<u8>,
        strength: Result<(), file::WeakKeyError>,
    ) -> Self {
        Self { key, strength }
    }

    pub fn generate_private_key() -> Result<Self, crypto::Error> {
        Ok(Self::new(crypto::generate_private_key()?.to_vec()))
    }

    pub fn generate_public_key(private_key: &Self) -> Result<Self, crypto::Error> {
        Ok(Self::new(crypto::generate_public_key(private_key)?))
    }

    pub fn generate_aes_key(
        private_key: &Self,
        server_public_key: &Self,
    ) -> Result<Self, crypto::Error> {
        Ok(Self::new(
            crypto::generate_aes_key(private_key, server_public_key)?.to_vec(),
        ))
    }
}

impl From<Key> for zvariant::Value<'static> {
    fn from(key: Key) -> Self {
        let mut key = key;
        let inner: Vec<u8> = std::mem::take(&mut key.key);
        zvariant::Array::from(inner).into()
    }
}

impl From<Key> for zvariant::OwnedValue {
    fn from(key: Key) -> Self {
        zvariant::Value::from(key).try_into_owned().unwrap()
    }
}

impl TryFrom<zvariant::Value<'_>> for Key {
    type Error = zvariant::Error;

    fn try_from(value: zvariant::Value<'_>) -> Result<Self, Self::Error> {
        Ok(Key::new(value.try_into()?))
    }
}

impl TryFrom<zvariant::OwnedValue> for Key {
    type Error = zvariant::Error;

    fn try_from(value: zvariant::OwnedValue) -> Result<Self, Self::Error> {
        Self::try_from(zvariant::Value::from(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn private_public_pair() {
        let private_key = Key::new(vec![
            41, 20, 63, 236, 246, 132, 109, 70, 172, 121, 45, 66, 129, 21, 247, 91, 96, 217, 56,
            201, 205, 56, 17, 178, 202, 81, 71, 104, 233, 89, 87, 32, 88, 146, 107, 224, 56, 103,
            111, 74, 143, 80, 170, 40, 5, 52, 48, 90, 75, 71, 193, 224, 222, 57, 91, 81, 66, 1, 6,
            88, 137, 66, 102, 207, 55, 95, 67, 92, 140, 227, 242, 153, 185, 195, 89, 236, 146, 242,
            88, 215, 1, 7, 135, 254, 85, 165, 236, 110, 22, 79, 107, 254, 149, 164, 243, 94, 129,
            198, 45, 208, 132, 166, 0, 153, 243, 160, 255, 188, 59, 216, 99, 221, 85, 162, 116,
            210, 160, 117, 201, 39, 179, 123, 107, 8, 242, 139, 207, 250,
        ]);
        let server_public_key = Key::new(vec![
            50, 233, 76, 88, 47, 206, 235, 107, 9, 232, 98, 14, 188, 214, 209, 77, 35, 66, 109,
            119, 24, 191, 120, 90, 242, 198, 240, 115, 200, 66, 51, 180, 8, 164, 89, 9, 229, 31,
            160, 31, 156, 101, 169, 60, 63, 247, 37, 255, 75, 198, 62, 235, 50, 29, 221, 245, 29,
            248, 140, 209, 62, 215, 2, 137, 82, 77, 248, 242, 56, 176, 118, 183, 124, 74, 26, 133,
            188, 47, 31, 141, 232, 194, 92, 18, 69, 3, 56, 153, 42, 9, 143, 81, 197, 159, 200, 197,
            221, 74, 186, 157, 158, 36, 74, 125, 11, 234, 33, 2, 5, 36, 206, 248, 155, 157, 145,
            159, 238, 19, 185, 194, 134, 3, 195, 198, 60, 100, 159, 31,
        ]);

        let expected_public_key = &[
            9, 192, 210, 81, 212, 191, 74, 119, 22, 172, 81, 142, 124, 89, 17, 71, 118, 190, 81,
            71, 49, 149, 200, 204, 14, 47, 111, 165, 119, 103, 216, 102, 111, 93, 242, 64, 73, 224,
            165, 11, 127, 219, 197, 188, 168, 222, 254, 10, 104, 81, 8, 206, 237, 119, 225, 100,
            78, 196, 89, 163, 63, 169, 77, 236, 80, 241, 189, 49, 27, 40, 243, 229, 66, 53, 80, 86,
            44, 213, 87, 186, 68, 55, 216, 56, 236, 51, 229, 44, 174, 18, 87, 141, 85, 71, 185,
            203, 208, 144, 190, 117, 141, 255, 153, 106, 123, 28, 152, 200, 237, 189, 176, 20, 80,
            211, 33, 158, 232, 194, 145, 45, 194, 35, 108, 106, 214, 221, 159, 137,
        ];
        let expected_aes_key = &[
            132, 3, 113, 222, 81, 209, 49, 43, 81, 232, 243, 46, 1, 103, 184, 42,
        ];

        let public_key = Key::generate_public_key(&private_key);
        let aes_key = Key::generate_aes_key(&private_key, &server_public_key);

        assert_eq!(public_key.unwrap().as_ref(), expected_public_key);
        assert_eq!(aes_key.unwrap().as_ref(), expected_aes_key);
    }

    #[test]
    fn key_debug_is_redacted() {
        let key = Key::new(vec![1, 2, 3, 4]);
        let debug_output = format!("{:?}", key);

        assert!(debug_output.contains("key: [REDACTED]"));
        assert!(debug_output.contains("strength:"));
    }
}
