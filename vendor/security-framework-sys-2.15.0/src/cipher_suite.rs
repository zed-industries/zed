#[cfg(not(target_os = "macos"))]
pub type SSLCipherSuite = u16;

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub type SSLCipherSuite = u16;

#[cfg(all(target_os = "macos", not(target_arch = "aarch64")))]
pub type SSLCipherSuite = u32;

pub const SSL_NULL_WITH_NULL_NULL: SSLCipherSuite = 0x0000;
pub const SSL_RSA_WITH_NULL_MD5: SSLCipherSuite = 0x0001;
pub const SSL_RSA_WITH_NULL_SHA: SSLCipherSuite = 0x0002;
pub const SSL_RSA_EXPORT_WITH_RC4_40_MD5: SSLCipherSuite = 0x0003;
pub const SSL_RSA_WITH_RC4_128_MD5: SSLCipherSuite = 0x0004;
pub const SSL_RSA_WITH_RC4_128_SHA: SSLCipherSuite = 0x0005;
pub const SSL_RSA_EXPORT_WITH_RC2_CBC_40_MD5: SSLCipherSuite = 0x0006;
pub const SSL_RSA_WITH_IDEA_CBC_SHA: SSLCipherSuite = 0x0007;
pub const SSL_RSA_EXPORT_WITH_DES40_CBC_SHA: SSLCipherSuite = 0x0008;
pub const SSL_RSA_WITH_DES_CBC_SHA: SSLCipherSuite = 0x0009;
pub const SSL_RSA_WITH_3DES_EDE_CBC_SHA: SSLCipherSuite = 0x000A;
pub const SSL_DH_DSS_EXPORT_WITH_DES40_CBC_SHA: SSLCipherSuite = 0x000B;
pub const SSL_DH_DSS_WITH_DES_CBC_SHA: SSLCipherSuite = 0x000C;
pub const SSL_DH_DSS_WITH_3DES_EDE_CBC_SHA: SSLCipherSuite = 0x000D;
pub const SSL_DH_RSA_EXPORT_WITH_DES40_CBC_SHA: SSLCipherSuite = 0x000E;
pub const SSL_DH_RSA_WITH_DES_CBC_SHA: SSLCipherSuite = 0x000F;
pub const SSL_DH_RSA_WITH_3DES_EDE_CBC_SHA: SSLCipherSuite = 0x0010;
pub const SSL_DHE_DSS_EXPORT_WITH_DES40_CBC_SHA: SSLCipherSuite = 0x0011;
pub const SSL_DHE_DSS_WITH_DES_CBC_SHA: SSLCipherSuite = 0x0012;
pub const SSL_DHE_DSS_WITH_3DES_EDE_CBC_SHA: SSLCipherSuite = 0x0013;
pub const SSL_DHE_RSA_EXPORT_WITH_DES40_CBC_SHA: SSLCipherSuite = 0x0014;
pub const SSL_DHE_RSA_WITH_DES_CBC_SHA: SSLCipherSuite = 0x0015;
pub const SSL_DHE_RSA_WITH_3DES_EDE_CBC_SHA: SSLCipherSuite = 0x0016;
pub const SSL_DH_anon_EXPORT_WITH_RC4_40_MD5: SSLCipherSuite = 0x0017;
pub const SSL_DH_anon_WITH_RC4_128_MD5: SSLCipherSuite = 0x0018;
pub const SSL_DH_anon_EXPORT_WITH_DES40_CBC_SHA: SSLCipherSuite = 0x0019;
pub const SSL_DH_anon_WITH_DES_CBC_SHA: SSLCipherSuite = 0x001A;
pub const SSL_DH_anon_WITH_3DES_EDE_CBC_SHA: SSLCipherSuite = 0x001B;
pub const SSL_FORTEZZA_DMS_WITH_NULL_SHA: SSLCipherSuite = 0x001C;
pub const SSL_FORTEZZA_DMS_WITH_FORTEZZA_CBC_SHA: SSLCipherSuite = 0x001D;

