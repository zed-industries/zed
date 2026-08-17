/* -*- Mode: C; tab-width: 8; indent-tabs-mode: nil; c-basic-offset: 4 -*- */
/*
 * This file contains prototypes for the public SSL functions.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef __sslt_h_
#define __sslt_h_

#include "certt.h"
#include "keyhi.h"
#include "prtypes.h"
#include "secitem.h"

typedef enum {
    ssl_hs_hello_request = 0,
    ssl_hs_client_hello = 1,
    ssl_hs_server_hello = 2,
    ssl_hs_hello_verify_request = 3,
    ssl_hs_new_session_ticket = 4,
    ssl_hs_end_of_early_data = 5,
    ssl_hs_hello_retry_request = 6,
    ssl_hs_encrypted_extensions = 8,
    ssl_hs_certificate = 11,
    ssl_hs_server_key_exchange = 12,
    ssl_hs_certificate_request = 13,
    ssl_hs_server_hello_done = 14,
    ssl_hs_certificate_verify = 15,
    ssl_hs_client_key_exchange = 16,
    ssl_hs_finished = 20,
    ssl_hs_certificate_status = 22,
    ssl_hs_key_update = 24,
    ssl_hs_next_proto = 67,
    ssl_hs_message_hash = 254, /* Not a real message. */
} SSLHandshakeType;

typedef enum {
    ssl_ct_change_cipher_spec = 20,
    ssl_ct_alert = 21,
    ssl_ct_handshake = 22,
    ssl_ct_application_data = 23,
    ssl_ct_ack = 26
} SSLContentType;

typedef enum {
    ssl_secret_read = 1,
    ssl_secret_write = 2,
} SSLSecretDirection;

typedef struct SSL3StatisticsStr {
    /* statistics from ssl3_SendClientHello (sch) */
    long sch_sid_cache_hits;
    long sch_sid_cache_misses;
    long sch_sid_cache_not_ok;

    /* statistics from ssl3_HandleServerHello (hsh) */
    long hsh_sid_cache_hits;
    long hsh_sid_cache_misses;
    long hsh_sid_cache_not_ok;

    /* statistics from ssl3_HandleClientHello (hch) */
    long hch_sid_cache_hits;
    long hch_sid_cache_misses;
    long hch_sid_cache_not_ok;

    /* statistics related to stateless resume */
    long sch_sid_stateless_resumes;
    long hsh_sid_stateless_resumes;
    long hch_sid_stateless_resumes;
    long hch_sid_ticket_parse_failures;
} SSL3Statistics;

/* Key Exchange algorithm values */
typedef enum {
    ssl_kea_null = 0,
    ssl_kea_rsa = 1,
    ssl_kea_dh = 2,
    ssl_kea_fortezza = 3, /* deprecated, now unused */
    ssl_kea_ecdh = 4,
    ssl_kea_ecdh_psk = 5,
    ssl_kea_dh_psk = 6,
    ssl_kea_tls13_any = 7,
    ssl_kea_size /* number of ssl_kea_ algorithms */
} SSLKEAType;

/* The following defines are for backwards compatibility.
** They will be removed in a forthcoming release to reduce namespace pollution.
** programs that use the kt_ symbols should convert to the ssl_kt_ symbols
** soon.
*/
#define kt_null ssl_kea_null
#define kt_rsa ssl_kea_rsa
#define kt_dh ssl_kea_dh
#define kt_fortezza ssl_kea_fortezza /* deprecated, now unused */
#define kt_ecdh ssl_kea_ecdh
#define kt_kea_size ssl_kea_size

/* Values of this enum match the SignatureAlgorithm enum from
 * https://tools.ietf.org/html/rfc5246#section-7.4.1.4.1 */
typedef enum {
    ssl_sign_null = 0, /* "anonymous" in TLS */
    ssl_sign_rsa = 1,
    ssl_sign_dsa = 2,
    ssl_sign_ecdsa = 3
} SSLSignType;

/* Values of this enum match the HashAlgorithm enum from
 * https://tools.ietf.org/html/rfc5246#section-7.4.1.4.1 */
