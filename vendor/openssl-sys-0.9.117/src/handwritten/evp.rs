use super::super::*;
use libc::size_t;
use std::ffi::{c_char, c_int, c_long, c_uchar, c_ulong, c_void};

cfg_if! {
    if #[cfg(ossl300)] {
        extern "C" {
            pub fn EVP_MD_get_block_size(md: *const EVP_MD) -> c_int;
            pub fn EVP_MD_get_size(md: *const EVP_MD) -> c_int;
            pub fn EVP_MD_get_type(md: *const EVP_MD) -> c_int;

            pub fn EVP_MD_CTX_get0_md(ctx: *const EVP_MD_CTX) -> *const EVP_MD;

            pub fn EVP_CIPHER_get_key_length(cipher: *const EVP_CIPHER) -> c_int;
            pub fn EVP_CIPHER_get_block_size(cipher: *const EVP_CIPHER) -> c_int;
            pub fn EVP_CIPHER_get_iv_length(cipher: *const EVP_CIPHER) -> c_int;
            pub fn EVP_CIPHER_get_nid(cipher: *const EVP_CIPHER) -> c_int;
            pub fn EVP_CIPHER_get_flags(cipher: *const EVP_CIPHER) -> c_ulong;
            pub fn EVP_CIPHER_fetch(
                ctx: *mut OSSL_LIB_CTX,
                algorithm: *const c_char,
                properties: *const c_char,
            ) -> *mut EVP_CIPHER;
            pub fn EVP_CIPHER_free(cipher: *mut EVP_CIPHER);

            pub fn EVP_CIPHER_CTX_get0_cipher(ctx: *const EVP_CIPHER_CTX) -> *const EVP_CIPHER;
            pub fn EVP_CIPHER_CTX_get_block_size(ctx: *const EVP_CIPHER_CTX) -> c_int;
            pub fn EVP_CIPHER_CTX_get_key_length(ctx: *const EVP_CIPHER_CTX) -> c_int;
            pub fn EVP_CIPHER_CTX_get_iv_length(ctx: *const EVP_CIPHER_CTX) -> c_int;
            pub fn EVP_CIPHER_CTX_get_tag_length(ctx: *const EVP_CIPHER_CTX) -> c_int;
            pub fn EVP_CIPHER_CTX_get_num(ctx: *const EVP_CIPHER_CTX) -> c_int;
        }
    } else {
        extern "C" {
            pub fn EVP_MD_block_size(md: *const EVP_MD) -> c_int;
            pub fn EVP_MD_size(md: *const EVP_MD) -> c_int;
            pub fn EVP_MD_type(md: *const EVP_MD) -> c_int;

            pub fn EVP_MD_CTX_md(ctx: *const EVP_MD_CTX) -> *const EVP_MD;

            pub fn EVP_CIPHER_key_length(cipher: *const EVP_CIPHER) -> c_int;
            pub fn EVP_CIPHER_block_size(cipher: *const EVP_CIPHER) -> c_int;
            pub fn EVP_CIPHER_iv_length(cipher: *const EVP_CIPHER) -> c_int;
            pub fn EVP_CIPHER_nid(cipher: *const EVP_CIPHER) -> c_int;
            pub fn EVP_CIPHER_flags(cipher: *const EVP_CIPHER) -> c_ulong;

            pub fn EVP_CIPHER_CTX_cipher(ctx: *const EVP_CIPHER_CTX) -> *const EVP_CIPHER;
            pub fn EVP_CIPHER_CTX_block_size(ctx: *const EVP_CIPHER_CTX) -> c_int;
            pub fn EVP_CIPHER_CTX_key_length(ctx: *const EVP_CIPHER_CTX) -> c_int;
            pub fn EVP_CIPHER_CTX_iv_length(ctx: *const EVP_CIPHER_CTX) -> c_int;
            #[cfg(ossl110)]
            pub fn EVP_CIPHER_CTX_num(ctx: *const EVP_CIPHER_CTX) -> c_int;
        }
    }
}

cfg_if! {
    if #[cfg(any(ossl110, libressl382))] {
        extern "C" {
            pub fn EVP_MD_CTX_new() -> *mut EVP_MD_CTX;
            pub fn EVP_MD_CTX_free(ctx: *mut EVP_MD_CTX);
        }
    } else {
        extern "C" {
            pub fn EVP_MD_CTX_create() -> *mut EVP_MD_CTX;
            pub fn EVP_MD_CTX_destroy(ctx: *mut EVP_MD_CTX);
        }
    }
}

cfg_if! {
    if #[cfg(ossl300)] {
        extern "C" {
            pub fn EVP_default_properties_is_fips_enabled(libctx: *mut OSSL_LIB_CTX) -> c_int;
            pub fn EVP_default_properties_enable_fips(libctx: *mut OSSL_LIB_CTX, enable: c_int) -> c_int;
        }
    }
}

