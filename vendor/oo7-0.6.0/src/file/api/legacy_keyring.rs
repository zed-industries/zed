//! Legacy GNOME Keyring file format low level API.

use std::{
    collections::HashMap,
    io::{self, Cursor, Read},
};

use endi::{Endian, ReadBytes};

use super::{Secret, UnlockedItem};
use crate::{
    AsAttributes, crypto,
    file::{Error, WeakKeyError},
};

const FILE_HEADER: &[u8] = b"GnomeKeyring\n\r\0\n";
const FILE_HEADER_LEN: usize = FILE_HEADER.len();

pub const MAJOR_VERSION: u8 = 0;
pub const MINOR_VERSION: u8 = 0;

#[derive(Debug)]
pub struct Keyring {
    salt: Vec<u8>,
    iteration_count: u32,
    encrypted_content: Vec<u8>,
    item_count: usize,
}

impl Keyring {
    pub fn decrypt_items(self, secret: &Secret) -> Result<Vec<UnlockedItem>, Error> {
        let (key, iv) = crypto::legacy_derive_key_and_iv(
            &**secret,
            self.key_strength(secret),
            &self.salt,
            self.iteration_count.try_into().unwrap(),
        )?;
        let decrypted = crypto::decrypt_no_padding(&self.encrypted_content, &key, iv)?;
        let (digest, content) = decrypted.split_at(16);
        if !crypto::verify_checksum_md5(digest, content) {
            return Err(Error::ChecksumMismatch);
        }
        self.read_items(content)
    }

    fn read_attributes<'a>(
        cursor: &mut Cursor<&'a [u8]>,
        count: usize,
    ) -> Result<impl AsAttributes + 'a, Error> {
        let mut result = HashMap::new();
        for _ in 0..count {
            let name = Self::read_string(cursor)?.ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "empty attribute name")
            })?;
            let value = match cursor.read_u32(Endian::Big)? {
                0 => Self::read_string(cursor)?
                    .ok_or_else(|| {
                        io::Error::new(io::ErrorKind::InvalidInput, "empty attribute value")
                    })?
                    .to_string(),
                1 => cursor.read_u32(Endian::Big)?.to_string(),
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "unknown attribute type",
                    )
                    .into());
                }
            };
            result.insert(name, value);
        }
        Ok(result)
    }

    fn read_items(self, decrypted: &[u8]) -> Result<Vec<UnlockedItem>, Error> {
        let mut cursor = Cursor::new(decrypted);
        let mut items = Vec::with_capacity(self.item_count);
        for _ in 0..self.item_count {
            let display_name = Self::read_string(&mut cursor)?
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "empty item label"))?;
            let secret = Self::read_byte_array(&mut cursor)?
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "empty item secret"))?;
            let _created_time = Self::read_time(&mut cursor)?;
            let _modified_time = Self::read_time(&mut cursor)?;
            let _reserved = Self::read_string(&mut cursor)?;
            for _ in 0..4 {
                let _ = cursor.read_u32(Endian::Big)?;
            }
            let attribute_count = cursor.read_u32(Endian::Big)? as usize;
            let attributes = Self::read_attributes(&mut cursor, attribute_count)?;
            items.push(UnlockedItem::new(display_name, &attributes, secret));
            let acl_count = cursor.read_u32(Endian::Big)? as usize;
            Self::skip_acls(&mut cursor, acl_count)?;
        }
        Ok(items)
    }

    fn key_strength(&self, _secret: &[u8]) -> Result<(), WeakKeyError> {
        Ok(())
    }

    fn read_byte_array<'a>(cursor: &mut Cursor<&'a [u8]>) -> Result<Option<&'a [u8]>, Error> {
        let len = cursor.read_u32(Endian::Big)? as usize;
        if len == 0xffffffff {
            Ok(None)
        } else if len >= 0x7fffffff {
            Err(io::Error::new(io::ErrorKind::OutOfMemory, "").into())
        } else if len > cursor.get_ref().len() {
            Err(Error::NoData)
        } else {
            let pos = cursor.position() as usize;
            let bytes = &cursor.get_ref()[pos..pos + len];
            cursor.set_position((pos + len) as u64);
            Ok(Some(bytes))
        }
    }

    fn read_string<'a>(cursor: &mut Cursor<&'a [u8]>) -> Result<Option<&'a str>, Error> {
        match Self::read_byte_array(cursor) {
            Ok(Some(bytes)) => Ok(Some(std::str::from_utf8(bytes)?)),
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }

    fn read_time(cursor: &mut Cursor<&[u8]>) -> Result<u64, Error> {
        let hi = cursor.read_u32(Endian::Big)? as u64;
        let lo = cursor.read_u32(Endian::Big)? as u64;
        Ok((hi << 32) | lo)
    }

    fn skip_hashed_items(cursor: &mut Cursor<&[u8]>, count: usize) -> Result<(), Error> {
        for _ in 0..count {
            let _id = cursor.read_u32(Endian::Big)?;
            let _type = cursor.read_u32(Endian::Big)?;
            let num_attributes = cursor.read_u32(Endian::Big)?;
            for _ in 0..num_attributes {
                let _name = Self::read_string(cursor)?;
                match cursor.read_u32(Endian::Big)? {
                    0 => {
                        let _value = Self::read_string(cursor);
                    }
                    1 => {
                        let _value = cursor.read_u32(Endian::Big);
                    }
                    _ => {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidInput,
                            "unknown attribute type",
                        )
                        .into());
                    }
                }
            }
        }
        Ok(())
    }

    fn skip_acls(cursor: &mut Cursor<&[u8]>, count: usize) -> Result<(), Error> {
        for _ in 0..count {
            let _flags = cursor.read_u32(Endian::Big)?;
            let _display_name = Self::read_string(cursor)?;
            let _path = Self::read_string(cursor)?;
            let _reserved0 = Self::read_string(cursor)?;
            let _reserved1 = cursor.read_u32(Endian::Big)?;
        }
        Ok(())
    }

    fn parse(data: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(data);
        let crypto = cursor.read_u8(Endian::Big)?;
        if crypto != 0 {
            return Err(Error::AlgorithmMismatch(crypto));
        }
        let hash = cursor.read_u8(Endian::Big)?;
        if hash != 0 {
            return Err(Error::AlgorithmMismatch(hash));
        }
        let _display_name = Self::read_string(&mut cursor)?;
        let _created_time = Self::read_time(&mut cursor)?;
        let _modified_time = Self::read_time(&mut cursor)?;
        let _flags = cursor.read_u32(Endian::Big)?;
        let _lock_timeout = cursor.read_u32(Endian::Big)?;
        let iteration_count = cursor.read_u32(Endian::Big)?;
        let mut salt = vec![0; 8];
        cursor.read_exact(salt.as_mut_slice())?;
        for _ in 0..4 {
            let _ = cursor.read_u32(Endian::Big)?;
        }
        let item_count = cursor.read_u32(Endian::Big)? as usize;
        Self::skip_hashed_items(&mut cursor, item_count)?;
        let mut size = cursor.read_u32(Endian::Big)? as usize;
        let pos = cursor.position() as usize;
        if size > cursor.get_ref()[pos..].len() {
            return Err(Error::NoData);
        }
        if !size.is_multiple_of(16) {
            size = (size / 16) * 16;
        }
        let encrypted_content = Vec::from(&cursor.get_ref()[pos..pos + size]);

        Ok(Self {
            salt,
            iteration_count,
            encrypted_content,
            item_count,
        })
    }
}