typedef enum {
    /* ssl_hash_none is used internally to mean the pre-1.2 combination of MD5
     * and SHA1. The other values are only used in TLS 1.2. */
    ssl_hash_none = 0,
    ssl_hash_md5 = 1,
    ssl_hash_sha1 = 2,
    ssl_hash_sha224 = 3,
    ssl_hash_sha256 = 4,
    ssl_hash_sha384 = 5,
    ssl_hash_sha512 = 6
} SSLHashType;

/* Deprecated */
typedef struct SSLSignatureAndHashAlgStr {
    SSLHashType hashAlg;
    SSLSignType sigAlg;
} SSLSignatureAndHashAlg;

typedef enum {
    ssl_sig_none = 0,
    ssl_sig_rsa_pkcs1_sha1 = 0x0201,
    ssl_sig_rsa_pkcs1_sha256 = 0x0401,
    ssl_sig_rsa_pkcs1_sha384 = 0x0501,
    ssl_sig_rsa_pkcs1_sha512 = 0x0601,
    /* For ECDSA, the pairing of the hash with a specific curve is only enforced
     * in TLS 1.3; in TLS 1.2 any curve can be used with each of these. */
    ssl_sig_ecdsa_secp256r1_sha256 = 0x0403,
    ssl_sig_ecdsa_secp384r1_sha384 = 0x0503,
    ssl_sig_ecdsa_secp521r1_sha512 = 0x0603,
    ssl_sig_rsa_pss_rsae_sha256 = 0x0804,
    ssl_sig_rsa_pss_rsae_sha384 = 0x0805,
    ssl_sig_rsa_pss_rsae_sha512 = 0x0806,
    ssl_sig_ed25519 = 0x0807,
    ssl_sig_ed448 = 0x0808,
    ssl_sig_rsa_pss_pss_sha256 = 0x0809,
    ssl_sig_rsa_pss_pss_sha384 = 0x080a,
    ssl_sig_rsa_pss_pss_sha512 = 0x080b,

    ssl_sig_dsa_sha1 = 0x0202,
    ssl_sig_dsa_sha256 = 0x0402,
    ssl_sig_dsa_sha384 = 0x0502,
    ssl_sig_dsa_sha512 = 0x0602,
    ssl_sig_ecdsa_sha1 = 0x0203,

    /* The following value (which can't be used in the protocol), represents
     * the RSA signature using SHA-1 and MD5 that is used in TLS 1.0 and 1.1.
     * This is reported as a signature scheme when TLS 1.0 or 1.1 is used.
     * This should not be passed to SSL_SignatureSchemePrefSet(); this
     * signature scheme is always used and cannot be disabled. */
    ssl_sig_rsa_pkcs1_sha1md5 = 0x10101,
} SSLSignatureScheme;

/* Deprecated names maintained only for source compatibility. */
#define ssl_sig_rsa_pss_sha256 ssl_sig_rsa_pss_rsae_sha256
#define ssl_sig_rsa_pss_sha384 ssl_sig_rsa_pss_rsae_sha384
#define ssl_sig_rsa_pss_sha512 ssl_sig_rsa_pss_rsae_sha512

/*
** SSLAuthType describes the type of key that is used to authenticate a
** connection.  That is, the type of key in the end-entity certificate.
*/
typedef enum {
    ssl_auth_null = 0,
    ssl_auth_rsa_decrypt = 1, /* RSA key exchange. */
    ssl_auth_dsa = 2,
    ssl_auth_kea = 3, /* unused */
    ssl_auth_ecdsa = 4,
    ssl_auth_ecdh_rsa = 5,   /* ECDH cert with an RSA signature. */
    ssl_auth_ecdh_ecdsa = 6, /* ECDH cert with an ECDSA signature. */
    ssl_auth_rsa_sign = 7,   /* RSA signing with an rsaEncryption key. */
    ssl_auth_rsa_pss = 8,    /* RSA signing with a PSS key. */
    ssl_auth_psk = 9,
    ssl_auth_tls13_any = 10,
    ssl_auth_size /* number of authentication types */
} SSLAuthType;

typedef enum {
    ssl_psk_none = 0,
    ssl_psk_resume = 1,
    ssl_psk_external = 2,
} SSLPskType;