extern "C" {
    pub fn EVP_DigestInit_ex(ctx: *mut EVP_MD_CTX, typ: *const EVP_MD, imple: *mut ENGINE)
        -> c_int;
    pub fn EVP_DigestUpdate(ctx: *mut EVP_MD_CTX, data: *const c_void, n: size_t) -> c_int;
    pub fn EVP_DigestFinal_ex(ctx: *mut EVP_MD_CTX, res: *mut u8, n: *mut u32) -> c_int;
    #[cfg(ossl300)]
    pub fn EVP_Q_digest(
        libctx: *mut OSSL_LIB_CTX,
        name: *const c_char,
        propq: *const c_char,
        data: *const c_void,
        count: size_t,
        md: *mut c_uchar,
        size: *mut size_t,
    ) -> c_int;
    pub fn EVP_DigestInit(ctx: *mut EVP_MD_CTX, typ: *const EVP_MD) -> c_int;
    pub fn EVP_DigestFinal(ctx: *mut EVP_MD_CTX, res: *mut u8, n: *mut u32) -> c_int;
    #[cfg(ossl111)]
    pub fn EVP_DigestFinalXOF(ctx: *mut EVP_MD_CTX, res: *mut u8, len: usize) -> c_int;
    #[cfg(any(ossl330, awslc))]
    pub fn EVP_DigestSqueeze(ctx: *mut EVP_MD_CTX, res: *mut u8, len: usize) -> c_int;

    #[cfg(ossl300)]
    pub fn EVP_MD_fetch(
        ctx: *mut OSSL_LIB_CTX,
        algorithm: *const c_char,
        properties: *const c_char,
    ) -> *mut EVP_MD;

    #[cfg(ossl300)]
    pub fn EVP_MD_free(md: *mut EVP_MD);

    pub fn EVP_BytesToKey(
        typ: *const EVP_CIPHER,
        md: *const EVP_MD,
        salt: *const u8,
        data: *const u8,
        datalen: c_int,
        count: c_int,
        key: *mut u8,
        iv: *mut u8,
    ) -> c_int;

    pub fn EVP_CipherInit(
        ctx: *mut EVP_CIPHER_CTX,
        evp: *const EVP_CIPHER,
        key: *const u8,
        iv: *const u8,
        mode: c_int,
    ) -> c_int;
    pub fn EVP_CipherInit_ex(
        ctx: *mut EVP_CIPHER_CTX,
        type_: *const EVP_CIPHER,
        impl_: *mut ENGINE,
        key: *const c_uchar,
        iv: *const c_uchar,
        enc: c_int,
    ) -> c_int;
    pub fn EVP_CipherUpdate(
        ctx: *mut EVP_CIPHER_CTX,
        outbuf: *mut u8,
        outlen: *mut c_int,
        inbuf: *const u8,
        inlen: c_int,
    ) -> c_int;
    pub fn EVP_CipherFinal(ctx: *mut EVP_CIPHER_CTX, res: *mut u8, len: *mut c_int) -> c_int;

    pub fn EVP_DigestSignInit(
        ctx: *mut EVP_MD_CTX,
        pctx: *mut *mut EVP_PKEY_CTX,
        type_: *const EVP_MD,
        e: *mut ENGINE,
        pkey: *mut EVP_PKEY,
    ) -> c_int;

    #[cfg(ossl300)]
    pub fn EVP_DigestSignUpdate(ctx: *mut EVP_MD_CTX, data: *const c_void, dsize: size_t) -> c_int;
    pub fn EVP_DigestSignFinal(
        ctx: *mut EVP_MD_CTX,
        sig: *mut c_uchar,
        siglen: *mut size_t,
    ) -> c_int;
    pub fn EVP_DigestVerifyInit(
        ctx: *mut EVP_MD_CTX,
        pctx: *mut *mut EVP_PKEY_CTX,
        type_: *const EVP_MD,
        e: *mut ENGINE,
        pkey: *mut EVP_PKEY,
    ) -> c_int;
    #[cfg(ossl300)]
    pub fn EVP_DigestVerifyUpdate(
        ctx: *mut EVP_MD_CTX,
        data: *const c_void,
        dsize: size_t,
    ) -> c_int;
    pub fn EVP_DigestVerifyFinal(
        ctx: *mut EVP_MD_CTX,
        sigret: *const c_uchar,
        siglen: size_t,
    ) -> c_int;
    pub fn EVP_SealInit(
        ctx: *mut EVP_CIPHER_CTX,
        type_: *const EVP_CIPHER,
        ek: *mut *mut c_uchar,
        ekl: *mut c_int,
        iv: *mut c_uchar,
        pubk: *mut *mut EVP_PKEY,
        npubk: c_int,
    ) -> c_int;
    pub fn EVP_SealFinal(ctx: *mut EVP_CIPHER_CTX, out: *mut c_uchar, outl: *mut c_int) -> c_int;
    pub fn EVP_EncryptInit_ex(
        ctx: *mut EVP_CIPHER_CTX,
        cipher: *const EVP_CIPHER,
        impl_: *mut ENGINE,
        key: *const c_uchar,
        iv: *const c_uchar,
    ) -> c_int;
    pub fn EVP_EncryptUpdate(
        ctx: *mut EVP_CIPHER_CTX,
        out: *mut c_uchar,
        outl: *mut c_int,
        in_: *const u8,
        inl: c_int,
    ) -> c_int;
    pub fn EVP_EncryptFinal_ex(
        ctx: *mut EVP_CIPHER_CTX,
        out: *mut c_uchar,
        outl: *mut c_int,
    ) -> c_int;
    pub fn EVP_OpenInit(
        ctx: *mut EVP_CIPHER_CTX,
        type_: *const EVP_CIPHER,
        ek: *const c_uchar,
        ekl: c_int,
        iv: *const c_uchar,
        priv_: *mut EVP_PKEY,
    ) -> c_int;
    pub fn EVP_OpenFinal(ctx: *mut EVP_CIPHER_CTX, out: *mut c_uchar, outl: *mut c_int) -> c_int;
    pub fn EVP_DecryptInit_ex(
        ctx: *mut EVP_CIPHER_CTX,
        cipher: *const EVP_CIPHER,
        impl_: *mut ENGINE,
        key: *const c_uchar,
        iv: *const c_uchar,
    ) -> c_int;
    pub fn EVP_DecryptUpdate(
        ctx: *mut EVP_CIPHER_CTX,
        out: *mut c_uchar,
        outl: *mut c_int,
        in_: *const u8,
        inl: c_int,
    ) -> c_int;
    pub fn EVP_DecryptFinal_ex(
        ctx: *mut EVP_CIPHER_CTX,
        outm: *mut c_uchar,
        outl: *mut c_int,
    ) -> c_int;
}
cfg_if! {
    if #[cfg(ossl300)] {
        extern "C" {
            pub fn EVP_PKEY_get_size(pkey: *const EVP_PKEY) -> c_int;
        }
    } else {
        const_ptr_api! {
            extern "C" {
                pub fn EVP_PKEY_size(pkey: #[const_ptr_if(any(ossl111b, libressl))] EVP_PKEY) -> c_int;
            }
        }
    }
}
cfg_if! {
    if #[cfg(any(ossl111, libressl))] {
        extern "C" {
            pub fn EVP_DigestSign(
                ctx: *mut EVP_MD_CTX,
                sigret: *mut c_uchar,
                siglen: *mut size_t,
                tbs: *const c_uchar,
                tbslen: size_t
            ) -> c_int;

            pub fn EVP_DigestVerify(
                ctx: *mut EVP_MD_CTX,
                sigret: *const c_uchar,
                siglen: size_t,
                tbs: *const c_uchar,
                tbslen: size_t
            ) -> c_int;
        }
    }
}

