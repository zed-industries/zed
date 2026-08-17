/*
 * Various and sundry protocol constants. DON'T CHANGE THESE. These values
 * are mostly defined by the SSL3 or TLS protocol specifications.
 * Cipher kinds and ciphersuites are part of the public API.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef __sslproto_h_
#define __sslproto_h_

/* clang-format off */

/* All versions less than 3_0 are treated as SSL version 2 */
#define SSL_LIBRARY_VERSION_2                   0x0002
#define SSL_LIBRARY_VERSION_3_0                 0x0300
#define SSL_LIBRARY_VERSION_TLS_1_0             0x0301
#define SSL_LIBRARY_VERSION_TLS_1_1             0x0302
#define SSL_LIBRARY_VERSION_TLS_1_2             0x0303
#define SSL_LIBRARY_VERSION_TLS_1_3             0x0304

/* Note: this is the internal format, not the wire format */
#define SSL_LIBRARY_VERSION_DTLS_1_0            SSL_LIBRARY_VERSION_TLS_1_1
#define SSL_LIBRARY_VERSION_DTLS_1_2            SSL_LIBRARY_VERSION_TLS_1_2
#define SSL_LIBRARY_VERSION_DTLS_1_3            SSL_LIBRARY_VERSION_TLS_1_3

/* deprecated old name */
#define SSL_LIBRARY_VERSION_3_1_TLS SSL_LIBRARY_VERSION_TLS_1_0

/* The DTLS versions used in the spec */
#define SSL_LIBRARY_VERSION_DTLS_1_0_WIRE       ((~0x0100) & 0xffff)
#define SSL_LIBRARY_VERSION_DTLS_1_2_WIRE       ((~0x0102) & 0xffff)
#define SSL_LIBRARY_VERSION_DTLS_1_3_WIRE       SSL_LIBRARY_VERSION_DTLS_1_3

/* Certificate types */
#define SSL_CT_X509_CERTIFICATE                 0x01
#if 0 /* XXX Not implemented yet */
#define SSL_PKCS6_CERTIFICATE                   0x02
#endif
#define SSL_AT_MD5_WITH_RSA_ENCRYPTION          0x01

/* Error codes */
#define SSL_PE_NO_CYPHERS                       0x0001
#define SSL_PE_NO_CERTIFICATE                   0x0002
#define SSL_PE_BAD_CERTIFICATE                  0x0004
#define SSL_PE_UNSUPPORTED_CERTIFICATE_TYPE     0x0006

/* Deprecated SSL 3.0 & libssl names replaced by IANA-registered TLS names. */
#ifndef SSL_DISABLE_DEPRECATED_CIPHER_SUITE_NAMES
#define SSL_NULL_WITH_NULL_NULL                TLS_NULL_WITH_NULL_NULL
#define SSL_RSA_WITH_NULL_MD5                  TLS_RSA_WITH_NULL_MD5
#define SSL_RSA_WITH_NULL_SHA                  TLS_RSA_WITH_NULL_SHA
#define SSL_RSA_WITH_RC4_128_MD5               TLS_RSA_WITH_RC4_128_MD5
#define SSL_RSA_WITH_RC4_128_SHA               TLS_RSA_WITH_RC4_128_SHA
#define SSL_RSA_WITH_IDEA_CBC_SHA              TLS_RSA_WITH_IDEA_CBC_SHA
#define SSL_RSA_WITH_DES_CBC_SHA               TLS_RSA_WITH_DES_CBC_SHA
#define SSL_RSA_WITH_3DES_EDE_CBC_SHA          TLS_RSA_WITH_3DES_EDE_CBC_SHA
#define SSL_DH_DSS_WITH_DES_CBC_SHA            TLS_DH_DSS_WITH_DES_CBC_SHA
#define SSL_DH_DSS_WITH_3DES_EDE_CBC_SHA       TLS_DH_DSS_WITH_3DES_EDE_CBC_SHA
#define SSL_DH_RSA_WITH_DES_CBC_SHA            TLS_DH_RSA_WITH_DES_CBC_SHA
#define SSL_DH_RSA_WITH_3DES_EDE_CBC_SHA       TLS_DH_RSA_WITH_3DES_EDE_CBC_SHA
#define SSL_DHE_DSS_WITH_DES_CBC_SHA           TLS_DHE_DSS_WITH_DES_CBC_SHA
#define SSL_DHE_DSS_WITH_3DES_EDE_CBC_SHA      TLS_DHE_DSS_WITH_3DES_EDE_CBC_SHA
#define SSL_DHE_RSA_WITH_DES_CBC_SHA           TLS_DHE_RSA_WITH_DES_CBC_SHA
#define SSL_DHE_RSA_WITH_3DES_EDE_CBC_SHA      TLS_DHE_RSA_WITH_3DES_EDE_CBC_SHA
#define SSL_DH_ANON_WITH_RC4_128_MD5           TLS_DH_anon_WITH_RC4_128_MD5
#define SSL_DH_ANON_WITH_DES_CBC_SHA           TLS_DH_anon_WITH_DES_CBC_SHA
#define SSL_DH_ANON_WITH_3DES_EDE_CBC_SHA      TLS_DH_anon_WITH_3DES_EDE_CBC_SHA
#define TLS_DH_ANON_WITH_AES_128_CBC_SHA       TLS_DH_anon_WITH_AES_128_CBC_SHA
#define TLS_DH_ANON_WITH_AES_256_CBC_SHA       TLS_DH_anon_WITH_AES_256_CBC_SHA
#define TLS_DH_ANON_WITH_CAMELLIA_128_CBC_SHA  TLS_DH_anon_WITH_CAMELLIA_128_CBC_SHA
#define TLS_DH_ANON_WITH_CAMELLIA_256_CBC_SHA  TLS_DH_anon_WITH_CAMELLIA_256_CBC_SHA
#endif