/* This is defined for backward compatibility reasons */
#define ssl_auth_rsa ssl_auth_rsa_decrypt

typedef enum {
    ssl_calg_null = 0,
    ssl_calg_rc4 = 1,
    ssl_calg_rc2 = 2,
    ssl_calg_des = 3,
    ssl_calg_3des = 4,
    ssl_calg_idea = 5,
    ssl_calg_fortezza = 6, /* deprecated, now unused */
    ssl_calg_aes = 7,
    ssl_calg_camellia = 8,
    ssl_calg_seed = 9,
    ssl_calg_aes_gcm = 10,
    ssl_calg_chacha20 = 11
} SSLCipherAlgorithm;

typedef enum {
    ssl_mac_null = 0,
    ssl_mac_md5 = 1,
    ssl_mac_sha = 2,
    ssl_hmac_md5 = 3, /* TLS HMAC version of mac_md5 */
    ssl_hmac_sha = 4, /* TLS HMAC version of mac_sha */
    ssl_hmac_sha256 = 5,
    ssl_mac_aead = 6,
    ssl_hmac_sha384 = 7
} SSLMACAlgorithm;

typedef enum {
    ssl_compression_null = 0,
    ssl_compression_deflate = 1 /* RFC 3749 */
} SSLCompressionMethod;

typedef enum {
    ssl_grp_ec_sect163k1 = 1,
    ssl_grp_ec_sect163r1 = 2,
    ssl_grp_ec_sect163r2 = 3,
    ssl_grp_ec_sect193r1 = 4,
    ssl_grp_ec_sect193r2 = 5,
    ssl_grp_ec_sect233k1 = 6,
    ssl_grp_ec_sect233r1 = 7,
    ssl_grp_ec_sect239k1 = 8,
    ssl_grp_ec_sect283k1 = 9,
    ssl_grp_ec_sect283r1 = 10,
    ssl_grp_ec_sect409k1 = 11,
    ssl_grp_ec_sect409r1 = 12,
    ssl_grp_ec_sect571k1 = 13,
    ssl_grp_ec_sect571r1 = 14,
    ssl_grp_ec_secp160k1 = 15,
    ssl_grp_ec_secp160r1 = 16,
    ssl_grp_ec_secp160r2 = 17,
    ssl_grp_ec_secp192k1 = 18,
    ssl_grp_ec_secp192r1 = 19,
    ssl_grp_ec_secp224k1 = 20,
    ssl_grp_ec_secp224r1 = 21,
    ssl_grp_ec_secp256k1 = 22,
    ssl_grp_ec_secp256r1 = 23,
    ssl_grp_ec_secp384r1 = 24,
    ssl_grp_ec_secp521r1 = 25,
    ssl_grp_ec_curve25519 = 29, /* RFC4492 */
    ssl_grp_ffdhe_2048 = 256,   /* RFC7919 */
    ssl_grp_ffdhe_3072 = 257,
    ssl_grp_ffdhe_4096 = 258,
    ssl_grp_ffdhe_6144 = 259,
    ssl_grp_ffdhe_8192 = 260,
    ssl_grp_none = 65537,        /* special value */
    ssl_grp_ffdhe_custom = 65538 /* special value */
} SSLNamedGroup;

typedef struct SSLExtraServerCertDataStr {
    /* When this struct is passed to SSL_ConfigServerCert, and authType is set
     * to a value other than ssl_auth_null, this limits the use of the key to
     * the type defined; otherwise, the certificate is configured for all
     * compatible types. */
    SSLAuthType authType;
    /* The remainder of the certificate chain. */
    const CERTCertificateList* certChain;
    /* A set of one or more stapled OCSP responses for the certificate.  This is
     * used to generate the OCSP stapling answer provided by the server. */
    const SECItemArray* stapledOCSPResponses;
    /* A serialized sign_certificate_timestamp extension, used to answer
     * requests from clients for this data. */
    const SECItem* signedCertTimestamps;

    /* Delegated credentials.
     *
     * A serialized delegated credential (DC) to use for authentication to peers
     * who indicate support for this extension (ietf-drafts-tls-subcerts). DCs
     * are used opportunistically if (1) the client indicates support, (2) TLS
     * 1.3 or higher is negotiated, and (3) the selected certificate is
     * configured with a DC.
     *
     * Note that it's the caller's responsibility to ensure that the DC is
     * well-formed.
     */
    const SECItem* delegCred;

    /* The secret key corresponding to the |delegCred|.
     *
     * Note that it's the caller's responsibility to ensure that this matches
     * the DC public key.
     */
    const SECKEYPrivateKey* delegCredPrivKey;
} SSLExtraServerCertData;

