use std::{
    ffi::{CString, OsStr},
    os::unix::ffi::OsStrExt,
    path::Path,
};

use serde::{Deserialize, Serialize};
use zbus::zvariant::Type;

/// A file name represented as a nul-terminated byte array.
#[derive(Type, Debug, Default, PartialEq)]
#[zvariant(signature = "ay")]
pub struct FilePath(CString);

impl AsRef<Path> for FilePath {
    fn as_ref(&self) -> &Path {
        OsStr::from_bytes(self.0.as_bytes()).as_ref()
    }
}

impl FilePath {
    pub(crate) fn new<T: AsRef<Path>>(s: T) -> Result<Self, crate::Error> {
        let c_string = CString::new(s.as_ref().as_os_str().as_bytes())
            .map_err(|err| crate::Error::NulTerminated(err.nul_position()))?;

        Ok(Self(c_string))
    }
}

impl Serialize for FilePath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(self.0.as_bytes_with_nul())
    }
}

impl<'de> Deserialize<'de> for FilePath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bytes = <Vec<u8>>::deserialize(deserializer)?;
        let c_string = CString::from_vec_with_nul(bytes)
            .map_err(|_| serde::de::Error::custom("Bytes are not nul-terminated"))?;

        Ok(Self(c_string))
    }
}

#[cfg(test)]
mod tests {
    use zbus::zvariant::{
        Endian,
        serialized::{Context, Data},
        to_bytes,
    };

    use super::*;

    #[test]
    fn test_serialize_is_nul_terminated() {
        let bytes = vec![97, 98, 99, 0]; // b"abc\0"

        assert_eq!(b"abc\0", bytes.as_slice());

        let c_string = CString::from_vec_with_nul(bytes.clone()).unwrap();

        assert_eq!(c_string.as_bytes_with_nul(), &bytes);

        let ctxt = Context::new_dbus(Endian::Little, 0);
        let file_path = FilePath(c_string);

        let file_path_2 = FilePath::new("abc").unwrap();

        let encoded_filename = to_bytes(ctxt, &file_path).unwrap().to_vec();
        let encoded_filename_2 = to_bytes(ctxt, &file_path_2).unwrap().to_vec();
        let encoded_bytes = to_bytes(ctxt, &bytes).unwrap().to_vec();

        // It does not matter whether we use new("abc") or deserialize from b"abc\0".
        assert_eq!(encoded_filename, encoded_bytes);
        assert_eq!(encoded_filename_2, encoded_bytes);

        let decoded: FilePath = Data::new(encoded_bytes, ctxt).deserialize().unwrap().0;
        assert_eq!(decoded, file_path);
        assert_eq!(decoded, file_path_2);
    }
}
