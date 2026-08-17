#[cfg(ossl300)]
struct EvpKdf(*mut ffi::EVP_KDF);

#[cfg(ossl300)]
impl Drop for EvpKdf {
    fn drop(&mut self) {
        unsafe {
            ffi::EVP_KDF_free(self.0);
        }
    }
}

#[cfg(ossl300)]
struct EvpKdfCtx(*mut ffi::EVP_KDF_CTX);

#[cfg(ossl300)]
impl Drop for EvpKdfCtx {
    fn drop(&mut self) {
        unsafe {
            ffi::EVP_KDF_CTX_free(self.0);
        }
    }
}

#[cfg(ossl300)]
#[derive(Copy, Clone)]
pub enum HkdfMode {
    ExtractAndExpand,
    ExtractOnly,
    ExpandOnly,
}

cfg_if::cfg_if! {
    if #[cfg(ossl300)] {
        use std::ffi::CStr;
        use std::ptr;
        use foreign_types::ForeignTypeRef;
        use libc::c_char;
        use crate::{cvt, cvt_p};
        use crate::lib_ctx::LibCtxRef;
        use crate::error::ErrorStack;
        use crate::ossl_param::{OsslParamBuilder, OsslParamArray};
        use crate::md::MdRef;

        const OSSL_KDF_PARAM_DIGEST: &CStr = c"digest";
        const OSSL_KDF_PARAM_KEY: &CStr = c"key";
        const OSSL_KDF_PARAM_SALT: &CStr = c"salt";
        const OSSL_KDF_PARAM_INFO: &CStr = c"info";
        const OSSL_KDF_PARAM_MODE: &CStr = c"mode";

        /// Derives a key using a KDF.
        fn kdf_digest(
            kdf_identifier: &CStr,
            ctx: Option<&LibCtxRef>,
            params: &OsslParamArray,
            out: &mut [u8],
        ) -> Result<(), ErrorStack> {
            let libctx = ctx.map_or(ptr::null_mut(), ForeignTypeRef::as_ptr);
            unsafe {
                let kdf = EvpKdf(cvt_p(ffi::EVP_KDF_fetch(
                    libctx,
                    kdf_identifier.as_ptr() as *const c_char,
                    ptr::null(),
                ))?);
                let ctx = EvpKdfCtx(cvt_p(ffi::EVP_KDF_CTX_new(kdf.0))?);
                cvt(ffi::EVP_KDF_derive(
                    ctx.0,
                    out.as_mut_ptr(),
                    out.len(),
                    params.as_ptr(),
                ))
                .map(|_| ())
            }
        }

        pub fn hkdf(
            digest: &MdRef,
            key: &[u8],
            salt: Option<&[u8]>,
            info: Option<&[u8]>,
            mode: HkdfMode,
            ctx: Option<&LibCtxRef>,
            out: &mut [u8],
        ) -> Result<(), ErrorStack> {
            let mut bld = OsslParamBuilder::new()?;
            let sn = digest.type_().short_name()?;
            bld.add_utf8_string(OSSL_KDF_PARAM_DIGEST, sn.as_bytes())?;
            bld.add_octet_string(OSSL_KDF_PARAM_KEY, key)?;
            if let Some(salt) = salt {
                bld.add_octet_string(OSSL_KDF_PARAM_SALT, salt)?;
            }
            if let Some(info) = info {
                bld.add_octet_string(OSSL_KDF_PARAM_INFO, info)?;
            }
            let mode_value = match mode {
                HkdfMode::ExtractAndExpand => ffi::EVP_KDF_HKDF_MODE_EXTRACT_AND_EXPAND,
                HkdfMode::ExtractOnly => ffi::EVP_KDF_HKDF_MODE_EXTRACT_ONLY,
                HkdfMode::ExpandOnly => ffi::EVP_KDF_HKDF_MODE_EXPAND_ONLY,
            };
            bld.add_int(OSSL_KDF_PARAM_MODE, mode_value)?;
            let params = bld.to_param()?;
            kdf_digest(c"HKDF", ctx, &params, out)
        }
    }
}