typedef struct SSLChannelInfoStr {
    /* On return, SSL_GetChannelInfo sets |length| to the smaller of
     * the |len| argument and the length of the struct used by NSS.
     * Callers must ensure the application uses a version of NSS that
     * isn't older than the version used at compile time. */
    PRUint32 length;
    PRUint16 protocolVersion;
    PRUint16 cipherSuite;

    /* The strength of the key used to authenticate the peer.  Before
     * interpreting this value, check authType, signatureScheme, and
     * peerDelegCred, to determine the type of the key and how it was used.
     *
     * Typically, this is the length of the key from the peer's end-entity
     * certificate.  If delegated credentials are used (i.e., peerDelegCred is
     * PR_TRUE), then this is the strength of the delegated credential key. */
    PRUint32 authKeyBits;

    /* key exchange algorithm info */
    PRUint32 keaKeyBits;

    /* session info */
    PRUint32 creationTime;    /* seconds since Jan 1, 1970 */
    PRUint32 lastAccessTime;  /* seconds since Jan 1, 1970 */
    PRUint32 expirationTime;  /* seconds since Jan 1, 1970 */
    PRUint32 sessionIDLength; /* up to 32 */
    PRUint8 sessionID[32];

    /* The following fields are added in NSS 3.12.5. */

    /* compression method info */
    const char* compressionMethodName;
    SSLCompressionMethod compressionMethod;

    /* The following fields are added in NSS 3.21.
     * This field only has meaning in TLS < 1.3 and will be set to
     *  PR_FALSE in TLS 1.3.
     */
    PRBool extendedMasterSecretUsed;

    /* The following fields were added in NSS 3.25.
     * This field only has meaning in TLS >= 1.3, and indicates on the
     * client side that the server accepted early (0-RTT) data.
     */
    PRBool earlyDataAccepted;

    /* The following fields were added in NSS 3.28. */
    /* These fields have the same meaning as in SSLCipherSuiteInfo. */
    SSLKEAType keaType;
    SSLNamedGroup keaGroup;
    SSLCipherAlgorithm symCipher;
    SSLMACAlgorithm macAlgorithm;
    SSLAuthType authType;
    SSLSignatureScheme signatureScheme;

    /* The following fields were added in NSS 3.34. */
    /* When the session was resumed this holds the key exchange group of the
     * original handshake. */
    SSLNamedGroup originalKeaGroup;
    /* This field is PR_TRUE when the session is resumed and PR_FALSE
     * otherwise. */
    PRBool resumed;

    /* Indicates whether the peer used a delegated credential (DC) for
     * authentication.
     */
    PRBool peerDelegCred;

    /* The following fields were added in NSS 3.54. */
    /* Indicates what type of PSK, if any, was used in a handshake. */
    SSLPskType pskType;

    /* The following fields were added in NSS 3.60 */
    /* This field is PR_TRUE when the connection is established
     * with TLS 1.3 Encrypted Client Hello. */
    PRBool echAccepted;

    /* When adding new fields to this structure, please document the
     * NSS version in which they were added. */
} SSLChannelInfo;

/* Preliminary channel info */
#define ssl_preinfo_version (1U << 0)
#define ssl_preinfo_cipher_suite (1U << 1)
#define ssl_preinfo_0rtt_cipher_suite (1U << 2)
/* ssl_preinfo_peer_auth covers peerDelegCred, authKeyBits,
 * and scheme. Not included in ssl_preinfo_all as it is client-only. */