#define TLS_NULL_WITH_NULL_NULL                 0x0000

#define TLS_RSA_WITH_NULL_MD5                   0x0001
#define TLS_RSA_WITH_NULL_SHA                   0x0002
#define TLS_RSA_WITH_RC4_128_MD5                0x0004
#define TLS_RSA_WITH_RC4_128_SHA                0x0005
#define TLS_RSA_WITH_IDEA_CBC_SHA               0x0007
#define TLS_RSA_WITH_DES_CBC_SHA                0x0009
#define TLS_RSA_WITH_3DES_EDE_CBC_SHA           0x000a

#define TLS_DH_DSS_WITH_DES_CBC_SHA             0x000c
#define TLS_DH_DSS_WITH_3DES_EDE_CBC_SHA        0x000d
#define TLS_DH_RSA_WITH_DES_CBC_SHA             0x000f
#define TLS_DH_RSA_WITH_3DES_EDE_CBC_SHA        0x0010

#define TLS_DHE_DSS_WITH_DES_CBC_SHA            0x0012
#define TLS_DHE_DSS_WITH_3DES_EDE_CBC_SHA       0x0013
#define TLS_DHE_RSA_WITH_DES_CBC_SHA            0x0015
#define TLS_DHE_RSA_WITH_3DES_EDE_CBC_SHA       0x0016

#define TLS_DH_anon_WITH_RC4_128_MD5            0x0018
#define TLS_DH_anon_WITH_DES_CBC_SHA            0x001a
#define TLS_DH_anon_WITH_3DES_EDE_CBC_SHA       0x001b

#define TLS_RSA_WITH_AES_128_CBC_SHA            0x002F
#define TLS_DH_DSS_WITH_AES_128_CBC_SHA         0x0030
#define TLS_DH_RSA_WITH_AES_128_CBC_SHA         0x0031
#define TLS_DHE_DSS_WITH_AES_128_CBC_SHA        0x0032
#define TLS_DHE_RSA_WITH_AES_128_CBC_SHA        0x0033
#define TLS_DH_anon_WITH_AES_128_CBC_SHA        0x0034

#define TLS_RSA_WITH_AES_256_CBC_SHA            0x0035
#define TLS_DH_DSS_WITH_AES_256_CBC_SHA         0x0036
#define TLS_DH_RSA_WITH_AES_256_CBC_SHA         0x0037
#define TLS_DHE_DSS_WITH_AES_256_CBC_SHA        0x0038
#define TLS_DHE_RSA_WITH_AES_256_CBC_SHA        0x0039
#define TLS_DH_anon_WITH_AES_256_CBC_SHA        0x003A
#define TLS_RSA_WITH_NULL_SHA256                0x003B
#define TLS_RSA_WITH_AES_128_CBC_SHA256         0x003C
#define TLS_RSA_WITH_AES_256_CBC_SHA256         0x003D