// TLS addenda using AES, per RFC 3268
pub const TLS_RSA_WITH_AES_128_CBC_SHA: SSLCipherSuite = 0x002F;
pub const TLS_DH_DSS_WITH_AES_128_CBC_SHA: SSLCipherSuite = 0x0030;
pub const TLS_DH_RSA_WITH_AES_128_CBC_SHA: SSLCipherSuite = 0x0031;
pub const TLS_DHE_DSS_WITH_AES_128_CBC_SHA: SSLCipherSuite = 0x0032;
pub const TLS_DHE_RSA_WITH_AES_128_CBC_SHA: SSLCipherSuite = 0x0033;
pub const TLS_DH_anon_WITH_AES_128_CBC_SHA: SSLCipherSuite = 0x0034;
pub const TLS_RSA_WITH_AES_256_CBC_SHA: SSLCipherSuite = 0x0035;
pub const TLS_DH_DSS_WITH_AES_256_CBC_SHA: SSLCipherSuite = 0x0036;
pub const TLS_DH_RSA_WITH_AES_256_CBC_SHA: SSLCipherSuite = 0x0037;
pub const TLS_DHE_DSS_WITH_AES_256_CBC_SHA: SSLCipherSuite = 0x0038;
pub const TLS_DHE_RSA_WITH_AES_256_CBC_SHA: SSLCipherSuite = 0x0039;
pub const TLS_DH_anon_WITH_AES_256_CBC_SHA: SSLCipherSuite = 0x003A;

// ECDSA addenda, RFC 4492
pub const TLS_ECDH_ECDSA_WITH_NULL_SHA: SSLCipherSuite = 0xC001;
pub const TLS_ECDH_ECDSA_WITH_RC4_128_SHA: SSLCipherSuite = 0xC002;
pub const TLS_ECDH_ECDSA_WITH_3DES_EDE_CBC_SHA: SSLCipherSuite = 0xC003;
pub const TLS_ECDH_ECDSA_WITH_AES_128_CBC_SHA: SSLCipherSuite = 0xC004;
pub const TLS_ECDH_ECDSA_WITH_AES_256_CBC_SHA: SSLCipherSuite = 0xC005;
pub const TLS_ECDHE_ECDSA_WITH_NULL_SHA: SSLCipherSuite = 0xC006;
pub const TLS_ECDHE_ECDSA_WITH_RC4_128_SHA: SSLCipherSuite = 0xC007;
pub const TLS_ECDHE_ECDSA_WITH_3DES_EDE_CBC_SHA: SSLCipherSuite = 0xC008;
pub const TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA: SSLCipherSuite = 0xC009;
pub const TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA: SSLCipherSuite = 0xC00A;
pub const TLS_ECDH_RSA_WITH_NULL_SHA: SSLCipherSuite = 0xC00B;
pub const TLS_ECDH_RSA_WITH_RC4_128_SHA: SSLCipherSuite = 0xC00C;
pub const TLS_ECDH_RSA_WITH_3DES_EDE_CBC_SHA: SSLCipherSuite = 0xC00D;
pub const TLS_ECDH_RSA_WITH_AES_128_CBC_SHA: SSLCipherSuite = 0xC00E;
pub const TLS_ECDH_RSA_WITH_AES_256_CBC_SHA: SSLCipherSuite = 0xC00F;
pub const TLS_ECDHE_RSA_WITH_NULL_SHA: SSLCipherSuite = 0xC010;
pub const TLS_ECDHE_RSA_WITH_RC4_128_SHA: SSLCipherSuite = 0xC011;
pub const TLS_ECDHE_RSA_WITH_3DES_EDE_CBC_SHA: SSLCipherSuite = 0xC012;
pub const TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA: SSLCipherSuite = 0xC013;
pub const TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA: SSLCipherSuite = 0xC014;
pub const TLS_ECDH_anon_WITH_NULL_SHA: SSLCipherSuite = 0xC015;
pub const TLS_ECDH_anon_WITH_RC4_128_SHA: SSLCipherSuite = 0xC016;
pub const TLS_ECDH_anon_WITH_3DES_EDE_CBC_SHA: SSLCipherSuite = 0xC017;
pub const TLS_ECDH_anon_WITH_AES_128_CBC_SHA: SSLCipherSuite = 0xC018;
pub const TLS_ECDH_anon_WITH_AES_256_CBC_SHA: SSLCipherSuite = 0xC019;

// TLS 1.2 addenda, RFC 5246

// Initial state.
pub const TLS_NULL_WITH_NULL_NULL: SSLCipherSuite = 0x0000;

