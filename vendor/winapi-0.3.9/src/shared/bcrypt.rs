// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Cryptographic Primitive API Prototypes and Definitions
use shared::minwindef::{PUCHAR, UCHAR, ULONG, USHORT};
use um::winnt::{BOOLEAN, HANDLE, LONG, LPCWSTR, LPWSTR, PVOID, PWSTR, ULONGLONG, VOID};
pub type NTSTATUS = LONG;
pub type PNTSTATUS = *mut NTSTATUS;
#[inline]
pub fn BCRYPT_SUCCESS(Status: NTSTATUS) -> bool {
    Status >= 0
}
pub const BCRYPT_OBJECT_ALIGNMENT: usize = 16;
pub const BCRYPT_KDF_HASH: &'static str = "HASH";
pub const BCRYPT_KDF_HMAC: &'static str = "HMAC";
pub const BCRYPT_KDF_TLS_PRF: &'static str = "TLS_PRF";
pub const BCRYPT_KDF_SP80056A_CONCAT: &'static str = "SP800_56A_CONCAT";
pub const BCRYPT_KDF_RAW_SECRET: &'static str = "TRUNCATE";
pub const KDF_HASH_ALGORITHM: ULONG = 0x0;
pub const KDF_SECRET_PREPEND: ULONG = 0x1;
pub const KDF_SECRET_APPEND: ULONG = 0x2;
pub const KDF_HMAC_KEY: ULONG = 0x3;
pub const KDF_TLS_PRF_LABEL: ULONG = 0x4;
pub const KDF_TLS_PRF_SEED: ULONG = 0x5;
pub const KDF_SECRET_HANDLE: ULONG = 0x6;
pub const KDF_TLS_PRF_PROTOCOL: ULONG = 0x7;
pub const KDF_ALGORITHMID: ULONG = 0x8;
pub const KDF_PARTYUINFO: ULONG = 0x9;
pub const KDF_PARTYVINFO: ULONG = 0xA;
pub const KDF_SUPPPUBINFO: ULONG = 0xB;
pub const KDF_SUPPPRIVINFO: ULONG = 0xC;
pub const KDF_LABEL: ULONG = 0xD;
pub const KDF_CONTEXT: ULONG = 0xE;
pub const KDF_SALT: ULONG = 0xF;
pub const KDF_ITERATION_COUNT: ULONG = 0x10;
pub const KDF_GENERIC_PARAMETER: ULONG = 0x11;
pub const KDF_KEYBITLENGTH: ULONG = 0x12;
pub const KDF_USE_SECRET_AS_HMAC_KEY_FLAG: ULONG = 0x1;
STRUCT!{struct BCRYPT_KEY_LENGTHS_STRUCT {
    dwMinLength: ULONG,
    dwMaxLength: ULONG,
    dwIncrement: ULONG,
}}
pub type BCRYPT_AUTH_TAG_LENGTHS_STRUCT = BCRYPT_KEY_LENGTHS_STRUCT;
STRUCT!{struct BCRYPT_OID {
    cbOID: ULONG,
    pbOID: PUCHAR,
}}
STRUCT!{struct BCRYPT_OID_LIST {
    dwOIDCount: ULONG,
    pOIDs: *mut BCRYPT_OID,
}}
STRUCT!{struct BCRYPT_PKCS1_PADDING_INFO {
    pszAlgId: LPCWSTR,
}}
STRUCT!{struct BCRYPT_PSS_PADDING_INFO {
    pszAlgId: LPCWSTR,
    cbSalt: ULONG,
}}
STRUCT!{struct BCRYPT_OAEP_PADDING_INFO {
    pszAlgId: LPCWSTR,
    pbLabel: PUCHAR,
    cbLabel: ULONG,
}}
pub const BCRYPT_AUTHENTICATED_CIPHER_MODE_INFO_VERSION: ULONG = 1;
pub const BCRYPT_AUTH_MODE_CHAIN_CALLS_FLAG: ULONG = 0x00000001;
pub const BCRYPT_AUTH_MODE_IN_PROGRESS_FLAG: ULONG = 0x00000002;
STRUCT!{struct BCRYPT_AUTHENTICATED_CIPHER_MODE_INFO {
    cbSize: ULONG,
    dwInfoVersion: ULONG,
    pbNonce: PUCHAR,
    cbNonce: ULONG,
    pbAuthData: PUCHAR,
    cbAuthData: ULONG,
    pbTag: PUCHAR,
    cbTag: ULONG,
    pbMacContext: PUCHAR,
    cbMacContext: ULONG,
    cbAAD: ULONG,
    cbData: ULONGLONG,
    dwFlags: ULONG,
}}
pub type PBCRYPT_AUTHENTICATED_CIPHER_MODE_INFO = *mut BCRYPT_AUTHENTICATED_CIPHER_MODE_INFO;
pub const BCRYPT_OPAQUE_KEY_BLOB: &'static str = "OpaqueKeyBlob";
pub const BCRYPT_KEY_DATA_BLOB: &'static str = "KeyDataBlob";
pub const BCRYPT_AES_WRAP_KEY_BLOB: &'static str = "Rfc3565KeyWrapBlob";
pub const BCRYPT_OBJECT_LENGTH: &'static str = "ObjectLength";
pub const BCRYPT_ALGORITHM_NAME: &'static str = "AlgorithmName";
pub const BCRYPT_PROVIDER_HANDLE: &'static str = "ProviderHandle";
pub const BCRYPT_CHAINING_MODE: &'static str = "ChainingMode";
pub const BCRYPT_BLOCK_LENGTH: &'static str = "BlockLength";
pub const BCRYPT_KEY_LENGTH: &'static str = "KeyLength";
pub const BCRYPT_KEY_OBJECT_LENGTH: &'static str = "KeyObjectLength";
pub const BCRYPT_KEY_STRENGTH: &'static str = "KeyStrength";
pub const BCRYPT_KEY_LENGTHS: &'static str = "KeyLengths";
pub const BCRYPT_BLOCK_SIZE_LIST: &'static str = "BlockSizeList";
pub const BCRYPT_EFFECTIVE_KEY_LENGTH: &'static str = "EffectiveKeyLength";
pub const BCRYPT_HASH_LENGTH: &'static str = "HashDigestLength";
pub const BCRYPT_HASH_OID_LIST: &'static str = "HashOIDList";
pub const BCRYPT_PADDING_SCHEMES: &'static str = "PaddingSchemes";
pub const BCRYPT_SIGNATURE_LENGTH: &'static str = "SignatureLength";
pub const BCRYPT_HASH_BLOCK_LENGTH: &'static str = "HashBlockLength";
pub const BCRYPT_AUTH_TAG_LENGTH: &'static str = "AuthTagLength";
pub const BCRYPT_PRIMITIVE_TYPE: &'static str = "PrimitiveType";
pub const BCRYPT_IS_KEYED_HASH: &'static str = "IsKeyedHash";
pub const BCRYPT_IS_REUSABLE_HASH: &'static str = "IsReusableHash";
pub const BCRYPT_MESSAGE_BLOCK_LENGTH: &'static str = "MessageBlockLength";
pub const BCRYPT_PUBLIC_KEY_LENGTH: &'static str = "PublicKeyLength";
pub const BCRYPT_PCP_PLATFORM_TYPE_PROPERTY: &'static str = "PCP_PLATFORM_TYPE";
pub const BCRYPT_PCP_PROVIDER_VERSION_PROPERTY: &'static str = "PCP_PROVIDER_VERSION";
pub const BCRYPT_MULTI_OBJECT_LENGTH: &'static str = "MultiObjectLength";
pub const BCRYPT_INITIALIZATION_VECTOR: &'static str = "IV";
pub const BCRYPT_CHAIN_MODE_NA: &'static str = "ChainingModeN/A";
pub const BCRYPT_CHAIN_MODE_CBC: &'static str = "ChainingModeCBC";
pub const BCRYPT_CHAIN_MODE_ECB: &'static str = "ChainingModeECB";
pub const BCRYPT_CHAIN_MODE_CFB: &'static str = "ChainingModeCFB";
pub const BCRYPT_CHAIN_MODE_CCM: &'static str = "ChainingModeCCM";
pub const BCRYPT_CHAIN_MODE_GCM: &'static str = "ChainingModeGCM";
pub const BCRYPT_PROV_DISPATCH: ULONG = 0x00000001;
pub const BCRYPT_BLOCK_PADDING: ULONG = 0x00000001;
pub const BCRYPT_PAD_NONE: ULONG = 0x00000001;
pub const BCRYPT_PAD_PKCS1: ULONG = 0x00000002;
pub const BCRYPT_PAD_OAEP: ULONG = 0x00000004;
pub const BCRYPT_PAD_PSS: ULONG = 0x00000008;
pub const BCRYPT_PAD_PKCS1_OPTIONAL_HASH_OID: ULONG = 0x00000010;
pub const BCRYPTBUFFER_VERSION: ULONG = 0;
STRUCT!{struct BCryptBuffer {
    cbBuffer: ULONG,
    BufferType: ULONG,
    pvBuffer: PVOID,
}}
pub type PBCryptBuffer = *mut BCryptBuffer;
STRUCT!{struct BCryptBufferDesc {
    ulVersion: ULONG,
    cBuffers: ULONG,
    pBuffers: PBCryptBuffer,
}}
pub type PBCryptBufferDesc = *mut BCryptBufferDesc;
pub type BCRYPT_HANDLE = PVOID;
pub type BCRYPT_ALG_HANDLE = PVOID;
pub type BCRYPT_KEY_HANDLE = PVOID;
pub type BCRYPT_HASH_HANDLE = PVOID;
pub type BCRYPT_SECRET_HANDLE = PVOID;
pub const BCRYPT_PUBLIC_KEY_BLOB: &'static str = "PUBLICBLOB";
pub const BCRYPT_PRIVATE_KEY_BLOB: &'static str = "PRIVATEBLOB";
STRUCT!{struct BCRYPT_KEY_BLOB {
    Magic: ULONG,
}}
pub const BCRYPT_RSAPUBLIC_BLOB: &'static str = "RSAPUBLICBLOB";
pub const BCRYPT_RSAPRIVATE_BLOB: &'static str = "RSAPRIVATEBLOB";
pub const LEGACY_RSAPUBLIC_BLOB: &'static str = "CAPIPUBLICBLOB";
pub const LEGACY_RSAPRIVATE_BLOB: &'static str = "CAPIPRIVATEBLOB";
pub const BCRYPT_RSAPUBLIC_MAGIC: ULONG = 0x31415352;
pub const BCRYPT_RSAPRIVATE_MAGIC: ULONG = 0x32415352;
STRUCT!{struct BCRYPT_RSAKEY_BLOB {
    Magic: ULONG,
    BitLength: ULONG,
    cbPublicExp: ULONG,
    cbModulus: ULONG,
    cbPrime1: ULONG,
    cbPrime2: ULONG,
}}
pub const BCRYPT_RSAFULLPRIVATE_BLOB: &'static str = "RSAFULLPRIVATEBLOB";
pub const BCRYPT_RSAFULLPRIVATE_MAGIC: ULONG = 0x33415352;
pub const BCRYPT_GLOBAL_PARAMETERS: &'static str = "SecretAgreementParam";
pub const BCRYPT_PRIVATE_KEY: &'static str = "PrivKeyVal";
pub const BCRYPT_ECCPUBLIC_BLOB: &'static str = "ECCPUBLICBLOB";
pub const BCRYPT_ECCPRIVATE_BLOB: &'static str = "ECCPRIVATEBLOB";
pub const BCRYPT_ECCFULLPUBLIC_BLOB: &'static str = "ECCFULLPUBLICBLOB";
pub const BCRYPT_ECCFULLPRIVATE_BLOB: &'static str = "ECCFULLPRIVATEBLOB";
pub const SSL_ECCPUBLIC_BLOB: &'static str = "SSLECCPUBLICBLOB";
pub const BCRYPT_ECDH_PUBLIC_P256_MAGIC: ULONG = 0x314B4345;
pub const BCRYPT_ECDH_PRIVATE_P256_MAGIC: ULONG = 0x324B4345;
pub const BCRYPT_ECDH_PUBLIC_P384_MAGIC: ULONG = 0x334B4345;
pub const BCRYPT_ECDH_PRIVATE_P384_MAGIC: ULONG = 0x344B4345;
pub const BCRYPT_ECDH_PUBLIC_P521_MAGIC: ULONG = 0x354B4345;
pub const BCRYPT_ECDH_PRIVATE_P521_MAGIC: ULONG = 0x364B4345;
pub const BCRYPT_ECDH_PUBLIC_GENERIC_MAGIC: ULONG = 0x504B4345;
pub const BCRYPT_ECDH_PRIVATE_GENERIC_MAGIC: ULONG = 0x564B4345;
pub const BCRYPT_ECDSA_PUBLIC_P256_MAGIC: ULONG = 0x31534345;
pub const BCRYPT_ECDSA_PRIVATE_P256_MAGIC: ULONG = 0x32534345;
pub const BCRYPT_ECDSA_PUBLIC_P384_MAGIC: ULONG = 0x33534345;
pub const BCRYPT_ECDSA_PRIVATE_P384_MAGIC: ULONG = 0x34534345;
pub const BCRYPT_ECDSA_PUBLIC_P521_MAGIC: ULONG = 0x35534345;
pub const BCRYPT_ECDSA_PRIVATE_P521_MAGIC: ULONG = 0x36534345;
pub const BCRYPT_ECDSA_PUBLIC_GENERIC_MAGIC: ULONG = 0x50444345;
pub const BCRYPT_ECDSA_PRIVATE_GENERIC_MAGIC: ULONG = 0x56444345;
STRUCT!{struct BCRYPT_ECCKEY_BLOB {
    dwMagic: ULONG,
    cbKey: ULONG,
}}
pub type PBCRYPT_ECCKEY_BLOB = *mut BCRYPT_ECCKEY_BLOB;
STRUCT!{struct SSL_ECCKEY_BLOB {
    dwCurveType: ULONG,
    cbKey: ULONG,
}}
pub type PSSL_ECCKEY_BLOB = *mut SSL_ECCKEY_BLOB;
pub const BCRYPT_ECC_FULLKEY_BLOB_V1: ULONG = 0x1;
ENUM!{enum ECC_CURVE_TYPE_ENUM {
    BCRYPT_ECC_PRIME_SHORT_WEIERSTRASS_CURVE = 0x1,
    BCRYPT_ECC_PRIME_TWISTED_EDWARDS_CURVE = 0x2,
    BCRYPT_ECC_PRIME_MONTGOMERY_CURVE = 0x3,
}}
ENUM!{enum ECC_CURVE_ALG_ID_ENUM {
    BCRYPT_NO_CURVE_GENERATION_ALG_ID = 0x0,
}}
STRUCT!{struct BCRYPT_ECCFULLKEY_BLOB {
    dwMagic: ULONG,
    dwVersion: ULONG,
    dwCurveType: ECC_CURVE_TYPE_ENUM,
    dwCurveGenerationAlgId: ECC_CURVE_ALG_ID_ENUM,
    cbFieldLength: ULONG,
    cbSubgroupOrder: ULONG,
    cbCofactor: ULONG,
    cbSeed: ULONG,
}}
pub type PBCRYPT_ECCFULLKEY_BLOB = *mut BCRYPT_ECCFULLKEY_BLOB;
pub const BCRYPT_DH_PUBLIC_BLOB: &'static str = "DHPUBLICBLOB";
pub const BCRYPT_DH_PRIVATE_BLOB: &'static str = "DHPRIVATEBLOB";
pub const LEGACY_DH_PUBLIC_BLOB: &'static str = "CAPIDHPUBLICBLOB";
pub const LEGACY_DH_PRIVATE_BLOB: &'static str = "CAPIDHPRIVATEBLOB";
pub const BCRYPT_DH_PUBLIC_MAGIC: ULONG = 0x42504844;
pub const BCRYPT_DH_PRIVATE_MAGIC: ULONG = 0x56504844;
STRUCT!{struct BCRYPT_DH_KEY_BLOB {
    dwMagic: ULONG,
    cbKey: ULONG,
}}
pub type PBCRYPT_DH_KEY_BLOB = *mut BCRYPT_DH_KEY_BLOB;
pub const BCRYPT_DH_PARAMETERS: &'static str = "DHParameters";
pub const BCRYPT_DH_PARAMETERS_MAGIC: ULONG = 0x4d504844;
STRUCT!{struct BCRYPT_DH_PARAMETER_HEADER {
    cbLength: ULONG,
    dwMagic: ULONG,
    cbKeyLength: ULONG,
}}
pub const BCRYPT_DSA_PUBLIC_BLOB: &'static str = "DSAPUBLICBLOB";
pub const BCRYPT_DSA_PRIVATE_BLOB: &'static str = "DSAPRIVATEBLOB";
pub const LEGACY_DSA_PUBLIC_BLOB: &'static str = "CAPIDSAPUBLICBLOB";
pub const LEGACY_DSA_PRIVATE_BLOB: &'static str = "CAPIDSAPRIVATEBLOB";
pub const LEGACY_DSA_V2_PUBLIC_BLOB: &'static str = "V2CAPIDSAPUBLICBLOB";
pub const LEGACY_DSA_V2_PRIVATE_BLOB: &'static str = "V2CAPIDSAPRIVATEBLOB";
pub const BCRYPT_DSA_PUBLIC_MAGIC: ULONG = 0x42505344;
pub const BCRYPT_DSA_PRIVATE_MAGIC: ULONG = 0x56505344;
pub const BCRYPT_DSA_PUBLIC_MAGIC_V2: ULONG = 0x32425044;
pub const BCRYPT_DSA_PRIVATE_MAGIC_V2: ULONG = 0x32565044;
STRUCT!{struct BCRYPT_DSA_KEY_BLOB {
    dwMagic: ULONG,
    cbKey: ULONG,
    Count: [UCHAR; 4],
    Seed: [UCHAR; 20],
    q: [UCHAR; 20],
}}
pub type PBCRYPT_DSA_KEY_BLOB = *mut BCRYPT_DSA_KEY_BLOB;
ENUM!{enum HASHALGORITHM_ENUM {
    DSA_HASH_ALGORITHM_SHA1,
    DSA_HASH_ALGORITHM_SHA256,
    DSA_HASH_ALGORITHM_SHA512,
}}
ENUM!{enum DSAFIPSVERSION_ENUM {
    DSA_FIPS186_2,
    DSA_FIPS186_3,
}}
STRUCT!{struct BCRYPT_DSA_KEY_BLOB_V2 {
    dwMagic: ULONG,
    cbKey: ULONG,
    hashAlgorithm: HASHALGORITHM_ENUM,
    standardVersion: DSAFIPSVERSION_ENUM,
    cbSeedLength: ULONG,
    cbGroupSize: ULONG,
    Count: [UCHAR; 4],
}}
pub type PBCRYPT_DSA_KEY_BLOB_V2 = *mut BCRYPT_DSA_KEY_BLOB_V2;
STRUCT!{struct BCRYPT_KEY_DATA_BLOB_HEADER {
    dwMagic: ULONG,
    dwVersion: ULONG,
    cbKeyData: ULONG,
}}
pub type PBCRYPT_KEY_DATA_BLOB_HEADER = *mut BCRYPT_KEY_DATA_BLOB_HEADER;
pub const BCRYPT_KEY_DATA_BLOB_MAGIC: ULONG = 0x4d42444b;
pub const BCRYPT_KEY_DATA_BLOB_VERSION1: ULONG = 0x1;
pub const BCRYPT_DSA_PARAMETERS: &'static str = "DSAParameters";
pub const BCRYPT_DSA_PARAMETERS_MAGIC: ULONG = 0x4d505344;
pub const BCRYPT_DSA_PARAMETERS_MAGIC_V2: ULONG = 0x324d5044;
STRUCT!{struct BCRYPT_DSA_PARAMETER_HEADER {
    cbLength: ULONG,
    dwMagic: ULONG,
    cbKeyLength: ULONG,
    Count: [UCHAR; 4],
    Seed: [UCHAR; 20],
    q: [UCHAR; 20],
}}
STRUCT!{struct BCRYPT_DSA_PARAMETER_HEADER_V2 {
    cbLength: ULONG,
    dwMagic: ULONG,
    cbKeyLength: ULONG,
    hashAlgorithm: HASHALGORITHM_ENUM,
    standardVersion: DSAFIPSVERSION_ENUM,
    cbSeedLength: ULONG,
    cbGroupSize: ULONG,
    Count: [UCHAR; 4],
}}
pub const BCRYPT_ECC_PARAMETERS: &'static str = "ECCParameters";
pub const BCRYPT_ECC_CURVE_NAME: &'static str = "ECCCurveName";
pub const BCRYPT_ECC_CURVE_NAME_LIST: &'static str = "ECCCurveNameList";
pub const BCRYPT_ECC_PARAMETERS_MAGIC: ULONG = 0x50434345;
STRUCT!{struct BCRYPT_ECC_CURVE_NAMES {
    dwEccCurveNames: ULONG,
    pEccCurveNames: LPWSTR,
}}
pub const BCRYPT_ECC_CURVE_BRAINPOOLP160R1: &'static str = "brainpoolP160r1";
pub const BCRYPT_ECC_CURVE_BRAINPOOLP160T1: &'static str = "brainpoolP160t1";
pub const BCRYPT_ECC_CURVE_BRAINPOOLP192R1: &'static str = "brainpoolP192r1";
pub const BCRYPT_ECC_CURVE_BRAINPOOLP192T1: &'static str = "brainpoolP192t1";
pub const BCRYPT_ECC_CURVE_BRAINPOOLP224R1: &'static str = "brainpoolP224r1";
pub const BCRYPT_ECC_CURVE_BRAINPOOLP224T1: &'static str = "brainpoolP224t1";
pub const BCRYPT_ECC_CURVE_BRAINPOOLP256R1: &'static str = "brainpoolP256r1";
pub const BCRYPT_ECC_CURVE_BRAINPOOLP256T1: &'static str = "brainpoolP256t1";
pub const BCRYPT_ECC_CURVE_BRAINPOOLP320R1: &'static str = "brainpoolP320r1";
pub const BCRYPT_ECC_CURVE_BRAINPOOLP320T1: &'static str = "brainpoolP320t1";
pub const BCRYPT_ECC_CURVE_BRAINPOOLP384R1: &'static str = "brainpoolP384r1";
pub const BCRYPT_ECC_CURVE_BRAINPOOLP384T1: &'static str = "brainpoolP384t1";
pub const BCRYPT_ECC_CURVE_BRAINPOOLP512R1: &'static str = "brainpoolP512r1";
pub const BCRYPT_ECC_CURVE_BRAINPOOLP512T1: &'static str = "brainpoolP512t1";
pub const BCRYPT_ECC_CURVE_25519: &'static str = "curve25519";
pub const BCRYPT_ECC_CURVE_EC192WAPI: &'static str = "ec192wapi";
pub const BCRYPT_ECC_CURVE_NISTP192: &'static str = "nistP192";
pub const BCRYPT_ECC_CURVE_NISTP224: &'static str = "nistP224";
pub const BCRYPT_ECC_CURVE_NISTP256: &'static str = "nistP256";
pub const BCRYPT_ECC_CURVE_NISTP384: &'static str = "nistP384";
pub const BCRYPT_ECC_CURVE_NISTP521: &'static str = "nistP521";
pub const BCRYPT_ECC_CURVE_NUMSP256T1: &'static str = "numsP256t1";
pub const BCRYPT_ECC_CURVE_NUMSP384T1: &'static str = "numsP384t1";
pub const BCRYPT_ECC_CURVE_NUMSP512T1: &'static str = "numsP512t1";
pub const BCRYPT_ECC_CURVE_SECP160K1: &'static str = "secP160k1";
pub const BCRYPT_ECC_CURVE_SECP160R1: &'static str = "secP160r1";
pub const BCRYPT_ECC_CURVE_SECP160R2: &'static str = "secP160r2";
pub const BCRYPT_ECC_CURVE_SECP192K1: &'static str = "secP192k1";
pub const BCRYPT_ECC_CURVE_SECP192R1: &'static str = "secP192r1";
pub const BCRYPT_ECC_CURVE_SECP224K1: &'static str = "secP224k1";
pub const BCRYPT_ECC_CURVE_SECP224R1: &'static str = "secP224r1";
pub const BCRYPT_ECC_CURVE_SECP256K1: &'static str = "secP256k1";
pub const BCRYPT_ECC_CURVE_SECP256R1: &'static str = "secP256r1";
pub const BCRYPT_ECC_CURVE_SECP384R1: &'static str = "secP384r1";
pub const BCRYPT_ECC_CURVE_SECP521R1: &'static str = "secP521r1";
pub const BCRYPT_ECC_CURVE_WTLS7: &'static str = "wtls7";
pub const BCRYPT_ECC_CURVE_WTLS9: &'static str = "wtls9";
pub const BCRYPT_ECC_CURVE_WTLS12: &'static str = "wtls12";
pub const BCRYPT_ECC_CURVE_X962P192V1: &'static str = "x962P192v1";
pub const BCRYPT_ECC_CURVE_X962P192V2: &'static str = "x962P192v2";
pub const BCRYPT_ECC_CURVE_X962P192V3: &'static str = "x962P192v3";
pub const BCRYPT_ECC_CURVE_X962P239V1: &'static str = "x962P239v1";
pub const BCRYPT_ECC_CURVE_X962P239V2: &'static str = "x962P239v2";
pub const BCRYPT_ECC_CURVE_X962P239V3: &'static str = "x962P239v3";
pub const BCRYPT_ECC_CURVE_X962P256V1: &'static str = "x962P256v1";
ENUM!{enum BCRYPT_HASH_OPERATION_TYPE {
    BCRYPT_HASH_OPERATION_HASH_DATA = 1,
    BCRYPT_HASH_OPERATION_FINISH_HASH = 2,
}}
STRUCT!{struct BCRYPT_MULTI_HASH_OPERATION {
    iHash: ULONG,
    hashOperation: BCRYPT_HASH_OPERATION_TYPE,
    pbBuffer: PUCHAR,
    cbBuffer: ULONG,
}}
ENUM!{enum BCRYPT_MULTI_OPERATION_TYPE {
    BCRYPT_OPERATION_TYPE_HASH = 1,
}}
STRUCT!{struct BCRYPT_MULTI_OBJECT_LENGTH_STRUCT {
    cbPerObject: ULONG,
    cbPerElement: ULONG,
}}
pub const MS_PRIMITIVE_PROVIDER: &'static str = "Microsoft Primitive Provider";
pub const MS_PLATFORM_CRYPTO_PROVIDER: &'static str = "Microsoft Platform Crypto Provider";
pub const BCRYPT_RSA_ALGORITHM: &'static str = "RSA";
pub const BCRYPT_RSA_SIGN_ALGORITHM: &'static str = "RSA_SIGN";
pub const BCRYPT_DH_ALGORITHM: &'static str = "DH";
pub const BCRYPT_DSA_ALGORITHM: &'static str = "DSA";
pub const BCRYPT_RC2_ALGORITHM: &'static str = "RC2";
pub const BCRYPT_RC4_ALGORITHM: &'static str = "RC4";
pub const BCRYPT_AES_ALGORITHM: &'static str = "AES";
pub const BCRYPT_DES_ALGORITHM: &'static str = "DES";
pub const BCRYPT_DESX_ALGORITHM: &'static str = "DESX";
pub const BCRYPT_3DES_ALGORITHM: &'static str = "3DES";
pub const BCRYPT_3DES_112_ALGORITHM: &'static str = "3DES_112";
pub const BCRYPT_MD2_ALGORITHM: &'static str = "MD2";
pub const BCRYPT_MD4_ALGORITHM: &'static str = "MD4";
pub const BCRYPT_MD5_ALGORITHM: &'static str = "MD5";
pub const BCRYPT_SHA1_ALGORITHM: &'static str = "SHA1";
pub const BCRYPT_SHA256_ALGORITHM: &'static str = "SHA256";
pub const BCRYPT_SHA384_ALGORITHM: &'static str = "SHA384";
pub const BCRYPT_SHA512_ALGORITHM: &'static str = "SHA512";
pub const BCRYPT_AES_GMAC_ALGORITHM: &'static str = "AES-GMAC";
pub const BCRYPT_AES_CMAC_ALGORITHM: &'static str = "AES-CMAC";
pub const BCRYPT_ECDSA_P256_ALGORITHM: &'static str = "ECDSA_P256";
pub const BCRYPT_ECDSA_P384_ALGORITHM: &'static str = "ECDSA_P384";
pub const BCRYPT_ECDSA_P521_ALGORITHM: &'static str = "ECDSA_P521";
pub const BCRYPT_ECDH_P256_ALGORITHM: &'static str = "ECDH_P256";
pub const BCRYPT_ECDH_P384_ALGORITHM: &'static str = "ECDH_P384";
pub const BCRYPT_ECDH_P521_ALGORITHM: &'static str = "ECDH_P521";
pub const BCRYPT_RNG_ALGORITHM: &'static str = "RNG";
pub const BCRYPT_RNG_FIPS186_DSA_ALGORITHM: &'static str = "FIPS186DSARNG";
pub const BCRYPT_RNG_DUAL_EC_ALGORITHM: &'static str = "DUALECRNG";
pub const BCRYPT_SP800108_CTR_HMAC_ALGORITHM: &'static str = "SP800_108_CTR_HMAC";
pub const BCRYPT_SP80056A_CONCAT_ALGORITHM: &'static str = "SP800_56A_CONCAT";
pub const BCRYPT_PBKDF2_ALGORITHM: &'static str = "PBKDF2";
pub const BCRYPT_CAPI_KDF_ALGORITHM: &'static str = "CAPI_KDF";
pub const BCRYPT_TLS1_1_KDF_ALGORITHM: &'static str = "TLS1_1_KDF";
pub const BCRYPT_TLS1_2_KDF_ALGORITHM: &'static str = "TLS1_2_KDF";
pub const BCRYPT_ECDSA_ALGORITHM: &'static str = "ECDSA";
pub const BCRYPT_ECDH_ALGORITHM: &'static str = "ECDH";
pub const BCRYPT_XTS_AES_ALGORITHM: &'static str = "XTS-AES";
pub const BCRYPT_CIPHER_INTERFACE: ULONG = 0x00000001;
pub const BCRYPT_HASH_INTERFACE: ULONG = 0x00000002;
pub const BCRYPT_ASYMMETRIC_ENCRYPTION_INTERFACE: ULONG = 0x00000003;
pub const BCRYPT_SECRET_AGREEMENT_INTERFACE: ULONG = 0x00000004;
pub const BCRYPT_SIGNATURE_INTERFACE: ULONG = 0x00000005;
pub const BCRYPT_RNG_INTERFACE: ULONG = 0x00000006;
pub const BCRYPT_KEY_DERIVATION_INTERFACE: ULONG = 0x00000007;
pub const BCRYPT_MD2_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x00000001 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_MD4_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x00000011 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_MD5_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x00000021 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_SHA1_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x00000031 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_SHA256_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x00000041 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_SHA384_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x00000051 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_SHA512_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x00000061 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_RC4_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x00000071 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_RNG_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x00000081 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_HMAC_MD5_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x00000091 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_HMAC_SHA1_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x000000a1 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_HMAC_SHA256_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x000000b1 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_HMAC_SHA384_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x000000c1 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_HMAC_SHA512_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x000000d1 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_RSA_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x000000e1 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_ECDSA_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x000000f1 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_AES_CMAC_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x00000101 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_AES_GMAC_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x00000111 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_HMAC_MD2_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x00000121 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_HMAC_MD4_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x00000131 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_3DES_CBC_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x00000141 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_3DES_ECB_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x00000151 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_3DES_CFB_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x00000161 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_3DES_112_CBC_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x00000171 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_3DES_112_ECB_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x00000181 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_3DES_112_CFB_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x00000191 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_AES_CBC_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x000001a1 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_AES_ECB_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x000001b1 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_AES_CFB_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x000001c1 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_AES_CCM_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x000001d1 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_AES_GCM_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x000001e1 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_DES_CBC_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x000001f1 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_DES_ECB_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x00000201 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_DES_CFB_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x00000211 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_DESX_CBC_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x00000221 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_DESX_ECB_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x00000231 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_DESX_CFB_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x00000241 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_RC2_CBC_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x00000251 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_RC2_ECB_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x00000261 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_RC2_CFB_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x00000271 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_DH_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x00000281 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_ECDH_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x00000291 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_ECDH_P256_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x000002a1 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_ECDH_P384_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x000002b1 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_ECDH_P521_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x000002c1 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_DSA_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x000002d1 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_ECDSA_P256_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x000002e1 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_ECDSA_P384_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x000002f1 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_ECDSA_P521_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x00000301 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_RSA_SIGN_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x00000311 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_CAPI_KDF_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x00000321 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_PBKDF2_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x00000331 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_SP800108_CTR_HMAC_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x00000341 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_SP80056A_CONCAT_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x00000351 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_TLS1_1_KDF_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x00000361 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_TLS1_2_KDF_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x00000371 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_XTS_AES_ALG_HANDLE: BCRYPT_ALG_HANDLE = 0x00000381 as BCRYPT_ALG_HANDLE;
pub const BCRYPT_ALG_HANDLE_HMAC_FLAG: ULONG = 0x00000008;
pub const BCRYPT_CAPI_AES_FLAG: ULONG = 0x00000010;
pub const BCRYPT_HASH_REUSABLE_FLAG: ULONG = 0x00000020;
pub const BCRYPT_BUFFERS_LOCKED_FLAG: ULONG = 0x00000040;
pub const BCRYPT_EXTENDED_KEYSIZE: ULONG = 0x00000080;
pub const BCRYPT_ENABLE_INCOMPATIBLE_FIPS_CHECKS: ULONG = 0x00000100;
extern "system" {
    pub fn BCryptOpenAlgorithmProvider(
        phAlgorithm: *mut BCRYPT_ALG_HANDLE,
        pszAlgId: LPCWSTR,
        pszImplementation: LPCWSTR,
        dwFlags: ULONG,
    ) -> NTSTATUS;
}
pub const BCRYPT_CIPHER_OPERATION: ULONG = 0x00000001;
pub const BCRYPT_HASH_OPERATION: ULONG = 0x00000002;
pub const BCRYPT_ASYMMETRIC_ENCRYPTION_OPERATION: ULONG = 0x00000004;
pub const BCRYPT_SECRET_AGREEMENT_OPERATION: ULONG = 0x00000008;
pub const BCRYPT_SIGNATURE_OPERATION: ULONG = 0x00000010;
pub const BCRYPT_RNG_OPERATION: ULONG = 0x00000020;
pub const BCRYPT_KEY_DERIVATION_OPERATION: ULONG = 0x00000040;
STRUCT!{struct BCRYPT_ALGORITHM_IDENTIFIER {
    pszName: LPWSTR,
    dwClass: ULONG,
    dwFlags: ULONG,
}}
extern "system" {
    pub fn BCryptEnumAlgorithms(
        dwAlgOperations: ULONG,
        pAlgCount: *mut ULONG,
        ppAlgList: *mut *mut BCRYPT_ALGORITHM_IDENTIFIER,
        dwFlags: ULONG,
    ) -> NTSTATUS;
}
STRUCT!{struct BCRYPT_PROVIDER_NAME {
    pszProviderName: LPWSTR,
}}
extern "system" {
    pub fn BCryptEnumProviders(
        pszAlgId: LPCWSTR,
        pImplCount: *mut ULONG,
        ppImplList: *mut *mut BCRYPT_PROVIDER_NAME,
        dwFlags: ULONG,
    ) -> NTSTATUS;
}
pub const BCRYPT_PUBLIC_KEY_FLAG: ULONG = 0x00000001;
pub const BCRYPT_PRIVATE_KEY_FLAG: ULONG = 0x00000002;
extern "system" {
    pub fn BCryptGetProperty(
        hObject: BCRYPT_HANDLE,
        pszProperty: LPCWSTR,
        pbOutput: PUCHAR,
        cbOutput: ULONG,
        pcbResult: *mut ULONG,
        dwFlags: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptSetProperty(
        hObject: BCRYPT_HANDLE,
        pszProperty: LPCWSTR,
        pbInput: PUCHAR,
        cbInput: ULONG,
        dwFlags: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptCloseAlgorithmProvider(
        hAlgorithm: BCRYPT_ALG_HANDLE,
        dwFlags: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptFreeBuffer(
        pvBuffer: PVOID,
    );
    pub fn BCryptGenerateSymmetricKey(
        hAlgorithm: BCRYPT_ALG_HANDLE,
        phKey: *mut BCRYPT_KEY_HANDLE,
        pbKeyObject: PUCHAR,
        cbKeyObject: ULONG,
        pbSecret: PUCHAR,
        cbSecret: ULONG,
        dwFlags: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptGenerateKeyPair(
        hAlgorithm: BCRYPT_ALG_HANDLE,
        phKey: *mut BCRYPT_KEY_HANDLE,
        dwLength: ULONG,
        dwFlags: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptEncrypt(
        hKey: BCRYPT_KEY_HANDLE,
        pbInput: PUCHAR,
        cbInput: ULONG,
        pPaddingInfo: *mut VOID,
        pbIV: PUCHAR,
        cbIV: ULONG,
        pbOutput: PUCHAR,
        cbOutput: ULONG,
        pcbResult: *mut ULONG,
        dwFlags: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptDecrypt(
        hKey: BCRYPT_KEY_HANDLE,
        pbInput: PUCHAR,
        cbInput: ULONG,
        pPaddingInfo: *mut VOID,
        pbIV: PUCHAR,
        cbIV: ULONG,
        pbOutput: PUCHAR,
        cbOutput: ULONG,
        pcbResult: *mut ULONG,
        dwFlags: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptExportKey(
        hKey: BCRYPT_KEY_HANDLE,
        hExportKey: BCRYPT_KEY_HANDLE,
        pszBlobType: LPCWSTR,
        pbOutput: PUCHAR,
        cbOutput: ULONG,
        pcbResult: *mut ULONG,
        dwFlags: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptImportKey(
        hAlgorithm: BCRYPT_ALG_HANDLE,
        hImportKey: BCRYPT_KEY_HANDLE,
        pszBlobType: LPCWSTR,
        phKey: *mut BCRYPT_KEY_HANDLE,
        pbKeyObject: PUCHAR,
        cbKeyObject: ULONG,
        pbInput: PUCHAR,
        cbInput: ULONG,
        dwFlags: ULONG,
    ) -> NTSTATUS;
}
pub const BCRYPT_NO_KEY_VALIDATION: ULONG = 0x00000008;
extern "system" {
    pub fn BCryptImportKeyPair(
        hAlgorithm: BCRYPT_ALG_HANDLE,
        hImportKey: BCRYPT_KEY_HANDLE,
        pszBlobType: LPCWSTR,
        phKey: *mut BCRYPT_KEY_HANDLE,
        pbInput: PUCHAR,
        cbInput: ULONG,
        dwFlags: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptDuplicateKey(
        hKey: BCRYPT_KEY_HANDLE,
        phNewKey: *mut BCRYPT_KEY_HANDLE,
        pbKeyObject: PUCHAR,
        cbKeyObject: ULONG,
        dwFlags: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptFinalizeKeyPair(
        hKey: BCRYPT_KEY_HANDLE,
        dwFlags: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptDestroyKey(
        hKey: BCRYPT_KEY_HANDLE,
    ) -> NTSTATUS;
    pub fn BCryptDestroySecret(
        hSecret: BCRYPT_SECRET_HANDLE,
    ) -> NTSTATUS;
    pub fn BCryptSignHash(
        hKey: BCRYPT_KEY_HANDLE,
        pPaddingInfo: *mut VOID,
        pbInput: PUCHAR,
        cbInput: ULONG,
        pbOutput: PUCHAR,
        cbOutput: ULONG,
        pcbResult: *mut ULONG,
        dwFlags: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptVerifySignature(
        hKey: BCRYPT_KEY_HANDLE,
        pPaddingInfo: *mut VOID,
        pbHash: PUCHAR,
        cbHash: ULONG,
        pbSignature: PUCHAR,
        cbSignature: ULONG,
        dwFlags: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptSecretAgreement(
        hPrivKey: BCRYPT_KEY_HANDLE,
        hPubKey: BCRYPT_KEY_HANDLE,
        phAgreedSecret: *mut BCRYPT_SECRET_HANDLE,
        dwFlags: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptDeriveKey(
        hSharedSecret: BCRYPT_SECRET_HANDLE,
        pwszKDF: LPCWSTR,
        pParameterList: *mut BCryptBufferDesc,
        pbDerivedKey: PUCHAR,
        cbDerivedKey: ULONG,
        pcbResult: *mut ULONG,
        dwFlags: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptKeyDerivation(
        hKey: BCRYPT_KEY_HANDLE,
        pParameterList: *mut BCryptBufferDesc,
        pbDerivedKey: PUCHAR,
        cbDerivedKey: ULONG,
        pcbResult: *mut ULONG,
        dwFlags: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptCreateHash(
        hAlgorithm: BCRYPT_ALG_HANDLE,
        phHash: *mut BCRYPT_HASH_HANDLE,
        pbHashObject: PUCHAR,
        cbHashObject: ULONG,
        pbSecret: PUCHAR,
        cbSecret: ULONG,
        dwFlags: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptHashData(
        hHash: BCRYPT_HASH_HANDLE,
        pbInput: PUCHAR,
        cbInput: ULONG,
        dwFlags: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptFinishHash(
        hHash: BCRYPT_HASH_HANDLE,
        pbOutput: PUCHAR,
        cbOutput: ULONG,
        dwFlags: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptCreateMultiHash(
        hAlgorithm: BCRYPT_ALG_HANDLE,
        phHash: *mut BCRYPT_HASH_HANDLE,
        nHashes: ULONG,
        pbHashObject: PUCHAR,
        cbHashObject: ULONG,
        pbSecret: PUCHAR,
        cbSecret: ULONG,
        dwFlags: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptProcessMultiOperations(
        hObject: BCRYPT_HANDLE,
        operationType: BCRYPT_MULTI_OPERATION_TYPE,
        pOperations: PVOID,
        cbOperations: ULONG,
        dwFlags: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptDuplicateHash(
        hHash: BCRYPT_HASH_HANDLE,
        phNewHash: *mut BCRYPT_HASH_HANDLE,
        pbHashObject: PUCHAR,
        cbHashObject: ULONG,
        dwFlags: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptDestroyHash(
        hHash: BCRYPT_HASH_HANDLE,
    ) -> NTSTATUS;
    pub fn BCryptHash(
        hAlgorithm: BCRYPT_ALG_HANDLE,
        pbSecret: PUCHAR,
        cbSecret: ULONG,
        pbInput: PUCHAR,
        cbInput: ULONG,
        pbOutput: PUCHAR,
        cbOutput: ULONG,
    ) -> NTSTATUS;
}
pub const BCRYPT_RNG_USE_ENTROPY_IN_BUFFER: ULONG = 0x00000001;
pub const BCRYPT_USE_SYSTEM_PREFERRED_RNG: ULONG = 0x00000002;
extern "system" {
    pub fn BCryptGenRandom(
        hAlgorithm: BCRYPT_ALG_HANDLE,
        pbBuffer: PUCHAR,
        cbBuffer: ULONG,
        dwFlags: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptDeriveKeyCapi(
        hHash: BCRYPT_HASH_HANDLE,
        hTargetAlg: BCRYPT_ALG_HANDLE,
        pbDerivedKey: PUCHAR,
        cbDerivedKey: ULONG,
        dwFlags: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptDeriveKeyPBKDF2(
        hPrf: BCRYPT_ALG_HANDLE,
        pbPassword: PUCHAR,
        cbPassword: ULONG,
        pbSalt: PUCHAR,
        cbSalt: ULONG,
        cIterations: ULONGLONG,
        pbDerivedKey: PUCHAR,
        cbDerivedKey: ULONG,
        dwFlags: ULONG,
    ) -> NTSTATUS;
}
STRUCT!{struct BCRYPT_INTERFACE_VERSION {
    MajorVersion: USHORT,
    MinorVersion: USHORT,
}}
pub type PBCRYPT_INTERFACE_VERSION = *mut BCRYPT_INTERFACE_VERSION;
#[inline]
pub fn BCRYPT_IS_INTERFACE_VERSION_COMPATIBLE(
    loader: BCRYPT_INTERFACE_VERSION,
    provider: BCRYPT_INTERFACE_VERSION,
) -> bool {
    loader.MajorVersion <= provider.MajorVersion
}
pub const BCRYPT_CIPHER_INTERFACE_VERSION_1: BCRYPT_INTERFACE_VERSION =
    BCRYPT_MAKE_INTERFACE_VERSION!(1, 0);
pub const BCRYPT_HASH_INTERFACE_VERSION_1: BCRYPT_INTERFACE_VERSION =
    BCRYPT_MAKE_INTERFACE_VERSION!(1, 0);
pub const BCRYPT_HASH_INTERFACE_MAJORVERSION_2: USHORT = 2;
pub const BCRYPT_HASH_INTERFACE_VERSION_2: BCRYPT_INTERFACE_VERSION =
    BCRYPT_MAKE_INTERFACE_VERSION!(BCRYPT_HASH_INTERFACE_MAJORVERSION_2, 0);
pub const BCRYPT_ASYMMETRIC_ENCRYPTION_INTERFACE_VERSION_1: BCRYPT_INTERFACE_VERSION =
    BCRYPT_MAKE_INTERFACE_VERSION!(1, 0);
pub const BCRYPT_SECRET_AGREEMENT_INTERFACE_VERSION_1: BCRYPT_INTERFACE_VERSION =
    BCRYPT_MAKE_INTERFACE_VERSION!(1, 0);
pub const BCRYPT_SIGNATURE_INTERFACE_VERSION_1: BCRYPT_INTERFACE_VERSION =
    BCRYPT_MAKE_INTERFACE_VERSION!(1, 0);
pub const BCRYPT_RNG_INTERFACE_VERSION_1: BCRYPT_INTERFACE_VERSION =
    BCRYPT_MAKE_INTERFACE_VERSION!(1, 0);
pub const CRYPT_MIN_DEPENDENCIES: ULONG = 0x00000001;
pub const CRYPT_PROCESS_ISOLATE: ULONG = 0x00010000;
pub const CRYPT_UM: ULONG = 0x00000001;
pub const CRYPT_KM: ULONG = 0x00000002;
pub const CRYPT_MM: ULONG = 0x00000003;
pub const CRYPT_ANY: ULONG = 0x00000004;
pub const CRYPT_OVERWRITE: ULONG = 0x00000001;
pub const CRYPT_LOCAL: ULONG = 0x00000001;
pub const CRYPT_DOMAIN: ULONG = 0x00000002;
pub const CRYPT_EXCLUSIVE: ULONG = 0x00000001;
pub const CRYPT_OVERRIDE: ULONG = 0x00010000;
pub const CRYPT_ALL_FUNCTIONS: ULONG = 0x00000001;
pub const CRYPT_ALL_PROVIDERS: ULONG = 0x00000002;
pub const CRYPT_PRIORITY_TOP: ULONG = 0x00000000;
pub const CRYPT_PRIORITY_BOTTOM: ULONG = 0xFFFFFFFF;
pub const CRYPT_DEFAULT_CONTEXT: &'static str = "Default";
STRUCT!{struct CRYPT_INTERFACE_REG {
    dwInterface: ULONG,
    dwFlags: ULONG,
    cFunctions: ULONG,
    rgpszFunctions: *mut PWSTR,
}}
pub type PCRYPT_INTERFACE_REG = *mut CRYPT_INTERFACE_REG;
STRUCT!{struct CRYPT_IMAGE_REG {
    pszImage: PWSTR,
    cInterfaces: ULONG,
    rgpInterfaces: *mut PCRYPT_INTERFACE_REG,
}}
pub type PCRYPT_IMAGE_REG = *mut CRYPT_IMAGE_REG;
STRUCT!{struct CRYPT_PROVIDER_REG {
    cAliases: ULONG,
    rgpszAliases: *mut PWSTR,
    pUM: PCRYPT_IMAGE_REG,
    pKM: PCRYPT_IMAGE_REG,
}}
pub type PCRYPT_PROVIDER_REG = *mut CRYPT_PROVIDER_REG;
STRUCT!{struct CRYPT_PROVIDERS {
    cProviders: ULONG,
    rgpszProviders: *mut PWSTR,
}}
pub type PCRYPT_PROVIDERS = *mut CRYPT_PROVIDERS;
STRUCT!{struct CRYPT_CONTEXT_CONFIG {
    dwFlags: ULONG,
    dwReserved: ULONG,
}}
pub type PCRYPT_CONTEXT_CONFIG = *mut CRYPT_CONTEXT_CONFIG;
STRUCT!{struct CRYPT_CONTEXT_FUNCTION_CONFIG {
    dwFlags: ULONG,
    dwReserved: ULONG,
}}
pub type PCRYPT_CONTEXT_FUNCTION_CONFIG = *mut CRYPT_CONTEXT_FUNCTION_CONFIG;
STRUCT!{struct CRYPT_CONTEXTS {
    cContexts: ULONG,
    rgpszContexts: *mut PWSTR,
}}
pub type PCRYPT_CONTEXTS = *mut CRYPT_CONTEXTS;
STRUCT!{struct CRYPT_CONTEXT_FUNCTIONS {
    cFunctions: ULONG,
    rgpszFunctions: *mut PWSTR,
}}
pub type PCRYPT_CONTEXT_FUNCTIONS = *mut CRYPT_CONTEXT_FUNCTIONS;
STRUCT!{struct CRYPT_CONTEXT_FUNCTION_PROVIDERS {
    cProviders: ULONG,
    rgpszProviders: *mut PWSTR,
}}
pub type PCRYPT_CONTEXT_FUNCTION_PROVIDERS = *mut CRYPT_CONTEXT_FUNCTION_PROVIDERS;
STRUCT!{struct CRYPT_PROPERTY_REF {
    pszProperty: PWSTR,
    cbValue: ULONG,
    pbValue: PUCHAR,
}}
pub type PCRYPT_PROPERTY_REF = *mut CRYPT_PROPERTY_REF;
STRUCT!{struct CRYPT_IMAGE_REF {
    pszImage: PWSTR,
    dwFlags: ULONG,
}}
pub type PCRYPT_IMAGE_REF = *mut CRYPT_IMAGE_REF;
STRUCT!{struct CRYPT_PROVIDER_REF {
    dwInterface: ULONG,
    pszFunction: PWSTR,
    pszProvider: PWSTR,
    cProperties: ULONG,
    rgpProperties: *mut PCRYPT_PROPERTY_REF,
    pUM: PCRYPT_IMAGE_REF,
    pKM: PCRYPT_IMAGE_REF,
}}
pub type PCRYPT_PROVIDER_REF = *mut CRYPT_PROVIDER_REF;
STRUCT!{struct CRYPT_PROVIDER_REFS {
    cProviders: ULONG,
    rgpProviders: *mut PCRYPT_PROVIDER_REF,
}}
pub type PCRYPT_PROVIDER_REFS = *mut CRYPT_PROVIDER_REFS;
extern "system" {
    pub fn BCryptQueryProviderRegistration(
        pszProvider: LPCWSTR,
        dwMode: ULONG,
        dwInterface: ULONG,
        pcbBuffer: *mut ULONG,
        ppBuffer: *mut PCRYPT_PROVIDER_REG,
    ) -> NTSTATUS;
    pub fn BCryptEnumRegisteredProviders(
        pcbBuffer: *mut ULONG,
        ppBuffer: *mut PCRYPT_PROVIDERS,
    ) -> NTSTATUS;
    pub fn BCryptCreateContext(
        dwTable: ULONG,
        pszContext: LPCWSTR,
        pConfig: PCRYPT_CONTEXT_CONFIG,
    ) -> NTSTATUS;
    pub fn BCryptDeleteContext(
        dwTable: ULONG,
        pszContext: LPCWSTR,
    ) -> NTSTATUS;
    pub fn BCryptEnumContexts(
        dwTable: ULONG,
        pcbBuffer: *mut ULONG,
        ppBuffer: *mut PCRYPT_CONTEXTS,
    ) -> NTSTATUS;
    pub fn BCryptConfigureContext(
        dwTable: ULONG,
        pszContext: LPCWSTR,
        pConfig: PCRYPT_CONTEXT_CONFIG,
    ) -> NTSTATUS;
    pub fn BCryptQueryContextConfiguration(
        dwTable: ULONG,
        pszContext: LPCWSTR,
        pcbBuffer: *mut ULONG,
        ppBuffer: *mut PCRYPT_CONTEXT_CONFIG,
    ) -> NTSTATUS;
    pub fn BCryptAddContextFunction(
        dwTable: ULONG,
        pszContext: LPCWSTR,
        dwInterface: ULONG,
        pszFunction: LPCWSTR,
        dwPosition: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptRemoveContextFunction(
        dwTable: ULONG,
        pszContext: LPCWSTR,
        dwInterface: ULONG,
        pszFunction: LPCWSTR,
    ) -> NTSTATUS;
    pub fn BCryptEnumContextFunctions(
        dwTable: ULONG,
        pszContext: LPCWSTR,
        dwInterface: ULONG,
        pcbBuffer: *mut ULONG,
        ppBuffer: *mut PCRYPT_CONTEXT_FUNCTIONS,
    ) -> NTSTATUS;
    pub fn BCryptConfigureContextFunction(
        dwTable: ULONG,
        pszContext: LPCWSTR,
        dwInterface: ULONG,
        pszFunction: LPCWSTR,
        pConfig: PCRYPT_CONTEXT_FUNCTION_CONFIG,
    ) -> NTSTATUS;
    pub fn BCryptQueryContextFunctionConfiguration(
        dwTable: ULONG,
        pszContext: LPCWSTR,
        dwInterface: ULONG,
        pszFunction: LPCWSTR,
        pcbBuffer: *mut ULONG,
        ppBuffer: *mut PCRYPT_CONTEXT_FUNCTION_CONFIG,
    ) -> NTSTATUS;
    pub fn BCryptEnumContextFunctionProviders(
        dwTable: ULONG,
        pszContext: LPCWSTR,
        dwInterface: ULONG,
        pszFunction: LPCWSTR,
        pcbBuffer: *mut ULONG,
        ppBuffer: *mut PCRYPT_CONTEXT_FUNCTION_PROVIDERS,
    ) -> NTSTATUS;
    pub fn BCryptSetContextFunctionProperty(
        dwTable: ULONG,
        pszContext: LPCWSTR,
        dwInterface: ULONG,
        pszFunction: LPCWSTR,
        pszProperty: LPCWSTR,
        cbValue: ULONG,
        pbValue: PUCHAR,
    ) -> NTSTATUS;
    pub fn BCryptQueryContextFunctionProperty(
        dwTable: ULONG,
        pszContext: LPCWSTR,
        dwInterface: ULONG,
        pszFunction: LPCWSTR,
        pszProperty: LPCWSTR,
        pcbValue: *mut ULONG,
        ppbValue: *mut PUCHAR,
    ) -> NTSTATUS;
    pub fn BCryptRegisterConfigChangeNotify(
        phEvent: *mut HANDLE,
    ) -> NTSTATUS;
    pub fn BCryptUnregisterConfigChangeNotify(
        hEvent: HANDLE,
    ) -> NTSTATUS;
    pub fn BCryptResolveProviders(
        pszContext: LPCWSTR,
        dwInterface: ULONG,
        pszFunction: LPCWSTR,
        pszProvider: LPCWSTR,
        dwMode: ULONG,
        dwFlags: ULONG,
        pcbBuffer: *mut ULONG,
        ppBuffer: *mut PCRYPT_PROVIDER_REFS,
    ) -> NTSTATUS;
    pub fn BCryptGetFipsAlgorithmMode(
        pfEnabled: *mut BOOLEAN,
    ) -> NTSTATUS;
    pub fn CngGetFipsAlgorithmMode() -> BOOLEAN;
}