cfg_if::cfg_if! {
    if #[cfg(all(ossl320, not(osslconf = "OPENSSL_NO_ARGON2")))] {
        use std::cmp;

        const OSSL_KDF_PARAM_PASSWORD: &CStr = c"pass";
        const OSSL_KDF_PARAM_SECRET: &CStr = c"secret";
        const OSSL_KDF_PARAM_ITER: &CStr = c"iter";
        const OSSL_KDF_PARAM_SIZE: &CStr = c"size";
        const OSSL_KDF_PARAM_THREADS: &CStr = c"threads";
        const OSSL_KDF_PARAM_ARGON2_AD: &CStr = c"ad";
        const OSSL_KDF_PARAM_ARGON2_LANES: &CStr = c"lanes";
        const OSSL_KDF_PARAM_ARGON2_MEMCOST: &CStr = c"memcost";

        #[allow(clippy::too_many_arguments)]
        pub fn argon2d(
            ctx: Option<&LibCtxRef>,
            pass: &[u8],
            salt: &[u8],
            ad: Option<&[u8]>,
            secret: Option<&[u8]>,
            iter: u32,
            lanes: u32,
            memcost: u32,
            out: &mut [u8],
        ) -> Result<(), ErrorStack> {
            argon2_helper(c"ARGON2D", ctx, pass, salt, ad, secret, iter, lanes, memcost, out)
        }

        #[allow(clippy::too_many_arguments)]
        pub fn argon2i(
            ctx: Option<&LibCtxRef>,
            pass: &[u8],
            salt: &[u8],
            ad: Option<&[u8]>,
            secret: Option<&[u8]>,
            iter: u32,
            lanes: u32,
            memcost: u32,
            out: &mut [u8],
        ) -> Result<(), ErrorStack> {
            argon2_helper(c"ARGON2I", ctx, pass, salt, ad, secret, iter, lanes, memcost, out)
        }

        #[allow(clippy::too_many_arguments)]
        pub fn argon2id(
            ctx: Option<&LibCtxRef>,
            pass: &[u8],
            salt: &[u8],
            ad: Option<&[u8]>,
            secret: Option<&[u8]>,
            iter: u32,
            lanes: u32,
            memcost: u32,
            out: &mut [u8],
        ) -> Result<(), ErrorStack> {
            argon2_helper(c"ARGON2ID", ctx, pass, salt, ad, secret, iter, lanes, memcost, out)
        }

        /// Derives a key using the argon2* algorithms.
        ///
        /// To use multiple cores to process the lanes in parallel you must
        /// set a global max thread count using `OSSL_set_max_threads`. On
        /// builds with no threads all lanes will be processed sequentially.
        ///
        /// Requires OpenSSL 3.2.0 or newer.
        #[allow(clippy::too_many_arguments)]
        fn argon2_helper(
            kdf_identifier: &CStr,
            ctx: Option<&LibCtxRef>,
            pass: &[u8],
            salt: &[u8],
            ad: Option<&[u8]>,
            secret: Option<&[u8]>,
            iter: u32,
            lanes: u32,
            memcost: u32,
            out: &mut [u8],
        ) -> Result<(), ErrorStack> {
            let libctx = ctx.map_or(ptr::null_mut(), ForeignTypeRef::as_ptr);
            let max_threads = unsafe {
                ffi::init();
                ffi::OSSL_get_max_threads(libctx)
            };
            let mut threads = 1;
            // If max_threads is 0, then this isn't a threaded build.
            // If max_threads is > u32::MAX we need to clamp since
            // argon2id's threads parameter is a u32.
            if max_threads > 0 {
                threads = cmp::min(lanes, cmp::min(max_threads, u32::MAX as u64) as u32);
            }
            let mut bld = OsslParamBuilder::new()?;
            bld.add_octet_string(OSSL_KDF_PARAM_PASSWORD, pass)?;
            bld.add_octet_string(OSSL_KDF_PARAM_SALT, salt)?;
            bld.add_uint(OSSL_KDF_PARAM_THREADS, threads)?;
            bld.add_uint(OSSL_KDF_PARAM_ARGON2_LANES, lanes)?;
            bld.add_uint(OSSL_KDF_PARAM_ARGON2_MEMCOST, memcost)?;
            bld.add_uint(OSSL_KDF_PARAM_ITER, iter)?;
            let size = out.len() as u32;
            bld.add_uint(OSSL_KDF_PARAM_SIZE, size)?;
            if let Some(ad) = ad {
                bld.add_octet_string(OSSL_KDF_PARAM_ARGON2_AD, ad)?;
            }
            if let Some(secret) = secret {
                bld.add_octet_string(OSSL_KDF_PARAM_SECRET, secret)?;
            }
            let params = bld.to_param()?;
            kdf_digest(kdf_identifier, ctx, &params, out)
        }
    }
}