// Server provided RSA certificate for key exchange.
pub const TLS_RSA_WITH_NULL_MD5: SSLCipherSuite = 0x0001;
pub const TLS_RSA_WITH_NULL_SHA: SSLCipherSuite = 0x0002;
pub const TLS_RSA_WITH_RC4_128_MD5: SSLCipherSuite = 0x0004;
pub const TLS_RSA_WITH_RC4_128_SHA: SSLCipherSuite = 0x0005;
pub const TLS_RSA_WITH_3DES_EDE_CBC_SHA: SSLCipherSuite = 0x000A;
// pub const TLS_RSA_WITH_AES_128_CBC_SHA: SSLCipherSuite = 0x002F;
// pub const TLS_RSA_WITH_AES_256_CBC_SHA: SSLCipherSuite = 0x0035;
pub const TLS_RSA_WITH_NULL_SHA256: SSLCipherSuite = 0x003B;
pub const TLS_RSA_WITH_AES_128_CBC_SHA256: SSLCipherSuite = 0x003C;
pub const TLS_RSA_WITH_AES_256_CBC_SHA256: SSLCipherSuite = 0x003D;

// Server-authenticated (and optionally client-authenticated) Diffie-Hellman.
pub const TLS_DH_DSS_WITH_3DES_EDE_CBC_SHA: SSLCipherSuite = 0x000D;
pub const TLS_DH_RSA_WITH_3DES_EDE_CBC_SHA: SSLCipherSuite = 0x0010;
pub const TLS_DHE_DSS_WITH_3DES_EDE_CBC_SHA: SSLCipherSuite = 0x0013;
pub const TLS_DHE_RSA_WITH_3DES_EDE_CBC_SHA: SSLCipherSuite = 0x0016;
// pub const TLS_DH_DSS_WITH_AES_128_CBC_SHA: SSLCipherSuite = 0x0030;
// pub const TLS_DH_RSA_WITH_AES_128_CBC_SHA: SSLCipherSuite = 0x0031;
// pub const TLS_DHE_DSS_WITH_AES_128_CBC_SHA: SSLCipherSuite = 0x0032;
// pub const TLS_DHE_RSA_WITH_AES_128_CBC_SHA: SSLCipherSuite = 0x0033;
// pub const TLS_DH_DSS_WITH_AES_256_CBC_SHA: SSLCipherSuite = 0x0036;
// pub const TLS_DH_RSA_WITH_AES_256_CBC_SHA: SSLCipherSuite = 0x0037;
// pub const TLS_DHE_DSS_WITH_AES_256_CBC_SHA: SSLCipherSuite = 0x0038;
// pub const TLS_DHE_RSA_WITH_AES_256_CBC_SHA: SSLCipherSuite = 0x0039;
pub const TLS_DH_DSS_WITH_AES_128_CBC_SHA256: SSLCipherSuite = 0x003E;
pub const TLS_DH_RSA_WITH_AES_128_CBC_SHA256: SSLCipherSuite = 0x003F;
pub const TLS_DHE_DSS_WITH_AES_128_CBC_SHA256: SSLCipherSuite = 0x0040;
pub const TLS_DHE_RSA_WITH_AES_128_CBC_SHA256: SSLCipherSuite = 0x0067;
pub const TLS_DH_DSS_WITH_AES_256_CBC_SHA256: SSLCipherSuite = 0x0068;
pub const TLS_DH_RSA_WITH_AES_256_CBC_SHA256: SSLCipherSuite = 0x0069;
pub const TLS_DHE_DSS_WITH_AES_256_CBC_SHA256: SSLCipherSuite = 0x006A;
pub const TLS_DHE_RSA_WITH_AES_256_CBC_SHA256: SSLCipherSuite = 0x006B;

// Completely anonymous Diffie-Hellman
pub const TLS_DH_anon_WITH_RC4_128_MD5: SSLCipherSuite = 0x0018;
pub const TLS_DH_anon_WITH_3DES_EDE_CBC_SHA: SSLCipherSuite = 0x001B;
// pub const TLS_DH_anon_WITH_AES_128_CBC_SHA: SSLCipherSuite = 0x0034;
// pub const TLS_DH_anon_WITH_AES_256_CBC_SHA: SSLCipherSuite = 0x003A;
pub const TLS_DH_anon_WITH_AES_128_CBC_SHA256: SSLCipherSuite = 0x006C;
pub const TLS_DH_anon_WITH_AES_256_CBC_SHA256: SSLCipherSuite = 0x006D;