#define ssl_preinfo_peer_auth (1U << 3)
#define ssl_preinfo_ech (1U << 4)
/* ssl_preinfo_all doesn't contain ssl_preinfo_0rtt_cipher_suite because that
 * field is only set if 0-RTT is sent (client) or accepted (server). */
#define ssl_preinfo_all (ssl_preinfo_version | ssl_preinfo_cipher_suite | ssl_preinfo_ech)

typedef struct SSLPreliminaryChannelInfoStr {
    /* On return, SSL_GetPreliminaryChannelInfo sets |length| to the smaller of
     * the |len| argument and the length of the struct used by NSS.
     * Callers must ensure the application uses a version of NSS that
     * isn't older than the version used at compile time. */
    PRUint32 length;
    /* A bitfield over SSLPreliminaryValueSet that describes which
     * preliminary values are set (see ssl_preinfo_*). */
    PRUint32 valuesSet;
    /* Protocol version: test (valuesSet & ssl_preinfo_version) */
    PRUint16 protocolVersion;
    /* Cipher suite: test (valuesSet & ssl_preinfo_cipher_suite) */
    PRUint16 cipherSuite;

    /* The following fields were added in NSS 3.29. */
    /* |canSendEarlyData| is true when a 0-RTT is enabled. This can only be
     * true after sending the ClientHello and before the handshake completes.
     */
    PRBool canSendEarlyData;

    /* The following fields were added in NSS 3.31. */
    /* The number of early data octets that a client is permitted to send on
     * this connection.  The value will be zero if the connection was not
     * resumed or early data is not permitted.  For a client, this value only
     * has meaning if |canSendEarlyData| is true.  For a server, this indicates
     * the value that was advertised in the session ticket that was used to
     * resume this session. */
    PRUint32 maxEarlyDataSize;

    /* The following fields were added in NSS 3.43. */
    /* This reports the cipher suite used for 0-RTT if it sent or accepted.  For
     * a client, this is set earlier than |cipherSuite|, and will match that
     * value if 0-RTT is accepted by the server.  The server only sets this
     * after accepting 0-RTT, so this will contain the same value. */
    PRUint16 zeroRttCipherSuite;

    /* The following fields were added in NSS 3.48. */
    /* These fields contain information about the key that will be used in
     * the CertificateVerify message. If Delegated Credentials are being used,
     * this is the DC-contained SPKI, else the EE-cert SPKI. These fields are
     * valid only after the Certificate message is handled. This can be determined
     * by checking the valuesSet field against |ssl_preinfo_peer_auth|. */
    PRBool peerDelegCred;
    PRUint32 authKeyBits;
    SSLSignatureScheme signatureScheme;

    /* The following fields were added in NSS 3.60. */
    PRBool echAccepted;
    /* If the application configured ECH but |!echAccepted|, authCertificate
     * should use the following hostname extracted from the ECHConfig. */
    const char* echPublicName;

    /* When adding new fields to this structure, please document the
     * NSS version in which they were added. */
} SSLPreliminaryChannelInfo;

typedef struct SSLCipherSuiteInfoStr {
    /* On return, SSL_GetCipherSuitelInfo sets |length| to the smaller of
     * the |len| argument and the length of the struct used by NSS.
     * Callers must ensure the application uses a version of NSS that
     * isn't older than the version used at compile time. */
    PRUint16 length;
    PRUint16 cipherSuite;

    /* Cipher Suite Name */
    const char* cipherSuiteName;

    /* server authentication info */
    const char* authAlgorithmName;
    SSLAuthType authAlgorithm; /* deprecated, use |authType| */

    /* key exchange algorithm info */
    const char* keaTypeName;
    SSLKEAType keaType;

    /* symmetric encryption info */
    const char* symCipherName;
    SSLCipherAlgorithm symCipher;
    PRUint16 symKeyBits;
    PRUint16 symKeySpace;
    PRUint16 effectiveKeyBits;

    /* MAC info */
    /* AEAD ciphers don't have a MAC. For an AEAD cipher, macAlgorithmName
     * is "AEAD", macAlgorithm is ssl_mac_aead, and macBits is the length in
     * bits of the authentication tag. */
    const char* macAlgorithmName;
    SSLMACAlgorithm macAlgorithm;
    PRUint16 macBits;

    PRUintn isFIPS : 1;
    PRUintn isExportable : 1; /* deprecated, don't use */
    PRUintn nonStandard : 1;
    PRUintn reservedBits : 29;

    /* The following fields were added in NSS 3.24. */
    /* This reports the correct authentication type for the cipher suite, use
     * this instead of |authAlgorithm|. */
    SSLAuthType authType;

    /* The following fields were added in NSS 3.43. */
    /* This reports the hash function used in the TLS KDF, or HKDF for TLS 1.3.
     * For suites defined for versions of TLS earlier than TLS 1.2, this reports
     * ssl_hash_none. */
    SSLHashType kdfHash;

    /* When adding new fields to this structure, please document the
     * NSS version in which they were added. */
} SSLCipherSuiteInfo;