extern "C" {
    pub fn EVP_CIPHER_CTX_new() -> *mut EVP_CIPHER_CTX;
    pub fn EVP_CIPHER_CTX_free(ctx: *mut EVP_CIPHER_CTX);
    pub fn EVP_CIPHER_CTX_copy(dst: *mut EVP_CIPHER_CTX, src: *const EVP_CIPHER_CTX) -> c_int;

    pub fn EVP_MD_CTX_copy_ex(dst: *mut EVP_MD_CTX, src: *const EVP_MD_CTX) -> c_int;
    #[cfg(any(ossl111, libressl))]
    pub fn EVP_MD_CTX_reset(ctx: *mut EVP_MD_CTX) -> c_int;
    pub fn EVP_CIPHER_CTX_set_key_length(ctx: *mut EVP_CIPHER_CTX, keylen: c_int) -> c_int;
    pub fn EVP_CIPHER_CTX_set_padding(ctx: *mut EVP_CIPHER_CTX, padding: c_int) -> c_int;
    pub fn EVP_CIPHER_CTX_ctrl(
        ctx: *mut EVP_CIPHER_CTX,
        type_: c_int,
        arg: c_int,
        ptr: *mut c_void,
    ) -> c_int;
    pub fn EVP_CIPHER_CTX_rand_key(ctx: *mut EVP_CIPHER_CTX, key: *mut c_uchar) -> c_int;
    pub fn EVP_CIPHER_CTX_set_flags(ctx: *mut EVP_CIPHER_CTX, flags: c_int);

    pub fn EVP_md_null() -> *const EVP_MD;
    pub fn EVP_md5() -> *const EVP_MD;
    pub fn EVP_sha1() -> *const EVP_MD;
    pub fn EVP_sha224() -> *const EVP_MD;
    pub fn EVP_sha256() -> *const EVP_MD;
    pub fn EVP_sha384() -> *const EVP_MD;
    pub fn EVP_sha512() -> *const EVP_MD;
    #[cfg(any(ossl111, libressl380))]
    pub fn EVP_sha3_224() -> *const EVP_MD;
    #[cfg(any(ossl111, libressl380))]
    pub fn EVP_sha3_256() -> *const EVP_MD;
    #[cfg(any(ossl111, libressl380))]
    pub fn EVP_sha3_384() -> *const EVP_MD;
    #[cfg(any(ossl111, libressl380))]
    pub fn EVP_sha3_512() -> *const EVP_MD;
    #[cfg(ossl111)]
    pub fn EVP_shake128() -> *const EVP_MD;
    #[cfg(ossl111)]
    pub fn EVP_shake256() -> *const EVP_MD;
    pub fn EVP_ripemd160() -> *const EVP_MD;
    #[cfg(all(any(ossl111, libressl), not(osslconf = "OPENSSL_NO_SM3")))]
    pub fn EVP_sm3() -> *const EVP_MD;
    pub fn EVP_des_ecb() -> *const EVP_CIPHER;
    pub fn EVP_des_ede3() -> *const EVP_CIPHER;
    pub fn EVP_des_ede3_cbc() -> *const EVP_CIPHER;
    pub fn EVP_des_ede3_ecb() -> *const EVP_CIPHER;
    pub fn EVP_des_ede3_cfb64() -> *const EVP_CIPHER;
    pub fn EVP_des_ede3_cfb8() -> *const EVP_CIPHER;
    pub fn EVP_des_ede3_ofb() -> *const EVP_CIPHER;
    pub fn EVP_des_cbc() -> *const EVP_CIPHER;
    #[cfg(not(osslconf = "OPENSSL_NO_RC4"))]
    pub fn EVP_rc4() -> *const EVP_CIPHER;
    pub fn EVP_bf_ecb() -> *const EVP_CIPHER;
    pub fn EVP_bf_cbc() -> *const EVP_CIPHER;
    pub fn EVP_bf_cfb64() -> *const EVP_CIPHER;
    pub fn EVP_bf_ofb() -> *const EVP_CIPHER;
    pub fn EVP_aes_128_ecb() -> *const EVP_CIPHER;
    pub fn EVP_aes_128_cbc() -> *const EVP_CIPHER;
    pub fn EVP_aes_128_cfb1() -> *const EVP_CIPHER;
    pub fn EVP_aes_128_cfb8() -> *const EVP_CIPHER;
    pub fn EVP_aes_128_cfb128() -> *const EVP_CIPHER;
    pub fn EVP_aes_128_ctr() -> *const EVP_CIPHER;
    pub fn EVP_aes_128_ccm() -> *const EVP_CIPHER;
    pub fn EVP_aes_128_gcm() -> *const EVP_CIPHER;
    pub fn EVP_aes_128_xts() -> *const EVP_CIPHER;
    pub fn EVP_aes_128_ofb() -> *const EVP_CIPHER;
    #[cfg(ossl110)]
    pub fn EVP_aes_128_ocb() -> *const EVP_CIPHER;
    #[cfg(ossl110)]
    pub fn EVP_aes_128_wrap() -> *const EVP_CIPHER;
    #[cfg(ossl110)]
    pub fn EVP_aes_128_wrap_pad() -> *const EVP_CIPHER;
    pub fn EVP_aes_192_ecb() -> *const EVP_CIPHER;
    pub fn EVP_aes_192_cbc() -> *const EVP_CIPHER;
    pub fn EVP_aes_192_cfb1() -> *const EVP_CIPHER;
    pub fn EVP_aes_192_cfb8() -> *const EVP_CIPHER;
    pub fn EVP_aes_192_cfb128() -> *const EVP_CIPHER;
    pub fn EVP_aes_192_ctr() -> *const EVP_CIPHER;
    pub fn EVP_aes_192_ccm() -> *const EVP_CIPHER;
    pub fn EVP_aes_192_gcm() -> *const EVP_CIPHER;
    pub fn EVP_aes_192_ofb() -> *const EVP_CIPHER;
    #[cfg(ossl110)]
    pub fn EVP_aes_192_ocb() -> *const EVP_CIPHER;
    #[cfg(ossl110)]
    pub fn EVP_aes_192_wrap() -> *const EVP_CIPHER;
    #[cfg(ossl110)]
    pub fn EVP_aes_192_wrap_pad() -> *const EVP_CIPHER;
    pub fn EVP_aes_256_ecb() -> *const EVP_CIPHER;
    pub fn EVP_aes_256_cbc() -> *const EVP_CIPHER;
    pub fn EVP_aes_256_cfb1() -> *const EVP_CIPHER;
    pub fn EVP_aes_256_cfb8() -> *const EVP_CIPHER;
    pub fn EVP_aes_256_cfb128() -> *const EVP_CIPHER;
    pub fn EVP_aes_256_ctr() -> *const EVP_CIPHER;
    pub fn EVP_aes_256_ccm() -> *const EVP_CIPHER;
    pub fn EVP_aes_256_gcm() -> *const EVP_CIPHER;
    pub fn EVP_aes_256_xts() -> *const EVP_CIPHER;
    pub fn EVP_aes_256_ofb() -> *const EVP_CIPHER;
    #[cfg(ossl110)]
    pub fn EVP_aes_256_ocb() -> *const EVP_CIPHER;
    #[cfg(ossl110)]
    pub fn EVP_aes_256_wrap() -> *const EVP_CIPHER;
    #[cfg(ossl110)]
    pub fn EVP_aes_256_wrap_pad() -> *const EVP_CIPHER;
    #[cfg(not(osslconf = "OPENSSL_NO_CHACHA"))]
    pub fn EVP_chacha20() -> *const EVP_CIPHER;
    #[cfg(all(any(ossl110, libressl360), not(osslconf = "OPENSSL_NO_CHACHA")))]
    pub fn EVP_chacha20_poly1305() -> *const EVP_CIPHER;
    #[cfg(not(osslconf = "OPENSSL_NO_SEED"))]
    pub fn EVP_seed_cbc() -> *const EVP_CIPHER;
    #[cfg(not(osslconf = "OPENSSL_NO_SEED"))]
    pub fn EVP_seed_cfb128() -> *const EVP_CIPHER;
    #[cfg(not(osslconf = "OPENSSL_NO_SEED"))]
    pub fn EVP_seed_ecb() -> *const EVP_CIPHER;
    #[cfg(not(osslconf = "OPENSSL_NO_SEED"))]
    pub fn EVP_seed_ofb() -> *const EVP_CIPHER;

    #[cfg(all(any(ossl111, libressl), not(osslconf = "OPENSSL_NO_SM4")))]
    pub fn EVP_sm4_ecb() -> *const EVP_CIPHER;
    #[cfg(all(any(ossl111, libressl), not(osslconf = "OPENSSL_NO_SM4")))]
    pub fn EVP_sm4_cbc() -> *const EVP_CIPHER;
    #[cfg(all(any(ossl111, libressl), not(osslconf = "OPENSSL_NO_SM4")))]
    pub fn EVP_sm4_cfb128() -> *const EVP_CIPHER;
    #[cfg(all(any(ossl111, libressl), not(osslconf = "OPENSSL_NO_SM4")))]
    pub fn EVP_sm4_ofb() -> *const EVP_CIPHER;
    #[cfg(all(any(ossl111, libressl), not(osslconf = "OPENSSL_NO_SM4")))]
    pub fn EVP_sm4_ctr() -> *const EVP_CIPHER;

    #[cfg(not(osslconf = "OPENSSL_NO_CAMELLIA"))]
    pub fn EVP_camellia_128_cfb128() -> *const EVP_CIPHER;
    #[cfg(not(osslconf = "OPENSSL_NO_CAMELLIA"))]
    pub fn EVP_camellia_128_ecb() -> *const EVP_CIPHER;
    #[cfg(not(osslconf = "OPENSSL_NO_CAMELLIA"))]
    pub fn EVP_camellia_128_cbc() -> *const EVP_CIPHER;
    #[cfg(not(osslconf = "OPENSSL_NO_CAMELLIA"))]
    pub fn EVP_camellia_128_ofb() -> *const EVP_CIPHER;
    #[cfg(not(osslconf = "OPENSSL_NO_CAMELLIA"))]
    pub fn EVP_camellia_192_cfb128() -> *const EVP_CIPHER;
    #[cfg(not(osslconf = "OPENSSL_NO_CAMELLIA"))]
    pub fn EVP_camellia_192_ecb() -> *const EVP_CIPHER;
    #[cfg(not(osslconf = "OPENSSL_NO_CAMELLIA"))]
    pub fn EVP_camellia_192_cbc() -> *const EVP_CIPHER;
    #[cfg(not(osslconf = "OPENSSL_NO_CAMELLIA"))]
    pub fn EVP_camellia_192_ofb() -> *const EVP_CIPHER;
    #[cfg(not(osslconf = "OPENSSL_NO_CAMELLIA"))]
    pub fn EVP_camellia_256_cfb128() -> *const EVP_CIPHER;
    #[cfg(not(osslconf = "OPENSSL_NO_CAMELLIA"))]
    pub fn EVP_camellia_256_ecb() -> *const EVP_CIPHER;
    #[cfg(not(osslconf = "OPENSSL_NO_CAMELLIA"))]
    pub fn EVP_camellia_256_cbc() -> *const EVP_CIPHER;
    #[cfg(not(osslconf = "OPENSSL_NO_CAMELLIA"))]
    pub fn EVP_camellia_256_ofb() -> *const EVP_CIPHER;

    #[cfg(not(osslconf = "OPENSSL_NO_CAST"))]
    pub fn EVP_cast5_cfb64() -> *const EVP_CIPHER;
    #[cfg(not(osslconf = "OPENSSL_NO_CAST"))]
    pub fn EVP_cast5_ecb() -> *const EVP_CIPHER;
    #[cfg(not(osslconf = "OPENSSL_NO_CAST"))]
    pub fn EVP_cast5_cbc() -> *const EVP_CIPHER;
    #[cfg(not(osslconf = "OPENSSL_NO_CAST"))]
    pub fn EVP_cast5_ofb() -> *const EVP_CIPHER;

    #[cfg(not(osslconf = "OPENSSL_NO_IDEA"))]
    pub fn EVP_idea_cfb64() -> *const EVP_CIPHER;
    #[cfg(not(osslconf = "OPENSSL_NO_IDEA"))]
    pub fn EVP_idea_ecb() -> *const EVP_CIPHER;
    #[cfg(not(osslconf = "OPENSSL_NO_IDEA"))]
    pub fn EVP_idea_cbc() -> *const EVP_CIPHER;
    #[cfg(not(osslconf = "OPENSSL_NO_IDEA"))]
    pub fn EVP_idea_ofb() -> *const EVP_CIPHER;

    #[cfg(not(osslconf = "OPENSSL_NO_RC2"))]
    pub fn EVP_rc2_cbc() -> *const EVP_CIPHER;
    #[cfg(not(osslconf = "OPENSSL_NO_RC2"))]
    pub fn EVP_rc2_40_cbc() -> *const EVP_CIPHER;

    #[cfg(not(ossl110))]
    pub fn OPENSSL_add_all_algorithms_noconf();

    pub fn EVP_get_digestbyname(name: *const c_char) -> *const EVP_MD;
    pub fn EVP_get_cipherbyname(name: *const c_char) -> *const EVP_CIPHER;
}