// Addendum from RFC 4279, TLS PSK

pub const TLS_PSK_WITH_RC4_128_SHA: SSLCipherSuite = 0x008A;
pub const TLS_PSK_WITH_3DES_EDE_CBC_SHA: SSLCipherSuite = 0x008B;
pub const TLS_PSK_WITH_AES_128_CBC_SHA: SSLCipherSuite = 0x008C;
pub const TLS_PSK_WITH_AES_256_CBC_SHA: SSLCipherSuite = 0x008D;
pub const TLS_DHE_PSK_WITH_RC4_128_SHA: SSLCipherSuite = 0x008E;
pub const TLS_DHE_PSK_WITH_3DES_EDE_CBC_SHA: SSLCipherSuite = 0x008F;
pub const TLS_DHE_PSK_WITH_AES_128_CBC_SHA: SSLCipherSuite = 0x0090;
pub const TLS_DHE_PSK_WITH_AES_256_CBC_SHA: SSLCipherSuite = 0x0091;
pub const TLS_RSA_PSK_WITH_RC4_128_SHA: SSLCipherSuite = 0x0092;
pub const TLS_RSA_PSK_WITH_3DES_EDE_CBC_SHA: SSLCipherSuite = 0x0093;
pub const TLS_RSA_PSK_WITH_AES_128_CBC_SHA: SSLCipherSuite = 0x0094;
pub const TLS_RSA_PSK_WITH_AES_256_CBC_SHA: SSLCipherSuite = 0x0095;

// RFC 4785 - Pre-Shared Key (PSK) Ciphersuites with NULL Encryption

pub const TLS_PSK_WITH_NULL_SHA: SSLCipherSuite = 0x002C;
pub const TLS_DHE_PSK_WITH_NULL_SHA: SSLCipherSuite = 0x002D;
pub const TLS_RSA_PSK_WITH_NULL_SHA: SSLCipherSuite = 0x002E;

// Addenda from rfc 5288 AES Galois Counter Mode (GCM) Cipher Suites
// for TLS.
pub const TLS_RSA_WITH_AES_128_GCM_SHA256: SSLCipherSuite = 0x009C;
pub const TLS_RSA_WITH_AES_256_GCM_SHA384: SSLCipherSuite = 0x009D;
pub const TLS_DHE_RSA_WITH_AES_128_GCM_SHA256: SSLCipherSuite = 0x009E;
pub const TLS_DHE_RSA_WITH_AES_256_GCM_SHA384: SSLCipherSuite = 0x009F;
pub const TLS_DH_RSA_WITH_AES_128_GCM_SHA256: SSLCipherSuite = 0x00A0;
pub const TLS_DH_RSA_WITH_AES_256_GCM_SHA384: SSLCipherSuite = 0x00A1;
pub const TLS_DHE_DSS_WITH_AES_128_GCM_SHA256: SSLCipherSuite = 0x00A2;
pub const TLS_DHE_DSS_WITH_AES_256_GCM_SHA384: SSLCipherSuite = 0x00A3;
pub const TLS_DH_DSS_WITH_AES_128_GCM_SHA256: SSLCipherSuite = 0x00A4;
pub const TLS_DH_DSS_WITH_AES_256_GCM_SHA384: SSLCipherSuite = 0x00A5;
pub const TLS_DH_anon_WITH_AES_128_GCM_SHA256: SSLCipherSuite = 0x00A6;
pub const TLS_DH_anon_WITH_AES_256_GCM_SHA384: SSLCipherSuite = 0x00A7;

// RFC 5487 - PSK with SHA-256/384 and AES GCM
pub const TLS_PSK_WITH_AES_128_GCM_SHA256: SSLCipherSuite = 0x00A8;
pub const TLS_PSK_WITH_AES_256_GCM_SHA384: SSLCipherSuite = 0x00A9;
pub const TLS_DHE_PSK_WITH_AES_128_GCM_SHA256: SSLCipherSuite = 0x00AA;
pub const TLS_DHE_PSK_WITH_AES_256_GCM_SHA384: SSLCipherSuite = 0x00AB;
pub const TLS_RSA_PSK_WITH_AES_128_GCM_SHA256: SSLCipherSuite = 0x00AC;
pub const TLS_RSA_PSK_WITH_AES_256_GCM_SHA384: SSLCipherSuite = 0x00AD;