#define TLS_DHE_DSS_WITH_AES_128_CBC_SHA256     0x0040
#define TLS_RSA_WITH_CAMELLIA_128_CBC_SHA       0x0041
#define TLS_DH_DSS_WITH_CAMELLIA_128_CBC_SHA    0x0042
#define TLS_DH_RSA_WITH_CAMELLIA_128_CBC_SHA    0x0043
#define TLS_DHE_DSS_WITH_CAMELLIA_128_CBC_SHA   0x0044
#define TLS_DHE_RSA_WITH_CAMELLIA_128_CBC_SHA   0x0045
#define TLS_DH_anon_WITH_CAMELLIA_128_CBC_SHA   0x0046

#define TLS_DHE_DSS_WITH_RC4_128_SHA            0x0066
#define TLS_DHE_RSA_WITH_AES_128_CBC_SHA256     0x0067
#define TLS_DHE_DSS_WITH_AES_256_CBC_SHA256     0x006A
#define TLS_DHE_RSA_WITH_AES_256_CBC_SHA256     0x006B

#define TLS_RSA_WITH_CAMELLIA_256_CBC_SHA       0x0084
#define TLS_DH_DSS_WITH_CAMELLIA_256_CBC_SHA    0x0085
#define TLS_DH_RSA_WITH_CAMELLIA_256_CBC_SHA    0x0086
#define TLS_DHE_DSS_WITH_CAMELLIA_256_CBC_SHA   0x0087
#define TLS_DHE_RSA_WITH_CAMELLIA_256_CBC_SHA   0x0088
#define TLS_DH_anon_WITH_CAMELLIA_256_CBC_SHA   0x0089

#define TLS_RSA_WITH_SEED_CBC_SHA               0x0096

#define TLS_RSA_WITH_AES_128_GCM_SHA256         0x009C
#define TLS_RSA_WITH_AES_256_GCM_SHA384         0x009D
#define TLS_DHE_RSA_WITH_AES_128_GCM_SHA256     0x009E
#define TLS_DHE_RSA_WITH_AES_256_GCM_SHA384     0x009F
#define TLS_DHE_DSS_WITH_AES_128_GCM_SHA256     0x00A2
#define TLS_DHE_DSS_WITH_AES_256_GCM_SHA384     0x00A3

/* TLS "Signaling Cipher Suite Value" (SCSV). May be requested by client.
 * Must NEVER be chosen by server.  SSL 3.0 server acknowledges by sending
 * back an empty Renegotiation Info (RI) server hello extension.
 */
#define TLS_EMPTY_RENEGOTIATION_INFO_SCSV       0x00FF

/* TLS_FALLBACK_SCSV is a signaling cipher suite value that indicates that a
 * handshake is the result of TLS version fallback.
 */
#define TLS_FALLBACK_SCSV                       0x5600

/* Cipher Suite Values starting with 0xC000 are defined in informational
 * RFCs.
 */
#define TLS_ECDH_ECDSA_WITH_NULL_SHA            0xC001
#define TLS_ECDH_ECDSA_WITH_RC4_128_SHA         0xC002
#define TLS_ECDH_ECDSA_WITH_3DES_EDE_CBC_SHA    0xC003
#define TLS_ECDH_ECDSA_WITH_AES_128_CBC_SHA     0xC004
#define TLS_ECDH_ECDSA_WITH_AES_256_CBC_SHA     0xC005

#define TLS_ECDHE_ECDSA_WITH_NULL_SHA           0xC006
#define TLS_ECDHE_ECDSA_WITH_RC4_128_SHA        0xC007
#define TLS_ECDHE_ECDSA_WITH_3DES_EDE_CBC_SHA   0xC008
#define TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA    0xC009
#define TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA    0xC00A

#define TLS_ECDH_RSA_WITH_NULL_SHA              0xC00B
#define TLS_ECDH_RSA_WITH_RC4_128_SHA           0xC00C
#define TLS_ECDH_RSA_WITH_3DES_EDE_CBC_SHA      0xC00D
#define TLS_ECDH_RSA_WITH_AES_128_CBC_SHA       0xC00E
#define TLS_ECDH_RSA_WITH_AES_256_CBC_SHA       0xC00F

#define TLS_ECDHE_RSA_WITH_NULL_SHA             0xC010
#define TLS_ECDHE_RSA_WITH_RC4_128_SHA          0xC011
#define TLS_ECDHE_RSA_WITH_3DES_EDE_CBC_SHA     0xC012
#define TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA      0xC013
#define TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA      0xC014