#[cfg(test)]
mod tests {
    #[cfg(ossl300)]
    fn hkdf_test_helper(
        digest: &crate::md::MdRef,
        key: &[u8],
        salt: Option<&[u8]>,
        info: &[u8],
        expected_prk_hex: &str,
        expected_hex: &str,
    ) {
        let expected_prk = hex::decode(expected_prk_hex).unwrap();
        let expected = hex::decode(expected_hex).unwrap();
        let mut prk = vec![0; expected_prk.len()];
        let mut actual = vec![0; expected.len()];

        // Test separate Extract then Expand to check PRK
        super::hkdf(
            digest,
            key,
            salt,
            None,
            crate::kdf::HkdfMode::ExtractOnly,
            None,
            &mut prk,
        )
        .unwrap();
        assert_eq!(prk, expected_prk);
        super::hkdf(
            digest,
            &prk,
            None,
            Some(info),
            crate::kdf::HkdfMode::ExpandOnly,
            None,
            &mut actual,
        )
        .unwrap();
        assert_eq!(actual, expected);

        // Test Extract+Expand
        super::hkdf(
            digest,
            key,
            salt,
            Some(info),
            crate::kdf::HkdfMode::ExtractAndExpand,
            None,
            &mut actual,
        )
        .unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    #[cfg(ossl300)]
    fn hkdf_rfc_case_1() {
        // RFC 5869 Test Case 1 test vector (SHA-256 basic)
        let digest = crate::md::Md::sha256();
        let key = hex::decode("0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b").unwrap();
        let salt = hex::decode("000102030405060708090a0b0c").unwrap();
        let info = hex::decode("f0f1f2f3f4f5f6f7f8f9").unwrap();
        let expected_prk = concat!(
            "077709362c2e32df0ddc3f0dc47bba63",
            "90b6c73bb50f9c3122ec844ad7c2b3e5"
        );
        let expected = concat!(
            "3cb25f25faacd57a90434f64d0362f2a",
            "2d2d0a90cf1a5a4c5db02d56ecc4c5bf",
            "34007208d5b887185865"
        );
        hkdf_test_helper(digest, &key, Some(&salt), &info, expected_prk, expected);
    }

    #[test]
    #[cfg(ossl300)]
    fn hkdf_rfc_case_2() {
        // RFC 5869 Test Case 2 test vector (SHA-256 longer)
        let digest = crate::md::Md::sha256();
        let key = hex::decode(concat!(
            "000102030405060708090a0b0c0d0e0f",
            "101112131415161718191a1b1c1d1e1f",
            "202122232425262728292a2b2c2d2e2f",
            "303132333435363738393a3b3c3d3e3f",
            "404142434445464748494a4b4c4d4e4f"
        ))
        .unwrap();
        let salt = hex::decode(concat!(
            "606162636465666768696a6b6c6d6e6f",
            "707172737475767778797a7b7c7d7e7f",
            "808182838485868788898a8b8c8d8e8f",
            "909192939495969798999a9b9c9d9e9f",
            "a0a1a2a3a4a5a6a7a8a9aaabacadaeaf",
        ))
        .unwrap();
        let info = hex::decode(concat!(
            "b0b1b2b3b4b5b6b7b8b9babbbcbdbebf",
            "c0c1c2c3c4c5c6c7c8c9cacbcccdcecf",
            "d0d1d2d3d4d5d6d7d8d9dadbdcdddedf",
            "e0e1e2e3e4e5e6e7e8e9eaebecedeeef",
            "f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff",
        ))
        .unwrap();
        let expected_prk = concat!(
            "06a6b88c5853361a06104c9ceb35b45c",
            "ef760014904671014a193f40c15fc244",
        );
        let expected = concat!(
            "b11e398dc80327a1c8e7f78c596a4934",
            "4f012eda2d4efad8a050cc4c19afa97c",
            "59045a99cac7827271cb41c65e590e09",
            "da3275600c2f09b8367793a9aca3db71",
            "cc30c58179ec3e87c14c01d5c1f3434f",
            "1d87",
        );
        hkdf_test_helper(digest, &key, Some(&salt), &info, expected_prk, expected);
    }

    #[test]
    #[cfg(ossl300)]
    fn hkdf_rfc_case_3() {
        // RFC 5869 Test Case 3 test vector (SHA-256 zero-length salt/info)
        let digest = crate::md::Md::sha256();
        let key = hex::decode("0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b").unwrap();
        let salt = [0u8; 0];
        let info = [0u8; 0];
        let expected_prk = concat!(
            "19ef24a32c717b167f33a91d6f648bdf",
            "96596776afdb6377ac434c1c293ccb04",
        );
        let expected = concat!(
            "8da4e775a563c18f715f802a063c5a31",
            "b8a11f5c5ee1879ec3454e5f3c738d2d",
            "9d201395faa4b61a96c8",
        );
        hkdf_test_helper(digest, &key, Some(&salt), &info, expected_prk, expected);
    }

    #[test]
    #[cfg(ossl300)]
    fn hkdf_rfc_case_4() {
        // RFC 5869 Test Case 4 test vector (SHA-1 basic)
        let digest = crate::md::Md::sha1();
        let key = hex::decode("0b0b0b0b0b0b0b0b0b0b0b").unwrap();
        let salt = hex::decode("000102030405060708090a0b0c").unwrap();
        let info = hex::decode("f0f1f2f3f4f5f6f7f8f9").unwrap();
        let expected_prk = "9b6c18c432a7bf8f0e71c8eb88f4b30baa2ba243";
        let expected = concat!(
            "085a01ea1b10f36933068b56efa5ad81",
            "a4f14b822f5b091568a9cdd4f155fda2",
            "c22e422478d305f3f896"
        );
        hkdf_test_helper(digest, &key, Some(&salt), &info, expected_prk, expected);
    }

    #[test]
    #[cfg(ossl300)]
    fn hkdf_rfc_case_5() {
        // RFC 5869 Test Case 5 test vector (SHA-1 longer)
        let digest = crate::md::Md::sha1();
        let key = hex::decode(concat!(
            "000102030405060708090a0b0c0d0e0f",
            "101112131415161718191a1b1c1d1e1f",
            "202122232425262728292a2b2c2d2e2f",
            "303132333435363738393a3b3c3d3e3f",
            "404142434445464748494a4b4c4d4e4f"
        ))
        .unwrap();
        let salt = hex::decode(concat!(
            "606162636465666768696a6b6c6d6e6f",
            "707172737475767778797a7b7c7d7e7f",
            "808182838485868788898a8b8c8d8e8f",
            "909192939495969798999a9b9c9d9e9f",
            "a0a1a2a3a4a5a6a7a8a9aaabacadaeaf",
        ))
        .unwrap();
        let info = hex::decode(concat!(
            "b0b1b2b3b4b5b6b7b8b9babbbcbdbebf",
            "c0c1c2c3c4c5c6c7c8c9cacbcccdcecf",
            "d0d1d2d3d4d5d6d7d8d9dadbdcdddedf",
            "e0e1e2e3e4e5e6e7e8e9eaebecedeeef",
            "f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff",
        ))
        .unwrap();
        let expected_prk = "8adae09a2a307059478d309b26c4115a224cfaf6";
        let expected = concat!(
            "0bd770a74d1160f7c9f12cd5912a06eb",
            "ff6adcae899d92191fe4305673ba2ffe",
            "8fa3f1a4e5ad79f3f334b3b202b2173c",
            "486ea37ce3d397ed034c7f9dfeb15c5e",
            "927336d0441f4c4300e2cff0d0900b52",
            "d3b4",
        );
        hkdf_test_helper(digest, &key, Some(&salt), &info, expected_prk, expected);
    }

    #[test]
    #[cfg(ossl300)]
    fn hkdf_rfc_case_6() {
        // RFC 5869 Test Case 6 test vector (SHA-1 zero-length salt/info)
        let digest = crate::md::Md::sha1();
        let key = hex::decode("0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b").unwrap();
        let salt = [0u8; 0];
        let info = [0u8; 0];
        let expected_prk = "da8c8a73c7fa77288ec6f5e7c297786aa0d32d01";
        let expected = concat!(
            "0ac1af7002b3d761d1e55298da9d0506",
            "b9ae52057220a306e07b6b87e8df21d0",
            "ea00033de03984d34918",
        );
        hkdf_test_helper(digest, &key, Some(&salt), &info, expected_prk, expected);
    }

    #[test]
    #[cfg(ossl300)]
    fn hkdf_rfc_case_7() {
        // RFC 5869 Test Case 7 test vector (SHA-1 no salt, zero-length info)
        let digest = crate::md::Md::sha1();
        let key = hex::decode("0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c").unwrap();
        let salt = None;
        let info = [0u8; 0];
        let expected_prk = "2adccada18779e7c2077ad2eb19d3f3e731385dd";
        let expected = concat!(
            "2c91117204d745f3500d636a62f64f0a",
            "b3bae548aa53d423b0d1f27ebba6f5e5",
            "673a081d70cce7acfc48",
        );
        hkdf_test_helper(digest, &key, salt, &info, expected_prk, expected);
    }

    #[test]
    #[cfg(all(ossl320, not(osslconf = "OPENSSL_NO_ARGON2")))]
    fn argon2id() {
        // RFC 9106 test vector for argon2id
        let pass = hex::decode("0101010101010101010101010101010101010101010101010101010101010101")
            .unwrap();
        let salt = hex::decode("02020202020202020202020202020202").unwrap();
        let secret = hex::decode("0303030303030303").unwrap();
        let ad = hex::decode("040404040404040404040404").unwrap();
        let expected = "0d640df58d78766c08c037a34a8b53c9d01ef0452d75b65eb52520e96b01e659";

        let mut actual = [0u8; 32];
        super::argon2id(
            None,
            &pass,
            &salt,
            Some(&ad),
            Some(&secret),
            3,
            4,
            32,
            &mut actual,
        )
        .unwrap();
        assert_eq!(hex::encode(&actual[..]), expected);
    }

    #[test]
    #[cfg(all(ossl320, not(osslconf = "OPENSSL_NO_ARGON2")))]
    fn argon2d() {
        // RFC 9106 test vector for argon2d
        let pass = hex::decode("0101010101010101010101010101010101010101010101010101010101010101")
            .unwrap();
        let salt = hex::decode("02020202020202020202020202020202").unwrap();
        let secret = hex::decode("0303030303030303").unwrap();
        let ad = hex::decode("040404040404040404040404").unwrap();
        let expected = "512b391b6f1162975371d30919734294f868e3be3984f3c1a13a4db9fabe4acb";

        let mut actual = [0u8; 32];
        super::argon2d(
            None,
            &pass,
            &salt,
            Some(&ad),
            Some(&secret),
            3,
            4,
            32,
            &mut actual,
        )
        .unwrap();
        assert_eq!(hex::encode(&actual[..]), expected);
    }

    #[test]
    #[cfg(all(ossl320, not(osslconf = "OPENSSL_NO_ARGON2")))]
    fn argon2i() {
        // RFC 9106 test vector for argon2i
        let pass = hex::decode("0101010101010101010101010101010101010101010101010101010101010101")
            .unwrap();
        let salt = hex::decode("02020202020202020202020202020202").unwrap();
        let secret = hex::decode("0303030303030303").unwrap();
        let ad = hex::decode("040404040404040404040404").unwrap();
        let expected = "c814d9d1dc7f37aa13f0d77f2494bda1c8de6b016dd388d29952a4c4672b6ce8";

        let mut actual = [0u8; 32];
        super::argon2i(
            None,
            &pass,
            &salt,
            Some(&ad),
            Some(&secret),
            3,
            4,
            32,
            &mut actual,
        )
        .unwrap();
        assert_eq!(hex::encode(&actual[..]), expected);
    }

    #[test]
    #[cfg(all(ossl320, not(osslconf = "OPENSSL_NO_ARGON2")))]
    fn argon2id_no_ad_secret() {
        // Test vector from OpenSSL
        let pass = b"";
        let salt = hex::decode("02020202020202020202020202020202").unwrap();
        let expected = "0a34f1abde67086c82e785eaf17c68382259a264f4e61b91cd2763cb75ac189a";

        let mut actual = [0u8; 32];
        super::argon2id(None, pass, &salt, None, None, 3, 4, 32, &mut actual).unwrap();
        assert_eq!(hex::encode(&actual[..]), expected);
    }
}