typedef enum {
    ssl_variant_stream = 0,
    ssl_variant_datagram = 1
} SSLProtocolVariant;

typedef struct SSLVersionRangeStr {
    PRUint16 min;
    PRUint16 max;
} SSLVersionRange;

typedef enum {
    SSL_sni_host_name = 0,
    SSL_sni_type_total
} SSLSniNameType;

/* Supported extensions. */
/* Update SSL_MAX_EXTENSIONS whenever a new extension type is added. */
typedef enum {
    ssl_server_name_xtn = 0,
    ssl_cert_status_xtn = 5,
    ssl_supported_groups_xtn = 10,
    ssl_ec_point_formats_xtn = 11,
    ssl_signature_algorithms_xtn = 13,
    ssl_use_srtp_xtn = 14,
    ssl_app_layer_protocol_xtn = 16,
    /* signed_certificate_timestamp extension, RFC 6962 */
    ssl_signed_cert_timestamp_xtn = 18,
    ssl_padding_xtn = 21,
    ssl_extended_master_secret_xtn = 23,
    ssl_record_size_limit_xtn = 28,
    ssl_delegated_credentials_xtn = 34,
    ssl_session_ticket_xtn = 35,
    /* 40 was used in draft versions of TLS 1.3; it is now reserved. */
    ssl_tls13_pre_shared_key_xtn = 41,
    ssl_tls13_early_data_xtn = 42,
    ssl_tls13_supported_versions_xtn = 43,
    ssl_tls13_cookie_xtn = 44,
    ssl_tls13_psk_key_exchange_modes_xtn = 45,
    ssl_tls13_ticket_early_data_info_xtn = 46, /* Deprecated. */
    ssl_tls13_certificate_authorities_xtn = 47,
    ssl_tls13_post_handshake_auth_xtn = 49,
    ssl_signature_algorithms_cert_xtn = 50,
    ssl_tls13_key_share_xtn = 51,
    ssl_next_proto_nego_xtn = 13172, /* Deprecated. */
    ssl_renegotiation_info_xtn = 0xff01,
    ssl_tls13_short_header_xtn = 0xff03, /* Deprecated. */
    ssl_tls13_outer_extensions_xtn = 0xfd00,
    ssl_tls13_encrypted_client_hello_xtn = 0xfe08,
    ssl_tls13_encrypted_sni_xtn = 0xffce, /* Deprecated. */
} SSLExtensionType;

/* This is the old name for the supported_groups extensions. */
#define ssl_elliptic_curves_xtn ssl_supported_groups_xtn

/* SSL_MAX_EXTENSIONS includes the maximum number of extensions that are
 * supported for any single message type.  That is, a ClientHello; ServerHello
 * and TLS 1.3 NewSessionTicket and HelloRetryRequest extensions have fewer. */
#define SSL_MAX_EXTENSIONS 21

/* Deprecated */
typedef enum {
    ssl_dhe_group_none = 0,
    ssl_ff_dhe_2048_group = 1,
    ssl_ff_dhe_3072_group = 2,
    ssl_ff_dhe_4096_group = 3,
    ssl_ff_dhe_6144_group = 4,
    ssl_ff_dhe_8192_group = 5,
    ssl_dhe_group_max
} SSLDHEGroupType;

#endif /* __sslt_h_ */