cfg_if! {
    if #[cfg(ossl300)] {
        extern "C" {
            pub fn EVP_PKEY_get_id(pkey: *const EVP_PKEY) -> c_int;
            pub fn EVP_PKEY_get_bits(key: *const EVP_PKEY) -> c_int;
            pub fn EVP_PKEY_get_security_bits(key: *const EVP_PKEY) -> c_int;
        }
    } else {
        extern "C" {
            pub fn EVP_PKEY_id(pkey: *const EVP_PKEY) -> c_int;
            pub fn EVP_PKEY_bits(key: *const EVP_PKEY) -> c_int;
            #[cfg(any(ossl110, libressl360))]
            pub fn EVP_PKEY_security_bits(pkey: *const EVP_PKEY) -> c_int;
        }
    }
}
#[cfg(not(osslconf = "OPENSSL_NO_DEPRECATED_3_0"))]
const_ptr_api! {
    extern "C" {
        pub fn EVP_PKEY_get1_RSA(k: #[const_ptr_if(libressl420)] EVP_PKEY) -> *mut RSA;
        pub fn EVP_PKEY_get1_DSA(k: #[const_ptr_if(libressl420)] EVP_PKEY) -> *mut DSA;
        pub fn EVP_PKEY_get1_DH(k: #[const_ptr_if(libressl420)] EVP_PKEY) -> *mut DH;
        pub fn EVP_PKEY_get1_EC_KEY(k: #[const_ptr_if(libressl420)] EVP_PKEY) -> *mut EC_KEY;
    }
}
#[cfg(not(osslconf = "OPENSSL_NO_DEPRECATED_3_0"))]
extern "C" {
    pub fn EVP_PKEY_assign(pkey: *mut EVP_PKEY, typ: c_int, key: *mut c_void) -> c_int;

    pub fn EVP_PKEY_set1_RSA(k: *mut EVP_PKEY, r: *mut RSA) -> c_int;
    pub fn EVP_PKEY_set1_DSA(k: *mut EVP_PKEY, k: *mut DSA) -> c_int;
    pub fn EVP_PKEY_set1_DH(k: *mut EVP_PKEY, k: *mut DH) -> c_int;
    pub fn EVP_PKEY_set1_EC_KEY(k: *mut EVP_PKEY, k: *mut EC_KEY) -> c_int;

    pub fn EVP_PKEY_cmp(a: *const EVP_PKEY, b: *const EVP_PKEY) -> c_int;
}

extern "C" {
    pub fn EVP_PKEY_new() -> *mut EVP_PKEY;
    pub fn EVP_PKEY_free(k: *mut EVP_PKEY);
    pub fn EVP_PKEY_up_ref(pkey: *mut EVP_PKEY) -> c_int;

    #[cfg(ossl300)]
    pub fn EVP_PKEY_fromdata_init(ctx: *mut EVP_PKEY_CTX) -> c_int;

    #[cfg(ossl300)]
    pub fn EVP_PKEY_fromdata(
        ctx: *mut EVP_PKEY_CTX,
        ppkey: *mut *mut EVP_PKEY,
        selection: c_int,
        param: *mut OSSL_PARAM,
    ) -> c_int;

    #[cfg(ossl300)]
    pub fn EVP_PKEY_todata(
        ppkey: *const EVP_PKEY,
        selection: c_int,
        param: *mut *mut OSSL_PARAM,
    ) -> c_int;

    #[cfg(ossl300)]
    pub fn EVP_PKEY_generate(ctx: *mut EVP_PKEY_CTX, k: *mut *mut EVP_PKEY) -> c_int;

    pub fn d2i_AutoPrivateKey(
        a: *mut *mut EVP_PKEY,
        pp: *mut *const c_uchar,
        length: c_long,
    ) -> *mut EVP_PKEY;

    #[cfg(ossl300)]
    pub fn EVP_PKEY_eq(a: *const EVP_PKEY, b: *const EVP_PKEY) -> c_int;
    #[cfg(ossl300)]
    pub fn EVP_PKEY_parameters_eq(a: *const EVP_PKEY, b: *const EVP_PKEY) -> c_int;

    pub fn EVP_PKEY_copy_parameters(to: *mut EVP_PKEY, from: *const EVP_PKEY) -> c_int;

    pub fn PKCS5_PBKDF2_HMAC_SHA1(
        pass: *const c_char,
        passlen: c_int,
        salt: *const u8,
        saltlen: c_int,
        iter: c_int,
        keylen: c_int,
        out: *mut u8,
    ) -> c_int;
    pub fn PKCS5_PBKDF2_HMAC(
        pass: *const c_char,
        passlen: c_int,
        salt: *const c_uchar,
        saltlen: c_int,
        iter: c_int,
        digest: *const EVP_MD,
        keylen: c_int,
        out: *mut u8,
    ) -> c_int;

    #[cfg(ossl110)]
    pub fn EVP_PBE_scrypt(
        pass: *const c_char,
        passlen: size_t,
        salt: *const c_uchar,
        saltlen: size_t,
        N: u64,
        r: u64,
        p: u64,
        maxmem: u64,
        key: *mut c_uchar,
        keylen: size_t,
    ) -> c_int;

    pub fn EVP_PKEY_CTX_new(k: *mut EVP_PKEY, e: *mut ENGINE) -> *mut EVP_PKEY_CTX;
    pub fn EVP_PKEY_CTX_new_id(id: c_int, e: *mut ENGINE) -> *mut EVP_PKEY_CTX;
    #[cfg(ossl300)]
    pub fn EVP_PKEY_CTX_new_from_name(
        libctx: *mut OSSL_LIB_CTX,
        name: *const c_char,
        propquery: *const c_char,
    ) -> *mut EVP_PKEY_CTX;
    pub fn EVP_PKEY_CTX_free(ctx: *mut EVP_PKEY_CTX);

    pub fn EVP_PKEY_CTX_ctrl(
        ctx: *mut EVP_PKEY_CTX,
        keytype: c_int,
        optype: c_int,
        cmd: c_int,
        p1: c_int,
        p2: *mut c_void,
    ) -> c_int;

    #[cfg(ossl300)]
    pub fn EVP_PKEY_CTX_set_signature_md(ctx: *mut EVP_PKEY_CTX, md: *const EVP_MD) -> c_int;

    #[cfg(ossl300)]
    pub fn EVP_PKEY_CTX_set_params(ctx: *mut EVP_PKEY_CTX, params: *const OSSL_PARAM) -> c_int;

    #[cfg(ossl300)]
    pub fn EVP_PKEY_CTX_get_params(ctx: *mut EVP_PKEY_CTX, params: *mut OSSL_PARAM) -> c_int;

    pub fn EVP_PKEY_new_mac_key(
        type_: c_int,
        e: *mut ENGINE,
        key: *const c_uchar,
        keylen: c_int,
    ) -> *mut EVP_PKEY;

    pub fn EVP_PKEY_derive_init(ctx: *mut EVP_PKEY_CTX) -> c_int;
    pub fn EVP_PKEY_derive_set_peer(ctx: *mut EVP_PKEY_CTX, peer: *mut EVP_PKEY) -> c_int;
    #[cfg(ossl300)]
    pub fn EVP_PKEY_derive_set_peer_ex(
        ctx: *mut EVP_PKEY_CTX,
        peer: *mut EVP_PKEY,
        validate_peer: c_int,
    ) -> c_int;
    pub fn EVP_PKEY_derive(ctx: *mut EVP_PKEY_CTX, key: *mut c_uchar, size: *mut size_t) -> c_int;

    #[cfg(ossl300)]
    pub fn EVP_PKEY_Q_keygen(
        libctx: *mut OSSL_LIB_CTX,
        propq: *const c_char,
        type_: *const c_char,
        ...
    ) -> *mut EVP_PKEY;
    pub fn EVP_PKEY_keygen_init(ctx: *mut EVP_PKEY_CTX) -> c_int;
    pub fn EVP_PKEY_paramgen_init(ctx: *mut EVP_PKEY_CTX) -> c_int;
    pub fn EVP_PKEY_keygen(ctx: *mut EVP_PKEY_CTX, key: *mut *mut EVP_PKEY) -> c_int;
    pub fn EVP_PKEY_paramgen(ctx: *mut EVP_PKEY_CTX, key: *mut *mut EVP_PKEY) -> c_int;

    #[cfg(ossl111)]
    pub fn EVP_PKEY_param_check(ctx: *mut EVP_PKEY_CTX) -> c_int;
    #[cfg(ossl111)]
    pub fn EVP_PKEY_public_check(ctx: *mut EVP_PKEY_CTX) -> c_int;
    #[cfg(ossl111)]
    pub fn EVP_PKEY_check(ctx: *mut EVP_PKEY_CTX) -> c_int;

    pub fn EVP_PKEY_sign_init(ctx: *mut EVP_PKEY_CTX) -> c_int;

    #[cfg(ossl340)]
    pub fn EVP_PKEY_sign_message_init(
        ctx: *mut EVP_PKEY_CTX,
        algo: *mut EVP_SIGNATURE,
        params: *const OSSL_PARAM,
    ) -> c_int;

    pub fn EVP_PKEY_sign(
        ctx: *mut EVP_PKEY_CTX,
        sig: *mut c_uchar,
        siglen: *mut size_t,
        tbs: *const c_uchar,
        tbslen: size_t,
    ) -> c_int;
    pub fn EVP_PKEY_verify_init(ctx: *mut EVP_PKEY_CTX) -> c_int;

    #[cfg(ossl340)]
    pub fn EVP_PKEY_verify_message_init(
        ctx: *mut EVP_PKEY_CTX,
        algo: *mut EVP_SIGNATURE,
        params: *const OSSL_PARAM,
    ) -> c_int;

    pub fn EVP_PKEY_verify(
        ctx: *mut EVP_PKEY_CTX,
        sig: *const c_uchar,
        siglen: size_t,
        tbs: *const c_uchar,
        tbslen: size_t,
    ) -> c_int;
    pub fn EVP_PKEY_encrypt_init(ctx: *mut EVP_PKEY_CTX) -> c_int;
    pub fn EVP_PKEY_encrypt(
        ctx: *mut EVP_PKEY_CTX,
        pout: *mut c_uchar,
        poutlen: *mut size_t,
        pin: *const c_uchar,
        pinlen: size_t,
    ) -> c_int;
    pub fn EVP_PKEY_decrypt_init(ctx: *mut EVP_PKEY_CTX) -> c_int;
    pub fn EVP_PKEY_decrypt(
        ctx: *mut EVP_PKEY_CTX,
        pout: *mut c_uchar,
        poutlen: *mut size_t,
        pin: *const c_uchar,
        pinlen: size_t,
    ) -> c_int;
    pub fn EVP_PKEY_verify_recover_init(ctx: *mut EVP_PKEY_CTX) -> c_int;
    pub fn EVP_PKEY_verify_recover(
        ctx: *mut EVP_PKEY_CTX,
        rout: *mut c_uchar,
        routlen: *mut size_t,
        sig: *const c_uchar,
        siglen: size_t,
    ) -> c_int;

    #[cfg(ossl300)]
    pub fn EVP_PKEY_encapsulate_init(ctx: *mut EVP_PKEY_CTX, params: *const OSSL_PARAM) -> c_int;
    #[cfg(ossl300)]
    pub fn EVP_PKEY_encapsulate(
        ctx: *mut EVP_PKEY_CTX,
        wrappedkey: *mut c_uchar,
        wrappedkeylen: *mut size_t,
        genkey: *mut c_uchar,
        genkeylen: *mut size_t,
    ) -> c_int;

    #[cfg(ossl300)]
    pub fn EVP_PKEY_decapsulate_init(ctx: *mut EVP_PKEY_CTX, params: *const OSSL_PARAM) -> c_int;
    #[cfg(ossl300)]
    pub fn EVP_PKEY_decapsulate(
        ctx: *mut EVP_PKEY_CTX,
        genkey: *mut c_uchar,
        genkeylen: *mut size_t,
        wrappedkey: *const c_uchar,
        wrappedkeylen: size_t,
    ) -> c_int;
}

const_ptr_api! {
    extern "C" {
        pub fn EVP_PKCS82PKEY(p8: *const PKCS8_PRIV_KEY_INFO) -> *mut EVP_PKEY;
        pub fn EVP_PKEY2PKCS8(pkey: #[const_ptr_if(any(ossl300))] EVP_PKEY) -> *mut PKCS8_PRIV_KEY_INFO;
    }
}

cfg_if! {
    if #[cfg(any(ossl111, libressl370))] {
        extern "C" {
            pub fn EVP_PKEY_get_raw_public_key(
                pkey: *const EVP_PKEY,
                ppub: *mut c_uchar,
                len: *mut size_t,
            ) -> c_int;
            pub fn EVP_PKEY_new_raw_public_key(
                ttype: c_int,
                e: *mut ENGINE,
                key: *const c_uchar,
                keylen: size_t,
            ) -> *mut EVP_PKEY;
            pub fn EVP_PKEY_get_raw_private_key(
                pkey: *const EVP_PKEY,
                ppriv: *mut c_uchar,
                len: *mut size_t,
            ) -> c_int;
            pub fn EVP_PKEY_new_raw_private_key(
                ttype: c_int,
                e: *mut ENGINE,
                key: *const c_uchar,
                keylen: size_t,
            ) -> *mut EVP_PKEY;
        }
    }
}

cfg_if! {
    if #[cfg(ossl300)] {
        extern "C" {
            pub fn EVP_PKEY_new_raw_private_key_ex(
                ctx: *mut OSSL_LIB_CTX,
                keytype: *const c_char,
                propq: *const c_char,
                key: *const c_uchar,
                keylen: size_t
            ) -> *mut EVP_PKEY;
            pub fn EVP_PKEY_new_raw_public_key_ex(
                ctx: *mut OSSL_LIB_CTX,
                keytype: *const c_char,
                propq: *const c_char,
                key: *const c_uchar,
                keylen: size_t
            ) -> *mut EVP_PKEY;
            pub fn EVP_PKEY_is_a(
                pkey: *const EVP_PKEY,
                name: *const c_char
            ) -> c_int;
        }
    }
}

extern "C" {
    pub fn EVP_EncodeBlock(dst: *mut c_uchar, src: *const c_uchar, src_len: c_int) -> c_int;
    pub fn EVP_DecodeBlock(dst: *mut c_uchar, src: *const c_uchar, src_len: c_int) -> c_int;
}

cfg_if! {
    if #[cfg(ossl300)] {
        extern "C" {
            pub fn EVP_PKEY_gettable_params(pkey: *const EVP_PKEY) -> *const OSSL_PARAM;
            pub fn EVP_PKEY_get_params(pkey: *const EVP_PKEY, params: *mut OSSL_PARAM) -> c_int;
            pub fn EVP_PKEY_get_int_param(
                pkey: *const EVP_PKEY,
                key_name: *const c_char,
                out: *mut c_int,
            ) -> c_int;
            pub fn EVP_PKEY_get_size_t_param(
                pkey: *const EVP_PKEY,
                key_name: *const c_char,
                out: *mut size_t,
            ) -> c_int;
            pub fn EVP_PKEY_get_bn_param(
                pkey: *const EVP_PKEY,
                key_name: *const c_char,
                out: *mut *mut BIGNUM,
            ) -> c_int;
            pub fn EVP_PKEY_get_utf8_string_param(
                pkey: *const EVP_PKEY,
                key_name: *const c_char,
                str: *mut c_char,
                max_buf_sz: size_t,
                out_len: *mut size_t,
            ) -> c_int;
            pub fn EVP_PKEY_get_octet_string_param(
                pkey: *const EVP_PKEY,
                key_name: *const c_char,
                buf: *mut c_uchar,
                max_buf_sz: size_t,
                out_len: *mut size_t,
            ) -> c_int;

            pub fn EVP_PKEY_settable_params(pkey: *const EVP_PKEY) -> *const OSSL_PARAM;
            pub fn EVP_PKEY_set_params(pkey: *mut EVP_PKEY, params: *mut OSSL_PARAM) -> c_int;
            pub fn EVP_PKEY_set_int_param(
                pkey: *mut EVP_PKEY,
                key_name: *const c_char,
                in_val: c_int,
            ) -> c_int;
            pub fn EVP_PKEY_set_size_t_param(
                pkey: *mut EVP_PKEY,
                key_name: *const c_char,
                in_val: size_t,
            ) -> c_int;
            pub fn EVP_PKEY_set_bn_param(
                pkey: *mut EVP_PKEY,
                key_name: *const c_char,
                bn: *const BIGNUM,
            ) -> c_int;
            pub fn EVP_PKEY_set_utf8_string_param(
                pkey: *mut EVP_PKEY,
                key_name: *const c_char,
                str: *const c_char,
            ) -> c_int;
            pub fn EVP_PKEY_set_octet_string_param(
                pkey: *mut EVP_PKEY,
                key_name: *const c_char,
                buf: *const c_uchar,
                bsize: size_t,
            ) -> c_int;
            pub fn EVP_SIGNATURE_free(s: *mut EVP_SIGNATURE);
            pub fn EVP_SIGNATURE_up_ref(s: *mut EVP_SIGNATURE) -> c_int;
            pub fn EVP_SIGNATURE_fetch(ctx: *mut OSSL_LIB_CTX,
                                       algorithm: *const c_char,
                                       properties: *const c_char)
                                       -> *mut EVP_SIGNATURE;
            pub fn EVP_SIGNATURE_get0_name(s: *const EVP_SIGNATURE) -> *const c_char;
            pub fn EVP_SIGNATURE_get0_description(s: *const EVP_SIGNATURE) -> *const c_char;
        }
    }
}

cfg_if! {
    if #[cfg(ossl300)] {
        extern "C" {
            pub fn EVP_MAC_fetch(libctx: *mut OSSL_LIB_CTX, algorithm: *const c_char, properties: *const c_char) -> *mut EVP_MAC;
            pub fn EVP_MAC_up_ref(mac: *mut EVP_MAC) -> c_int;
            pub fn EVP_MAC_free(mac: *mut EVP_MAC);
            pub fn EVP_MAC_is_a(mac: *const EVP_MAC, name: *const c_char) -> c_int;
            pub fn EVP_MAC_get0_name(mac: *const EVP_MAC) -> *const c_char;
            pub fn EVP_MAC_get0_description(mac: *const EVP_MAC) -> *const c_char;
            pub fn EVP_MAC_get0_provider(mac: *const EVP_MAC) -> *const OSSL_PROVIDER;
            pub fn EVP_MAC_get_params(mac: *mut EVP_MAC, params: *mut OSSL_PARAM) -> c_int;

            pub fn EVP_MAC_CTX_new(mac: *mut EVP_MAC) -> *mut EVP_MAC_CTX;
            pub fn EVP_MAC_CTX_free(ctx: *mut EVP_MAC_CTX);
            pub fn EVP_MAC_CTX_dup(src: *const EVP_MAC_CTX) -> *mut EVP_MAC_CTX;
            pub fn EVP_MAC_CTX_get0_mac(ctx: *mut EVP_MAC_CTX) -> *mut EVP_MAC;
            pub fn EVP_MAC_CTX_get_params(ctx: *mut EVP_MAC_CTX, params: *mut OSSL_PARAM) -> c_int;
            pub fn EVP_MAC_CTX_set_params(ctx: *mut EVP_MAC_CTX, params: *const OSSL_PARAM) -> c_int;

            pub fn EVP_MAC_CTX_get_mac_size(ctx: *mut EVP_MAC_CTX) -> size_t;
            pub fn EVP_MAC_CTX_get_block_size(ctx: *mut EVP_MAC_CTX) -> size_t;

            pub fn EVP_MAC_init(ctx: *mut EVP_MAC_CTX, key: *const c_uchar, keylen: size_t, params: *const OSSL_PARAM) -> c_int;
            pub fn EVP_MAC_update(ctx: *mut EVP_MAC_CTX, data: *const c_uchar, datalen: size_t) -> c_int;
            pub fn EVP_MAC_final(ctx: *mut EVP_MAC_CTX, out: *mut c_uchar, outl: *mut size_t, outsize: size_t) -> c_int;
            pub fn EVP_MAC_finalXOF(ctx: *mut EVP_MAC_CTX, out: *mut c_uchar, outsize: size_t) -> c_int;

            pub fn EVP_MAC_gettable_params(mac: *const EVP_MAC) -> *const OSSL_PARAM;
            pub fn EVP_MAC_gettable_ctx_params(mac: *const EVP_MAC) -> *const OSSL_PARAM;
            pub fn EVP_MAC_settable_ctx_params(mac: *const EVP_MAC) -> *const OSSL_PARAM;
            pub fn EVP_MAC_CTX_gettable_params(ctx: *mut EVP_MAC_CTX) -> *const OSSL_PARAM;
            pub fn EVP_MAC_CTX_settable_params(ctx: *mut EVP_MAC_CTX) -> *const OSSL_PARAM;
        }
    }
}
