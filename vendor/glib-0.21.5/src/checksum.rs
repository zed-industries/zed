// Take a look at the license at the top of the repository in the LICENSE file.

use libc::size_t;

use crate::{ffi, translate::*, Checksum};

impl Checksum {
    #[doc(alias = "g_checksum_get_digest")]
    #[doc(alias = "get_digest")]
    pub fn digest(self) -> Vec<u8> {
        unsafe {
            //Don't forget update when `ChecksumType` contains type bigger that Sha512.
            let mut digest_len: size_t = 512 / 8;
            let mut vec = Vec::with_capacity(digest_len as _);

            ffi::g_checksum_get_digest(
                mut_override(self.to_glib_none().0),
                vec.as_mut_ptr(),
                &mut digest_len,
            );

            vec.set_len(digest_len);
            vec
        }
    }

    #[doc(alias = "g_checksum_get_string")]
    #[doc(alias = "get_string")]
    pub fn string(self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_checksum_get_string(mut_override(
                self.to_glib_none().0,
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Checksum, ChecksumType};

    const CS_TYPE: ChecksumType = ChecksumType::Md5;
    const CS_VALUE: &str = "fc3ff98e8c6a0d3087d515c0473f8677";
    const CS_SLICE: &[u8] = &[
        0xfc, 0x3f, 0xf9, 0x8e, 0x8c, 0x6a, 0x0d, 0x30, 0x87, 0xd5, 0x15, 0xc0, 0x47, 0x3f, 0x86,
        0x77,
    ];

    #[test]
    fn update() {
        let mut cs = Checksum::new(CS_TYPE).unwrap();
        cs.update(b"hello world!");
        assert_eq!(cs.string().unwrap(), CS_VALUE);
    }

    #[test]
    fn update_multi_call() {
        let mut cs = Checksum::new(CS_TYPE).unwrap();
        cs.update(b"hello ");
        cs.update(b"world!");
        assert_eq!(cs.string().unwrap(), CS_VALUE);
    }

    #[test]
    #[doc(alias = "get_digest")]
    fn digest() {
        let mut cs = Checksum::new(CS_TYPE).unwrap();
        cs.update(b"hello world!");
        let vec = cs.digest();
        assert_eq!(vec, CS_SLICE);
    }
}