#define TLS_ECDH_anon_WITH_NULL_SHA             0xC015
#define TLS_ECDH_anon_WITH_RC4_128_SHA          0xC016
#define TLS_ECDH_anon_WITH_3DES_EDE_CBC_SHA     0xC017
#define TLS_ECDH_anon_WITH_AES_128_CBC_SHA      0xC018
#define TLS_ECDH_anon_WITH_AES_256_CBC_SHA      0xC019

#define TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA256 0xC023
#define TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA384 0xC024
#define TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA256   0xC027
#define TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA384   0xC028

#define TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256 0xC02B
#define TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384 0xC02C
#define TLS_ECDH_ECDSA_WITH_AES_128_GCM_SHA256  0xC02D
#define TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256   0xC02F
#define TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384   0xC030
#define TLS_ECDH_RSA_WITH_AES_128_GCM_SHA256    0xC031

/* draft-ietf-tls-chacha20-poly1305-04 */
#define TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256   0xCCA8
#define TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256 0xCCA9
#define TLS_DHE_RSA_WITH_CHACHA20_POLY1305_SHA256     0xCCAA

/* Special TLS 1.3 cipher suites that really just specify AEAD */
#define TLS_AES_128_GCM_SHA256                0x1301
#define TLS_AES_256_GCM_SHA384                0x1302
#define TLS_CHACHA20_POLY1305_SHA256          0x1303

/* PSK cipher suites. NSS doesn't actually support these, but we
 * exposed them when TLS 1.3 used them so we need to keep them
 * in the API. */
#define TLS_ECDHE_PSK_WITH_CHACHA20_POLY1305_SHA256   0xCCAC
#define TLS_DHE_PSK_WITH_CHACHA20_POLY1305_SHA256     0xCCAD
#define TLS_ECDHE_PSK_WITH_AES_128_GCM_SHA256   0xD001
#define TLS_ECDHE_PSK_WITH_AES_256_GCM_SHA384   0xD002
#define TLS_DHE_PSK_WITH_AES_128_GCM_SHA256     0x00AA /* RFC 5487 */
#define TLS_DHE_PSK_WITH_AES_256_GCM_SHA384     0x00AB /* RFC 5487 */

/* DTLS-SRTP cipher suites from RFC 5764 */
/* If you modify this, also modify MAX_DTLS_SRTP_CIPHER_SUITES in sslimpl.h */
#define SRTP_AES128_CM_HMAC_SHA1_80             0x0001
#define SRTP_AES128_CM_HMAC_SHA1_32             0x0002
#define SRTP_NULL_HMAC_SHA1_80                  0x0005
#define SRTP_NULL_HMAC_SHA1_32                  0x0006