impl TryFrom<&[u8]> for Keyring {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Error> {
        let header = value.get(..FILE_HEADER.len());
        if header != Some(FILE_HEADER) {
            return Err(Error::FileHeaderMismatch(
                header.map(|x| String::from_utf8_lossy(x).to_string()),
            ));
        }

        let version = value.get(FILE_HEADER_LEN..(FILE_HEADER_LEN + 2));
        if version != Some(&[MAJOR_VERSION, MINOR_VERSION]) {
            return Err(Error::VersionMismatch(version.map(|x| x.to_vec())));
        }

        if let Some(data) = value.get((FILE_HEADER_LEN + 2)..) {
            Self::parse(data)
        } else {
            Err(Error::NoData)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn legacy_decrypt() -> Result<(), Error> {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("fixtures")
            .join("legacy.keyring");
        let blob = std::fs::read(path)?;
        let keyring = Keyring::try_from(blob.as_slice())?;
        let secret = Secret::blob("test");
        let items = keyring.decrypt_items(&secret)?;

        assert_eq!(items.len(), 1);
        assert_eq!(items[0].label(), "foo");
        assert_eq!(items[0].secret(), Secret::blob("foo"));
        let attributes = items[0].attributes();
        assert_eq!(attributes.len(), 2); // also content-type
        assert_eq!(
            attributes
                .get(crate::XDG_SCHEMA_ATTRIBUTE)
                .map(|v| v.as_ref()),
            Some("org.gnome.keyring.Note")
        );

        Ok(())
    }
}