pub const TLS_PSK_WITH_AES_128_CBC_SHA256: SSLCipherSuite = 0x00AE;
pub const TLS_PSK_WITH_AES_256_CBC_SHA384: SSLCipherSuite = 0x00AF;
pub const TLS_PSK_WITH_NULL_SHA256: SSLCipherSuite = 0x00B0;
pub const TLS_PSK_WITH_NULL_SHA384: SSLCipherSuite = 0x00B1;

pub const TLS_DHE_PSK_WITH_AES_128_CBC_SHA256: SSLCipherSuite = 0x00B2;
pub const TLS_DHE_PSK_WITH_AES_256_CBC_SHA384: SSLCipherSuite = 0x00B3;
pub const TLS_DHE_PSK_WITH_NULL_SHA256: SSLCipherSuite = 0x00B4;
pub const TLS_DHE_PSK_WITH_NULL_SHA384: SSLCipherSuite = 0x00B5;

pub const TLS_RSA_PSK_WITH_AES_128_CBC_SHA256: SSLCipherSuite = 0x00B6;
pub const TLS_RSA_PSK_WITH_AES_256_CBC_SHA384: SSLCipherSuite = 0x00B7;
pub const TLS_RSA_PSK_WITH_NULL_SHA256: SSLCipherSuite = 0x00B8;
pub const TLS_RSA_PSK_WITH_NULL_SHA384: SSLCipherSuite = 0x00B9;

// Addenda from rfc 5289  Elliptic Curve Cipher Suites with
// HMAC SHA-256/384.
pub const TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA256: SSLCipherSuite = 0xC023;
pub const TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA384: SSLCipherSuite = 0xC024;
pub const TLS_ECDH_ECDSA_WITH_AES_128_CBC_SHA256: SSLCipherSuite = 0xC025;
pub const TLS_ECDH_ECDSA_WITH_AES_256_CBC_SHA384: SSLCipherSuite = 0xC026;
pub const TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA256: SSLCipherSuite = 0xC027;
pub const TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA384: SSLCipherSuite = 0xC028;
pub const TLS_ECDH_RSA_WITH_AES_128_CBC_SHA256: SSLCipherSuite = 0xC029;
pub const TLS_ECDH_RSA_WITH_AES_256_CBC_SHA384: SSLCipherSuite = 0xC02A;

// Addenda from rfc 5289  Elliptic Curve Cipher Suites with
// SHA-256/384 and AES Galois Counter Mode (GCM)
pub const TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256: SSLCipherSuite = 0xC02B;
pub const TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384: SSLCipherSuite = 0xC02C;
pub const TLS_ECDH_ECDSA_WITH_AES_128_GCM_SHA256: SSLCipherSuite = 0xC02D;
pub const TLS_ECDH_ECDSA_WITH_AES_256_GCM_SHA384: SSLCipherSuite = 0xC02E;
pub const TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256: SSLCipherSuite = 0xC02F;
pub const TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384: SSLCipherSuite = 0xC030;
pub const TLS_ECDH_RSA_WITH_AES_128_GCM_SHA256: SSLCipherSuite = 0xC031;
pub const TLS_ECDH_RSA_WITH_AES_256_GCM_SHA384: SSLCipherSuite = 0xC032;

// RFC 5746 - Secure Renegotiation
pub const TLS_EMPTY_RENEGOTIATION_INFO_SCSV: SSLCipherSuite = 0x00FF;
// Tags for SSL 2 cipher kinds which are not specified
// for SSL 3.
//
pub const SSL_RSA_WITH_RC2_CBC_MD5: SSLCipherSuite = 0xFF80;
pub const SSL_RSA_WITH_IDEA_CBC_MD5: SSLCipherSuite = 0xFF81;
pub const SSL_RSA_WITH_DES_CBC_MD5: SSLCipherSuite = 0xFF82;
pub const SSL_RSA_WITH_3DES_EDE_CBC_MD5: SSLCipherSuite = 0xFF83;
pub const SSL_NO_SUCH_CIPHERSUITE: SSLCipherSuite = 0xFFFF;