/* DO NOT USE. (deprecated, will be removed) */
#define SSL_HL_ERROR_HBYTES                     3
#define SSL_HL_CLIENT_HELLO_HBYTES              9
#define SSL_HL_CLIENT_MASTER_KEY_HBYTES         10
#define SSL_HL_CLIENT_FINISHED_HBYTES           1
#define SSL_HL_SERVER_HELLO_HBYTES              11
#define SSL_HL_SERVER_VERIFY_HBYTES             1
#define SSL_HL_SERVER_FINISHED_HBYTES           1
#define SSL_HL_REQUEST_CERTIFICATE_HBYTES       2
#define SSL_HL_CLIENT_CERTIFICATE_HBYTES        6
#define SSL_MT_ERROR                            0
#define SSL_MT_CLIENT_HELLO                     1
#define SSL_MT_CLIENT_MASTER_KEY                2
#define SSL_MT_CLIENT_FINISHED                  3
#define SSL_MT_SERVER_HELLO                     4
#define SSL_MT_SERVER_VERIFY                    5
#define SSL_MT_SERVER_FINISHED                  6
#define SSL_MT_REQUEST_CERTIFICATE              7
#define SSL_MT_CLIENT_CERTIFICATE               8
#define SSL_CK_RC4_128_WITH_MD5                 0x01
#define SSL_CK_RC4_128_EXPORT40_WITH_MD5        0x02
#define SSL_CK_RC2_128_CBC_WITH_MD5             0x03
#define SSL_CK_RC2_128_CBC_EXPORT40_WITH_MD5    0x04
#define SSL_CK_IDEA_128_CBC_WITH_MD5            0x05
#define SSL_CK_DES_64_CBC_WITH_MD5              0x06
#define SSL_CK_DES_192_EDE3_CBC_WITH_MD5        0x07
#define SSL_EN_RC4_128_WITH_MD5                 0xFF01
#define SSL_EN_RC4_128_EXPORT40_WITH_MD5        0xFF02
#define SSL_EN_RC2_128_CBC_WITH_MD5             0xFF03
#define SSL_EN_RC2_128_CBC_EXPORT40_WITH_MD5    0xFF04
#define SSL_EN_IDEA_128_CBC_WITH_MD5            0xFF05
#define SSL_EN_DES_64_CBC_WITH_MD5              0xFF06
#define SSL_EN_DES_192_EDE3_CBC_WITH_MD5        0xFF07
#define TLS_RSA_EXPORT_WITH_RC4_40_MD5          0x0003
#define TLS_RSA_EXPORT_WITH_RC2_CBC_40_MD5      0x0006
#define TLS_RSA_EXPORT_WITH_DES40_CBC_SHA       0x0008
#define TLS_RSA_EXPORT1024_WITH_DES_CBC_SHA     0x0062
#define TLS_RSA_EXPORT1024_WITH_RC4_56_SHA      0x0064
#define TLS_DHE_RSA_EXPORT_WITH_DES40_CBC_SHA   0x0014
#define TLS_DH_RSA_EXPORT_WITH_DES40_CBC_SHA    0x000e
#define TLS_DHE_DSS_EXPORT1024_WITH_DES_CBC_SHA 0x0063
#define TLS_DHE_DSS_EXPORT1024_WITH_RC4_56_SHA  0x0065
#define TLS_DH_DSS_EXPORT_WITH_DES40_CBC_SHA    0x000b
#define TLS_DHE_DSS_EXPORT_WITH_DES40_CBC_SHA   0x0011
#define TLS_DH_anon_EXPORT_WITH_RC4_40_MD5      0x0017
#define TLS_DH_anon_EXPORT_WITH_DES40_CBC_SHA   0x0019
#define SSL_FORTEZZA_DMS_WITH_NULL_SHA          0x001c
#define SSL_FORTEZZA_DMS_WITH_FORTEZZA_CBC_SHA  0x001d
#define SSL_FORTEZZA_DMS_WITH_RC4_128_SHA       0x001e
#define SSL_RSA_OLDFIPS_WITH_3DES_EDE_CBC_SHA   0xffe0
#define SSL_RSA_OLDFIPS_WITH_DES_CBC_SHA        0xffe1
#define SSL_RSA_FIPS_WITH_3DES_EDE_CBC_SHA      0xfeff
#define SSL_RSA_FIPS_WITH_DES_CBC_SHA           0xfefe
#define SSL_RSA_EXPORT_WITH_RC4_40_MD5         TLS_RSA_EXPORT_WITH_RC4_40_MD5
#define SSL_RSA_EXPORT_WITH_RC2_CBC_40_MD5     TLS_RSA_EXPORT_WITH_RC2_CBC_40_MD5
#define SSL_RSA_EXPORT_WITH_DES40_CBC_SHA      TLS_RSA_EXPORT_WITH_DES40_CBC_SHA
#define SSL_DH_RSA_EXPORT_WITH_DES40_CBC_SHA   TLS_DH_RSA_EXPORT_WITH_DES40_CBC_SHA
#define SSL_DHE_RSA_EXPORT_WITH_DES40_CBC_SHA  TLS_DHE_RSA_EXPORT_WITH_DES40_CBC_SHA
#define SSL_DH_DSS_EXPORT_WITH_DES40_CBC_SHA   TLS_DH_DSS_EXPORT_WITH_DES40_CBC_SHA
#define SSL_DHE_DSS_EXPORT_WITH_DES40_CBC_SHA  TLS_DHE_DSS_EXPORT_WITH_DES40_CBC_SHA
#define SSL_DH_ANON_EXPORT_WITH_DES40_CBC_SHA  TLS_DH_anon_EXPORT_WITH_DES40_CBC_SHA
#define SSL_DH_ANON_EXPORT_WITH_RC4_40_MD5     TLS_DH_anon_EXPORT_WITH_RC4_40_MD5

/* clang-format on */

#endif /* __sslproto_h_ */
