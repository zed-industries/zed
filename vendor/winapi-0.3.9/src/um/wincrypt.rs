// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Cryptographic API Prototypes and Definitions
use ctypes::{c_int, c_uchar, c_uint, c_void};
use shared::basetsd::ULONG_PTR;
use shared::bcrypt::BCRYPT_KEY_HANDLE;
use shared::guiddef::{GUID, LPCGUID};
use shared::minwindef::{
    BOOL, BYTE, DWORD, FALSE, FILETIME, HKEY, HMODULE, LPFILETIME, LPVOID, PBYTE, PDWORD,
    PFILETIME, TRUE, ULONG, WORD,
};
use um::minwinbase::PSYSTEMTIME;
use um::ncrypt::NCRYPT_KEY_HANDLE;
use um::winnt::{
    CHAR, HANDLE, HRESULT, LONG, LPCSTR, LPCWSTR, LPSTR, LPWSTR, PCWSTR, PVOID, PWSTR, WCHAR,
};
use vc::vcruntime::size_t;
//108
#[inline]
pub fn GET_ALG_CLASS(x: ALG_ID) -> ALG_ID {
    x & (7 << 13)
}
#[inline]
pub fn GET_ALG_TYPE(x: ALG_ID) -> ALG_ID {
    x & (15 << 9)
}
#[inline]
pub fn GET_ALG_SID(x: ALG_ID) -> ALG_ID {
    x & 511
}
pub const ALG_CLASS_ANY: ALG_ID = 0;
pub const ALG_CLASS_SIGNATURE: ALG_ID = 1 << 13;
pub const ALG_CLASS_MSG_ENCRYPT: ALG_ID = 2 << 13;
pub const ALG_CLASS_DATA_ENCRYPT: ALG_ID = 3 << 13;
pub const ALG_CLASS_HASH: ALG_ID = 4 << 13;
pub const ALG_CLASS_KEY_EXCHANGE: ALG_ID = 5 << 13;
pub const ALG_CLASS_ALL: ALG_ID = 7 << 13;
pub const ALG_TYPE_ANY: ALG_ID = 0;
pub const ALG_TYPE_DSS: ALG_ID = 1 << 9;
pub const ALG_TYPE_RSA: ALG_ID = 2 << 9;
pub const ALG_TYPE_BLOCK: ALG_ID = 3 << 9;
pub const ALG_TYPE_STREAM: ALG_ID = 4 << 9;
pub const ALG_TYPE_DH: ALG_ID = 5 << 9;
pub const ALG_TYPE_SECURECHANNEL: ALG_ID = 6 << 9;
pub const ALG_TYPE_ECDH: ALG_ID = 7 << 9;
pub const ALG_TYPE_THIRDPARTY: ALG_ID = 8 << 9;
pub const ALG_SID_ANY: ALG_ID = 0;
pub const ALG_SID_THIRDPARTY_ANY: ALG_ID = 0;
pub const ALG_SID_RSA_ANY: ALG_ID = 0;
pub const ALG_SID_RSA_PKCS: ALG_ID = 1;
pub const ALG_SID_RSA_MSATWORK: ALG_ID = 2;
pub const ALG_SID_RSA_ENTRUST: ALG_ID = 3;
pub const ALG_SID_RSA_PGP: ALG_ID = 4;
pub const ALG_SID_DSS_ANY: ALG_ID = 0;
pub const ALG_SID_DSS_PKCS: ALG_ID = 1;
pub const ALG_SID_DSS_DMS: ALG_ID = 2;
pub const ALG_SID_ECDSA: ALG_ID = 3;
pub const ALG_SID_DES: ALG_ID = 1;
pub const ALG_SID_3DES: ALG_ID = 3;
pub const ALG_SID_DESX: ALG_ID = 4;
pub const ALG_SID_IDEA: ALG_ID = 5;
pub const ALG_SID_CAST: ALG_ID = 6;
pub const ALG_SID_SAFERSK64: ALG_ID = 7;
pub const ALG_SID_SAFERSK128: ALG_ID = 8;
pub const ALG_SID_3DES_112: ALG_ID = 9;
pub const ALG_SID_CYLINK_MEK: ALG_ID = 12;
pub const ALG_SID_RC5: ALG_ID = 13;
pub const ALG_SID_AES_128: ALG_ID = 14;
pub const ALG_SID_AES_192: ALG_ID = 15;
pub const ALG_SID_AES_256: ALG_ID = 16;
pub const ALG_SID_AES: ALG_ID = 17;
pub const ALG_SID_SKIPJACK: ALG_ID = 10;
pub const ALG_SID_TEK: ALG_ID = 11;
pub const CRYPT_MODE_CBCI: ALG_ID = 6;
pub const CRYPT_MODE_CFBP: ALG_ID = 7;
pub const CRYPT_MODE_OFBP: ALG_ID = 8;
pub const CRYPT_MODE_CBCOFM: ALG_ID = 9;
pub const CRYPT_MODE_CBCOFMI: ALG_ID = 10;
pub const ALG_SID_RC2: ALG_ID = 2;
pub const ALG_SID_RC4: ALG_ID = 1;
pub const ALG_SID_SEAL: ALG_ID = 2;
pub const ALG_SID_DH_SANDF: ALG_ID = 1;
pub const ALG_SID_DH_EPHEM: ALG_ID = 2;
pub const ALG_SID_AGREED_KEY_ANY: ALG_ID = 3;
pub const ALG_SID_KEA: ALG_ID = 4;
pub const ALG_SID_ECDH: ALG_ID = 5;
pub const ALG_SID_ECDH_EPHEM: ALG_ID = 6;
pub const ALG_SID_MD2: ALG_ID = 1;
pub const ALG_SID_MD4: ALG_ID = 2;
pub const ALG_SID_MD5: ALG_ID = 3;
pub const ALG_SID_SHA: ALG_ID = 4;
pub const ALG_SID_SHA1: ALG_ID = 4;
pub const ALG_SID_MAC: ALG_ID = 5;
pub const ALG_SID_RIPEMD: ALG_ID = 6;
pub const ALG_SID_RIPEMD160: ALG_ID = 7;
pub const ALG_SID_SSL3SHAMD5: ALG_ID = 8;
pub const ALG_SID_HMAC: ALG_ID = 9;
pub const ALG_SID_TLS1PRF: ALG_ID = 10;
pub const ALG_SID_HASH_REPLACE_OWF: ALG_ID = 11;
pub const ALG_SID_SHA_256: ALG_ID = 12;
pub const ALG_SID_SHA_384: ALG_ID = 13;
pub const ALG_SID_SHA_512: ALG_ID = 14;
pub const ALG_SID_SSL3_MASTER: ALG_ID = 1;
pub const ALG_SID_SCHANNEL_MASTER_HASH: ALG_ID = 2;
pub const ALG_SID_SCHANNEL_MAC_KEY: ALG_ID = 3;
pub const ALG_SID_PCT1_MASTER: ALG_ID = 4;
pub const ALG_SID_SSL2_MASTER: ALG_ID = 5;
pub const ALG_SID_TLS1_MASTER: ALG_ID = 6;
pub const ALG_SID_SCHANNEL_ENC_KEY: ALG_ID = 7;
pub const ALG_SID_ECMQV: ALG_ID = 1;
pub const ALG_SID_EXAMPLE: ALG_ID = 80;
pub type ALG_ID = c_uint;
pub const CALG_MD2: ALG_ID = ALG_CLASS_HASH | ALG_TYPE_ANY | ALG_SID_MD2;
pub const CALG_MD4: ALG_ID = ALG_CLASS_HASH | ALG_TYPE_ANY | ALG_SID_MD4;
pub const CALG_MD5: ALG_ID = ALG_CLASS_HASH | ALG_TYPE_ANY | ALG_SID_MD5;
pub const CALG_SHA: ALG_ID = ALG_CLASS_HASH | ALG_TYPE_ANY | ALG_SID_SHA;
pub const CALG_SHA1: ALG_ID = ALG_CLASS_HASH | ALG_TYPE_ANY | ALG_SID_SHA1;
pub const CALG_MAC: ALG_ID = ALG_CLASS_HASH | ALG_TYPE_ANY | ALG_SID_MAC;
pub const CALG_RSA_SIGN: ALG_ID = ALG_CLASS_SIGNATURE | ALG_TYPE_RSA | ALG_SID_RSA_ANY;
pub const CALG_DSS_SIGN: ALG_ID = ALG_CLASS_SIGNATURE | ALG_TYPE_DSS | ALG_SID_DSS_ANY;
pub const CALG_NO_SIGN: ALG_ID = ALG_CLASS_SIGNATURE | ALG_TYPE_ANY | ALG_SID_ANY;
pub const CALG_RSA_KEYX: ALG_ID = ALG_CLASS_KEY_EXCHANGE | ALG_TYPE_RSA | ALG_SID_RSA_ANY;
pub const CALG_DES: ALG_ID = ALG_CLASS_DATA_ENCRYPT | ALG_TYPE_BLOCK | ALG_SID_DES;
pub const CALG_3DES_112: ALG_ID = ALG_CLASS_DATA_ENCRYPT | ALG_TYPE_BLOCK | ALG_SID_3DES_112;
pub const CALG_3DES: ALG_ID = ALG_CLASS_DATA_ENCRYPT | ALG_TYPE_BLOCK | ALG_SID_3DES;
pub const CALG_DESX: ALG_ID = ALG_CLASS_DATA_ENCRYPT | ALG_TYPE_BLOCK | ALG_SID_DESX;
pub const CALG_RC2: ALG_ID = ALG_CLASS_DATA_ENCRYPT | ALG_TYPE_BLOCK | ALG_SID_RC2;
pub const CALG_RC4: ALG_ID = ALG_CLASS_DATA_ENCRYPT | ALG_TYPE_STREAM | ALG_SID_RC4;
pub const CALG_SEAL: ALG_ID = ALG_CLASS_DATA_ENCRYPT | ALG_TYPE_STREAM | ALG_SID_SEAL;
pub const CALG_DH_SF: ALG_ID = ALG_CLASS_KEY_EXCHANGE | ALG_TYPE_DH | ALG_SID_DH_SANDF;
pub const CALG_DH_EPHEM: ALG_ID = ALG_CLASS_KEY_EXCHANGE | ALG_TYPE_DH | ALG_SID_DH_EPHEM;
pub const CALG_AGREEDKEY_ANY: ALG_ID = ALG_CLASS_KEY_EXCHANGE | ALG_TYPE_DH
    | ALG_SID_AGREED_KEY_ANY;
pub const CALG_KEA_KEYX: ALG_ID = ALG_CLASS_KEY_EXCHANGE | ALG_TYPE_DH | ALG_SID_KEA;
pub const CALG_HUGHES_MD5: ALG_ID = ALG_CLASS_KEY_EXCHANGE | ALG_TYPE_ANY | ALG_SID_MD5;
pub const CALG_SKIPJACK: ALG_ID = ALG_CLASS_DATA_ENCRYPT | ALG_TYPE_BLOCK | ALG_SID_SKIPJACK;
pub const CALG_TEK: ALG_ID = ALG_CLASS_DATA_ENCRYPT | ALG_TYPE_BLOCK | ALG_SID_TEK;
pub const CALG_CYLINK_MEK: ALG_ID = ALG_CLASS_DATA_ENCRYPT | ALG_TYPE_BLOCK | ALG_SID_CYLINK_MEK;
pub const CALG_SSL3_SHAMD5: ALG_ID = ALG_CLASS_HASH | ALG_TYPE_ANY | ALG_SID_SSL3SHAMD5;
pub const CALG_SSL3_MASTER: ALG_ID = ALG_CLASS_MSG_ENCRYPT | ALG_TYPE_SECURECHANNEL
    | ALG_SID_SSL3_MASTER;
pub const CALG_SCHANNEL_MASTER_HASH: ALG_ID = ALG_CLASS_MSG_ENCRYPT | ALG_TYPE_SECURECHANNEL
    | ALG_SID_SCHANNEL_MASTER_HASH;
pub const CALG_SCHANNEL_MAC_KEY: ALG_ID = ALG_CLASS_MSG_ENCRYPT | ALG_TYPE_SECURECHANNEL
    | ALG_SID_SCHANNEL_MAC_KEY;
pub const CALG_SCHANNEL_ENC_KEY: ALG_ID = ALG_CLASS_MSG_ENCRYPT | ALG_TYPE_SECURECHANNEL
    | ALG_SID_SCHANNEL_ENC_KEY;
pub const CALG_PCT1_MASTER: ALG_ID = ALG_CLASS_MSG_ENCRYPT | ALG_TYPE_SECURECHANNEL
    | ALG_SID_PCT1_MASTER;
pub const CALG_SSL2_MASTER: ALG_ID = ALG_CLASS_MSG_ENCRYPT | ALG_TYPE_SECURECHANNEL
    | ALG_SID_SSL2_MASTER;
pub const CALG_TLS1_MASTER: ALG_ID = ALG_CLASS_MSG_ENCRYPT | ALG_TYPE_SECURECHANNEL
    | ALG_SID_TLS1_MASTER;
pub const CALG_RC5: ALG_ID = ALG_CLASS_DATA_ENCRYPT | ALG_TYPE_BLOCK | ALG_SID_RC5;
pub const CALG_HMAC: ALG_ID = ALG_CLASS_HASH | ALG_TYPE_ANY | ALG_SID_HMAC;
pub const CALG_TLS1PRF: ALG_ID = ALG_CLASS_HASH | ALG_TYPE_ANY | ALG_SID_TLS1PRF;
pub const CALG_HASH_REPLACE_OWF: ALG_ID = ALG_CLASS_HASH | ALG_TYPE_ANY | ALG_SID_HASH_REPLACE_OWF;
pub const CALG_AES_128: ALG_ID = ALG_CLASS_DATA_ENCRYPT | ALG_TYPE_BLOCK | ALG_SID_AES_128;
pub const CALG_AES_192: ALG_ID = ALG_CLASS_DATA_ENCRYPT | ALG_TYPE_BLOCK | ALG_SID_AES_192;
pub const CALG_AES_256: ALG_ID = ALG_CLASS_DATA_ENCRYPT | ALG_TYPE_BLOCK | ALG_SID_AES_256;
pub const CALG_AES: ALG_ID = ALG_CLASS_DATA_ENCRYPT | ALG_TYPE_BLOCK | ALG_SID_AES;
pub const CALG_SHA_256: ALG_ID = ALG_CLASS_HASH | ALG_TYPE_ANY | ALG_SID_SHA_256;
pub const CALG_SHA_384: ALG_ID = ALG_CLASS_HASH | ALG_TYPE_ANY | ALG_SID_SHA_384;
pub const CALG_SHA_512: ALG_ID = ALG_CLASS_HASH | ALG_TYPE_ANY | ALG_SID_SHA_512;
pub const CALG_ECDH: ALG_ID = ALG_CLASS_KEY_EXCHANGE | ALG_TYPE_DH | ALG_SID_ECDH;
pub const CALG_ECDH_EPHEM: ALG_ID = ALG_CLASS_KEY_EXCHANGE | ALG_TYPE_ECDH | ALG_SID_ECDH_EPHEM;
pub const CALG_ECMQV: ALG_ID = ALG_CLASS_KEY_EXCHANGE | ALG_TYPE_ANY | ALG_SID_ECMQV;
pub const CALG_ECDSA: ALG_ID = ALG_CLASS_SIGNATURE | ALG_TYPE_DSS | ALG_SID_ECDSA;
pub const CALG_NULLCIPHER: ALG_ID = ALG_CLASS_DATA_ENCRYPT | ALG_TYPE_ANY | 0;
pub const CALG_THIRDPARTY_KEY_EXCHANGE: ALG_ID = ALG_CLASS_KEY_EXCHANGE | ALG_TYPE_THIRDPARTY
    | ALG_SID_THIRDPARTY_ANY;
pub const CALG_THIRDPARTY_SIGNATURE: ALG_ID = ALG_CLASS_SIGNATURE | ALG_TYPE_THIRDPARTY
    | ALG_SID_THIRDPARTY_ANY;
pub const CALG_THIRDPARTY_CIPHER: ALG_ID = ALG_CLASS_DATA_ENCRYPT | ALG_TYPE_THIRDPARTY
    | ALG_SID_THIRDPARTY_ANY;
pub const CALG_THIRDPARTY_HASH: ALG_ID = ALG_CLASS_HASH | ALG_TYPE_THIRDPARTY
    | ALG_SID_THIRDPARTY_ANY;
pub type HCRYPTPROV = ULONG_PTR;
pub type HCRYPTKEY = ULONG_PTR;
pub type HCRYPTHASH = ULONG_PTR;
pub const CRYPT_VERIFYCONTEXT: DWORD = 0xF0000000;
pub const CRYPT_NEWKEYSET: DWORD = 0x00000008;
pub const CRYPT_DELETEKEYSET: DWORD = 0x00000010;
pub const CRYPT_MACHINE_KEYSET: DWORD = 0x00000020;
pub const CRYPT_SILENT: DWORD = 0x00000040;
pub const CRYPT_DEFAULT_CONTAINER_OPTIONAL: DWORD = 0x00000080;
pub const CRYPT_EXPORTABLE: DWORD = 0x00000001;
pub const CRYPT_USER_PROTECTED: DWORD = 0x00000002;
pub const CRYPT_CREATE_SALT: DWORD = 0x00000004;
pub const CRYPT_UPDATE_KEY: DWORD = 0x00000008;
pub const CRYPT_NO_SALT: DWORD = 0x00000010;
pub const CRYPT_PREGEN: DWORD = 0x00000040;
pub const CRYPT_RECIPIENT: DWORD = 0x00000010;
pub const CRYPT_INITIATOR: DWORD = 0x00000040;
pub const CRYPT_ONLINE: DWORD = 0x00000080;
pub const CRYPT_SF: DWORD = 0x00000100;
pub const CRYPT_CREATE_IV: DWORD = 0x00000200;
pub const CRYPT_KEK: DWORD = 0x00000400;
pub const CRYPT_DATA_KEY: DWORD = 0x00000800;
pub const CRYPT_VOLATILE: DWORD = 0x00001000;
pub const CRYPT_SGCKEY: DWORD = 0x00002000;
pub const CRYPT_USER_PROTECTED_STRONG: DWORD = 0x00100000;
pub const CRYPT_ARCHIVABLE: DWORD = 0x00004000;
pub const CRYPT_FORCE_KEY_PROTECTION_HIGH: DWORD = 0x00008000;
pub const RSA1024BIT_KEY: DWORD = 0x04000000;
pub const CRYPT_SERVER: DWORD = 0x00000400;
pub const KEY_LENGTH_MASK: DWORD = 0xFFFF0000;
pub const CRYPT_Y_ONLY: DWORD = 0x00000001;
pub const CRYPT_SSL2_FALLBACK: DWORD = 0x00000002;
pub const CRYPT_DESTROYKEY: DWORD = 0x00000004;
pub const CRYPT_OAEP: DWORD = 0x00000040;
pub const CRYPT_BLOB_VER3: DWORD = 0x00000080;
pub const CRYPT_IPSEC_HMAC_KEY: DWORD = 0x00000100;
pub const CRYPT_DECRYPT_RSA_NO_PADDING_CHECK: DWORD = 0x00000020;
pub const CRYPT_SECRETDIGEST: DWORD = 0x00000001;
pub const CRYPT_OWF_REPL_LM_HASH: DWORD = 0x00000001;
pub const CRYPT_LITTLE_ENDIAN: DWORD = 0x00000001;
pub const CRYPT_NOHASHOID: DWORD = 0x00000001;
pub const CRYPT_TYPE2_FORMAT: DWORD = 0x00000002;
pub const CRYPT_X931_FORMAT: DWORD = 0x00000004;
pub const CRYPT_MACHINE_DEFAULT: DWORD = 0x00000001;
pub const CRYPT_USER_DEFAULT: DWORD = 0x00000002;
pub const CRYPT_DELETE_DEFAULT: DWORD = 0x00000004;
pub const SIMPLEBLOB: DWORD = 0x1;
pub const PUBLICKEYBLOB: DWORD = 0x6;
pub const PRIVATEKEYBLOB: DWORD = 0x7;
pub const PLAINTEXTKEYBLOB: DWORD = 0x8;
pub const OPAQUEKEYBLOB: DWORD = 0x9;
pub const PUBLICKEYBLOBEX: DWORD = 0xA;
pub const SYMMETRICWRAPKEYBLOB: DWORD = 0xB;
pub const KEYSTATEBLOB: DWORD = 0xC;
pub const AT_KEYEXCHANGE: DWORD = 1;
pub const AT_SIGNATURE: DWORD = 2;
pub const CRYPT_USERDATA: DWORD = 1;
pub const KP_IV: DWORD = 1;
pub const KP_SALT: DWORD = 2;
pub const KP_PADDING: DWORD = 3;
pub const KP_MODE: DWORD = 4;
pub const KP_MODE_BITS: DWORD = 5;
pub const KP_PERMISSIONS: DWORD = 6;
pub const KP_ALGID: DWORD = 7;
pub const KP_BLOCKLEN: DWORD = 8;
pub const KP_KEYLEN: DWORD = 9;
pub const KP_SALT_EX: DWORD = 10;
pub const KP_P: DWORD = 11;
pub const KP_G: DWORD = 12;
pub const KP_Q: DWORD = 13;
pub const KP_X: DWORD = 14;
pub const KP_Y: DWORD = 15;
pub const KP_RA: DWORD = 16;
pub const KP_RB: DWORD = 17;
pub const KP_INFO: DWORD = 18;
pub const KP_EFFECTIVE_KEYLEN: DWORD = 19;
pub const KP_SCHANNEL_ALG: DWORD = 20;
pub const KP_CLIENT_RANDOM: DWORD = 21;
pub const KP_SERVER_RANDOM: DWORD = 22;
pub const KP_RP: DWORD = 23;
pub const KP_PRECOMP_MD5: DWORD = 24;
pub const KP_PRECOMP_SHA: DWORD = 25;
pub const KP_CERTIFICATE: DWORD = 26;
pub const KP_CLEAR_KEY: DWORD = 27;
pub const KP_PUB_EX_LEN: DWORD = 28;
pub const KP_PUB_EX_VAL: DWORD = 29;
pub const KP_KEYVAL: DWORD = 30;
pub const KP_ADMIN_PIN: DWORD = 31;
pub const KP_KEYEXCHANGE_PIN: DWORD = 32;
pub const KP_SIGNATURE_PIN: DWORD = 33;
pub const KP_PREHASH: DWORD = 34;
pub const KP_ROUNDS: DWORD = 35;
pub const KP_OAEP_PARAMS: DWORD = 36;
pub const KP_CMS_KEY_INFO: DWORD = 37;
pub const KP_CMS_DH_KEY_INFO: DWORD = 38;
pub const KP_PUB_PARAMS: DWORD = 39;
pub const KP_VERIFY_PARAMS: DWORD = 40;
pub const KP_HIGHEST_VERSION: DWORD = 41;
pub const KP_GET_USE_COUNT: DWORD = 42;
pub const KP_PIN_ID: DWORD = 43;
pub const KP_PIN_INFO: DWORD = 44;
pub const PKCS5_PADDING: DWORD = 1;
pub const RANDOM_PADDING: DWORD = 2;
pub const ZERO_PADDING: DWORD = 3;
pub const CRYPT_MODE_CBC: DWORD = 1;
pub const CRYPT_MODE_ECB: DWORD = 2;
pub const CRYPT_MODE_OFB: DWORD = 3;
pub const CRYPT_MODE_CFB: DWORD = 4;
pub const CRYPT_MODE_CTS: DWORD = 5;
pub const CRYPT_ENCRYPT: DWORD = 0x0001;
pub const CRYPT_DECRYPT: DWORD = 0x0002;
pub const CRYPT_EXPORT: DWORD = 0x0004;
pub const CRYPT_READ: DWORD = 0x0008;
pub const CRYPT_WRITE: DWORD = 0x0010;
pub const CRYPT_MAC: DWORD = 0x0020;
pub const CRYPT_EXPORT_KEY: DWORD = 0x0040;
pub const CRYPT_IMPORT_KEY: DWORD = 0x0080;
pub const CRYPT_ARCHIVE: DWORD = 0x0100;
pub const HP_ALGID: DWORD = 0x0001;
pub const HP_HASHVAL: DWORD = 0x0002;
pub const HP_HASHSIZE: DWORD = 0x0004;
pub const HP_HMAC_INFO: DWORD = 0x0005;
pub const HP_TLS1PRF_LABEL: DWORD = 0x0006;
pub const HP_TLS1PRF_SEED: DWORD = 0x0007;
pub const CRYPT_FAILED: BOOL = FALSE;
pub const CRYPT_SUCCEED: BOOL = TRUE;
#[inline]
pub fn RCRYPT_SUCCEEDED(rt: BOOL) -> bool {
    rt == CRYPT_SUCCEED
}
#[inline]
pub fn RCRYPT_FAILED(rt: BOOL) -> bool {
    rt == CRYPT_FAILED
}
pub const PP_ENUMALGS: DWORD = 1;
pub const PP_ENUMCONTAINERS: DWORD = 2;
pub const PP_IMPTYPE: DWORD = 3;
pub const PP_NAME: DWORD = 4;
pub const PP_VERSION: DWORD = 5;
pub const PP_CONTAINER: DWORD = 6;
pub const PP_CHANGE_PASSWORD: DWORD = 7;
pub const PP_KEYSET_SEC_DESCR: DWORD = 8;
pub const PP_CERTCHAIN: DWORD = 9;
pub const PP_KEY_TYPE_SUBTYPE: DWORD = 10;
pub const PP_PROVTYPE: DWORD = 16;
pub const PP_KEYSTORAGE: DWORD = 17;
pub const PP_APPLI_CERT: DWORD = 18;
pub const PP_SYM_KEYSIZE: DWORD = 19;
pub const PP_SESSION_KEYSIZE: DWORD = 20;
pub const PP_UI_PROMPT: DWORD = 21;
pub const PP_ENUMALGS_EX: DWORD = 22;
pub const PP_ENUMMANDROOTS: DWORD = 25;
pub const PP_ENUMELECTROOTS: DWORD = 26;
pub const PP_KEYSET_TYPE: DWORD = 27;
pub const PP_ADMIN_PIN: DWORD = 31;
pub const PP_KEYEXCHANGE_PIN: DWORD = 32;
pub const PP_SIGNATURE_PIN: DWORD = 33;
pub const PP_SIG_KEYSIZE_INC: DWORD = 34;
pub const PP_KEYX_KEYSIZE_INC: DWORD = 35;
pub const PP_UNIQUE_CONTAINER: DWORD = 36;
pub const PP_SGC_INFO: DWORD = 37;
pub const PP_USE_HARDWARE_RNG: DWORD = 38;
pub const PP_KEYSPEC: DWORD = 39;
pub const PP_ENUMEX_SIGNING_PROT: DWORD = 40;
pub const PP_CRYPT_COUNT_KEY_USE: DWORD = 41;
pub const PP_USER_CERTSTORE: DWORD = 42;
pub const PP_SMARTCARD_READER: DWORD = 43;
pub const PP_SMARTCARD_GUID: DWORD = 45;
pub const PP_ROOT_CERTSTORE: DWORD = 46;
pub const PP_SMARTCARD_READER_ICON: DWORD = 47;
pub const CRYPT_FIRST: DWORD = 1;
pub const CRYPT_NEXT: DWORD = 2;
pub const CRYPT_SGC_ENUM: DWORD = 4;
pub const CRYPT_IMPL_HARDWARE: DWORD = 1;
pub const CRYPT_IMPL_SOFTWARE: DWORD = 2;
pub const CRYPT_IMPL_MIXED: DWORD = 3;
pub const CRYPT_IMPL_UNKNOWN: DWORD = 4;
pub const CRYPT_IMPL_REMOVABLE: DWORD = 8;
pub const CRYPT_SEC_DESCR: DWORD = 0x00000001;
pub const CRYPT_PSTORE: DWORD = 0x00000002;
pub const CRYPT_UI_PROMPT: DWORD = 0x00000004;
pub const CRYPT_FLAG_PCT1: DWORD = 0x0001;
pub const CRYPT_FLAG_SSL2: DWORD = 0x0002;
pub const CRYPT_FLAG_SSL3: DWORD = 0x0004;
pub const CRYPT_FLAG_TLS1: DWORD = 0x0008;
pub const CRYPT_FLAG_IPSEC: DWORD = 0x0010;
pub const CRYPT_FLAG_SIGNING: DWORD = 0x0020;
pub const CRYPT_SGC: DWORD = 0x0001;
pub const CRYPT_FASTSGC: DWORD = 0x0002;
pub const PP_CLIENT_HWND: DWORD = 1;
pub const PP_CONTEXT_INFO: DWORD = 11;
pub const PP_KEYEXCHANGE_KEYSIZE: DWORD = 12;
pub const PP_SIGNATURE_KEYSIZE: DWORD = 13;
pub const PP_KEYEXCHANGE_ALG: DWORD = 14;
pub const PP_SIGNATURE_ALG: DWORD = 15;
pub const PP_DELETEKEY: DWORD = 24;
pub const PP_PIN_PROMPT_STRING: DWORD = 44;
pub const PP_SECURE_KEYEXCHANGE_PIN: DWORD = 47;
pub const PP_SECURE_SIGNATURE_PIN: DWORD = 48;
pub const PROV_RSA_FULL: DWORD = 1;
pub const PROV_RSA_SIG: DWORD = 2;
pub const PROV_DSS: DWORD = 3;
pub const PROV_FORTEZZA: DWORD = 4;
pub const PROV_MS_EXCHANGE: DWORD = 5;
pub const PROV_SSL: DWORD = 6;
pub const PROV_RSA_SCHANNEL: DWORD = 12;
pub const PROV_DSS_DH: DWORD = 13;
pub const PROV_EC_ECDSA_SIG: DWORD = 14;
pub const PROV_EC_ECNRA_SIG: DWORD = 15;
pub const PROV_EC_ECDSA_FULL: DWORD = 16;
pub const PROV_EC_ECNRA_FULL: DWORD = 17;
pub const PROV_DH_SCHANNEL: DWORD = 18;
pub const PROV_SPYRUS_LYNKS: DWORD = 20;
pub const PROV_RNG: DWORD = 21;
pub const PROV_INTEL_SEC: DWORD = 22;
pub const PROV_REPLACE_OWF: DWORD = 23;
pub const PROV_RSA_AES: DWORD = 24;
pub const MS_DEF_PROV: &'static str = "Microsoft Base Cryptographic Provider v1.0";
pub const MS_ENHANCED_PROV: &'static str = "Microsoft Enhanced Cryptographic Provider v1.0";
pub const MS_STRONG_PROV: &'static str = "Microsoft Strong Cryptographic Provider";
pub const MS_DEF_RSA_SIG_PROV: &'static str = "Microsoft RSA Signature Cryptographic Provider";
pub const MS_DEF_RSA_SCHANNEL_PROV: &'static str = "Microsoft RSA SChannel Cryptographic Provider";
pub const MS_DEF_DSS_PROV: &'static str = "Microsoft Base DSS Cryptographic Provider";
pub const MS_DEF_DSS_DH_PROV: &'static str
    = "Microsoft Base DSS and Diffie-Hellman Cryptographic Provider";
pub const MS_ENH_DSS_DH_PROV: &'static str
    = "Microsoft Enhanced DSS and Diffie-Hellman Cryptographic Provider";
pub const MS_DEF_DH_SCHANNEL_PROV: &'static str = "Microsoft DH SChannel Cryptographic Provider";
pub const MS_SCARD_PROV: &'static str = "Microsoft Base Smart Card Crypto Provider";
pub const MS_ENH_RSA_AES_PROV: &'static str
    = "Microsoft Enhanced RSA and AES Cryptographic Provider";
pub const MS_ENH_RSA_AES_PROV_XP: &'static str
    = "Microsoft Enhanced RSA and AES Cryptographic Provider (Prototype)";
pub const MAXUIDLEN: usize = 64;
pub const EXPO_OFFLOAD_REG_VALUE: &'static str = "ExpoOffload";
pub const EXPO_OFFLOAD_FUNC_NAME: &'static str = "OffloadModExpo";
pub const szKEY_CRYPTOAPI_PRIVATE_KEY_OPTIONS: &'static str
    = "Software\\Policies\\Microsoft\\Cryptography";
pub const szKEY_CACHE_ENABLED: &'static str = "CachePrivateKeys";
pub const szKEY_CACHE_SECONDS: &'static str = "PrivateKeyLifetimeSeconds";
pub const szPRIV_KEY_CACHE_MAX_ITEMS: &'static str = "PrivKeyCacheMaxItems";
pub const cPRIV_KEY_CACHE_MAX_ITEMS_DEFAULT: DWORD = 20;
pub const szPRIV_KEY_CACHE_PURGE_INTERVAL_SECONDS: &'static str
    = "PrivKeyCachePurgeIntervalSeconds";
pub const cPRIV_KEY_CACHE_PURGE_INTERVAL_SECONDS_DEFAULT: DWORD = 86400;
pub const CUR_BLOB_VERSION: DWORD = 2;
STRUCT!{struct CMS_KEY_INFO {
    dwVersion: DWORD,
    Algid: ALG_ID,
    pbOID: *mut BYTE,
    cbOID: DWORD,
}}
pub type PCMS_KEY_INFO = *mut CMS_KEY_INFO;
STRUCT!{struct HMAC_INFO {
    HashAlgid: ALG_ID,
    pbInnerString: *mut BYTE,
    cbInnerString: DWORD,
    pbOuterString: *mut BYTE,
    cbOuterString: DWORD,
}}
pub type PHMAC_INFO = *mut HMAC_INFO;
STRUCT!{struct SCHANNEL_ALG {
    dwUse: DWORD,
    Algid: ALG_ID,
    cBits: DWORD,
    dwFlags: DWORD,
    dwReserved: DWORD,
}}
pub type PSCHANNEL_ALG = *mut SCHANNEL_ALG;
pub const SCHANNEL_MAC_KEY: DWORD = 0x00000000;
pub const SCHANNEL_ENC_KEY: DWORD = 0x00000001;
pub const INTERNATIONAL_USAGE: DWORD = 0x00000001;
STRUCT!{struct PROV_ENUMALGS {
    aiAlgid: ALG_ID,
    dwBitLen: DWORD,
    dwNameLen: DWORD,
    szName: [CHAR; 20],
}}
STRUCT!{struct PROV_ENUMALGS_EX {
    aiAlgid: ALG_ID,
    dwDefaultLen: DWORD,
    dwMinLen: DWORD,
    dwMaxLen: DWORD,
    dwProtocols: DWORD,
    dwNameLen: DWORD,
    szName: [CHAR; 20],
    dwLongNameLen: DWORD,
    szLongName: [CHAR; 40],
}}
STRUCT!{struct BLOBHEADER {
    bType: BYTE,
    bVersion: BYTE,
    reserved: WORD,
    aiKeyAlg: ALG_ID,
}}
pub type PUBLICKEYSTRUC = BLOBHEADER;
STRUCT!{struct RSAPUBKEY {
    magic: DWORD,
    bitlen: DWORD,
    pubexp: DWORD,
}}
STRUCT!{struct DHPUBKEY {
    magic: DWORD,
    bitlen: DWORD,
}}
pub type DSSPUBKEY = DHPUBKEY;
pub type KEAPUBKEY = DHPUBKEY;
pub type TEKPUBKEY = DHPUBKEY;
STRUCT!{struct DSSSEED {
    counter: DWORD,
    seed: [BYTE; 20],
}}
STRUCT!{struct DHPUBKEY_VER3 {
    magic: DWORD,
    bitlenP: DWORD,
    bitlenQ: DWORD,
    bitlenJ: DWORD,
    DSSSeed: DSSSEED,
}}
pub type DSSPUBKEY_VER3 = DHPUBKEY_VER3;
STRUCT!{struct DHPRIVKEY_VER3 {
    magic: DWORD,
    bitlenP: DWORD,
    bitlenQ: DWORD,
    bitlenJ: DWORD,
    bitlenX: DWORD,
    DSSSeed: DSSSEED,
}}
pub type DSSPRIVKEY_VER3 = DHPRIVKEY_VER3;
STRUCT!{struct KEY_TYPE_SUBTYPE {
    dwKeySpec: DWORD,
    Type: GUID,
    Subtype: GUID,
}}
pub type PKEY_TYPE_SUBTYPE = *mut KEY_TYPE_SUBTYPE;
STRUCT!{struct CERT_FORTEZZA_DATA_PROP {
    SerialNumber: [c_uchar; 8],
    CertIndex: c_int,
    CertLabel: [c_uchar; 36],
}}
STRUCT!{struct CRYPT_RC4_KEY_STATE {
    Key: [c_uchar; 16],
    SBox: [c_uchar; 256],
    i: c_uchar,
    j: c_uchar,
}}
pub type PCRYPT_RC4_KEY_STATE = *mut CRYPT_RC4_KEY_STATE;
STRUCT!{struct CRYPT_DES_KEY_STATE {
    Key: [c_uchar; 8],
    IV: [c_uchar; 8],
    Feedback: [c_uchar; 8],
}}
pub type PCRYPT_DES_KEY_STATE = *mut CRYPT_DES_KEY_STATE;
STRUCT!{struct CRYPT_3DES_KEY_STATE {
    Key: [c_uchar; 24],
    IV: [c_uchar; 8],
    Feedback: [c_uchar; 8],
}}
pub type PCRYPT_3DES_KEY_STATE = *mut CRYPT_3DES_KEY_STATE;
STRUCT!{struct CRYPT_AES_128_KEY_STATE {
    Key: [c_uchar; 16],
    IV: [c_uchar; 16],
    EncryptionState: [[c_uchar; 16]; 11],
    DecryptionState: [[c_uchar; 16]; 11],
    Feedback: [c_uchar; 16],
}}
pub type PCRYPT_AES_128_KEY_STATE = *mut CRYPT_AES_128_KEY_STATE;
STRUCT!{struct CRYPT_AES_256_KEY_STATE {
    Key: [c_uchar; 32],
    IV: [c_uchar; 16],
    EncryptionState: [[c_uchar; 16]; 15],
    DecryptionState: [[c_uchar; 16]; 15],
    Feedback: [c_uchar; 16],
}}
pub type PCRYPT_AES_256_KEY_STATE = *mut CRYPT_AES_256_KEY_STATE;
STRUCT!{struct CRYPTOAPI_BLOB {
    cbData: DWORD,
    pbData: *mut BYTE,
}}
pub type CRYPT_INTEGER_BLOB = CRYPTOAPI_BLOB;
pub type PCRYPT_INTEGER_BLOB = *mut CRYPTOAPI_BLOB;
pub type CRYPT_UINT_BLOB = CRYPTOAPI_BLOB;
pub type PCRYPT_UINT_BLOB = *mut CRYPTOAPI_BLOB;
pub type CRYPT_OBJID_BLOB = CRYPTOAPI_BLOB;
pub type PCRYPT_OBJID_BLOB = *mut CRYPTOAPI_BLOB;
pub type CERT_NAME_BLOB = CRYPTOAPI_BLOB;
pub type PCERT_NAME_BLOB = *mut CRYPTOAPI_BLOB;
pub type CERT_RDN_VALUE_BLOB = CRYPTOAPI_BLOB;
pub type PCERT_RDN_VALUE_BLOB = *mut CRYPTOAPI_BLOB;
pub type CERT_BLOB = CRYPTOAPI_BLOB;
pub type PCERT_BLOB = *mut CRYPTOAPI_BLOB;
pub type CRL_BLOB = CRYPTOAPI_BLOB;
pub type PCRL_BLOB = *mut CRYPTOAPI_BLOB;
pub type DATA_BLOB = CRYPTOAPI_BLOB;
pub type PDATA_BLOB = *mut CRYPTOAPI_BLOB;
pub type CRYPT_DATA_BLOB = CRYPTOAPI_BLOB;
pub type PCRYPT_DATA_BLOB = *mut CRYPTOAPI_BLOB;
pub type CRYPT_HASH_BLOB = CRYPTOAPI_BLOB;
pub type PCRYPT_HASH_BLOB = *mut CRYPTOAPI_BLOB;
pub type CRYPT_DIGEST_BLOB = CRYPTOAPI_BLOB;
pub type PCRYPT_DIGEST_BLOB = *mut CRYPTOAPI_BLOB;
pub type CRYPT_DER_BLOB = CRYPTOAPI_BLOB;
pub type PCRYPT_DER_BLOB = *mut CRYPTOAPI_BLOB;
pub type CRYPT_ATTR_BLOB = CRYPTOAPI_BLOB;
pub type PCRYPT_ATTR_BLOB = *mut CRYPTOAPI_BLOB;
STRUCT!{struct CMS_DH_KEY_INFO {
    dwVersion: DWORD,
    Algid: ALG_ID,
    pszContentEncObjId: LPSTR,
    PubInfo: CRYPT_DATA_BLOB,
    pReserved: *mut c_void,
}}
pub type PCMS_DH_KEY_INFO = *mut CMS_DH_KEY_INFO;
extern "system" {
    pub fn CryptAcquireContextA(
        phProv: *mut HCRYPTPROV,
        szContainer: LPCSTR,
        szProvider: LPCSTR,
        dwProvType: DWORD,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn CryptAcquireContextW(
        phProv: *mut HCRYPTPROV,
        szContainer: LPCWSTR,
        szProvider: LPCWSTR,
        dwProvType: DWORD,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn CryptReleaseContext(
        hProv: HCRYPTPROV,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn CryptGenKey(
        hProv: HCRYPTPROV,
        Algid: ALG_ID,
        dwFlags: DWORD,
        phKey: *mut HCRYPTKEY,
    ) -> BOOL;
    pub fn CryptDeriveKey(
        hProv: HCRYPTPROV,
        Algid: ALG_ID,
        hBaseData: HCRYPTHASH,
        dwFlags: DWORD,
        phKey: *mut HCRYPTKEY,
    ) -> BOOL;
    pub fn CryptDestroyKey(
        hKey: HCRYPTKEY,
    ) -> BOOL;
    pub fn CryptSetKeyParam(
        hKey: HCRYPTKEY,
        dwParam: DWORD,
        pbData: *const BYTE,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn CryptGetKeyParam(
        hKey: HCRYPTKEY,
        dwParam: DWORD,
        pbData: *mut BYTE,
        pdwDataLen: *mut DWORD,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn CryptSetHashParam(
        hHash: HCRYPTHASH,
        dwParam: DWORD,
        pbData: *const BYTE,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn CryptGetHashParam(
        hHash: HCRYPTHASH,
        dwParam: DWORD,
        pbData: *mut BYTE,
        pdwDataLen: *mut DWORD,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn CryptSetProvParam(
        hProv: HCRYPTPROV,
        dwParam: DWORD,
        pbData: *const BYTE,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn CryptGetProvParam(
        hProv: HCRYPTPROV,
        dwParam: DWORD,
        pbData: *mut BYTE,
        pdwDataLen: *mut DWORD,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn CryptGenRandom(
        hProv: HCRYPTPROV,
        dwLen: DWORD,
        pbBuffer: *mut BYTE,
    ) -> BOOL;
    pub fn CryptGetUserKey(
        hProv: HCRYPTPROV,
        dwKeySpec: DWORD,
        phUserKey: *mut HCRYPTKEY,
    ) -> BOOL;
    pub fn CryptExportKey(
        hKey: HCRYPTKEY,
        hExpKey: HCRYPTKEY,
        dwBlobType: DWORD,
        dwFlags: DWORD,
        pbData: *mut BYTE,
        pdwDataLen: *mut DWORD,
    ) -> BOOL;
    pub fn CryptImportKey(
        hProv: HCRYPTPROV,
        pbData: *const BYTE,
        dwDataLen: DWORD,
        hPubKey: HCRYPTKEY,
        dwFlags: DWORD,
        phKey: *mut HCRYPTKEY,
    ) -> BOOL;
    pub fn CryptEncrypt(
        hKey: HCRYPTKEY,
        hHash: HCRYPTHASH,
        Final: BOOL,
        dwFlags: DWORD,
        pbData: *mut BYTE,
        pdwDataLen: *mut DWORD,
        dwBufLen: DWORD,
    ) -> BOOL;
    pub fn CryptDecrypt(
        hKey: HCRYPTKEY,
        hHash: HCRYPTHASH,
        Final: BOOL,
        dwFlags: DWORD,
        pbData: *mut BYTE,
        pdwDataLen: *mut DWORD,
    ) -> BOOL;
    pub fn CryptCreateHash(
        hProv: HCRYPTPROV,
        Algid: ALG_ID,
        hKey: HCRYPTKEY,
        dwFlags: DWORD,
        phHash: *mut HCRYPTHASH,
    ) -> BOOL;
    pub fn CryptHashData(
        hHash: HCRYPTHASH,
        pbData: *const BYTE,
        dwDataLen: DWORD,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn CryptHashSessionKey(
        hHash: HCRYPTHASH,
        hKey: HCRYPTKEY,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn CryptDestroyHash(
        hHash: HCRYPTHASH,
    ) -> BOOL;
    pub fn CryptSignHashA(
        hHash: HCRYPTHASH,
        dwKeySpec: DWORD,
        szDescription: LPCSTR,
        dwFlags: DWORD,
        pbSignature: *mut BYTE,
        pdwSigLen: *mut DWORD,
    ) -> BOOL;
    pub fn CryptSignHashW(
        hHash: HCRYPTHASH,
        dwKeySpec: DWORD,
        szDescription: LPCWSTR,
        dwFlags: DWORD,
        pbSignature: *mut BYTE,
        pdwSigLen: *mut DWORD,
    ) -> BOOL;
    pub fn CryptVerifySignatureA(
        hHash: HCRYPTHASH,
        pbSignature: *const BYTE,
        dwSigLen: DWORD,
        hPubKey: HCRYPTKEY,
        szDescription: LPCSTR,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn CryptVerifySignatureW(
        hHash: HCRYPTHASH,
        pbSignature: *const BYTE,
        dwSigLen: DWORD,
        hPubKey: HCRYPTKEY,
        szDescription: LPCWSTR,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn CryptSetProviderA(
        pszProvName: LPCSTR,
        dwProvType: DWORD,
    ) -> BOOL;
    pub fn CryptSetProviderW(
        pszProvName: LPCWSTR,
        dwProvType: DWORD,
    ) -> BOOL;
    pub fn CryptSetProviderExA(
        pszProvName: LPCSTR,
        dwProvType: DWORD,
        pdwReserved: *mut DWORD,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn CryptSetProviderExW(
        pszProvName: LPCWSTR,
        dwProvType: DWORD,
        pdwReserved: *mut DWORD,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn CryptGetDefaultProviderA(
        dwProvType: DWORD,
        pdwReserved: *mut DWORD,
        dwFlags: DWORD,
        pszProvName: LPSTR,
        pcbProvName: *mut DWORD,
    ) -> BOOL;
    pub fn CryptGetDefaultProviderW(
        dwProvType: DWORD,
        pdwReserved: *mut DWORD,
        dwFlags: DWORD,
        pszProvName: LPWSTR,
        pcbProvName: *mut DWORD,
    ) -> BOOL;
    pub fn CryptEnumProviderTypesA(
        dwIndex: DWORD,
        pdwReserved: *mut DWORD,
        dwFlags: DWORD,
        pdwProvType: *mut DWORD,
        szTypeName: LPSTR,
        pcbTypeName: *mut DWORD,
    ) -> BOOL;
    pub fn CryptEnumProviderTypesW(
        dwIndex: DWORD,
        pdwReserved: *mut DWORD,
        dwFlags: DWORD,
        pdwProvType: *mut DWORD,
        szTypeName: LPWSTR,
        pcbTypeName: *mut DWORD,
    ) -> BOOL;
    pub fn CryptEnumProvidersA(
        dwIndex: DWORD,
        pdwReserved: *mut DWORD,
        dwFlags: DWORD,
        pdwProvType: *mut DWORD,
        szProvName: LPSTR,
        pcbProvName: *mut DWORD,
    ) -> BOOL;
    pub fn CryptEnumProvidersW(
        dwIndex: DWORD,
        pdwReserved: *mut DWORD,
        dwFlags: DWORD,
        pdwProvType: *mut DWORD,
        szProvName: LPWSTR,
        pcbProvName: *mut DWORD,
    ) -> BOOL;
    pub fn CryptContextAddRef(
        hProv: HCRYPTPROV,
        pdwReserved: *mut DWORD,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn CryptDuplicateKey(
        hKey: HCRYPTKEY,
        pdwReserved: *mut DWORD,
        dwFlags: DWORD,
        phKey: *mut HCRYPTKEY,
    ) -> BOOL;
    pub fn CryptDuplicateHash(
        hHash: HCRYPTHASH,
        pdwReserved: *mut DWORD,
        dwFlags: DWORD,
        phHash: *mut HCRYPTHASH,
    ) -> BOOL;
}
extern "C" {
    pub fn GetEncSChannel(
        pData: *mut *mut BYTE,
        dwDecSize: *mut DWORD,
    ) -> BOOL;
}
pub type HCRYPTPROV_OR_NCRYPT_KEY_HANDLE = ULONG_PTR;
pub type HCRYPTPROV_LEGACY = ULONG_PTR;
STRUCT!{struct CRYPT_BIT_BLOB {
    cbData: DWORD,
    pbData: *mut BYTE,
    cUnusedBits: DWORD,
}}
pub type PCRYPT_BIT_BLOB = *mut CRYPT_BIT_BLOB;
STRUCT!{struct CRYPT_ALGORITHM_IDENTIFIER {
    pszObjId: LPSTR,
    Parameters: CRYPT_OBJID_BLOB,
}}
pub type PCRYPT_ALGORITHM_IDENTIFIER = *mut CRYPT_ALGORITHM_IDENTIFIER;
pub const szOID_RSA: &'static str = "1.2.840.113549";
pub const szOID_PKCS: &'static str = "1.2.840.113549.1";
pub const szOID_RSA_HASH: &'static str = "1.2.840.113549.2";
pub const szOID_RSA_ENCRYPT: &'static str = "1.2.840.113549.3";
pub const szOID_PKCS_1: &'static str = "1.2.840.113549.1.1";
pub const szOID_PKCS_2: &'static str = "1.2.840.113549.1.2";
pub const szOID_PKCS_3: &'static str = "1.2.840.113549.1.3";
pub const szOID_PKCS_4: &'static str = "1.2.840.113549.1.4";
pub const szOID_PKCS_5: &'static str = "1.2.840.113549.1.5";
pub const szOID_PKCS_6: &'static str = "1.2.840.113549.1.6";
pub const szOID_PKCS_7: &'static str = "1.2.840.113549.1.7";
pub const szOID_PKCS_8: &'static str = "1.2.840.113549.1.8";
pub const szOID_PKCS_9: &'static str = "1.2.840.113549.1.9";
pub const szOID_PKCS_10: &'static str = "1.2.840.113549.1.10";
pub const szOID_PKCS_12: &'static str = "1.2.840.113549.1.12";
pub const szOID_RSA_RSA: &'static str = "1.2.840.113549.1.1.1";
pub const szOID_RSA_MD2RSA: &'static str = "1.2.840.113549.1.1.2";
pub const szOID_RSA_MD4RSA: &'static str = "1.2.840.113549.1.1.3";
pub const szOID_RSA_MD5RSA: &'static str = "1.2.840.113549.1.1.4";
pub const szOID_RSA_SHA1RSA: &'static str = "1.2.840.113549.1.1.5";
pub const szOID_RSA_SETOAEP_RSA: &'static str = "1.2.840.113549.1.1.6";
pub const szOID_RSAES_OAEP: &'static str = "1.2.840.113549.1.1.7";
pub const szOID_RSA_MGF1: &'static str = "1.2.840.113549.1.1.8";
pub const szOID_RSA_PSPECIFIED: &'static str = "1.2.840.113549.1.1.9";
pub const szOID_RSA_SSA_PSS: &'static str = "1.2.840.113549.1.1.10";
pub const szOID_RSA_SHA256RSA: &'static str = "1.2.840.113549.1.1.11";
pub const szOID_RSA_SHA384RSA: &'static str = "1.2.840.113549.1.1.12";
pub const szOID_RSA_SHA512RSA: &'static str = "1.2.840.113549.1.1.13";
pub const szOID_RSA_DH: &'static str = "1.2.840.113549.1.3.1";
pub const szOID_RSA_data: &'static str = "1.2.840.113549.1.7.1";
pub const szOID_RSA_signedData: &'static str = "1.2.840.113549.1.7.2";
pub const szOID_RSA_envelopedData: &'static str = "1.2.840.113549.1.7.3";
pub const szOID_RSA_signEnvData: &'static str = "1.2.840.113549.1.7.4";
pub const szOID_RSA_digestedData: &'static str = "1.2.840.113549.1.7.5";
pub const szOID_RSA_hashedData: &'static str = "1.2.840.113549.1.7.5";
pub const szOID_RSA_encryptedData: &'static str = "1.2.840.113549.1.7.6";
pub const szOID_RSA_emailAddr: &'static str = "1.2.840.113549.1.9.1";
pub const szOID_RSA_unstructName: &'static str = "1.2.840.113549.1.9.2";
pub const szOID_RSA_contentType: &'static str = "1.2.840.113549.1.9.3";
pub const szOID_RSA_messageDigest: &'static str = "1.2.840.113549.1.9.4";
pub const szOID_RSA_signingTime: &'static str = "1.2.840.113549.1.9.5";
pub const szOID_RSA_counterSign: &'static str = "1.2.840.113549.1.9.6";
pub const szOID_RSA_challengePwd: &'static str = "1.2.840.113549.1.9.7";
pub const szOID_RSA_unstructAddr: &'static str = "1.2.840.113549.1.9.8";
pub const szOID_RSA_extCertAttrs: &'static str = "1.2.840.113549.1.9.9";
pub const szOID_RSA_certExtensions: &'static str = "1.2.840.113549.1.9.14";
pub const szOID_RSA_SMIMECapabilities: &'static str = "1.2.840.113549.1.9.15";
pub const szOID_RSA_preferSignedData: &'static str = "1.2.840.113549.1.9.15.1";
pub const szOID_TIMESTAMP_TOKEN: &'static str = "1.2.840.113549.1.9.16.1.4";
pub const szOID_RFC3161_counterSign: &'static str = "1.3.6.1.4.1.311.3.3.1";
pub const szOID_RSA_SMIMEalg: &'static str = "1.2.840.113549.1.9.16.3";
pub const szOID_RSA_SMIMEalgESDH: &'static str = "1.2.840.113549.1.9.16.3.5";
pub const szOID_RSA_SMIMEalgCMS3DESwrap: &'static str = "1.2.840.113549.1.9.16.3.6";
pub const szOID_RSA_SMIMEalgCMSRC2wrap: &'static str = "1.2.840.113549.1.9.16.3.7";
pub const szOID_RSA_MD2: &'static str = "1.2.840.113549.2.2";
pub const szOID_RSA_MD4: &'static str = "1.2.840.113549.2.4";
pub const szOID_RSA_MD5: &'static str = "1.2.840.113549.2.5";
pub const szOID_RSA_RC2CBC: &'static str = "1.2.840.113549.3.2";
pub const szOID_RSA_RC4: &'static str = "1.2.840.113549.3.4";
pub const szOID_RSA_DES_EDE3_CBC: &'static str = "1.2.840.113549.3.7";
pub const szOID_RSA_RC5_CBCPad: &'static str = "1.2.840.113549.3.9";
pub const szOID_ANSI_X942: &'static str = "1.2.840.10046";
pub const szOID_ANSI_X942_DH: &'static str = "1.2.840.10046.2.1";
pub const szOID_X957: &'static str = "1.2.840.10040";
pub const szOID_X957_DSA: &'static str = "1.2.840.10040.4.1";
pub const szOID_X957_SHA1DSA: &'static str = "1.2.840.10040.4.3";
pub const szOID_ECC_PUBLIC_KEY: &'static str = "1.2.840.10045.2.1";
pub const szOID_ECC_CURVE_P256: &'static str = "1.2.840.10045.3.1.7";
pub const szOID_ECC_CURVE_P384: &'static str = "1.3.132.0.34";
pub const szOID_ECC_CURVE_P521: &'static str = "1.3.132.0.35";
pub const szOID_ECC_CURVE_BRAINPOOLP160R1: &'static str = "1.3.36.3.3.2.8.1.1.1";
pub const szOID_ECC_CURVE_BRAINPOOLP160T1: &'static str = "1.3.36.3.3.2.8.1.1.2";
pub const szOID_ECC_CURVE_BRAINPOOLP192R1: &'static str = "1.3.36.3.3.2.8.1.1.3";
pub const szOID_ECC_CURVE_BRAINPOOLP192T1: &'static str = "1.3.36.3.3.2.8.1.1.4";
pub const szOID_ECC_CURVE_BRAINPOOLP224R1: &'static str = "1.3.36.3.3.2.8.1.1.5";
pub const szOID_ECC_CURVE_BRAINPOOLP224T1: &'static str = "1.3.36.3.3.2.8.1.1.6";
pub const szOID_ECC_CURVE_BRAINPOOLP256R1: &'static str = "1.3.36.3.3.2.8.1.1.7";
pub const szOID_ECC_CURVE_BRAINPOOLP256T1: &'static str = "1.3.36.3.3.2.8.1.1.8";
pub const szOID_ECC_CURVE_BRAINPOOLP320R1: &'static str = "1.3.36.3.3.2.8.1.1.9";
pub const szOID_ECC_CURVE_BRAINPOOLP320T1: &'static str = "1.3.36.3.3.2.8.1.1.10";
pub const szOID_ECC_CURVE_BRAINPOOLP384R1: &'static str = "1.3.36.3.3.2.8.1.1.11";
pub const szOID_ECC_CURVE_BRAINPOOLP384T1: &'static str = "1.3.36.3.3.2.8.1.1.12";
pub const szOID_ECC_CURVE_BRAINPOOLP512R1: &'static str = "1.3.36.3.3.2.8.1.1.13";
pub const szOID_ECC_CURVE_BRAINPOOLP512T1: &'static str = "1.3.36.3.3.2.8.1.1.14";
pub const szOID_ECC_CURVE_EC192WAPI: &'static str = "1.2.156.11235.1.1.2.1";
pub const szOID_CN_ECDSA_SHA256: &'static str = "1.2.156.11235.1.1.1";
pub const szOID_ECC_CURVE_NISTP192: &'static str = "1.2.840.10045.3.1.1";
pub const szOID_ECC_CURVE_NISTP224: &'static str = "1.3.132.0.33";
pub const szOID_ECC_CURVE_NISTP256: &'static str = szOID_ECC_CURVE_P256;
pub const szOID_ECC_CURVE_NISTP384: &'static str = szOID_ECC_CURVE_P384;
pub const szOID_ECC_CURVE_NISTP521: &'static str = szOID_ECC_CURVE_P521;
pub const szOID_ECC_CURVE_SECP160K1: &'static str = "1.3.132.0.9";
pub const szOID_ECC_CURVE_SECP160R1: &'static str = "1.3.132.0.8";
pub const szOID_ECC_CURVE_SECP160R2: &'static str = "1.3.132.0.30";
pub const szOID_ECC_CURVE_SECP192K1: &'static str = "1.3.132.0.31";
pub const szOID_ECC_CURVE_SECP192R1: &'static str = szOID_ECC_CURVE_NISTP192;
pub const szOID_ECC_CURVE_SECP224K1: &'static str = "1.3.132.0.32";
pub const szOID_ECC_CURVE_SECP224R1: &'static str = szOID_ECC_CURVE_NISTP224;
pub const szOID_ECC_CURVE_SECP256K1: &'static str = "1.3.132.0.10";
pub const szOID_ECC_CURVE_SECP256R1: &'static str = szOID_ECC_CURVE_P256;
pub const szOID_ECC_CURVE_SECP384R1: &'static str = szOID_ECC_CURVE_P384;
pub const szOID_ECC_CURVE_SECP521R1: &'static str = szOID_ECC_CURVE_P521;
pub const szOID_ECC_CURVE_WTLS7: &'static str = szOID_ECC_CURVE_SECP160R2;
pub const szOID_ECC_CURVE_WTLS9: &'static str = "2.23.43.1.4.9";
pub const szOID_ECC_CURVE_WTLS12: &'static str = szOID_ECC_CURVE_NISTP224;
pub const szOID_ECC_CURVE_X962P192V1: &'static str = "1.2.840.10045.3.1.1";
pub const szOID_ECC_CURVE_X962P192V2: &'static str = "1.2.840.10045.3.1.2";
pub const szOID_ECC_CURVE_X962P192V3: &'static str = "1.2.840.10045.3.1.3";
pub const szOID_ECC_CURVE_X962P239V1: &'static str = "1.2.840.10045.3.1.4";
pub const szOID_ECC_CURVE_X962P239V2: &'static str = "1.2.840.10045.3.1.5";
pub const szOID_ECC_CURVE_X962P239V3: &'static str = "1.2.840.10045.3.1.6";
pub const szOID_ECC_CURVE_X962P256V1: &'static str = szOID_ECC_CURVE_P256;
pub const szOID_ECDSA_SHA1: &'static str = "1.2.840.10045.4.1";
pub const szOID_ECDSA_SPECIFIED: &'static str = "1.2.840.10045.4.3";
pub const szOID_ECDSA_SHA256: &'static str = "1.2.840.10045.4.3.2";
pub const szOID_ECDSA_SHA384: &'static str = "1.2.840.10045.4.3.3";
pub const szOID_ECDSA_SHA512: &'static str = "1.2.840.10045.4.3.4";
pub const szOID_NIST_AES128_CBC: &'static str = "2.16.840.1.101.3.4.1.2";
pub const szOID_NIST_AES192_CBC: &'static str = "2.16.840.1.101.3.4.1.22";
pub const szOID_NIST_AES256_CBC: &'static str = "2.16.840.1.101.3.4.1.42";
pub const szOID_NIST_AES128_WRAP: &'static str = "2.16.840.1.101.3.4.1.5";
pub const szOID_NIST_AES192_WRAP: &'static str = "2.16.840.1.101.3.4.1.25";
pub const szOID_NIST_AES256_WRAP: &'static str = "2.16.840.1.101.3.4.1.45";
pub const szOID_DH_SINGLE_PASS_STDDH_SHA1_KDF: &'static str = "1.3.133.16.840.63.0.2";
pub const szOID_DH_SINGLE_PASS_STDDH_SHA256_KDF: &'static str = "1.3.132.1.11.1";
pub const szOID_DH_SINGLE_PASS_STDDH_SHA384_KDF: &'static str = "1.3.132.1.11.2";
pub const szOID_DS: &'static str = "2.5";
pub const szOID_DSALG: &'static str = "2.5.8";
pub const szOID_DSALG_CRPT: &'static str = "2.5.8.1";
pub const szOID_DSALG_HASH: &'static str = "2.5.8.2";
pub const szOID_DSALG_SIGN: &'static str = "2.5.8.3";
pub const szOID_DSALG_RSA: &'static str = "2.5.8.1.1";
pub const szOID_OIW: &'static str = "1.3.14";
pub const szOID_OIWSEC: &'static str = "1.3.14.3.2";
pub const szOID_OIWSEC_md4RSA: &'static str = "1.3.14.3.2.2";
pub const szOID_OIWSEC_md5RSA: &'static str = "1.3.14.3.2.3";
pub const szOID_OIWSEC_md4RSA2: &'static str = "1.3.14.3.2.4";
pub const szOID_OIWSEC_desECB: &'static str = "1.3.14.3.2.6";
pub const szOID_OIWSEC_desCBC: &'static str = "1.3.14.3.2.7";
pub const szOID_OIWSEC_desOFB: &'static str = "1.3.14.3.2.8";
pub const szOID_OIWSEC_desCFB: &'static str = "1.3.14.3.2.9";
pub const szOID_OIWSEC_desMAC: &'static str = "1.3.14.3.2.10";
pub const szOID_OIWSEC_rsaSign: &'static str = "1.3.14.3.2.11";
pub const szOID_OIWSEC_dsa: &'static str = "1.3.14.3.2.12";
pub const szOID_OIWSEC_shaDSA: &'static str = "1.3.14.3.2.13";
pub const szOID_OIWSEC_mdc2RSA: &'static str = "1.3.14.3.2.14";
pub const szOID_OIWSEC_shaRSA: &'static str = "1.3.14.3.2.15";
pub const szOID_OIWSEC_dhCommMod: &'static str = "1.3.14.3.2.16";
pub const szOID_OIWSEC_desEDE: &'static str = "1.3.14.3.2.17";
pub const szOID_OIWSEC_sha: &'static str = "1.3.14.3.2.18";
pub const szOID_OIWSEC_mdc2: &'static str = "1.3.14.3.2.19";
pub const szOID_OIWSEC_dsaComm: &'static str = "1.3.14.3.2.20";
pub const szOID_OIWSEC_dsaCommSHA: &'static str = "1.3.14.3.2.21";
pub const szOID_OIWSEC_rsaXchg: &'static str = "1.3.14.3.2.22";
pub const szOID_OIWSEC_keyHashSeal: &'static str = "1.3.14.3.2.23";
pub const szOID_OIWSEC_md2RSASign: &'static str = "1.3.14.3.2.24";
pub const szOID_OIWSEC_md5RSASign: &'static str = "1.3.14.3.2.25";
pub const szOID_OIWSEC_sha1: &'static str = "1.3.14.3.2.26";
pub const szOID_OIWSEC_dsaSHA1: &'static str = "1.3.14.3.2.27";
pub const szOID_OIWSEC_dsaCommSHA1: &'static str = "1.3.14.3.2.28";
pub const szOID_OIWSEC_sha1RSASign: &'static str = "1.3.14.3.2.29";
pub const szOID_OIWDIR: &'static str = "1.3.14.7.2";
pub const szOID_OIWDIR_CRPT: &'static str = "1.3.14.7.2.1";
pub const szOID_OIWDIR_HASH: &'static str = "1.3.14.7.2.2";
pub const szOID_OIWDIR_SIGN: &'static str = "1.3.14.7.2.3";
pub const szOID_OIWDIR_md2: &'static str = "1.3.14.7.2.2.1";
pub const szOID_OIWDIR_md2RSA: &'static str = "1.3.14.7.2.3.1";
pub const szOID_INFOSEC: &'static str = "2.16.840.1.101.2.1";
pub const szOID_INFOSEC_sdnsSignature: &'static str = "2.16.840.1.101.2.1.1.1";
pub const szOID_INFOSEC_mosaicSignature: &'static str = "2.16.840.1.101.2.1.1.2";
pub const szOID_INFOSEC_sdnsConfidentiality: &'static str = "2.16.840.1.101.2.1.1.3";
pub const szOID_INFOSEC_mosaicConfidentiality: &'static str = "2.16.840.1.101.2.1.1.4";
pub const szOID_INFOSEC_sdnsIntegrity: &'static str = "2.16.840.1.101.2.1.1.5";
pub const szOID_INFOSEC_mosaicIntegrity: &'static str = "2.16.840.1.101.2.1.1.6";
pub const szOID_INFOSEC_sdnsTokenProtection: &'static str = "2.16.840.1.101.2.1.1.7";
pub const szOID_INFOSEC_mosaicTokenProtection: &'static str = "2.16.840.1.101.2.1.1.8";
pub const szOID_INFOSEC_sdnsKeyManagement: &'static str = "2.16.840.1.101.2.1.1.9";
pub const szOID_INFOSEC_mosaicKeyManagement: &'static str = "2.16.840.1.101.2.1.1.10";
pub const szOID_INFOSEC_sdnsKMandSig: &'static str = "2.16.840.1.101.2.1.1.11";
pub const szOID_INFOSEC_mosaicKMandSig: &'static str = "2.16.840.1.101.2.1.1.12";
pub const szOID_INFOSEC_SuiteASignature: &'static str = "2.16.840.1.101.2.1.1.13";
pub const szOID_INFOSEC_SuiteAConfidentiality: &'static str = "2.16.840.1.101.2.1.1.14";
pub const szOID_INFOSEC_SuiteAIntegrity: &'static str = "2.16.840.1.101.2.1.1.15";
pub const szOID_INFOSEC_SuiteATokenProtection: &'static str = "2.16.840.1.101.2.1.1.16";
pub const szOID_INFOSEC_SuiteAKeyManagement: &'static str = "2.16.840.1.101.2.1.1.17";
pub const szOID_INFOSEC_SuiteAKMandSig: &'static str = "2.16.840.1.101.2.1.1.18";
pub const szOID_INFOSEC_mosaicUpdatedSig: &'static str = "2.16.840.1.101.2.1.1.19";
pub const szOID_INFOSEC_mosaicKMandUpdSig: &'static str = "2.16.840.1.101.2.1.1.20";
pub const szOID_INFOSEC_mosaicUpdatedInteg: &'static str = "2.16.840.1.101.2.1.1.21";
pub const szOID_NIST_sha256: &'static str = "2.16.840.1.101.3.4.2.1";
pub const szOID_NIST_sha384: &'static str = "2.16.840.1.101.3.4.2.2";
pub const szOID_NIST_sha512: &'static str = "2.16.840.1.101.3.4.2.3";
STRUCT!{struct CRYPT_OBJID_TABLE {
    dwAlgId: DWORD,
    pszObjId: LPCSTR,
}}
pub type PCRYPT_OBJID_TABLE = *mut CRYPT_OBJID_TABLE;
STRUCT!{struct CRYPT_HASH_INFO {
    HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    Hash: CRYPT_HASH_BLOB,
}}
pub type PCRYPT_HASH_INFO = *mut CRYPT_HASH_INFO;
STRUCT!{struct CERT_EXTENSION {
    pszObjId: LPSTR,
    fCritical: BOOL,
    Value: CRYPT_OBJID_BLOB,
}}
pub type PCERT_EXTENSION = *mut CERT_EXTENSION;
pub type PCCERT_EXTENSION = *const CERT_EXTENSION;
STRUCT!{struct CRYPT_ATTRIBUTE_TYPE_VALUE {
    pszObjId: LPSTR,
    Value: CRYPT_OBJID_BLOB,
}}
pub type PCRYPT_ATTRIBUTE_TYPE_VALUE = *mut CRYPT_ATTRIBUTE_TYPE_VALUE;
STRUCT!{struct CRYPT_ATTRIBUTE {
    pszObjId: LPSTR,
    cValue: DWORD,
    rgValue: PCRYPT_ATTR_BLOB,
}}
pub type PCRYPT_ATTRIBUTE = *mut CRYPT_ATTRIBUTE;
STRUCT!{struct CRYPT_ATTRIBUTES {
    cAttr: DWORD,
    rgAttr: PCRYPT_ATTRIBUTE,
}}
pub type PCRYPT_ATTRIBUTES = *mut CRYPT_ATTRIBUTES;
STRUCT!{struct CERT_RDN_ATTR {
    pszObjId: LPSTR,
    dwValueType: DWORD,
    Value: CERT_RDN_VALUE_BLOB,
}}
pub type PCERT_RDN_ATTR = *mut CERT_RDN_ATTR;
pub const szOID_COMMON_NAME: &'static str = "2.5.4.3";
pub const szOID_SUR_NAME: &'static str = "2.5.4.4";
pub const szOID_DEVICE_SERIAL_NUMBER: &'static str = "2.5.4.5";
pub const szOID_COUNTRY_NAME: &'static str = "2.5.4.6";
pub const szOID_LOCALITY_NAME: &'static str = "2.5.4.7";
pub const szOID_STATE_OR_PROVINCE_NAME: &'static str = "2.5.4.8";
pub const szOID_STREET_ADDRESS: &'static str = "2.5.4.9";
pub const szOID_ORGANIZATION_NAME: &'static str = "2.5.4.10";
pub const szOID_ORGANIZATIONAL_UNIT_NAME: &'static str = "2.5.4.11";
pub const szOID_TITLE: &'static str = "2.5.4.12";
pub const szOID_DESCRIPTION: &'static str = "2.5.4.13";
pub const szOID_SEARCH_GUIDE: &'static str = "2.5.4.14";
pub const szOID_BUSINESS_CATEGORY: &'static str = "2.5.4.15";
pub const szOID_POSTAL_ADDRESS: &'static str = "2.5.4.16";
pub const szOID_POSTAL_CODE: &'static str = "2.5.4.17";
pub const szOID_POST_OFFICE_BOX: &'static str = "2.5.4.18";
pub const szOID_PHYSICAL_DELIVERY_OFFICE_NAME: &'static str = "2.5.4.19";
pub const szOID_TELEPHONE_NUMBER: &'static str = "2.5.4.20";
pub const szOID_TELEX_NUMBER: &'static str = "2.5.4.21";
pub const szOID_TELETEXT_TERMINAL_IDENTIFIER: &'static str = "2.5.4.22";
pub const szOID_FACSIMILE_TELEPHONE_NUMBER: &'static str = "2.5.4.23";
pub const szOID_X21_ADDRESS: &'static str = "2.5.4.24";
pub const szOID_INTERNATIONAL_ISDN_NUMBER: &'static str = "2.5.4.25";
pub const szOID_REGISTERED_ADDRESS: &'static str = "2.5.4.26";
pub const szOID_DESTINATION_INDICATOR: &'static str = "2.5.4.27";
pub const szOID_PREFERRED_DELIVERY_METHOD: &'static str = "2.5.4.28";
pub const szOID_PRESENTATION_ADDRESS: &'static str = "2.5.4.29";
pub const szOID_SUPPORTED_APPLICATION_CONTEXT: &'static str = "2.5.4.30";
pub const szOID_MEMBER: &'static str = "2.5.4.31";
pub const szOID_OWNER: &'static str = "2.5.4.32";
pub const szOID_ROLE_OCCUPANT: &'static str = "2.5.4.33";
pub const szOID_SEE_ALSO: &'static str = "2.5.4.34";
pub const szOID_USER_PASSWORD: &'static str = "2.5.4.35";
pub const szOID_USER_CERTIFICATE: &'static str = "2.5.4.36";
pub const szOID_CA_CERTIFICATE: &'static str = "2.5.4.37";
pub const szOID_AUTHORITY_REVOCATION_LIST: &'static str = "2.5.4.38";
pub const szOID_CERTIFICATE_REVOCATION_LIST: &'static str = "2.5.4.39";
pub const szOID_CROSS_CERTIFICATE_PAIR: &'static str = "2.5.4.40";
pub const szOID_GIVEN_NAME: &'static str = "2.5.4.42";
pub const szOID_INITIALS: &'static str = "2.5.4.43";
pub const szOID_DN_QUALIFIER: &'static str = "2.5.4.46";
pub const szOID_DOMAIN_COMPONENT: &'static str = "0.9.2342.19200300.100.1.25";
pub const szOID_PKCS_12_FRIENDLY_NAME_ATTR: &'static str = "1.2.840.113549.1.9.20";
pub const szOID_PKCS_12_LOCAL_KEY_ID: &'static str = "1.2.840.113549.1.9.21";
pub const szOID_PKCS_12_KEY_PROVIDER_NAME_ATTR: &'static str = "1.3.6.1.4.1.311.17.1";
pub const szOID_LOCAL_MACHINE_KEYSET: &'static str = "1.3.6.1.4.1.311.17.2";
pub const szOID_PKCS_12_EXTENDED_ATTRIBUTES: &'static str = "1.3.6.1.4.1.311.17.3";
pub const szOID_PKCS_12_PROTECTED_PASSWORD_SECRET_BAG_TYPE_ID: &'static str
    = "1.3.6.1.4.1.311.17.4";
pub const szOID_KEYID_RDN: &'static str = "1.3.6.1.4.1.311.10.7.1";
pub const szOID_EV_RDN_LOCALE: &'static str = "1.3.6.1.4.1.311.60.2.1.1";
pub const szOID_EV_RDN_STATE_OR_PROVINCE: &'static str = "1.3.6.1.4.1.311.60.2.1.2";
pub const szOID_EV_RDN_COUNTRY: &'static str = "1.3.6.1.4.1.311.60.2.1.3";
pub const CERT_RDN_ANY_TYPE: DWORD = 0;
pub const CERT_RDN_ENCODED_BLOB: DWORD = 1;
pub const CERT_RDN_OCTET_STRING: DWORD = 2;
pub const CERT_RDN_NUMERIC_STRING: DWORD = 3;
pub const CERT_RDN_PRINTABLE_STRING: DWORD = 4;
pub const CERT_RDN_TELETEX_STRING: DWORD = 5;
pub const CERT_RDN_T61_STRING: DWORD = 5;
pub const CERT_RDN_VIDEOTEX_STRING: DWORD = 6;
pub const CERT_RDN_IA5_STRING: DWORD = 7;
pub const CERT_RDN_GRAPHIC_STRING: DWORD = 8;
pub const CERT_RDN_VISIBLE_STRING: DWORD = 9;
pub const CERT_RDN_ISO646_STRING: DWORD = 9;
pub const CERT_RDN_GENERAL_STRING: DWORD = 10;
pub const CERT_RDN_UNIVERSAL_STRING: DWORD = 11;
pub const CERT_RDN_INT4_STRING: DWORD = 11;
pub const CERT_RDN_BMP_STRING: DWORD = 12;
pub const CERT_RDN_UNICODE_STRING: DWORD = 12;
pub const CERT_RDN_UTF8_STRING: DWORD = 13;
pub const CERT_RDN_TYPE_MASK: DWORD = 0x000000FF;
pub const CERT_RDN_FLAGS_MASK: DWORD = 0xFF000000;
pub const CERT_RDN_ENABLE_T61_UNICODE_FLAG: DWORD = 0x80000000;
pub const CERT_RDN_ENABLE_UTF8_UNICODE_FLAG: DWORD = 0x20000000;
pub const CERT_RDN_FORCE_UTF8_UNICODE_FLAG: DWORD = 0x10000000;
pub const CERT_RDN_DISABLE_CHECK_TYPE_FLAG: DWORD = 0x40000000;
pub const CERT_RDN_DISABLE_IE4_UTF8_FLAG: DWORD = 0x01000000;
pub const CERT_RDN_ENABLE_PUNYCODE_FLAG: DWORD = 0x02000000;
#[inline]
pub fn IS_CERT_RDN_CHAR_STRING(X: DWORD) -> bool {
    (X & CERT_RDN_TYPE_MASK) >= CERT_RDN_NUMERIC_STRING
}
STRUCT!{struct CERT_RDN {
    cRDNAttr: DWORD,
    rgRDNAttr: PCERT_RDN_ATTR,
}}
pub type PCERT_RDN = *mut CERT_RDN;
STRUCT!{struct CERT_NAME_INFO {
    cRDN: DWORD,
    rgRDN: PCERT_RDN,
}}
pub type PCERT_NAME_INFO = *mut CERT_NAME_INFO;
STRUCT!{struct CERT_NAME_VALUE {
    dwValueType: DWORD,
    Value: CERT_RDN_VALUE_BLOB,
}}
pub type PCERT_NAME_VALUE = *mut CERT_NAME_VALUE;
STRUCT!{struct CERT_PUBLIC_KEY_INFO {
    Algorithm: CRYPT_ALGORITHM_IDENTIFIER,
    PublicKey: CRYPT_BIT_BLOB,
}}
pub type PCERT_PUBLIC_KEY_INFO = *mut CERT_PUBLIC_KEY_INFO;
pub const CERT_RSA_PUBLIC_KEY_OBJID: &'static str = szOID_RSA_RSA;
pub const CERT_DEFAULT_OID_PUBLIC_KEY_SIGN: &'static str = szOID_RSA_RSA;
pub const CERT_DEFAULT_OID_PUBLIC_KEY_XCHG: &'static str = szOID_RSA_RSA;
STRUCT!{struct CRYPT_ECC_PRIVATE_KEY_INFO {
    dwVersion: DWORD,
    PrivateKey: CRYPT_DER_BLOB,
    szCurveOid: LPSTR,
    PublicKey: CRYPT_BIT_BLOB,
}}
pub type PCRYPT_ECC_PRIVATE_KEY_INFO = *mut CRYPT_ECC_PRIVATE_KEY_INFO;
pub const CRYPT_ECC_PRIVATE_KEY_INFO_v1: DWORD = 1;
STRUCT!{struct CRYPT_PRIVATE_KEY_INFO {
    Version: DWORD,
    Algorithm: CRYPT_ALGORITHM_IDENTIFIER,
    PrivateKey: CRYPT_DER_BLOB,
    pAttributes: PCRYPT_ATTRIBUTES,
}}
pub type PCRYPT_PRIVATE_KEY_INFO = *mut CRYPT_PRIVATE_KEY_INFO;
STRUCT!{struct CRYPT_ENCRYPTED_PRIVATE_KEY_INFO {
    EncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    EncryptedPrivateKey: CRYPT_DATA_BLOB,
}}
pub type PCRYPT_ENCRYPTED_PRIVATE_KEY_INFO = *mut CRYPT_ENCRYPTED_PRIVATE_KEY_INFO;
FN!{stdcall PCRYPT_DECRYPT_PRIVATE_KEY_FUNC(
    Algorithm: CRYPT_ALGORITHM_IDENTIFIER,
    EncryptedPrivateKey: CRYPT_DATA_BLOB,
    pbClearTextKey: *mut BYTE,
    pcbClearTextKey: *mut DWORD,
    pVoidDecryptFunc: LPVOID,
) -> BOOL}
FN!{stdcall PCRYPT_ENCRYPT_PRIVATE_KEY_FUNC(
    Algorithm: *mut CRYPT_ALGORITHM_IDENTIFIER,
    pClearTextPrivateKey: *mut CRYPT_DATA_BLOB,
    pbEncryptedKey: *mut BYTE,
    pcbEncryptedKey: *mut DWORD,
    pVoidEncryptFunc: LPVOID,
) -> BOOL}
FN!{stdcall PCRYPT_RESOLVE_HCRYPTPROV_FUNC(
    pPrivateKeyInfo: *mut CRYPT_PRIVATE_KEY_INFO,
    phCryptProv: *mut HCRYPTPROV,
    pVoidResolveFunc: LPVOID,
) -> BOOL}
STRUCT!{struct CRYPT_PKCS8_IMPORT_PARAMS {
    PrivateKey: CRYPT_DIGEST_BLOB,
    pResolvehCryptProvFunc: PCRYPT_RESOLVE_HCRYPTPROV_FUNC,
    pVoidResolveFunc: LPVOID,
    pDecryptPrivateKeyFunc: PCRYPT_DECRYPT_PRIVATE_KEY_FUNC,
    pVoidDecryptFunc: LPVOID,
}}
pub type PCRYPT_PKCS8_IMPORT_PARAMS = *mut CRYPT_PKCS8_IMPORT_PARAMS;
pub type CRYPT_PRIVATE_KEY_BLOB_AND_PARAMS = CRYPT_PKCS8_IMPORT_PARAMS;
pub type PPCRYPT_PRIVATE_KEY_BLOB_AND_PARAMS = *mut CRYPT_PKCS8_IMPORT_PARAMS;
STRUCT!{struct CRYPT_PKCS8_EXPORT_PARAMS {
    hCryptProv: HCRYPTPROV,
    dwKeySpec: DWORD,
    pszPrivateKeyObjId: LPSTR,
    pEncryptPrivateKeyFunc: PCRYPT_ENCRYPT_PRIVATE_KEY_FUNC,
    pVoidEncryptFunc: LPVOID,
}}
pub type PCRYPT_PKCS8_EXPORT_PARAMS = *mut CRYPT_PKCS8_EXPORT_PARAMS;
STRUCT!{struct CERT_INFO {
    dwVersion: DWORD,
    SerialNumber: CRYPT_INTEGER_BLOB,
    SignatureAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    Issuer: CERT_NAME_BLOB,
    NotBefore: FILETIME,
    NotAfter: FILETIME,
    Subject: CERT_NAME_BLOB,
    SubjectPublicKeyInfo: CERT_PUBLIC_KEY_INFO,
    IssuerUniqueId: CRYPT_BIT_BLOB,
    SubjectUniqueId: CRYPT_BIT_BLOB,
    cExtension: DWORD,
    rgExtension: PCERT_EXTENSION,
}}
pub type PCERT_INFO = *mut CERT_INFO;
pub const CERT_V1: DWORD = 0;
pub const CERT_V2: DWORD = 1;
pub const CERT_V3: DWORD = 2;
pub const CERT_INFO_VERSION_FLAG: DWORD = 1;
pub const CERT_INFO_SERIAL_NUMBER_FLAG: DWORD = 2;
pub const CERT_INFO_SIGNATURE_ALGORITHM_FLAG: DWORD = 3;
pub const CERT_INFO_ISSUER_FLAG: DWORD = 4;
pub const CERT_INFO_NOT_BEFORE_FLAG: DWORD = 5;
pub const CERT_INFO_NOT_AFTER_FLAG: DWORD = 6;
pub const CERT_INFO_SUBJECT_FLAG: DWORD = 7;
pub const CERT_INFO_SUBJECT_PUBLIC_KEY_INFO_FLAG: DWORD = 8;
pub const CERT_INFO_ISSUER_UNIQUE_ID_FLAG: DWORD = 9;
pub const CERT_INFO_SUBJECT_UNIQUE_ID_FLAG: DWORD = 10;
pub const CERT_INFO_EXTENSION_FLAG: DWORD = 11;
STRUCT!{struct CRL_ENTRY {
    SerialNumber: CRYPT_INTEGER_BLOB,
    RevocationDate: FILETIME,
    cExtension: DWORD,
    rgExtension: PCERT_EXTENSION,
}}
pub type PCRL_ENTRY = *mut CRL_ENTRY;
STRUCT!{struct CRL_INFO {
    dwVersion: DWORD,
    SignatureAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    Issuer: CERT_NAME_BLOB,
    ThisUpdate: FILETIME,
    NextUpdate: FILETIME,
    cCRLEntry: DWORD,
    rgCRLEntry: PCRL_ENTRY,
    cExtension: DWORD,
    rgExtension: PCERT_EXTENSION,
}}
pub type PCRL_INFO = *mut CRL_INFO;
pub const CRL_V1: DWORD = 0;
pub const CRL_V2: DWORD = 1;
pub const CERT_BUNDLE_CERTIFICATE: DWORD = 0;
pub const CERT_BUNDLE_CRL: DWORD = 1;
STRUCT!{struct CERT_OR_CRL_BLOB {
    dwChoice: DWORD,
    cbEncoded: DWORD,
    pbEncoded: *mut BYTE,
}}
pub type PCERT_OR_CRL_BLOB = *mut CERT_OR_CRL_BLOB;
STRUCT!{struct CERT_OR_CRL_BUNDLE {
    cItem: DWORD,
    rgItem: PCERT_OR_CRL_BLOB,
}}
pub type PCERT_OR_CRL_BUNDLE = *mut CERT_OR_CRL_BUNDLE;
STRUCT!{struct CERT_REQUEST_INFO {
    dwVersion: DWORD,
    Subject: CERT_NAME_BLOB,
    SubjectPublicKeyInfo: CERT_PUBLIC_KEY_INFO,
    cAttribute: DWORD,
    rgAttribute: PCRYPT_ATTRIBUTE,
}}
pub type PCERT_REQUEST_INFO = *mut CERT_REQUEST_INFO;
pub const CERT_REQUEST_V1: DWORD = 0;
STRUCT!{struct CERT_KEYGEN_REQUEST_INFO {
    dwVersion: DWORD,
    SubjectPublicKeyInfo: CERT_PUBLIC_KEY_INFO,
    pwszChallengeString: LPWSTR,
}}
pub type PCERT_KEYGEN_REQUEST_INFO = *mut CERT_KEYGEN_REQUEST_INFO;
pub const CERT_KEYGEN_REQUEST_V1: DWORD = 0;
STRUCT!{struct CERT_SIGNED_CONTENT_INFO {
    ToBeSigned: CRYPT_DER_BLOB,
    SignatureAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    Signature: CRYPT_BIT_BLOB,
}}
pub type PCERT_SIGNED_CONTENT_INFO = *mut CERT_SIGNED_CONTENT_INFO;
STRUCT!{struct CTL_USAGE {
    cUsageIdentifier: DWORD,
    rgpszUsageIdentifier: *mut LPSTR,
}}
pub type PCTL_USAGE = *mut CTL_USAGE;
pub type CERT_ENHKEY_USAGE = CTL_USAGE;
pub type PCERT_ENHKEY_USAGE = *mut CERT_ENHKEY_USAGE;
pub type PCCTL_USAGE = *const CTL_USAGE;
pub type PCCERT_ENHKEY_USAGE = *const CERT_ENHKEY_USAGE;
STRUCT!{struct CTL_ENTRY {
    SubjectIdentifier: CRYPT_DATA_BLOB,
    cAttribute: DWORD,
    rgAttribute: PCRYPT_ATTRIBUTE,
}}
pub type PCTL_ENTRY = *mut CTL_ENTRY;
STRUCT!{struct CTL_INFO {
    dwVersion: DWORD,
    SubjectUsage: CTL_USAGE,
    ListIdentifier: CRYPT_DATA_BLOB,
    SequenceNumber: CRYPT_INTEGER_BLOB,
    ThisUpdate: FILETIME,
    NextUpdate: FILETIME,
    SubjectAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    cCTLEntry: DWORD,
    rgCTLEntry: PCTL_ENTRY,
    cExtension: DWORD,
    rgExtension: PCERT_EXTENSION,
}}
pub type PCTL_INFO = *mut CTL_INFO;
pub const CTL_V1: DWORD = 0;
STRUCT!{struct CRYPT_TIME_STAMP_REQUEST_INFO {
    pszTimeStampAlgorithm: LPSTR,
    pszContentType: LPSTR,
    Content: CRYPT_OBJID_BLOB,
    cAttribute: DWORD,
    rgAttribute: PCRYPT_ATTRIBUTE,
}}
pub type PCRYPT_TIME_STAMP_REQUEST_INFO = *mut CRYPT_TIME_STAMP_REQUEST_INFO;
STRUCT!{struct CRYPT_ENROLLMENT_NAME_VALUE_PAIR {
    pwszName: LPWSTR,
    pwszValue: LPWSTR,
}}
pub type PCRYPT_ENROLLMENT_NAME_VALUE_PAIR = *mut CRYPT_ENROLLMENT_NAME_VALUE_PAIR;
STRUCT!{struct CRYPT_CSP_PROVIDER {
    dwKeySpec: DWORD,
    pwszProviderName: LPWSTR,
    Signature: CRYPT_BIT_BLOB,
}}
pub type PCRYPT_CSP_PROVIDER = *mut CRYPT_CSP_PROVIDER;
pub const CERT_ENCODING_TYPE_MASK: DWORD = 0x0000FFFF;
pub const CMSG_ENCODING_TYPE_MASK: DWORD = 0xFFFF0000;
#[inline]
pub fn GET_CERT_ENCODING_TYPE(X: DWORD) -> DWORD {
    X & CERT_ENCODING_TYPE_MASK
}
#[inline]
pub fn GET_CMSG_ENCODING_TYPE(X: DWORD) -> DWORD {
    X & CMSG_ENCODING_TYPE_MASK
}
pub const CRYPT_ASN_ENCODING: DWORD = 0x00000001;
pub const CRYPT_NDR_ENCODING: DWORD = 0x00000002;
pub const X509_ASN_ENCODING: DWORD = 0x00000001;
pub const X509_NDR_ENCODING: DWORD = 0x00000002;
pub const PKCS_7_ASN_ENCODING: DWORD = 0x00010000;
pub const PKCS_7_NDR_ENCODING: DWORD = 0x00020000;
extern "system" {
    pub fn CryptFormatObject(
        dwCertEncodingType: DWORD,
        dwFormatType: DWORD,
        dwFormatStrType: DWORD,
        pFormatStruct: *mut c_void,
        lpszStructType: LPCSTR,
        pbEncoded: *const BYTE,
        cbEncoded: DWORD,
        pbFormat: *mut c_void,
        pcbFormat: *mut DWORD,
    ) -> BOOL;
}
pub const CRYPT_FORMAT_STR_MULTI_LINE: DWORD = 0x0001;
pub const CRYPT_FORMAT_STR_NO_HEX: DWORD = 0x0010;
pub const CRYPT_FORMAT_SIMPLE: DWORD = 0x0001;
pub const CRYPT_FORMAT_X509: DWORD = 0x0002;
pub const CRYPT_FORMAT_OID: DWORD = 0x0004;
pub const CRYPT_FORMAT_RDN_SEMICOLON: DWORD = 0x0100;
pub const CRYPT_FORMAT_RDN_CRLF: DWORD = 0x0200;
pub const CRYPT_FORMAT_RDN_UNQUOTE: DWORD = 0x0400;
pub const CRYPT_FORMAT_RDN_REVERSE: DWORD = 0x0800;
pub const CRYPT_FORMAT_COMMA: DWORD = 0x1000;
pub const CRYPT_FORMAT_SEMICOLON: DWORD = CRYPT_FORMAT_RDN_SEMICOLON;
pub const CRYPT_FORMAT_CRLF: DWORD = CRYPT_FORMAT_RDN_CRLF;
FN!{stdcall PFN_CRYPT_ALLOC(
    cbSize: size_t,
) -> LPVOID}
FN!{stdcall PFN_CRYPT_FREE(
    pv: LPVOID,
) -> ()}
STRUCT!{struct CRYPT_ENCODE_PARA {
    cbSize: DWORD,
    pfnAlloc: PFN_CRYPT_ALLOC,
    pfnFree: PFN_CRYPT_FREE,
}}
pub type PCRYPT_ENCODE_PARA = *mut CRYPT_ENCODE_PARA;
extern "system" {
    pub fn CryptEncodeObjectEx(
        dwCertEncodingType: DWORD,
        lpszStructType: LPCSTR,
        pvStructInfo: *const c_void,
        dwFlags: DWORD,
        pEncodePara: PCRYPT_ENCODE_PARA,
        pvEncoded: *mut c_void,
        pcbEncoded: *mut DWORD,
    ) -> BOOL;
    pub fn CryptEncodeObject(
        dwCertEncodingType: DWORD,
        lpszStructType: LPCSTR,
        pvStructInfo: *const c_void,
        pbEncoded: *mut BYTE,
        pcbEncoded: *mut DWORD,
    ) -> BOOL;
}
pub const CRYPT_ENCODE_NO_SIGNATURE_BYTE_REVERSAL_FLAG: DWORD = 0x8;
pub const CRYPT_ENCODE_ALLOC_FLAG: DWORD = 0x8000;
pub const CRYPT_UNICODE_NAME_ENCODE_ENABLE_T61_UNICODE_FLAG: DWORD
    = CERT_RDN_ENABLE_T61_UNICODE_FLAG;
pub const CRYPT_UNICODE_NAME_ENCODE_ENABLE_UTF8_UNICODE_FLAG: DWORD
    = CERT_RDN_ENABLE_UTF8_UNICODE_FLAG;
pub const CRYPT_UNICODE_NAME_ENCODE_FORCE_UTF8_UNICODE_FLAG: DWORD
    = CERT_RDN_FORCE_UTF8_UNICODE_FLAG;
pub const CRYPT_UNICODE_NAME_ENCODE_DISABLE_CHECK_TYPE_FLAG: DWORD
    = CERT_RDN_DISABLE_CHECK_TYPE_FLAG;
pub const CRYPT_SORTED_CTL_ENCODE_HASHED_SUBJECT_IDENTIFIER_FLAG: DWORD = 0x10000;
pub const CRYPT_ENCODE_ENABLE_PUNYCODE_FLAG: DWORD = 0x20000;
pub const CRYPT_ENCODE_ENABLE_UTF8PERCENT_FLAG: DWORD = 0x40000;
pub const CRYPT_ENCODE_ENABLE_IA5CONVERSION_FLAG: DWORD = CRYPT_ENCODE_ENABLE_PUNYCODE_FLAG
    | CRYPT_ENCODE_ENABLE_UTF8PERCENT_FLAG;
STRUCT!{struct CRYPT_DECODE_PARA {
    cbSize: DWORD,
    pfnAlloc: PFN_CRYPT_ALLOC,
    pfnFree: PFN_CRYPT_FREE,
}}
pub type PCRYPT_DECODE_PARA = *mut CRYPT_DECODE_PARA;
extern "system" {
    pub fn CryptDecodeObjectEx(
        dwCertEncodingType: DWORD,
        lpszStructType: LPCSTR,
        pbEncoded: *const BYTE,
        cbEncoded: DWORD,
        dwFlags: DWORD,
        pDecodePara: PCRYPT_DECODE_PARA,
        pvStructInfo: *mut c_void,
        pcbStructInfo: *mut DWORD,
    ) -> BOOL;
    pub fn CryptDecodeObject(
        dwCertEncodingType: DWORD,
        lpszStructType: LPCSTR,
        pbEncoded: *const BYTE,
        cbEncoded: DWORD,
        dwFlags: DWORD,
        pvStructInfo: *mut c_void,
        pcbStructInfo: *mut DWORD,
    ) -> BOOL;
}
pub const CRYPT_DECODE_NOCOPY_FLAG: DWORD = 0x1;
pub const CRYPT_DECODE_TO_BE_SIGNED_FLAG: DWORD = 0x2;
pub const CRYPT_DECODE_SHARE_OID_STRING_FLAG: DWORD = 0x4;
pub const CRYPT_DECODE_NO_SIGNATURE_BYTE_REVERSAL_FLAG: DWORD = 0x8;
pub const CRYPT_DECODE_ALLOC_FLAG: DWORD = 0x8000;
pub const CRYPT_UNICODE_NAME_DECODE_DISABLE_IE4_UTF8_FLAG: DWORD
    = CERT_RDN_DISABLE_IE4_UTF8_FLAG;
pub const CRYPT_DECODE_ENABLE_PUNYCODE_FLAG: DWORD = 0x02000000;
pub const CRYPT_DECODE_ENABLE_UTF8PERCENT_FLAG: DWORD = 0x04000000;
pub const CRYPT_DECODE_ENABLE_IA5CONVERSION_FLAG: DWORD = CRYPT_DECODE_ENABLE_PUNYCODE_FLAG
    | CRYPT_DECODE_ENABLE_UTF8PERCENT_FLAG;
pub const CRYPT_ENCODE_DECODE_NONE: LPCSTR = 0 as LPCSTR;
pub const X509_CERT: LPCSTR = 1 as LPCSTR;
pub const X509_CERT_TO_BE_SIGNED: LPCSTR = 2 as LPCSTR;
pub const X509_CERT_CRL_TO_BE_SIGNED: LPCSTR = 3 as LPCSTR;
pub const X509_CERT_REQUEST_TO_BE_SIGNED: LPCSTR = 4 as LPCSTR;
pub const X509_EXTENSIONS: LPCSTR = 5 as LPCSTR;
pub const X509_NAME_VALUE: LPCSTR = 6 as LPCSTR;
pub const X509_NAME: LPCSTR = 7 as LPCSTR;
pub const X509_PUBLIC_KEY_INFO: LPCSTR = 8 as LPCSTR;
pub const X509_AUTHORITY_KEY_ID: LPCSTR = 9 as LPCSTR;
pub const X509_KEY_ATTRIBUTES: LPCSTR = 10 as LPCSTR;
pub const X509_KEY_USAGE_RESTRICTION: LPCSTR = 11 as LPCSTR;
pub const X509_ALTERNATE_NAME: LPCSTR = 12 as LPCSTR;
pub const X509_BASIC_CONSTRAINTS: LPCSTR = 13 as LPCSTR;
pub const X509_KEY_USAGE: LPCSTR = 14 as LPCSTR;
pub const X509_BASIC_CONSTRAINTS2: LPCSTR = 15 as LPCSTR;
pub const X509_CERT_POLICIES: LPCSTR = 16 as LPCSTR;
pub const PKCS_UTC_TIME: LPCSTR = 17 as LPCSTR;
pub const PKCS_TIME_REQUEST: LPCSTR = 18 as LPCSTR;
pub const RSA_CSP_PUBLICKEYBLOB: LPCSTR = 19 as LPCSTR;
pub const X509_UNICODE_NAME: LPCSTR = 20 as LPCSTR;
pub const X509_KEYGEN_REQUEST_TO_BE_SIGNED: LPCSTR = 21 as LPCSTR;
pub const PKCS_ATTRIBUTE: LPCSTR = 22 as LPCSTR;
pub const PKCS_CONTENT_INFO_SEQUENCE_OF_ANY: LPCSTR = 23 as LPCSTR;
pub const X509_UNICODE_NAME_VALUE: LPCSTR = 24 as LPCSTR;
pub const X509_ANY_STRING: LPCSTR = X509_NAME_VALUE;
pub const X509_UNICODE_ANY_STRING: LPCSTR = X509_UNICODE_NAME_VALUE;
pub const X509_OCTET_STRING: LPCSTR = 25 as LPCSTR;
pub const X509_BITS: LPCSTR = 26 as LPCSTR;
pub const X509_INTEGER: LPCSTR = 27 as LPCSTR;
pub const X509_MULTI_BYTE_INTEGER: LPCSTR = 28 as LPCSTR;
pub const X509_ENUMERATED: LPCSTR = 29 as LPCSTR;
pub const X509_CHOICE_OF_TIME: LPCSTR = 30 as LPCSTR;
pub const X509_AUTHORITY_KEY_ID2: LPCSTR = 31 as LPCSTR;
pub const X509_AUTHORITY_INFO_ACCESS: LPCSTR = 32 as LPCSTR;
pub const X509_SUBJECT_INFO_ACCESS: LPCSTR = X509_AUTHORITY_INFO_ACCESS;
pub const X509_CRL_REASON_CODE: LPCSTR = X509_ENUMERATED;
pub const PKCS_CONTENT_INFO: LPCSTR = 33 as LPCSTR;
pub const X509_SEQUENCE_OF_ANY: LPCSTR = 34 as LPCSTR;
pub const X509_CRL_DIST_POINTS: LPCSTR = 35 as LPCSTR;
pub const X509_ENHANCED_KEY_USAGE: LPCSTR = 36 as LPCSTR;
pub const PKCS_CTL: LPCSTR = 37 as LPCSTR;
pub const X509_MULTI_BYTE_UINT: LPCSTR = 38 as LPCSTR;
pub const X509_DSS_PUBLICKEY: LPCSTR = X509_MULTI_BYTE_UINT;
pub const X509_DSS_PARAMETERS: LPCSTR = 39 as LPCSTR;
pub const X509_DSS_SIGNATURE: LPCSTR = 40 as LPCSTR;
pub const PKCS_RC2_CBC_PARAMETERS: LPCSTR = 41 as LPCSTR;
pub const PKCS_SMIME_CAPABILITIES: LPCSTR = 42 as LPCSTR;
pub const X509_QC_STATEMENTS_EXT: LPCSTR = 42 as LPCSTR;
pub const PKCS_RSA_PRIVATE_KEY: LPCSTR = 43 as LPCSTR;
pub const PKCS_PRIVATE_KEY_INFO: LPCSTR = 44 as LPCSTR;
pub const PKCS_ENCRYPTED_PRIVATE_KEY_INFO: LPCSTR = 45 as LPCSTR;
pub const X509_PKIX_POLICY_QUALIFIER_USERNOTICE: LPCSTR = 46 as LPCSTR;
pub const X509_DH_PUBLICKEY: LPCSTR = X509_MULTI_BYTE_UINT;
pub const X509_DH_PARAMETERS: LPCSTR = 47 as LPCSTR;
pub const PKCS_ATTRIBUTES: LPCSTR = 48 as LPCSTR;
pub const PKCS_SORTED_CTL: LPCSTR = 49 as LPCSTR;
pub const X509_ECC_SIGNATURE: LPCSTR = 47 as LPCSTR;
pub const X942_DH_PARAMETERS: LPCSTR = 50 as LPCSTR;
pub const X509_BITS_WITHOUT_TRAILING_ZEROES: LPCSTR = 51 as LPCSTR;
pub const X942_OTHER_INFO: LPCSTR = 52 as LPCSTR;
pub const X509_CERT_PAIR: LPCSTR = 53 as LPCSTR;
pub const X509_ISSUING_DIST_POINT: LPCSTR = 54 as LPCSTR;
pub const X509_NAME_CONSTRAINTS: LPCSTR = 55 as LPCSTR;
pub const X509_POLICY_MAPPINGS: LPCSTR = 56 as LPCSTR;
pub const X509_POLICY_CONSTRAINTS: LPCSTR = 57 as LPCSTR;
pub const X509_CROSS_CERT_DIST_POINTS: LPCSTR = 58 as LPCSTR;
pub const CMC_DATA: LPCSTR = 59 as LPCSTR;
pub const CMC_RESPONSE: LPCSTR = 60 as LPCSTR;
pub const CMC_STATUS: LPCSTR = 61 as LPCSTR;
pub const CMC_ADD_EXTENSIONS: LPCSTR = 62 as LPCSTR;
pub const CMC_ADD_ATTRIBUTES: LPCSTR = 63 as LPCSTR;
pub const X509_CERTIFICATE_TEMPLATE: LPCSTR = 64 as LPCSTR;
pub const OCSP_SIGNED_REQUEST: LPCSTR = 65 as LPCSTR;
pub const OCSP_REQUEST: LPCSTR = 66 as LPCSTR;
pub const OCSP_RESPONSE: LPCSTR = 67 as LPCSTR;
pub const OCSP_BASIC_SIGNED_RESPONSE: LPCSTR = 68 as LPCSTR;
pub const OCSP_BASIC_RESPONSE: LPCSTR = 69 as LPCSTR;
pub const X509_LOGOTYPE_EXT: LPCSTR = 70 as LPCSTR;
pub const X509_BIOMETRIC_EXT: LPCSTR = 71 as LPCSTR;
pub const CNG_RSA_PUBLIC_KEY_BLOB: LPCSTR = 72 as LPCSTR;
pub const X509_OBJECT_IDENTIFIER: LPCSTR = 73 as LPCSTR;
pub const X509_ALGORITHM_IDENTIFIER: LPCSTR = 74 as LPCSTR;
pub const PKCS_RSA_SSA_PSS_PARAMETERS: LPCSTR = 75 as LPCSTR;
pub const PKCS_RSAES_OAEP_PARAMETERS: LPCSTR = 76 as LPCSTR;
pub const ECC_CMS_SHARED_INFO: LPCSTR = 77 as LPCSTR;
pub const TIMESTAMP_REQUEST: LPCSTR = 78 as LPCSTR;
pub const TIMESTAMP_RESPONSE: LPCSTR = 79 as LPCSTR;
pub const TIMESTAMP_INFO: LPCSTR = 80 as LPCSTR;
pub const X509_CERT_BUNDLE: LPCSTR = 81 as LPCSTR;
pub const X509_ECC_PRIVATE_KEY: LPCSTR = 82 as LPCSTR;
pub const CNG_RSA_PRIVATE_KEY_BLOB: LPCSTR = 83 as LPCSTR;
pub const X509_SUBJECT_DIR_ATTRS: LPCSTR = 84 as LPCSTR;
pub const X509_ECC_PARAMETERS: LPCSTR = 85 as LPCSTR;
pub const PKCS7_SIGNER_INFO: LPCSTR = 500 as LPCSTR;
pub const CMS_SIGNER_INFO: LPCSTR = 501 as LPCSTR;
pub const szOID_AUTHORITY_KEY_IDENTIFIER: &'static str = "2.5.29.1";
pub const szOID_KEY_ATTRIBUTES: &'static str = "2.5.29.2";
pub const szOID_CERT_POLICIES_95: &'static str = "2.5.29.3";
pub const szOID_KEY_USAGE_RESTRICTION: &'static str = "2.5.29.4";
pub const szOID_SUBJECT_ALT_NAME: &'static str = "2.5.29.7";
pub const szOID_ISSUER_ALT_NAME: &'static str = "2.5.29.8";
pub const szOID_BASIC_CONSTRAINTS: &'static str = "2.5.29.10";
pub const szOID_KEY_USAGE: &'static str = "2.5.29.15";
pub const szOID_PRIVATEKEY_USAGE_PERIOD: &'static str = "2.5.29.16";
pub const szOID_BASIC_CONSTRAINTS2: &'static str = "2.5.29.19";
pub const szOID_CERT_POLICIES: &'static str = "2.5.29.32";
pub const szOID_ANY_CERT_POLICY: &'static str = "2.5.29.32.0";
pub const szOID_INHIBIT_ANY_POLICY: &'static str = "2.5.29.54";
pub const szOID_AUTHORITY_KEY_IDENTIFIER2: &'static str = "2.5.29.35";
pub const szOID_SUBJECT_KEY_IDENTIFIER: &'static str = "2.5.29.14";
pub const szOID_SUBJECT_ALT_NAME2: &'static str = "2.5.29.17";
pub const szOID_ISSUER_ALT_NAME2: &'static str = "2.5.29.18";
pub const szOID_CRL_REASON_CODE: &'static str = "2.5.29.21";
pub const szOID_REASON_CODE_HOLD: &'static str = "2.5.29.23";
pub const szOID_CRL_DIST_POINTS: &'static str = "2.5.29.31";
pub const szOID_ENHANCED_KEY_USAGE: &'static str = "2.5.29.37";
pub const szOID_ANY_ENHANCED_KEY_USAGE: &'static str = "2.5.29.37.0";
pub const szOID_CRL_NUMBER: &'static str = "2.5.29.20";
pub const szOID_DELTA_CRL_INDICATOR: &'static str = "2.5.29.27";
pub const szOID_ISSUING_DIST_POINT: &'static str = "2.5.29.28";
pub const szOID_FRESHEST_CRL: &'static str = "2.5.29.46";
pub const szOID_NAME_CONSTRAINTS: &'static str = "2.5.29.30";
pub const szOID_POLICY_MAPPINGS: &'static str = "2.5.29.33";
pub const szOID_LEGACY_POLICY_MAPPINGS: &'static str = "2.5.29.5";
pub const szOID_POLICY_CONSTRAINTS: &'static str = "2.5.29.36";
pub const szOID_RENEWAL_CERTIFICATE: &'static str = "1.3.6.1.4.1.311.13.1";
pub const szOID_ENROLLMENT_NAME_VALUE_PAIR: &'static str = "1.3.6.1.4.1.311.13.2.1";
pub const szOID_ENROLLMENT_CSP_PROVIDER: &'static str = "1.3.6.1.4.1.311.13.2.2";
pub const szOID_OS_VERSION: &'static str = "1.3.6.1.4.1.311.13.2.3";
pub const szOID_ENROLLMENT_AGENT: &'static str = "1.3.6.1.4.1.311.20.2.1";
pub const szOID_PKIX: &'static str = "1.3.6.1.5.5.7";
pub const szOID_PKIX_PE: &'static str = "1.3.6.1.5.5.7.1";
pub const szOID_AUTHORITY_INFO_ACCESS: &'static str = "1.3.6.1.5.5.7.1.1";
pub const szOID_SUBJECT_INFO_ACCESS: &'static str = "1.3.6.1.5.5.7.1.11";
pub const szOID_BIOMETRIC_EXT: &'static str = "1.3.6.1.5.5.7.1.2";
pub const szOID_QC_STATEMENTS_EXT: &'static str = "1.3.6.1.5.5.7.1.3";
pub const szOID_LOGOTYPE_EXT: &'static str = "1.3.6.1.5.5.7.1.12";
pub const szOID_TLS_FEATURES_EXT: &'static str = "1.3.6.1.5.5.7.1.24";
pub const szOID_CERT_EXTENSIONS: &'static str = "1.3.6.1.4.1.311.2.1.14";
pub const szOID_NEXT_UPDATE_LOCATION: &'static str = "1.3.6.1.4.1.311.10.2";
pub const szOID_REMOVE_CERTIFICATE: &'static str = "1.3.6.1.4.1.311.10.8.1";
pub const szOID_CROSS_CERT_DIST_POINTS: &'static str = "1.3.6.1.4.1.311.10.9.1";
pub const szOID_CTL: &'static str = "1.3.6.1.4.1.311.10.1";
pub const szOID_SORTED_CTL: &'static str = "1.3.6.1.4.1.311.10.1.1";
pub const szOID_SERIALIZED: &'static str = "1.3.6.1.4.1.311.10.3.3.1";
pub const szOID_NT_PRINCIPAL_NAME: &'static str = "1.3.6.1.4.1.311.20.2.3";
pub const szOID_INTERNATIONALIZED_EMAIL_ADDRESS: &'static str = "1.3.6.1.4.1.311.20.2.4";
pub const szOID_PRODUCT_UPDATE: &'static str = "1.3.6.1.4.1.311.31.1";
pub const szOID_ANY_APPLICATION_POLICY: &'static str = "1.3.6.1.4.1.311.10.12.1";
pub const szOID_AUTO_ENROLL_CTL_USAGE: &'static str = "1.3.6.1.4.1.311.20.1";
pub const szOID_ENROLL_CERTTYPE_EXTENSION: &'static str = "1.3.6.1.4.1.311.20.2";
pub const szOID_CERT_MANIFOLD: &'static str = "1.3.6.1.4.1.311.20.3";
pub const szOID_CERTSRV_CA_VERSION: &'static str = "1.3.6.1.4.1.311.21.1";
pub const szOID_CERTSRV_PREVIOUS_CERT_HASH: &'static str = "1.3.6.1.4.1.311.21.2";
pub const szOID_CRL_VIRTUAL_BASE: &'static str = "1.3.6.1.4.1.311.21.3";
pub const szOID_CRL_NEXT_PUBLISH: &'static str = "1.3.6.1.4.1.311.21.4";
pub const szOID_KP_CA_EXCHANGE: &'static str = "1.3.6.1.4.1.311.21.5";
pub const szOID_KP_PRIVACY_CA: &'static str = "1.3.6.1.4.1.311.21.36";
pub const szOID_KP_KEY_RECOVERY_AGENT: &'static str = "1.3.6.1.4.1.311.21.6";
pub const szOID_CERTIFICATE_TEMPLATE: &'static str = "1.3.6.1.4.1.311.21.7";
pub const szOID_ENTERPRISE_OID_ROOT: &'static str = "1.3.6.1.4.1.311.21.8";
pub const szOID_RDN_DUMMY_SIGNER: &'static str = "1.3.6.1.4.1.311.21.9";
pub const szOID_APPLICATION_CERT_POLICIES: &'static str = "1.3.6.1.4.1.311.21.10";
pub const szOID_APPLICATION_POLICY_MAPPINGS: &'static str = "1.3.6.1.4.1.311.21.11";
pub const szOID_APPLICATION_POLICY_CONSTRAINTS: &'static str = "1.3.6.1.4.1.311.21.12";
pub const szOID_ARCHIVED_KEY_ATTR: &'static str = "1.3.6.1.4.1.311.21.13";
pub const szOID_CRL_SELF_CDP: &'static str = "1.3.6.1.4.1.311.21.14";
pub const szOID_REQUIRE_CERT_CHAIN_POLICY: &'static str = "1.3.6.1.4.1.311.21.15";
pub const szOID_ARCHIVED_KEY_CERT_HASH: &'static str = "1.3.6.1.4.1.311.21.16";
pub const szOID_ISSUED_CERT_HASH: &'static str = "1.3.6.1.4.1.311.21.17";
pub const szOID_DS_EMAIL_REPLICATION: &'static str = "1.3.6.1.4.1.311.21.19";
pub const szOID_REQUEST_CLIENT_INFO: &'static str = "1.3.6.1.4.1.311.21.20";
pub const szOID_ENCRYPTED_KEY_HASH: &'static str = "1.3.6.1.4.1.311.21.21";
pub const szOID_CERTSRV_CROSSCA_VERSION: &'static str = "1.3.6.1.4.1.311.21.22";
pub const szOID_NTDS_REPLICATION: &'static str = "1.3.6.1.4.1.311.25.1";
pub const szOID_SUBJECT_DIR_ATTRS: &'static str = "2.5.29.9";
pub const szOID_PKIX_KP: &'static str = "1.3.6.1.5.5.7.3";
pub const szOID_PKIX_KP_SERVER_AUTH: &'static str = "1.3.6.1.5.5.7.3.1";
pub const szOID_PKIX_KP_CLIENT_AUTH: &'static str = "1.3.6.1.5.5.7.3.2";
pub const szOID_PKIX_KP_CODE_SIGNING: &'static str = "1.3.6.1.5.5.7.3.3";
pub const szOID_PKIX_KP_EMAIL_PROTECTION: &'static str = "1.3.6.1.5.5.7.3.4";
pub const szOID_PKIX_KP_IPSEC_END_SYSTEM: &'static str = "1.3.6.1.5.5.7.3.5";
pub const szOID_PKIX_KP_IPSEC_TUNNEL: &'static str = "1.3.6.1.5.5.7.3.6";
pub const szOID_PKIX_KP_IPSEC_USER: &'static str = "1.3.6.1.5.5.7.3.7";
pub const szOID_PKIX_KP_TIMESTAMP_SIGNING: &'static str = "1.3.6.1.5.5.7.3.8";
pub const szOID_PKIX_KP_OCSP_SIGNING: &'static str = "1.3.6.1.5.5.7.3.9";
pub const szOID_PKIX_OCSP_NOCHECK: &'static str = "1.3.6.1.5.5.7.48.1.5";
pub const szOID_PKIX_OCSP_NONCE: &'static str = "1.3.6.1.5.5.7.48.1.2";
pub const szOID_IPSEC_KP_IKE_INTERMEDIATE: &'static str = "1.3.6.1.5.5.8.2.2";
pub const szOID_PKINIT_KP_KDC: &'static str = "1.3.6.1.5.2.3.5";
pub const szOID_KP_CTL_USAGE_SIGNING: &'static str = "1.3.6.1.4.1.311.10.3.1";
pub const szOID_KP_TIME_STAMP_SIGNING: &'static str = "1.3.6.1.4.1.311.10.3.2";
pub const szOID_SERVER_GATED_CRYPTO: &'static str = "1.3.6.1.4.1.311.10.3.3";
pub const szOID_SGC_NETSCAPE: &'static str = "2.16.840.1.113730.4.1";
pub const szOID_KP_EFS: &'static str = "1.3.6.1.4.1.311.10.3.4";
pub const szOID_EFS_RECOVERY: &'static str = "1.3.6.1.4.1.311.10.3.4.1";
pub const szOID_WHQL_CRYPTO: &'static str = "1.3.6.1.4.1.311.10.3.5";
pub const szOID_ATTEST_WHQL_CRYPTO: &'static str = "1.3.6.1.4.1.311.10.3.5.1";
pub const szOID_NT5_CRYPTO: &'static str = "1.3.6.1.4.1.311.10.3.6";
pub const szOID_OEM_WHQL_CRYPTO: &'static str = "1.3.6.1.4.1.311.10.3.7";
pub const szOID_EMBEDDED_NT_CRYPTO: &'static str = "1.3.6.1.4.1.311.10.3.8";
pub const szOID_ROOT_LIST_SIGNER: &'static str = "1.3.6.1.4.1.311.10.3.9";
pub const szOID_KP_QUALIFIED_SUBORDINATION: &'static str = "1.3.6.1.4.1.311.10.3.10";
pub const szOID_KP_KEY_RECOVERY: &'static str = "1.3.6.1.4.1.311.10.3.11";
pub const szOID_KP_DOCUMENT_SIGNING: &'static str = "1.3.6.1.4.1.311.10.3.12";
pub const szOID_KP_LIFETIME_SIGNING: &'static str = "1.3.6.1.4.1.311.10.3.13";
pub const szOID_KP_MOBILE_DEVICE_SOFTWARE: &'static str = "1.3.6.1.4.1.311.10.3.14";
pub const szOID_KP_SMART_DISPLAY: &'static str = "1.3.6.1.4.1.311.10.3.15";
pub const szOID_KP_CSP_SIGNATURE: &'static str = "1.3.6.1.4.1.311.10.3.16";
pub const szOID_KP_FLIGHT_SIGNING: &'static str = "1.3.6.1.4.1.311.10.3.27";
pub const szOID_PLATFORM_MANIFEST_BINARY_ID: &'static str = "1.3.6.1.4.1.311.10.3.28";
pub const szOID_DRM: &'static str = "1.3.6.1.4.1.311.10.5.1";
pub const szOID_DRM_INDIVIDUALIZATION: &'static str = "1.3.6.1.4.1.311.10.5.2";
pub const szOID_LICENSES: &'static str = "1.3.6.1.4.1.311.10.6.1";
pub const szOID_LICENSE_SERVER: &'static str = "1.3.6.1.4.1.311.10.6.2";
pub const szOID_KP_SMARTCARD_LOGON: &'static str = "1.3.6.1.4.1.311.20.2.2";
pub const szOID_KP_KERNEL_MODE_CODE_SIGNING: &'static str = "1.3.6.1.4.1.311.61.1.1";
pub const szOID_KP_KERNEL_MODE_TRUSTED_BOOT_SIGNING: &'static str = "1.3.6.1.4.1.311.61.4.1";
pub const szOID_REVOKED_LIST_SIGNER: &'static str = "1.3.6.1.4.1.311.10.3.19";
pub const szOID_WINDOWS_KITS_SIGNER: &'static str = "1.3.6.1.4.1.311.10.3.20";
pub const szOID_WINDOWS_RT_SIGNER: &'static str = "1.3.6.1.4.1.311.10.3.21";
pub const szOID_PROTECTED_PROCESS_LIGHT_SIGNER: &'static str = "1.3.6.1.4.1.311.10.3.22";
pub const szOID_WINDOWS_TCB_SIGNER: &'static str = "1.3.6.1.4.1.311.10.3.23";
pub const szOID_PROTECTED_PROCESS_SIGNER: &'static str = "1.3.6.1.4.1.311.10.3.24";
pub const szOID_WINDOWS_THIRD_PARTY_COMPONENT_SIGNER: &'static str = "1.3.6.1.4.1.311.10.3.25";
pub const szOID_WINDOWS_SOFTWARE_EXTENSION_SIGNER: &'static str = "1.3.6.1.4.1.311.10.3.26";
pub const szOID_DISALLOWED_LIST: &'static str = "1.3.6.1.4.1.311.10.3.30";
pub const szOID_PIN_RULES_SIGNER: &'static str = "1.3.6.1.4.1.311.10.3.31";
pub const szOID_PIN_RULES_CTL: &'static str = "1.3.6.1.4.1.311.10.3.32";
pub const szOID_PIN_RULES_EXT: &'static str = "1.3.6.1.4.1.311.10.3.33";
pub const szOID_PIN_RULES_DOMAIN_NAME: &'static str = "1.3.6.1.4.1.311.10.3.34";
pub const szOID_PIN_RULES_LOG_END_DATE_EXT: &'static str = "1.3.6.1.4.1.311.10.3.35";
pub const szOID_IUM_SIGNING: &'static str = "1.3.6.1.4.1.311.10.3.37";
pub const szOID_EV_WHQL_CRYPTO: &'static str = "1.3.6.1.4.1.311.10.3.39";
pub const szOID_SYNC_ROOT_CTL_EXT: &'static str = "1.3.6.1.4.1.311.10.3.50";
pub const szOID_HPKP_DOMAIN_NAME_CTL: &'static str = "1.3.6.1.4.1.311.10.3.60";
pub const szOID_HPKP_HEADER_VALUE_CTL: &'static str = "1.3.6.1.4.1.311.10.3.61";
pub const szOID_KP_KERNEL_MODE_HAL_EXTENSION_SIGNING: &'static str = "1.3.6.1.4.1.311.61.5.1";
pub const szOID_WINDOWS_STORE_SIGNER: &'static str = "1.3.6.1.4.1.311.76.3.1";
pub const szOID_DYNAMIC_CODE_GEN_SIGNER: &'static str = "1.3.6.1.4.1.311.76.5.1";
pub const szOID_MICROSOFT_PUBLISHER_SIGNER: &'static str = "1.3.6.1.4.1.311.76.8.1";
pub const szOID_YESNO_TRUST_ATTR: &'static str = "1.3.6.1.4.1.311.10.4.1";
pub const szOID_SITE_PIN_RULES_INDEX_ATTR: &'static str = "1.3.6.1.4.1.311.10.4.2";
pub const szOID_SITE_PIN_RULES_FLAGS_ATTR: &'static str = "1.3.6.1.4.1.311.10.4.3";
pub const szOID_PKIX_POLICY_QUALIFIER_CPS: &'static str = "1.3.6.1.5.5.7.2.1";
pub const szOID_PKIX_POLICY_QUALIFIER_USERNOTICE: &'static str = "1.3.6.1.5.5.7.2.2";
pub const szOID_ROOT_PROGRAM_FLAGS: &'static str = "1.3.6.1.4.1.311.60.1.1";
pub const CERT_ROOT_PROGRAM_FLAG_ORG: DWORD = 0x80;
pub const CERT_ROOT_PROGRAM_FLAG_LSC: DWORD = 0x40;
pub const CERT_ROOT_PROGRAM_FLAG_SUBJECT_LOGO: DWORD = 0x20;
pub const CERT_ROOT_PROGRAM_FLAG_OU: DWORD = 0x10;
pub const CERT_ROOT_PROGRAM_FLAG_ADDRESS: DWORD = 0x08;
pub const szOID_CERT_POLICIES_95_QUALIFIER1: &'static str = "2.16.840.1.113733.1.7.1.1";
pub const szOID_RDN_TPM_MANUFACTURER: &'static str = "2.23.133.2.1";
pub const szOID_RDN_TPM_MODEL: &'static str = "2.23.133.2.2";
pub const szOID_RDN_TPM_VERSION: &'static str = "2.23.133.2.3";
pub const szOID_RDN_TCG_PLATFORM_MANUFACTURER: &'static str = "2.23.133.2.4";
pub const szOID_RDN_TCG_PLATFORM_MODEL: &'static str = "2.23.133.2.5";
pub const szOID_RDN_TCG_PLATFORM_VERSION: &'static str = "2.23.133.2.6";
pub const szOID_ENROLL_EK_INFO: &'static str = "1.3.6.1.4.1.311.21.23";
pub const szOID_ENROLL_AIK_INFO: &'static str = "1.3.6.1.4.1.311.21.39";
pub const szOID_ENROLL_ATTESTATION_STATEMENT: &'static str = "1.3.6.1.4.1.311.21.24";
pub const szOID_ENROLL_KSP_NAME: &'static str = "1.3.6.1.4.1.311.21.25";
pub const szOID_ENROLL_EKPUB_CHALLENGE: &'static str = "1.3.6.1.4.1.311.21.26";
pub const szOID_ENROLL_CAXCHGCERT_HASH: &'static str = "1.3.6.1.4.1.311.21.27";
pub const szOID_ENROLL_ATTESTATION_CHALLENGE: &'static str = "1.3.6.1.4.1.311.21.28";
pub const szOID_ENROLL_ENCRYPTION_ALGORITHM: &'static str = "1.3.6.1.4.1.311.21.29";
pub const szOID_KP_TPM_EK_CERTIFICATE: &'static str = "2.23.133.8.1";
pub const szOID_KP_TPM_PLATFORM_CERTIFICATE: &'static str = "2.23.133.8.2";
pub const szOID_KP_TPM_AIK_CERTIFICATE: &'static str = "2.23.133.8.3";
pub const szOID_ENROLL_EKVERIFYKEY: &'static str = "1.3.6.1.4.1.311.21.30";
pub const szOID_ENROLL_EKVERIFYCERT: &'static str = "1.3.6.1.4.1.311.21.31";
pub const szOID_ENROLL_EKVERIFYCREDS: &'static str = "1.3.6.1.4.1.311.21.32";
pub const szOID_ENROLL_SCEP_ERROR: &'static str = "1.3.6.1.4.1.311.21.33";
pub const szOID_ENROLL_SCEP_SERVER_STATE: &'static str = "1.3.6.1.4.1.311.21.34";
pub const szOID_ENROLL_SCEP_CHALLENGE_ANSWER: &'static str = "1.3.6.1.4.1.311.21.35";
pub const szOID_ENROLL_SCEP_CLIENT_REQUEST: &'static str = "1.3.6.1.4.1.311.21.37";
pub const szOID_ENROLL_SCEP_SERVER_MESSAGE: &'static str = "1.3.6.1.4.1.311.21.38";
pub const szOID_ENROLL_SCEP_SERVER_SECRET: &'static str = "1.3.6.1.4.1.311.21.40";
pub const szOID_ENROLL_KEY_AFFINITY: &'static str = "1.3.6.1.4.1.311.21.41";
pub const szOID_ENROLL_SCEP_SIGNER_HASH: &'static str = "1.3.6.1.4.1.311.21.42";
pub const szOID_ENROLL_EK_CA_KEYID: &'static str = "1.3.6.1.4.1.311.21.43";
pub const szOID_ATTR_SUPPORTED_ALGORITHMS: &'static str = "2.5.4.52";
pub const szOID_ATTR_TPM_SPECIFICATION: &'static str = "2.23.133.2.16";
pub const szOID_ATTR_PLATFORM_SPECIFICATION: &'static str = "2.23.133.2.17";
pub const szOID_ATTR_TPM_SECURITY_ASSERTIONS: &'static str = "2.23.133.2.18";
STRUCT!{struct CERT_EXTENSIONS {
    cExtension: DWORD,
    rgExtension: PCERT_EXTENSION,
}}
pub type PCERT_EXTENSIONS = *mut CERT_EXTENSIONS;
pub const CERT_UNICODE_RDN_ERR_INDEX_MASK: DWORD = 0x3FF;
pub const CERT_UNICODE_RDN_ERR_INDEX_SHIFT: DWORD = 22;
pub const CERT_UNICODE_ATTR_ERR_INDEX_MASK: DWORD = 0x003F;
pub const CERT_UNICODE_ATTR_ERR_INDEX_SHIFT: DWORD = 16;
pub const CERT_UNICODE_VALUE_ERR_INDEX_MASK: DWORD = 0x0000FFFF;
pub const CERT_UNICODE_VALUE_ERR_INDEX_SHIFT: DWORD = 0;
#[inline]
pub fn GET_CERT_UNICODE_RDN_ERR_INDEX(X: DWORD) -> DWORD {
    (X >> CERT_UNICODE_RDN_ERR_INDEX_SHIFT) & CERT_UNICODE_RDN_ERR_INDEX_MASK
}
#[inline]
pub fn GET_CERT_UNICODE_ATTR_ERR_INDEX(X: DWORD) -> DWORD {
    (X >> CERT_UNICODE_ATTR_ERR_INDEX_SHIFT) & CERT_UNICODE_ATTR_ERR_INDEX_MASK
}
#[inline]
pub fn GET_CERT_UNICODE_VALUE_ERR_INDEX(X: DWORD) -> DWORD {
    X & CERT_UNICODE_VALUE_ERR_INDEX_MASK
}
STRUCT!{struct CERT_AUTHORITY_KEY_ID_INFO {
    KeyId: CRYPT_DATA_BLOB,
    CertIssuer: CERT_NAME_BLOB,
    CertSerialNumber: CRYPT_INTEGER_BLOB,
}}
pub type PCERT_AUTHORITY_KEY_ID_INFO = *mut CERT_AUTHORITY_KEY_ID_INFO;
STRUCT!{struct CERT_PRIVATE_KEY_VALIDITY {
    NotBefore: FILETIME,
    NotAfter: FILETIME,
}}
pub type PCERT_PRIVATE_KEY_VALIDITY = *mut CERT_PRIVATE_KEY_VALIDITY;
STRUCT!{struct CERT_KEY_ATTRIBUTES_INFO {
    KeyId: CRYPT_DATA_BLOB,
    IntendedKeyUsage: CRYPT_BIT_BLOB,
    pPrivateKeyUsagePeriod: PCERT_PRIVATE_KEY_VALIDITY,
}}
pub type PCERT_KEY_ATTRIBUTES_INFO = *mut CERT_KEY_ATTRIBUTES_INFO;
pub const CERT_DIGITAL_SIGNATURE_KEY_USAGE: DWORD = 0x80;
pub const CERT_NON_REPUDIATION_KEY_USAGE: DWORD = 0x40;
pub const CERT_KEY_ENCIPHERMENT_KEY_USAGE: DWORD = 0x20;
pub const CERT_DATA_ENCIPHERMENT_KEY_USAGE: DWORD = 0x10;
pub const CERT_KEY_AGREEMENT_KEY_USAGE: DWORD = 0x08;
pub const CERT_KEY_CERT_SIGN_KEY_USAGE: DWORD = 0x04;
pub const CERT_OFFLINE_CRL_SIGN_KEY_USAGE: DWORD = 0x02;
pub const CERT_CRL_SIGN_KEY_USAGE: DWORD = 0x02;
pub const CERT_ENCIPHER_ONLY_KEY_USAGE: DWORD = 0x01;
pub const CERT_DECIPHER_ONLY_KEY_USAGE: DWORD = 0x80;
STRUCT!{struct CERT_POLICY_ID {
    cCertPolicyElementId: DWORD,
    rgpszCertPolicyElementId: *mut LPSTR,
}}
pub type PCERT_POLICY_ID = *mut CERT_POLICY_ID;
STRUCT!{struct CERT_KEY_USAGE_RESTRICTION_INFO {
    cCertPolicyId: DWORD,
    rgCertPolicyId: PCERT_POLICY_ID,
    RestrictedKeyUsage: CRYPT_BIT_BLOB,
}}
pub type PCERT_KEY_USAGE_RESTRICTION_INFO = *mut CERT_KEY_USAGE_RESTRICTION_INFO;
STRUCT!{struct CERT_OTHER_NAME {
    pszObjId: LPSTR,
    Value: CRYPT_OBJID_BLOB,
}}
pub type PCERT_OTHER_NAME = *mut CERT_OTHER_NAME;
UNION!{union CERT_ALT_NAME_ENTRY_u {
    [usize; 2],
    pOtherName pOtherName_mut: PCERT_OTHER_NAME,
    pwszRfc822Name pwszRfc822Name_mut: LPWSTR,
    pwszDNSName pwszDNSName_mut: LPWSTR,
    DirectoryName DirectoryName_mut: CERT_NAME_BLOB,
    pwszURL pwszURL_mut: LPWSTR,
    IPAddress IPAddress_mut: CRYPT_DATA_BLOB,
    pszRegisteredID pszRegisteredID_mut: LPSTR,
}}
STRUCT!{struct CERT_ALT_NAME_ENTRY {
    dwAltNameChoice: DWORD,
    u: CERT_ALT_NAME_ENTRY_u,
}}
pub type PCERT_ALT_NAME_ENTRY = *mut CERT_ALT_NAME_ENTRY;
pub const CERT_ALT_NAME_OTHER_NAME: DWORD = 1;
pub const CERT_ALT_NAME_RFC822_NAME: DWORD = 2;
pub const CERT_ALT_NAME_DNS_NAME: DWORD = 3;
pub const CERT_ALT_NAME_X400_ADDRESS: DWORD = 4;
pub const CERT_ALT_NAME_DIRECTORY_NAME: DWORD = 5;
pub const CERT_ALT_NAME_EDI_PARTY_NAME: DWORD = 6;
pub const CERT_ALT_NAME_URL: DWORD = 7;
pub const CERT_ALT_NAME_IP_ADDRESS: DWORD = 8;
pub const CERT_ALT_NAME_REGISTERED_ID: DWORD = 9;
STRUCT!{struct CERT_ALT_NAME_INFO {
    cAltEntry: DWORD,
    rgAltEntry: PCERT_ALT_NAME_ENTRY,
}}
pub type PCERT_ALT_NAME_INFO = *mut CERT_ALT_NAME_INFO;
pub const CERT_ALT_NAME_ENTRY_ERR_INDEX_MASK: DWORD = 0xFF;
pub const CERT_ALT_NAME_ENTRY_ERR_INDEX_SHIFT: DWORD = 16;
pub const CERT_ALT_NAME_VALUE_ERR_INDEX_MASK: DWORD = 0x0000FFFF;
pub const CERT_ALT_NAME_VALUE_ERR_INDEX_SHIFT: DWORD = 0;
#[inline]
pub fn GET_CERT_ALT_NAME_ENTRY_ERR_INDEX(X: DWORD) -> DWORD {
    (X >> CERT_ALT_NAME_ENTRY_ERR_INDEX_SHIFT) & CERT_ALT_NAME_ENTRY_ERR_INDEX_MASK
}
#[inline]
pub fn GET_CERT_ALT_NAME_VALUE_ERR_INDEX(X: DWORD) -> DWORD {
    X & CERT_ALT_NAME_VALUE_ERR_INDEX_MASK
}
STRUCT!{struct CERT_BASIC_CONSTRAINTS_INFO {
    SubjectType: CRYPT_BIT_BLOB,
    fPathLenConstraint: BOOL,
    dwPathLenConstraint: DWORD,
    cSubtreesConstraint: DWORD,
    rgSubtreesConstraint: *mut CERT_NAME_BLOB,
}}
pub type PCERT_BASIC_CONSTRAINTS_INFO = *mut CERT_BASIC_CONSTRAINTS_INFO;
pub const CERT_CA_SUBJECT_FLAG: DWORD = 0x80;
pub const CERT_END_ENTITY_SUBJECT_FLAG: DWORD = 0x40;
STRUCT!{struct CERT_BASIC_CONSTRAINTS2_INFO {
    fCA: BOOL,
    fPathLenConstraint: BOOL,
    dwPathLenConstraint: DWORD,
}}
pub type PCERT_BASIC_CONSTRAINTS2_INFO = *mut CERT_BASIC_CONSTRAINTS2_INFO;
STRUCT!{struct CERT_POLICY_QUALIFIER_INFO {
    pszPolicyQualifierId: LPSTR,
    Qualifier: CRYPT_OBJID_BLOB,
}}
pub type PCERT_POLICY_QUALIFIER_INFO = *mut CERT_POLICY_QUALIFIER_INFO;
STRUCT!{struct CERT_POLICY_INFO {
    pszPolicyIdentifier: LPSTR,
    cPolicyQualifier: DWORD,
    rgPolicyQualifier: *mut CERT_POLICY_QUALIFIER_INFO,
}}
pub type PCERT_POLICY_INFO = *mut CERT_POLICY_INFO;
STRUCT!{struct CERT_POLICIES_INFO {
    cPolicyInfo: DWORD,
    rgPolicyInfo: *mut CERT_POLICY_INFO,
}}
pub type PCERT_POLICIES_INFO = *mut CERT_POLICIES_INFO;
STRUCT!{struct CERT_POLICY_QUALIFIER_NOTICE_REFERENCE {
    pszOrganization: LPSTR,
    cNoticeNumbers: DWORD,
    rgNoticeNumbers: *mut c_int,
}}
pub type PCERT_POLICY_QUALIFIER_NOTICE_REFERENCE = *mut CERT_POLICY_QUALIFIER_NOTICE_REFERENCE;
STRUCT!{struct CERT_POLICY_QUALIFIER_USER_NOTICE {
    pNoticeReference: *mut CERT_POLICY_QUALIFIER_NOTICE_REFERENCE,
    pszDisplayText: LPWSTR,
}}
pub type PCERT_POLICY_QUALIFIER_USER_NOTICE = *mut CERT_POLICY_QUALIFIER_USER_NOTICE;
STRUCT!{struct CPS_URLS {
    pszURL: LPWSTR,
    pAlgorithm: *mut CRYPT_ALGORITHM_IDENTIFIER,
    pDigest: *mut CRYPT_DATA_BLOB,
}}
pub type PCPS_URLS = *mut CPS_URLS;
STRUCT!{struct CERT_POLICY95_QUALIFIER1 {
    pszPracticesReference: LPWSTR,
    pszNoticeIdentifier: LPSTR,
    pszNSINoticeIdentifier: LPSTR,
    cCPSURLs: DWORD,
    rgCPSURLs: *mut CPS_URLS,
}}
pub type PCERT_POLICY95_QUALIFIER1 = *mut CERT_POLICY95_QUALIFIER1;
STRUCT!{struct CERT_POLICY_MAPPING {
    pszIssuerDomainPolicy: LPSTR,
    pszSubjectDomainPolicy: LPSTR,
}}
pub type PCERT_POLICY_MAPPING = *mut CERT_POLICY_MAPPING;
STRUCT!{struct CERT_POLICY_MAPPINGS_INFO {
    cPolicyMapping: DWORD,
    rgPolicyMapping: PCERT_POLICY_MAPPING,
}}
pub type PCERT_POLICY_MAPPINGS_INFO = *mut CERT_POLICY_MAPPINGS_INFO;
STRUCT!{struct CERT_POLICY_CONSTRAINTS_INFO {
    fRequireExplicitPolicy: BOOL,
    dwRequireExplicitPolicySkipCerts: DWORD,
    fInhibitPolicyMapping: BOOL,
    dwInhibitPolicyMappingSkipCerts: DWORD,
}}
pub type PCERT_POLICY_CONSTRAINTS_INFO = *mut CERT_POLICY_CONSTRAINTS_INFO;
STRUCT!{struct CRYPT_CONTENT_INFO_SEQUENCE_OF_ANY {
    pszObjId: LPSTR,
    cValue: DWORD,
    rgValue: PCRYPT_DER_BLOB,
}}
pub type PCRYPT_CONTENT_INFO_SEQUENCE_OF_ANY = *mut CRYPT_CONTENT_INFO_SEQUENCE_OF_ANY;
STRUCT!{struct CRYPT_CONTENT_INFO {
    pszObjId: LPSTR,
    Content: CRYPT_DER_BLOB,
}}
pub type PCRYPT_CONTENT_INFO = *mut CRYPT_CONTENT_INFO;
STRUCT!{struct CRYPT_SEQUENCE_OF_ANY {
    cValue: DWORD,
    rgValue: PCRYPT_DER_BLOB,
}}
pub type PCRYPT_SEQUENCE_OF_ANY = *mut CRYPT_SEQUENCE_OF_ANY;
STRUCT!{struct CERT_AUTHORITY_KEY_ID2_INFO {
    KeyId: CRYPT_DATA_BLOB,
    AuthorityCertIssuer: CERT_ALT_NAME_INFO,
    AuthorityCertSerialNumber: CRYPT_INTEGER_BLOB,
}}
pub type PCERT_AUTHORITY_KEY_ID2_INFO = *mut CERT_AUTHORITY_KEY_ID2_INFO;
STRUCT!{struct CERT_ACCESS_DESCRIPTION {
    pszAccessMethod: LPSTR,
    AccessLocation: CERT_ALT_NAME_ENTRY,
}}
pub type PCERT_ACCESS_DESCRIPTION = *mut CERT_ACCESS_DESCRIPTION;
STRUCT!{struct CERT_AUTHORITY_INFO_ACCESS {
    cAccDescr: DWORD,
    rgAccDescr: PCERT_ACCESS_DESCRIPTION,
}}
pub type PCERT_AUTHORITY_INFO_ACCESS = *mut CERT_AUTHORITY_INFO_ACCESS;
pub type CERT_SUBJECT_INFO_ACCESS = CERT_AUTHORITY_INFO_ACCESS;
pub type PCERT_SUBJECT_INFO_ACCESS = *mut CERT_AUTHORITY_INFO_ACCESS;
pub const szOID_PKIX_ACC_DESCR: &'static str = "1.3.6.1.5.5.7.48";
pub const szOID_PKIX_OCSP: &'static str = "1.3.6.1.5.5.7.48.1";
pub const szOID_PKIX_CA_ISSUERS: &'static str = "1.3.6.1.5.5.7.48.2";
pub const szOID_PKIX_TIME_STAMPING: &'static str = "1.3.6.1.5.5.7.48.3";
pub const szOID_PKIX_CA_REPOSITORY: &'static str = "1.3.6.1.5.5.7.48.5";
pub const CRL_REASON_UNSPECIFIED: DWORD = 0;
pub const CRL_REASON_KEY_COMPROMISE: DWORD = 1;
pub const CRL_REASON_CA_COMPROMISE: DWORD = 2;
pub const CRL_REASON_AFFILIATION_CHANGED: DWORD = 3;
pub const CRL_REASON_SUPERSEDED: DWORD = 4;
pub const CRL_REASON_CESSATION_OF_OPERATION: DWORD = 5;
pub const CRL_REASON_CERTIFICATE_HOLD: DWORD = 6;
pub const CRL_REASON_REMOVE_FROM_CRL: DWORD = 8;
pub const CRL_REASON_PRIVILEGE_WITHDRAWN: DWORD = 9;
pub const CRL_REASON_AA_COMPROMISE: DWORD = 10;
UNION!{union CRL_DIST_POINT_NAME_u {
    [usize; 2],
    FullName FullName_mut: CERT_ALT_NAME_INFO,
}}
STRUCT!{struct CRL_DIST_POINT_NAME {
    dwDistPointNameChoice: DWORD,
    u: CRL_DIST_POINT_NAME_u,
}}
pub type PCRL_DIST_POINT_NAME = *mut CRL_DIST_POINT_NAME;
pub const CRL_DIST_POINT_NO_NAME: DWORD = 0;
pub const CRL_DIST_POINT_FULL_NAME: DWORD = 1;
pub const CRL_DIST_POINT_ISSUER_RDN_NAME: DWORD = 2;
STRUCT!{struct CRL_DIST_POINT {
    DistPointName: CRL_DIST_POINT_NAME,
    ReasonFlags: CRYPT_BIT_BLOB,
    CRLIssuer: CERT_ALT_NAME_INFO,
}}
pub type PCRL_DIST_POINT = *mut CRL_DIST_POINT;
pub const CRL_REASON_UNUSED_FLAG: DWORD = 0x80;
pub const CRL_REASON_KEY_COMPROMISE_FLAG: DWORD = 0x40;
pub const CRL_REASON_CA_COMPROMISE_FLAG: DWORD = 0x20;
pub const CRL_REASON_AFFILIATION_CHANGED_FLAG: DWORD = 0x10;
pub const CRL_REASON_SUPERSEDED_FLAG: DWORD = 0x08;
pub const CRL_REASON_CESSATION_OF_OPERATION_FLAG: DWORD = 0x04;
pub const CRL_REASON_CERTIFICATE_HOLD_FLAG: DWORD = 0x02;
pub const CRL_REASON_PRIVILEGE_WITHDRAWN_FLAG: DWORD = 0x01;
pub const CRL_REASON_AA_COMPROMISE_FLAG: DWORD = 0x80;
STRUCT!{struct CRL_DIST_POINTS_INFO {
    cDistPoint: DWORD,
    rgDistPoint: PCRL_DIST_POINT,
}}
pub type PCRL_DIST_POINTS_INFO = *mut CRL_DIST_POINTS_INFO;
pub const CRL_DIST_POINT_ERR_INDEX_MASK: DWORD = 0x7F;
pub const CRL_DIST_POINT_ERR_INDEX_SHIFT: DWORD = 24;
#[inline]
pub fn GET_CRL_DIST_POINT_ERR_INDEX(X: DWORD) -> DWORD {
    (X >> CRL_DIST_POINT_ERR_INDEX_SHIFT) & CRL_DIST_POINT_ERR_INDEX_MASK
}
pub const CRL_DIST_POINT_ERR_CRL_ISSUER_BIT: DWORD = 0x80000000;
#[inline]
pub fn IS_CRL_DIST_POINT_ERR_CRL_ISSUER(X: DWORD) -> bool {
    0 != (X & CRL_DIST_POINT_ERR_CRL_ISSUER_BIT)
}
STRUCT!{struct CROSS_CERT_DIST_POINTS_INFO {
    dwSyncDeltaTime: DWORD,
    cDistPoint: DWORD,
    rgDistPoint: PCERT_ALT_NAME_INFO,
}}
pub type PCROSS_CERT_DIST_POINTS_INFO = *mut CROSS_CERT_DIST_POINTS_INFO;
pub const CROSS_CERT_DIST_POINT_ERR_INDEX_MASK: DWORD = 0xFF;
pub const CROSS_CERT_DIST_POINT_ERR_INDEX_SHIFT: DWORD = 24;
#[inline]
pub fn GET_CROSS_CERT_DIST_POINT_ERR_INDEX(X: DWORD) -> DWORD {
    (X >> CROSS_CERT_DIST_POINT_ERR_INDEX_SHIFT) & CROSS_CERT_DIST_POINT_ERR_INDEX_MASK
}
STRUCT!{struct CERT_PAIR {
    Forward: CERT_BLOB,
    Reverse: CERT_BLOB,
}}
pub type PCERT_PAIR = *mut CERT_PAIR;
STRUCT!{struct CRL_ISSUING_DIST_POINT {
    DistPointName: CRL_DIST_POINT_NAME,
    fOnlyContainsUserCerts: BOOL,
    fOnlyContainsCACerts: BOOL,
    OnlySomeReasonFlags: CRYPT_BIT_BLOB,
    fIndirectCRL: BOOL,
}}
pub type PCRL_ISSUING_DIST_POINT = *mut CRL_ISSUING_DIST_POINT;
STRUCT!{struct CERT_GENERAL_SUBTREE {
    Base: CERT_ALT_NAME_ENTRY,
    dwMinimum: DWORD,
    fMaximum: BOOL,
    dwMaximum: DWORD,
}}
pub type PCERT_GENERAL_SUBTREE = *mut CERT_GENERAL_SUBTREE;
STRUCT!{struct CERT_NAME_CONSTRAINTS_INFO {
    cPermittedSubtree: DWORD,
    rgPermittedSubtree: PCERT_GENERAL_SUBTREE,
    cExcludedSubtree: DWORD,
    rgExcludedSubtree: PCERT_GENERAL_SUBTREE,
}}
pub type PCERT_NAME_CONSTRAINTS_INFO = *mut CERT_NAME_CONSTRAINTS_INFO;
pub const CERT_EXCLUDED_SUBTREE_BIT: DWORD = 0x80000000;
#[inline]
pub fn IS_CERT_EXCLUDED_SUBTREE(X: DWORD) -> bool {
    0 != (X & CERT_EXCLUDED_SUBTREE_BIT)
}
pub const SORTED_CTL_EXT_FLAGS_OFFSET: c_int = 0 * 4;
pub const SORTED_CTL_EXT_COUNT_OFFSET: c_int = 1 * 4;
pub const SORTED_CTL_EXT_MAX_COLLISION_OFFSET: c_int = 2 * 4;
pub const SORTED_CTL_EXT_HASH_BUCKET_OFFSET: c_int = 3 * 4;
pub const SORTED_CTL_EXT_HASHED_SUBJECT_IDENTIFIER_FLAG: DWORD = 0x1;
STRUCT!{struct CERT_DSS_PARAMETERS {
    p: CRYPT_UINT_BLOB,
    q: CRYPT_UINT_BLOB,
    g: CRYPT_UINT_BLOB,
}}
pub type PCERT_DSS_PARAMETERS = *mut CERT_DSS_PARAMETERS;
pub const CERT_DSS_R_LEN: usize = 20;
pub const CERT_DSS_S_LEN: usize = 20;
pub const CERT_DSS_SIGNATURE_LEN: usize = CERT_DSS_R_LEN + CERT_DSS_S_LEN;
pub const CERT_MAX_ASN_ENCODED_DSS_SIGNATURE_LEN: usize = 2 + 2 * (2 + 20 + 1);
STRUCT!{struct CERT_DH_PARAMETERS {
    p: CRYPT_UINT_BLOB,
    g: CRYPT_UINT_BLOB,
}}
pub type PCERT_DH_PARAMETERS = *mut CERT_DH_PARAMETERS;
STRUCT!{struct CERT_ECC_SIGNATURE {
    r: CRYPT_UINT_BLOB,
    s: CRYPT_UINT_BLOB,
}}
pub type PCERT_ECC_SIGNATURE = *mut CERT_ECC_SIGNATURE;
STRUCT!{struct CERT_X942_DH_VALIDATION_PARAMS {
    seed: CRYPT_BIT_BLOB,
    pgenCounter: DWORD,
}}
pub type PCERT_X942_DH_VALIDATION_PARAMS = *mut CERT_X942_DH_VALIDATION_PARAMS;
STRUCT!{struct CERT_X942_DH_PARAMETERS {
    p: CRYPT_UINT_BLOB,
    g: CRYPT_UINT_BLOB,
    q: CRYPT_UINT_BLOB,
    j: CRYPT_UINT_BLOB,
    pValidationParams: PCERT_X942_DH_VALIDATION_PARAMS,
}}
pub type PCERT_X942_DH_PARAMETERS = *mut CERT_X942_DH_PARAMETERS;
pub const CRYPT_X942_COUNTER_BYTE_LENGTH: usize = 4;
pub const CRYPT_X942_KEY_LENGTH_BYTE_LENGTH: usize = 4;
pub const CRYPT_X942_PUB_INFO_BYTE_LENGTH: usize = 512 / 8;
STRUCT!{struct CRYPT_X942_OTHER_INFO {
    pszContentEncryptionObjId: LPSTR,
    rgbCounter: [BYTE; CRYPT_X942_COUNTER_BYTE_LENGTH],
    rgbKeyLength: [BYTE; CRYPT_X942_KEY_LENGTH_BYTE_LENGTH],
    PubInfo: CRYPT_DATA_BLOB,
}}
pub type PCRYPT_X942_OTHER_INFO = *mut CRYPT_X942_OTHER_INFO;
pub const CRYPT_ECC_CMS_SHARED_INFO_SUPPPUBINFO_BYTE_LENGTH: usize = 4;
STRUCT!{struct CRYPT_ECC_CMS_SHARED_INFO {
    Algorithm: CRYPT_ALGORITHM_IDENTIFIER,
    EntityUInfo: CRYPT_DATA_BLOB,
    rgbSuppPubInfo: [BYTE; CRYPT_ECC_CMS_SHARED_INFO_SUPPPUBINFO_BYTE_LENGTH],
}}
pub type PCRYPT_ECC_CMS_SHARED_INFO = *mut CRYPT_ECC_CMS_SHARED_INFO;
STRUCT!{struct CRYPT_RC2_CBC_PARAMETERS {
    dwVersion: DWORD,
    fIV: BOOL,
    rgbIV: [BYTE; 8],
}}
pub type PCRYPT_RC2_CBC_PARAMETERS = *mut CRYPT_RC2_CBC_PARAMETERS;
pub const CRYPT_RC2_40BIT_VERSION: DWORD = 160;
pub const CRYPT_RC2_56BIT_VERSION: DWORD = 52;
pub const CRYPT_RC2_64BIT_VERSION: DWORD = 120;
pub const CRYPT_RC2_128BIT_VERSION: DWORD = 58;
STRUCT!{struct CRYPT_SMIME_CAPABILITY {
    pszObjId: LPSTR,
    Parameters: CRYPT_OBJID_BLOB,
}}
pub type PCRYPT_SMIME_CAPABILITY = *mut CRYPT_SMIME_CAPABILITY;
STRUCT!{struct CRYPT_SMIME_CAPABILITIES {
    cCapability: DWORD,
    rgCapability: PCRYPT_SMIME_CAPABILITY,
}}
pub type PCRYPT_SMIME_CAPABILITIES = *mut CRYPT_SMIME_CAPABILITIES;
STRUCT!{struct CERT_QC_STATEMENT {
    pszStatementId: LPSTR,
    StatementInfo: CRYPT_OBJID_BLOB,
}}
pub type PCERT_QC_STATEMENT = *mut CERT_QC_STATEMENT;
STRUCT!{struct CERT_QC_STATEMENTS_EXT_INFO {
    cStatement: DWORD,
    rgStatement: PCERT_QC_STATEMENT,
}}
pub type PCERT_QC_STATEMENTS_EXT_INFO = *mut CERT_QC_STATEMENTS_EXT_INFO;
pub const szOID_QC_EU_COMPLIANCE: &'static str = "0.4.0.1862.1.1";
pub const szOID_QC_SSCD: &'static str = "0.4.0.1862.1.4";
STRUCT!{struct CRYPT_MASK_GEN_ALGORITHM {
    pszObjId: LPSTR,
    HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
}}
pub type PCRYPT_MASK_GEN_ALGORITHM = *mut CRYPT_MASK_GEN_ALGORITHM;
STRUCT!{struct CRYPT_RSA_SSA_PSS_PARAMETERS {
    HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    MaskGenAlgorithm: CRYPT_MASK_GEN_ALGORITHM,
    dwSaltLength: DWORD,
    dwTrailerField: DWORD,
}}
pub type PCRYPT_RSA_SSA_PSS_PARAMETERS = *mut CRYPT_RSA_SSA_PSS_PARAMETERS;
pub const PKCS_RSA_SSA_PSS_TRAILER_FIELD_BC: DWORD = 1;
STRUCT!{struct CRYPT_PSOURCE_ALGORITHM {
    pszObjId: LPSTR,
    EncodingParameters: CRYPT_DATA_BLOB,
}}
pub type PCRYPT_PSOURCE_ALGORITHM = *mut CRYPT_PSOURCE_ALGORITHM;
STRUCT!{struct CRYPT_RSAES_OAEP_PARAMETERS {
    HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    MaskGenAlgorithm: CRYPT_MASK_GEN_ALGORITHM,
    PSourceAlgorithm: CRYPT_PSOURCE_ALGORITHM,
}}
pub type PCRYPT_RSAES_OAEP_PARAMETERS = *mut CRYPT_RSAES_OAEP_PARAMETERS;
pub const szOID_VERISIGN_PRIVATE_6_9: &'static str = "2.16.840.1.113733.1.6.9";
pub const szOID_VERISIGN_ONSITE_JURISDICTION_HASH: &'static str = "2.16.840.1.113733.1.6.11";
pub const szOID_VERISIGN_BITSTRING_6_13: &'static str = "2.16.840.1.113733.1.6.13";
pub const szOID_VERISIGN_ISS_STRONG_CRYPTO: &'static str = "2.16.840.1.113733.1.8.1";
pub const szOIDVerisign_MessageType: &'static str = "2.16.840.1.113733.1.9.2";
pub const szOIDVerisign_PkiStatus: &'static str = "2.16.840.1.113733.1.9.3";
pub const szOIDVerisign_FailInfo: &'static str = "2.16.840.1.113733.1.9.4";
pub const szOIDVerisign_SenderNonce: &'static str = "2.16.840.1.113733.1.9.5";
pub const szOIDVerisign_RecipientNonce: &'static str = "2.16.840.1.113733.1.9.6";
pub const szOIDVerisign_TransactionID: &'static str = "2.16.840.1.113733.1.9.7";
pub const szOID_NETSCAPE: &'static str = "2.16.840.1.113730";
pub const szOID_NETSCAPE_CERT_EXTENSION: &'static str = "2.16.840.1.113730.1";
pub const szOID_NETSCAPE_CERT_TYPE: &'static str = "2.16.840.1.113730.1.1";
pub const szOID_NETSCAPE_BASE_URL: &'static str = "2.16.840.1.113730.1.2";
pub const szOID_NETSCAPE_REVOCATION_URL: &'static str = "2.16.840.1.113730.1.3";
pub const szOID_NETSCAPE_CA_REVOCATION_URL: &'static str = "2.16.840.1.113730.1.4";
pub const szOID_NETSCAPE_CERT_RENEWAL_URL: &'static str = "2.16.840.1.113730.1.7";
pub const szOID_NETSCAPE_CA_POLICY_URL: &'static str = "2.16.840.1.113730.1.8";
pub const szOID_NETSCAPE_SSL_SERVER_NAME: &'static str = "2.16.840.1.113730.1.12";
pub const szOID_NETSCAPE_COMMENT: &'static str = "2.16.840.1.113730.1.13";
pub const szOID_NETSCAPE_DATA_TYPE: &'static str = "2.16.840.1.113730.2";
pub const szOID_NETSCAPE_CERT_SEQUENCE: &'static str = "2.16.840.1.113730.2.5";
pub const NETSCAPE_SSL_CLIENT_AUTH_CERT_TYPE: DWORD = 0x80;
pub const NETSCAPE_SSL_SERVER_AUTH_CERT_TYPE: DWORD = 0x40;
pub const NETSCAPE_SMIME_CERT_TYPE: DWORD = 0x20;
pub const NETSCAPE_SIGN_CERT_TYPE: DWORD = 0x10;
pub const NETSCAPE_SSL_CA_CERT_TYPE: DWORD = 0x04;
pub const NETSCAPE_SMIME_CA_CERT_TYPE: DWORD = 0x02;
pub const NETSCAPE_SIGN_CA_CERT_TYPE: DWORD = 0x01;
pub const szOID_CT_PKI_DATA: &'static str = "1.3.6.1.5.5.7.12.2";
pub const szOID_CT_PKI_RESPONSE: &'static str = "1.3.6.1.5.5.7.12.3";
pub const szOID_PKIX_NO_SIGNATURE: &'static str = "1.3.6.1.5.5.7.6.2";
pub const szOID_CMC: &'static str = "1.3.6.1.5.5.7.7";
pub const szOID_CMC_STATUS_INFO: &'static str = "1.3.6.1.5.5.7.7.1";
pub const szOID_CMC_IDENTIFICATION: &'static str = "1.3.6.1.5.5.7.7.2";
pub const szOID_CMC_IDENTITY_PROOF: &'static str = "1.3.6.1.5.5.7.7.3";
pub const szOID_CMC_DATA_RETURN: &'static str = "1.3.6.1.5.5.7.7.4";
pub const szOID_CMC_TRANSACTION_ID: &'static str = "1.3.6.1.5.5.7.7.5";
pub const szOID_CMC_SENDER_NONCE: &'static str = "1.3.6.1.5.5.7.7.6";
pub const szOID_CMC_RECIPIENT_NONCE: &'static str = "1.3.6.1.5.5.7.7.7";
pub const szOID_CMC_ADD_EXTENSIONS: &'static str = "1.3.6.1.5.5.7.7.8";
pub const szOID_CMC_ENCRYPTED_POP: &'static str = "1.3.6.1.5.5.7.7.9";
pub const szOID_CMC_DECRYPTED_POP: &'static str = "1.3.6.1.5.5.7.7.10";
pub const szOID_CMC_LRA_POP_WITNESS: &'static str = "1.3.6.1.5.5.7.7.11";
pub const szOID_CMC_GET_CERT: &'static str = "1.3.6.1.5.5.7.7.15";
pub const szOID_CMC_GET_CRL: &'static str = "1.3.6.1.5.5.7.7.16";
pub const szOID_CMC_REVOKE_REQUEST: &'static str = "1.3.6.1.5.5.7.7.17";
pub const szOID_CMC_REG_INFO: &'static str = "1.3.6.1.5.5.7.7.18";
pub const szOID_CMC_RESPONSE_INFO: &'static str = "1.3.6.1.5.5.7.7.19";
pub const szOID_CMC_QUERY_PENDING: &'static str = "1.3.6.1.5.5.7.7.21";
pub const szOID_CMC_ID_POP_LINK_RANDOM: &'static str = "1.3.6.1.5.5.7.7.22";
pub const szOID_CMC_ID_POP_LINK_WITNESS: &'static str = "1.3.6.1.5.5.7.7.23";
pub const szOID_CMC_ID_CONFIRM_CERT_ACCEPTANCE: &'static str = "1.3.6.1.5.5.7.7.24";
pub const szOID_CMC_ADD_ATTRIBUTES: &'static str = "1.3.6.1.4.1.311.10.10.1";
STRUCT!{struct CMC_TAGGED_ATTRIBUTE {
    dwBodyPartID: DWORD,
    Attribute: CRYPT_ATTRIBUTE,
}}
pub type PCMC_TAGGED_ATTRIBUTE = *mut CMC_TAGGED_ATTRIBUTE;
STRUCT!{struct CMC_TAGGED_CERT_REQUEST {
    dwBodyPartID: DWORD,
    SignedCertRequest: CRYPT_DER_BLOB,
}}
pub type PCMC_TAGGED_CERT_REQUEST = *mut CMC_TAGGED_CERT_REQUEST;
UNION!{union CMC_TAGGED_REQUEST_u {
    [usize; 1],
    pTaggedCertRequest pTaggedCertRequest_mut: PCMC_TAGGED_CERT_REQUEST,
}}
STRUCT!{struct CMC_TAGGED_REQUEST {
    dwTaggedRequestChoice: DWORD,
    u: CMC_TAGGED_REQUEST_u,
}}
pub type PCMC_TAGGED_REQUEST = *mut CMC_TAGGED_REQUEST;
STRUCT!{struct CMC_TAGGED_CONTENT_INFO {
    dwBodyPartID: DWORD,
    EncodedContentInfo: CRYPT_DER_BLOB,
}}
pub type PCMC_TAGGED_CONTENT_INFO = *mut CMC_TAGGED_CONTENT_INFO;
STRUCT!{struct CMC_TAGGED_OTHER_MSG {
    dwBodyPartID: DWORD,
    pszObjId: LPSTR,
    Value: CRYPT_OBJID_BLOB,
}}
pub type PCMC_TAGGED_OTHER_MSG = *mut CMC_TAGGED_OTHER_MSG;
STRUCT!{struct CMC_DATA_INFO {
    cTaggedAttribute: DWORD,
    rgTaggedAttribute: PCMC_TAGGED_ATTRIBUTE,
    cTaggedRequest: DWORD,
    rgTaggedRequest: PCMC_TAGGED_REQUEST,
    cTaggedContentInfo: DWORD,
    rgTaggedContentInfo: PCMC_TAGGED_CONTENT_INFO,
    cTaggedOtherMsg: DWORD,
    rgTaggedOtherMsg: PCMC_TAGGED_OTHER_MSG,
}}
pub type PCMC_DATA_INFO = *mut CMC_DATA_INFO;
STRUCT!{struct CMC_RESPONSE_INFO {
    cTaggedAttribute: DWORD,
    rgTaggedAttribute: PCMC_TAGGED_ATTRIBUTE,
    cTaggedContentInfo: DWORD,
    rgTaggedContentInfo: PCMC_TAGGED_CONTENT_INFO,
    cTaggedOtherMsg: DWORD,
    rgTaggedOtherMsg: PCMC_TAGGED_OTHER_MSG,
}}
pub type PCMC_RESPONSE_INFO = *mut CMC_RESPONSE_INFO;
STRUCT!{struct CMC_PEND_INFO {
    PendToken: CRYPT_DATA_BLOB,
    PendTime: FILETIME,
}}
pub type PCMC_PEND_INFO = *mut CMC_PEND_INFO;
UNION!{union CMC_STATUS_INFO_u {
    [usize; 1],
    dwFailInfo dwFailInfo_mut: DWORD,
    pPendInfo pPendInfo_mut: PCMC_PEND_INFO,
}}
STRUCT!{struct CMC_STATUS_INFO {
    dwStatus: DWORD,
    cBodyList: DWORD,
    rgdwBodyList: *mut DWORD,
    pwszStatusString: LPWSTR,
    dwOtherInfoChoice: DWORD,
    u: CMC_STATUS_INFO_u,
}}
pub type PCMC_STATUS_INFO = *mut CMC_STATUS_INFO;
pub const CMC_OTHER_INFO_NO_CHOICE: DWORD = 0;
pub const CMC_OTHER_INFO_FAIL_CHOICE: DWORD = 1;
pub const CMC_OTHER_INFO_PEND_CHOICE: DWORD = 2;
pub const CMC_STATUS_SUCCESS: DWORD = 0;
pub const CMC_STATUS_FAILED: DWORD = 2;
pub const CMC_STATUS_PENDING: DWORD = 3;
pub const CMC_STATUS_NO_SUPPORT: DWORD = 4;
pub const CMC_STATUS_CONFIRM_REQUIRED: DWORD = 5;
pub const CMC_FAIL_BAD_ALG: DWORD = 0;
pub const CMC_FAIL_BAD_MESSAGE_CHECK: DWORD = 1;
pub const CMC_FAIL_BAD_REQUEST: DWORD = 2;
pub const CMC_FAIL_BAD_TIME: DWORD = 3;
pub const CMC_FAIL_BAD_CERT_ID: DWORD = 4;
pub const CMC_FAIL_UNSUPORTED_EXT: DWORD = 5;
pub const CMC_FAIL_MUST_ARCHIVE_KEYS: DWORD = 6;
pub const CMC_FAIL_BAD_IDENTITY: DWORD = 7;
pub const CMC_FAIL_POP_REQUIRED: DWORD = 8;
pub const CMC_FAIL_POP_FAILED: DWORD = 9;
pub const CMC_FAIL_NO_KEY_REUSE: DWORD = 10;
pub const CMC_FAIL_INTERNAL_CA_ERROR: DWORD = 11;
pub const CMC_FAIL_TRY_LATER: DWORD = 12;
STRUCT!{struct CMC_ADD_EXTENSIONS_INFO {
    dwCmcDataReference: DWORD,
    cCertReference: DWORD,
    rgdwCertReference: *mut DWORD,
    cExtension: DWORD,
    rgExtension: PCERT_EXTENSION,
}}
pub type PCMC_ADD_EXTENSIONS_INFO = *mut CMC_ADD_EXTENSIONS_INFO;
STRUCT!{struct CMC_ADD_ATTRIBUTES_INFO {
    dwCmcDataReference: DWORD,
    cCertReference: DWORD,
    rgdwCertReference: *mut DWORD,
    cAttribute: DWORD,
    rgAttribute: PCRYPT_ATTRIBUTE,
}}
pub type PCMC_ADD_ATTRIBUTES_INFO = *mut CMC_ADD_ATTRIBUTES_INFO;
STRUCT!{struct CERT_TEMPLATE_EXT {
    pszObjId: LPSTR,
    dwMajorVersion: DWORD,
    fMinorVersion: BOOL,
    dwMinorVersion: DWORD,
}}
pub type PCERT_TEMPLATE_EXT = *mut CERT_TEMPLATE_EXT;
STRUCT!{struct CERT_HASHED_URL {
    HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    Hash: CRYPT_HASH_BLOB,
    pwszUrl: LPWSTR,
}}
pub type PCERT_HASHED_URL = *mut CERT_HASHED_URL;
STRUCT!{struct CERT_LOGOTYPE_DETAILS {
    pwszMimeType: LPWSTR,
    cHashedUrl: DWORD,
    rgHashedUrl: PCERT_HASHED_URL,
}}
pub type PCERT_LOGOTYPE_DETAILS = *mut CERT_LOGOTYPE_DETAILS;
STRUCT!{struct CERT_LOGOTYPE_REFERENCE {
    cHashedUrl: DWORD,
    rgHashedUrl: PCERT_HASHED_URL,
}}
pub type PCERT_LOGOTYPE_REFERENCE = *mut CERT_LOGOTYPE_REFERENCE;
UNION!{union CERT_LOGOTYPE_IMAGE_INFO_u {
    [u32; 1],
    dwNumBits dwNumBits_mut: DWORD,
    dwTableSize dwTableSize_mut: DWORD,
}}
STRUCT!{struct CERT_LOGOTYPE_IMAGE_INFO {
    dwLogotypeImageInfoChoice: DWORD,
    dwFileSize: DWORD,
    dwXSize: DWORD,
    dwYSize: DWORD,
    dwLogotypeImageResolutionChoice: DWORD,
    u: CERT_LOGOTYPE_IMAGE_INFO_u,
    pwszLanguage: LPWSTR,
}}
pub type PCERT_LOGOTYPE_IMAGE_INFO = *mut CERT_LOGOTYPE_IMAGE_INFO;
pub const CERT_LOGOTYPE_GRAY_SCALE_IMAGE_INFO_CHOICE: DWORD = 1;
pub const CERT_LOGOTYPE_COLOR_IMAGE_INFO_CHOICE: DWORD = 2;
pub const CERT_LOGOTYPE_NO_IMAGE_RESOLUTION_CHOICE: DWORD = 0;
pub const CERT_LOGOTYPE_BITS_IMAGE_RESOLUTION_CHOICE: DWORD = 1;
pub const CERT_LOGOTYPE_TABLE_SIZE_IMAGE_RESOLUTION_CHOICE: DWORD = 2;
STRUCT!{struct CERT_LOGOTYPE_IMAGE {
    LogotypeDetails: CERT_LOGOTYPE_DETAILS,
    pLogotypeImageInfo: PCERT_LOGOTYPE_IMAGE_INFO,
}}
pub type PCERT_LOGOTYPE_IMAGE = *mut CERT_LOGOTYPE_IMAGE;
STRUCT!{struct CERT_LOGOTYPE_AUDIO_INFO {
    dwFileSize: DWORD,
    dwPlayTime: DWORD,
    dwChannels: DWORD,
    dwSampleRate: DWORD,
    pwszLanguage: LPWSTR,
}}
pub type PCERT_LOGOTYPE_AUDIO_INFO = *mut CERT_LOGOTYPE_AUDIO_INFO;
STRUCT!{struct CERT_LOGOTYPE_AUDIO {
    LogotypeDetails: CERT_LOGOTYPE_DETAILS,
    pLogotypeAudioInfo: PCERT_LOGOTYPE_AUDIO_INFO,
}}
pub type PCERT_LOGOTYPE_AUDIO = *mut CERT_LOGOTYPE_AUDIO;
STRUCT!{struct CERT_LOGOTYPE_DATA {
    cLogotypeImage: DWORD,
    rgLogotypeImage: PCERT_LOGOTYPE_IMAGE,
    cLogotypeAudio: DWORD,
    rgLogotypeAudio: PCERT_LOGOTYPE_AUDIO,
}}
pub type PCERT_LOGOTYPE_DATA = *mut CERT_LOGOTYPE_DATA;
UNION!{union CERT_LOGOTYPE_INFO_u {
    [usize; 1],
    pLogotypeDirectInfo pLogotypeDirectInfo_mut: PCERT_LOGOTYPE_DATA,
    pLogotypeIndirectInfo pLogotypeIndirectInfo__mut: PCERT_LOGOTYPE_REFERENCE,
}}
STRUCT!{struct CERT_LOGOTYPE_INFO {
    dwLogotypeInfoChoice: DWORD,
    u: CERT_LOGOTYPE_INFO_u,
}}
pub type PCERT_LOGOTYPE_INFO = *mut CERT_LOGOTYPE_INFO;
pub const CERT_LOGOTYPE_DIRECT_INFO_CHOICE: DWORD = 1;
pub const CERT_LOGOTYPE_INDIRECT_INFO_CHOICE: DWORD = 2;
STRUCT!{struct CERT_OTHER_LOGOTYPE_INFO {
    pszObjId: LPSTR,
    LogotypeInfo: CERT_LOGOTYPE_INFO,
}}
pub type PCERT_OTHER_LOGOTYPE_INFO = *mut CERT_OTHER_LOGOTYPE_INFO;
pub const szOID_LOYALTY_OTHER_LOGOTYPE: &'static str = "1.3.6.1.5.5.7.20.1";
pub const szOID_BACKGROUND_OTHER_LOGOTYPE: &'static str = "1.3.6.1.5.5.7.20.2";
STRUCT!{struct CERT_LOGOTYPE_EXT_INFO {
    cCommunityLogo: DWORD,
    rgCommunityLogo: PCERT_LOGOTYPE_INFO,
    pIssuerLogo: PCERT_LOGOTYPE_INFO,
    pSubjectLogo: PCERT_LOGOTYPE_INFO,
    cOtherLogo: DWORD,
    rgOtherLogo: PCERT_OTHER_LOGOTYPE_INFO,
}}
pub type PCERT_LOGOTYPE_EXT_INFO = *mut CERT_LOGOTYPE_EXT_INFO;
UNION!{union CERT_BIOMETRIC_DATA_u {
    [usize; 1],
    dwPredefined dwPredefined_mut: DWORD,
    pszObjId pszObjId_mut: LPSTR,
}}
STRUCT!{struct CERT_BIOMETRIC_DATA {
    dwTypeOfBiometricDataChoice: DWORD,
    u: CERT_BIOMETRIC_DATA_u,
    HashedUrl: CERT_HASHED_URL,
}}
pub type PCERT_BIOMETRIC_DATA = *mut CERT_BIOMETRIC_DATA;
pub const CERT_BIOMETRIC_PREDEFINED_DATA_CHOICE: DWORD = 1;
pub const CERT_BIOMETRIC_OID_DATA_CHOICE: DWORD = 2;
pub const CERT_BIOMETRIC_PICTURE_TYPE: DWORD = 0;
pub const CERT_BIOMETRIC_SIGNATURE_TYPE: DWORD = 1;
STRUCT!{struct CERT_BIOMETRIC_EXT_INFO {
    cBiometricData: DWORD,
    rgBiometricData: PCERT_BIOMETRIC_DATA,
}}
pub type PCERT_BIOMETRIC_EXT_INFO = *mut CERT_BIOMETRIC_EXT_INFO;
STRUCT!{struct OCSP_SIGNATURE_INFO {
    SignatureAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    Signature: CRYPT_BIT_BLOB,
    cCertEncoded: DWORD,
    rgCertEncoded: PCERT_BLOB,
}}
pub type POCSP_SIGNATURE_INFO = *mut OCSP_SIGNATURE_INFO;
STRUCT!{struct OCSP_SIGNED_REQUEST_INFO {
    ToBeSigned: CRYPT_DER_BLOB,
    pOptionalSignatureInfo: POCSP_SIGNATURE_INFO,
}}
pub type POCSP_SIGNED_REQUEST_INFO = *mut OCSP_SIGNED_REQUEST_INFO;
STRUCT!{struct OCSP_CERT_ID {
    HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    IssuerNameHash: CRYPT_HASH_BLOB,
    IssuerKeyHash: CRYPT_HASH_BLOB,
    SerialNumber: CRYPT_INTEGER_BLOB,
}}
pub type POCSP_CERT_ID = *mut OCSP_CERT_ID;
STRUCT!{struct OCSP_REQUEST_ENTRY {
    CertId: OCSP_CERT_ID,
    cExtension: DWORD,
    rgExtension: PCERT_EXTENSION,
}}
pub type POCSP_REQUEST_ENTRY = *mut OCSP_REQUEST_ENTRY;
STRUCT!{struct OCSP_REQUEST_INFO {
    dwVersion: DWORD,
    pRequestorName: PCERT_ALT_NAME_ENTRY,
    cRequestEntry: DWORD,
    rgRequestEntry: POCSP_REQUEST_ENTRY,
    cExtension: DWORD,
    rgExtension: PCERT_EXTENSION,
}}
pub type POCSP_REQUEST_INFO = *mut OCSP_REQUEST_INFO;
pub const OCSP_REQUEST_V1: DWORD = 0;
STRUCT!{struct OCSP_RESPONSE_INFO {
    dwStatus: DWORD,
    pszObjId: LPSTR,
    Value: CRYPT_OBJID_BLOB,
}}
pub type POCSP_RESPONSE_INFO = *mut OCSP_RESPONSE_INFO;
pub const OCSP_SUCCESSFUL_RESPONSE: DWORD = 0;
pub const OCSP_MALFORMED_REQUEST_RESPONSE: DWORD = 1;
pub const OCSP_INTERNAL_ERROR_RESPONSE: DWORD = 2;
pub const OCSP_TRY_LATER_RESPONSE: DWORD = 3;
pub const OCSP_SIG_REQUIRED_RESPONSE: DWORD = 5;
pub const OCSP_UNAUTHORIZED_RESPONSE: DWORD = 6;
pub const szOID_PKIX_OCSP_BASIC_SIGNED_RESPONSE: &'static str = "1.3.6.1.5.5.7.48.1.1";
STRUCT!{struct OCSP_BASIC_SIGNED_RESPONSE_INFO {
    ToBeSigned: CRYPT_DER_BLOB,
    SignatureInfo: OCSP_SIGNATURE_INFO,
}}
pub type POCSP_BASIC_SIGNED_RESPONSE_INFO = *mut OCSP_BASIC_SIGNED_RESPONSE_INFO;
STRUCT!{struct OCSP_BASIC_REVOKED_INFO {
    RevocationDate: FILETIME,
    dwCrlReasonCode: DWORD,
}}
pub type POCSP_BASIC_REVOKED_INFO = *mut OCSP_BASIC_REVOKED_INFO;
UNION!{union OCSP_BASIC_RESPONSE_ENTRY_u {
    [usize; 1],
    pRevokedInfo pRevokedInfo_mut: POCSP_BASIC_REVOKED_INFO,
}}
STRUCT!{struct OCSP_BASIC_RESPONSE_ENTRY {
    CertId: OCSP_CERT_ID,
    dwCertStatus: DWORD,
    u: OCSP_BASIC_RESPONSE_ENTRY_u,
    ThisUpdate: FILETIME,
    NextUpdate: FILETIME,
    cExtension: DWORD,
    rgExtension: PCERT_EXTENSION,
}}
pub type POCSP_BASIC_RESPONSE_ENTRY = *mut OCSP_BASIC_RESPONSE_ENTRY;
pub const OCSP_BASIC_GOOD_CERT_STATUS: DWORD = 0;
pub const OCSP_BASIC_REVOKED_CERT_STATUS: DWORD = 1;
pub const OCSP_BASIC_UNKNOWN_CERT_STATUS: DWORD = 2;
UNION!{union OCSP_BASIC_RESPONSE_INFO_u {
    [usize; 2],
    ByNameResponderId ByNameResponderId_mut: CERT_NAME_BLOB,
    ByKeyResponderId ByKeyResponderId_mut: CRYPT_HASH_BLOB,
}}
STRUCT!{struct OCSP_BASIC_RESPONSE_INFO {
    dwVersion: DWORD,
    dwResponderIdChoice: DWORD,
    u: OCSP_BASIC_RESPONSE_INFO_u,
    ProducedAt: FILETIME,
    cResponseEntry: DWORD,
    rgResponseEntry: POCSP_BASIC_RESPONSE_ENTRY,
    cExtension: DWORD,
    rgExtension: PCERT_EXTENSION,
}}
pub type POCSP_BASIC_RESPONSE_INFO = *mut OCSP_BASIC_RESPONSE_INFO;
pub const OCSP_BASIC_RESPONSE_V1: DWORD = 0;
pub const OCSP_BASIC_BY_NAME_RESPONDER_ID: DWORD = 1;
pub const OCSP_BASIC_BY_KEY_RESPONDER_ID: DWORD = 2;
STRUCT!{struct CERT_SUPPORTED_ALGORITHM_INFO {
    Algorithm: CRYPT_ALGORITHM_IDENTIFIER,
    IntendedKeyUsage: CRYPT_BIT_BLOB,
    IntendedCertPolicies: CERT_POLICIES_INFO,
}}
pub type PCERT_SUPPORTED_ALGORITHM_INFO = *mut CERT_SUPPORTED_ALGORITHM_INFO;
STRUCT!{struct CERT_TPM_SPECIFICATION_INFO {
    pwszFamily: LPWSTR,
    dwLevel: DWORD,
    dwRevision: DWORD,
}}
pub type PCERT_TPM_SPECIFICATION_INFO = *mut CERT_TPM_SPECIFICATION_INFO;
pub type HCRYPTOIDFUNCSET = *mut c_void;
pub type HCRYPTOIDFUNCADDR = *mut c_void;
pub const CRYPT_OID_ENCODE_OBJECT_FUNC: &'static str = "CryptDllEncodeObject";
pub const CRYPT_OID_DECODE_OBJECT_FUNC: &'static str = "CryptDllDecodeObject";
pub const CRYPT_OID_ENCODE_OBJECT_EX_FUNC: &'static str = "CryptDllEncodeObjectEx";
pub const CRYPT_OID_DECODE_OBJECT_EX_FUNC: &'static str = "CryptDllDecodeObjectEx";
pub const CRYPT_OID_CREATE_COM_OBJECT_FUNC: &'static str = "CryptDllCreateCOMObject";
pub const CRYPT_OID_VERIFY_REVOCATION_FUNC: &'static str = "CertDllVerifyRevocation";
pub const CRYPT_OID_VERIFY_CTL_USAGE_FUNC: &'static str = "CertDllVerifyCTLUsage";
pub const CRYPT_OID_FORMAT_OBJECT_FUNC: &'static str = "CryptDllFormatObject";
pub const CRYPT_OID_FIND_OID_INFO_FUNC: &'static str = "CryptDllFindOIDInfo";
pub const CRYPT_OID_FIND_LOCALIZED_NAME_FUNC: &'static str = "CryptDllFindLocalizedName";
pub const CRYPT_OID_REGPATH: &'static str = "Software\\Microsoft\\Cryptography\\OID";
pub const CRYPT_OID_REG_ENCODING_TYPE_PREFIX: &'static str = "EncodingType ";
pub const CRYPT_OID_REG_DLL_VALUE_NAME: &'static str = "Dll";
pub const CRYPT_OID_REG_FUNC_NAME_VALUE_NAME: &'static str = "FuncName";
pub const CRYPT_OID_REG_FLAGS_VALUE_NAME: &'static str = "CryptFlags";
pub const CRYPT_DEFAULT_OID: &'static str = "DEFAULT";
STRUCT!{struct CRYPT_OID_FUNC_ENTRY {
    pszOID: LPCSTR,
    pvFuncAddr: *mut c_void,
}}
pub type PCRYPT_OID_FUNC_ENTRY = *mut CRYPT_OID_FUNC_ENTRY;
pub const CRYPT_INSTALL_OID_FUNC_BEFORE_FLAG: DWORD = 1;
extern "system" {
    pub fn CryptInstallOIDFunctionAddress(
        hModule: HMODULE,
        dwEncodingType: DWORD,
        pszFuncName: LPCSTR,
        cFuncEntry: DWORD,
        rgFuncEntry: *const CRYPT_OID_FUNC_ENTRY,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn CryptInitOIDFunctionSet(
        pszFuncName: LPCSTR,
        dwFlags: DWORD,
    ) -> HCRYPTOIDFUNCSET;
    pub fn CryptGetOIDFunctionAddress(
        hFuncSet: HCRYPTOIDFUNCSET,
        dwEncodingType: DWORD,
        pszOID: LPCSTR,
        dwFlags: DWORD,
        ppvFuncAddr: *mut *mut c_void,
        phFuncAddr: *mut HCRYPTOIDFUNCADDR,
    ) -> BOOL;
}
pub const CRYPT_GET_INSTALLED_OID_FUNC_FLAG: DWORD = 0x1;
extern "system" {
    pub fn CryptGetDefaultOIDDllList(
        hFuncSet: HCRYPTOIDFUNCSET,
        dwEncodingType: DWORD,
        pwszDllList: *mut WCHAR,
        pcchDllList: *mut DWORD,
    ) -> BOOL;
    pub fn CryptGetDefaultOIDFunctionAddress(
        hFuncSet: HCRYPTOIDFUNCSET,
        dwEncodingType: DWORD,
        pwszDll: LPCWSTR,
        dwFlags: DWORD,
        ppvFuncAddr: *mut *mut c_void,
        phFuncAddr: *mut HCRYPTOIDFUNCADDR,
    ) -> BOOL;
    pub fn CryptFreeOIDFunctionAddress(
        hFuncAddr: HCRYPTOIDFUNCADDR,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn CryptRegisterOIDFunction(
        dwEncodingType: DWORD,
        pszFuncName: LPCSTR,
        pszOID: LPCSTR,
        pwszDll: LPCWSTR,
        pszOverrideFuncName: LPCSTR,
    ) -> BOOL;
    pub fn CryptUnregisterOIDFunction(
        dwEncodingType: DWORD,
        pszFuncName: LPCSTR,
        pszOID: LPCSTR,
    ) -> BOOL;
    pub fn CryptRegisterDefaultOIDFunction(
        dwEncodingType: DWORD,
        pszFuncName: LPCSTR,
        dwIndex: DWORD,
        pwszDll: LPCWSTR,
    ) -> BOOL;
}
pub const CRYPT_REGISTER_FIRST_INDEX: DWORD = 0;
pub const CRYPT_REGISTER_LAST_INDEX: DWORD = 0xFFFFFFFF;
extern "system" {
    pub fn CryptUnregisterDefaultOIDFunction(
        dwEncodingType: DWORD,
        pszFuncName: LPCSTR,
        pwszDll: LPCWSTR,
    ) -> BOOL;
    pub fn CryptSetOIDFunctionValue(
        dwEncodingType: DWORD,
        pszFuncName: LPCSTR,
        pszOID: LPCSTR,
        pwszValueName: LPCWSTR,
        dwValueType: DWORD,
        pbValueData: *const BYTE,
        cbValueData: DWORD,
    ) -> BOOL;
    pub fn CryptGetOIDFunctionValue(
        dwEncodingType: DWORD,
        pszFuncName: LPCSTR,
        pszOID: LPCSTR,
        pwszValueName: LPCWSTR,
        pdwValueType: *mut DWORD,
        pbValueData: *mut BYTE,
        pcbValueData: *mut DWORD,
    ) -> BOOL;
}
FN!{stdcall PFN_CRYPT_ENUM_OID_FUNC(
    dwEncodingType: DWORD,
    pszFuncName: LPCSTR,
    pszOID: LPCSTR,
    cValue: DWORD,
    rgdwValueType: *const DWORD,
    rgpwszValueName: *const LPCWSTR,
    rgpbValueData: *const *const BYTE,
    rgcbValueData: *const DWORD,
    pvArg: *mut c_void,
) -> BOOL}
extern "system" {
    pub fn CryptEnumOIDFunction(
        dwEncodingType: DWORD,
        pszFuncName: LPCSTR,
        pszOID: LPCSTR,
        dwFlags: DWORD,
        pvArg: *mut c_void,
        pfnEnumOIDFunc: PFN_CRYPT_ENUM_OID_FUNC,
    ) -> BOOL;
}
pub const CRYPT_MATCH_ANY_ENCODING_TYPE: DWORD = 0xFFFFFFFF;
pub const CALG_OID_INFO_CNG_ONLY: ALG_ID = 0xFFFFFFFF;
pub const CALG_OID_INFO_PARAMETERS: ALG_ID = 0xFFFFFFFE;
#[inline]
pub fn IS_SPECIAL_OID_INFO_ALGID(Algid: ALG_ID) -> bool {
    Algid >= CALG_OID_INFO_PARAMETERS
}
pub const CRYPT_OID_INFO_HASH_PARAMETERS_ALGORITHM: &'static str = "CryptOIDInfoHashParameters";
pub const CRYPT_OID_INFO_ECC_PARAMETERS_ALGORITHM: &'static str = "CryptOIDInfoECCParameters";
pub const CRYPT_OID_INFO_MGF1_PARAMETERS_ALGORITHM: &'static str = "CryptOIDInfoMgf1Parameters";
pub const CRYPT_OID_INFO_NO_SIGN_ALGORITHM: &'static str = "CryptOIDInfoNoSign";
pub const CRYPT_OID_INFO_OAEP_PARAMETERS_ALGORITHM: &'static str = "CryptOIDInfoOAEPParameters";
pub const CRYPT_OID_INFO_ECC_WRAP_PARAMETERS_ALGORITHM: &'static str
    = "CryptOIDInfoECCWrapParameters";
pub const CRYPT_OID_INFO_NO_PARAMETERS_ALGORITHM: &'static str = "CryptOIDInfoNoParameters";
UNION!{union CRYPT_OID_INFO_u {
    [u32; 1],
    dwValue dwValue_mut: DWORD,
    Algid Algid_mut: ALG_ID,
    dwLength dwLength_mut: DWORD,
}}
STRUCT!{struct CRYPT_OID_INFO {
    cbSize: DWORD,
    oszOID: LPCSTR,
    pwszName: LPCWSTR,
    dwGroupId: DWORD,
    u: CRYPT_OID_INFO_u,
    ExtraInfo: CRYPT_DATA_BLOB,
    pwszCNGAlgid: LPCWSTR,
    pwszCNGExtraAlgid: LPCWSTR,
}}
pub type PCRYPT_OID_INFO = *mut CRYPT_OID_INFO;
pub type PCCRYPT_OID_INFO = *const CRYPT_OID_INFO;
pub const CRYPT_HASH_ALG_OID_GROUP_ID: DWORD = 1;
pub const CRYPT_ENCRYPT_ALG_OID_GROUP_ID: DWORD = 2;
pub const CRYPT_PUBKEY_ALG_OID_GROUP_ID: DWORD = 3;
pub const CRYPT_SIGN_ALG_OID_GROUP_ID: DWORD = 4;
pub const CRYPT_RDN_ATTR_OID_GROUP_ID: DWORD = 5;
pub const CRYPT_EXT_OR_ATTR_OID_GROUP_ID: DWORD = 6;
pub const CRYPT_ENHKEY_USAGE_OID_GROUP_ID: DWORD = 7;
pub const CRYPT_POLICY_OID_GROUP_ID: DWORD = 8;
pub const CRYPT_TEMPLATE_OID_GROUP_ID: DWORD = 9;
pub const CRYPT_KDF_OID_GROUP_ID: DWORD = 10;
pub const CRYPT_LAST_OID_GROUP_ID: DWORD = 10;
pub const CRYPT_FIRST_ALG_OID_GROUP_ID: DWORD = CRYPT_HASH_ALG_OID_GROUP_ID;
pub const CRYPT_LAST_ALG_OID_GROUP_ID: DWORD = CRYPT_SIGN_ALG_OID_GROUP_ID;
pub const CRYPT_OID_INHIBIT_SIGNATURE_FORMAT_FLAG: DWORD = 0x00000001;
pub const CRYPT_OID_USE_PUBKEY_PARA_FOR_PKCS7_FLAG: DWORD = 0x00000002;
pub const CRYPT_OID_NO_NULL_ALGORITHM_PARA_FLAG: DWORD = 0x00000004;
pub const CRYPT_OID_PUBKEY_SIGN_ONLY_FLAG: DWORD = 0x80000000;
pub const CRYPT_OID_PUBKEY_ENCRYPT_ONLY_FLAG: DWORD = 0x40000000;
pub const CRYPT_OID_USE_CURVE_NAME_FOR_ENCODE_FLAG: DWORD = 0x20000000;
pub const CRYPT_OID_USE_CURVE_PARAMETERS_FOR_ENCODE_FLAG: DWORD = 0x10000000;
extern "system" {
    pub fn CryptFindOIDInfo(
        dwKeyType: DWORD,
        pvKey: *mut c_void,
        dwGroupId: DWORD,
    ) -> PCCRYPT_OID_INFO;
}
pub const CRYPT_OID_INFO_OID_KEY: DWORD = 1;
pub const CRYPT_OID_INFO_NAME_KEY: DWORD = 2;
pub const CRYPT_OID_INFO_ALGID_KEY: DWORD = 3;
pub const CRYPT_OID_INFO_SIGN_KEY: DWORD = 4;
pub const CRYPT_OID_INFO_CNG_ALGID_KEY: DWORD = 5;
pub const CRYPT_OID_INFO_CNG_SIGN_KEY: DWORD = 6;
pub const CRYPT_OID_INFO_OID_KEY_FLAGS_MASK: DWORD = 0xFFFF0000;
pub const CRYPT_OID_INFO_PUBKEY_SIGN_KEY_FLAG: DWORD = 0x80000000;
pub const CRYPT_OID_INFO_PUBKEY_ENCRYPT_KEY_FLAG: DWORD = 0x40000000;
pub const CRYPT_OID_DISABLE_SEARCH_DS_FLAG: DWORD = 0x80000000;
pub const CRYPT_OID_PREFER_CNG_ALGID_FLAG: DWORD = 0x40000000;
pub const CRYPT_OID_INFO_OID_GROUP_BIT_LEN_MASK: DWORD = 0x0FFF0000;
pub const CRYPT_OID_INFO_OID_GROUP_BIT_LEN_SHIFT: DWORD = 16;
extern "system" {
    pub fn CryptRegisterOIDInfo(
        pInfo: PCCRYPT_OID_INFO,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn CryptUnregisterOIDInfo(
        pInfo: PCCRYPT_OID_INFO,
    ) -> BOOL;
}
FN!{stdcall PFN_CRYPT_ENUM_OID_INFO(
    pInfo: PCCRYPT_OID_INFO,
    pvArg: *mut c_void,
) -> BOOL}
extern "system" {
    pub fn CryptEnumOIDInfo(
        dwGroupId: DWORD,
        dwFlags: DWORD,
        pvArg: *mut c_void,
        pfnEnumOIDInfo: PFN_CRYPT_ENUM_OID_INFO,
    ) -> BOOL;
    pub fn CryptFindLocalizedName(
        pwszCryptName: LPCWSTR,
    ) -> LPCWSTR;
}
pub const CRYPT_LOCALIZED_NAME_ENCODING_TYPE: DWORD = 0;
pub const CRYPT_LOCALIZED_NAME_OID: &'static str = "LocalizedNames";
STRUCT!{struct CERT_STRONG_SIGN_SERIALIZED_INFO {
    dwFlags: DWORD,
    pwszCNGSignHashAlgids: LPWSTR,
    pwszCNGPubKeyMinBitLengths: LPWSTR,
}}
pub type PCERT_STRONG_SIGN_SERIALIZED_INFO = *mut CERT_STRONG_SIGN_SERIALIZED_INFO;
pub const CERT_STRONG_SIGN_ECDSA_ALGORITHM: &'static str = "ECDSA";
UNION!{union CERT_STRONG_SIGN_PARA_u {
    [usize; 1],
    pvInfo pvInfo_mut: *mut c_void,
    pSerializedInfo pSerializedInfo_mut: PCERT_STRONG_SIGN_SERIALIZED_INFO,
    pszOID pszOID_mut: LPSTR,
}}
STRUCT!{struct CERT_STRONG_SIGN_PARA {
    cbSize: DWORD,
    dwInfoChoice: DWORD,
    u: CERT_STRONG_SIGN_PARA_u,
}}
pub type PCERT_STRONG_SIGN_PARA = *mut CERT_STRONG_SIGN_PARA;
pub type PCCERT_STRONG_SIGN_PARA = *const CERT_STRONG_SIGN_PARA;
pub const CERT_STRONG_SIGN_SERIALIZED_INFO_CHOICE: DWORD = 1;
pub const CERT_STRONG_SIGN_OID_INFO_CHOICE: DWORD = 2;
pub const CERT_STRONG_SIGN_ENABLE_CRL_CHECK: DWORD = 0x1;
pub const CERT_STRONG_SIGN_ENABLE_OCSP_CHECK: DWORD = 0x2;
pub const szOID_CERT_STRONG_SIGN_OS_PREFIX: &'static str = "1.3.6.1.4.1.311.72.1.";
pub const szOID_CERT_STRONG_SIGN_OS_1: &'static str = "1.3.6.1.4.1.311.72.1.1";
pub const szOID_CERT_STRONG_SIGN_OS_CURRENT: &'static str = szOID_CERT_STRONG_SIGN_OS_1;
pub const szOID_CERT_STRONG_KEY_OS_PREFIX: &'static str = "1.3.6.1.4.1.311.72.2.";
pub const szOID_CERT_STRONG_KEY_OS_1: &'static str = "1.3.6.1.4.1.311.72.2.1";
pub const szOID_CERT_STRONG_KEY_OS_CURRENT: &'static str = szOID_CERT_STRONG_KEY_OS_1;
pub type HCRYPTMSG = *mut c_void;
pub const szOID_PKCS_7_DATA: &'static str = "1.2.840.113549.1.7.1";
pub const szOID_PKCS_7_SIGNED: &'static str = "1.2.840.113549.1.7.2";
pub const szOID_PKCS_7_ENVELOPED: &'static str = "1.2.840.113549.1.7.3";
pub const szOID_PKCS_7_SIGNEDANDENVELOPED: &'static str = "1.2.840.113549.1.7.4";
pub const szOID_PKCS_7_DIGESTED: &'static str = "1.2.840.113549.1.7.5";
pub const szOID_PKCS_7_ENCRYPTED: &'static str = "1.2.840.113549.1.7.6";
pub const szOID_PKCS_9_CONTENT_TYPE: &'static str = "1.2.840.113549.1.9.3";
pub const szOID_PKCS_9_MESSAGE_DIGEST: &'static str = "1.2.840.113549.1.9.4";
pub const CMSG_DATA: DWORD = 1;
pub const CMSG_SIGNED: DWORD = 2;
pub const CMSG_ENVELOPED: DWORD = 3;
pub const CMSG_SIGNED_AND_ENVELOPED: DWORD = 4;
pub const CMSG_HASHED: DWORD = 5;
pub const CMSG_ENCRYPTED: DWORD = 6;
pub const CMSG_ALL_FLAGS: DWORD = !0;
pub const CMSG_DATA_FLAG: DWORD = 1 << CMSG_DATA;
pub const CMSG_SIGNED_FLAG: DWORD = 1 << CMSG_SIGNED;
pub const CMSG_ENVELOPED_FLAG: DWORD = 1 << CMSG_ENVELOPED;
pub const CMSG_SIGNED_AND_ENVELOPED_FLAG: DWORD = 1 << CMSG_SIGNED_AND_ENVELOPED;
pub const CMSG_HASHED_FLAG: DWORD = 1 << CMSG_HASHED;
pub const CMSG_ENCRYPTED_FLAG: DWORD = 1 << CMSG_ENCRYPTED;
STRUCT!{struct CERT_ISSUER_SERIAL_NUMBER {
    Issuer: CERT_NAME_BLOB,
    SerialNumber: CRYPT_INTEGER_BLOB,
}}
pub type PCERT_ISSUER_SERIAL_NUMBER = *mut CERT_ISSUER_SERIAL_NUMBER;
UNION!{union CERT_ID_u {
    [usize; 4],
    IssuerSerialNumber IssuerSerialNumber_mut: CERT_ISSUER_SERIAL_NUMBER,
    KeyId KeyId_mut: CRYPT_HASH_BLOB,
    HashId HashId_mut: CRYPT_HASH_BLOB,
}}
STRUCT!{struct CERT_ID {
    dwIdChoice: DWORD,
    u: CERT_ID_u,
}}
pub type PCERT_ID = *mut CERT_ID;
pub const CERT_ID_ISSUER_SERIAL_NUMBER: DWORD = 1;
pub const CERT_ID_KEY_IDENTIFIER: DWORD = 2;
pub const CERT_ID_SHA1_HASH: DWORD = 3;
UNION!{union CMSG_SIGNER_ENCODE_INFO_u {
    [usize; 1],
    hCryptProv hCryptProv_mut: HCRYPTPROV,
    hNCryptKey hNCryptKey_mut: NCRYPT_KEY_HANDLE,
    hBCryptKey hBCryptKey_mut: BCRYPT_KEY_HANDLE,
}}
STRUCT!{struct CMSG_SIGNER_ENCODE_INFO {
    cbSize: DWORD,
    pCertInfo: PCERT_INFO,
    u: CMSG_SIGNER_ENCODE_INFO_u,
    dwKeySpec: DWORD,
    HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pvHashAuxInfo: *mut c_void,
    cAuthAttr: DWORD,
    rgAuthAttr: PCRYPT_ATTRIBUTE,
    cUnauthAttr: DWORD,
    rgUnauthAttr: PCRYPT_ATTRIBUTE,
    SignerId: CERT_ID,
    HashEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pvHashEncryptionAuxInfo: *mut c_void,
}}
pub type PCMSG_SIGNER_ENCODE_INFO = *mut CMSG_SIGNER_ENCODE_INFO;
STRUCT!{struct CMSG_SIGNED_ENCODE_INFO {
    cbSize: DWORD,
    cSigners: DWORD,
    rgSigners: PCMSG_SIGNER_ENCODE_INFO,
    cCertEncoded: DWORD,
    rgCertEncoded: PCERT_BLOB,
    cCrlEncoded: DWORD,
    rgCrlEncoded: PCRL_BLOB,
    cAttrCertEncoded: DWORD,
    rgAttrCertEncoded: PCERT_BLOB,
}}
pub type PCMSG_SIGNED_ENCODE_INFO = *mut CMSG_SIGNED_ENCODE_INFO;
pub type PCMSG_RECIPIENT_ENCODE_INFO = *mut CMSG_RECIPIENT_ENCODE_INFO;
STRUCT!{struct CMSG_ENVELOPED_ENCODE_INFO {
    cbSize: DWORD,
    hCryptProv: HCRYPTPROV_LEGACY,
    ContentEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pvEncryptionAuxInfo: *mut c_void,
    cRecipients: DWORD,
    rgpRecipients: *mut PCERT_INFO,
    rgCmsRecipients: PCMSG_RECIPIENT_ENCODE_INFO,
    cCertEncoded: DWORD,
    rgCertEncoded: PCERT_BLOB,
    cCrlEncoded: DWORD,
    rgCrlEncoded: PCRL_BLOB,
    cAttrCertEncoded: DWORD,
    rgAttrCertEncoded: PCERT_BLOB,
    cUnprotectedAttr: DWORD,
    rgUnprotectedAttr: PCRYPT_ATTRIBUTE,
}}
pub type PCMSG_ENVELOPED_ENCODE_INFO = *mut CMSG_ENVELOPED_ENCODE_INFO;
STRUCT!{struct CMSG_KEY_TRANS_RECIPIENT_ENCODE_INFO {
    cbSize: DWORD,
    KeyEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pvKeyEncryptionAuxInfo: *mut c_void,
    hCryptProv: HCRYPTPROV_LEGACY,
    RecipientPublicKey: CRYPT_BIT_BLOB,
    RecipientId: CERT_ID,
}}
pub type PCMSG_KEY_TRANS_RECIPIENT_ENCODE_INFO = *mut CMSG_KEY_TRANS_RECIPIENT_ENCODE_INFO;
STRUCT!{struct CMSG_RECIPIENT_ENCRYPTED_KEY_ENCODE_INFO {
    cbSize: DWORD,
    RecipientPublicKey: CRYPT_BIT_BLOB,
    RecipientId: CERT_ID,
    Date: FILETIME,
    pOtherAttr: PCRYPT_ATTRIBUTE_TYPE_VALUE,
}}
pub type PCMSG_RECIPIENT_ENCRYPTED_KEY_ENCODE_INFO = *mut CMSG_RECIPIENT_ENCRYPTED_KEY_ENCODE_INFO;
UNION!{union CMSG_KEY_AGREE_RECIPIENT_ENCODE_INFO_u {
    [usize; 1],
    pEphemeralAlgorithm pEphemeralAlgorithm_mut: PCRYPT_ALGORITHM_IDENTIFIER,
    pSenderId pSenderId_mut: PCERT_ID,
}}
STRUCT!{struct CMSG_KEY_AGREE_RECIPIENT_ENCODE_INFO {
    cbSize: DWORD,
    KeyEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pvKeyEncryptionAuxInfo: *mut c_void,
    KeyWrapAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pvKeyWrapAuxInfo: *mut c_void,
    hCryptProv: HCRYPTPROV_LEGACY,
    dwKeySpec: DWORD,
    dwKeyChoice: DWORD,
    u: CMSG_KEY_AGREE_RECIPIENT_ENCODE_INFO_u,
    UserKeyingMaterial: CRYPT_DATA_BLOB,
    cRecipientEncryptedKeys: DWORD,
    rgpRecipientEncryptedKeys: *mut PCMSG_RECIPIENT_ENCRYPTED_KEY_ENCODE_INFO,
}}
pub type PCMSG_KEY_AGREE_RECIPIENT_ENCODE_INFO = *mut CMSG_KEY_AGREE_RECIPIENT_ENCODE_INFO;
pub const CMSG_KEY_AGREE_EPHEMERAL_KEY_CHOICE: DWORD = 1;
pub const CMSG_KEY_AGREE_STATIC_KEY_CHOICE: DWORD = 2;
UNION!{union CMSG_MAIL_LIST_RECIPIENT_ENCODE_INFO_u {
    [usize; 1],
    hKeyEncryptionKey hKeyEncryptionKey_mut: HCRYPTKEY,
    pvKeyEncryptionKey pvKeyEncryptionKey_mut: *mut c_void,
}}
STRUCT!{struct CMSG_MAIL_LIST_RECIPIENT_ENCODE_INFO {
    cbSize: DWORD,
    KeyEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pvKeyEncryptionAuxInfo: *mut c_void,
    hCryptProv: HCRYPTPROV,
    dwKeyChoice: DWORD,
    u: CMSG_MAIL_LIST_RECIPIENT_ENCODE_INFO_u,
    KeyId: CRYPT_DATA_BLOB,
    Date: FILETIME,
    pOtherAttr: PCRYPT_ATTRIBUTE_TYPE_VALUE,
}}
pub type PCMSG_MAIL_LIST_RECIPIENT_ENCODE_INFO = *mut CMSG_MAIL_LIST_RECIPIENT_ENCODE_INFO;
pub const CMSG_MAIL_LIST_HANDLE_KEY_CHOICE: DWORD = 1;
UNION!{union CMSG_RECIPIENT_ENCODE_INFO_u {
    [usize; 1],
    pKeyTrans pKeyTrans_mut: PCMSG_KEY_TRANS_RECIPIENT_ENCODE_INFO,
    pKeyAgree pKeyAgree_mut: PCMSG_KEY_AGREE_RECIPIENT_ENCODE_INFO,
    pMailList pMailList_mut: PCMSG_MAIL_LIST_RECIPIENT_ENCODE_INFO,
}}
STRUCT!{struct CMSG_RECIPIENT_ENCODE_INFO {
    dwRecipientChoice: DWORD,
    u: CMSG_RECIPIENT_ENCODE_INFO_u,
}}
pub const CMSG_KEY_TRANS_RECIPIENT: DWORD = 1;
pub const CMSG_KEY_AGREE_RECIPIENT: DWORD = 2;
pub const CMSG_MAIL_LIST_RECIPIENT: DWORD = 3;
STRUCT!{struct CMSG_RC2_AUX_INFO {
    cbSize: DWORD,
    dwBitLen: DWORD,
}}
pub type PCMSG_RC2_AUX_INFO = *mut CMSG_RC2_AUX_INFO;
STRUCT!{struct CMSG_SP3_COMPATIBLE_AUX_INFO {
    cbSize: DWORD,
    dwFlags: DWORD,
}}
pub type PCMSG_SP3_COMPATIBLE_AUX_INFO = *mut CMSG_SP3_COMPATIBLE_AUX_INFO;
pub const CMSG_SP3_COMPATIBLE_ENCRYPT_FLAG: DWORD = 0x80000000;
STRUCT!{struct CMSG_RC4_AUX_INFO {
    cbSize: DWORD,
    dwBitLen: DWORD,
}}
pub type PCMSG_RC4_AUX_INFO = *mut CMSG_RC4_AUX_INFO;
pub const CMSG_RC4_NO_SALT_FLAG: DWORD = 0x40000000;
STRUCT!{struct CMSG_SIGNED_AND_ENVELOPED_ENCODE_INFO {
    cbSize: DWORD,
    SignedInfo: CMSG_SIGNED_ENCODE_INFO,
    EnvelopedInfo: CMSG_ENVELOPED_ENCODE_INFO,
}}
pub type PCMSG_SIGNED_AND_ENVELOPED_ENCODE_INFO = *mut CMSG_SIGNED_AND_ENVELOPED_ENCODE_INFO;
STRUCT!{struct CMSG_HASHED_ENCODE_INFO {
    cbSize: DWORD,
    hCryptProv: HCRYPTPROV_LEGACY,
    HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pvHashAuxInfo: *mut c_void,
}}
pub type PCMSG_HASHED_ENCODE_INFO = *mut CMSG_HASHED_ENCODE_INFO;
STRUCT!{struct CMSG_ENCRYPTED_ENCODE_INFO {
    cbSize: DWORD,
    ContentEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pvEncryptionAuxInfo: *mut c_void,
}}
pub type PCMSG_ENCRYPTED_ENCODE_INFO = *mut CMSG_ENCRYPTED_ENCODE_INFO;
FN!{stdcall PFN_CMSG_STREAM_OUTPUT(
    pvArg: *const c_void,
    pbData: *mut BYTE,
    cbData: DWORD,
    fFinal: BOOL,
) -> BOOL}
pub const CMSG_INDEFINITE_LENGTH: DWORD = 0xFFFFFFFF;
STRUCT!{struct CMSG_STREAM_INFO {
    cbContent: DWORD,
    pfnStreamOutput: PFN_CMSG_STREAM_OUTPUT,
    pvArg: *mut c_void,
}}
pub type PCMSG_STREAM_INFO = *mut CMSG_STREAM_INFO;
pub const CMSG_BARE_CONTENT_FLAG: DWORD = 0x00000001;
pub const CMSG_LENGTH_ONLY_FLAG: DWORD = 0x00000002;
pub const CMSG_DETACHED_FLAG: DWORD = 0x00000004;
pub const CMSG_AUTHENTICATED_ATTRIBUTES_FLAG: DWORD = 0x00000008;
pub const CMSG_CONTENTS_OCTETS_FLAG: DWORD = 0x00000010;
pub const CMSG_MAX_LENGTH_FLAG: DWORD = 0x00000020;
pub const CMSG_CMS_ENCAPSULATED_CONTENT_FLAG: DWORD = 0x00000040;
pub const CMSG_SIGNED_DATA_NO_SIGN_FLAG: DWORD = 0x00000080;
pub const CMSG_CRYPT_RELEASE_CONTEXT_FLAG: DWORD = 0x00008000;
extern "system" {
    pub fn CryptMsgOpenToEncode(
        dwMsgEncodingType: DWORD,
        dwFlags: DWORD,
        dwMsgType: DWORD,
        pvMsgEncodeInfo: *mut c_void,
        pszInnerContentObjID: LPSTR,
        pStreamInfo: PCMSG_STREAM_INFO,
    ) -> HCRYPTMSG;
    pub fn CryptMsgCalculateEncodedLength(
        dwMsgEncodingType: DWORD,
        dwFlags: DWORD,
        dwMsgType: DWORD,
        pvMsgEncodeInfo: *const c_void,
        pszInnerContentObjID: LPSTR,
        cbData: DWORD,
    ) -> DWORD;
    pub fn CryptMsgOpenToDecode(
        dwMsgEncodingType: DWORD,
        dwFlags: DWORD,
        dwMsgType: DWORD,
        hCryptProv: HCRYPTPROV_LEGACY,
        pRecipientInfo: PCERT_INFO,
        pStreamInfo: PCMSG_STREAM_INFO,
    ) -> HCRYPTMSG;
    pub fn CryptMsgDuplicate(
        hCryptMsg: HCRYPTMSG,
    ) -> HCRYPTMSG;
    pub fn CryptMsgClose(
        hCryptMsg: HCRYPTMSG,
    ) -> BOOL;
    pub fn CryptMsgUpdate(
        hCryptMsg: HCRYPTMSG,
        pbData: *const BYTE,
        cbData: DWORD,
        fFinal: BOOL,
    ) -> BOOL;
    pub fn CryptMsgGetParam(
        hCryptMsg: HCRYPTMSG,
        dwParamType: DWORD,
        dwIndex: DWORD,
        pvData: *mut c_void,
        pcbData: *mut DWORD,
    ) -> BOOL;
}
pub const CMSG_TYPE_PARAM: DWORD = 1;
pub const CMSG_CONTENT_PARAM: DWORD = 2;
pub const CMSG_BARE_CONTENT_PARAM: DWORD = 3;
pub const CMSG_INNER_CONTENT_TYPE_PARAM: DWORD = 4;
pub const CMSG_SIGNER_COUNT_PARAM: DWORD = 5;
pub const CMSG_SIGNER_INFO_PARAM: DWORD = 6;
pub const CMSG_SIGNER_CERT_INFO_PARAM: DWORD = 7;
pub const CMSG_SIGNER_HASH_ALGORITHM_PARAM: DWORD = 8;
pub const CMSG_SIGNER_AUTH_ATTR_PARAM: DWORD = 9;
pub const CMSG_SIGNER_UNAUTH_ATTR_PARAM: DWORD = 10;
pub const CMSG_CERT_COUNT_PARAM: DWORD = 11;
pub const CMSG_CERT_PARAM: DWORD = 12;
pub const CMSG_CRL_COUNT_PARAM: DWORD = 13;
pub const CMSG_CRL_PARAM: DWORD = 14;
pub const CMSG_ENVELOPE_ALGORITHM_PARAM: DWORD = 15;
pub const CMSG_RECIPIENT_COUNT_PARAM: DWORD = 17;
pub const CMSG_RECIPIENT_INDEX_PARAM: DWORD = 18;
pub const CMSG_RECIPIENT_INFO_PARAM: DWORD = 19;
pub const CMSG_HASH_ALGORITHM_PARAM: DWORD = 20;
pub const CMSG_HASH_DATA_PARAM: DWORD = 21;
pub const CMSG_COMPUTED_HASH_PARAM: DWORD = 22;
pub const CMSG_ENCRYPT_PARAM: DWORD = 26;
pub const CMSG_ENCRYPTED_DIGEST: DWORD = 27;
pub const CMSG_ENCODED_SIGNER: DWORD = 28;
pub const CMSG_ENCODED_MESSAGE: DWORD = 29;
pub const CMSG_VERSION_PARAM: DWORD = 30;
pub const CMSG_ATTR_CERT_COUNT_PARAM: DWORD = 31;
pub const CMSG_ATTR_CERT_PARAM: DWORD = 32;
pub const CMSG_CMS_RECIPIENT_COUNT_PARAM: DWORD = 33;
pub const CMSG_CMS_RECIPIENT_INDEX_PARAM: DWORD = 34;
pub const CMSG_CMS_RECIPIENT_ENCRYPTED_KEY_INDEX_PARAM: DWORD = 35;
pub const CMSG_CMS_RECIPIENT_INFO_PARAM: DWORD = 36;
pub const CMSG_UNPROTECTED_ATTR_PARAM: DWORD = 37;
pub const CMSG_SIGNER_CERT_ID_PARAM: DWORD = 38;
pub const CMSG_CMS_SIGNER_INFO_PARAM: DWORD = 39;
STRUCT!{struct CMSG_SIGNER_INFO {
    dwVersion: DWORD,
    Issuer: CERT_NAME_BLOB,
    SerialNumber: CRYPT_INTEGER_BLOB,
    HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    HashEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    EncryptedHash: CRYPT_DATA_BLOB,
    AuthAttrs: CRYPT_ATTRIBUTES,
    UnauthAttrs: CRYPT_ATTRIBUTES,
}}
pub type PCMSG_SIGNER_INFO = *mut CMSG_SIGNER_INFO;
STRUCT!{struct CMSG_CMS_SIGNER_INFO {
    dwVersion: DWORD,
    SignerId: CERT_ID,
    HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    HashEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    EncryptedHash: CRYPT_DATA_BLOB,
    AuthAttrs: CRYPT_ATTRIBUTES,
    UnauthAttrs: CRYPT_ATTRIBUTES,
}}
pub type PCMSG_CMS_SIGNER_INFO = *mut CMSG_CMS_SIGNER_INFO;
pub type CMSG_ATTR = CRYPT_ATTRIBUTES;
pub type PCMSG_ATTR = *mut CRYPT_ATTRIBUTES;
pub const CMSG_SIGNED_DATA_V1: DWORD = 1;
pub const CMSG_SIGNED_DATA_V3: DWORD = 3;
pub const CMSG_SIGNED_DATA_PKCS_1_5_VERSION: DWORD = CMSG_SIGNED_DATA_V1;
pub const CMSG_SIGNED_DATA_CMS_VERSION: DWORD = CMSG_SIGNED_DATA_V3;
pub const CMSG_SIGNER_INFO_V1: DWORD = 1;
pub const CMSG_SIGNER_INFO_V3: DWORD = 3;
pub const CMSG_SIGNER_INFO_PKCS_1_5_VERSION: DWORD = CMSG_SIGNER_INFO_V1;
pub const CMSG_SIGNER_INFO_CMS_VERSION: DWORD = CMSG_SIGNER_INFO_V3;
pub const CMSG_HASHED_DATA_V0: DWORD = 0;
pub const CMSG_HASHED_DATA_V2: DWORD = 2;
pub const CMSG_HASHED_DATA_PKCS_1_5_VERSION: DWORD = CMSG_HASHED_DATA_V0;
pub const CMSG_HASHED_DATA_CMS_VERSION: DWORD = CMSG_HASHED_DATA_V2;
pub const CMSG_ENVELOPED_DATA_V0: DWORD = 0;
pub const CMSG_ENVELOPED_DATA_V2: DWORD = 2;
pub const CMSG_ENVELOPED_DATA_PKCS_1_5_VERSION: DWORD = CMSG_ENVELOPED_DATA_V0;
pub const CMSG_ENVELOPED_DATA_CMS_VERSION: DWORD = CMSG_ENVELOPED_DATA_V2;
STRUCT!{struct CMSG_KEY_TRANS_RECIPIENT_INFO {
    dwVersion: DWORD,
    RecipientId: CERT_ID,
    KeyEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    EncryptedKey: CRYPT_DATA_BLOB,
}}
pub type PCMSG_KEY_TRANS_RECIPIENT_INFO = *mut CMSG_KEY_TRANS_RECIPIENT_INFO;
STRUCT!{struct CMSG_RECIPIENT_ENCRYPTED_KEY_INFO {
    RecipientId: CERT_ID,
    EncryptedKey: CRYPT_DATA_BLOB,
    Date: FILETIME,
    pOtherAttr: PCRYPT_ATTRIBUTE_TYPE_VALUE,
}}
pub type PCMSG_RECIPIENT_ENCRYPTED_KEY_INFO = *mut CMSG_RECIPIENT_ENCRYPTED_KEY_INFO;
UNION!{union CMSG_KEY_AGREE_RECIPIENT_INFO_u {
    [usize; 6],
    OriginatorCertId OriginatorCertId_mut: CERT_ID,
    OriginatorPublicKeyInfo OriginatorPublicKeyInfo_mut: CERT_PUBLIC_KEY_INFO,
}}
STRUCT!{struct CMSG_KEY_AGREE_RECIPIENT_INFO {
    dwVersion: DWORD,
    dwOriginatorChoice: DWORD,
    u: CMSG_KEY_AGREE_RECIPIENT_INFO_u,
    UserKeyingMaterial: CRYPT_DATA_BLOB,
    KeyEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    cRecipientEncryptedKeys: DWORD,
    rgpRecipientEncryptedKeys: *mut PCMSG_RECIPIENT_ENCRYPTED_KEY_INFO,
}}
pub type PCMSG_KEY_AGREE_RECIPIENT_INFO = *mut CMSG_KEY_AGREE_RECIPIENT_INFO;
pub const CMSG_KEY_AGREE_ORIGINATOR_CERT: DWORD = 1;
pub const CMSG_KEY_AGREE_ORIGINATOR_PUBLIC_KEY: DWORD = 2;
STRUCT!{struct CMSG_MAIL_LIST_RECIPIENT_INFO {
    dwVersion: DWORD,
    KeyId: CRYPT_DATA_BLOB,
    KeyEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    EncryptedKey: CRYPT_DATA_BLOB,
    Date: FILETIME,
    pOtherAttr: PCRYPT_ATTRIBUTE_TYPE_VALUE,
}}
pub type PCMSG_MAIL_LIST_RECIPIENT_INFO = *mut CMSG_MAIL_LIST_RECIPIENT_INFO;
UNION!{union CMSG_CMS_RECIPIENT_INFO_u {
    [usize; 1],
    pKeyTrans pKeyTrans_mut: PCMSG_KEY_TRANS_RECIPIENT_INFO,
    pKeyAgree pKeyAgree_mut: PCMSG_KEY_AGREE_RECIPIENT_INFO,
    pMailList pMailList_mut: PCMSG_MAIL_LIST_RECIPIENT_INFO,
}}
STRUCT!{struct CMSG_CMS_RECIPIENT_INFO {
    dwRecipientChoice: DWORD,
    u: CMSG_CMS_RECIPIENT_INFO_u,
}}
pub type PCMSG_CMS_RECIPIENT_INFO = *mut CMSG_CMS_RECIPIENT_INFO;
pub const CMSG_ENVELOPED_RECIPIENT_V0: DWORD = 0;
pub const CMSG_ENVELOPED_RECIPIENT_V2: DWORD = 2;
pub const CMSG_ENVELOPED_RECIPIENT_V3: DWORD = 3;
pub const CMSG_ENVELOPED_RECIPIENT_V4: DWORD = 4;
pub const CMSG_KEY_TRANS_PKCS_1_5_VERSION: DWORD = CMSG_ENVELOPED_RECIPIENT_V0;
pub const CMSG_KEY_TRANS_CMS_VERSION: DWORD = CMSG_ENVELOPED_RECIPIENT_V2;
pub const CMSG_KEY_AGREE_VERSION: DWORD = CMSG_ENVELOPED_RECIPIENT_V3;
pub const CMSG_MAIL_LIST_VERSION: DWORD = CMSG_ENVELOPED_RECIPIENT_V4;
extern "system" {
    pub fn CryptMsgControl(
        hCryptMsg: HCRYPTMSG,
        dwFlags: DWORD,
        dwCtrlType: DWORD,
        pvCtrlPara: *const c_void,
    ) -> BOOL;
}
pub const CMSG_CTRL_VERIFY_SIGNATURE: DWORD = 1;
pub const CMSG_CTRL_DECRYPT: DWORD = 2;
pub const CMSG_CTRL_VERIFY_HASH: DWORD = 5;
pub const CMSG_CTRL_ADD_SIGNER: DWORD = 6;
pub const CMSG_CTRL_DEL_SIGNER: DWORD = 7;
pub const CMSG_CTRL_ADD_SIGNER_UNAUTH_ATTR: DWORD = 8;
pub const CMSG_CTRL_DEL_SIGNER_UNAUTH_ATTR: DWORD = 9;
pub const CMSG_CTRL_ADD_CERT: DWORD = 10;
pub const CMSG_CTRL_DEL_CERT: DWORD = 11;
pub const CMSG_CTRL_ADD_CRL: DWORD = 12;
pub const CMSG_CTRL_DEL_CRL: DWORD = 13;
pub const CMSG_CTRL_ADD_ATTR_CERT: DWORD = 14;
pub const CMSG_CTRL_DEL_ATTR_CERT: DWORD = 15;
pub const CMSG_CTRL_KEY_TRANS_DECRYPT: DWORD = 16;
pub const CMSG_CTRL_KEY_AGREE_DECRYPT: DWORD = 17;
pub const CMSG_CTRL_MAIL_LIST_DECRYPT: DWORD = 18;
pub const CMSG_CTRL_VERIFY_SIGNATURE_EX: DWORD = 19;
pub const CMSG_CTRL_ADD_CMS_SIGNER_INFO: DWORD = 20;
pub const CMSG_CTRL_ENABLE_STRONG_SIGNATURE: DWORD = 21;
STRUCT!{struct CMSG_CTRL_VERIFY_SIGNATURE_EX_PARA {
    cbSize: DWORD,
    hCryptProv: HCRYPTPROV_LEGACY,
    dwSignerIndex: DWORD,
    dwSignerType: DWORD,
    pvSigner: *mut c_void,
}}
pub type PCMSG_CTRL_VERIFY_SIGNATURE_EX_PARA = *mut CMSG_CTRL_VERIFY_SIGNATURE_EX_PARA;
pub const CMSG_VERIFY_SIGNER_PUBKEY: DWORD = 1;
pub const CMSG_VERIFY_SIGNER_CERT: DWORD = 2;
pub const CMSG_VERIFY_SIGNER_CHAIN: DWORD = 3;
pub const CMSG_VERIFY_SIGNER_NULL: DWORD = 4;
UNION!{union CMSG_CTRL_DECRYPT_PARA_u {
    [usize; 1],
    hCryptProv hCryptProv_mut: HCRYPTPROV,
    hNCryptKey hNCryptKey_mut: NCRYPT_KEY_HANDLE,
}}
STRUCT!{struct CMSG_CTRL_DECRYPT_PARA {
    cbSize: DWORD,
    u: CMSG_CTRL_DECRYPT_PARA_u,
    dwKeySpec: DWORD,
    dwRecipientIndex: DWORD,
}}
pub type PCMSG_CTRL_DECRYPT_PARA = *mut CMSG_CTRL_DECRYPT_PARA;
UNION!{union CMSG_CTRL_KEY_TRANS_DECRYPT_PARA_u {
    [usize; 1],
    hCryptProv hCryptProv_mut: HCRYPTPROV,
    hNCryptKey hNCryptKey_mut: NCRYPT_KEY_HANDLE,
}}
STRUCT!{struct CMSG_CTRL_KEY_TRANS_DECRYPT_PARA {
    cbSize: DWORD,
    u: CMSG_CTRL_KEY_TRANS_DECRYPT_PARA_u,
    dwKeySpec: DWORD,
    pKeyTrans: PCMSG_KEY_TRANS_RECIPIENT_INFO,
    dwRecipientIndex: DWORD,
}}
pub type PCMSG_CTRL_KEY_TRANS_DECRYPT_PARA = *mut CMSG_CTRL_KEY_TRANS_DECRYPT_PARA;
UNION!{union CMSG_CTRL_KEY_AGREE_DECRYPT_PARA_u {
    [usize; 1],
    hCryptProv hCryptProv_mut: HCRYPTPROV,
    hNCryptKey hNCryptKey_mut: NCRYPT_KEY_HANDLE,
}}
STRUCT!{struct CMSG_CTRL_KEY_AGREE_DECRYPT_PARA {
    cbSize: DWORD,
    u: CMSG_CTRL_KEY_AGREE_DECRYPT_PARA_u,
    dwKeySpec: DWORD,
    pKeyAgree: PCMSG_KEY_AGREE_RECIPIENT_INFO,
    dwRecipientIndex: DWORD,
    dwRecipientEncryptedKeyIndex: DWORD,
    OriginatorPublicKey: CRYPT_BIT_BLOB,
}}
pub type PCMSG_CTRL_KEY_AGREE_DECRYPT_PARA = *mut CMSG_CTRL_KEY_AGREE_DECRYPT_PARA;
UNION!{union CMSG_CTRL_MAIL_LIST_DECRYPT_PARA_u {
    [usize; 1],
    hKeyEncryptionKey hKeyEncryptionKey_mut: HCRYPTKEY,
    pvKeyEncryptionKey pvKeyEncryptionKey_mut: *mut c_void,
}}
STRUCT!{struct CMSG_CTRL_MAIL_LIST_DECRYPT_PARA {
    cbSize: DWORD,
    hCryptProv: HCRYPTPROV,
    pMailList: PCMSG_MAIL_LIST_RECIPIENT_INFO,
    dwRecipientIndex: DWORD,
    dwKeyChoice: DWORD,
    u: CMSG_CTRL_MAIL_LIST_DECRYPT_PARA_u,
}}
pub type PCMSG_CTRL_MAIL_LIST_DECRYPT_PARA = *mut CMSG_CTRL_MAIL_LIST_DECRYPT_PARA;
STRUCT!{struct CMSG_CTRL_ADD_SIGNER_UNAUTH_ATTR_PARA {
    cbSize: DWORD,
    dwSignerIndex: DWORD,
    blob: CRYPT_DATA_BLOB,
}}
pub type PCMSG_CTRL_ADD_SIGNER_UNAUTH_ATTR_PARA = *mut CMSG_CTRL_ADD_SIGNER_UNAUTH_ATTR_PARA;
STRUCT!{struct CMSG_CTRL_DEL_SIGNER_UNAUTH_ATTR_PARA {
    cbSize: DWORD,
    dwSignerIndex: DWORD,
    dwUnauthAttrIndex: DWORD,
}}
pub type PCMSG_CTRL_DEL_SIGNER_UNAUTH_ATTR_PARA = *mut CMSG_CTRL_DEL_SIGNER_UNAUTH_ATTR_PARA;
extern "system" {
    pub fn CryptMsgVerifyCountersignatureEncoded(
        hCryptProv: HCRYPTPROV_LEGACY,
        dwEncodingType: DWORD,
        pbSignerInfo: PBYTE,
        cbSignerInfo: DWORD,
        pbSignerInfoCountersignature: PBYTE,
        cbSignerInfoCountersignature: DWORD,
        pciCountersigner: PCERT_INFO,
    ) -> BOOL;
    pub fn CryptMsgVerifyCountersignatureEncodedEx(
        hCryptProv: HCRYPTPROV_LEGACY,
        dwEncodingType: DWORD,
        pbSignerInfo: PBYTE,
        cbSignerInfo: DWORD,
        pbSignerInfoCountersignature: PBYTE,
        cbSignerInfoCountersignature: DWORD,
        dwSignerType: DWORD,
        pvSigner: *mut c_void,
        dwFlags: DWORD,
        pvExtra: *mut c_void,
    ) -> BOOL;
}
pub const CMSG_VERIFY_COUNTER_SIGN_ENABLE_STRONG_FLAG: DWORD = 0x00000001;
extern "system" {
    pub fn CryptMsgCountersign(
        hCryptMsg: HCRYPTMSG,
        dwIndex: DWORD,
        cCountersigners: DWORD,
        rgCountersigners: PCMSG_SIGNER_ENCODE_INFO,
    ) -> BOOL;
    pub fn CryptMsgCountersignEncoded(
        dwEncodingType: DWORD,
        pbSignerInfo: PBYTE,
        cbSignerInfo: DWORD,
        cCountersigners: DWORD,
        rgCountersigners: PCMSG_SIGNER_ENCODE_INFO,
        pbCountersignature: PBYTE,
        pcbCountersignature: PDWORD,
    ) -> BOOL;
}
FN!{stdcall PFN_CMSG_ALLOC(
    cb: size_t,
) -> ()}
FN!{stdcall PFN_CMSG_FREE(
    pv: *mut c_void,
) -> ()}
pub const CMSG_OID_GEN_ENCRYPT_KEY_FUNC: &'static str = "CryptMsgDllGenEncryptKey";
FN!{stdcall PFN_CMSG_GEN_ENCRYPT_KEY(
    phCryptProv: *mut HCRYPTPROV,
    paiEncrypt: PCRYPT_ALGORITHM_IDENTIFIER,
    pvEncryptAuxInfo: PVOID,
    pPublicKeyInfo: PCERT_PUBLIC_KEY_INFO,
    pfnAlloc: PFN_CMSG_ALLOC,
    phEncryptKey: *mut HCRYPTKEY,
    ppbEncryptParameters: *mut PBYTE,
    pcbEncryptParameters: PDWORD,
) -> BOOL}
pub const CMSG_OID_EXPORT_ENCRYPT_KEY_FUNC: &'static str = "CryptMsgDllExportEncryptKey";
FN!{stdcall PFN_CMSG_EXPORT_ENCRYPT_KEY(
    hCryptProv: HCRYPTPROV,
    hEncryptKey: HCRYPTKEY,
    pPublicKeyInfo: PCERT_PUBLIC_KEY_INFO,
    pbData: PBYTE,
    pcbData: PDWORD,
) -> BOOL}
pub const CMSG_OID_IMPORT_ENCRYPT_KEY_FUNC: &'static str = "CryptMsgDllImportEncryptKey";
FN!{stdcall PFN_CMSG_IMPORT_ENCRYPT_KEY(
    hCryptProv: HCRYPTPROV,
    dwKeySpec: DWORD,
    paiEncrypt: PCRYPT_ALGORITHM_IDENTIFIER,
    paiPubKey: PCRYPT_ALGORITHM_IDENTIFIER,
    pbEncodedKey: PBYTE,
    cbEncodedKey: DWORD,
    phEncryptKey: *mut HCRYPTKEY,
) -> BOOL}
pub const CMSG_DEFAULT_INSTALLABLE_FUNC_OID: LPCSTR = 1 as LPCSTR;
UNION!{union CMSG_CONTENT_ENCRYPT_INFO_u {
    [usize; 1],
    hContentEncryptKey hContentEncryptKey_mut: HCRYPTKEY,
    hCNGContentEncryptKey hCNGContentEncryptKey_mut: BCRYPT_KEY_HANDLE,
}}
STRUCT!{struct CMSG_CONTENT_ENCRYPT_INFO {
    cbSize: DWORD,
    hCryptProv: HCRYPTPROV_LEGACY,
    ContentEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pvEncryptionAuxInfo: *mut c_void,
    cRecipients: DWORD,
    rgCmsRecipients: PCMSG_RECIPIENT_ENCODE_INFO,
    pfnAlloc: PFN_CMSG_ALLOC,
    pfnFree: PFN_CMSG_FREE,
    dwEncryptFlags: DWORD,
    u: CMSG_CONTENT_ENCRYPT_INFO_u,
    dwFlags: DWORD,
    fCNG: BOOL,
    pbCNGContentEncryptKeyObject: *mut BYTE,
    pbContentEncryptKey: *mut BYTE,
    cbContentEncryptKey: DWORD,
}}
pub type PCMSG_CONTENT_ENCRYPT_INFO = *mut CMSG_CONTENT_ENCRYPT_INFO;
pub const CMSG_CONTENT_ENCRYPT_PAD_ENCODED_LEN_FLAG: DWORD = 0x00000001;
pub const CMSG_CONTENT_ENCRYPT_FREE_PARA_FLAG: DWORD = 0x00000001;
pub const CMSG_CONTENT_ENCRYPT_FREE_OBJID_FLAG: DWORD = 0x00000002;
pub const CMSG_CONTENT_ENCRYPT_RELEASE_CONTEXT_FLAG: DWORD = 0x00008000;
pub const CMSG_OID_GEN_CONTENT_ENCRYPT_KEY_FUNC: &'static str = "CryptMsgDllGenContentEncryptKey";
pub const CMSG_OID_CAPI1_GEN_CONTENT_ENCRYPT_KEY_FUNC: &'static str
    = CMSG_OID_GEN_CONTENT_ENCRYPT_KEY_FUNC;
FN!{stdcall PFN_CMSG_GEN_CONTENT_ENCRYPT_KEY(
    pContentEncryptInfo: PCMSG_CONTENT_ENCRYPT_INFO,
    dwFlags: DWORD,
    pvReserved: *mut c_void,
) -> BOOL}
pub const CMSG_OID_CNG_GEN_CONTENT_ENCRYPT_KEY_FUNC: &'static str
    = "CryptMsgDllCNGGenContentEncryptKey";
STRUCT!{struct CMSG_KEY_TRANS_ENCRYPT_INFO {
    cbSize: DWORD,
    dwRecipientIndex: DWORD,
    KeyEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    EncryptedKey: CRYPT_DATA_BLOB,
    dwFlags: DWORD,
}}
pub type PCMSG_KEY_TRANS_ENCRYPT_INFO = *mut CMSG_KEY_TRANS_ENCRYPT_INFO;
pub const CMSG_KEY_TRANS_ENCRYPT_FREE_PARA_FLAG: DWORD = 0x00000001;
pub const CMSG_KEY_TRANS_ENCRYPT_FREE_OBJID_FLAG: DWORD = 0x00000002;
pub const CMSG_OID_EXPORT_KEY_TRANS_FUNC: &'static str = "CryptMsgDllExportKeyTrans";
pub const CMSG_OID_CAPI1_EXPORT_KEY_TRANS_FUNC: &'static str = CMSG_OID_EXPORT_KEY_TRANS_FUNC;
FN!{stdcall PFN_CMSG_EXPORT_KEY_TRANS(
    pContentEncryptInfo: PCMSG_CONTENT_ENCRYPT_INFO,
    pKeyTransEncodeInfo: PCMSG_KEY_TRANS_RECIPIENT_ENCODE_INFO,
    pKeyTransEncryptInfo: PCMSG_KEY_TRANS_ENCRYPT_INFO,
    dwFlags: DWORD,
    pvReserved: *mut c_void,
) -> BOOL}
pub const CMSG_OID_CNG_EXPORT_KEY_TRANS_FUNC: &'static str = "CryptMsgDllCNGExportKeyTrans";
STRUCT!{struct CMSG_KEY_AGREE_KEY_ENCRYPT_INFO {
    cbSize: DWORD,
    EncryptedKey: CRYPT_DATA_BLOB,
}}
pub type PCMSG_KEY_AGREE_KEY_ENCRYPT_INFO = *mut CMSG_KEY_AGREE_KEY_ENCRYPT_INFO;
UNION!{union CMSG_KEY_AGREE_ENCRYPT_INFO_u {
    [usize; 6],
    OriginatorCertId OriginatorCertId_mut: CERT_ID,
    OriginatorPublicKeyInfo OriginatorPublicKeyInfo_mut: CERT_PUBLIC_KEY_INFO,
}}
STRUCT!{struct CMSG_KEY_AGREE_ENCRYPT_INFO {
    cbSize: DWORD,
    dwRecipientIndex: DWORD,
    KeyEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    UserKeyingMaterial: CRYPT_DATA_BLOB,
    dwOriginatorChoice: DWORD,
    u: CMSG_KEY_AGREE_ENCRYPT_INFO_u,
    cKeyAgreeKeyEncryptInfo: DWORD,
    rgpKeyAgreeKeyEncryptInfo: *mut PCMSG_KEY_AGREE_KEY_ENCRYPT_INFO,
    dwFlags: DWORD,
}}
pub type PCMSG_KEY_AGREE_ENCRYPT_INFO = *mut CMSG_KEY_AGREE_ENCRYPT_INFO;
pub const CMSG_KEY_AGREE_ENCRYPT_FREE_PARA_FLAG: DWORD = 0x00000001;
pub const CMSG_KEY_AGREE_ENCRYPT_FREE_MATERIAL_FLAG: DWORD = 0x00000002;
pub const CMSG_KEY_AGREE_ENCRYPT_FREE_PUBKEY_ALG_FLAG: DWORD = 0x00000004;
pub const CMSG_KEY_AGREE_ENCRYPT_FREE_PUBKEY_PARA_FLAG: DWORD = 0x00000008;
pub const CMSG_KEY_AGREE_ENCRYPT_FREE_PUBKEY_BITS_FLAG: DWORD = 0x00000010;
pub const CMSG_KEY_AGREE_ENCRYPT_FREE_OBJID_FLAG: DWORD = 0x00000020;
pub const CMSG_OID_EXPORT_KEY_AGREE_FUNC: &'static str = "CryptMsgDllExportKeyAgree";
pub const CMSG_OID_CAPI1_EXPORT_KEY_AGREE_FUNC: &'static str = CMSG_OID_EXPORT_KEY_AGREE_FUNC;
FN!{stdcall PFN_CMSG_EXPORT_KEY_AGREE(
    pContentEncryptInfo: PCMSG_CONTENT_ENCRYPT_INFO,
    pKeyAgreeEncodeInfo: PCMSG_KEY_AGREE_RECIPIENT_ENCODE_INFO,
    pKeyAgreeEncryptInfo: PCMSG_KEY_AGREE_ENCRYPT_INFO,
    dwFlags: DWORD,
    pvReserved: *mut c_void,
) -> BOOL}
pub const CMSG_OID_CNG_EXPORT_KEY_AGREE_FUNC: &'static str = "CryptMsgDllCNGExportKeyAgree";
STRUCT!{struct CMSG_MAIL_LIST_ENCRYPT_INFO {
    cbSize: DWORD,
    dwRecipientIndex: DWORD,
    KeyEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    EncryptedKey: CRYPT_DATA_BLOB,
    dwFlags: DWORD,
}}
pub type PCMSG_MAIL_LIST_ENCRYPT_INFO = *mut CMSG_MAIL_LIST_ENCRYPT_INFO;
pub const CMSG_MAIL_LIST_ENCRYPT_FREE_PARA_FLAG: DWORD = 0x00000001;
pub const CMSG_MAIL_LIST_ENCRYPT_FREE_OBJID_FLAG: DWORD = 0x00000002;
pub const CMSG_OID_EXPORT_MAIL_LIST_FUNC: &'static str = "CryptMsgDllExportMailList";
pub const CMSG_OID_CAPI1_EXPORT_MAIL_LIST_FUNC: &'static str = CMSG_OID_EXPORT_MAIL_LIST_FUNC;
FN!{stdcall PFN_CMSG_EXPORT_MAIL_LIST(
    pContentEncryptInfo: PCMSG_CONTENT_ENCRYPT_INFO,
    pMailListEncodeInfo: PCMSG_MAIL_LIST_RECIPIENT_ENCODE_INFO,
    pMailListEncryptInfo: PCMSG_MAIL_LIST_ENCRYPT_INFO,
    dwFlags: DWORD,
    pvReserved: *mut c_void,
) -> BOOL}
pub const CMSG_OID_IMPORT_KEY_TRANS_FUNC: &'static str = "CryptMsgDllImportKeyTrans";
pub const CMSG_OID_CAPI1_IMPORT_KEY_TRANS_FUNC: &'static str = CMSG_OID_IMPORT_KEY_TRANS_FUNC;
FN!{stdcall PFN_CMSG_IMPORT_KEY_TRANS(
    pContentEncryptionAlgorithm: PCRYPT_ALGORITHM_IDENTIFIER,
    pKeyTransDecryptPara: PCMSG_CTRL_KEY_TRANS_DECRYPT_PARA,
    dwFlags: DWORD,
    pvReserved: *mut c_void,
    phContentEncryptKey: *mut HCRYPTKEY,
) -> BOOL}
pub const CMSG_OID_IMPORT_KEY_AGREE_FUNC: &'static str = "CryptMsgDllImportKeyAgree";
pub const CMSG_OID_CAPI1_IMPORT_KEY_AGREE_FUNC: &'static str = CMSG_OID_IMPORT_KEY_AGREE_FUNC;
FN!{stdcall PFN_CMSG_IMPORT_KEY_AGREE(
    pContentEncryptionAlgorithm: PCRYPT_ALGORITHM_IDENTIFIER,
    pKeyAgreeDecryptPara: PCMSG_CTRL_KEY_AGREE_DECRYPT_PARA,
    dwFlags: DWORD,
    pvReserved: *mut c_void,
    phContentEncryptKey: *mut HCRYPTKEY,
) -> BOOL}
pub const CMSG_OID_IMPORT_MAIL_LIST_FUNC: &'static str = "CryptMsgDllImportMailList";
pub const CMSG_OID_CAPI1_IMPORT_MAIL_LIST_FUNC: &'static str = CMSG_OID_IMPORT_MAIL_LIST_FUNC;
FN!{stdcall PFN_CMSG_IMPORT_MAIL_LIST(
    pContentEncryptionAlgorithm: PCRYPT_ALGORITHM_IDENTIFIER,
    pMailListDecryptPara: PCMSG_CTRL_MAIL_LIST_DECRYPT_PARA,
    dwFlags: DWORD,
    pvReserved: *mut c_void,
    phContentEncryptKey: *mut HCRYPTKEY,
) -> BOOL}
STRUCT!{struct CMSG_CNG_CONTENT_DECRYPT_INFO {
    cbSize: DWORD,
    ContentEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pfnAlloc: PFN_CMSG_ALLOC,
    pfnFree: PFN_CMSG_FREE,
    hNCryptKey: NCRYPT_KEY_HANDLE,
    pbContentEncryptKey: *mut BYTE,
    cbContentEncryptKey: DWORD,
    hCNGContentEncryptKey: BCRYPT_KEY_HANDLE,
    pbCNGContentEncryptKeyObject: *mut BYTE,
}}
pub type PCMSG_CNG_CONTENT_DECRYPT_INFO = *mut CMSG_CNG_CONTENT_DECRYPT_INFO;
pub const CMSG_OID_CNG_IMPORT_KEY_TRANS_FUNC: &'static str = "CryptMsgDllCNGImportKeyTrans";
FN!{stdcall PFN_CMSG_CNG_IMPORT_KEY_TRANS(
    pCNGContentDecryptInfo: PCMSG_CNG_CONTENT_DECRYPT_INFO,
    pKeyTransDecryptPara: PCMSG_CTRL_KEY_TRANS_DECRYPT_PARA,
    dwFlags: DWORD,
    pvReserved: *mut c_void,
) -> BOOL}
pub const CMSG_OID_CNG_IMPORT_KEY_AGREE_FUNC: &'static str = "CryptMsgDllCNGImportKeyAgree";
FN!{stdcall PFN_CMSG_CNG_IMPORT_KEY_AGREE(
    pCNGContentDecryptInfo: PCMSG_CNG_CONTENT_DECRYPT_INFO,
    pKeyAgreeDecryptPara: PCMSG_CTRL_KEY_AGREE_DECRYPT_PARA,
    dwFlags: DWORD,
    pvReserved: *mut c_void,
) -> BOOL}
pub const CMSG_OID_CNG_IMPORT_CONTENT_ENCRYPT_KEY_FUNC: &'static str
    = "CryptMsgDllCNGImportContentEncryptKey";
FN!{stdcall PFN_CMSG_CNG_IMPORT_CONTENT_ENCRYPT_KEY(
    pCNGContentDecryptInfo: PCMSG_CNG_CONTENT_DECRYPT_INFO,
    dwFlags: DWORD,
    pvReserved: *mut c_void,
) -> BOOL}
pub type HCERTSTORE = *mut c_void;
STRUCT!{struct CERT_CONTEXT {
    dwCertEncodingType: DWORD,
    pbCertEncoded: *mut BYTE,
    cbCertEncoded: DWORD,
    pCertInfo: PCERT_INFO,
    hCertStore: HCERTSTORE,
}}
pub type PCERT_CONTEXT = *mut CERT_CONTEXT;
pub type PCCERT_CONTEXT = *const CERT_CONTEXT;
STRUCT!{struct CRL_CONTEXT {
    dwCertEncodingType: DWORD,
    pbCrlEncoded: *mut BYTE,
    cbCrlEncoded: DWORD,
    pCrlInfo: PCRL_INFO,
    hCertStore: HCERTSTORE,
}}
pub type PCRL_CONTEXT = *mut CRL_CONTEXT;
pub type PCCRL_CONTEXT = *const CRL_CONTEXT;
STRUCT!{struct CTL_CONTEXT {
    dwMsgAndCertEncodingType: DWORD,
    pbCtlEncoded: *mut BYTE,
    cbCtlEncoded: DWORD,
    pCtlInfo: PCTL_INFO,
    hCertStore: HCERTSTORE,
    hCryptMsg: HCRYPTMSG,
    pbCtlContent: *mut BYTE,
    cbCtlContent: DWORD,
}}
pub type PCTL_CONTEXT = *mut CTL_CONTEXT;
pub type PCCTL_CONTEXT = *const CTL_CONTEXT;
pub const CERT_KEY_PROV_HANDLE_PROP_ID: DWORD = 1;
pub const CERT_KEY_PROV_INFO_PROP_ID: DWORD = 2;
pub const CERT_SHA1_HASH_PROP_ID: DWORD = 3;
pub const CERT_MD5_HASH_PROP_ID: DWORD = 4;
pub const CERT_HASH_PROP_ID: DWORD = CERT_SHA1_HASH_PROP_ID;
pub const CERT_KEY_CONTEXT_PROP_ID: DWORD = 5;
pub const CERT_KEY_SPEC_PROP_ID: DWORD = 6;
pub const CERT_IE30_RESERVED_PROP_ID: DWORD = 7;
pub const CERT_PUBKEY_HASH_RESERVED_PROP_ID: DWORD = 8;
pub const CERT_ENHKEY_USAGE_PROP_ID: DWORD = 9;
pub const CERT_CTL_USAGE_PROP_ID: DWORD = CERT_ENHKEY_USAGE_PROP_ID;
pub const CERT_NEXT_UPDATE_LOCATION_PROP_ID: DWORD = 10;
pub const CERT_FRIENDLY_NAME_PROP_ID: DWORD = 11;
pub const CERT_PVK_FILE_PROP_ID: DWORD = 12;
pub const CERT_DESCRIPTION_PROP_ID: DWORD = 13;
pub const CERT_ACCESS_STATE_PROP_ID: DWORD = 14;
pub const CERT_SIGNATURE_HASH_PROP_ID: DWORD = 15;
pub const CERT_SMART_CARD_DATA_PROP_ID: DWORD = 16;
pub const CERT_EFS_PROP_ID: DWORD = 17;
pub const CERT_FORTEZZA_DATA_PROP_ID: DWORD = 18;
pub const CERT_ARCHIVED_PROP_ID: DWORD = 19;
pub const CERT_KEY_IDENTIFIER_PROP_ID: DWORD = 20;
pub const CERT_AUTO_ENROLL_PROP_ID: DWORD = 21;
pub const CERT_PUBKEY_ALG_PARA_PROP_ID: DWORD = 22;
pub const CERT_CROSS_CERT_DIST_POINTS_PROP_ID: DWORD = 23;
pub const CERT_ISSUER_PUBLIC_KEY_MD5_HASH_PROP_ID: DWORD = 24;
pub const CERT_SUBJECT_PUBLIC_KEY_MD5_HASH_PROP_ID: DWORD = 25;
pub const CERT_ENROLLMENT_PROP_ID: DWORD = 26;
pub const CERT_DATE_STAMP_PROP_ID: DWORD = 27;
pub const CERT_ISSUER_SERIAL_NUMBER_MD5_HASH_PROP_ID: DWORD = 28;
pub const CERT_SUBJECT_NAME_MD5_HASH_PROP_ID: DWORD = 29;
pub const CERT_EXTENDED_ERROR_INFO_PROP_ID: DWORD = 30;
pub const CERT_RENEWAL_PROP_ID: DWORD = 64;
pub const CERT_ARCHIVED_KEY_HASH_PROP_ID: DWORD = 65;
pub const CERT_AUTO_ENROLL_RETRY_PROP_ID: DWORD = 66;
pub const CERT_AIA_URL_RETRIEVED_PROP_ID: DWORD = 67;
pub const CERT_AUTHORITY_INFO_ACCESS_PROP_ID: DWORD = 68;
pub const CERT_BACKED_UP_PROP_ID: DWORD = 69;
pub const CERT_OCSP_RESPONSE_PROP_ID: DWORD = 70;
pub const CERT_REQUEST_ORIGINATOR_PROP_ID: DWORD = 71;
pub const CERT_SOURCE_LOCATION_PROP_ID: DWORD = 72;
pub const CERT_SOURCE_URL_PROP_ID: DWORD = 73;
pub const CERT_NEW_KEY_PROP_ID: DWORD = 74;
pub const CERT_OCSP_CACHE_PREFIX_PROP_ID: DWORD = 75;
pub const CERT_SMART_CARD_ROOT_INFO_PROP_ID: DWORD = 76;
pub const CERT_NO_AUTO_EXPIRE_CHECK_PROP_ID: DWORD = 77;
pub const CERT_NCRYPT_KEY_HANDLE_PROP_ID: DWORD = 78;
pub const CERT_HCRYPTPROV_OR_NCRYPT_KEY_HANDLE_PROP_ID: DWORD = 79;
pub const CERT_SUBJECT_INFO_ACCESS_PROP_ID: DWORD = 80;
pub const CERT_CA_OCSP_AUTHORITY_INFO_ACCESS_PROP_ID: DWORD = 81;
pub const CERT_CA_DISABLE_CRL_PROP_ID: DWORD = 82;
pub const CERT_ROOT_PROGRAM_CERT_POLICIES_PROP_ID: DWORD = 83;
pub const CERT_ROOT_PROGRAM_NAME_CONSTRAINTS_PROP_ID: DWORD = 84;
pub const CERT_SUBJECT_OCSP_AUTHORITY_INFO_ACCESS_PROP_ID: DWORD = 85;
pub const CERT_SUBJECT_DISABLE_CRL_PROP_ID: DWORD = 86;
pub const CERT_CEP_PROP_ID: DWORD = 87;
pub const CERT_SIGN_HASH_CNG_ALG_PROP_ID: DWORD = 89;
pub const CERT_SCARD_PIN_ID_PROP_ID: DWORD = 90;
pub const CERT_SCARD_PIN_INFO_PROP_ID: DWORD = 91;
pub const CERT_SUBJECT_PUB_KEY_BIT_LENGTH_PROP_ID: DWORD = 92;
pub const CERT_PUB_KEY_CNG_ALG_BIT_LENGTH_PROP_ID: DWORD = 93;
pub const CERT_ISSUER_PUB_KEY_BIT_LENGTH_PROP_ID: DWORD = 94;
pub const CERT_ISSUER_CHAIN_SIGN_HASH_CNG_ALG_PROP_ID: DWORD = 95;
pub const CERT_ISSUER_CHAIN_PUB_KEY_CNG_ALG_BIT_LENGTH_PROP_ID: DWORD = 96;
pub const CERT_NO_EXPIRE_NOTIFICATION_PROP_ID: DWORD = 97;
pub const CERT_AUTH_ROOT_SHA256_HASH_PROP_ID: DWORD = 98;
pub const CERT_NCRYPT_KEY_HANDLE_TRANSFER_PROP_ID: DWORD = 99;
pub const CERT_HCRYPTPROV_TRANSFER_PROP_ID: DWORD = 100;
pub const CERT_SMART_CARD_READER_PROP_ID: DWORD = 101;
pub const CERT_SEND_AS_TRUSTED_ISSUER_PROP_ID: DWORD = 102;
pub const CERT_KEY_REPAIR_ATTEMPTED_PROP_ID: DWORD = 103;
pub const CERT_DISALLOWED_FILETIME_PROP_ID: DWORD = 104;
pub const CERT_ROOT_PROGRAM_CHAIN_POLICIES_PROP_ID: DWORD = 105;
pub const CERT_SMART_CARD_READER_NON_REMOVABLE_PROP_ID: DWORD = 106;
pub const CERT_SHA256_HASH_PROP_ID: DWORD = 107;
pub const CERT_SCEP_SERVER_CERTS_PROP_ID: DWORD = 108;
pub const CERT_SCEP_RA_SIGNATURE_CERT_PROP_ID: DWORD = 109;
pub const CERT_SCEP_RA_ENCRYPTION_CERT_PROP_ID: DWORD = 110;
pub const CERT_SCEP_CA_CERT_PROP_ID: DWORD = 111;
pub const CERT_SCEP_SIGNER_CERT_PROP_ID: DWORD = 112;
pub const CERT_SCEP_NONCE_PROP_ID: DWORD = 113;
pub const CERT_SCEP_ENCRYPT_HASH_CNG_ALG_PROP_ID: DWORD = 114;
pub const CERT_SCEP_FLAGS_PROP_ID: DWORD = 115;
pub const CERT_SCEP_GUID_PROP_ID: DWORD = 116;
pub const CERT_SERIALIZABLE_KEY_CONTEXT_PROP_ID: DWORD = 117;
pub const CERT_ISOLATED_KEY_PROP_ID: DWORD = 118;
pub const CERT_SERIAL_CHAIN_PROP_ID: DWORD = 119;
pub const CERT_KEY_CLASSIFICATION_PROP_ID: DWORD = 120;
pub const CERT_OCSP_MUST_STAPLE_PROP_ID: DWORD = 121;
pub const CERT_DISALLOWED_ENHKEY_USAGE_PROP_ID: DWORD = 122;
pub const CERT_NONCOMPLIANT_ROOT_URL_PROP_ID: DWORD = 123;
pub const CERT_PIN_SHA256_HASH_PROP_ID: DWORD = 124;
pub const CERT_CLR_DELETE_KEY_PROP_ID: DWORD = 125;
pub const CERT_NOT_BEFORE_FILETIME_PROP_ID: DWORD = 126;
pub const CERT_NOT_BEFORE_ENHKEY_USAGE_PROP_ID: DWORD = 127;
pub const CERT_FIRST_RESERVED_PROP_ID: DWORD = 128;
pub const CERT_LAST_RESERVED_PROP_ID: DWORD = 0x00007FFF;
pub const CERT_FIRST_USER_PROP_ID: DWORD = 0x00008000;
pub const CERT_LAST_USER_PROP_ID: DWORD = 0x0000FFFF;
ENUM!{enum CertKeyType {
    KeyTypeOther = 0,
    KeyTypeVirtualSmartCard = 1,
    KeyTypePhysicalSmartCard = 2,
    KeyTypePassport = 3,
    KeyTypePassportRemote = 4,
    KeyTypePassportSmartCard = 5,
    KeyTypeHardware = 6,
    KeyTypeSoftware = 7,
    KeyTypeSelfSigned = 8,
}}
#[inline]
pub fn IS_CERT_HASH_PROP_ID(X: DWORD) -> bool {
    CERT_SHA1_HASH_PROP_ID == X || CERT_MD5_HASH_PROP_ID == X || CERT_SHA256_HASH_PROP_ID == X
    || CERT_SIGNATURE_HASH_PROP_ID == X
}
#[inline]
pub fn IS_PUBKEY_HASH_PROP_ID(X: DWORD) -> bool {
    CERT_ISSUER_PUBLIC_KEY_MD5_HASH_PROP_ID == X || CERT_PIN_SHA256_HASH_PROP_ID == X
    || CERT_SUBJECT_PUBLIC_KEY_MD5_HASH_PROP_ID == X
}
#[inline]
pub fn IS_CHAIN_HASH_PROP_ID(X: DWORD) -> bool {
    CERT_ISSUER_PUBLIC_KEY_MD5_HASH_PROP_ID == X || CERT_SUBJECT_PUBLIC_KEY_MD5_HASH_PROP_ID == X
    || CERT_ISSUER_SERIAL_NUMBER_MD5_HASH_PROP_ID == X || CERT_SUBJECT_NAME_MD5_HASH_PROP_ID == X
}
#[inline]
pub fn IS_STRONG_SIGN_PROP_ID(X: DWORD) -> bool {
    CERT_SIGN_HASH_CNG_ALG_PROP_ID == X || CERT_SUBJECT_PUB_KEY_BIT_LENGTH_PROP_ID == X
    || CERT_PUB_KEY_CNG_ALG_BIT_LENGTH_PROP_ID == X
}
pub const szOID_CERT_PROP_ID_PREFIX: &'static str = "1.3.6.1.4.1.311.10.11.";
pub const szOID_CERT_KEY_IDENTIFIER_PROP_ID: &'static str = "1.3.6.1.4.1.311.10.11.20";
pub const szOID_CERT_ISSUER_SERIAL_NUMBER_MD5_HASH_PROP_ID: &'static str
    = "1.3.6.1.4.1.311.10.11.28";
pub const szOID_CERT_SUBJECT_NAME_MD5_HASH_PROP_ID: &'static str = "1.3.6.1.4.1.311.10.11.29";
pub const szOID_CERT_MD5_HASH_PROP_ID: &'static str = "1.3.6.1.4.1.311.10.11.4";
pub const szOID_CERT_SIGNATURE_HASH_PROP_ID: &'static str = "1.3.6.1.4.1.311.10.11.15";
pub const szOID_DISALLOWED_HASH: &'static str = szOID_CERT_SIGNATURE_HASH_PROP_ID;
pub const szOID_CERT_DISALLOWED_FILETIME_PROP_ID: &'static str = "1.3.6.1.4.1.311.10.11.104";
pub const CERT_ACCESS_STATE_WRITE_PERSIST_FLAG: DWORD = 0x1;
pub const CERT_ACCESS_STATE_SYSTEM_STORE_FLAG: DWORD = 0x2;
pub const CERT_ACCESS_STATE_LM_SYSTEM_STORE_FLAG: DWORD = 0x4;
pub const CERT_ACCESS_STATE_GP_SYSTEM_STORE_FLAG: DWORD = 0x8;
pub const CERT_ACCESS_STATE_SHARED_USER_FLAG: DWORD = 0x10;
pub const szOID_ROOT_PROGRAM_AUTO_UPDATE_CA_REVOCATION: &'static str = "1.3.6.1.4.1.311.60.3.1";
pub const szOID_ROOT_PROGRAM_AUTO_UPDATE_END_REVOCATION: &'static str = "1.3.6.1.4.1.311.60.3.2";
pub const szOID_ROOT_PROGRAM_NO_OCSP_FAILOVER_TO_CRL: &'static str = "1.3.6.1.4.1.311.60.3.3";
STRUCT!{struct CRYPT_KEY_PROV_PARAM {
    dwParam: DWORD,
    pbData: *mut BYTE,
    cbData: DWORD,
    dwFlags: DWORD,
}}
pub type PCRYPT_KEY_PROV_PARAM = *mut CRYPT_KEY_PROV_PARAM;
STRUCT!{struct CRYPT_KEY_PROV_INFO {
    pwszContainerName: LPWSTR,
    pwszProvName: LPWSTR,
    dwProvType: DWORD,
    dwFlags: DWORD,
    cProvParam: DWORD,
    rgProvParam: PCRYPT_KEY_PROV_PARAM,
    dwKeySpec: DWORD,
}}
pub type PCRYPT_KEY_PROV_INFO = *mut CRYPT_KEY_PROV_INFO;
pub const CERT_SET_KEY_PROV_HANDLE_PROP_ID: DWORD = 0x00000001;
pub const CERT_SET_KEY_CONTEXT_PROP_ID: DWORD = 0x00000001;
pub const CERT_NCRYPT_KEY_SPEC: DWORD = 0xFFFFFFFF;
UNION!{union CERT_KEY_CONTEXT_u {
    [usize; 1],
    hCryptProv hCryptProv_mut: HCRYPTPROV,
    hNCryptKey hNCryptKey_mut: NCRYPT_KEY_HANDLE,
}}
STRUCT!{struct CERT_KEY_CONTEXT {
    cbSize: DWORD,
    u: CERT_KEY_CONTEXT_u,
    dwKeySpec: DWORD,
}}
pub type PCERT_KEY_CONTEXT = *mut CERT_KEY_CONTEXT;
STRUCT!{struct ROOT_INFO_LUID {
    LowPart: DWORD,
    HighPart: LONG,
}}
pub type PROOT_INFO_LUID = *mut ROOT_INFO_LUID;
STRUCT!{struct CRYPT_SMART_CARD_ROOT_INFO {
    rgbCardID: [BYTE; 16],
    luid: ROOT_INFO_LUID,
}}
pub type PCRYPT_SMART_CARD_ROOT_INFO = *mut CRYPT_SMART_CARD_ROOT_INFO;
pub const CERT_STORE_PROV_MSG: LPCSTR = 1 as LPCSTR;
pub const CERT_STORE_PROV_MEMORY: LPCSTR = 2 as LPCSTR;
pub const CERT_STORE_PROV_FILE: LPCSTR = 3 as LPCSTR;
pub const CERT_STORE_PROV_REG: LPCSTR = 4 as LPCSTR;
pub const CERT_STORE_PROV_PKCS7: LPCSTR = 5 as LPCSTR;
pub const CERT_STORE_PROV_SERIALIZED: LPCSTR = 6 as LPCSTR;
pub const CERT_STORE_PROV_FILENAME_A: LPCSTR = 7 as LPCSTR;
pub const CERT_STORE_PROV_FILENAME_W: LPCSTR = 8 as LPCSTR;
pub const CERT_STORE_PROV_FILENAME: LPCSTR = CERT_STORE_PROV_FILENAME_W;
pub const CERT_STORE_PROV_SYSTEM_A: LPCSTR = 9 as LPCSTR;
pub const CERT_STORE_PROV_SYSTEM_W: LPCSTR = 10 as LPCSTR;
pub const CERT_STORE_PROV_SYSTEM: LPCSTR = CERT_STORE_PROV_SYSTEM_W;
pub const CERT_STORE_PROV_COLLECTION: LPCSTR = 11 as LPCSTR;
pub const CERT_STORE_PROV_SYSTEM_REGISTRY_A: LPCSTR = 12 as LPCSTR;
pub const CERT_STORE_PROV_SYSTEM_REGISTRY_W: LPCSTR = 13 as LPCSTR;
pub const CERT_STORE_PROV_SYSTEM_REGISTRY: LPCSTR = CERT_STORE_PROV_SYSTEM_REGISTRY_W;
pub const CERT_STORE_PROV_PHYSICAL_W: LPCSTR = 14 as LPCSTR;
pub const CERT_STORE_PROV_PHYSICAL: LPCSTR = CERT_STORE_PROV_PHYSICAL_W;
pub const CERT_STORE_PROV_SMART_CARD_W: LPCSTR = 15 as LPCSTR;
pub const CERT_STORE_PROV_SMART_CARD: LPCSTR = CERT_STORE_PROV_SMART_CARD_W;
pub const CERT_STORE_PROV_LDAP_W: LPCSTR = 16 as LPCSTR;
pub const CERT_STORE_PROV_LDAP: LPCSTR = CERT_STORE_PROV_LDAP_W;
pub const CERT_STORE_PROV_PKCS12: LPCSTR = 17 as LPCSTR;
pub const sz_CERT_STORE_PROV_MEMORY: &'static str = "Memory";
pub const sz_CERT_STORE_PROV_FILENAME_W: &'static str = "File";
pub const sz_CERT_STORE_PROV_FILENAME: &'static str = sz_CERT_STORE_PROV_FILENAME_W;
pub const sz_CERT_STORE_PROV_SYSTEM_W: &'static str = "System";
pub const sz_CERT_STORE_PROV_SYSTEM: &'static str = sz_CERT_STORE_PROV_SYSTEM_W;
pub const sz_CERT_STORE_PROV_PKCS7: &'static str = "PKCS7";
pub const sz_CERT_STORE_PROV_PKCS12: &'static str = "PKCS12";
pub const sz_CERT_STORE_PROV_SERIALIZED: &'static str = "Serialized";
pub const sz_CERT_STORE_PROV_COLLECTION: &'static str = "Collection";
pub const sz_CERT_STORE_PROV_SYSTEM_REGISTRY_W: &'static str = "SystemRegistry";
pub const sz_CERT_STORE_PROV_SYSTEM_REGISTRY: &'static str = sz_CERT_STORE_PROV_SYSTEM_REGISTRY_W;
pub const sz_CERT_STORE_PROV_PHYSICAL_W: &'static str = "Physical";
pub const sz_CERT_STORE_PROV_PHYSICAL: &'static str = sz_CERT_STORE_PROV_PHYSICAL_W;
pub const sz_CERT_STORE_PROV_SMART_CARD_W: &'static str = "SmartCard";
pub const sz_CERT_STORE_PROV_SMART_CARD: &'static str = sz_CERT_STORE_PROV_SMART_CARD_W;
pub const sz_CERT_STORE_PROV_LDAP_W: &'static str = "Ldap";
pub const sz_CERT_STORE_PROV_LDAP: &'static str = sz_CERT_STORE_PROV_LDAP_W;
pub const CERT_STORE_SIGNATURE_FLAG: DWORD = 0x00000001;
pub const CERT_STORE_TIME_VALIDITY_FLAG: DWORD = 0x00000002;
pub const CERT_STORE_REVOCATION_FLAG: DWORD = 0x00000004;
pub const CERT_STORE_NO_CRL_FLAG: DWORD = 0x00010000;
pub const CERT_STORE_NO_ISSUER_FLAG: DWORD = 0x00020000;
pub const CERT_STORE_BASE_CRL_FLAG: DWORD = 0x00000100;
pub const CERT_STORE_DELTA_CRL_FLAG: DWORD = 0x00000200;
pub const CERT_STORE_NO_CRYPT_RELEASE_FLAG: DWORD = 0x00000001;
pub const CERT_STORE_SET_LOCALIZED_NAME_FLAG: DWORD = 0x00000002;
pub const CERT_STORE_DEFER_CLOSE_UNTIL_LAST_FREE_FLAG: DWORD = 0x00000004;
pub const CERT_STORE_DELETE_FLAG: DWORD = 0x00000010;
pub const CERT_STORE_SHARE_STORE_FLAG: DWORD = 0x00000040;
pub const CERT_STORE_SHARE_CONTEXT_FLAG: DWORD = 0x00000080;
pub const CERT_STORE_MANIFOLD_FLAG: DWORD = 0x00000100;
pub const CERT_STORE_ENUM_ARCHIVED_FLAG: DWORD = 0x00000200;
pub const CERT_STORE_UPDATE_KEYID_FLAG: DWORD = 0x00000400;
pub const CERT_STORE_BACKUP_RESTORE_FLAG: DWORD = 0x00000800;
pub const CERT_STORE_READONLY_FLAG: DWORD = 0x00008000;
pub const CERT_STORE_OPEN_EXISTING_FLAG: DWORD = 0x00004000;
pub const CERT_STORE_CREATE_NEW_FLAG: DWORD = 0x00002000;
pub const CERT_STORE_MAXIMUM_ALLOWED_FLAG: DWORD = 0x00001000;
pub const CERT_SYSTEM_STORE_MASK: DWORD = 0xFFFF0000;
pub const CERT_SYSTEM_STORE_RELOCATE_FLAG: DWORD = 0x80000000;
UNION!{union CERT_SYSTEM_STORE_RELOCATE_PARA_u1 {
    [usize; 1],
    hKeyBase hKeyBase_mut: HKEY,
    pvBase pvBase_mut: *mut c_void,
}}
UNION!{union CERT_SYSTEM_STORE_RELOCATE_PARA_u2 {
    [usize; 1],
    pvSystemStore pvSystemStore__mut: *mut c_void,
    pszSystemStore pszSystemStore_mut: LPCSTR,
    pwszSystemStore pwszSystemStore_mut: LPCWSTR,
}}
STRUCT!{struct CERT_SYSTEM_STORE_RELOCATE_PARA {
    u1: CERT_SYSTEM_STORE_RELOCATE_PARA_u1,
    u2: CERT_SYSTEM_STORE_RELOCATE_PARA_u2,
}}
pub type PCERT_SYSTEM_STORE_RELOCATE_PARA = *mut CERT_SYSTEM_STORE_RELOCATE_PARA;
pub const CERT_SYSTEM_STORE_UNPROTECTED_FLAG: DWORD = 0x40000000;
pub const CERT_SYSTEM_STORE_LOCATION_MASK: DWORD = 0x00FF0000;
pub const CERT_SYSTEM_STORE_LOCATION_SHIFT: DWORD = 16;
pub const CERT_SYSTEM_STORE_CURRENT_USER_ID: DWORD = 1;
pub const CERT_SYSTEM_STORE_LOCAL_MACHINE_ID: DWORD = 2;
pub const CERT_SYSTEM_STORE_CURRENT_SERVICE_ID: DWORD = 4;
pub const CERT_SYSTEM_STORE_SERVICES_ID: DWORD = 5;
pub const CERT_SYSTEM_STORE_USERS_ID: DWORD = 6;
pub const CERT_SYSTEM_STORE_CURRENT_USER_GROUP_POLICY_ID: DWORD = 7;
pub const CERT_SYSTEM_STORE_LOCAL_MACHINE_GROUP_POLICY_ID: DWORD = 8;
pub const CERT_SYSTEM_STORE_LOCAL_MACHINE_ENTERPRISE_ID: DWORD = 9;
pub const CERT_SYSTEM_STORE_CURRENT_USER: DWORD = CERT_SYSTEM_STORE_CURRENT_USER_ID
    << CERT_SYSTEM_STORE_LOCATION_SHIFT;
pub const CERT_SYSTEM_STORE_LOCAL_MACHINE: DWORD = CERT_SYSTEM_STORE_LOCAL_MACHINE_ID
    << CERT_SYSTEM_STORE_LOCATION_SHIFT;
pub const CERT_SYSTEM_STORE_CURRENT_SERVICE: DWORD = CERT_SYSTEM_STORE_CURRENT_SERVICE_ID
    << CERT_SYSTEM_STORE_LOCATION_SHIFT;
pub const CERT_SYSTEM_STORE_SERVICES: DWORD = CERT_SYSTEM_STORE_SERVICES_ID
    << CERT_SYSTEM_STORE_LOCATION_SHIFT;
pub const CERT_SYSTEM_STORE_USERS: DWORD = CERT_SYSTEM_STORE_USERS_ID
    << CERT_SYSTEM_STORE_LOCATION_SHIFT;
pub const CERT_SYSTEM_STORE_CURRENT_USER_GROUP_POLICY: DWORD
    = CERT_SYSTEM_STORE_CURRENT_USER_GROUP_POLICY_ID << CERT_SYSTEM_STORE_LOCATION_SHIFT;
pub const CERT_SYSTEM_STORE_LOCAL_MACHINE_GROUP_POLICY: DWORD
    = CERT_SYSTEM_STORE_LOCAL_MACHINE_GROUP_POLICY_ID << CERT_SYSTEM_STORE_LOCATION_SHIFT;
pub const CERT_SYSTEM_STORE_LOCAL_MACHINE_ENTERPRISE: DWORD
    = CERT_SYSTEM_STORE_LOCAL_MACHINE_ENTERPRISE_ID << CERT_SYSTEM_STORE_LOCATION_SHIFT;
pub const CERT_GROUP_POLICY_SYSTEM_STORE_REGPATH: &'static str
    = "Software\\Policies\\Microsoft\\SystemCertificates";
pub const CERT_EFSBLOB_REGPATH: &'static str
    = "Software\\Policies\\Microsoft\\SystemCertificates\\EFS";
pub const CERT_EFSBLOB_VALUE_NAME: &'static str = "EFSBlob";
pub const CERT_PROT_ROOT_FLAGS_REGPATH: &'static str
    = "Software\\Policies\\Microsoft\\SystemCertificates\\Root\\ProtectedRoots";
pub const CERT_PROT_ROOT_FLAGS_VALUE_NAME: &'static str = "Flags";
pub const CERT_PROT_ROOT_DISABLE_CURRENT_USER_FLAG: DWORD = 0x1;
pub const CERT_PROT_ROOT_INHIBIT_ADD_AT_INIT_FLAG: DWORD = 0x2;
pub const CERT_PROT_ROOT_INHIBIT_PURGE_LM_FLAG: DWORD = 0x4;
pub const CERT_PROT_ROOT_DISABLE_LM_AUTH_FLAG: DWORD = 0x8;
pub const CERT_PROT_ROOT_ONLY_LM_GPT_FLAG: DWORD = 0x8;
pub const CERT_PROT_ROOT_DISABLE_NT_AUTH_REQUIRED_FLAG: DWORD = 0x10;
pub const CERT_PROT_ROOT_DISABLE_NOT_DEFINED_NAME_CONSTRAINT_FLAG: DWORD = 0x20;
pub const CERT_PROT_ROOT_DISABLE_PEER_TRUST: DWORD = 0x10000;
pub const CERT_PROT_ROOT_PEER_USAGES_VALUE_NAME: &'static str = "PeerUsages";
pub const CERT_PROT_ROOT_PEER_USAGES_VALUE_NAME_A: &'static str = "PeerUsages";
pub const CERT_PROT_ROOT_PEER_USAGES_DEFAULT_A: &'static str
    = "1.3.6.1.5.5.7.3.2\01.3.6.1.5.5.7.3.4\01.3.6.1.4.1.311.10.3.4\0";
pub const CERT_TRUST_PUB_SAFER_GROUP_POLICY_REGPATH: &'static str
    = "Software\\Policies\\Microsoft\\SystemCertificates\\TrustedPublisher\\Safer";
pub const CERT_LOCAL_MACHINE_SYSTEM_STORE_REGPATH: &'static str
    = "Software\\Microsoft\\SystemCertificates";
pub const CERT_TRUST_PUB_SAFER_LOCAL_MACHINE_REGPATH: &'static str
    = "Software\\Microsoft\\SystemCertificates\\TrustedPublisher\\Safer";
pub const CERT_TRUST_PUB_AUTHENTICODE_FLAGS_VALUE_NAME: &'static str = "AuthenticodeFlags";
pub const CERT_TRUST_PUB_ALLOW_TRUST_MASK: DWORD = 0x00000003;
pub const CERT_TRUST_PUB_ALLOW_END_USER_TRUST: DWORD = 0x00000000;
pub const CERT_TRUST_PUB_ALLOW_MACHINE_ADMIN_TRUST: DWORD = 0x00000001;
pub const CERT_TRUST_PUB_ALLOW_ENTERPRISE_ADMIN_TRUST: DWORD = 0x00000002;
pub const CERT_TRUST_PUB_CHECK_PUBLISHER_REV_FLAG: DWORD = 0x00000100;
pub const CERT_TRUST_PUB_CHECK_TIMESTAMP_REV_FLAG: DWORD = 0x00000200;
pub const CERT_OCM_SUBCOMPONENTS_LOCAL_MACHINE_REGPATH: &'static str
    = "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Setup\\OC Manager\\Subcomponents";
pub const CERT_OCM_SUBCOMPONENTS_ROOT_AUTO_UPDATE_VALUE_NAME: &'static str = "RootAutoUpdate";
pub const CERT_DISABLE_ROOT_AUTO_UPDATE_REGPATH: &'static str
    = "Software\\Policies\\Microsoft\\SystemCertificates\\AuthRoot";
pub const CERT_DISABLE_ROOT_AUTO_UPDATE_VALUE_NAME: &'static str = "DisableRootAutoUpdate";
pub const CERT_ENABLE_DISALLOWED_CERT_AUTO_UPDATE_VALUE_NAME: &'static str
    = "EnableDisallowedCertAutoUpdate";
pub const CERT_DISABLE_PIN_RULES_AUTO_UPDATE_VALUE_NAME: &'static str
    = "DisablePinRulesAutoUpdate";
pub const CERT_AUTO_UPDATE_LOCAL_MACHINE_REGPATH: &'static str
    = "Software\\Microsoft\\SystemCertificates\\AuthRoot\\AutoUpdate";
pub const CERT_AUTO_UPDATE_ROOT_DIR_URL_VALUE_NAME: &'static str = "RootDirUrl";
pub const CERT_AUTO_UPDATE_SYNC_FROM_DIR_URL_VALUE_NAME: &'static str = "SyncFromDirUrl";
pub const CERT_AUTH_ROOT_AUTO_UPDATE_LOCAL_MACHINE_REGPATH: &'static str
    = CERT_AUTO_UPDATE_LOCAL_MACHINE_REGPATH;
pub const CERT_AUTH_ROOT_AUTO_UPDATE_ROOT_DIR_URL_VALUE_NAME: &'static str
    = CERT_AUTO_UPDATE_ROOT_DIR_URL_VALUE_NAME;
pub const CERT_AUTH_ROOT_AUTO_UPDATE_SYNC_DELTA_TIME_VALUE_NAME: &'static str = "SyncDeltaTime";
pub const CERT_AUTH_ROOT_AUTO_UPDATE_FLAGS_VALUE_NAME: &'static str = "Flags";
pub const CERT_AUTH_ROOT_AUTO_UPDATE_DISABLE_UNTRUSTED_ROOT_LOGGING_FLAG: DWORD = 0x1;
pub const CERT_AUTH_ROOT_AUTO_UPDATE_DISABLE_PARTIAL_CHAIN_LOGGING_FLAG: DWORD = 0x2;
pub const CERT_AUTO_UPDATE_DISABLE_RANDOM_QUERY_STRING_FLAG: DWORD = 0x4;
pub const CERT_AUTH_ROOT_AUTO_UPDATE_LAST_SYNC_TIME_VALUE_NAME: &'static str = "LastSyncTime";
pub const CERT_AUTH_ROOT_AUTO_UPDATE_ENCODED_CTL_VALUE_NAME: &'static str = "EncodedCt";
pub const CERT_AUTH_ROOT_CTL_FILENAME: &'static str = "authroot.st";
pub const CERT_AUTH_ROOT_CTL_FILENAME_A: &'static str = "authroot.st";
pub const CERT_AUTH_ROOT_CAB_FILENAME: &'static str = "authrootstl.cab";
pub const CERT_AUTH_ROOT_SEQ_FILENAME: &'static str = "authrootseq.txt";
pub const CERT_AUTH_ROOT_CERT_EXT: &'static str = ".crt";
pub const CERT_DISALLOWED_CERT_AUTO_UPDATE_SYNC_DELTA_TIME_VALUE_NAME: &'static str
    = "DisallowedCertSyncDeltaTime";
pub const CERT_DISALLOWED_CERT_AUTO_UPDATE_LAST_SYNC_TIME_VALUE_NAME: &'static str
    = "DisallowedCertLastSyncTime";
pub const CERT_DISALLOWED_CERT_AUTO_UPDATE_ENCODED_CTL_VALUE_NAME: &'static str
    = "DisallowedCertEncodedCt";
pub const CERT_DISALLOWED_CERT_CTL_FILENAME: &'static str = "disallowedcert.st";
pub const CERT_DISALLOWED_CERT_CTL_FILENAME_A: &'static str = "disallowedcert.st";
pub const CERT_DISALLOWED_CERT_CAB_FILENAME: &'static str = "disallowedcertstl.cab";
pub const CERT_DISALLOWED_CERT_AUTO_UPDATE_LIST_IDENTIFIER: &'static str
    = "DisallowedCert_AutoUpdate_1";
pub const CERT_PIN_RULES_AUTO_UPDATE_SYNC_DELTA_TIME_VALUE_NAME: &'static str
    = "PinRulesSyncDeltaTime";
pub const CERT_PIN_RULES_AUTO_UPDATE_LAST_SYNC_TIME_VALUE_NAME: &'static str
    = "PinRulesLastSyncTime";
pub const CERT_PIN_RULES_AUTO_UPDATE_ENCODED_CTL_VALUE_NAME: &'static str = "PinRulesEncodedCt";
pub const CERT_PIN_RULES_CTL_FILENAME: &'static str = "pinrules.st";
pub const CERT_PIN_RULES_CTL_FILENAME_A: &'static str = "pinrules.st";
pub const CERT_PIN_RULES_CAB_FILENAME: &'static str = "pinrulesstl.cab";
pub const CERT_PIN_RULES_AUTO_UPDATE_LIST_IDENTIFIER: &'static str = "PinRules_AutoUpdate_1";
pub const CERT_REGISTRY_STORE_REMOTE_FLAG: DWORD = 0x10000;
pub const CERT_REGISTRY_STORE_SERIALIZED_FLAG: DWORD = 0x20000;
pub const CERT_REGISTRY_STORE_CLIENT_GPT_FLAG: DWORD = 0x80000000;
pub const CERT_REGISTRY_STORE_LM_GPT_FLAG: DWORD = 0x01000000;
STRUCT!{struct CERT_REGISTRY_STORE_CLIENT_GPT_PARA {
    hKeyBase: HKEY,
    pwszRegPath: LPWSTR,
}}
pub type PCERT_REGISTRY_STORE_CLIENT_GPT_PARA = *mut CERT_REGISTRY_STORE_CLIENT_GPT_PARA;
pub const CERT_REGISTRY_STORE_ROAMING_FLAG: DWORD = 0x40000;
STRUCT!{struct CERT_REGISTRY_STORE_ROAMING_PARA {
    hKeyBase: HKEY,
    pwszStoreDirectory: LPWSTR,
}}
pub type PCERT_REGISTRY_STORE_ROAMING_PARA = *mut CERT_REGISTRY_STORE_ROAMING_PARA;
pub const CERT_REGISTRY_STORE_MY_IE_DIRTY_FLAG: DWORD = 0x80000;
pub const CERT_REGISTRY_STORE_EXTERNAL_FLAG: DWORD = 0x100000;
pub const CERT_IE_DIRTY_FLAGS_REGPATH: &'static str
    = "Software\\Microsoft\\Cryptography\\IEDirtyFlags";
pub const CERT_FILE_STORE_COMMIT_ENABLE_FLAG: DWORD = 0x10000;
pub const CERT_LDAP_STORE_SIGN_FLAG: DWORD = 0x10000;
pub const CERT_LDAP_STORE_AREC_EXCLUSIVE_FLAG: DWORD = 0x20000;
pub const CERT_LDAP_STORE_OPENED_FLAG: DWORD = 0x40000;
STRUCT!{struct CERT_LDAP_STORE_OPENED_PARA {
    pvLdapSessionHandle: *mut c_void,
    pwszLdapUrl: LPCWSTR,
}}
pub type PCERT_LDAP_STORE_OPENED_PARA = *mut CERT_LDAP_STORE_OPENED_PARA;
pub const CERT_LDAP_STORE_UNBIND_FLAG: DWORD = 0x80000;
extern "system" {
    pub fn CertOpenStore(
        lpszStoreProvider: LPCSTR,
        dwEncodingType: DWORD,
        hCryptProv: HCRYPTPROV_LEGACY,
        dwFlags: DWORD,
        pvPara: *const c_void,
    ) -> HCERTSTORE;
}
pub type HCERTSTOREPROV = *mut c_void;
pub const CRYPT_OID_OPEN_STORE_PROV_FUNC: &'static str = "CertDllOpenStoreProv";
STRUCT!{struct CERT_STORE_PROV_INFO {
    cbSize: DWORD,
    cStoreProvFunc: DWORD,
    rgpvStoreProvFunc: *mut *mut c_void,
    hStoreProv: HCERTSTOREPROV,
    dwStoreProvFlags: DWORD,
    hStoreProvFuncAddr2: HCRYPTOIDFUNCADDR,
}}
pub type PCERT_STORE_PROV_INFO = *mut CERT_STORE_PROV_INFO;
FN!{stdcall PFN_CERT_DLL_OPEN_STORE_PROV_FUNC(
    lpszStoreProvider: LPCSTR,
    dwEncodingType: DWORD,
    hCryptProv: HCRYPTPROV_LEGACY,
    dwFlags: DWORD,
    pvPara: *const c_void,
    hCertStore: HCERTSTORE,
    pStoreProvInfo: PCERT_STORE_PROV_INFO,
) -> BOOL}
pub const CERT_STORE_PROV_EXTERNAL_FLAG: DWORD = 0x1;
pub const CERT_STORE_PROV_DELETED_FLAG: DWORD = 0x2;
pub const CERT_STORE_PROV_NO_PERSIST_FLAG: DWORD = 0x4;
pub const CERT_STORE_PROV_SYSTEM_STORE_FLAG: DWORD = 0x8;
pub const CERT_STORE_PROV_LM_SYSTEM_STORE_FLAG: DWORD = 0x10;
pub const CERT_STORE_PROV_GP_SYSTEM_STORE_FLAG: DWORD = 0x20;
pub const CERT_STORE_PROV_SHARED_USER_FLAG: DWORD = 0x40;
pub const CERT_STORE_PROV_CLOSE_FUNC: DWORD = 0;
pub const CERT_STORE_PROV_READ_CERT_FUNC: DWORD = 1;
pub const CERT_STORE_PROV_WRITE_CERT_FUNC: DWORD = 2;
pub const CERT_STORE_PROV_DELETE_CERT_FUNC: DWORD = 3;
pub const CERT_STORE_PROV_SET_CERT_PROPERTY_FUNC: DWORD = 4;
pub const CERT_STORE_PROV_READ_CRL_FUNC: DWORD = 5;
pub const CERT_STORE_PROV_WRITE_CRL_FUNC: DWORD = 6;
pub const CERT_STORE_PROV_DELETE_CRL_FUNC: DWORD = 7;
pub const CERT_STORE_PROV_SET_CRL_PROPERTY_FUNC: DWORD = 8;
pub const CERT_STORE_PROV_READ_CTL_FUNC: DWORD = 9;
pub const CERT_STORE_PROV_WRITE_CTL_FUNC: DWORD = 10;
pub const CERT_STORE_PROV_DELETE_CTL_FUNC: DWORD = 11;
pub const CERT_STORE_PROV_SET_CTL_PROPERTY_FUNC: DWORD = 12;
pub const CERT_STORE_PROV_CONTROL_FUNC: DWORD = 13;
pub const CERT_STORE_PROV_FIND_CERT_FUNC: DWORD = 14;
pub const CERT_STORE_PROV_FREE_FIND_CERT_FUNC: DWORD = 15;
pub const CERT_STORE_PROV_GET_CERT_PROPERTY_FUNC: DWORD = 16;
pub const CERT_STORE_PROV_FIND_CRL_FUNC: DWORD = 17;
pub const CERT_STORE_PROV_FREE_FIND_CRL_FUNC: DWORD = 18;
pub const CERT_STORE_PROV_GET_CRL_PROPERTY_FUNC: DWORD = 19;
pub const CERT_STORE_PROV_FIND_CTL_FUNC: DWORD = 20;
pub const CERT_STORE_PROV_FREE_FIND_CTL_FUNC: DWORD = 21;
pub const CERT_STORE_PROV_GET_CTL_PROPERTY_FUNC: DWORD = 22;
FN!{stdcall PFN_CERT_STORE_PROV_CLOSE(
    hStoreProv: HCERTSTOREPROV,
    dwFlags: DWORD,
) -> ()}
FN!{stdcall PFN_CERT_STORE_PROV_READ_CERT(
    hStoreProv: HCERTSTOREPROV,
    pStoreCertContext: PCCERT_CONTEXT,
    dwFlags: DWORD,
    ppProvCertContext: *mut PCCERT_CONTEXT,
) -> BOOL}
pub const CERT_STORE_PROV_WRITE_ADD_FLAG: DWORD = 0x1;
FN!{stdcall PFN_CERT_STORE_PROV_WRITE_CERT(
    hStoreProv: HCERTSTOREPROV,
    pCertContext: PCCERT_CONTEXT,
    dwFlags: DWORD,
) -> BOOL}
FN!{stdcall PFN_CERT_STORE_PROV_DELETE_CERT(
    hStoreProv: HCERTSTOREPROV,
    pCertContext: PCCERT_CONTEXT,
    dwFlags: DWORD,
) -> BOOL}
FN!{stdcall PFN_CERT_STORE_PROV_SET_CERT_PROPERTY(
    hStoreProv: HCERTSTOREPROV,
    pCertContext: PCCERT_CONTEXT,
    dwPropId: DWORD,
    dwFlags: DWORD,
    pvData: *const c_void,
) -> BOOL}
FN!{stdcall PFN_CERT_STORE_PROV_READ_CRL(
    hStoreProv: HCERTSTOREPROV,
    pStoreCrlContext: PCCRL_CONTEXT,
    dwFlags: DWORD,
    ppProvCrlContext: *mut PCCRL_CONTEXT,
) -> BOOL}
FN!{stdcall PFN_CERT_STORE_PROV_WRITE_CRL(
    hStoreProv: HCERTSTOREPROV,
    pCrlContext: PCCRL_CONTEXT,
    dwFlags: DWORD,
) -> BOOL}
FN!{stdcall PFN_CERT_STORE_PROV_DELETE_CRL(
    hStoreProv: HCERTSTOREPROV,
    pCrlContext: PCCRL_CONTEXT,
    dwFlags: DWORD,
) -> BOOL}
FN!{stdcall PFN_CERT_STORE_PROV_SET_CRL_PROPERTY(
    hStoreProv: HCERTSTOREPROV,
    pCrlContext: PCCRL_CONTEXT,
    dwPropId: DWORD,
    dwFlags: DWORD,
    pvData: *const c_void,
) -> BOOL}
FN!{stdcall PFN_CERT_STORE_PROV_READ_CTL(
    hStoreProv: HCERTSTOREPROV,
    pStoreCtlContext: PCCTL_CONTEXT,
    dwFlags: DWORD,
    ppProvCtlContext: *mut PCCTL_CONTEXT,
) -> BOOL}
FN!{stdcall PFN_CERT_STORE_PROV_WRITE_CTL(
    hStoreProv: HCERTSTOREPROV,
    pCtlContext: PCCTL_CONTEXT,
    dwFlags: DWORD,
) -> BOOL}
FN!{stdcall PFN_CERT_STORE_PROV_DELETE_CTL(
    hStoreProv: HCERTSTOREPROV,
    pCtlContext: PCCTL_CONTEXT,
    dwFlags: DWORD,
) -> BOOL}
FN!{stdcall PFN_CERT_STORE_PROV_SET_CTL_PROPERTY(
    hStoreProv: HCERTSTOREPROV,
    pCtlContext: PCCTL_CONTEXT,
    dwPropId: DWORD,
    dwFlags: DWORD,
    pvData: *const c_void,
) -> BOOL}
FN!{stdcall PFN_CERT_STORE_PROV_CONTROL(
    hStoreProv: HCERTSTOREPROV,
    dwFlags: DWORD,
    dwCtrlType: DWORD,
    pvCtrlPara: *const c_void,
) -> BOOL}
STRUCT!{struct CERT_STORE_PROV_FIND_INFO {
    cbSize: DWORD,
    dwMsgAndCertEncodingType: DWORD,
    dwFindFlags: DWORD,
    dwFindType: DWORD,
    pvFindPara: *const c_void,
}}
pub type PCERT_STORE_PROV_FIND_INFO = *mut CERT_STORE_PROV_FIND_INFO;
pub type CCERT_STORE_PROV_FIND_INFO = CERT_STORE_PROV_FIND_INFO;
pub type PCCERT_STORE_PROV_FIND_INFO = *const CERT_STORE_PROV_FIND_INFO;
FN!{stdcall PFN_CERT_STORE_PROV_FIND_CERT(
    hStoreProv: HCERTSTOREPROV,
    pFindInfo: PCCERT_STORE_PROV_FIND_INFO,
    pPrevCertContext: PCCERT_CONTEXT,
    dwFlags: DWORD,
    ppvStoreProvFindInfo: *mut *mut c_void,
    ppProvCertContext: *mut PCCERT_CONTEXT,
) -> BOOL}
FN!{stdcall PFN_CERT_STORE_PROV_FREE_FIND_CERT(
    hStoreProv: HCERTSTOREPROV,
    pCertContext: PCCERT_CONTEXT,
    pvStoreProvFindInfo: *mut c_void,
    dwFlags: DWORD,
) -> BOOL}
FN!{stdcall PFN_CERT_STORE_PROV_GET_CERT_PROPERTY(
    hStoreProv: HCERTSTOREPROV,
    pCertContext: PCCERT_CONTEXT,
    dwPropId: DWORD,
    dwFlags: DWORD,
    pvData: *mut c_void,
    pcbData: *mut DWORD,
) -> BOOL}
FN!{stdcall PFN_CERT_STORE_PROV_FIND_CRL(
    hStoreProv: HCERTSTOREPROV,
    pFindInfo: PCCERT_STORE_PROV_FIND_INFO,
    pPrevCrlContext: PCCRL_CONTEXT,
    dwFlags: DWORD,
    ppvStoreProvFindInfo: *mut *mut c_void,
    ppProvCrlContext: *mut PCCRL_CONTEXT,
) -> BOOL}
FN!{stdcall PFN_CERT_STORE_PROV_FREE_FIND_CRL(
    hStoreProv: HCERTSTOREPROV,
    pCrlContext: PCCRL_CONTEXT,
    pvStoreProvFindInfo: *mut c_void,
    dwFlags: DWORD,
) -> BOOL}
FN!{stdcall PFN_CERT_STORE_PROV_GET_CRL_PROPERTY(
    hStoreProv: HCERTSTOREPROV,
    pCrlContext: PCCRL_CONTEXT,
    dwPropId: DWORD,
    dwFlags: DWORD,
    pvData: *mut c_void,
    pcbData: *mut DWORD,
) -> BOOL}
FN!{stdcall PFN_CERT_STORE_PROV_FIND_CTL(
    hStoreProv: HCERTSTOREPROV,
    pFindInfo: PCCERT_STORE_PROV_FIND_INFO,
    pPrevCtlContext: PCCTL_CONTEXT,
    dwFlags: DWORD,
    ppvStoreProvFindInfo: *mut *mut c_void,
    ppProvCtlContext: *mut PCCTL_CONTEXT,
) -> BOOL}
FN!{stdcall PFN_CERT_STORE_PROV_FREE_FIND_CTL(
    hStoreProv: HCERTSTOREPROV,
    pCtlContext: PCCTL_CONTEXT,
    pvStoreProvFindInfo: *mut c_void,
    dwFlags: DWORD,
) -> BOOL}
FN!{stdcall PFN_CERT_STORE_PROV_GET_CTL_PROPERTY(
    hStoreProv: HCERTSTOREPROV,
    pCtlContext: PCCTL_CONTEXT,
    dwPropId: DWORD,
    dwFlags: DWORD,
    pvData: *mut c_void,
    pcbData: *mut DWORD,
) -> BOOL}
extern "system" {
    pub fn CertDuplicateStore(
        hCertStore: HCERTSTORE,
    ) -> HCERTSTORE;
}
pub const CERT_STORE_SAVE_AS_STORE: DWORD = 1;
pub const CERT_STORE_SAVE_AS_PKCS7: DWORD = 2;
pub const CERT_STORE_SAVE_AS_PKCS12: DWORD = 3;
pub const CERT_STORE_SAVE_TO_FILE: DWORD = 1;
pub const CERT_STORE_SAVE_TO_MEMORY: DWORD = 2;
pub const CERT_STORE_SAVE_TO_FILENAME_A: DWORD = 3;
pub const CERT_STORE_SAVE_TO_FILENAME_W: DWORD = 4;
pub const CERT_STORE_SAVE_TO_FILENAME: DWORD = CERT_STORE_SAVE_TO_FILENAME_W;
extern "system" {
    pub fn CertSaveStore(
        hCertStore: HCERTSTORE,
        dwEncodingType: DWORD,
        dwSaveAs: DWORD,
        dwSaveTo: DWORD,
        pvSaveToPara: *mut c_void,
        dwFlags: DWORD,
    ) -> BOOL;
}
pub const CERT_CLOSE_STORE_FORCE_FLAG: DWORD = 0x00000001;
pub const CERT_CLOSE_STORE_CHECK_FLAG: DWORD = 0x00000002;
extern "system" {
    pub fn CertCloseStore(
        hCertStore: HCERTSTORE,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn CertGetSubjectCertificateFromStore(
        hCertStore: HCERTSTORE,
        dwCertEncodingType: DWORD,
        pCertId: PCERT_INFO,
    ) -> PCCERT_CONTEXT;
    pub fn CertEnumCertificatesInStore(
        hCertStore: HCERTSTORE,
        pPrevCertContext: PCCERT_CONTEXT,
    ) -> PCCERT_CONTEXT;
    pub fn CertFindCertificateInStore(
        hCertStore: HCERTSTORE,
        dwCertEncodingType: DWORD,
        dwFindFlags: DWORD,
        dwFindType: DWORD,
        pvFindPara: *const c_void,
        pPrevCertContext: PCCERT_CONTEXT,
    ) -> PCCERT_CONTEXT;
}
pub const CERT_COMPARE_MASK: DWORD = 0xFFFF;
pub const CERT_COMPARE_SHIFT: DWORD = 16;
pub const CERT_COMPARE_ANY: DWORD = 0;
pub const CERT_COMPARE_SHA1_HASH: DWORD = 1;
pub const CERT_COMPARE_NAME: DWORD = 2;
pub const CERT_COMPARE_ATTR: DWORD = 3;
pub const CERT_COMPARE_MD5_HASH: DWORD = 4;
pub const CERT_COMPARE_PROPERTY: DWORD = 5;
pub const CERT_COMPARE_PUBLIC_KEY: DWORD = 6;
pub const CERT_COMPARE_HASH: DWORD = CERT_COMPARE_SHA1_HASH;
pub const CERT_COMPARE_NAME_STR_A: DWORD = 7;
pub const CERT_COMPARE_NAME_STR_W: DWORD = 8;
pub const CERT_COMPARE_KEY_SPEC: DWORD = 9;
pub const CERT_COMPARE_ENHKEY_USAGE: DWORD = 10;
pub const CERT_COMPARE_CTL_USAGE: DWORD = CERT_COMPARE_ENHKEY_USAGE;
pub const CERT_COMPARE_SUBJECT_CERT: DWORD = 11;
pub const CERT_COMPARE_ISSUER_OF: DWORD = 12;
pub const CERT_COMPARE_EXISTING: DWORD = 13;
pub const CERT_COMPARE_SIGNATURE_HASH: DWORD = 14;
pub const CERT_COMPARE_KEY_IDENTIFIER: DWORD = 15;
pub const CERT_COMPARE_CERT_ID: DWORD = 16;
pub const CERT_COMPARE_CROSS_CERT_DIST_POINTS: DWORD = 17;
pub const CERT_COMPARE_PUBKEY_MD5_HASH: DWORD = 18;
pub const CERT_COMPARE_SUBJECT_INFO_ACCESS: DWORD = 19;
pub const CERT_COMPARE_HASH_STR: DWORD = 20;
pub const CERT_COMPARE_HAS_PRIVATE_KEY: DWORD = 21;
pub const CERT_FIND_ANY: DWORD = CERT_COMPARE_ANY << CERT_COMPARE_SHIFT;
pub const CERT_FIND_SHA1_HASH: DWORD = CERT_COMPARE_SHA1_HASH << CERT_COMPARE_SHIFT;
pub const CERT_FIND_MD5_HASH: DWORD = CERT_COMPARE_MD5_HASH << CERT_COMPARE_SHIFT;
pub const CERT_FIND_SIGNATURE_HASH: DWORD = CERT_COMPARE_SIGNATURE_HASH << CERT_COMPARE_SHIFT;
pub const CERT_FIND_KEY_IDENTIFIER: DWORD = CERT_COMPARE_KEY_IDENTIFIER << CERT_COMPARE_SHIFT;
pub const CERT_FIND_HASH: DWORD = CERT_FIND_SHA1_HASH;
pub const CERT_FIND_PROPERTY: DWORD = CERT_COMPARE_PROPERTY << CERT_COMPARE_SHIFT;
pub const CERT_FIND_PUBLIC_KEY: DWORD = CERT_COMPARE_PUBLIC_KEY << CERT_COMPARE_SHIFT;
pub const CERT_FIND_SUBJECT_NAME: DWORD = (CERT_COMPARE_NAME << CERT_COMPARE_SHIFT)
    | CERT_INFO_SUBJECT_FLAG;
pub const CERT_FIND_SUBJECT_ATTR: DWORD = (CERT_COMPARE_ATTR << CERT_COMPARE_SHIFT)
    | CERT_INFO_SUBJECT_FLAG;
pub const CERT_FIND_ISSUER_NAME: DWORD = (CERT_COMPARE_NAME << CERT_COMPARE_SHIFT)
    | CERT_INFO_ISSUER_FLAG;
pub const CERT_FIND_ISSUER_ATTR: DWORD = (CERT_COMPARE_ATTR << CERT_COMPARE_SHIFT)
    | CERT_INFO_ISSUER_FLAG;
pub const CERT_FIND_SUBJECT_STR_A: DWORD = (CERT_COMPARE_NAME_STR_A << CERT_COMPARE_SHIFT)
    | CERT_INFO_SUBJECT_FLAG;
pub const CERT_FIND_SUBJECT_STR_W: DWORD = (CERT_COMPARE_NAME_STR_W << CERT_COMPARE_SHIFT)
    | CERT_INFO_SUBJECT_FLAG;
pub const CERT_FIND_SUBJECT_STR: DWORD = CERT_FIND_SUBJECT_STR_W;
pub const CERT_FIND_ISSUER_STR_A: DWORD = (CERT_COMPARE_NAME_STR_A << CERT_COMPARE_SHIFT)
    | CERT_INFO_ISSUER_FLAG;
pub const CERT_FIND_ISSUER_STR_W: DWORD = (CERT_COMPARE_NAME_STR_W << CERT_COMPARE_SHIFT)
    | CERT_INFO_ISSUER_FLAG;
pub const CERT_FIND_ISSUER_STR: DWORD = CERT_FIND_ISSUER_STR_W;
pub const CERT_FIND_KEY_SPEC: DWORD = CERT_COMPARE_KEY_SPEC << CERT_COMPARE_SHIFT;
pub const CERT_FIND_ENHKEY_USAGE: DWORD = CERT_COMPARE_ENHKEY_USAGE << CERT_COMPARE_SHIFT;
pub const CERT_FIND_CTL_USAGE: DWORD = CERT_FIND_ENHKEY_USAGE;
pub const CERT_FIND_SUBJECT_CERT: DWORD = CERT_COMPARE_SUBJECT_CERT << CERT_COMPARE_SHIFT;
pub const CERT_FIND_ISSUER_OF: DWORD = CERT_COMPARE_ISSUER_OF << CERT_COMPARE_SHIFT;
pub const CERT_FIND_EXISTING: DWORD = CERT_COMPARE_EXISTING << CERT_COMPARE_SHIFT;
pub const CERT_FIND_CERT_ID: DWORD = CERT_COMPARE_CERT_ID << CERT_COMPARE_SHIFT;
pub const CERT_FIND_CROSS_CERT_DIST_POINTS: DWORD = CERT_COMPARE_CROSS_CERT_DIST_POINTS
    << CERT_COMPARE_SHIFT;
pub const CERT_FIND_PUBKEY_MD5_HASH: DWORD = CERT_COMPARE_PUBKEY_MD5_HASH << CERT_COMPARE_SHIFT;
pub const CERT_FIND_SUBJECT_INFO_ACCESS: DWORD = CERT_COMPARE_SUBJECT_INFO_ACCESS
    << CERT_COMPARE_SHIFT;
pub const CERT_FIND_HASH_STR: DWORD = CERT_COMPARE_HASH_STR << CERT_COMPARE_SHIFT;
pub const CERT_FIND_HAS_PRIVATE_KEY: DWORD = CERT_COMPARE_HAS_PRIVATE_KEY << CERT_COMPARE_SHIFT;
pub const CERT_FIND_OPTIONAL_ENHKEY_USAGE_FLAG: DWORD = 0x1;
pub const CERT_FIND_EXT_ONLY_ENHKEY_USAGE_FLAG: DWORD = 0x2;
pub const CERT_FIND_PROP_ONLY_ENHKEY_USAGE_FLAG: DWORD = 0x4;
pub const CERT_FIND_NO_ENHKEY_USAGE_FLAG: DWORD = 0x8;
pub const CERT_FIND_OR_ENHKEY_USAGE_FLAG: DWORD = 0x10;
pub const CERT_FIND_VALID_ENHKEY_USAGE_FLAG: DWORD = 0x20;
pub const CERT_FIND_OPTIONAL_CTL_USAGE_FLAG: DWORD = CERT_FIND_OPTIONAL_ENHKEY_USAGE_FLAG;
pub const CERT_FIND_EXT_ONLY_CTL_USAGE_FLAG: DWORD = CERT_FIND_EXT_ONLY_ENHKEY_USAGE_FLAG;
pub const CERT_FIND_PROP_ONLY_CTL_USAGE_FLAG: DWORD = CERT_FIND_PROP_ONLY_ENHKEY_USAGE_FLAG;
pub const CERT_FIND_NO_CTL_USAGE_FLAG: DWORD = CERT_FIND_NO_ENHKEY_USAGE_FLAG;
pub const CERT_FIND_OR_CTL_USAGE_FLAG: DWORD = CERT_FIND_OR_ENHKEY_USAGE_FLAG;
pub const CERT_FIND_VALID_CTL_USAGE_FLAG: DWORD = CERT_FIND_VALID_ENHKEY_USAGE_FLAG;
extern "system" {
    pub fn CertGetIssuerCertificateFromStore(
        hCertStore: HCERTSTORE,
        pSubjectContext: PCCERT_CONTEXT,
        pPrevIssuerContext: PCCERT_CONTEXT,
        pdwFlags: *mut DWORD,
    ) -> PCCERT_CONTEXT;
    pub fn CertVerifySubjectCertificateContext(
        pSubject: PCCERT_CONTEXT,
        pIssuer: PCCERT_CONTEXT,
        pdwFlags: *mut DWORD,
    ) -> BOOL;
    pub fn CertDuplicateCertificateContext(
        pCertContext: PCCERT_CONTEXT,
    ) -> PCCERT_CONTEXT;
    pub fn CertCreateCertificateContext(
        dwCertEncodingType: DWORD,
        pbCertEncoded: *const BYTE,
        cbCertEncoded: DWORD,
    ) -> PCCERT_CONTEXT;
    pub fn CertFreeCertificateContext(
        pCertContext: PCCERT_CONTEXT,
    ) -> BOOL;
    pub fn CertSetCertificateContextProperty(
        pCertContext: PCCERT_CONTEXT,
        dwPropId: DWORD,
        dwFlags: DWORD,
        pvData: *const c_void,
    ) -> BOOL;
}
pub const CERT_SET_PROPERTY_IGNORE_PERSIST_ERROR_FLAG: DWORD = 0x80000000;
pub const CERT_SET_PROPERTY_INHIBIT_PERSIST_FLAG: DWORD = 0x40000000;
extern "system" {
    pub fn CertGetCertificateContextProperty(
        pCertContext: PCCERT_CONTEXT,
        dwPropId: DWORD,
        pvData: *mut c_void,
        pcbData: *mut DWORD,
    ) -> BOOL;
    pub fn CertEnumCertificateContextProperties(
        pCertContext: PCCERT_CONTEXT,
        dwPropId: DWORD,
    ) -> DWORD;
    pub fn CertCreateCTLEntryFromCertificateContextProperties(
        pCertContext: PCCERT_CONTEXT,
        cOptAttr: DWORD,
        rgOptAttr: PCRYPT_ATTRIBUTE,
        dwFlags: DWORD,
        pvReserved: *mut c_void,
        pCtlEntry: PCTL_ENTRY,
        pcbCtlEntry: *mut DWORD,
    ) -> BOOL;
    pub fn CertSetCertificateContextPropertiesFromCTLEntry(
        pCertContext: PCCERT_CONTEXT,
        pCtlEntry: PCTL_ENTRY,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn CertGetCRLFromStore(
        hCertStore: HCERTSTORE,
        pIssuerContext: PCCERT_CONTEXT,
        pPrevCrlContext: PCCRL_CONTEXT,
        pdwFlags: *mut DWORD,
    ) -> PCCRL_CONTEXT;
    pub fn CertEnumCRLsInStore(
        hCertStore: HCERTSTORE,
        pPrevCrlContext: PCCRL_CONTEXT,
    ) -> PCCRL_CONTEXT;
    pub fn CertFindCRLInStore(
        hCertStore: HCERTSTORE,
        dwCertEncodingType: DWORD,
        dwFindFlags: DWORD,
        dwFindType: DWORD,
        pvFindPara: *const c_void,
        pPrevCrlContext: PCCRL_CONTEXT,
    ) -> PCCRL_CONTEXT;
}
pub const CRL_FIND_ANY: DWORD = 0;
pub const CRL_FIND_ISSUED_BY: DWORD = 1;
pub const CRL_FIND_EXISTING: DWORD = 2;
pub const CRL_FIND_ISSUED_FOR: DWORD = 3;
pub const CRL_FIND_ISSUED_BY_AKI_FLAG: DWORD = 0x1;
pub const CRL_FIND_ISSUED_BY_SIGNATURE_FLAG: DWORD = 0x2;
pub const CRL_FIND_ISSUED_BY_DELTA_FLAG: DWORD = 0x4;
pub const CRL_FIND_ISSUED_BY_BASE_FLAG: DWORD = 0x8;
STRUCT!{struct CRL_FIND_ISSUED_FOR_PARA {
    pSubjectCert: PCCERT_CONTEXT,
    pIssuerCert: PCCERT_CONTEXT,
}}
pub type PCRL_FIND_ISSUED_FOR_PARA = *mut CRL_FIND_ISSUED_FOR_PARA;
pub const CRL_FIND_ISSUED_FOR_SET_STRONG_PROPERTIES_FLAG: DWORD = 0x10;
extern "system" {
    pub fn CertDuplicateCRLContext(
        pCrlContext: PCCRL_CONTEXT,
    ) -> PCCRL_CONTEXT;
    pub fn CertCreateCRLContext(
        dwCertEncodingType: DWORD,
        pbCrlEncoded: *const BYTE,
        cbCrlEncoded: DWORD,
    ) -> PCCRL_CONTEXT;
    pub fn CertFreeCRLContext(
        pCrlContext: PCCRL_CONTEXT,
    ) -> BOOL;
    pub fn CertSetCRLContextProperty(
        pCrlContext: PCCRL_CONTEXT,
        dwPropId: DWORD,
        dwFlags: DWORD,
        pvData: *const c_void,
    ) -> BOOL;
    pub fn CertGetCRLContextProperty(
        pCrlContext: PCCRL_CONTEXT,
        dwPropId: DWORD,
        pvData: *mut c_void,
        pcbData: *mut DWORD,
    ) -> BOOL;
    pub fn CertEnumCRLContextProperties(
        pCrlContext: PCCRL_CONTEXT,
        dwPropId: DWORD,
    ) -> DWORD;
    pub fn CertFindCertificateInCRL(
        pCert: PCCERT_CONTEXT,
        pCrlContext: PCCRL_CONTEXT,
        dwFlags: DWORD,
        pvReserved: *mut c_void,
        ppCrlEntry: *mut PCRL_ENTRY,
    ) -> BOOL;
    pub fn CertIsValidCRLForCertificate(
        pCert: PCCERT_CONTEXT,
        pCrl: PCCRL_CONTEXT,
        dwFlags: DWORD,
        pvReserved: *mut c_void,
    ) -> BOOL;
}
pub const CERT_STORE_ADD_NEW: DWORD = 1;
pub const CERT_STORE_ADD_USE_EXISTING: DWORD = 2;
pub const CERT_STORE_ADD_REPLACE_EXISTING: DWORD = 3;
pub const CERT_STORE_ADD_ALWAYS: DWORD = 4;
pub const CERT_STORE_ADD_REPLACE_EXISTING_INHERIT_PROPERTIES: DWORD = 5;
pub const CERT_STORE_ADD_NEWER: DWORD = 6;
pub const CERT_STORE_ADD_NEWER_INHERIT_PROPERTIES: DWORD = 7;
extern "system" {
    pub fn CertAddEncodedCertificateToStore(
        hCertStore: HCERTSTORE,
        dwCertEncodingType: DWORD,
        pbCertEncoded: *const BYTE,
        cbCertEncoded: DWORD,
        dwAddDisposition: DWORD,
        ppCertContext: *mut PCCERT_CONTEXT,
    ) -> BOOL;
    pub fn CertAddCertificateContextToStore(
        hCertStore: HCERTSTORE,
        pCertContext: PCCERT_CONTEXT,
        dwAddDisposition: DWORD,
        ppStoreContext: *mut PCCERT_CONTEXT,
    ) -> BOOL;
}
pub const CERT_STORE_CERTIFICATE_CONTEXT: DWORD = 1;
pub const CERT_STORE_CRL_CONTEXT: DWORD = 2;
pub const CERT_STORE_CTL_CONTEXT: DWORD = 3;
pub const CERT_STORE_ALL_CONTEXT_FLAG: DWORD = !0;
pub const CERT_STORE_CERTIFICATE_CONTEXT_FLAG: DWORD = 1 << CERT_STORE_CERTIFICATE_CONTEXT;
pub const CERT_STORE_CRL_CONTEXT_FLAG: DWORD = 1 << CERT_STORE_CRL_CONTEXT;
pub const CERT_STORE_CTL_CONTEXT_FLAG: DWORD = 1 << CERT_STORE_CTL_CONTEXT;
extern "system" {
    pub fn CertAddSerializedElementToStore(
        hCertStore: HCERTSTORE,
        pbElement: *const BYTE,
        cbElement: DWORD,
        dwAddDisposition: DWORD,
        dwFlags: DWORD,
        dwContextTypeFlags: DWORD,
        pdwContextType: *mut DWORD,
        ppvContext: *mut *const c_void,
    ) -> BOOL;
    pub fn CertDeleteCertificateFromStore(
        pCertContext: PCCERT_CONTEXT,
    ) -> BOOL;
    pub fn CertAddEncodedCRLToStore(
        hCertStore: HCERTSTORE,
        dwCertEncodingType: DWORD,
        pbCrlEncoded: *const BYTE,
        cbCrlEncoded: DWORD,
        dwAddDisposition: DWORD,
        ppCrlContext: *mut PCCRL_CONTEXT,
    ) -> BOOL;
    pub fn CertAddCRLContextToStore(
        hCertStore: HCERTSTORE,
        pCrlContext: PCCRL_CONTEXT,
        dwAddDisposition: DWORD,
        ppStoreContext: *mut PCCRL_CONTEXT,
    ) -> BOOL;
    pub fn CertDeleteCRLFromStore(
        pCrlContext: PCCRL_CONTEXT,
    ) -> BOOL;
    pub fn CertSerializeCertificateStoreElement(
        pCertContext: PCCERT_CONTEXT,
        dwFlags: DWORD,
        pbElement: *mut BYTE,
        pcbElement: *mut DWORD,
    ) -> BOOL;
    pub fn CertSerializeCRLStoreElement(
        pCrlContext: PCCRL_CONTEXT,
        dwFlags: DWORD,
        pbElement: *mut BYTE,
        pcbElement: *mut DWORD,
    ) -> BOOL;
    pub fn CertDuplicateCTLContext(
        pCtlContext: PCCTL_CONTEXT,
    ) -> PCCTL_CONTEXT;
    pub fn CertCreateCTLContext(
        dwMsgAndCertEncodingType: DWORD,
        pbCtlEncoded: *const BYTE,
        cbCtlEncoded: DWORD,
    ) -> PCCTL_CONTEXT;
    pub fn CertFreeCTLContext(
        pCtlContext: PCCTL_CONTEXT,
    ) -> BOOL;
    pub fn CertSetCTLContextProperty(
        pCtlContext: PCCTL_CONTEXT,
        dwPropId: DWORD,
        dwFlags: DWORD,
        pvData: *const c_void,
    ) -> BOOL;
    pub fn CertGetCTLContextProperty(
        pCtlContext: PCCTL_CONTEXT,
        dwPropId: DWORD,
        pvData: *mut c_void,
        pcbData: *mut DWORD,
    ) -> BOOL;
    pub fn CertEnumCTLContextProperties(
        pCtlContext: PCCTL_CONTEXT,
        dwPropId: DWORD,
    ) -> DWORD;
    pub fn CertEnumCTLsInStore(
        hCertStore: HCERTSTORE,
        pPrevCtlContext: PCCTL_CONTEXT,
    ) -> PCCTL_CONTEXT;
    pub fn CertFindSubjectInCTL(
        dwEncodingType: DWORD,
        dwSubjectType: DWORD,
        pvSubject: *mut c_void,
        pCtlContext: PCCTL_CONTEXT,
        dwFlags: DWORD,
    ) -> PCTL_ENTRY;
}
pub const CTL_ANY_SUBJECT_TYPE: DWORD = 1;
pub const CTL_CERT_SUBJECT_TYPE: DWORD = 2;
STRUCT!{struct CTL_ANY_SUBJECT_INFO {
    SubjectAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    SubjectIdentifier: CRYPT_DATA_BLOB,
}}
pub type PCTL_ANY_SUBJECT_INFO = *mut CTL_ANY_SUBJECT_INFO;
extern "system" {
    pub fn CertFindCTLInStore(
        hCertStore: HCERTSTORE,
        dwMsgAndCertEncodingType: DWORD,
        dwFindFlags: DWORD,
        dwFindType: DWORD,
        pvFindPara: *const c_void,
        pPrevCtlContext: PCCTL_CONTEXT,
    ) -> PCCTL_CONTEXT;
}
pub const CTL_FIND_ANY: DWORD = 0;
pub const CTL_FIND_SHA1_HASH: DWORD = 1;
pub const CTL_FIND_MD5_HASH: DWORD = 2;
pub const CTL_FIND_USAGE: DWORD = 3;
pub const CTL_FIND_SUBJECT: DWORD = 4;
pub const CTL_FIND_EXISTING: DWORD = 5;
STRUCT!{struct CTL_FIND_USAGE_PARA {
    cbSize: DWORD,
    SubjectUsage: CTL_USAGE,
    ListIdentifier: CRYPT_DATA_BLOB,
    pSigner: PCERT_INFO,
}}
pub type PCTL_FIND_USAGE_PARA = *mut CTL_FIND_USAGE_PARA;
pub const CTL_FIND_NO_LIST_ID_CBDATA: DWORD = 0xFFFFFFFF;
pub const CTL_FIND_NO_SIGNER_PTR: PCERT_INFO = -1isize as PCERT_INFO;
pub const CTL_FIND_SAME_USAGE_FLAG: DWORD = 0x1;
STRUCT!{struct CTL_FIND_SUBJECT_PARA {
    cbSize: DWORD,
    pUsagePara: PCTL_FIND_USAGE_PARA,
    dwSubjectType: DWORD,
    pvSubject: *mut c_void,
}}
pub type PCTL_FIND_SUBJECT_PARA = *mut CTL_FIND_SUBJECT_PARA;
extern "system" {
    pub fn CertAddEncodedCTLToStore(
        hCertStore: HCERTSTORE,
        dwMsgAndCertEncodingType: DWORD,
        pbCtlEncoded: *const BYTE,
        cbCtlEncoded: DWORD,
        dwAddDisposition: DWORD,
        ppCtlContext: *mut PCCTL_CONTEXT,
    ) -> BOOL;
    pub fn CertAddCTLContextToStore(
        hCertStore: HCERTSTORE,
        pCtlContext: PCCTL_CONTEXT,
        dwAddDisposition: DWORD,
        ppStoreContext: *mut PCCTL_CONTEXT,
    ) -> BOOL;
    pub fn CertSerializeCTLStoreElement(
        pCtlContext: PCCTL_CONTEXT,
        dwFlags: DWORD,
        pbElement: *mut BYTE,
        pcbElement: *mut DWORD,
    ) -> BOOL;
    pub fn CertDeleteCTLFromStore(
        pCtlContext: PCCTL_CONTEXT,
    ) -> BOOL;
    pub fn CertAddCertificateLinkToStore(
        hCertStore: HCERTSTORE,
        pCertContext: PCCERT_CONTEXT,
        dwAddDisposition: DWORD,
        ppStoreContext: *mut PCCERT_CONTEXT,
    ) -> BOOL;
    pub fn CertAddCRLLinkToStore(
        hCertStore: HCERTSTORE,
        pCrlContext: PCCRL_CONTEXT,
        dwAddDisposition: DWORD,
        ppStoreContext: *mut PCCRL_CONTEXT,
    ) -> BOOL;
    pub fn CertAddCTLLinkToStore(
        hCertStore: HCERTSTORE,
        pCtlContext: PCCTL_CONTEXT,
        dwAddDisposition: DWORD,
        ppStoreContext: *mut PCCTL_CONTEXT,
    ) -> BOOL;
    pub fn CertAddStoreToCollection(
        hCollectionStore: HCERTSTORE,
        hSiblingStore: HCERTSTORE,
        dwUpdateFlags: DWORD,
        dwPriority: DWORD,
    ) -> BOOL;
    pub fn CertRemoveStoreFromCollection(
        hCollectionStore: HCERTSTORE,
        hSiblingStore: HCERTSTORE,
    );
    pub fn CertControlStore(
        hCertStore: HCERTSTORE,
        dwFlags: DWORD,
        dwCtrlType: DWORD,
        pvCtrlPara: *const c_void,
    ) -> BOOL;
}
pub const CERT_STORE_CTRL_RESYNC: DWORD = 1;
pub const CERT_STORE_CTRL_NOTIFY_CHANGE: DWORD = 2;
pub const CERT_STORE_CTRL_COMMIT: DWORD = 3;
pub const CERT_STORE_CTRL_AUTO_RESYNC: DWORD = 4;
pub const CERT_STORE_CTRL_CANCEL_NOTIFY: DWORD = 5;
pub const CERT_STORE_CTRL_INHIBIT_DUPLICATE_HANDLE_FLAG: DWORD = 0x1;
pub const CERT_STORE_CTRL_COMMIT_FORCE_FLAG: DWORD = 0x1;
pub const CERT_STORE_CTRL_COMMIT_CLEAR_FLAG: DWORD = 0x2;
pub const CERT_STORE_LOCALIZED_NAME_PROP_ID: DWORD = 0x1000;
extern "system" {
    pub fn CertSetStoreProperty(
        hCertStore: HCERTSTORE,
        dwPropId: DWORD,
        dwFlags: DWORD,
        pvData: *const c_void,
    ) -> BOOL;
    pub fn CertGetStoreProperty(
        hCertStore: HCERTSTORE,
        dwPropId: DWORD,
        pvData: *mut c_void,
        pcbData: *mut DWORD,
    ) -> BOOL;
}
FN!{stdcall PFN_CERT_CREATE_CONTEXT_SORT_FUNC(
    cbTotalEncoded: DWORD,
    cbRemainEncoded: DWORD,
    cEntry: DWORD,
    pvSort: *mut c_void,
) -> BOOL}
STRUCT!{struct CERT_CREATE_CONTEXT_PARA {
    cbSize: DWORD,
    pfnFree: PFN_CRYPT_FREE,
    pvFree: *mut c_void,
    pfnSort: PFN_CERT_CREATE_CONTEXT_SORT_FUNC,
    pvSort: *mut c_void,
}}
pub type PCERT_CREATE_CONTEXT_PARA = *mut CERT_CREATE_CONTEXT_PARA;
extern "system" {
    pub fn CertCreateContext(
        dwContextType: DWORD,
        dwEncodingType: DWORD,
        pbEncoded: *const BYTE,
        cbEncoded: DWORD,
        dwFlags: DWORD,
        pCreatePara: PCERT_CREATE_CONTEXT_PARA,
    ) -> *const c_void;
}
pub const CERT_CREATE_CONTEXT_NOCOPY_FLAG: DWORD = 0x1;
pub const CERT_CREATE_CONTEXT_SORTED_FLAG: DWORD = 0x2;
pub const CERT_CREATE_CONTEXT_NO_HCRYPTMSG_FLAG: DWORD = 0x4;
pub const CERT_CREATE_CONTEXT_NO_ENTRY_FLAG: DWORD = 0x8;
STRUCT!{struct CERT_SYSTEM_STORE_INFO {
    cbSize: DWORD,
}}
pub type PCERT_SYSTEM_STORE_INFO = *mut CERT_SYSTEM_STORE_INFO;
STRUCT!{struct CERT_PHYSICAL_STORE_INFO {
    cbSize: DWORD,
    pszOpenStoreProvider: LPSTR,
    dwOpenEncodingType: DWORD,
    dwOpenFlags: DWORD,
    OpenParameters: CRYPT_DATA_BLOB,
    dwFlags: DWORD,
    dwPriority: DWORD,
}}
pub type PCERT_PHYSICAL_STORE_INFO = *mut CERT_PHYSICAL_STORE_INFO;
pub const CERT_PHYSICAL_STORE_ADD_ENABLE_FLAG: DWORD = 0x1;
pub const CERT_PHYSICAL_STORE_OPEN_DISABLE_FLAG: DWORD = 0x2;
pub const CERT_PHYSICAL_STORE_REMOTE_OPEN_DISABLE_FLAG: DWORD = 0x4;
pub const CERT_PHYSICAL_STORE_INSERT_COMPUTER_NAME_ENABLE_FLAG: DWORD = 0x8;
extern "system" {
    pub fn CertRegisterSystemStore(
        pvSystemStore: *const c_void,
        dwFlags: DWORD,
        pStoreInfo: PCERT_SYSTEM_STORE_INFO,
        pvReserved: *mut c_void,
    ) -> BOOL;
    pub fn CertRegisterPhysicalStore(
        pvSystemStore: *const c_void,
        dwFlags: DWORD,
        pwszStoreName: LPCWSTR,
        pStoreInfo: PCERT_PHYSICAL_STORE_INFO,
        pvReserved: *mut c_void,
    ) -> BOOL;
    pub fn CertUnregisterSystemStore(
        pvSystemStore: *const c_void,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn CertUnregisterPhysicalStore(
        pvSystemStore: *const c_void,
        dwFlags: DWORD,
        pwszStoreName: LPCWSTR,
    ) -> BOOL;
}
FN!{stdcall PFN_CERT_ENUM_SYSTEM_STORE_LOCATION(
    pwszStoreLocation: LPCWSTR,
    dwFlags: DWORD,
    pvReserved: *mut c_void,
    pvArg: *mut c_void,
) -> BOOL}
FN!{stdcall PFN_CERT_ENUM_SYSTEM_STORE(
    pvSystemStore: *const c_void,
    dwFlags: DWORD,
    pStoreInfo: PCERT_SYSTEM_STORE_INFO,
    pvReserved: *mut c_void,
    pvArg: *mut c_void,
) -> BOOL}
FN!{stdcall PFN_CERT_ENUM_PHYSICAL_STORE(
    pvSystemStore: *const c_void,
    dwFlags: DWORD,
    pwszStoreName: LPCWSTR,
    pStoreInfo: PCERT_PHYSICAL_STORE_INFO,
    pvReserved: *mut c_void,
    pvArg: *mut c_void,
) -> BOOL}
pub const CERT_PHYSICAL_STORE_PREDEFINED_ENUM_FLAG: DWORD = 0x1;
pub const CERT_PHYSICAL_STORE_DEFAULT_NAME: &'static str = ".Default";
pub const CERT_PHYSICAL_STORE_GROUP_POLICY_NAME: &'static str = ".GroupPolicy";
pub const CERT_PHYSICAL_STORE_LOCAL_MACHINE_NAME: &'static str = ".LocalMachine";
pub const CERT_PHYSICAL_STORE_DS_USER_CERTIFICATE_NAME: &'static str = ".UserCertificate";
pub const CERT_PHYSICAL_STORE_LOCAL_MACHINE_GROUP_POLICY_NAME: &'static str
    = ".LocalMachineGroupPolicy";
pub const CERT_PHYSICAL_STORE_ENTERPRISE_NAME: &'static str = ".Enterprise";
pub const CERT_PHYSICAL_STORE_AUTH_ROOT_NAME: &'static str = ".AuthRoot";
pub const CERT_PHYSICAL_STORE_SMART_CARD_NAME: &'static str = ".SmartCard";
extern "system" {
    pub fn CertEnumSystemStoreLocation(
        dwFlags: DWORD,
        pvArg: *mut c_void,
        pfnEnum: PFN_CERT_ENUM_SYSTEM_STORE_LOCATION,
    ) -> BOOL;
    pub fn CertEnumSystemStore(
        dwFlags: DWORD,
        pvSystemStoreLocationPara: *mut c_void,
        pvArg: *mut c_void,
        pfnEnum: PFN_CERT_ENUM_SYSTEM_STORE,
    ) -> BOOL;
    pub fn CertEnumPhysicalStore(
        pvSystemStore: *const c_void,
        dwFlags: DWORD,
        pvArg: *mut c_void,
        pfnEnum: PFN_CERT_ENUM_PHYSICAL_STORE,
    ) -> BOOL;
}
pub const CRYPT_OID_OPEN_SYSTEM_STORE_PROV_FUNC: &'static str = "CertDllOpenSystemStoreProv";
pub const CRYPT_OID_REGISTER_SYSTEM_STORE_FUNC: &'static str = "CertDllRegisterSystemStore";
pub const CRYPT_OID_UNREGISTER_SYSTEM_STORE_FUNC: &'static str = "CertDllUnregisterSystemStore";
pub const CRYPT_OID_ENUM_SYSTEM_STORE_FUNC: &'static str = "CertDllEnumSystemStore";
pub const CRYPT_OID_REGISTER_PHYSICAL_STORE_FUNC: &'static str = "CertDllRegisterPhysicalStore";
pub const CRYPT_OID_UNREGISTER_PHYSICAL_STORE_FUNC: &'static str
    = "CertDllUnregisterPhysicalStore";
pub const CRYPT_OID_ENUM_PHYSICAL_STORE_FUNC: &'static str = "CertDllEnumPhysicalStore";
pub const CRYPT_OID_SYSTEM_STORE_LOCATION_VALUE_NAME: &'static str = "SystemStoreLocation";
extern "system" {
    pub fn CertGetEnhancedKeyUsage(
        pCertContext: PCCERT_CONTEXT,
        dwFlags: DWORD,
        pUsage: PCERT_ENHKEY_USAGE,
        pcbUsage: *mut DWORD,
    ) -> BOOL;
    pub fn CertSetEnhancedKeyUsage(
        pCertContext: PCCERT_CONTEXT,
        pUsage: PCERT_ENHKEY_USAGE,
    ) -> BOOL;
    pub fn CertAddEnhancedKeyUsageIdentifier(
        pCertContext: PCCERT_CONTEXT,
        pszUsageIdentifier: LPCSTR,
    ) -> BOOL;
    pub fn CertRemoveEnhancedKeyUsageIdentifier(
        pCertContext: PCCERT_CONTEXT,
        pszUsageIdentifier: LPCSTR,
    ) -> BOOL;
    pub fn CertGetValidUsages(
        cCerts: DWORD,
        rghCerts: *mut PCCERT_CONTEXT,
        cNumOIDs: *mut c_int,
        rghOIDs: *mut LPSTR,
        pcbOIDs: *mut DWORD,
    ) -> BOOL;
    pub fn CryptMsgGetAndVerifySigner(
        hCryptMsg: HCRYPTMSG,
        cSignerStore: DWORD,
        rghSignerStore: *mut HCERTSTORE,
        dwFlags: DWORD,
        ppSigner: *mut PCCERT_CONTEXT,
        pdwSignerIndex: *mut DWORD,
    ) -> BOOL;
}
pub const CMSG_TRUSTED_SIGNER_FLAG: DWORD = 0x1;
pub const CMSG_SIGNER_ONLY_FLAG: DWORD = 0x2;
pub const CMSG_USE_SIGNER_INDEX_FLAG: DWORD = 0x4;
extern "system" {
    pub fn CryptMsgSignCTL(
        dwMsgEncodingType: DWORD,
        pbCtlContent: *mut BYTE,
        cbCtlContent: DWORD,
        pSignInfo: PCMSG_SIGNED_ENCODE_INFO,
        dwFlags: DWORD,
        pbEncoded: *mut BYTE,
        pcbEncoded: *mut DWORD,
    ) -> BOOL;
    pub fn CryptMsgEncodeAndSignCTL(
        dwMsgEncodingType: DWORD,
        pCtlInfo: PCTL_INFO,
        pSignInfo: PCMSG_SIGNED_ENCODE_INFO,
        dwFlags: DWORD,
        pbEncoded: *mut BYTE,
        pcbEncoded: *mut DWORD,
    ) -> BOOL;
}
pub const CMSG_ENCODE_SORTED_CTL_FLAG: DWORD = 0x1;
pub const CMSG_ENCODE_HASHED_SUBJECT_IDENTIFIER_FLAG: DWORD = 0x2;
extern "system" {
    pub fn CertFindSubjectInSortedCTL(
        pSubjectIdentifier: PCRYPT_DATA_BLOB,
        pCtlContext: PCCTL_CONTEXT,
        dwFlags: DWORD,
        pvReserved: *mut c_void,
        pEncodedAttributes: PCRYPT_DER_BLOB,
    ) -> BOOL;
    pub fn CertEnumSubjectInSortedCTL(
        pCtlContext: PCCTL_CONTEXT,
        ppvNextSubject: *mut *mut c_void,
        pSubjectIdentifier: PCRYPT_DER_BLOB,
        pEncodedAttributes: PCRYPT_DER_BLOB,
    ) -> BOOL;
}
STRUCT!{struct CTL_VERIFY_USAGE_PARA {
    cbSize: DWORD,
    ListIdentifier: CRYPT_DATA_BLOB,
    cCtlStore: DWORD,
    rghCtlStore: *mut HCERTSTORE,
    cSignerStore: DWORD,
    rghSignerStore: *mut HCERTSTORE,
}}
pub type PCTL_VERIFY_USAGE_PARA = *mut CTL_VERIFY_USAGE_PARA;
STRUCT!{struct CTL_VERIFY_USAGE_STATUS {
    cbSize: DWORD,
    dwError: DWORD,
    dwFlags: DWORD,
    ppCtl: *mut PCCTL_CONTEXT,
    dwCtlEntryIndex: DWORD,
    ppSigner: *mut PCCERT_CONTEXT,
    dwSignerIndex: DWORD,
}}
pub type PCTL_VERIFY_USAGE_STATUS = *mut CTL_VERIFY_USAGE_STATUS;
pub const CERT_VERIFY_INHIBIT_CTL_UPDATE_FLAG: DWORD = 0x1;
pub const CERT_VERIFY_TRUSTED_SIGNERS_FLAG: DWORD = 0x2;
pub const CERT_VERIFY_NO_TIME_CHECK_FLAG: DWORD = 0x4;
pub const CERT_VERIFY_ALLOW_MORE_USAGE_FLAG: DWORD = 0x8;
pub const CERT_VERIFY_UPDATED_CTL_FLAG: DWORD = 0x1;
extern "system" {
    pub fn CertVerifyCTLUsage(
        dwEncodingType: DWORD,
        dwSubjectType: DWORD,
        pvSubject: *mut c_void,
        pSubjectUsage: PCTL_USAGE,
        dwFlags: DWORD,
        pVerifyUsagePara: PCTL_VERIFY_USAGE_PARA,
        pVerifyUsageStatus: PCTL_VERIFY_USAGE_STATUS,
    ) -> BOOL;
}
STRUCT!{struct CERT_REVOCATION_CRL_INFO {
    cbSize: DWORD,
    pBaseCrlContext: PCCRL_CONTEXT,
    pDeltaCrlContext: PCCRL_CONTEXT,
    pCrlEntry: PCRL_ENTRY,
    fDeltaCrlEntry: BOOL,
}}
pub type PCERT_REVOCATION_CRL_INFO = *mut CERT_REVOCATION_CRL_INFO;
pub type PCERT_REVOCATION_CHAIN_PARA = *mut CERT_REVOCATION_CHAIN_PARA;
STRUCT!{struct CERT_REVOCATION_PARA {
    cbSize: DWORD,
    pIssuerCert: PCCERT_CONTEXT,
    cCertStore: DWORD,
    rgCertStore: *mut HCERTSTORE,
    hCrlStore: HCERTSTORE,
    pftTimeToUse: LPFILETIME,
    dwUrlRetrievalTimeout: DWORD,
    fCheckFreshnessTime: BOOL,
    dwFreshnessTime: DWORD,
    pftCurrentTime: LPFILETIME,
    pCrlInfo: PCERT_REVOCATION_CRL_INFO,
    pftCacheResync: LPFILETIME,
    pChainPara: PCERT_REVOCATION_CHAIN_PARA,
}}
pub type PCERT_REVOCATION_PARA = *mut CERT_REVOCATION_PARA;
STRUCT!{struct CERT_REVOCATION_STATUS {
    cbSize: DWORD,
    dwIndex: DWORD,
    dwError: DWORD,
    dwReason: DWORD,
    fHasFreshnessTime: BOOL,
    dwFreshnessTime: DWORD,
}}
pub type PCERT_REVOCATION_STATUS = *mut CERT_REVOCATION_STATUS;
extern "system" {
    pub fn CertVerifyRevocation(
        dwEncodingType: DWORD,
        dwRevType: DWORD,
        cContext: DWORD,
        rgpvContext: *mut PVOID,
        dwFlags: DWORD,
        pRevPara: PCERT_REVOCATION_PARA,
        pRevStatus: PCERT_REVOCATION_STATUS,
    ) -> BOOL;
}
pub const CERT_CONTEXT_REVOCATION_TYPE: DWORD = 1;
pub const CERT_VERIFY_REV_CHAIN_FLAG: DWORD = 0x00000001;
pub const CERT_VERIFY_CACHE_ONLY_BASED_REVOCATION: DWORD = 0x00000002;
pub const CERT_VERIFY_REV_ACCUMULATIVE_TIMEOUT_FLAG: DWORD = 0x00000004;
pub const CERT_VERIFY_REV_SERVER_OCSP_FLAG: DWORD = 0x00000008;
pub const CERT_VERIFY_REV_NO_OCSP_FAILOVER_TO_CRL_FLAG: DWORD = 0x00000010;
extern "system" {
    pub fn CertCompareIntegerBlob(
        pInt1: PCRYPT_INTEGER_BLOB,
        pInt2: PCRYPT_INTEGER_BLOB,
    ) -> BOOL;
    pub fn CertCompareCertificate(
        dwCertEncodingType: DWORD,
        pCertId1: PCERT_INFO,
        pCertId2: PCERT_INFO,
    ) -> BOOL;
    pub fn CertCompareCertificateName(
        dwCertEncodingType: DWORD,
        pCertName1: PCERT_NAME_BLOB,
        pCertName2: PCERT_NAME_BLOB,
    ) -> BOOL;
    pub fn CertIsRDNAttrsInCertificateName(
        dwCertEncodingType: DWORD,
        dwFlags: DWORD,
        pCertName: PCERT_NAME_BLOB,
        pRDN: PCERT_RDN,
    ) -> BOOL;
    pub fn CertComparePublicKeyInfo(
        dwCertEncodingType: DWORD,
        pPublicKey1: PCERT_PUBLIC_KEY_INFO,
        pPublicKey2: PCERT_PUBLIC_KEY_INFO,
    ) -> BOOL;
    pub fn CertGetPublicKeyLength(
        dwCertEncodingType: DWORD,
        pPublicKey: PCERT_PUBLIC_KEY_INFO,
    ) -> DWORD;
    pub fn CryptVerifyCertificateSignature(
        hCryptProv: HCRYPTPROV_LEGACY,
        dwCertEncodingType: DWORD,
        pbEncoded: *const BYTE,
        cbEncoded: DWORD,
        pPublicKey: PCERT_PUBLIC_KEY_INFO,
    ) -> BOOL;
    pub fn CryptVerifyCertificateSignatureEx(
        hCryptProv: HCRYPTPROV_LEGACY,
        dwCertEncodingType: DWORD,
        dwSubjectType: DWORD,
        pvSubject: *mut c_void,
        dwIssuerType: DWORD,
        pvIssuer: *mut c_void,
        dwFlags: DWORD,
        pvExtra: *mut c_void,
    ) -> BOOL;
}
pub const CRYPT_VERIFY_CERT_SIGN_SUBJECT_BLOB: DWORD = 1;
pub const CRYPT_VERIFY_CERT_SIGN_SUBJECT_CERT: DWORD = 2;
pub const CRYPT_VERIFY_CERT_SIGN_SUBJECT_CRL: DWORD = 3;
pub const CRYPT_VERIFY_CERT_SIGN_SUBJECT_OCSP_BASIC_SIGNED_RESPONSE: DWORD = 4;
pub const CRYPT_VERIFY_CERT_SIGN_ISSUER_PUBKEY: DWORD = 1;
pub const CRYPT_VERIFY_CERT_SIGN_ISSUER_CERT: DWORD = 2;
pub const CRYPT_VERIFY_CERT_SIGN_ISSUER_CHAIN: DWORD = 3;
pub const CRYPT_VERIFY_CERT_SIGN_ISSUER_NULL: DWORD = 4;
pub const CRYPT_VERIFY_CERT_SIGN_DISABLE_MD2_MD4_FLAG: DWORD = 0x00000001;
pub const CRYPT_VERIFY_CERT_SIGN_SET_STRONG_PROPERTIES_FLAG: DWORD = 0x00000002;
pub const CRYPT_VERIFY_CERT_SIGN_RETURN_STRONG_PROPERTIES_FLAG: DWORD = 0x00000004;
STRUCT!{struct CRYPT_VERIFY_CERT_SIGN_STRONG_PROPERTIES_INFO {
    CertSignHashCNGAlgPropData: CRYPT_DATA_BLOB,
    CertIssuerPubKeyBitLengthPropData: CRYPT_DATA_BLOB,
}}
pub type PCRYPT_VERIFY_CERT_SIGN_STRONG_PROPERTIES_INFO
    = *mut CRYPT_VERIFY_CERT_SIGN_STRONG_PROPERTIES_INFO;
STRUCT!{struct CRYPT_VERIFY_CERT_SIGN_WEAK_HASH_INFO {
    cCNGHashAlgid: DWORD,
    rgpwszCNGHashAlgid: *mut PCWSTR,
    dwWeakIndex: DWORD,
}}
pub type PCRYPT_VERIFY_CERT_SIGN_WEAK_HASH_INFO = *mut CRYPT_VERIFY_CERT_SIGN_WEAK_HASH_INFO;
extern "system" {
    pub fn CertIsStrongHashToSign(
        pStrongSignPara: PCCERT_STRONG_SIGN_PARA,
        pwszCNGHashAlgid: LPCWSTR,
        pSigningCert: PCCERT_CONTEXT,
    ) -> BOOL;
    pub fn CryptHashToBeSigned(
        hCryptProv: HCRYPTPROV_LEGACY,
        dwCertEncodingType: DWORD,
        pbEncoded: *const BYTE,
        cbEncoded: DWORD,
        pbComputedHash: *mut BYTE,
        pcbComputedHash: *mut DWORD,
    ) -> BOOL;
    pub fn CryptHashCertificate(
        hCryptProv: HCRYPTPROV_LEGACY,
        Algid: ALG_ID,
        dwFlags: DWORD,
        pbEncoded: *const BYTE,
        cbEncoded: DWORD,
        pbComputedHash: *mut BYTE,
        pcbComputedHash: *mut DWORD,
    ) -> BOOL;
    pub fn CryptHashCertificate2(
        pwszCNGHashAlgid: LPCWSTR,
        dwFlags: DWORD,
        pvReserved: *mut c_void,
        pbEncoded: *const BYTE,
        cbEncoded: DWORD,
        pbComputedHash: *mut BYTE,
        pcbComputedHash: *mut DWORD,
    ) -> BOOL;
    pub fn CryptSignCertificate(
        hCryptProvOrNCryptKey: HCRYPTPROV_OR_NCRYPT_KEY_HANDLE,
        dwKeySpec: DWORD,
        dwCertEncodingType: DWORD,
        pbEncodedToBeSigned: *const BYTE,
        cbEncodedToBeSigned: DWORD,
        pSignatureAlgorithm: PCRYPT_ALGORITHM_IDENTIFIER,
        pvHashAuxInfo: *const c_void,
        pbSignature: *mut BYTE,
        pcbSignature: *mut DWORD,
    ) -> BOOL;
    pub fn CryptSignAndEncodeCertificate(
        hCryptProvOrNCryptKey: HCRYPTPROV_OR_NCRYPT_KEY_HANDLE,
        dwKeySpec: DWORD,
        dwCertEncodingType: DWORD,
        lpszStructType: LPCSTR,
        pvStructInfo: *const c_void,
        pSignatureAlgorithm: PCRYPT_ALGORITHM_IDENTIFIER,
        pvHashAuxInfo: *const c_void,
        pbEncoded: *mut BYTE,
        pcbEncoded: *mut DWORD,
    ) -> BOOL;
}
pub const CRYPT_OID_EXTRACT_ENCODED_SIGNATURE_PARAMETERS_FUNC: &'static str
    = "CryptDllExtractEncodedSignatureParameters";
FN!{stdcall PFN_CRYPT_EXTRACT_ENCODED_SIGNATURE_PARAMETERS_FUNC(
    dwCertEncodingType: DWORD,
    pSignatureAlgorithm: PCRYPT_ALGORITHM_IDENTIFIER,
    ppvDecodedSignPara: *mut *mut c_void,
    ppwszCNGHashAlgid: LPWSTR,
) -> BOOL}
pub const CRYPT_OID_SIGN_AND_ENCODE_HASH_FUNC: &'static str = "CryptDllSignAndEncodeHash";
FN!{stdcall PFN_CRYPT_SIGN_AND_ENCODE_HASH_FUNC(
    hKey: NCRYPT_KEY_HANDLE,
    dwCertEncodingType: DWORD,
    pSignatureAlgorithm: PCRYPT_ALGORITHM_IDENTIFIER,
    pvDecodedSignPara: *mut c_void,
    pwszCNGPubKeyAlgid: LPCWSTR,
    pwszCNGHashAlgid: LPCWSTR,
    pbComputedHash: *mut BYTE,
    cbComputedHash: DWORD,
    pbSignature: *mut BYTE,
    pcbSignature: *mut DWORD,
) -> BOOL}
pub const CRYPT_OID_VERIFY_ENCODED_SIGNATURE_FUNC: &'static str = "CryptDllVerifyEncodedSignature";
FN!{stdcall PFN_CRYPT_VERIFY_ENCODED_SIGNATURE_FUNC(
    dwCertEncodingType: DWORD,
    pPubKeyInfo: PCERT_PUBLIC_KEY_INFO,
    pSignatureAlgorithm: PCRYPT_ALGORITHM_IDENTIFIER,
    pvDecodedSignPara: *mut c_void,
    pwszCNGPubKeyAlgid: LPCWSTR,
    pwszCNGHashAlgid: LPCWSTR,
    pbComputedHash: *mut BYTE,
    cbComputedHash: DWORD,
    pbSignature: *mut BYTE,
    cbSignature: DWORD,
) -> BOOL}
extern "system" {
    pub fn CertVerifyTimeValidity(
        pTimeToVerify: LPFILETIME,
        pCertInfo: PCERT_INFO,
    ) -> LONG;
    pub fn CertVerifyCRLTimeValidity(
        pTimeToVerify: LPFILETIME,
        pCrlInfo: PCRL_INFO,
    ) -> LONG;
    pub fn CertVerifyValidityNesting(
        pSubjectInfo: PCERT_INFO,
        pIssuerInfo: PCERT_INFO,
    ) -> BOOL;
    pub fn CertVerifyCRLRevocation(
        dwCertEncodingType: DWORD,
        pCertId: PCERT_INFO,
        cCrlInfo: DWORD,
        rgpCrlInfo: *mut PCRL_INFO,
    ) -> BOOL;
    pub fn CertAlgIdToOID(
        dwAlgId: DWORD,
    ) -> LPCSTR;
    pub fn CertOIDToAlgId(
        pszObjId: LPCSTR,
    ) -> DWORD;
    pub fn CertFindExtension(
        pszObjId: LPCSTR,
        cExtensions: DWORD,
        rgExtensions: *mut CERT_EXTENSION,
    ) -> PCERT_EXTENSION;
    pub fn CertFindAttribute(
        pszObjId: LPCSTR,
        cAttr: DWORD,
        rgAttr: *mut CRYPT_ATTRIBUTE,
    ) -> PCRYPT_ATTRIBUTE;
    pub fn CertFindRDNAttr(
        pszObjId: LPCSTR,
        pName: PCERT_NAME_INFO,
    ) -> PCERT_RDN_ATTR;
    pub fn CertGetIntendedKeyUsage(
        dwCertEncodingType: DWORD,
        pCertInfo: PCERT_INFO,
        pbKeyUsage: *mut BYTE,
        cbKeyUsage: DWORD,
    ) -> BOOL;
}
pub type HCRYPTDEFAULTCONTEXT = *mut c_void;
extern "system" {
    pub fn CryptInstallDefaultContext(
        hCryptProv: HCRYPTPROV,
        dwDefaultType: DWORD,
        pvDefaultPara: *const c_void,
        dwFlags: DWORD,
        pvReserved: *mut c_void,
        phDefaultContext: *mut HCRYPTDEFAULTCONTEXT,
    ) -> BOOL;
}
pub const CRYPT_DEFAULT_CONTEXT_AUTO_RELEASE_FLAG: DWORD = 0x00000001;
pub const CRYPT_DEFAULT_CONTEXT_PROCESS_FLAG: DWORD = 0x00000002;
pub const CRYPT_DEFAULT_CONTEXT_CERT_SIGN_OID: DWORD = 1;
pub const CRYPT_DEFAULT_CONTEXT_MULTI_CERT_SIGN_OID: DWORD = 2;
STRUCT!{struct CRYPT_DEFAULT_CONTEXT_MULTI_OID_PARA {
    cOID: DWORD,
    rgpszOID: *mut LPSTR,
}}
pub type PCRYPT_DEFAULT_CONTEXT_MULTI_OID_PARA = *mut CRYPT_DEFAULT_CONTEXT_MULTI_OID_PARA;
extern "system" {
    pub fn CryptUninstallDefaultContext(
        hDefaultContext: HCRYPTDEFAULTCONTEXT,
        dwFlags: DWORD,
        pvReserved: *mut c_void,
    ) -> BOOL;
    pub fn CryptExportPublicKeyInfo(
        hCryptProvOrNCryptKey: HCRYPTPROV_OR_NCRYPT_KEY_HANDLE,
        dwKeySpec: DWORD,
        dwCertEncodingType: DWORD,
        pInfo: PCERT_PUBLIC_KEY_INFO,
        pcbInfo: *mut DWORD,
    ) -> BOOL;
    pub fn CryptExportPublicKeyInfoEx(
        hCryptProvOrNCryptKey: HCRYPTPROV_OR_NCRYPT_KEY_HANDLE,
        dwKeySpec: DWORD,
        dwCertEncodingType: DWORD,
        pszPublicKeyObjId: LPSTR,
        dwFlags: DWORD,
        pvAuxInfo: *mut c_void,
        pInfo: PCERT_PUBLIC_KEY_INFO,
        pcbInfo: *mut DWORD,
    ) -> BOOL;
}
pub const CRYPT_OID_EXPORT_PUBLIC_KEY_INFO_FUNC: &'static str = "CryptDllExportPublicKeyInfoEx";
pub const CRYPT_OID_EXPORT_PUBLIC_KEY_INFO_EX2_FUNC: &'static str
    = "CryptDllExportPublicKeyInfoEx2";
FN!{stdcall PFN_CRYPT_EXPORT_PUBLIC_KEY_INFO_EX2_FUNC(
    hNCryptKey: NCRYPT_KEY_HANDLE,
    dwCertEncodingType: DWORD,
    pszPublicKeyObjId: LPSTR,
    dwFlags: DWORD,
    pvAuxInfo: *mut c_void,
    pInfo: PCERT_PUBLIC_KEY_INFO,
    pcbInfo: *mut DWORD,
) -> BOOL}
extern "system" {
    pub fn CryptExportPublicKeyInfoFromBCryptKeyHandle(
        hBCryptKey: BCRYPT_KEY_HANDLE,
        dwCertEncodingType: DWORD,
        pszPublicKeyObjId: LPSTR,
        dwFlags: DWORD,
        pvAuxInfo: *mut c_void,
        pInfo: PCERT_PUBLIC_KEY_INFO,
        pcbInfo: *mut DWORD,
    ) -> BOOL;
}
pub const CRYPT_OID_EXPORT_PUBLIC_KEY_INFO_FROM_BCRYPT_HANDLE_FUNC: &'static str
    = "CryptDllExportPublicKeyInfoFromBCryptKeyHandle";
FN!{stdcall PFN_CRYPT_EXPORT_PUBLIC_KEY_INFO_FROM_BCRYPT_HANDLE_FUNC(
    hBCryptKey: BCRYPT_KEY_HANDLE,
    dwCertEncodingType: DWORD,
    pszPublicKeyObjId: LPSTR,
    dwFlags: DWORD,
    pvAuxInfo: *mut c_void,
    pInfo: PCERT_PUBLIC_KEY_INFO,
    pcbInfo: *mut DWORD,
) -> BOOL}
extern "system" {
    pub fn CryptImportPublicKeyInfo(
        hCryptProv: HCRYPTPROV,
        dwCertEncodingType: DWORD,
        pInfo: PCERT_PUBLIC_KEY_INFO,
        phKey: *mut HCRYPTKEY,
    ) -> BOOL;
}
pub const CRYPT_OID_IMPORT_PUBLIC_KEY_INFO_FUNC: &'static str = "CryptDllImportPublicKeyInfoEx";
extern "system" {
    pub fn CryptImportPublicKeyInfoEx(
        hCryptProv: HCRYPTPROV,
        dwCertEncodingType: DWORD,
        pInfo: PCERT_PUBLIC_KEY_INFO,
        aiKeyAlg: ALG_ID,
        dwFlags: DWORD,
        pvAuxInfo: *mut c_void,
        phKey: *mut HCRYPTKEY,
    ) -> BOOL;
    pub fn CryptImportPublicKeyInfoEx2(
        dwCertEncodingType: DWORD,
        pInfo: PCERT_PUBLIC_KEY_INFO,
        dwFlags: DWORD,
        pvAuxInfo: *mut c_void,
        phKey: *mut BCRYPT_KEY_HANDLE,
    ) -> BOOL;
}
pub const CRYPT_OID_IMPORT_PUBLIC_KEY_INFO_EX2_FUNC: &'static str
    = "CryptDllImportPublicKeyInfoEx2";
FN!{stdcall PFN_IMPORT_PUBLIC_KEY_INFO_EX2_FUNC(
    dwCertEncodingType: DWORD,
    pInfo: PCERT_PUBLIC_KEY_INFO,
    dwFlags: DWORD,
    pvAuxInfo: *mut c_void,
    phKey: *mut BCRYPT_KEY_HANDLE,
) -> BOOL}
extern "system" {
    pub fn CryptAcquireCertificatePrivateKey(
        pCert: PCCERT_CONTEXT,
        dwFlags: DWORD,
        pvParameters: *mut c_void,
        phCryptProvOrNCryptKey: *mut HCRYPTPROV_OR_NCRYPT_KEY_HANDLE,
        pdwKeySpec: *mut DWORD,
        pfCallerFreeProvOrNCryptKey: *mut BOOL,
    ) -> BOOL;
}
pub const CRYPT_ACQUIRE_CACHE_FLAG: DWORD = 0x00000001;
pub const CRYPT_ACQUIRE_USE_PROV_INFO_FLAG: DWORD = 0x00000002;
pub const CRYPT_ACQUIRE_COMPARE_KEY_FLAG: DWORD = 0x00000004;
pub const CRYPT_ACQUIRE_NO_HEALING: DWORD = 0x00000008;
pub const CRYPT_ACQUIRE_SILENT_FLAG: DWORD = 0x00000040;
pub const CRYPT_ACQUIRE_WINDOW_HANDLE_FLAG: DWORD = 0x00000080;
pub const CRYPT_ACQUIRE_NCRYPT_KEY_FLAGS_MASK: DWORD = 0x00070000;
pub const CRYPT_ACQUIRE_ALLOW_NCRYPT_KEY_FLAG: DWORD = 0x00010000;
pub const CRYPT_ACQUIRE_PREFER_NCRYPT_KEY_FLAG: DWORD = 0x00020000;
pub const CRYPT_ACQUIRE_ONLY_NCRYPT_KEY_FLAG: DWORD = 0x00040000;
extern "system" {
    pub fn CryptFindCertificateKeyProvInfo(
        pCert: PCCERT_CONTEXT,
        dwFlags: DWORD,
        pvReserved: *mut c_void,
    ) -> BOOL;
}
pub const CRYPT_FIND_USER_KEYSET_FLAG: DWORD = 0x00000001;
pub const CRYPT_FIND_MACHINE_KEYSET_FLAG: DWORD = 0x00000002;
pub const CRYPT_FIND_SILENT_KEYSET_FLAG: DWORD = 0x00000040;
FN!{stdcall PFN_IMPORT_PRIV_KEY_FUNC(
    hCryptProv: HCRYPTPROV,
    pPrivateKeyInfo: *mut CRYPT_PRIVATE_KEY_INFO,
    dwFlags: DWORD,
    pvAuxInfo: *mut c_void,
) -> BOOL}
pub const CRYPT_OID_IMPORT_PRIVATE_KEY_INFO_FUNC: &'static str = "CryptDllImportPrivateKeyInfoEx";
extern "system" {
    pub fn CryptImportPKCS8(
        sPrivateKeyAndParams: CRYPT_PKCS8_IMPORT_PARAMS,
        dwFlags: DWORD,
        phCryptProv: *mut HCRYPTPROV,
        pvAuxInfo: *mut c_void,
    ) -> BOOL;
}
FN!{stdcall PFN_EXPORT_PRIV_KEY_FUNC(
    hCryptProv: HCRYPTPROV,
    dwKeySpec: DWORD,
    pszPrivateKeyObjId: LPSTR,
    dwFlags: DWORD,
    pvAuxInfo: *mut c_void,
    pPrivateKeyInfo: *mut CRYPT_PRIVATE_KEY_INFO,
    pcbPrivateKeyInfo: *mut DWORD,
) -> BOOL}
pub const CRYPT_OID_EXPORT_PRIVATE_KEY_INFO_FUNC: &'static str = "CryptDllExportPrivateKeyInfoEx";
pub const CRYPT_DELETE_KEYSET: DWORD = CRYPT_DELETEKEYSET;
extern "system" {
    pub fn CryptExportPKCS8(
        hCryptProv: HCRYPTPROV,
        dwKeySpec: DWORD,
        pszPrivateKeyObjId: LPSTR,
        dwFlags: DWORD,
        pvAuxInfo: *mut c_void,
        pbPrivateKeyBlob: *mut BYTE,
        pcbPrivateKeyBlob: *mut DWORD,
    ) -> BOOL;
    pub fn CryptExportPKCS8Ex(
        psExportParams: CRYPT_PKCS8_EXPORT_PARAMS,
        dwKeySpec: DWORD,
        pvAuxInfo: *mut c_void,
        pbPrivateKeyBlob: *mut BYTE,
        pcbPrivateKeyBlob: *mut DWORD,
    ) -> BOOL;
    pub fn CryptHashPublicKeyInfo(
        hCryptProv: HCRYPTPROV_LEGACY,
        Algid: ALG_ID,
        dwFlags: DWORD,
        dwCertEncodingType: DWORD,
        pInfo: PCERT_PUBLIC_KEY_INFO,
        pbComputedHash: *mut BYTE,
        pcbComputedHash: *mut DWORD,
    ) -> BOOL;
    pub fn CertRDNValueToStrA(
        dwValueType: DWORD,
        pValue: PCERT_RDN_VALUE_BLOB,
        psz: LPSTR,
        csz: DWORD,
    ) -> DWORD;
    pub fn CertRDNValueToStrW(
        dwValueType: DWORD,
        pValue: PCERT_RDN_VALUE_BLOB,
        psz: LPWSTR,
        csz: DWORD,
    ) -> DWORD;
    pub fn CertNameToStrA(
        dwCertEncodingType: DWORD,
        pName: PCERT_NAME_BLOB,
        dwStrType: DWORD,
        psz: LPSTR,
        csz: DWORD,
    ) -> DWORD;
    pub fn CertNameToStrW(
        dwCertEncodingType: DWORD,
        pName: PCERT_NAME_BLOB,
        dwStrType: DWORD,
        psz: LPWSTR,
        csz: DWORD,
    ) -> DWORD;
}
pub const CERT_SIMPLE_NAME_STR: DWORD = 1;
pub const CERT_OID_NAME_STR: DWORD = 2;
pub const CERT_X500_NAME_STR: DWORD = 3;
pub const CERT_XML_NAME_STR: DWORD = 4;
pub const CERT_NAME_STR_SEMICOLON_FLAG: DWORD = 0x40000000;
pub const CERT_NAME_STR_NO_PLUS_FLAG: DWORD = 0x20000000;
pub const CERT_NAME_STR_NO_QUOTING_FLAG: DWORD = 0x10000000;
pub const CERT_NAME_STR_CRLF_FLAG: DWORD = 0x08000000;
pub const CERT_NAME_STR_COMMA_FLAG: DWORD = 0x04000000;
pub const CERT_NAME_STR_REVERSE_FLAG: DWORD = 0x02000000;
pub const CERT_NAME_STR_FORWARD_FLAG: DWORD = 0x01000000;
pub const CERT_NAME_STR_DISABLE_IE4_UTF8_FLAG: DWORD = 0x00010000;
pub const CERT_NAME_STR_ENABLE_T61_UNICODE_FLAG: DWORD = 0x00020000;
pub const CERT_NAME_STR_ENABLE_UTF8_UNICODE_FLAG: DWORD = 0x00040000;
pub const CERT_NAME_STR_FORCE_UTF8_DIR_STR_FLAG: DWORD = 0x00080000;
pub const CERT_NAME_STR_DISABLE_UTF8_DIR_STR_FLAG: DWORD = 0x00100000;
pub const CERT_NAME_STR_ENABLE_PUNYCODE_FLAG: DWORD = 0x00200000;
extern "system" {
    pub fn CertStrToNameA(
        dwCertEncodingType: DWORD,
        pszX500: LPCSTR,
        dwStrType: DWORD,
        pvReserved: *mut c_void,
        pbEncoded: *mut BYTE,
        pcbEncoded: *mut DWORD,
        ppszError: *mut LPCSTR,
    ) -> BOOL;
    pub fn CertStrToNameW(
        dwCertEncodingType: DWORD,
        pszX500: LPCWSTR,
        dwStrType: DWORD,
        pvReserved: *mut c_void,
        pbEncoded: *mut BYTE,
        pcbEncoded: *mut DWORD,
        ppszError: *mut LPCWSTR,
    ) -> BOOL;
    pub fn CertGetNameStringA(
        pCertContext: PCCERT_CONTEXT,
        dwType: DWORD,
        dwFlags: DWORD,
        pvTypePara: *mut c_void,
        pszNameString: LPSTR,
        cchNameString: DWORD,
    ) -> DWORD;
    pub fn CertGetNameStringW(
        pCertContext: PCCERT_CONTEXT,
        dwType: DWORD,
        dwFlags: DWORD,
        pvTypePara: *mut c_void,
        pszNameString: LPWSTR,
        cchNameString: DWORD,
    ) -> DWORD;
}
pub const CERT_NAME_EMAIL_TYPE: DWORD = 1;
pub const CERT_NAME_RDN_TYPE: DWORD = 2;
pub const CERT_NAME_ATTR_TYPE: DWORD = 3;
pub const CERT_NAME_SIMPLE_DISPLAY_TYPE: DWORD = 4;
pub const CERT_NAME_FRIENDLY_DISPLAY_TYPE: DWORD = 5;
pub const CERT_NAME_DNS_TYPE: DWORD = 6;
pub const CERT_NAME_URL_TYPE: DWORD = 7;
pub const CERT_NAME_UPN_TYPE: DWORD = 8;
pub const CERT_NAME_ISSUER_FLAG: DWORD = 0x1;
pub const CERT_NAME_DISABLE_IE4_UTF8_FLAG: DWORD = 0x00010000;
pub const CERT_NAME_SEARCH_ALL_NAMES_FLAG: DWORD = 0x2;
FN!{stdcall PFN_CRYPT_GET_SIGNER_CERTIFICATE(
    pvGetArg: *mut c_void,
    dwCertEncodingType: DWORD,
    pSignerId: PCERT_INFO,
    hMsgCertStore: HCERTSTORE,
) -> PCCERT_CONTEXT}
STRUCT!{struct CRYPT_SIGN_MESSAGE_PARA {
    cbSize: DWORD,
    dwMsgEncodingType: DWORD,
    pSigningCert: PCCERT_CONTEXT,
    HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pvHashAuxInfo: *mut c_void,
    cMsgCert: DWORD,
    rgpMsgCert: *mut PCCERT_CONTEXT,
    cMsgCrl: DWORD,
    rgpMsgCrl: *mut PCCRL_CONTEXT,
    cAuthAttr: DWORD,
    rgAuthAttr: PCRYPT_ATTRIBUTE,
    cUnauthAttr: DWORD,
    rgUnauthAttr: PCRYPT_ATTRIBUTE,
    dwFlags: DWORD,
    dwInnerContentType: DWORD,
    HashEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pvHashEncryptionAuxInfo: *mut c_void,
}}
pub type PCRYPT_SIGN_MESSAGE_PARA = *mut CRYPT_SIGN_MESSAGE_PARA;
pub const CRYPT_MESSAGE_BARE_CONTENT_OUT_FLAG: DWORD = 0x00000001;
pub const CRYPT_MESSAGE_ENCAPSULATED_CONTENT_OUT_FLAG: DWORD = 0x00000002;
pub const CRYPT_MESSAGE_KEYID_SIGNER_FLAG: DWORD = 0x00000004;
pub const CRYPT_MESSAGE_SILENT_KEYSET_FLAG: DWORD = 0x00000040;
STRUCT!{struct CRYPT_VERIFY_MESSAGE_PARA {
    cbSize: DWORD,
    dwMsgAndCertEncodingType: DWORD,
    hCryptProv: HCRYPTPROV_LEGACY,
    pfnGetSignerCertificate: PFN_CRYPT_GET_SIGNER_CERTIFICATE,
    pvGetArg: *mut c_void,
    pStrongSignPara: PCCERT_STRONG_SIGN_PARA,
}}
pub type PCRYPT_VERIFY_MESSAGE_PARA = *mut CRYPT_VERIFY_MESSAGE_PARA;
STRUCT!{struct CRYPT_ENCRYPT_MESSAGE_PARA {
    cbSize: DWORD,
    dwMsgEncodingType: DWORD,
    hCryptProv: HCRYPTPROV_LEGACY,
    ContentEncryptionAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pvEncryptionAuxInfo: *mut c_void,
    dwFlags: DWORD,
    dwInnerContentType: DWORD,
}}
pub type PCRYPT_ENCRYPT_MESSAGE_PARA = *mut CRYPT_DECRYPT_MESSAGE_PARA;
pub const CRYPT_MESSAGE_KEYID_RECIPIENT_FLAG: DWORD = 0x4;
STRUCT!{struct CRYPT_DECRYPT_MESSAGE_PARA {
    cbSize: DWORD,
    dwMsgAndCertEncodingType: DWORD,
    cCertStore: DWORD,
    rghCertStore: *mut HCERTSTORE,
    dwFlags: DWORD,
}}
pub type PCRYPT_DECRYPT_MESSAGE_PARA = *mut CRYPT_DECRYPT_MESSAGE_PARA;
STRUCT!{struct CRYPT_HASH_MESSAGE_PARA {
    cbSize: DWORD,
    dwMsgEncodingType: DWORD,
    hCryptProv: HCRYPTPROV_LEGACY,
    HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pvHashAuxInfo: *mut c_void,
}}
pub type PCRYPT_HASH_MESSAGE_PARA = *mut CRYPT_HASH_MESSAGE_PARA;
UNION!{union CRYPT_KEY_SIGN_MESSAGE_PARA_u {
    [usize; 1],
    hCryptProv hCryptProv_mut: HCRYPTPROV,
    hNCryptKey hNCryptKey_mut: NCRYPT_KEY_HANDLE,
}}
STRUCT!{struct CRYPT_KEY_SIGN_MESSAGE_PARA {
    cbSize: DWORD,
    dwMsgAndCertEncodingType: DWORD,
    u: CRYPT_KEY_SIGN_MESSAGE_PARA_u,
    dwKeySpec: DWORD,
    HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pvHashAuxInfo: *mut c_void,
    PubKeyAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
}}
pub type PCRYPT_KEY_SIGN_MESSAGE_PARA = *mut CRYPT_KEY_SIGN_MESSAGE_PARA;
STRUCT!{struct CRYPT_KEY_VERIFY_MESSAGE_PARA {
    cbSize: DWORD,
    dwMsgEncodingType: DWORD,
    hCryptProv: HCRYPTPROV_LEGACY,
}}
pub type PCRYPT_KEY_VERIFY_MESSAGE_PARA = *mut CRYPT_KEY_VERIFY_MESSAGE_PARA;
extern "system" {
    pub fn CryptSignMessage(
        pSignPara: PCRYPT_SIGN_MESSAGE_PARA,
        fDetachedSignature: BOOL,
        cToBeSigned: DWORD,
        rgpbToBeSigned: *mut *const BYTE,
        rgcbToBeSigned: *mut DWORD,
        pbSignedBlob: *mut BYTE,
        pcbSignedBlob: *mut DWORD,
    ) -> BOOL;
    pub fn CryptVerifyMessageSignature(
        pVerifyPara: PCRYPT_VERIFY_MESSAGE_PARA,
        dwSignerIndex: DWORD,
        pbSignedBlob: *const BYTE,
        cbSignedBlob: DWORD,
        pbDecoded: *mut BYTE,
        pcbDecoded: *mut DWORD,
        ppSignerCert: *mut PCCERT_CONTEXT,
    ) -> BOOL;
    pub fn CryptGetMessageSignerCount(
        dwMsgEncodingType: DWORD,
        pbSignedBlob: *const BYTE,
        cbSignedBlob: DWORD,
    ) -> LONG;
    pub fn CryptGetMessageCertificates(
        dwMsgAndCertEncodingType: DWORD,
        hCryptProv: HCRYPTPROV_LEGACY,
        dwFlags: DWORD,
        pbSignedBlob: *const BYTE,
        cbSignedBlob: DWORD,
    ) -> HCERTSTORE;
    pub fn CryptVerifyDetachedMessageSignature(
        pVerifyPara: PCRYPT_VERIFY_MESSAGE_PARA,
        dwSignerIndex: DWORD,
        pbDetachedSignBlob: *const BYTE,
        cbDetachedSignBlob: DWORD,
        cToBeSigned: DWORD,
        rgpbToBeSigned: *mut *const BYTE,
        rgcbToBeSigned: *mut DWORD,
        ppSignerCert: *mut PCCERT_CONTEXT,
    ) -> BOOL;
    pub fn CryptEncryptMessage(
        pEncryptPara: PCRYPT_ENCRYPT_MESSAGE_PARA,
        cRecipientCert: DWORD,
        rgpRecipientCert: *mut PCCERT_CONTEXT,
        pbToBeEncrypted: *const BYTE,
        cbToBeEncrypted: DWORD,
        pbEncryptedBlob: *mut BYTE,
        pcbEncryptedBlob: *mut DWORD,
    ) -> BOOL;
    pub fn CryptDecryptMessage(
        pDecryptPara: PCRYPT_DECRYPT_MESSAGE_PARA,
        pbEncryptedBlob: *const BYTE,
        cbEncryptedBlob: DWORD,
        pbDecrypted: *mut BYTE,
        pcbDecrypted: *mut DWORD,
        ppXchgCert: *mut PCCERT_CONTEXT,
    ) -> BOOL;
    pub fn CryptSignAndEncryptMessage(
        pSignPara: PCRYPT_SIGN_MESSAGE_PARA,
        pEncryptPara: PCRYPT_ENCRYPT_MESSAGE_PARA,
        cRecipientCert: DWORD,
        rgpRecipientCert: *mut PCCERT_CONTEXT,
        pbToBeSignedAndEncrypted: *const BYTE,
        cbToBeSignedAndEncrypted: DWORD,
        pbSignedAndEncryptedBlob: *mut BYTE,
        pcbSignedAndEncryptedBlob: *mut DWORD,
    ) -> BOOL;
    pub fn CryptDecryptAndVerifyMessageSignature(
        pDecryptPara: PCRYPT_DECRYPT_MESSAGE_PARA,
        pVerifyPara: PCRYPT_VERIFY_MESSAGE_PARA,
        dwSignerIndex: DWORD,
        pbEncryptedBlob: *const BYTE,
        cbEncryptedBlob: DWORD,
        pbDecrypted: *mut BYTE,
        pcbDecrypted: *mut DWORD,
        ppXchgCert: *mut PCCERT_CONTEXT,
        ppSignerCert: *mut PCCERT_CONTEXT,
    ) -> BOOL;
    pub fn CryptDecodeMessage(
        dwMsgTypeFlags: DWORD,
        pDecryptPara: PCRYPT_DECRYPT_MESSAGE_PARA,
        pVerifyPara: PCRYPT_VERIFY_MESSAGE_PARA,
        dwSignerIndex: DWORD,
        pbEncodedBlob: *const BYTE,
        cbEncodedBlob: DWORD,
        dwPrevInnerContentType: DWORD,
        pdwMsgType: *mut DWORD,
        pdwInnerContentType: *mut DWORD,
        pbDecoded: *mut BYTE,
        pcbDecoded: *mut DWORD,
        ppXchgCert: *mut PCCERT_CONTEXT,
        ppSignerCert: *mut PCCERT_CONTEXT,
    ) -> BOOL;
    pub fn CryptHashMessage(
        pHashPara: PCRYPT_HASH_MESSAGE_PARA,
        fDetachedHash: BOOL,
        cToBeHashed: DWORD,
        rgpbToBeHashed: *mut *const BYTE,
        rgcbToBeHashed: *mut DWORD,
        pbHashedBlob: *mut BYTE,
        pcbHashedBlob: *mut DWORD,
        pbComputedHash: *mut BYTE,
        pcbComputedHash: *mut DWORD,
    ) -> BOOL;
    pub fn CryptVerifyMessageHash(
        pHashPara: PCRYPT_HASH_MESSAGE_PARA,
        pbHashedBlob: *mut BYTE,
        cbHashedBlob: DWORD,
        pbToBeHashed: *mut BYTE,
        pcbToBeHashed: *mut DWORD,
        pbComputedHash: *mut BYTE,
        pcbComputedHash: *mut DWORD,
    ) -> BOOL;
    pub fn CryptVerifyDetachedMessageHash(
        pHashPara: PCRYPT_HASH_MESSAGE_PARA,
        pbDetachedHashBlob: *mut BYTE,
        cbDetachedHashBlob: DWORD,
        cToBeHashed: DWORD,
        rgpbToBeHashed: *mut *const BYTE,
        rgcbToBeHashed: *mut DWORD,
        pbComputedHash: *mut BYTE,
        pcbComputedHash: *mut DWORD,
    ) -> BOOL;
    pub fn CryptSignMessageWithKey(
        pSignPara: PCRYPT_KEY_SIGN_MESSAGE_PARA,
        pbToBeSigned: *const BYTE,
        cbToBeSigned: DWORD,
        pbSignedBlob: *mut BYTE,
        pcbSignedBlob: *mut DWORD,
    ) -> BOOL;
    pub fn CryptVerifyMessageSignatureWithKey(
        pVerifyPara: PCRYPT_KEY_VERIFY_MESSAGE_PARA,
        pPublicKeyInfo: PCERT_PUBLIC_KEY_INFO,
        pbSignedBlob: *const BYTE,
        cbSignedBlob: DWORD,
        pbDecoded: *mut BYTE,
        pcbDecoded: *mut DWORD,
    ) -> BOOL;
    pub fn CertOpenSystemStoreA(
        hProv: HCRYPTPROV_LEGACY,
        szSubsystemProtocol: LPCSTR,
    ) -> HCERTSTORE;
    pub fn CertOpenSystemStoreW(
        hProv: HCRYPTPROV_LEGACY,
        szSubsystemProtocol: LPCWSTR,
    ) -> HCERTSTORE;
    pub fn CertAddEncodedCertificateToSystemStoreA(
        szCertStoreName: LPCSTR,
        pbCertEncoded: *const BYTE,
        cbCertEncoded: DWORD,
    ) -> BOOL;
    pub fn CertAddEncodedCertificateToSystemStoreW(
        szCertStoreName: LPCWSTR,
        pbCertEncoded: *const BYTE,
        cbCertEncoded: DWORD,
    ) -> BOOL;
}
STRUCT!{struct CERT_CHAIN {
    cCerts: DWORD,
    certs: PCERT_BLOB,
    keyLocatorInfo: CRYPT_KEY_PROV_INFO,
}}
pub type PCERT_CHAIN = *mut CERT_CHAIN;
extern "system" {
    pub fn FindCertsByIssuer(
        pCertChains: PCERT_CHAIN,
        pcbCertChains: *mut DWORD,
        pcCertChains: *mut DWORD,
        pbEncodedIssuerName: *mut BYTE,
        cbEncodedIssuerName: DWORD,
        pwszPurpose: LPCWSTR,
        dwKeySpec: DWORD,
    ) -> HRESULT;
    pub fn CryptQueryObject(
        dwObjectType: DWORD,
        pvObject: *const c_void,
        dwExpectedContentTypeFlags: DWORD,
        dwExpectedFormatTypeFlags: DWORD,
        dwFlags: DWORD,
        pdwMsgAndCertEncodingType: *mut DWORD,
        pdwContentType: *mut DWORD,
        pdwFormatType: *mut DWORD,
        phCertStore: *mut HCERTSTORE,
        phMsg: *mut HCRYPTMSG,
        ppvContext: *mut *const c_void,
    ) -> BOOL;
}
pub const CERT_QUERY_OBJECT_FILE: DWORD = 0x00000001;
pub const CERT_QUERY_OBJECT_BLOB: DWORD = 0x00000002;
pub const CERT_QUERY_CONTENT_CERT: DWORD = 1;
pub const CERT_QUERY_CONTENT_CTL: DWORD = 2;
pub const CERT_QUERY_CONTENT_CRL: DWORD = 3;
pub const CERT_QUERY_CONTENT_SERIALIZED_STORE: DWORD = 4;
pub const CERT_QUERY_CONTENT_SERIALIZED_CERT: DWORD = 5;
pub const CERT_QUERY_CONTENT_SERIALIZED_CTL: DWORD = 6;
pub const CERT_QUERY_CONTENT_SERIALIZED_CRL: DWORD = 7;
pub const CERT_QUERY_CONTENT_PKCS7_SIGNED: DWORD = 8;
pub const CERT_QUERY_CONTENT_PKCS7_UNSIGNED: DWORD = 9;
pub const CERT_QUERY_CONTENT_PKCS7_SIGNED_EMBED: DWORD = 10;
pub const CERT_QUERY_CONTENT_PKCS10: DWORD = 11;
pub const CERT_QUERY_CONTENT_PFX: DWORD = 12;
pub const CERT_QUERY_CONTENT_CERT_PAIR: DWORD = 13;
pub const CERT_QUERY_CONTENT_PFX_AND_LOAD: DWORD = 14;
pub const CERT_QUERY_CONTENT_FLAG_CERT: DWORD = 1 << CERT_QUERY_CONTENT_CERT;
pub const CERT_QUERY_CONTENT_FLAG_CTL: DWORD = 1 << CERT_QUERY_CONTENT_CTL;
pub const CERT_QUERY_CONTENT_FLAG_CRL: DWORD = 1 << CERT_QUERY_CONTENT_CRL;
pub const CERT_QUERY_CONTENT_FLAG_SERIALIZED_STORE: DWORD
    = 1<< CERT_QUERY_CONTENT_SERIALIZED_STORE;
pub const CERT_QUERY_CONTENT_FLAG_SERIALIZED_CERT: DWORD = 1 << CERT_QUERY_CONTENT_SERIALIZED_CERT;
pub const CERT_QUERY_CONTENT_FLAG_SERIALIZED_CTL: DWORD = 1 << CERT_QUERY_CONTENT_SERIALIZED_CTL;
pub const CERT_QUERY_CONTENT_FLAG_SERIALIZED_CRL: DWORD = 1 << CERT_QUERY_CONTENT_SERIALIZED_CRL;
pub const CERT_QUERY_CONTENT_FLAG_PKCS7_SIGNED: DWORD = 1 << CERT_QUERY_CONTENT_PKCS7_SIGNED;
pub const CERT_QUERY_CONTENT_FLAG_PKCS7_UNSIGNED: DWORD = 1 << CERT_QUERY_CONTENT_PKCS7_UNSIGNED;
pub const CERT_QUERY_CONTENT_FLAG_PKCS7_SIGNED_EMBED: DWORD
    = 1 << CERT_QUERY_CONTENT_PKCS7_SIGNED_EMBED;
pub const CERT_QUERY_CONTENT_FLAG_PKCS10: DWORD = 1 << CERT_QUERY_CONTENT_PKCS10;
pub const CERT_QUERY_CONTENT_FLAG_PFX: DWORD = 1 << CERT_QUERY_CONTENT_PFX;
pub const CERT_QUERY_CONTENT_FLAG_CERT_PAIR: DWORD = 1 << CERT_QUERY_CONTENT_CERT_PAIR;
pub const CERT_QUERY_CONTENT_FLAG_PFX_AND_LOAD: DWORD = 1 << CERT_QUERY_CONTENT_PFX_AND_LOAD;
pub const CERT_QUERY_CONTENT_FLAG_ALL: DWORD = CERT_QUERY_CONTENT_FLAG_CERT
    | CERT_QUERY_CONTENT_FLAG_CTL | CERT_QUERY_CONTENT_FLAG_CRL
    | CERT_QUERY_CONTENT_FLAG_SERIALIZED_STORE | CERT_QUERY_CONTENT_FLAG_SERIALIZED_CERT
    | CERT_QUERY_CONTENT_FLAG_SERIALIZED_CTL | CERT_QUERY_CONTENT_FLAG_SERIALIZED_CRL
    | CERT_QUERY_CONTENT_FLAG_PKCS7_SIGNED | CERT_QUERY_CONTENT_FLAG_PKCS7_UNSIGNED
    | CERT_QUERY_CONTENT_FLAG_PKCS7_SIGNED_EMBED | CERT_QUERY_CONTENT_FLAG_PKCS10
    | CERT_QUERY_CONTENT_FLAG_PFX | CERT_QUERY_CONTENT_FLAG_CERT_PAIR;
pub const CERT_QUERY_CONTENT_FLAG_ALL_ISSUER_CERT: DWORD = CERT_QUERY_CONTENT_FLAG_CERT
    | CERT_QUERY_CONTENT_FLAG_SERIALIZED_STORE | CERT_QUERY_CONTENT_FLAG_SERIALIZED_CERT
    | CERT_QUERY_CONTENT_FLAG_PKCS7_SIGNED | CERT_QUERY_CONTENT_FLAG_PKCS7_UNSIGNED;
pub const CERT_QUERY_FORMAT_BINARY: DWORD = 1;
pub const CERT_QUERY_FORMAT_BASE64_ENCODED: DWORD = 2;
pub const CERT_QUERY_FORMAT_ASN_ASCII_HEX_ENCODED: DWORD = 3;
pub const CERT_QUERY_FORMAT_FLAG_BINARY: DWORD = 1 << CERT_QUERY_FORMAT_BINARY;
pub const CERT_QUERY_FORMAT_FLAG_BASE64_ENCODED: DWORD = 1 << CERT_QUERY_FORMAT_BASE64_ENCODED;
pub const CERT_QUERY_FORMAT_FLAG_ASN_ASCII_HEX_ENCODED: DWORD
    = 1 << CERT_QUERY_FORMAT_ASN_ASCII_HEX_ENCODED;
pub const CERT_QUERY_FORMAT_FLAG_ALL: DWORD = CERT_QUERY_FORMAT_FLAG_BINARY
    | CERT_QUERY_FORMAT_FLAG_BASE64_ENCODED | CERT_QUERY_FORMAT_FLAG_ASN_ASCII_HEX_ENCODED;
extern "system" {
    pub fn CryptMemAlloc(
        cbSize: ULONG,
    ) -> LPVOID;
    pub fn CryptMemRealloc(
        pv: LPVOID,
        cbSize: ULONG,
    ) -> LPVOID;
    pub fn CryptMemFree(
        pv: LPVOID,
    );
}
pub type HCRYPTASYNC = HANDLE;
pub type PHCRYPTASYNC = *mut HANDLE;
FN!{stdcall PFN_CRYPT_ASYNC_PARAM_FREE_FUNC(
    pszParamOid: LPSTR,
    pvParam: LPVOID,
) -> ()}
extern "system" {
    pub fn CryptCreateAsyncHandle(
        dwFlags: DWORD,
        phAsync: PHCRYPTASYNC,
    ) -> BOOL;
    pub fn CryptSetAsyncParam(
        hAsync: HCRYPTASYNC,
        pszParamOid: LPSTR,
        pvParam: LPVOID,
        pfnFree: PFN_CRYPT_ASYNC_PARAM_FREE_FUNC,
    ) -> BOOL;
    pub fn CryptGetAsyncParam(
        hAsync: HCRYPTASYNC,
        pszParamOid: LPSTR,
        ppvParam: *mut LPVOID,
        ppfnFree: *mut PFN_CRYPT_ASYNC_PARAM_FREE_FUNC,
    ) -> BOOL;
    pub fn CryptCloseAsyncHandle(
        hAsync: HCRYPTASYNC,
    ) -> BOOL;
}
STRUCT!{struct CRYPT_BLOB_ARRAY {
    cBlob: DWORD,
    rgBlob: PCRYPT_DATA_BLOB,
}}
pub type PCRYPT_BLOB_ARRAY = *mut CRYPT_BLOB_ARRAY;
STRUCT!{struct CRYPT_CREDENTIALS {
    cbSize: DWORD,
    pszCredentialsOid: LPCSTR,
    pvCredentials: LPVOID,
}}
pub type PCRYPT_CREDENTIALS = *mut CRYPT_CREDENTIALS;
pub const CREDENTIAL_OID_PASSWORD_CREDENTIALS_A: LPCSTR = 1 as LPCSTR;
pub const CREDENTIAL_OID_PASSWORD_CREDENTIALS_W: LPCSTR = 2 as LPCSTR;
STRUCT!{struct CRYPT_PASSWORD_CREDENTIALSA {
    cbSize: DWORD,
    pszUsername: LPSTR,
    pszPassword: LPSTR,
}}
pub type PCRYPT_PASSWORD_CREDENTIALSA = *mut CRYPT_PASSWORD_CREDENTIALSA;
STRUCT!{struct CRYPT_PASSWORD_CREDENTIALSW {
    cbSize: DWORD,
    pszUsername: LPWSTR,
    pszPassword: LPWSTR,
}}
pub type PCRYPT_PASSWORD_CREDENTIALSW = *mut CRYPT_PASSWORD_CREDENTIALSW;
pub const SCHEME_OID_RETRIEVE_ENCODED_OBJECT_FUNC: &'static str = "SchemeDllRetrieveEncodedObject";
pub const SCHEME_OID_RETRIEVE_ENCODED_OBJECTW_FUNC: &'static str
    = "SchemeDllRetrieveEncodedObjectW";
FN!{stdcall PFN_FREE_ENCODED_OBJECT_FUNC(
    pszObjectOid: LPCSTR,
    pObject: PCRYPT_BLOB_ARRAY,
    pvFreeContext: LPVOID,
) -> ()}
pub const CONTEXT_OID_CREATE_OBJECT_CONTEXT_FUNC: &'static str = "ContextDllCreateObjectContext";
pub const CONTEXT_OID_CERTIFICATE: LPCSTR = 1 as LPCSTR;
pub const CONTEXT_OID_CRL: LPCSTR = 2 as LPCSTR;
pub const CONTEXT_OID_CTL: LPCSTR = 3 as LPCSTR;
pub const CONTEXT_OID_PKCS7: LPCSTR = 4 as LPCSTR;
pub const CONTEXT_OID_CAPI2_ANY: LPCSTR = 5 as LPCSTR;
pub const CONTEXT_OID_OCSP_RESP: LPCSTR = 6 as LPCSTR;
pub const CRYPT_RETRIEVE_MULTIPLE_OBJECTS: DWORD = 0x00000001;
pub const CRYPT_CACHE_ONLY_RETRIEVAL: DWORD = 0x00000002;
pub const CRYPT_WIRE_ONLY_RETRIEVAL: DWORD = 0x00000004;
pub const CRYPT_DONT_CACHE_RESULT: DWORD = 0x00000008;
pub const CRYPT_ASYNC_RETRIEVAL: DWORD = 0x00000010;
pub const CRYPT_STICKY_CACHE_RETRIEVAL: DWORD = 0x00001000;
pub const CRYPT_LDAP_SCOPE_BASE_ONLY_RETRIEVAL: DWORD = 0x00002000;
pub const CRYPT_OFFLINE_CHECK_RETRIEVAL: DWORD = 0x00004000;
pub const CRYPT_LDAP_INSERT_ENTRY_ATTRIBUTE: DWORD = 0x00008000;
pub const CRYPT_LDAP_SIGN_RETRIEVAL: DWORD = 0x00010000;
pub const CRYPT_NO_AUTH_RETRIEVAL: DWORD = 0x00020000;
pub const CRYPT_LDAP_AREC_EXCLUSIVE_RETRIEVAL: DWORD = 0x00040000;
pub const CRYPT_AIA_RETRIEVAL: DWORD = 0x00080000;
pub const CRYPT_HTTP_POST_RETRIEVAL: DWORD = 0x00100000;
pub const CRYPT_PROXY_CACHE_RETRIEVAL: DWORD = 0x00200000;
pub const CRYPT_NOT_MODIFIED_RETRIEVAL: DWORD = 0x00400000;
pub const CRYPT_ENABLE_SSL_REVOCATION_RETRIEVAL: DWORD = 0x00800000;
pub const CRYPT_RANDOM_QUERY_STRING_RETRIEVAL: DWORD = 0x04000000;
pub const CRYPT_ENABLE_FILE_RETRIEVAL: DWORD = 0x08000000;
pub const CRYPT_CREATE_NEW_FLUSH_ENTRY: DWORD = 0x10000000;
pub const CRYPT_VERIFY_CONTEXT_SIGNATURE: DWORD = 0x00000020;
pub const CRYPT_VERIFY_DATA_HASH: DWORD = 0x00000040;
pub const CRYPT_KEEP_TIME_VALID: DWORD = 0x00000080;
pub const CRYPT_DONT_VERIFY_SIGNATURE: DWORD = 0x00000100;
pub const CRYPT_DONT_CHECK_TIME_VALIDITY: DWORD = 0x00000200;
pub const CRYPT_CHECK_FRESHNESS_TIME_VALIDITY: DWORD = 0x00000400;
pub const CRYPT_ACCUMULATIVE_TIMEOUT: DWORD = 0x00000800;
pub const CRYPT_OCSP_ONLY_RETRIEVAL: DWORD = 0x01000000;
pub const CRYPT_NO_OCSP_FAILOVER_TO_CRL_RETRIEVAL: DWORD = 0x02000000;
STRUCT!{struct CRYPTNET_URL_CACHE_PRE_FETCH_INFO {
    cbSize: DWORD,
    dwObjectType: DWORD,
    dwError: DWORD,
    dwReserved: DWORD,
    ThisUpdateTime: FILETIME,
    NextUpdateTime: FILETIME,
    PublishTime: FILETIME,
}}
pub type PCRYPTNET_URL_CACHE_PRE_FETCH_INFO = *mut CRYPTNET_URL_CACHE_PRE_FETCH_INFO;
pub const CRYPTNET_URL_CACHE_PRE_FETCH_NONE: DWORD = 0;
pub const CRYPTNET_URL_CACHE_PRE_FETCH_BLOB: DWORD = 1;
pub const CRYPTNET_URL_CACHE_PRE_FETCH_CRL: DWORD = 2;
pub const CRYPTNET_URL_CACHE_PRE_FETCH_OCSP: DWORD = 3;
pub const CRYPTNET_URL_CACHE_PRE_FETCH_AUTOROOT_CAB: DWORD = 5;
pub const CRYPTNET_URL_CACHE_PRE_FETCH_DISALLOWED_CERT_CAB: DWORD = 6;
pub const CRYPTNET_URL_CACHE_PRE_FETCH_PIN_RULES_CAB: DWORD = 7;
STRUCT!{struct CRYPTNET_URL_CACHE_FLUSH_INFO {
    cbSize: DWORD,
    dwExemptSeconds: DWORD,
    ExpireTime: FILETIME,
}}
pub type PCRYPTNET_URL_CACHE_FLUSH_INFO = *mut CRYPTNET_URL_CACHE_FLUSH_INFO;
pub const CRYPTNET_URL_CACHE_DEFAULT_FLUSH: DWORD = 0;
pub const CRYPTNET_URL_CACHE_DISABLE_FLUSH: DWORD = 0xFFFFFFFF;
STRUCT!{struct CRYPTNET_URL_CACHE_RESPONSE_INFO {
    cbSize: DWORD,
    wResponseType: WORD,
    wResponseFlags: WORD,
    LastModifiedTime: FILETIME,
    dwMaxAge: DWORD,
    pwszETag: LPCWSTR,
    dwProxyId: DWORD,
}}
pub type PCRYPTNET_URL_CACHE_RESPONSE_INFO = *mut CRYPTNET_URL_CACHE_RESPONSE_INFO;
pub const CRYPTNET_URL_CACHE_RESPONSE_NONE: WORD = 0;
pub const CRYPTNET_URL_CACHE_RESPONSE_HTTP: WORD = 1;
pub const CRYPTNET_URL_CACHE_RESPONSE_VALIDATED: WORD = 0x8000;
STRUCT!{struct CRYPT_RETRIEVE_AUX_INFO {
    cbSize: DWORD,
    pLastSyncTime: *mut FILETIME,
    dwMaxUrlRetrievalByteCount: DWORD,
    pPreFetchInfo: PCRYPTNET_URL_CACHE_PRE_FETCH_INFO,
    pFlushInfo: PCRYPTNET_URL_CACHE_FLUSH_INFO,
    ppResponseInfo: *mut PCRYPTNET_URL_CACHE_RESPONSE_INFO,
    pwszCacheFileNamePrefix: LPWSTR,
    pftCacheResync: LPFILETIME,
    fProxyCacheRetrieval: BOOL,
    dwHttpStatusCode: DWORD,
    ppwszErrorResponseHeaders: *mut LPWSTR,
    ppErrorContentBlob: *mut PCRYPT_DATA_BLOB,
}}
pub type PCRYPT_RETRIEVE_AUX_INFO = *mut CRYPT_RETRIEVE_AUX_INFO;
pub const CRYPT_RETRIEVE_MAX_ERROR_CONTENT_LENGTH: DWORD = 0x1000;
extern "system" {
    pub fn CryptRetrieveObjectByUrlA(
        pszUrl: LPCSTR,
        pszObjectOid: LPCSTR,
        dwRetrievalFlags: DWORD,
        dwTimeout: DWORD,
        ppvObject: *mut LPVOID,
        hAsyncRetrieve: HCRYPTASYNC,
        pCredentials: PCRYPT_CREDENTIALS,
        pvVerify: LPVOID,
        pAuxInfo: PCRYPT_RETRIEVE_AUX_INFO,
    ) -> BOOL;
    pub fn CryptRetrieveObjectByUrlW(
        pszUrl: LPCWSTR,
        pszObjectOid: LPCSTR,
        dwRetrievalFlags: DWORD,
        dwTimeout: DWORD,
        ppvObject: *mut LPVOID,
        hAsyncRetrieve: HCRYPTASYNC,
        pCredentials: PCRYPT_CREDENTIALS,
        pvVerify: LPVOID,
        pAuxInfo: PCRYPT_RETRIEVE_AUX_INFO,
    ) -> BOOL;
}
FN!{stdcall PFN_CRYPT_CANCEL_RETRIEVAL(
    dwFlags: DWORD,
    pvArg: *mut c_void,
) -> BOOL}
extern "system" {
    pub fn CryptInstallCancelRetrieval(
        pfnCancel: PFN_CRYPT_CANCEL_RETRIEVAL,
        pvArg: *const c_void,
        dwFlags: DWORD,
        pvReserved: *mut c_void,
    ) -> BOOL;
    pub fn CryptUninstallCancelRetrieval(
        dwFlags: DWORD,
        pvReserved: *mut c_void,
    ) -> BOOL;
    pub fn CryptCancelAsyncRetrieval(
        hAsyncRetrieval: HCRYPTASYNC,
    ) -> BOOL;
}
pub const CRYPT_PARAM_ASYNC_RETRIEVAL_COMPLETION: LPCSTR = 1 as LPCSTR;
FN!{stdcall PFN_CRYPT_ASYNC_RETRIEVAL_COMPLETION_FUNC(
    pvCompletion: LPVOID,
    dwCompletionCode: DWORD,
    pszUrl: LPCSTR,
    pszObjectOid: LPSTR,
    pvObject: LPVOID,
) -> ()}
STRUCT!{struct CRYPT_ASYNC_RETRIEVAL_COMPLETION {
    pfnCompletion: PFN_CRYPT_ASYNC_RETRIEVAL_COMPLETION_FUNC,
    pvCompletion: LPVOID,
}}
pub type PCRYPT_ASYNC_RETRIEVAL_COMPLETION = *mut CRYPT_ASYNC_RETRIEVAL_COMPLETION;
pub const CRYPT_PARAM_CANCEL_ASYNC_RETRIEVAL: LPCSTR = 2 as LPCSTR;
FN!{stdcall PFN_CANCEL_ASYNC_RETRIEVAL_FUNC(
    hAsyncRetrieve: HCRYPTASYNC,
) -> BOOL}
pub const CRYPT_GET_URL_FROM_PROPERTY: DWORD = 0x00000001;
pub const CRYPT_GET_URL_FROM_EXTENSION: DWORD = 0x00000002;
pub const CRYPT_GET_URL_FROM_UNAUTH_ATTRIBUTE: DWORD = 0x00000004;
pub const CRYPT_GET_URL_FROM_AUTH_ATTRIBUTE: DWORD = 0x00000008;
STRUCT!{struct CRYPT_URL_ARRAY {
    cUrl: DWORD,
    rgwszUrl: *mut LPWSTR,
}}
pub type PCRYPT_URL_ARRAY = *mut CRYPT_URL_ARRAY;
STRUCT!{struct CRYPT_URL_INFO {
    cbSize: DWORD,
    dwSyncDeltaTime: DWORD,
    cGroup: DWORD,
    rgcGroupEntry: *mut DWORD,
}}
pub type PCRYPT_URL_INFO = *mut CRYPT_URL_INFO;
extern "system" {
    pub fn CryptGetObjectUrl(
        pszUrlOid: LPCSTR,
        pvPara: LPVOID,
        dwFlags: DWORD,
        pUrlArray: PCRYPT_URL_ARRAY,
        pcbUrlArray: *mut DWORD,
        pUrlInfo: PCRYPT_URL_INFO,
        pcbUrlInfo: *mut DWORD,
        pvReserved: LPVOID,
    ) -> BOOL;
}
pub const URL_OID_GET_OBJECT_URL_FUNC: &'static str = "UrlDllGetObjectUrl";
pub const URL_OID_CERTIFICATE_ISSUER: LPCSTR = 1 as LPCSTR;
pub const URL_OID_CERTIFICATE_CRL_DIST_POINT: LPCSTR = 2 as LPCSTR;
pub const URL_OID_CTL_ISSUER: LPCSTR = 3 as LPCSTR;
pub const URL_OID_CTL_NEXT_UPDATE: LPCSTR = 4 as LPCSTR;
pub const URL_OID_CRL_ISSUER: LPCSTR = 5 as LPCSTR;
pub const URL_OID_CERTIFICATE_FRESHEST_CRL: LPCSTR = 6 as LPCSTR;
pub const URL_OID_CRL_FRESHEST_CRL: LPCSTR = 7 as LPCSTR;
pub const URL_OID_CROSS_CERT_DIST_POINT: LPCSTR = 8 as LPCSTR;
pub const URL_OID_CERTIFICATE_OCSP: LPCSTR = 9 as LPCSTR;
pub const URL_OID_CERTIFICATE_OCSP_AND_CRL_DIST_POINT: LPCSTR = 10 as LPCSTR;
pub const URL_OID_CERTIFICATE_CRL_DIST_POINT_AND_OCSP: LPCSTR = 11 as LPCSTR;
pub const URL_OID_CROSS_CERT_SUBJECT_INFO_ACCESS: LPCSTR = 12 as LPCSTR;
pub const URL_OID_CERTIFICATE_ONLY_OCSP: LPCSTR = 13 as LPCSTR;
STRUCT!{struct CERT_CRL_CONTEXT_PAIR {
    pCertContext: PCCERT_CONTEXT,
    pCrlContext: PCCRL_CONTEXT,
}}
pub type PCERT_CRL_CONTEXT_PAIR = *mut CERT_CRL_CONTEXT_PAIR;
pub type PCCERT_CRL_CONTEXT_PAIR = *const CERT_CRL_CONTEXT_PAIR;
STRUCT!{struct CRYPT_GET_TIME_VALID_OBJECT_EXTRA_INFO {
    cbSize: DWORD,
    iDeltaCrlIndicator: c_int,
    pftCacheResync: LPFILETIME,
    pLastSyncTime: LPFILETIME,
    pMaxAgeTime: LPFILETIME,
    pChainPara: PCERT_REVOCATION_CHAIN_PARA,
    pDeltaCrlIndicator: PCRYPT_INTEGER_BLOB,
}}
pub type PCRYPT_GET_TIME_VALID_OBJECT_EXTRA_INFO = *mut CRYPT_GET_TIME_VALID_OBJECT_EXTRA_INFO;
extern "system" {
    pub fn CryptGetTimeValidObject(
        pszTimeValidOid: LPCSTR,
        pvPara: LPVOID,
        pIssuer: PCCERT_CONTEXT,
        pftValidFor: LPFILETIME,
        dwFlags: DWORD,
        dwTimeout: DWORD,
        ppvObject: *mut LPVOID,
        pCredentials: PCRYPT_CREDENTIALS,
        pExtraInfo: PCRYPT_GET_TIME_VALID_OBJECT_EXTRA_INFO,
    ) -> BOOL;
}
pub const TIME_VALID_OID_GET_OBJECT_FUNC: &'static str = "TimeValidDllGetObject";
pub const TIME_VALID_OID_GET_CTL: LPCSTR = 1 as LPCSTR;
pub const TIME_VALID_OID_GET_CRL: LPCSTR = 2 as LPCSTR;
pub const TIME_VALID_OID_GET_CRL_FROM_CERT: LPCSTR = 3 as LPCSTR;
pub const TIME_VALID_OID_GET_FRESHEST_CRL_FROM_CERT: LPCSTR = 4 as LPCSTR;
pub const TIME_VALID_OID_GET_FRESHEST_CRL_FROM_CRL: LPCSTR = 5 as LPCSTR;
extern "system" {
    pub fn CryptFlushTimeValidObject(
        pszFlushTimeValidOid: LPCSTR,
        pvPara: LPVOID,
        pIssuer: PCCERT_CONTEXT,
        dwFlags: DWORD,
        pvReserved: LPVOID,
    ) -> BOOL;
}
pub const TIME_VALID_OID_FLUSH_OBJECT_FUNC: &'static str = "TimeValidDllFlushObject";
pub const TIME_VALID_OID_FLUSH_CTL: LPCSTR = 1 as LPCSTR;
pub const TIME_VALID_OID_FLUSH_CRL: LPCSTR = 2 as LPCSTR;
pub const TIME_VALID_OID_FLUSH_CRL_FROM_CERT: LPCSTR = 3 as LPCSTR;
pub const TIME_VALID_OID_FLUSH_FRESHEST_CRL_FROM_CERT: LPCSTR = 4 as LPCSTR;
pub const TIME_VALID_OID_FLUSH_FRESHEST_CRL_FROM_CRL: LPCSTR = 5 as LPCSTR;
extern "system" {
    pub fn CertCreateSelfSignCertificate(
        hCryptProvOrNCryptKey: HCRYPTPROV_OR_NCRYPT_KEY_HANDLE,
        pSubjectIssuerBlob: PCERT_NAME_BLOB,
        dwFlags: DWORD,
        pKeyProvInfo: PCRYPT_KEY_PROV_INFO,
        pSignatureAlgorithm: PCRYPT_ALGORITHM_IDENTIFIER,
        pStartTime: PSYSTEMTIME,
        pEndTime: PSYSTEMTIME,
        pExtensions: PCERT_EXTENSIONS,
    ) -> PCCERT_CONTEXT;
}
pub const CERT_CREATE_SELFSIGN_NO_SIGN: DWORD = 1;
pub const CERT_CREATE_SELFSIGN_NO_KEY_INFO: DWORD = 2;
extern "system" {
    pub fn CryptGetKeyIdentifierProperty(
        pKeyIdentifier: *const CRYPT_HASH_BLOB,
        dwPropId: DWORD,
        dwFlags: DWORD,
        pwszComputerName: LPCWSTR,
        pvReserved: *mut c_void,
        pvData: *mut c_void,
        pcbData: *mut DWORD,
    ) -> BOOL;
}
pub const CRYPT_KEYID_MACHINE_FLAG: DWORD = 0x00000020;
pub const CRYPT_KEYID_ALLOC_FLAG: DWORD = 0x00008000;
extern "system" {
    pub fn CryptSetKeyIdentifierProperty(
        pKeyIdentifier: *const CRYPT_HASH_BLOB,
        dwPropId: DWORD,
        dwFlags: DWORD,
        pwszComputerName: LPCWSTR,
        pvReserved: *mut c_void,
        pvData: *const c_void,
    ) -> BOOL;
}
pub const CRYPT_KEYID_DELETE_FLAG: DWORD = 0x00000010;
pub const CRYPT_KEYID_SET_NEW_FLAG: DWORD = 0x00002000;
FN!{stdcall PFN_CRYPT_ENUM_KEYID_PROP(
    pKeyIdentifier: *const CRYPT_HASH_BLOB,
    dwFlags: DWORD,
    pvReserved: *mut c_void,
    pvArg: *mut c_void,
    cProp: DWORD,
    rgdwPropId: *mut DWORD,
    rgpvData: *mut *mut c_void,
    rgcbData: *mut DWORD,
) -> BOOL}
extern "system" {
    pub fn CryptEnumKeyIdentifierProperties(
        pKeyIdentifier: *const CRYPT_HASH_BLOB,
        dwPropId: DWORD,
        dwFlags: DWORD,
        pwszComputerName: LPCWSTR,
        pvReserved: *mut c_void,
        pvArg: *mut c_void,
        pfnEnum: PFN_CRYPT_ENUM_KEYID_PROP,
    ) -> BOOL;
    pub fn CryptCreateKeyIdentifierFromCSP(
        dwCertEncodingType: DWORD,
        pszPubKeyOID: LPCSTR,
        pPubKeyStruc: *const PUBLICKEYSTRUC,
        cbPubKeyStruc: DWORD,
        dwFlags: DWORD,
        pvReserved: *mut c_void,
        pbHash: *mut BYTE,
        pcbHash: *mut DWORD,
    ) -> BOOL;
}
pub const CERT_CHAIN_CONFIG_REGPATH: &'static str
    = "Software\\Microsoft\\Cryptography\\OID\\EncodingType 0\\CertDllCreateCertificateChainEngine\\Config";
pub const CERT_CHAIN_MAX_URL_RETRIEVAL_BYTE_COUNT_VALUE_NAME: &'static str
    = "MaxUrlRetrievalByteCount";
pub const CERT_CHAIN_MAX_URL_RETRIEVAL_BYTE_COUNT_DEFAULT: DWORD = 100 * 1024 * 1024;
pub const CERT_CHAIN_CACHE_RESYNC_FILETIME_VALUE_NAME: &'static str = "ChainCacheResyncFiletime";
pub const CERT_CHAIN_DISABLE_MANDATORY_BASIC_CONSTRAINTS_VALUE_NAME: &'static str
    = "DisableMandatoryBasicConstraints";
pub const CERT_CHAIN_DISABLE_CA_NAME_CONSTRAINTS_VALUE_NAME: &'static str
    = "DisableCANameConstraints";
pub const CERT_CHAIN_DISABLE_UNSUPPORTED_CRITICAL_EXTENSIONS_VALUE_NAME: &'static str
    = "DisableUnsupportedCriticalExtensions";
pub const CERT_CHAIN_MAX_AIA_URL_COUNT_IN_CERT_VALUE_NAME: &'static str = "MaxAIAUrlCountInCert";
pub const CERT_CHAIN_MAX_AIA_URL_COUNT_IN_CERT_DEFAULT: DWORD = 5;
pub const CERT_CHAIN_MAX_AIA_URL_RETRIEVAL_COUNT_PER_CHAIN_VALUE_NAME: &'static str
    = "MaxAIAUrlRetrievalCountPerChain";
pub const CERT_CHAIN_MAX_AIA_URL_RETRIEVAL_COUNT_PER_CHAIN_DEFAULT: DWORD = 3;
pub const CERT_CHAIN_MAX_AIA_URL_RETRIEVAL_BYTE_COUNT_VALUE_NAME: &'static str
    = "MaxAIAUrlRetrievalByteCount";
pub const CERT_CHAIN_MAX_AIA_URL_RETRIEVAL_BYTE_COUNT_DEFAULT: DWORD = 100000;
pub const CERT_CHAIN_MAX_AIA_URL_RETRIEVAL_CERT_COUNT_VALUE_NAME: &'static str
    = "MaxAIAUrlRetrievalCertCount";
pub const CERT_CHAIN_MAX_AIA_URL_RETRIEVAL_CERT_COUNT_DEFAULT: DWORD = 10;
pub const CERT_CHAIN_OCSP_VALIDITY_SECONDS_VALUE_NAME: &'static str
    = "OcspValiditySeconds";
pub const CERT_CHAIN_OCSP_VALIDITY_SECONDS_DEFAULT: DWORD = 12 * 60 * 60;
pub const CERT_CHAIN_DISABLE_SERIAL_CHAIN_VALUE_NAME: &'static str = "DisableSerialChain";
pub const CERT_CHAIN_SERIAL_CHAIN_LOG_FILE_NAME_VALUE_NAME: &'static str
    = "SerialChainLogFileName";
pub const CERT_CHAIN_DISABLE_SYNC_WITH_SSL_TIME_VALUE_NAME: &'static str
    = "DisableSyncWithSslTime";
pub const CERT_CHAIN_MAX_SSL_TIME_UPDATED_EVENT_COUNT_VALUE_NAME: &'static str
    = "MaxSslTimeUpdatedEventCount";
pub const CERT_CHAIN_MAX_SSL_TIME_UPDATED_EVENT_COUNT_DEFAULT: DWORD = 5;
pub const CERT_CHAIN_MAX_SSL_TIME_UPDATED_EVENT_COUNT_DISABLE: DWORD = 0xFFFFFFFF;
pub const CERT_CHAIN_SSL_HANDSHAKE_LOG_FILE_NAME_VALUE_NAME: &'static str
    = "SslHandshakeLogFileName";
pub const CERT_CHAIN_ENABLE_WEAK_SIGNATURE_FLAGS_VALUE_NAME: &'static str
    = "EnableWeakSignatureFlags";
pub const CERT_CHAIN_ENABLE_MD2_MD4_FLAG: DWORD = 0x00000001;
pub const CERT_CHAIN_ENABLE_WEAK_RSA_ROOT_FLAG: DWORD = 0x00000002;
pub const CERT_CHAIN_ENABLE_WEAK_LOGGING_FLAG: DWORD = 0x00000004;
pub const CERT_CHAIN_ENABLE_ONLY_WEAK_LOGGING_FLAG: DWORD = 0x00000008;
pub const CERT_CHAIN_MIN_RSA_PUB_KEY_BIT_LENGTH_VALUE_NAME: &'static str = "MinRsaPubKeyBitLength";
pub const CERT_CHAIN_MIN_RSA_PUB_KEY_BIT_LENGTH_DEFAULT: DWORD = 1023;
pub const CERT_CHAIN_MIN_RSA_PUB_KEY_BIT_LENGTH_DISABLE: DWORD = 0xFFFFFFFF;
pub const CERT_CHAIN_WEAK_RSA_PUB_KEY_TIME_VALUE_NAME: &'static str = "WeakRsaPubKeyTime";
pub const CERT_CHAIN_WEAK_RSA_PUB_KEY_TIME_DEFAULT: u64 = 0x01CA8A755C6E0000;
pub const CERT_CHAIN_WEAK_SIGNATURE_LOG_DIR_VALUE_NAME: &'static str = "WeakSignatureLogDir";
pub const CERT_CHAIN_DEFAULT_CONFIG_SUBDIR: &'static str = "Default";
pub const CERT_CHAIN_WEAK_PREFIX_NAME: &'static str = "Weak";
pub const CERT_CHAIN_WEAK_THIRD_PARTY_CONFIG_NAME: &'static str = "ThirdParty";
pub const CERT_CHAIN_WEAK_ALL_CONFIG_NAME: &'static str = "Al";
pub const CERT_CHAIN_WEAK_FLAGS_NAME: &'static str = "Flags";
pub const CERT_CHAIN_WEAK_HYGIENE_NAME: &'static str = "Hygiene";
pub const CERT_CHAIN_WEAK_AFTER_TIME_NAME: &'static str = "AfterTime";
pub const CERT_CHAIN_WEAK_FILE_HASH_AFTER_TIME_NAME: &'static str = "FileHashAfterTime";
pub const CERT_CHAIN_WEAK_TIMESTAMP_HASH_AFTER_TIME_NAME: &'static str = "TimestampHashAfterTime";
pub const CERT_CHAIN_WEAK_MIN_BIT_LENGTH_NAME: &'static str = "MinBitLength";
pub const CERT_CHAIN_WEAK_SHA256_ALLOW_NAME: &'static str = "Sha256Allow";
pub const CERT_CHAIN_MIN_PUB_KEY_BIT_LENGTH_DISABLE: DWORD = 0xFFFFFFFF;
pub const CERT_CHAIN_ENABLE_WEAK_SETTINGS_FLAG: DWORD = 0x80000000;
pub const CERT_CHAIN_DISABLE_ALL_EKU_WEAK_FLAG: DWORD = 0x00010000;
pub const CERT_CHAIN_ENABLE_ALL_EKU_HYGIENE_FLAG: DWORD = 0x00020000;
pub const CERT_CHAIN_DISABLE_OPT_IN_SERVER_AUTH_WEAK_FLAG: DWORD = 0x00040000;
pub const CERT_CHAIN_DISABLE_SERVER_AUTH_WEAK_FLAG: DWORD = 0x00100000;
pub const CERT_CHAIN_ENABLE_SERVER_AUTH_HYGIENE_FLAG: DWORD = 0x00200000;
pub const CERT_CHAIN_DISABLE_CODE_SIGNING_WEAK_FLAG: DWORD = 0x00400000;
pub const CERT_CHAIN_DISABLE_MOTW_CODE_SIGNING_WEAK_FLAG: DWORD = 0x00800000;
pub const CERT_CHAIN_ENABLE_CODE_SIGNING_HYGIENE_FLAG: DWORD = 0x01000000;
pub const CERT_CHAIN_ENABLE_MOTW_CODE_SIGNING_HYGIENE_FLAG: DWORD = 0x02000000;
pub const CERT_CHAIN_DISABLE_TIMESTAMP_WEAK_FLAG: DWORD = 0x04000000;
pub const CERT_CHAIN_DISABLE_MOTW_TIMESTAMP_WEAK_FLAG: DWORD = 0x08000000;
pub const CERT_CHAIN_ENABLE_TIMESTAMP_HYGIENE_FLAG: DWORD = 0x10000000;
pub const CERT_CHAIN_ENABLE_MOTW_TIMESTAMP_HYGIENE_FLAG: DWORD = 0x20000000;
pub const CERT_CHAIN_MOTW_IGNORE_AFTER_TIME_WEAK_FLAG: DWORD = 0x40000000;
pub const CERT_CHAIN_DISABLE_FILE_HASH_WEAK_FLAG: DWORD = 0x00001000;
pub const CERT_CHAIN_DISABLE_MOTW_FILE_HASH_WEAK_FLAG: DWORD = 0x00002000;
pub const CERT_CHAIN_DISABLE_TIMESTAMP_HASH_WEAK_FLAG: DWORD = 0x00004000;
pub const CERT_CHAIN_DISABLE_MOTW_TIMESTAMP_HASH_WEAK_FLAG: DWORD = 0x00008000;
pub const CERT_CHAIN_DISABLE_WEAK_FLAGS: DWORD = CERT_CHAIN_DISABLE_ALL_EKU_WEAK_FLAG
    | CERT_CHAIN_DISABLE_SERVER_AUTH_WEAK_FLAG | CERT_CHAIN_DISABLE_OPT_IN_SERVER_AUTH_WEAK_FLAG
    | CERT_CHAIN_DISABLE_CODE_SIGNING_WEAK_FLAG | CERT_CHAIN_DISABLE_MOTW_CODE_SIGNING_WEAK_FLAG
    | CERT_CHAIN_DISABLE_TIMESTAMP_WEAK_FLAG | CERT_CHAIN_DISABLE_MOTW_TIMESTAMP_WEAK_FLAG;
pub const CERT_CHAIN_DISABLE_FILE_HASH_WEAK_FLAGS: DWORD = CERT_CHAIN_DISABLE_FILE_HASH_WEAK_FLAG
    | CERT_CHAIN_DISABLE_MOTW_FILE_HASH_WEAK_FLAG;
pub const CERT_CHAIN_DISABLE_TIMESTAMP_HASH_WEAK_FLAGS: DWORD
    = CERT_CHAIN_DISABLE_TIMESTAMP_HASH_WEAK_FLAG
    | CERT_CHAIN_DISABLE_MOTW_TIMESTAMP_HASH_WEAK_FLAG;
pub const CERT_CHAIN_ENABLE_HYGIENE_FLAGS: DWORD = CERT_CHAIN_ENABLE_ALL_EKU_HYGIENE_FLAG
    | CERT_CHAIN_ENABLE_SERVER_AUTH_HYGIENE_FLAG | CERT_CHAIN_ENABLE_CODE_SIGNING_HYGIENE_FLAG
    | CERT_CHAIN_ENABLE_MOTW_CODE_SIGNING_HYGIENE_FLAG | CERT_CHAIN_ENABLE_TIMESTAMP_HYGIENE_FLAG
    | CERT_CHAIN_ENABLE_MOTW_TIMESTAMP_HYGIENE_FLAG;
pub const CERT_CHAIN_MOTW_WEAK_FLAGS: DWORD = CERT_CHAIN_DISABLE_MOTW_CODE_SIGNING_WEAK_FLAG
    | CERT_CHAIN_DISABLE_MOTW_TIMESTAMP_WEAK_FLAG
    | CERT_CHAIN_ENABLE_MOTW_CODE_SIGNING_HYGIENE_FLAG
    | CERT_CHAIN_ENABLE_MOTW_TIMESTAMP_HYGIENE_FLAG | CERT_CHAIN_MOTW_IGNORE_AFTER_TIME_WEAK_FLAG;
pub const CERT_CHAIN_OPT_IN_WEAK_FLAGS: DWORD = CERT_CHAIN_DISABLE_OPT_IN_SERVER_AUTH_WEAK_FLAG;
pub const CERT_CHAIN_AUTO_CURRENT_USER: DWORD = 1;
pub const CERT_CHAIN_AUTO_LOCAL_MACHINE: DWORD = 2;
pub const CERT_CHAIN_AUTO_IMPERSONATED: DWORD = 3;
pub const CERT_CHAIN_AUTO_PROCESS_INFO: DWORD = 4;
pub const CERT_CHAIN_AUTO_PINRULE_INFO: DWORD = 5;
pub const CERT_CHAIN_AUTO_NETWORK_INFO: DWORD = 6;
pub const CERT_CHAIN_AUTO_SERIAL_LOCAL_MACHINE: DWORD = 7;
pub const CERT_CHAIN_AUTO_HPKP_RULE_INFO: DWORD = 8;
pub const CERT_CHAIN_AUTO_FLAGS_VALUE_NAME: &'static str = "AutoFlags";
pub const CERT_CHAIN_AUTO_FLUSH_DISABLE_FLAG: DWORD = 0x00000001;
pub const CERT_CHAIN_AUTO_LOG_CREATE_FLAG: DWORD = 0x00000002;
pub const CERT_CHAIN_AUTO_LOG_FREE_FLAG: DWORD = 0x00000004;
pub const CERT_CHAIN_AUTO_LOG_FLUSH_FLAG: DWORD = 0x00000008;
pub const CERT_CHAIN_AUTO_LOG_FLAGS: DWORD = CERT_CHAIN_AUTO_LOG_CREATE_FLAG
    | CERT_CHAIN_AUTO_LOG_FREE_FLAG | CERT_CHAIN_AUTO_LOG_FLUSH_FLAG;
pub const CERT_CHAIN_AUTO_FLUSH_FIRST_DELTA_SECONDS_VALUE_NAME: &'static str
    = "AutoFlushFirstDeltaSeconds";
pub const CERT_CHAIN_AUTO_FLUSH_FIRST_DELTA_SECONDS_DEFAULT: DWORD = 5 * 60;
pub const CERT_CHAIN_AUTO_FLUSH_NEXT_DELTA_SECONDS_VALUE_NAME: &'static str
    = "AutoFlushNextDeltaSeconds";
pub const CERT_CHAIN_AUTO_FLUSH_NEXT_DELTA_SECONDS_DEFAULT: DWORD = 30 * 60;
pub const CERT_CHAIN_AUTO_LOG_FILE_NAME_VALUE_NAME: &'static str = "AutoLogFileName";
pub const CERT_CHAIN_DISABLE_AUTO_FLUSH_PROCESS_NAME_LIST_VALUE_NAME: &'static str
    = "DisableAutoFlushProcessNameList";
pub const CERT_SRV_OCSP_RESP_MIN_VALIDITY_SECONDS_VALUE_NAME: &'static str
    = "SrvOcspRespMinValiditySeconds";
pub const CERT_SRV_OCSP_RESP_MIN_VALIDITY_SECONDS_DEFAULT: DWORD = 10 * 60;
pub const CERT_SRV_OCSP_RESP_URL_RETRIEVAL_TIMEOUT_MILLISECONDS_VALUE_NAME: &'static str
    = "SrvOcspRespUrlRetrievalTimeoutMilliseconds";
pub const CERT_SRV_OCSP_RESP_URL_RETRIEVAL_TIMEOUT_MILLISECONDS_DEFAULT: DWORD = 15 * 1000;
pub const CERT_SRV_OCSP_RESP_MAX_BEFORE_NEXT_UPDATE_SECONDS_VALUE_NAME: &'static str
    = "SrvOcspRespMaxBeforeNextUpdateSeconds";
pub const CERT_SRV_OCSP_RESP_MAX_BEFORE_NEXT_UPDATE_SECONDS_DEFAULT: DWORD = 4 * 60 * 60;
pub const CERT_SRV_OCSP_RESP_MIN_BEFORE_NEXT_UPDATE_SECONDS_VALUE_NAME: &'static str
    = "SrvOcspRespMinBeforeNextUpdateSeconds";
pub const CERT_SRV_OCSP_RESP_MIN_BEFORE_NEXT_UPDATE_SECONDS_DEFAULT: DWORD = 2 * 60;
pub const CERT_SRV_OCSP_RESP_MIN_AFTER_NEXT_UPDATE_SECONDS_VALUE_NAME: &'static str
    = "SrvOcspRespMinAfterNextUpdateSeconds";
pub const CERT_SRV_OCSP_RESP_MIN_AFTER_NEXT_UPDATE_SECONDS_DEFAULT: DWORD = 1 * 60;
pub const CERT_SRV_OCSP_RESP_MIN_SYNC_CERT_FILE_SECONDS_VALUE_NAME: &'static str
    = "SrvOcspRespMinSyncCertFileSeconds";
pub const CERT_SRV_OCSP_RESP_MIN_SYNC_CERT_FILE_SECONDS_DEFAULT: DWORD = 5;
pub const CERT_SRV_OCSP_RESP_MAX_SYNC_CERT_FILE_SECONDS_VALUE_NAME: &'static str
    = "SrvOcspRespMaxSyncCertFileSeconds";
pub const CERT_SRV_OCSP_RESP_MAX_SYNC_CERT_FILE_SECONDS_DEFAULT: DWORD = 1 * 60 * 60;
pub const CRYPTNET_MAX_CACHED_OCSP_PER_CRL_COUNT_VALUE_NAME: &'static str
    = "CryptnetMaxCachedOcspPerCrlCount";
pub const CRYPTNET_MAX_CACHED_OCSP_PER_CRL_COUNT_DEFAULT: DWORD = 500;
pub const CRYPTNET_OCSP_AFTER_CRL_DISABLE: DWORD = 0xFFFFFFFF;
pub const CRYPTNET_URL_CACHE_DEFAULT_FLUSH_EXEMPT_SECONDS_VALUE_NAME: &'static str
    = "CryptnetDefaultFlushExemptSeconds";
pub const CRYPTNET_URL_CACHE_DEFAULT_FLUSH_EXEMPT_SECONDS_DEFAULT: DWORD = 28 * 24 * 60 * 60;
pub const CRYPTNET_PRE_FETCH_MIN_MAX_AGE_SECONDS_VALUE_NAME: &'static str
    = "CryptnetPreFetchMinMaxAgeSeconds";
pub const CRYPTNET_PRE_FETCH_MIN_MAX_AGE_SECONDS_DEFAULT: DWORD = 1 * 60 * 60;
pub const CRYPTNET_PRE_FETCH_MAX_MAX_AGE_SECONDS_VALUE_NAME: &'static str
    = "CryptnetPreFetchMaxMaxAgeSeconds";
pub const CRYPTNET_PRE_FETCH_MAX_MAX_AGE_SECONDS_DEFAULT: DWORD = 14 * 24 * 60 * 60;
pub const CRYPTNET_PRE_FETCH_MIN_OCSP_VALIDITY_PERIOD_SECONDS_VALUE_NAME: &'static str
    = "CryptnetPreFetchMinOcspValidityPeriodSeconds";
pub const CRYPTNET_PRE_FETCH_MIN_OCSP_VALIDITY_PERIOD_SECONDS_DEFAULT: DWORD = 14 * 24 * 60 * 60;
pub const CRYPTNET_PRE_FETCH_AFTER_PUBLISH_PRE_FETCH_DIVISOR_VALUE_NAME: &'static str
    = "CryptnetPreFetchAfterPublishPreFetchDivisor";
pub const CRYPTNET_PRE_FETCH_AFTER_PUBLISH_PRE_FETCH_DIVISOR_DEFAULT: DWORD = 10;
pub const CRYPTNET_PRE_FETCH_BEFORE_NEXT_UPDATE_PRE_FETCH_DIVISOR_VALUE_NAME: &'static str
    = "CryptnetPreFetchBeforeNextUpdatePreFetchDivisor";
pub const CRYPTNET_PRE_FETCH_BEFORE_NEXT_UPDATE_PRE_FETCH_DIVISOR_DEFAULT: DWORD = 20;
pub const CRYPTNET_PRE_FETCH_MIN_BEFORE_NEXT_UPDATE_PRE_FETCH_PERIOD_SECONDS_VALUE_NAME:
    &'static str = "CryptnetPreFetchMinBeforeNextUpdatePreFetchSeconds";
pub const CRYPTNET_PRE_FETCH_MIN_BEFORE_NEXT_UPDATE_PRE_FETCH_PERIOD_SECONDS_DEFAULT: DWORD
    = 1 * 60 * 60;
pub const CRYPTNET_PRE_FETCH_VALIDITY_PERIOD_AFTER_NEXT_UPDATE_PRE_FETCH_DIVISOR_VALUE_NAME:
    &'static str = "CryptnetPreFetchValidityPeriodAfterNextUpdatePreFetchDivisor";
pub const CRYPTNET_PRE_FETCH_VALIDITY_PERIOD_AFTER_NEXT_UPDATE_PRE_FETCH_DIVISOR_DEFAULT: DWORD
    = 10;
pub const CRYPTNET_PRE_FETCH_MAX_AFTER_NEXT_UPDATE_PRE_FETCH_PERIOD_SECONDS_VALUE_NAME:
    &'static str = "CryptnetPreFetchMaxAfterNextUpdatePreFetchPeriodSeconds";
pub const CRYPTNET_PRE_FETCH_MAX_AFTER_NEXT_UPDATE_PRE_FETCH_PERIOD_SECONDS_DEFAULT: DWORD
    = 4 * 60 * 60;
pub const CRYPTNET_PRE_FETCH_MIN_AFTER_NEXT_UPDATE_PRE_FETCH_PERIOD_SECONDS_VALUE_NAME:
    &'static str = "CryptnetPreFetchMinAfterNextUpdatePreFetchPeriodSeconds";
pub const CRYPTNET_PRE_FETCH_MIN_AFTER_NEXT_UPDATE_PRE_FETCH_PERIOD_SECONDS_DEFAULT: DWORD
    = 30 * 60;
pub const CRYPTNET_PRE_FETCH_AFTER_CURRENT_TIME_PRE_FETCH_PERIOD_SECONDS_VALUE_NAME: &'static str
    = "CryptnetPreFetchAfterCurrentTimePreFetchPeriodSeconds";
pub const CRYPTNET_PRE_FETCH_AFTER_CURRENT_TIME_PRE_FETCH_PERIOD_SECONDS_DEFAULT: DWORD
    = 30 * 60;
pub const CRYPTNET_PRE_FETCH_TRIGGER_PERIOD_SECONDS_VALUE_NAME: &'static str
    = "CryptnetPreFetchTriggerPeriodSeconds";
pub const CRYPTNET_PRE_FETCH_TRIGGER_PERIOD_SECONDS_DEFAULT: DWORD = 10 * 60;
pub const CRYPTNET_PRE_FETCH_TRIGGER_DISABLE: DWORD = 0xFFFFFFFF;
pub const CRYPTNET_PRE_FETCH_SCAN_AFTER_TRIGGER_DELAY_SECONDS_VALUE_NAME: &'static str
    = "CryptnetPreFetchScanAfterTriggerDelaySeconds";
pub const CRYPTNET_PRE_FETCH_SCAN_AFTER_TRIGGER_DELAY_SECONDS_DEFAULT: DWORD = 60;
pub const CRYPTNET_PRE_FETCH_RETRIEVAL_TIMEOUT_SECONDS_VALUE_NAME: &'static str
    = "CryptnetPreFetchRetrievalTimeoutSeconds";
pub const CRYPTNET_PRE_FETCH_RETRIEVAL_TIMEOUT_SECONDS_DEFAULT: DWORD = 5 * 60;
pub const CRYPTNET_CRL_PRE_FETCH_CONFIG_REGPATH: &'static str
    = "Software\\Microsoft\\Cryptography\\OID\\EncodingType 0\\CertDllCreateCertificateChainEngine\\Config\\CrlPreFetch";
pub const CRYPTNET_CRL_PRE_FETCH_PROCESS_NAME_LIST_VALUE_NAME: &'static str = "ProcessNameList";
pub const CRYPTNET_CRL_PRE_FETCH_URL_LIST_VALUE_NAME: &'static str = "PreFetchUrlList";
pub const CRYPTNET_CRL_PRE_FETCH_DISABLE_INFORMATION_EVENTS_VALUE_NAME: &'static str
    = "DisableInformationEvents";
pub const CRYPTNET_CRL_PRE_FETCH_LOG_FILE_NAME_VALUE_NAME: &'static str = "LogFileName";
pub const CRYPTNET_CRL_PRE_FETCH_TIMEOUT_SECONDS_VALUE_NAME: &'static str = "TimeoutSeconds";
pub const CRYPTNET_CRL_PRE_FETCH_TIMEOUT_SECONDS_DEFAULT: DWORD = 5 * 60;
pub const CRYPTNET_CRL_PRE_FETCH_MAX_AGE_SECONDS_VALUE_NAME: &'static str = "MaxAgeSeconds";
pub const CRYPTNET_CRL_PRE_FETCH_MAX_AGE_SECONDS_DEFAULT: DWORD = 2 * 60 * 60;
pub const CRYPTNET_CRL_PRE_FETCH_MAX_AGE_SECONDS_MIN: DWORD = 5 * 60;
pub const CRYPTNET_CRL_PRE_FETCH_PUBLISH_BEFORE_NEXT_UPDATE_SECONDS_VALUE_NAME: &'static str
    = "PublishBeforeNextUpdateSeconds";
pub const CRYPTNET_CRL_PRE_FETCH_PUBLISH_BEFORE_NEXT_UPDATE_SECONDS_DEFAULT: DWORD = 1 * 60 * 60;
pub const CRYPTNET_CRL_PRE_FETCH_PUBLISH_RANDOM_INTERVAL_SECONDS_VALUE_NAME: &'static str
    = "PublishRandomIntervalSeconds";
pub const CRYPTNET_CRL_PRE_FETCH_PUBLISH_RANDOM_INTERVAL_SECONDS_DEFAULT: DWORD = 5 * 60;
pub const CRYPTNET_CRL_PRE_FETCH_MIN_BEFORE_NEXT_UPDATE_SECONDS_VALUE_NAME: &'static str
    = "MinBeforeNextUpdateSeconds";
pub const CRYPTNET_CRL_PRE_FETCH_MIN_BEFORE_NEXT_UPDATE_SECONDS_DEFAULT: DWORD = 5 * 60;
pub const CRYPTNET_CRL_PRE_FETCH_MIN_AFTER_NEXT_UPDATE_SECONDS_VALUE_NAME: &'static str
    = "MinAfterNextUpdateSeconds";
pub const CRYPTNET_CRL_PRE_FETCH_MIN_AFTER_NEXT_UPDATE_SECONDS_DEFAULT: DWORD = 5 * 60;
pub const CERT_GROUP_POLICY_CHAIN_CONFIG_REGPATH: &'static str
    = "Software\\Policies\\Microsoft\\SystemCertificates\\ChainEngine\\Config";
pub const CERT_CHAIN_URL_RETRIEVAL_TIMEOUT_MILLISECONDS_VALUE_NAME: &'static str
    = "ChainUrlRetrievalTimeoutMilliseconds";
pub const CERT_CHAIN_URL_RETRIEVAL_TIMEOUT_MILLISECONDS_DEFAULT: DWORD = 15 * 1000;
pub const CERT_CHAIN_REV_ACCUMULATIVE_URL_RETRIEVAL_TIMEOUT_MILLISECONDS_VALUE_NAME: &'static str
    = "ChainRevAccumulativeUrlRetrievalTimeoutMilliseconds";
pub const CERT_CHAIN_REV_ACCUMULATIVE_URL_RETRIEVAL_TIMEOUT_MILLISECONDS_DEFAULT: DWORD
    = 20 * 1000;
pub const CERT_RETR_BEHAVIOR_INET_AUTH_VALUE_NAME: &'static str = "EnableInetUnknownAuth";
pub const CERT_RETR_BEHAVIOR_INET_STATUS_VALUE_NAME: &'static str = "EnableInetLocal";
pub const CERT_RETR_BEHAVIOR_FILE_VALUE_NAME: &'static str = "AllowFileUrlScheme";
pub const CERT_RETR_BEHAVIOR_LDAP_VALUE_NAME: &'static str = "DisableLDAPSignAndEncrypt";
pub const CRYPTNET_CACHED_OCSP_SWITCH_TO_CRL_COUNT_VALUE_NAME: &'static str
    = "CryptnetCachedOcspSwitchToCrlCount";
pub const CRYPTNET_CACHED_OCSP_SWITCH_TO_CRL_COUNT_DEFAULT: DWORD = 50;
pub const CRYPTNET_CRL_BEFORE_OCSP_ENABLE: DWORD = 0xFFFFFFFF;
pub const CERT_CHAIN_DISABLE_AIA_URL_RETRIEVAL_VALUE_NAME: &'static str = "DisableAIAUrlRetrieval";
pub const CERT_CHAIN_OPTIONS_VALUE_NAME: &'static str = "Options";
pub const CERT_CHAIN_OPTION_DISABLE_AIA_URL_RETRIEVAL: DWORD = 0x2;
pub const CERT_CHAIN_OPTION_ENABLE_SIA_URL_RETRIEVAL: DWORD = 0x4;
pub const CERT_CHAIN_CROSS_CERT_DOWNLOAD_INTERVAL_HOURS_VALUE_NAME: &'static str
    = "CrossCertDownloadIntervalHours";
pub const CERT_CHAIN_CROSS_CERT_DOWNLOAD_INTERVAL_HOURS_DEFAULT: DWORD = 24 * 7;
pub const CERT_CHAIN_CRL_VALIDITY_EXT_PERIOD_HOURS_VALUE_NAME: &'static str
    = "CRLValidityExtensionPeriod";
pub const CERT_CHAIN_CRL_VALIDITY_EXT_PERIOD_HOURS_DEFAULT: DWORD = 12;
pub type HCERTCHAINENGINE = HANDLE;
pub const HCCE_CURRENT_USER: HCERTCHAINENGINE = 0 as HCERTCHAINENGINE;
pub const HCCE_LOCAL_MACHINE: HCERTCHAINENGINE = 0x1 as HCERTCHAINENGINE;
pub const HCCE_SERIAL_LOCAL_MACHINE: HCERTCHAINENGINE = 0x2 as HCERTCHAINENGINE;
pub const CERT_CHAIN_CACHE_END_CERT: DWORD = 0x00000001;
pub const CERT_CHAIN_THREAD_STORE_SYNC: DWORD = 0x00000002;
pub const CERT_CHAIN_CACHE_ONLY_URL_RETRIEVAL: DWORD = 0x00000004;
pub const CERT_CHAIN_USE_LOCAL_MACHINE_STORE: DWORD = 0x00000008;
pub const CERT_CHAIN_ENABLE_CACHE_AUTO_UPDATE: DWORD = 0x00000010;
pub const CERT_CHAIN_ENABLE_SHARE_STORE: DWORD = 0x00000020;
STRUCT!{struct CERT_CHAIN_ENGINE_CONFIG {
    cbSize: DWORD,
    hRestrictedRoot: HCERTSTORE,
    hRestrictedTrust: HCERTSTORE,
    hRestrictedOther: HCERTSTORE,
    cAdditionalStore: DWORD,
    rghAdditionalStore: *mut HCERTSTORE,
    dwFlags: DWORD,
    dwUrlRetrievalTimeout: DWORD,
    MaximumCachedCertificates: DWORD,
    CycleDetectionModulus: DWORD,
    hExclusiveRoot: HCERTSTORE,
    hExclusiveTrustedPeople: HCERTSTORE,
    dwExclusiveFlags: DWORD,
}}
pub type PCERT_CHAIN_ENGINE_CONFIG = *mut CERT_CHAIN_ENGINE_CONFIG;
extern "system" {
    pub fn CertCreateCertificateChainEngine(
        pConfig: PCERT_CHAIN_ENGINE_CONFIG,
        phChainEngine: *mut HCERTCHAINENGINE,
    ) -> BOOL;
    pub fn CertFreeCertificateChainEngine(
        hChainEngine: HCERTCHAINENGINE,
    );
    pub fn CertResyncCertificateChainEngine(
        hChainEngine: HCERTCHAINENGINE,
    ) -> BOOL;
}
STRUCT!{struct CERT_TRUST_STATUS {
    dwErrorStatus: DWORD,
    dwInfoStatus: DWORD,
}}
pub type PCERT_TRUST_STATUS = *mut CERT_TRUST_STATUS;
pub const CERT_TRUST_NO_ERROR: DWORD = 0x00000000;
pub const CERT_TRUST_IS_NOT_TIME_VALID: DWORD = 0x00000001;
pub const CERT_TRUST_IS_NOT_TIME_NESTED: DWORD = 0x00000002;
pub const CERT_TRUST_IS_REVOKED: DWORD = 0x00000004;
pub const CERT_TRUST_IS_NOT_SIGNATURE_VALID: DWORD = 0x00000008;
pub const CERT_TRUST_IS_NOT_VALID_FOR_USAGE: DWORD = 0x00000010;
pub const CERT_TRUST_IS_UNTRUSTED_ROOT: DWORD = 0x00000020;
pub const CERT_TRUST_REVOCATION_STATUS_UNKNOWN: DWORD = 0x00000040;
pub const CERT_TRUST_IS_CYCLIC: DWORD = 0x00000080;
pub const CERT_TRUST_INVALID_EXTENSION: DWORD = 0x00000100;
pub const CERT_TRUST_INVALID_POLICY_CONSTRAINTS: DWORD = 0x00000200;
pub const CERT_TRUST_INVALID_BASIC_CONSTRAINTS: DWORD = 0x00000400;
pub const CERT_TRUST_INVALID_NAME_CONSTRAINTS: DWORD = 0x00000800;
pub const CERT_TRUST_HAS_NOT_SUPPORTED_NAME_CONSTRAINT: DWORD = 0x00001000;
pub const CERT_TRUST_HAS_NOT_DEFINED_NAME_CONSTRAINT: DWORD = 0x00002000;
pub const CERT_TRUST_HAS_NOT_PERMITTED_NAME_CONSTRAINT: DWORD = 0x00004000;
pub const CERT_TRUST_HAS_EXCLUDED_NAME_CONSTRAINT: DWORD = 0x00008000;
pub const CERT_TRUST_IS_OFFLINE_REVOCATION: DWORD = 0x01000000;
pub const CERT_TRUST_NO_ISSUANCE_CHAIN_POLICY: DWORD = 0x02000000;
pub const CERT_TRUST_IS_PARTIAL_CHAIN: DWORD = 0x00010000;
pub const CERT_TRUST_CTL_IS_NOT_TIME_VALID: DWORD = 0x00020000;
pub const CERT_TRUST_CTL_IS_NOT_SIGNATURE_VALID: DWORD = 0x00040000;
pub const CERT_TRUST_CTL_IS_NOT_VALID_FOR_USAGE: DWORD = 0x00080000;
pub const CERT_TRUST_HAS_EXACT_MATCH_ISSUER: DWORD = 0x00000001;
pub const CERT_TRUST_HAS_KEY_MATCH_ISSUER: DWORD = 0x00000002;
pub const CERT_TRUST_HAS_NAME_MATCH_ISSUER: DWORD = 0x00000004;
pub const CERT_TRUST_IS_SELF_SIGNED: DWORD = 0x00000008;
pub const CERT_TRUST_AUTO_UPDATE_CA_REVOCATION: DWORD = 0x00000010;
pub const CERT_TRUST_AUTO_UPDATE_END_REVOCATION: DWORD = 0x00000020;
pub const CERT_TRUST_NO_OCSP_FAILOVER_TO_CRL: DWORD = 0x00000040;
pub const CERT_TRUST_IS_KEY_ROLLOVER: DWORD = 0x00000080;
pub const CERT_TRUST_SSL_HANDSHAKE_OCSP: DWORD = 0x00040000;
pub const CERT_TRUST_SSL_TIME_VALID_OCSP: DWORD = 0x00080000;
pub const CERT_TRUST_SSL_RECONNECT_OCSP: DWORD = 0x00100000;
pub const CERT_TRUST_HAS_PREFERRED_ISSUER: DWORD = 0x00000100;
pub const CERT_TRUST_HAS_ISSUANCE_CHAIN_POLICY: DWORD = 0x00000200;
pub const CERT_TRUST_HAS_VALID_NAME_CONSTRAINTS: DWORD = 0x00000400;
pub const CERT_TRUST_IS_PEER_TRUSTED: DWORD = 0x00000800;
pub const CERT_TRUST_HAS_CRL_VALIDITY_EXTENDED: DWORD = 0x00001000;
pub const CERT_TRUST_IS_FROM_EXCLUSIVE_TRUST_STORE: DWORD = 0x00002000;
pub const CERT_TRUST_IS_CA_TRUSTED: DWORD = 0x00004000;
pub const CERT_TRUST_HAS_AUTO_UPDATE_WEAK_SIGNATURE: DWORD = 0x00008000;
pub const CERT_TRUST_HAS_ALLOW_WEAK_SIGNATURE: DWORD = 0x00020000;
pub const CERT_TRUST_IS_COMPLEX_CHAIN: DWORD = 0x00010000;
pub const CERT_TRUST_SSL_TIME_VALID: DWORD = 0x01000000;
pub const CERT_TRUST_NO_TIME_CHECK: DWORD = 0x02000000;
STRUCT!{struct CERT_REVOCATION_INFO {
    cbSize: DWORD,
    dwRevocationResult: DWORD,
    pszRevocationOid: LPCSTR,
    pvOidSpecificInfo: LPVOID,
    fHasFreshnessTime: BOOL,
    dwFreshnessTime: DWORD,
    pCrlInfo: PCERT_REVOCATION_CRL_INFO,
}}
pub type PCERT_REVOCATION_INFO = *mut CERT_REVOCATION_INFO;
STRUCT!{struct CERT_TRUST_LIST_INFO {
    cbSize: DWORD,
    pCtlEntry: PCTL_ENTRY,
    pCtlContext: PCCTL_CONTEXT,
}}
pub type PCERT_TRUST_LIST_INFO = *mut CERT_TRUST_LIST_INFO;
STRUCT!{struct CERT_CHAIN_ELEMENT {
    cbSize: DWORD,
    pCertContext: PCCERT_CONTEXT,
    TrustStatus: CERT_TRUST_STATUS,
    pRevocationInfo: PCERT_REVOCATION_INFO,
    pIssuanceUsage: PCERT_ENHKEY_USAGE,
    pApplicationUsage: PCERT_ENHKEY_USAGE,
    pwszExtendedErrorInfo: LPWSTR,
}}
pub type PCERT_CHAIN_ELEMENT = *mut CERT_CHAIN_ELEMENT;
pub type PCCERT_CHAIN_ELEMENT = *const CERT_CHAIN_ELEMENT;
STRUCT!{struct CERT_SIMPLE_CHAIN {
    cbSize: DWORD,
    TrustStatus: CERT_TRUST_STATUS,
    cElement: DWORD,
    rgpElement: *mut PCERT_CHAIN_ELEMENT,
    pTrustListInfo: PCERT_TRUST_LIST_INFO,
    fHasRevocationFreshnessTime: BOOL,
    dwRevocationFreshnessTime: DWORD,
}}
pub type PCERT_SIMPLE_CHAIN = *mut CERT_SIMPLE_CHAIN;
pub type PCCERT_SIMPLE_CHAIN = *const CERT_SIMPLE_CHAIN;
pub type PCERT_CHAIN_CONTEXT = *mut CERT_CHAIN_CONTEXT;
pub type PCCERT_CHAIN_CONTEXT = *const CERT_CHAIN_CONTEXT;
STRUCT!{struct CERT_CHAIN_CONTEXT {
    cbSize: DWORD,
    TrustStatus: CERT_TRUST_STATUS,
    cChain: DWORD,
    rgpChain: *mut PCERT_SIMPLE_CHAIN,
    cLowerQualityChainContext: DWORD,
    rgpLowerQualityChainContext: *mut PCCERT_CHAIN_CONTEXT,
    fHasRevocationFreshnessTime: BOOL,
    dwRevocationFreshnessTime: DWORD,
    dwCreateFlags: DWORD,
    ChainId: GUID,
}}
pub const USAGE_MATCH_TYPE_AND: DWORD = 0x00000000;
pub const USAGE_MATCH_TYPE_OR: DWORD = 0x00000001;
STRUCT!{struct CERT_USAGE_MATCH {
    dwType: DWORD,
    Usage: CERT_ENHKEY_USAGE,
}}
pub type PCERT_USAGE_MATCH = *mut CERT_USAGE_MATCH;
STRUCT!{struct CTL_USAGE_MATCH {
    dwType: DWORD,
    Usage: CTL_USAGE,
}}
pub type PCTL_USAGE_MATCH = *mut CTL_USAGE_MATCH;
STRUCT!{struct CERT_CHAIN_PARA {
    cbSize: DWORD,
    RequestedUsage: CERT_USAGE_MATCH,
    RequestedIssuancePolicy: CERT_USAGE_MATCH,
    dwUrlRetrievalTimeout: DWORD,
    fCheckRevocationFreshnessTime: BOOL,
    dwRevocationFreshnessTime: DWORD,
    pftCacheResync: LPFILETIME,
    pStrongSignPara: PCCERT_STRONG_SIGN_PARA,
    dwStrongSignFlags: DWORD,
}}
pub type PCERT_CHAIN_PARA = *mut CERT_CHAIN_PARA;
pub const CERT_CHAIN_STRONG_SIGN_DISABLE_END_CHECK_FLAG: DWORD = 0x00000001;
pub const CERT_CHAIN_REVOCATION_CHECK_END_CERT: DWORD = 0x10000000;
pub const CERT_CHAIN_REVOCATION_CHECK_CHAIN: DWORD = 0x20000000;
pub const CERT_CHAIN_REVOCATION_CHECK_CHAIN_EXCLUDE_ROOT: DWORD = 0x40000000;
pub const CERT_CHAIN_REVOCATION_CHECK_CACHE_ONLY: DWORD = 0x80000000;
pub const CERT_CHAIN_REVOCATION_ACCUMULATIVE_TIMEOUT: DWORD = 0x08000000;
pub const CERT_CHAIN_REVOCATION_CHECK_OCSP_CERT: DWORD = 0x04000000;
pub const CERT_CHAIN_DISABLE_PASS1_QUALITY_FILTERING: DWORD = 0x00000040;
pub const CERT_CHAIN_RETURN_LOWER_QUALITY_CONTEXTS: DWORD = 0x00000080;
pub const CERT_CHAIN_DISABLE_AUTH_ROOT_AUTO_UPDATE: DWORD = 0x00000100;
pub const CERT_CHAIN_TIMESTAMP_TIME: DWORD = 0x00000200;
pub const CERT_CHAIN_ENABLE_PEER_TRUST: DWORD = 0x00000400;
pub const CERT_CHAIN_DISABLE_MY_PEER_TRUST: DWORD = 0x00000800;
pub const CERT_CHAIN_DISABLE_MD2_MD4: DWORD = 0x00001000;
pub const CERT_CHAIN_DISABLE_AIA: DWORD = 0x00002000;
pub const CERT_CHAIN_HAS_MOTW: DWORD = 0x00004000;
pub const CERT_CHAIN_ONLY_ADDITIONAL_AND_AUTH_ROOT: DWORD = 0x00008000;
pub const CERT_CHAIN_OPT_IN_WEAK_SIGNATURE: DWORD = 0x00010000;
extern "system" {
    pub fn CertGetCertificateChain(
        hChainEngine: HCERTCHAINENGINE,
        pCertContext: PCCERT_CONTEXT,
        pTime: LPFILETIME,
        hAdditionalStore: HCERTSTORE,
        pChainPara: PCERT_CHAIN_PARA,
        dwFlags: DWORD,
        pvReserved: LPVOID,
        ppChainContext: *mut PCCERT_CHAIN_CONTEXT,
    ) -> BOOL;
    pub fn CertFreeCertificateChain(
        pChainContext: PCCERT_CHAIN_CONTEXT,
    );
    pub fn CertDuplicateCertificateChain(
        pChainContext: PCCERT_CHAIN_CONTEXT,
    ) -> PCCERT_CHAIN_CONTEXT;
}
STRUCT!{struct CERT_REVOCATION_CHAIN_PARA {
    cbSize: DWORD,
    hChainEngine: HCERTCHAINENGINE,
    hAdditionalStore: HCERTSTORE,
    dwChainFlags: DWORD,
    dwUrlRetrievalTimeout: DWORD,
    pftCurrentTime: LPFILETIME,
    pftCacheResync: LPFILETIME,
    cbMaxUrlRetrievalByteCount: DWORD,
}}
pub const REVOCATION_OID_CRL_REVOCATION: LPCSTR = 1 as LPCSTR;
STRUCT!{struct CRL_REVOCATION_INFO {
    pCrlEntry: PCRL_ENTRY,
    pCrlContext: PCCRL_CONTEXT,
    pCrlIssuerChain: PCCERT_CHAIN_CONTEXT,
}}
pub type PCRL_REVOCATION_INFO = *mut CRL_REVOCATION_INFO;
extern "system" {
    pub fn CertFindChainInStore(
        hCertStore: HCERTSTORE,
        dwCertEncodingType: DWORD,
        dwFindFlags: DWORD,
        dwFindType: DWORD,
        pvFindPara: *const c_void,
        pPrevChainContext: PCCERT_CHAIN_CONTEXT,
    ) -> PCCERT_CHAIN_CONTEXT;
}
pub const CERT_CHAIN_FIND_BY_ISSUER: DWORD = 1;
FN!{stdcall PFN_CERT_CHAIN_FIND_BY_ISSUER_CALLBACK(
    pCert: PCCERT_CONTEXT,
    pvFindArg: *mut c_void,
) -> BOOL}
STRUCT!{struct CERT_CHAIN_FIND_ISSUER_PARA {
    cbSize: DWORD,
    pszUsageIdentifier: LPCSTR,
    dwKeySpec: DWORD,
    dwAcquirePrivateKeyFlags: DWORD,
    cIssuer: DWORD,
    rgIssuer: *mut CERT_NAME_BLOB,
    pfnFindCallback: PFN_CERT_CHAIN_FIND_BY_ISSUER_CALLBACK,
    pvFindArg: *mut c_void,
    pdwIssuerChainIndex: *mut DWORD,
    pdwIssuerElementIndex: *mut DWORD,
}}
pub type PCERT_CHAIN_FIND_ISSUER_PARA = *mut CERT_CHAIN_FIND_ISSUER_PARA;
pub type CERT_CHAIN_FIND_BY_ISSUER_PARA = CERT_CHAIN_FIND_ISSUER_PARA;
pub type PCERT_CHAIN_FIND_BY_ISSUER_PARA = *mut CERT_CHAIN_FIND_ISSUER_PARA;
pub const CERT_CHAIN_FIND_BY_ISSUER_COMPARE_KEY_FLAG: DWORD = 0x0001;
pub const CERT_CHAIN_FIND_BY_ISSUER_COMPLEX_CHAIN_FLAG: DWORD = 0x0002;
pub const CERT_CHAIN_FIND_BY_ISSUER_CACHE_ONLY_URL_FLAG: DWORD = 0x0004;
pub const CERT_CHAIN_FIND_BY_ISSUER_LOCAL_MACHINE_FLAG: DWORD = 0x0008;
pub const CERT_CHAIN_FIND_BY_ISSUER_NO_KEY_FLAG: DWORD = 0x4000;
pub const CERT_CHAIN_FIND_BY_ISSUER_CACHE_ONLY_FLAG: DWORD = 0x8000;
STRUCT!{struct CERT_CHAIN_POLICY_PARA {
    cbSize: DWORD,
    dwFlags: DWORD,
    pvExtraPolicyPara: *mut c_void,
}}
pub type PCERT_CHAIN_POLICY_PARA = *mut CERT_CHAIN_POLICY_PARA;
STRUCT!{struct CERT_CHAIN_POLICY_STATUS {
    cbSize: DWORD,
    dwError: DWORD,
    lChainIndex: LONG,
    lElementIndex: LONG,
    pvExtraPolicyStatus: *mut c_void,
}}
pub type PCERT_CHAIN_POLICY_STATUS = *mut CERT_CHAIN_POLICY_STATUS;
pub const CERT_CHAIN_POLICY_IGNORE_NOT_TIME_VALID_FLAG: DWORD = 0x00000001;
pub const CERT_CHAIN_POLICY_IGNORE_CTL_NOT_TIME_VALID_FLAG: DWORD = 0x00000002;
pub const CERT_CHAIN_POLICY_IGNORE_NOT_TIME_NESTED_FLAG: DWORD = 0x00000004;
pub const CERT_CHAIN_POLICY_IGNORE_INVALID_BASIC_CONSTRAINTS_FLAG: DWORD = 0x00000008;
pub const CERT_CHAIN_POLICY_IGNORE_ALL_NOT_TIME_VALID_FLAGS: DWORD
    = CERT_CHAIN_POLICY_IGNORE_NOT_TIME_VALID_FLAG
    | CERT_CHAIN_POLICY_IGNORE_CTL_NOT_TIME_VALID_FLAG
    | CERT_CHAIN_POLICY_IGNORE_NOT_TIME_NESTED_FLAG;
pub const CERT_CHAIN_POLICY_ALLOW_UNKNOWN_CA_FLAG: DWORD = 0x00000010;
pub const CERT_CHAIN_POLICY_IGNORE_WRONG_USAGE_FLAG: DWORD = 0x00000020;
pub const CERT_CHAIN_POLICY_IGNORE_INVALID_NAME_FLAG: DWORD = 0x00000040;
pub const CERT_CHAIN_POLICY_IGNORE_INVALID_POLICY_FLAG: DWORD = 0x00000080;
pub const CERT_CHAIN_POLICY_IGNORE_END_REV_UNKNOWN_FLAG: DWORD = 0x00000100;
pub const CERT_CHAIN_POLICY_IGNORE_CTL_SIGNER_REV_UNKNOWN_FLAG: DWORD = 0x00000200;
pub const CERT_CHAIN_POLICY_IGNORE_CA_REV_UNKNOWN_FLAG: DWORD = 0x00000400;
pub const CERT_CHAIN_POLICY_IGNORE_ROOT_REV_UNKNOWN_FLAG: DWORD = 0x00000800;
pub const CERT_CHAIN_POLICY_IGNORE_ALL_REV_UNKNOWN_FLAGS: DWORD
    = CERT_CHAIN_POLICY_IGNORE_END_REV_UNKNOWN_FLAG
    | CERT_CHAIN_POLICY_IGNORE_CTL_SIGNER_REV_UNKNOWN_FLAG
    | CERT_CHAIN_POLICY_IGNORE_CA_REV_UNKNOWN_FLAG
    | CERT_CHAIN_POLICY_IGNORE_ROOT_REV_UNKNOWN_FLAG;
pub const CERT_CHAIN_POLICY_ALLOW_TESTROOT_FLAG: DWORD = 0x00008000;
pub const CERT_CHAIN_POLICY_TRUST_TESTROOT_FLAG: DWORD = 0x00004000;
pub const CERT_CHAIN_POLICY_IGNORE_NOT_SUPPORTED_CRITICAL_EXT_FLAG: DWORD = 0x00002000;
pub const CERT_CHAIN_POLICY_IGNORE_PEER_TRUST_FLAG: DWORD = 0x00001000;
pub const CERT_CHAIN_POLICY_IGNORE_WEAK_SIGNATURE_FLAG: DWORD = 0x08000000;
extern "system" {
    pub fn CertVerifyCertificateChainPolicy(
        pszPolicyOID: LPCSTR,
        pChainContext: PCCERT_CHAIN_CONTEXT,
        pPolicyPara: PCERT_CHAIN_POLICY_PARA,
        pPolicyStatus: PCERT_CHAIN_POLICY_STATUS,
    ) -> BOOL;
}
pub const CRYPT_OID_VERIFY_CERTIFICATE_CHAIN_POLICY_FUNC: &'static str
    = "CertDllVerifyCertificateChainPolicy";
pub const CERT_CHAIN_POLICY_BASE: LPCSTR = 1 as LPCSTR;
pub const CERT_CHAIN_POLICY_AUTHENTICODE: LPCSTR = 2 as LPCSTR;
pub const CERT_CHAIN_POLICY_AUTHENTICODE_TS: LPCSTR = 3 as LPCSTR;
pub const CERT_CHAIN_POLICY_SSL: LPCSTR = 4 as LPCSTR;
pub const CERT_CHAIN_POLICY_BASIC_CONSTRAINTS: LPCSTR = 5 as LPCSTR;
pub const CERT_CHAIN_POLICY_NT_AUTH: LPCSTR = 6 as LPCSTR;
pub const CERT_CHAIN_POLICY_MICROSOFT_ROOT: LPCSTR = 7 as LPCSTR;
pub const CERT_CHAIN_POLICY_EV: LPCSTR = 8 as LPCSTR;
pub const CERT_CHAIN_POLICY_SSL_F12: LPCSTR = 9 as LPCSTR;
pub const CERT_CHAIN_POLICY_SSL_HPKP_HEADER: LPCSTR = 10 as LPCSTR;
pub const CERT_CHAIN_POLICY_THIRD_PARTY_ROOT: LPCSTR = 11 as LPCSTR;
pub const CERT_CHAIN_POLICY_SSL_KEY_PIN: LPCSTR = 12 as LPCSTR;
STRUCT!{struct AUTHENTICODE_EXTRA_CERT_CHAIN_POLICY_PARA {
    cbSize: DWORD,
    dwRegPolicySettings: DWORD,
    pSignerInfo: PCMSG_SIGNER_INFO,
}}
pub type PAUTHENTICODE_EXTRA_CERT_CHAIN_POLICY_PARA
    = *mut AUTHENTICODE_EXTRA_CERT_CHAIN_POLICY_PARA;
STRUCT!{struct AUTHENTICODE_EXTRA_CERT_CHAIN_POLICY_STATUS {
    cbSize: DWORD,
    fCommercial: BOOL,
}}
pub type PAUTHENTICODE_EXTRA_CERT_CHAIN_POLICY_STATUS
    = *mut AUTHENTICODE_EXTRA_CERT_CHAIN_POLICY_STATUS;
STRUCT!{struct AUTHENTICODE_TS_EXTRA_CERT_CHAIN_POLICY_PARA {
    cbSize: DWORD,
    dwRegPolicySettings: DWORD,
    fCommercial: BOOL,
}}
pub type PAUTHENTICODE_TS_EXTRA_CERT_CHAIN_POLICY_PARA
    = *mut AUTHENTICODE_TS_EXTRA_CERT_CHAIN_POLICY_PARA;
UNION!{union HTTPSPolicyCallbackData_u {
    [u32; 1],
    cbStruct cbStruct_mut: DWORD,
    cbSize cbSize_mut: DWORD,
}}
STRUCT!{struct HTTPSPolicyCallbackData {
    u: HTTPSPolicyCallbackData_u,
    dwAuthType: DWORD,
    fdwChecks: DWORD,
    pwszServerName: *mut WCHAR,
}}
pub type PHTTPSPolicyCallbackData = *mut HTTPSPolicyCallbackData;
pub type SSL_EXTRA_CERT_CHAIN_POLICY_PARA = HTTPSPolicyCallbackData;
pub type PSSL_EXTRA_CERT_CHAIN_POLICY_PARA = *mut HTTPSPolicyCallbackData;
pub const AUTHTYPE_CLIENT: DWORD = 1;
pub const AUTHTYPE_SERVER: DWORD = 2;
pub const BASIC_CONSTRAINTS_CERT_CHAIN_POLICY_CA_FLAG: DWORD = 0x80000000;
pub const BASIC_CONSTRAINTS_CERT_CHAIN_POLICY_END_ENTITY_FLAG: DWORD = 0x40000000;
pub const MICROSOFT_ROOT_CERT_CHAIN_POLICY_ENABLE_TEST_ROOT_FLAG: DWORD = 0x00010000;
pub const MICROSOFT_ROOT_CERT_CHAIN_POLICY_CHECK_APPLICATION_ROOT_FLAG: DWORD = 0x00020000;
pub const MICROSOFT_ROOT_CERT_CHAIN_POLICY_DISABLE_FLIGHT_ROOT_FLAG: DWORD = 0x00040000;
STRUCT!{struct EV_EXTRA_CERT_CHAIN_POLICY_PARA {
    cbSize: DWORD,
    dwRootProgramQualifierFlags: DWORD,
}}
pub type PEV_EXTRA_CERT_CHAIN_POLICY_PARA = *mut EV_EXTRA_CERT_CHAIN_POLICY_PARA;
STRUCT!{struct EV_EXTRA_CERT_CHAIN_POLICY_STATUS {
    cbSize: DWORD,
    dwQualifiers: DWORD,
    dwIssuanceUsageIndex: DWORD,
}}
pub type PEV_EXTRA_CERT_CHAIN_POLICY_STATUS = *mut EV_EXTRA_CERT_CHAIN_POLICY_STATUS;
pub const SSL_F12_ERROR_TEXT_LENGTH: usize = 256;
STRUCT!{struct SSL_F12_EXTRA_CERT_CHAIN_POLICY_STATUS {
    cbSize: DWORD,
    dwErrorLevel: DWORD,
    dwErrorCategory: DWORD,
    dwReserved: DWORD,
    wszErrorText: [WCHAR; SSL_F12_ERROR_TEXT_LENGTH],
}}
pub type PSSL_F12_EXTRA_CERT_CHAIN_POLICY_STATUS = *mut SSL_F12_EXTRA_CERT_CHAIN_POLICY_STATUS;
pub const CERT_CHAIN_POLICY_SSL_F12_SUCCESS_LEVEL: DWORD = 0;
pub const CERT_CHAIN_POLICY_SSL_F12_WARNING_LEVEL: DWORD = 1;
pub const CERT_CHAIN_POLICY_SSL_F12_ERROR_LEVEL: DWORD = 2;
pub const CERT_CHAIN_POLICY_SSL_F12_NONE_CATEGORY: DWORD = 0;
pub const CERT_CHAIN_POLICY_SSL_F12_WEAK_CRYPTO_CATEGORY: DWORD = 1;
pub const CERT_CHAIN_POLICY_SSL_F12_ROOT_PROGRAM_CATEGORY: DWORD = 2;
pub const SSL_HPKP_PKP_HEADER_INDEX: usize = 0;
pub const SSL_HPKP_PKP_RO_HEADER_INDEX: usize = 1;
pub const SSL_HPKP_HEADER_COUNT: usize = 2;
STRUCT!{struct SSL_HPKP_HEADER_EXTRA_CERT_CHAIN_POLICY_PARA {
    cbSize: DWORD,
    dwReserved: DWORD,
    pwszServerName: LPWSTR,
    rgpszHpkpValue: [LPSTR; SSL_HPKP_HEADER_COUNT],
}}
pub type PSSL_HPKP_HEADER_EXTRA_CERT_CHAIN_POLICY_PARA
    = *mut SSL_HPKP_HEADER_EXTRA_CERT_CHAIN_POLICY_PARA;
STRUCT!{struct SSL_KEY_PIN_EXTRA_CERT_CHAIN_POLICY_PARA {
    cbSize: DWORD,
    dwReserved: DWORD,
    pwszServerName: PCWSTR,
}}
pub type PSSL_KEY_PIN_EXTRA_CERT_CHAIN_POLICY_PARA = *mut SSL_KEY_PIN_EXTRA_CERT_CHAIN_POLICY_PARA;
pub const SSL_KEY_PIN_ERROR_TEXT_LENGTH: usize = 512;
STRUCT!{struct SSL_KEY_PIN_EXTRA_CERT_CHAIN_POLICY_STATUS {
    cbSize: DWORD,
    lError: LONG,
    wszErrorText: [WCHAR; SSL_KEY_PIN_ERROR_TEXT_LENGTH],
}}
pub type PSSL_KEY_PIN_EXTRA_CERT_CHAIN_POLICY_STATUS
    = *mut SSL_KEY_PIN_EXTRA_CERT_CHAIN_POLICY_STATUS;
pub const CERT_CHAIN_POLICY_SSL_KEY_PIN_MISMATCH_ERROR: LONG = -2;
pub const CERT_CHAIN_POLICY_SSL_KEY_PIN_MITM_ERROR: LONG = -1;
pub const CERT_CHAIN_POLICY_SSL_KEY_PIN_SUCCESS: LONG = 0;
pub const CERT_CHAIN_POLICY_SSL_KEY_PIN_MITM_WARNING: LONG = 1;
pub const CERT_CHAIN_POLICY_SSL_KEY_PIN_MISMATCH_WARNING: LONG = 2;
extern "system" {
    pub fn CryptStringToBinaryA(
        pszString: LPCSTR,
        cchString: DWORD,
        dwFlags: DWORD,
        pbBinary: *mut BYTE,
        pcbBinary: *mut DWORD,
        pdwSkip: *mut DWORD,
        pdwFlags: *mut DWORD,
    ) -> BOOL;
    pub fn CryptStringToBinaryW(
        pszString: LPCWSTR,
        cchString: DWORD,
        dwFlags: DWORD,
        pbBinary: *mut BYTE,
        pcbBinary: *mut DWORD,
        pdwSkip: *mut DWORD,
        pdwFlags: *mut DWORD,
    ) -> BOOL;
    pub fn CryptBinaryToStringA(
        pbBinary: *const BYTE,
        cbBinary: DWORD,
        dwFlags: DWORD,
        pszString: LPSTR,
        pcchString: *mut DWORD,
    ) -> BOOL;
    pub fn CryptBinaryToStringW(
        pbBinary: *const BYTE,
        cbBinary: DWORD,
        dwFlags: DWORD,
        pszString: LPWSTR,
        pcchString: *mut DWORD,
    ) -> BOOL;
}
pub const CRYPT_STRING_BASE64HEADER: DWORD = 0x00000000;
pub const CRYPT_STRING_BASE64: DWORD = 0x00000001;
pub const CRYPT_STRING_BINARY: DWORD = 0x00000002;
pub const CRYPT_STRING_BASE64REQUESTHEADER: DWORD = 0x00000003;
pub const CRYPT_STRING_HEX: DWORD = 0x00000004;
pub const CRYPT_STRING_HEXASCII: DWORD = 0x00000005;
pub const CRYPT_STRING_BASE64_ANY: DWORD = 0x00000006;
pub const CRYPT_STRING_ANY: DWORD = 0x00000007;
pub const CRYPT_STRING_HEX_ANY: DWORD = 0x00000008;
pub const CRYPT_STRING_BASE64X509CRLHEADER: DWORD = 0x00000009;
pub const CRYPT_STRING_HEXADDR: DWORD = 0x0000000a;
pub const CRYPT_STRING_HEXASCIIADDR: DWORD = 0x0000000b;
pub const CRYPT_STRING_HEXRAW: DWORD = 0x0000000c;
pub const CRYPT_STRING_BASE64URI: DWORD = 0x0000000d;
pub const CRYPT_STRING_ENCODEMASK: DWORD = 0x000000ff;
pub const CRYPT_STRING_RESERVED100: DWORD = 0x00000100;
pub const CRYPT_STRING_RESERVED200: DWORD = 0x00000200;
pub const CRYPT_STRING_PERCENTESCAPE: DWORD = 0x08000000;
pub const CRYPT_STRING_HASHDATA: DWORD = 0x10000000;
pub const CRYPT_STRING_STRICT: DWORD = 0x20000000;
pub const CRYPT_STRING_NOCRLF: DWORD = 0x40000000;
pub const CRYPT_STRING_NOCR: DWORD = 0x80000000;
pub const szOID_PKCS_12_PbeIds: &'static str = "1.2.840.113549.1.12.1";
pub const szOID_PKCS_12_pbeWithSHA1And128BitRC4: &'static str = "1.2.840.113549.1.12.1.1";
pub const szOID_PKCS_12_pbeWithSHA1And40BitRC4: &'static str = "1.2.840.113549.1.12.1.2";
pub const szOID_PKCS_12_pbeWithSHA1And3KeyTripleDES: &'static str = "1.2.840.113549.1.12.1.3";
pub const szOID_PKCS_12_pbeWithSHA1And2KeyTripleDES: &'static str = "1.2.840.113549.1.12.1.4";
pub const szOID_PKCS_12_pbeWithSHA1And128BitRC2: &'static str = "1.2.840.113549.1.12.1.5";
pub const szOID_PKCS_12_pbeWithSHA1And40BitRC2: &'static str = "1.2.840.113549.1.12.1.6";
STRUCT!{struct CRYPT_PKCS12_PBE_PARAMS {
    iIterations: c_int,
    cbSalt: ULONG,
}}
extern "system" {
    pub fn PFXImportCertStore(
        pPFX: *mut CRYPT_DATA_BLOB,
        szPassword: LPCWSTR,
        dwFlags: DWORD,
    ) -> HCERTSTORE;
}
pub const PKCS12_IMPORT_SILENT: DWORD = 0x00000040;
pub const CRYPT_USER_KEYSET: DWORD = 0x00001000;
pub const PKCS12_PREFER_CNG_KSP: DWORD = 0x00000100;
pub const PKCS12_ALWAYS_CNG_KSP: DWORD = 0x00000200;
pub const PKCS12_ONLY_CERTIFICATES: DWORD = 0x00000400;
pub const PKCS12_ONLY_NOT_ENCRYPTED_CERTIFICATES: DWORD = 0x00000800;
pub const PKCS12_ALLOW_OVERWRITE_KEY: DWORD = 0x00004000;
pub const PKCS12_NO_PERSIST_KEY: DWORD = 0x00008000;
pub const PKCS12_IMPORT_RESERVED_MASK: DWORD = 0xffff0000;
pub const PKCS12_OBJECT_LOCATOR_ALL_IMPORT_FLAGS: DWORD = PKCS12_ALWAYS_CNG_KSP
    | PKCS12_NO_PERSIST_KEY | PKCS12_IMPORT_SILENT | PKCS12_INCLUDE_EXTENDED_PROPERTIES;
pub const PKCS12_ONLY_CERTIFICATES_PROVIDER_TYPE: DWORD = 0;
pub const PKCS12_ONLY_CERTIFICATES_PROVIDER_NAME: &'static str = "PfxProvider";
pub const PKCS12_ONLY_CERTIFICATES_CONTAINER_NAME: &'static str = "PfxContainer";
extern "system" {
    pub fn PFXIsPFXBlob(
        pPFX: *mut CRYPT_DATA_BLOB,
    ) -> BOOL;
    pub fn PFXVerifyPassword(
        pPFX: *mut CRYPT_DATA_BLOB,
        szPassword: LPCWSTR,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn PFXExportCertStoreEx(
        hStore: HCERTSTORE,
        pPFX: *mut CRYPT_DATA_BLOB,
        szPassword: LPCWSTR,
        pvPara: *mut c_void,
        dwFlags: DWORD,
    ) -> BOOL;
}
pub const REPORT_NO_PRIVATE_KEY: DWORD = 0x0001;
pub const REPORT_NOT_ABLE_TO_EXPORT_PRIVATE_KEY: DWORD = 0x0002;
pub const EXPORT_PRIVATE_KEYS: DWORD = 0x0004;
pub const PKCS12_INCLUDE_EXTENDED_PROPERTIES: DWORD = 0x0010;
pub const PKCS12_PROTECT_TO_DOMAIN_SIDS: DWORD = 0x0020;
pub const PKCS12_EXPORT_SILENT: DWORD = 0x0040;
pub const PKCS12_DISABLE_ENCRYPT_CERTIFICATES: DWORD = 0x0100;
pub const PKCS12_ENCRYPT_CERTIFICATES: DWORD = 0x0200;
pub const PKCS12_EXPORT_ECC_CURVE_PARAMETERS: DWORD = 0x1000;
pub const PKCS12_EXPORT_ECC_CURVE_OID: DWORD = 0x2000;
pub const PKCS12_EXPORT_RESERVED_MASK: DWORD = 0xffff0000;
pub const PKCS12_CONFIG_REGPATH: &'static str
    = "Software\\Microsoft\\Windows\\CurrentVersion\\PFX";
pub const PKCS12_ENCRYPT_CERTIFICATES_VALUE_NAME: &'static str = "EncryptCertificates";
extern "system" {
    pub fn PFXExportCertStore(
        hStore: HCERTSTORE,
        pPFX: *mut CRYPT_DATA_BLOB,
        szPassword: LPCWSTR,
        dwFlags: DWORD,
    ) -> BOOL;
}
pub type HCERT_SERVER_OCSP_RESPONSE = *mut c_void;
pub type PCERT_SERVER_OCSP_RESPONSE_CONTEXT = *mut CERT_SERVER_OCSP_RESPONSE_CONTEXT;
pub type PCCERT_SERVER_OCSP_RESPONSE_CONTEXT = *const CERT_SERVER_OCSP_RESPONSE_CONTEXT;
STRUCT!{struct CERT_SERVER_OCSP_RESPONSE_CONTEXT {
    cbSize: DWORD,
    pbEncodedOcspResponse: *mut BYTE,
    cbEncodedOcspResponse: DWORD,
}}
FN!{stdcall PFN_CERT_SERVER_OCSP_RESPONSE_UPDATE_CALLBACK(
    pChainContext: PCCERT_CHAIN_CONTEXT,
    pServerOcspResponseContext: PCCERT_SERVER_OCSP_RESPONSE_CONTEXT,
    pNewCrlContext: PCCRL_CONTEXT,
    pPrevCrlContext: PCCRL_CONTEXT,
    pvArg: PVOID,
    dwWriteOcspFileError: DWORD,
) -> ()}
STRUCT!{struct CERT_SERVER_OCSP_RESPONSE_OPEN_PARA {
    cbSize: DWORD,
    dwFlags: DWORD,
    pcbUsedSize: *mut DWORD,
    pwszOcspDirectory: PWSTR,
    pfnUpdateCallback: PFN_CERT_SERVER_OCSP_RESPONSE_UPDATE_CALLBACK,
    pvUpdateCallbackArg: PVOID,
}}
pub type PCERT_SERVER_OCSP_RESPONSE_OPEN_PARA = *mut CERT_SERVER_OCSP_RESPONSE_OPEN_PARA;
pub const CERT_SERVER_OCSP_RESPONSE_OPEN_PARA_READ_FLAG: DWORD = 0x00000001;
pub const CERT_SERVER_OCSP_RESPONSE_OPEN_PARA_WRITE_FLAG: DWORD = 0x00000002;
extern "system" {
    pub fn CertOpenServerOcspResponse(
        pChainContext: PCCERT_CHAIN_CONTEXT,
        dwFlags: DWORD,
        pvReserved: LPVOID,
    ) -> HCERT_SERVER_OCSP_RESPONSE;
}
pub const CERT_SERVER_OCSP_RESPONSE_ASYNC_FLAG: DWORD = 0x00000001;
extern "system" {
    pub fn CertAddRefServerOcspResponse(
        hServerOcspResponse: HCERT_SERVER_OCSP_RESPONSE,
    );
    pub fn CertCloseServerOcspResponse(
        hServerOcspResponse: HCERT_SERVER_OCSP_RESPONSE,
        dwFlags: DWORD,
    );
    pub fn CertGetServerOcspResponseContext(
        hServerOcspResponse: HCERT_SERVER_OCSP_RESPONSE,
        dwFlags: DWORD,
        pvReserved: LPVOID,
    ) -> PCCERT_SERVER_OCSP_RESPONSE_CONTEXT;
    pub fn CertAddRefServerOcspResponseContext(
        pServerOcspResponseContext: PCCERT_SERVER_OCSP_RESPONSE_CONTEXT,
    );
    pub fn CertFreeServerOcspResponseContext(
        pServerOcspResponseContext: PCCERT_SERVER_OCSP_RESPONSE_CONTEXT,
    );
    pub fn CertRetrieveLogoOrBiometricInfo(
        pCertContext: PCCERT_CONTEXT,
        lpszLogoOrBiometricType: LPCSTR,
        dwRetrievalFlags: DWORD,
        dwTimeout: DWORD,
        dwFlags: DWORD,
        pvReserved: *mut c_void,
        ppbData: *mut *mut BYTE,
        pcbData: *mut DWORD,
        ppwszMimeType: *mut LPWSTR,
    ) -> BOOL;
}
pub const CERT_RETRIEVE_ISSUER_LOGO: LPCSTR = 1 as LPCSTR;
pub const CERT_RETRIEVE_SUBJECT_LOGO: LPCSTR = 2 as LPCSTR;
pub const CERT_RETRIEVE_COMMUNITY_LOGO: LPCSTR = 3 as LPCSTR;
pub const CERT_RETRIEVE_BIOMETRIC_PREDEFINED_BASE_TYPE: LPCSTR = 1000 as LPCSTR;
pub const CERT_RETRIEVE_BIOMETRIC_PICTURE_TYPE: LPCSTR
    = (1000 + CERT_BIOMETRIC_PICTURE_TYPE) as LPCSTR;
pub const CERT_RETRIEVE_BIOMETRIC_SIGNATURE_TYPE: LPCSTR
    = (1000 + CERT_BIOMETRIC_SIGNATURE_TYPE) as LPCSTR;
STRUCT!{struct CERT_SELECT_CHAIN_PARA {
    hChainEngine: HCERTCHAINENGINE,
    pTime: PFILETIME,
    hAdditionalStore: HCERTSTORE,
    pChainPara: PCERT_CHAIN_PARA,
    dwFlags: DWORD,
}}
pub type PCERT_SELECT_CHAIN_PARA = *mut CERT_SELECT_CHAIN_PARA;
pub type PCCERT_SELECT_CHAIN_PARA = *const CERT_SELECT_CHAIN_PARA;
pub const CERT_SELECT_MAX_PARA: DWORD = 500;
STRUCT!{struct CERT_SELECT_CRITERIA {
    dwType: DWORD,
    cPara: DWORD,
    ppPara: *mut *mut c_void,
}}
pub type PCERT_SELECT_CRITERIA = *mut CERT_SELECT_CRITERIA;
pub type PCCERT_SELECT_CRITERIA = *const CERT_SELECT_CRITERIA;
pub const CERT_SELECT_BY_ENHKEY_USAGE: DWORD = 1;
pub const CERT_SELECT_BY_KEY_USAGE: DWORD = 2;
pub const CERT_SELECT_BY_POLICY_OID: DWORD = 3;
pub const CERT_SELECT_BY_PROV_NAME: DWORD = 4;
pub const CERT_SELECT_BY_EXTENSION: DWORD = 5;
pub const CERT_SELECT_BY_SUBJECT_HOST_NAME: DWORD = 6;
pub const CERT_SELECT_BY_ISSUER_ATTR: DWORD = 7;
pub const CERT_SELECT_BY_SUBJECT_ATTR: DWORD = 8;
pub const CERT_SELECT_BY_ISSUER_NAME: DWORD = 9;
pub const CERT_SELECT_BY_PUBLIC_KEY: DWORD = 10;
pub const CERT_SELECT_BY_TLS_SIGNATURES: DWORD = 11;
pub const CERT_SELECT_BY_ISSUER_DISPLAYNAME: DWORD = 12;
pub const CERT_SELECT_BY_FRIENDLYNAME: DWORD = 13;
pub const CERT_SELECT_BY_THUMBPRINT: DWORD = 14;
pub const CERT_SELECT_LAST: DWORD = CERT_SELECT_BY_TLS_SIGNATURES;
pub const CERT_SELECT_MAX: DWORD = CERT_SELECT_LAST * 3;
pub const CERT_SELECT_ALLOW_EXPIRED: DWORD = 0x00000001;
pub const CERT_SELECT_TRUSTED_ROOT: DWORD = 0x00000002;
pub const CERT_SELECT_DISALLOW_SELFSIGNED: DWORD = 0x00000004;
pub const CERT_SELECT_HAS_PRIVATE_KEY: DWORD = 0x00000008;
pub const CERT_SELECT_HAS_KEY_FOR_SIGNATURE: DWORD = 0x00000010;
pub const CERT_SELECT_HAS_KEY_FOR_KEY_EXCHANGE: DWORD = 0x00000020;
pub const CERT_SELECT_HARDWARE_ONLY: DWORD = 0x00000040;
pub const CERT_SELECT_ALLOW_DUPLICATES: DWORD = 0x00000080;
pub const CERT_SELECT_IGNORE_AUTOSELECT: DWORD = 0x00000100;
extern "system" {
    pub fn CertSelectCertificateChains(
        pSelectionContext: LPCGUID,
        dwFlags: DWORD,
        pChainParameters: PCCERT_SELECT_CHAIN_PARA,
        cCriteria: DWORD,
        rgpCriteria: PCCERT_SELECT_CRITERIA,
        hStore: HCERTSTORE,
        pcSelection: PDWORD,
        pprgpSelection: *mut *mut PCCERT_CHAIN_CONTEXT,
    ) -> BOOL;
    pub fn CertFreeCertificateChainList(
        prgpSelection: *mut PCCERT_CHAIN_CONTEXT,
    );
}
pub const TIMESTAMP_VERSION: DWORD = 1;
STRUCT!{struct CRYPT_TIMESTAMP_REQUEST {
    dwVersion: DWORD,
    HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    HashedMessage: CRYPT_DER_BLOB,
    pszTSAPolicyId: LPSTR,
    Nonce: CRYPT_INTEGER_BLOB,
    fCertReq: BOOL,
    cExtension: DWORD,
    rgExtension: PCERT_EXTENSION,
}}
pub type PCRYPT_TIMESTAMP_REQUEST = *mut CRYPT_TIMESTAMP_REQUEST;
STRUCT!{struct CRYPT_TIMESTAMP_RESPONSE {
    dwStatus: DWORD,
    cFreeText: DWORD,
    rgFreeText: *mut LPWSTR,
    FailureInfo: CRYPT_BIT_BLOB,
    ContentInfo: CRYPT_DER_BLOB,
}}
pub type PCRYPT_TIMESTAMP_RESPONSE = *mut CRYPT_TIMESTAMP_RESPONSE;
pub const TIMESTAMP_STATUS_GRANTED: DWORD = 0;
pub const TIMESTAMP_STATUS_GRANTED_WITH_MODS: DWORD = 1;
pub const TIMESTAMP_STATUS_REJECTED: DWORD = 2;
pub const TIMESTAMP_STATUS_WAITING: DWORD = 3;
pub const TIMESTAMP_STATUS_REVOCATION_WARNING: DWORD = 4;
pub const TIMESTAMP_STATUS_REVOKED: DWORD = 5;
pub const TIMESTAMP_FAILURE_BAD_ALG: DWORD = 0;
pub const TIMESTAMP_FAILURE_BAD_REQUEST: DWORD = 2;
pub const TIMESTAMP_FAILURE_BAD_FORMAT: DWORD = 5;
pub const TIMESTAMP_FAILURE_TIME_NOT_AVAILABLE: DWORD = 14;
pub const TIMESTAMP_FAILURE_POLICY_NOT_SUPPORTED: DWORD = 15;
pub const TIMESTAMP_FAILURE_EXTENSION_NOT_SUPPORTED: DWORD = 16;
pub const TIMESTAMP_FAILURE_INFO_NOT_AVAILABLE: DWORD = 17;
pub const TIMESTAMP_FAILURE_SYSTEM_FAILURE: DWORD = 25;
STRUCT!{struct CRYPT_TIMESTAMP_ACCURACY {
    dwSeconds: DWORD,
    dwMillis: DWORD,
    dwMicros: DWORD,
}}
pub type PCRYPT_TIMESTAMP_ACCURACY = *mut CRYPT_TIMESTAMP_ACCURACY;
STRUCT!{struct CRYPT_TIMESTAMP_INFO {
    dwVersion: DWORD,
    pszTSAPolicyId: LPSTR,
    HashAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    HashedMessage: CRYPT_DER_BLOB,
    SerialNumber: CRYPT_INTEGER_BLOB,
    ftTime: FILETIME,
    pvAccuracy: PCRYPT_TIMESTAMP_ACCURACY,
    fOrdering: BOOL,
    Nonce: CRYPT_DER_BLOB,
    Tsa: CRYPT_DER_BLOB,
    cExtension: DWORD,
    rgExtension: PCERT_EXTENSION,
}}
pub type PCRYPT_TIMESTAMP_INFO = *mut CRYPT_TIMESTAMP_INFO;
STRUCT!{struct CRYPT_TIMESTAMP_CONTEXT {
    cbEncoded: DWORD,
    pbEncoded: *mut BYTE,
    pTimeStamp: PCRYPT_TIMESTAMP_INFO,
}}
pub type PCRYPT_TIMESTAMP_CONTEXT = *mut CRYPT_TIMESTAMP_CONTEXT;
STRUCT!{struct CRYPT_TIMESTAMP_PARA {
    pszTSAPolicyId: LPCSTR,
    fRequestCerts: BOOL,
    Nonce: CRYPT_INTEGER_BLOB,
    cExtension: DWORD,
    rgExtension: PCERT_EXTENSION,
}}
pub type PCRYPT_TIMESTAMP_PARA = *mut CRYPT_TIMESTAMP_PARA;
extern "system" {
    pub fn CryptRetrieveTimeStamp(
        wszUrl: LPCWSTR,
        dwRetrievalFlags: DWORD,
        dwTimeout: DWORD,
        pszHashId: LPCSTR,
        pPara: *const CRYPT_TIMESTAMP_PARA,
        pbData: *const BYTE,
        cbData: DWORD,
        ppTsContext: *mut PCRYPT_TIMESTAMP_CONTEXT,
        ppTsSigner: *mut PCCERT_CONTEXT,
        phStore: *mut HCERTSTORE,
    ) -> BOOL;
}
pub const TIMESTAMP_DONT_HASH_DATA: DWORD = 0x00000001;
pub const TIMESTAMP_VERIFY_CONTEXT_SIGNATURE: DWORD = 0x00000020;
pub const TIMESTAMP_NO_AUTH_RETRIEVAL: DWORD = 0x00020000;
extern "system" {
    pub fn CryptVerifyTimeStampSignature(
        pbTSContentInfo: *const BYTE,
        cbTSContentInfo: DWORD,
        pbData: *const BYTE,
        cbData: DWORD,
        hAdditionalStore: HCERTSTORE,
        ppTsContext: *mut PCRYPT_TIMESTAMP_CONTEXT,
        ppTsSigner: *mut PCCERT_CONTEXT,
        phStore: *mut HCERTSTORE,
    ) -> BOOL;
}
pub const CRYPT_OBJECT_LOCATOR_SPN_NAME_TYPE: DWORD = 1;
pub const CRYPT_OBJECT_LOCATOR_LAST_RESERVED_NAME_TYPE: DWORD = 32;
pub const CRYPT_OBJECT_LOCATOR_FIRST_RESERVED_USER_NAME_TYPE: DWORD = 33;
pub const CRYPT_OBJECT_LOCATOR_LAST_RESERVED_USER_NAME_TYPE: DWORD = 0x0000FFFF;
pub const SSL_OBJECT_LOCATOR_PFX_FUNC: &'static str = "SslObjectLocatorInitializePfx";
pub const SSL_OBJECT_LOCATOR_ISSUER_LIST_FUNC: &'static str
    = "SslObjectLocatorInitializeIssuerList";
pub const SSL_OBJECT_LOCATOR_CERT_VALIDATION_CONFIG_FUNC: &'static str
    = "SslObjectLocatorInitializeCertValidationConfig";
pub const CRYPT_OBJECT_LOCATOR_RELEASE_SYSTEM_SHUTDOWN: DWORD = 1;
pub const CRYPT_OBJECT_LOCATOR_RELEASE_SERVICE_STOP: DWORD = 2;
pub const CRYPT_OBJECT_LOCATOR_RELEASE_PROCESS_EXIT: DWORD = 3;
pub const CRYPT_OBJECT_LOCATOR_RELEASE_DLL_UNLOAD: DWORD = 4;
FN!{stdcall PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_FLUSH(
    pContext: LPVOID,
    rgIdentifierOrNameList: *mut PCERT_NAME_BLOB,
    dwIdentifierOrNameListCount: DWORD,
) -> BOOL}
FN!{stdcall PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_GET(
    pPluginContext: LPVOID,
    pIdentifier: PCRYPT_DATA_BLOB,
    dwNameType: DWORD,
    pNameBlob: PCERT_NAME_BLOB,
    ppbContent: *mut PBYTE,
    pcbContent: *mut DWORD,
    ppwszPassword: *mut PCWSTR,
    ppIdentifier: *mut PCRYPT_DATA_BLOB,
) -> BOOL}
FN!{stdcall PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_RELEASE(
    dwReason: DWORD,
    pPluginContext: LPVOID,
) -> ()}
FN!{stdcall PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_FREE_PASSWORD(
    pPluginContext: LPVOID,
    pwszPassword: PCWSTR,
) -> ()}
FN!{stdcall PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_FREE(
    pPluginContext: LPVOID,
    pbData: PBYTE,
) -> ()}
FN!{stdcall PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_FREE_IDENTIFIER(
    pPluginContext: LPVOID,
    pIdentifier: PCRYPT_DATA_BLOB,
) -> ()}
STRUCT!{struct CRYPT_OBJECT_LOCATOR_PROVIDER_TABLE {
    cbSize: DWORD,
    pfnGet: PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_GET,
    pfnRelease: PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_RELEASE,
    pfnFreePassword: PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_FREE_PASSWORD,
    pfnFree: PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_FREE,
    pfnFreeIdentifier: PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_FREE_IDENTIFIER,
}}
pub type PCRYPT_OBJECT_LOCATOR_PROVIDER_TABLE = *mut CRYPT_OBJECT_LOCATOR_PROVIDER_TABLE;
FN!{stdcall PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_INITIALIZE(
    pfnFlush: PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_FLUSH,
    pContext: LPVOID,
    pdwExpectedObjectCount: *mut DWORD,
    ppFuncTable: *mut PCRYPT_OBJECT_LOCATOR_PROVIDER_TABLE,
    ppPluginContext: *mut *mut c_void,
) -> BOOL}
extern "system" {
    pub fn CertIsWeakHash(
        dwHashUseType: DWORD,
        pwszCNGHashAlgid: LPCWSTR,
        dwChainFlags: DWORD,
        pSignerChainContext: PCCERT_CHAIN_CONTEXT,
        pTimeStamp: LPFILETIME,
        pwszFileName: LPCWSTR,
    ) -> BOOL;
}
FN!{stdcall PFN_CERT_IS_WEAK_HASH(
    dwHashUseType: DWORD,
    pwszCNGHashAlgid: LPCWSTR,
    dwChainFlags: DWORD,
    pSignerChainContext: PCCERT_CHAIN_CONTEXT,
    pTimeStamp: LPFILETIME,
    pwszFileName: LPCWSTR,
) -> BOOL}
pub const CERT_FILE_HASH_USE_TYPE: DWORD = 1;
pub const CERT_TIMESTAMP_HASH_USE_TYPE: DWORD = 2;
