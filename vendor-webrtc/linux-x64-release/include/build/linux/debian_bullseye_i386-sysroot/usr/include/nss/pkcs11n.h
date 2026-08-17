/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef _PKCS11N_H_
#define _PKCS11N_H_

/*
 * pkcs11n.h
 *
 * This file contains the NSS-specific type definitions for Cryptoki
 * (PKCS#11).
 */

/*
 * NSSCK_VENDOR_NSS
 *
 * Cryptoki reserves the high half of all the number spaces for
 * vendor-defined use.  I'd like to keep all of our NSS-
 * specific values together, but not in the oh-so-obvious
 * 0x80000001, 0x80000002, etc. area.  So I've picked an offset,
 * and constructed values for the beginnings of our spaces.
 *
 * Note that some "historical" Netscape values don't fall within
 * this range.
 */
#define NSSCK_VENDOR_NSS 0x4E534350 /* NSCP */

/*
 * NSS-defined object classes
 *
 */
#define CKO_NSS (CKO_VENDOR_DEFINED | NSSCK_VENDOR_NSS)

#define CKO_NSS_CRL (CKO_NSS + 1)
#define CKO_NSS_SMIME (CKO_NSS + 2)
#define CKO_NSS_TRUST (CKO_NSS + 3)
#define CKO_NSS_BUILTIN_ROOT_LIST (CKO_NSS + 4)
#define CKO_NSS_NEWSLOT (CKO_NSS + 5)
#define CKO_NSS_DELSLOT (CKO_NSS + 6)

/*
 * NSS-defined key types
 *
 */
#define CKK_NSS (CKK_VENDOR_DEFINED | NSSCK_VENDOR_NSS)

#define CKK_NSS_PKCS8 (CKK_NSS + 1)

#define CKK_NSS_JPAKE_ROUND1 (CKK_NSS + 2)
#define CKK_NSS_JPAKE_ROUND2 (CKK_NSS + 3)

#define CKK_NSS_CHACHA20 (CKK_NSS + 4)

/*
 * NSS-defined certificate types
 *
 */
#define CKC_NSS (CKC_VENDOR_DEFINED | NSSCK_VENDOR_NSS)

/* FAKE PKCS #11 defines */
#define CKA_DIGEST 0x81000000L
#define CKA_NSS_MESSAGE 0x82000000L
#define CKA_NSS_MESSAGE_MASK 0xff000000L
#define CKA_FLAGS_ONLY 0 /* CKA_CLASS */

/*
 * NSS-defined object attributes
 *
 */
#define CKA_NSS (CKA_VENDOR_DEFINED | NSSCK_VENDOR_NSS)

#define CKA_NSS_URL (CKA_NSS + 1)
#define CKA_NSS_EMAIL (CKA_NSS + 2)
#define CKA_NSS_SMIME_INFO (CKA_NSS + 3)
#define CKA_NSS_SMIME_TIMESTAMP (CKA_NSS + 4)
#define CKA_NSS_PKCS8_SALT (CKA_NSS + 5)
#define CKA_NSS_PASSWORD_CHECK (CKA_NSS + 6)
#define CKA_NSS_EXPIRES (CKA_NSS + 7)
#define CKA_NSS_KRL (CKA_NSS + 8)

#define CKA_NSS_PQG_COUNTER (CKA_NSS + 20)
#define CKA_NSS_PQG_SEED (CKA_NSS + 21)
#define CKA_NSS_PQG_H (CKA_NSS + 22)
#define CKA_NSS_PQG_SEED_BITS (CKA_NSS + 23)
#define CKA_NSS_MODULE_SPEC (CKA_NSS + 24)
#define CKA_NSS_OVERRIDE_EXTENSIONS (CKA_NSS + 25)

#define CKA_NSS_JPAKE_SIGNERID (CKA_NSS + 26)
#define CKA_NSS_JPAKE_PEERID (CKA_NSS + 27)
#define CKA_NSS_JPAKE_GX1 (CKA_NSS + 28)
#define CKA_NSS_JPAKE_GX2 (CKA_NSS + 29)
#define CKA_NSS_JPAKE_GX3 (CKA_NSS + 30)
#define CKA_NSS_JPAKE_GX4 (CKA_NSS + 31)
#define CKA_NSS_JPAKE_X2 (CKA_NSS + 32)
#define CKA_NSS_JPAKE_X2S (CKA_NSS + 33)

#define CKA_NSS_MOZILLA_CA_POLICY (CKA_NSS + 34)
#define CKA_NSS_SERVER_DISTRUST_AFTER (CKA_NSS + 35)
#define CKA_NSS_EMAIL_DISTRUST_AFTER (CKA_NSS + 36)

/*
 * Trust attributes:
 *
 * If trust goes standard, these probably will too.  So I'll
 * put them all in one place.
 */

#define CKA_TRUST (CKA_NSS + 0x2000)

/* "Usage" key information */
#define CKA_TRUST_DIGITAL_SIGNATURE (CKA_TRUST + 1)
#define CKA_TRUST_NON_REPUDIATION (CKA_TRUST + 2)
#define CKA_TRUST_KEY_ENCIPHERMENT (CKA_TRUST + 3)
#define CKA_TRUST_DATA_ENCIPHERMENT (CKA_TRUST + 4)
#define CKA_TRUST_KEY_AGREEMENT (CKA_TRUST + 5)
#define CKA_TRUST_KEY_CERT_SIGN (CKA_TRUST + 6)
#define CKA_TRUST_CRL_SIGN (CKA_TRUST + 7)

/* "Purpose" trust information */
#define CKA_TRUST_SERVER_AUTH (CKA_TRUST + 8)
#define CKA_TRUST_CLIENT_AUTH (CKA_TRUST + 9)
#define CKA_TRUST_CODE_SIGNING (CKA_TRUST + 10)
#define CKA_TRUST_EMAIL_PROTECTION (CKA_TRUST + 11)
#define CKA_TRUST_IPSEC_END_SYSTEM (CKA_TRUST + 12)
#define CKA_TRUST_IPSEC_TUNNEL (CKA_TRUST + 13)
#define CKA_TRUST_IPSEC_USER (CKA_TRUST + 14)
#define CKA_TRUST_TIME_STAMPING (CKA_TRUST + 15)
#define CKA_TRUST_STEP_UP_APPROVED (CKA_TRUST + 16)

#define CKA_CERT_SHA1_HASH (CKA_TRUST + 100)
#define CKA_CERT_MD5_HASH (CKA_TRUST + 101)

/* NSS trust stuff */

/* HISTORICAL: define used to pass in the database key for DSA private keys */
#define CKA_NSS_DB 0xD5A0DB00L
#define CKA_NSS_TRUST 0x80000001L

/* FAKE PKCS #11 defines */
#define CKM_FAKE_RANDOM 0x80000efeUL
#define CKM_INVALID_MECHANISM 0xffffffffUL
#define CKT_INVALID_TYPE 0xffffffffUL

/*
 * NSS-defined crypto mechanisms
 *
 */
#define CKM_NSS (CKM_VENDOR_DEFINED | NSSCK_VENDOR_NSS)

#define CKM_NSS_AES_KEY_WRAP (CKM_NSS + 1)
#define CKM_NSS_AES_KEY_WRAP_PAD (CKM_NSS + 2)

/* HKDF key derivation mechanisms. See CK_NSS_HKDFParams for documentation. */
#define CKM_NSS_HKDF_SHA1 (CKM_NSS + 3)
#define CKM_NSS_HKDF_SHA256 (CKM_NSS + 4)
#define CKM_NSS_HKDF_SHA384 (CKM_NSS + 5)
#define CKM_NSS_HKDF_SHA512 (CKM_NSS + 6)

/* J-PAKE round 1 key generation mechanisms.
 *
 * Required template attributes: CKA_PRIME, CKA_SUBPRIME, CKA_BASE,
 *                               CKA_NSS_JPAKE_SIGNERID
 * Output key type: CKK_NSS_JPAKE_ROUND1
 * Output key class: CKO_PRIVATE_KEY
 * Parameter type: CK_NSS_JPAKERound1Params
 *
 */
#define CKM_NSS_JPAKE_ROUND1_SHA1 (CKM_NSS + 7)
#define CKM_NSS_JPAKE_ROUND1_SHA256 (CKM_NSS + 8)
#define CKM_NSS_JPAKE_ROUND1_SHA384 (CKM_NSS + 9)
#define CKM_NSS_JPAKE_ROUND1_SHA512 (CKM_NSS + 10)

/* J-PAKE round 2 key derivation mechanisms.
 *
 * Required template attributes: CKA_NSS_JPAKE_PEERID
 * Input key type:  CKK_NSS_JPAKE_ROUND1
 * Output key type: CKK_NSS_JPAKE_ROUND2
 * Output key class: CKO_PRIVATE_KEY
 * Parameter type: CK_NSS_JPAKERound2Params
 */
#define CKM_NSS_JPAKE_ROUND2_SHA1 (CKM_NSS + 11)
#define CKM_NSS_JPAKE_ROUND2_SHA256 (CKM_NSS + 12)
#define CKM_NSS_JPAKE_ROUND2_SHA384 (CKM_NSS + 13)
#define CKM_NSS_JPAKE_ROUND2_SHA512 (CKM_NSS + 14)

/* J-PAKE final key material derivation mechanisms
 *
 * Input key type:  CKK_NSS_JPAKE_ROUND2
 * Output key type: CKK_GENERIC_SECRET
 * Output key class: CKO_SECRET_KEY
 * Parameter type: CK_NSS_JPAKEFinalParams
 *
 * You must apply a KDF (e.g. CKM_NSS_HKDF_*) to resultant keying material
 * to get a key with uniformly distributed bits.
 */
#define CKM_NSS_JPAKE_FINAL_SHA1 (CKM_NSS + 15)
#define CKM_NSS_JPAKE_FINAL_SHA256 (CKM_NSS + 16)
#define CKM_NSS_JPAKE_FINAL_SHA384 (CKM_NSS + 17)
#define CKM_NSS_JPAKE_FINAL_SHA512 (CKM_NSS + 18)

/* Constant-time MAC mechanisms:
 *
 * These operations verify a padded, MAC-then-encrypt block of data in
 * constant-time. Because of the order of operations, the padding bytes are not
 * protected by the MAC. However, disclosing the value of the padding bytes
 * gives an attacker the ability to decrypt ciphertexts. Such disclosure can be
 * as subtle as taking slightly less time to perform the MAC when the padding
 * is one byte longer. See https://www.isg.rhul.ac.uk/tls/
 *
 * CKM_NSS_HMAC_CONSTANT_TIME: performs an HMAC authentication.
 * CKM_NSS_SSL3_MAC_CONSTANT_TIME: performs an authentication with SSLv3 MAC.
 *
 * Parameter type: CK_NSS_MAC_CONSTANT_TIME_PARAMS
 */
#define CKM_NSS_HMAC_CONSTANT_TIME (CKM_NSS + 19)
#define CKM_NSS_SSL3_MAC_CONSTANT_TIME (CKM_NSS + 20)

/* TLS 1.2 mechanisms */
#define CKM_NSS_TLS_PRF_GENERAL_SHA256 (CKM_NSS + 21)
#define CKM_NSS_TLS_MASTER_KEY_DERIVE_SHA256 (CKM_NSS + 22)
#define CKM_NSS_TLS_KEY_AND_MAC_DERIVE_SHA256 (CKM_NSS + 23)
#define CKM_NSS_TLS_MASTER_KEY_DERIVE_DH_SHA256 (CKM_NSS + 24)

/* TLS extended master secret derivation */
#define CKM_NSS_TLS_EXTENDED_MASTER_KEY_DERIVE (CKM_NSS + 25)
#define CKM_NSS_TLS_EXTENDED_MASTER_KEY_DERIVE_DH (CKM_NSS + 26)

#define CKM_NSS_CHACHA20_KEY_GEN (CKM_NSS + 27)
#define CKM_NSS_CHACHA20_POLY1305 (CKM_NSS + 28)

/* Additional PKCS #12 PBE algorithms defined in v1.1 */
#define CKM_NSS_PKCS12_PBE_SHA224_HMAC_KEY_GEN (CKM_NSS + 29)
#define CKM_NSS_PKCS12_PBE_SHA256_HMAC_KEY_GEN (CKM_NSS + 30)
#define CKM_NSS_PKCS12_PBE_SHA384_HMAC_KEY_GEN (CKM_NSS + 31)
#define CKM_NSS_PKCS12_PBE_SHA512_HMAC_KEY_GEN (CKM_NSS + 32)

#define CKM_NSS_CHACHA20_CTR (CKM_NSS + 33)

/* IKE mechanism (to be proposed to PKCS #11 */
#define CKM_NSS_IKE_PRF_PLUS_DERIVE (CKM_NSS + 34)
#define CKM_NSS_IKE_PRF_DERIVE (CKM_NSS + 35)
#define CKM_NSS_IKE1_PRF_DERIVE (CKM_NSS + 36)
#define CKM_NSS_IKE1_APP_B_PRF_DERIVE (CKM_NSS + 37)

#define CKM_NSS_PUB_FROM_PRIV (CKM_NSS + 40)

/* SP800-108 NSS mechanism with support for data object derivation */
#define CKM_NSS_SP800_108_COUNTER_KDF_DERIVE_DATA (CKM_NSS + 42)
#define CKM_NSS_SP800_108_FEEDBACK_KDF_DERIVE_DATA (CKM_NSS + 43)
#define CKM_NSS_SP800_108_DOUBLE_PIPELINE_KDF_DERIVE_DATA (CKM_NSS + 44)

/*
 * HISTORICAL:
 * Do not attempt to use these. They are only used by NSS's internal
 * PKCS #11 interface. Most of these are place holders for other mechanism
 * and will change in the future.
 */
#define CKM_NSS_PBE_SHA1_DES_CBC 0x80000002UL
#define CKM_NSS_PBE_SHA1_TRIPLE_DES_CBC 0x80000003UL
#define CKM_NSS_PBE_SHA1_40_BIT_RC2_CBC 0x80000004UL
#define CKM_NSS_PBE_SHA1_128_BIT_RC2_CBC 0x80000005UL
#define CKM_NSS_PBE_SHA1_40_BIT_RC4 0x80000006UL
#define CKM_NSS_PBE_SHA1_128_BIT_RC4 0x80000007UL
#define CKM_NSS_PBE_SHA1_FAULTY_3DES_CBC 0x80000008UL
#define CKM_NSS_PBE_SHA1_HMAC_KEY_GEN 0x80000009UL
#define CKM_NSS_PBE_MD5_HMAC_KEY_GEN 0x8000000aUL
#define CKM_NSS_PBE_MD2_HMAC_KEY_GEN 0x8000000bUL

#define CKM_TLS_PRF_GENERAL 0x80000373UL

typedef struct CK_NSS_JPAKEPublicValue {
    CK_BYTE *pGX;
    CK_ULONG ulGXLen;
    CK_BYTE *pGV;
    CK_ULONG ulGVLen;
    CK_BYTE *pR;
    CK_ULONG ulRLen;
} CK_NSS_JPAKEPublicValue;

typedef struct CK_NSS_JPAKERound1Params {
    CK_NSS_JPAKEPublicValue gx1; /* out */
    CK_NSS_JPAKEPublicValue gx2; /* out */
} CK_NSS_JPAKERound1Params;

typedef struct CK_NSS_JPAKERound2Params {
    CK_BYTE *pSharedKey;         /* in */
    CK_ULONG ulSharedKeyLen;     /* in */
    CK_NSS_JPAKEPublicValue gx3; /* in */
    CK_NSS_JPAKEPublicValue gx4; /* in */
    CK_NSS_JPAKEPublicValue A;   /* out */
} CK_NSS_JPAKERound2Params;

typedef struct CK_NSS_JPAKEFinalParams {
    CK_NSS_JPAKEPublicValue B; /* in */
} CK_NSS_JPAKEFinalParams;

/* macAlg: the MAC algorithm to use. This determines the hash function used in
 *     the HMAC/SSLv3 MAC calculations.
 * ulBodyTotalLen: the total length of the data, including padding bytes and
 *     padding length.
 * pHeader: points to a block of data that contains additional data to
 *     authenticate. For TLS this includes the sequence number etc. For SSLv3,
 *     this also includes the initial padding bytes.
 *
 * NOTE: the softoken's implementation of CKM_NSS_HMAC_CONSTANT_TIME and
 * CKM_NSS_SSL3_MAC_CONSTANT_TIME requires that the sum of ulBodyTotalLen
 * and ulHeaderLen be much smaller than 2^32 / 8 bytes because it uses an
 * unsigned int variable to represent the length in bits. This should not
 * be a problem because the SSL/TLS protocol limits the size of an SSL
 * record to something considerably less than 2^32 bytes.
 */
typedef struct CK_NSS_MAC_CONSTANT_TIME_PARAMS {
    CK_MECHANISM_TYPE macAlg; /* in */
    CK_ULONG ulBodyTotalLen;  /* in */
    CK_BYTE *pHeader;         /* in */
    CK_ULONG ulHeaderLen;     /* in */
} CK_NSS_MAC_CONSTANT_TIME_PARAMS;

typedef struct CK_NSS_AEAD_PARAMS {
    CK_BYTE_PTR pNonce;
    CK_ULONG ulNonceLen;
    CK_BYTE_PTR pAAD;
    CK_ULONG ulAADLen;
    CK_ULONG ulTagLen;
} CK_NSS_AEAD_PARAMS;

/*
 * NSS-defined return values
 *
 */
#define CKR_NSS (CKM_VENDOR_DEFINED | NSSCK_VENDOR_NSS)

#define CKR_NSS_CERTDB_FAILED (CKR_NSS + 1)
#define CKR_NSS_KEYDB_FAILED (CKR_NSS + 2)

/* Mandatory parameter for the CKM_NSS_HKDF_* key deriviation mechanisms.
   See RFC 5869.

    bExtract: If set, HKDF-Extract will be applied to the input key. If
              the optional salt is given, it is used; otherwise, the salt is
              set to a sequence of zeros equal in length to the HMAC output.
              If bExpand is not set, then the key template given to
              C_DeriveKey must indicate an output key size less than or equal
              to the output size of the HMAC.

    bExpand:  If set, HKDF-Expand will be applied to the input key (if
              bExtract is not set) or to the result of HKDF-Extract (if
              bExtract is set). Any info given in the optional pInfo field will
              be included in the calculation.

    The size of the output key must be specified in the template passed to
    C_DeriveKey.
*/
typedef struct CK_NSS_HKDFParams {
    CK_BBOOL bExtract;
    CK_BYTE_PTR pSalt;
    CK_ULONG ulSaltLen;
    CK_BBOOL bExpand;
    CK_BYTE_PTR pInfo;
    CK_ULONG ulInfoLen;
} CK_NSS_HKDFParams;

/*
 * CK_NSS_IKE_PRF_PLUS_PARAMS is a structure that provides the parameters to
 * the CKM_NSS_IKE_PRF_PLUS_DERIVE mechanism.
 * The fields of the structure have the following meanings:
 *      prfMechanism    underlying MAC mechanism used to generate the prf.
 *      bHasSeedKey     hSeed key is present.
 *      hSeedKey        optional seed from key
 *      pSeedData       optional seed from data.
 *      ulSeedDataLen   length of optional seed data.
 *        If no seed data is present this value is NULL.
 */
typedef struct CK_NSS_IKE_PRF_PLUS_DERIVE_PARAMS {
    CK_MECHANISM_TYPE prfMechanism;
    CK_BBOOL bHasSeedKey;
    CK_OBJECT_HANDLE hSeedKey;
    CK_BYTE_PTR pSeedData;
    CK_ULONG ulSeedDataLen;
} CK_NSS_IKE_PRF_PLUS_DERIVE_PARAMS;

/* CK_NSS_IKE_PRF_DERIVE_PARAMS is a structure that provides the parameters to
 *  the CKM_NSS_IKE_PRF_DERIVE mechanism.
 *
 * The fields of the structure have the following meanings:
 *     prfMechanism underlying MAC mechanism used to generate the prf.
 *     bRekey       hNewKey is present.
 *     pNi          Ni value
 *     ulNiLen      length of Ni
 *     pNr          Nr value
 *     ulNrLen      length of Nr
 *     hNewKey      New key value to drive the rekey.
 */
typedef struct CK_NSS_IKE_PRF_DERIVE_PARAMS {
    CK_MECHANISM_TYPE prfMechanism;
    CK_BBOOL bDataAsKey;
    CK_BBOOL bRekey;
    CK_BYTE_PTR pNi;
    CK_ULONG ulNiLen;
    CK_BYTE_PTR pNr;
    CK_ULONG ulNrLen;
    CK_OBJECT_HANDLE hNewKey;
} CK_NSS_IKE_PRF_DERIVE_PARAMS;

/* CK_NSS_IKE1_PRF_DERIVE_PARAMS is a structure that provides the parameters
 * to the CKM_NSS_IKE_PRF_DERIVE mechanism.
 *
 * The fields of the structure have the following meanings:
 *     prfMechanism  underlying MAC mechanism used to generate the prf.
 *     bRekey        hNewKey is present.
 *     pCKYi         CKYi value
 *     ulCKYiLen     length of CKYi
 *     pCKYr         CKYr value
 *     ulCKYrLen     length of CKYr
 *     hNewKey       New key value to drive the rekey.
 */
typedef struct CK_NSS_IKE1_PRF_DERIVE_PARAMS {
    CK_MECHANISM_TYPE prfMechanism;
    CK_BBOOL bHasPrevKey;
    CK_OBJECT_HANDLE hKeygxy;
    CK_OBJECT_HANDLE hPrevKey;
    CK_BYTE_PTR pCKYi;
    CK_ULONG ulCKYiLen;
    CK_BYTE_PTR pCKYr;
    CK_ULONG ulCKYrLen;
    CK_BYTE keyNumber;
} CK_NSS_IKE1_PRF_DERIVE_PARAMS;

/* CK_NSS_IKE1_APP_B_PRF_DERIVE_PARAMS is a structure that provides the
 * parameters to the CKM_NSS_IKE_APP_B_PRF_DERIVE mechanism.
 *
 * The fields of the structure have the following meanings:
 *     prfMechanism  underlying MAC mechanism used to generate the prf.
 *     bHasKeygxy    hKeygxy exists
 *     hKeygxy       optional key to hash in the prf
 *     pExtraData    optional extra data to hash in the prf
 *     ulExtraData   length of the optional extra data.
 *
 * CK_NSS_IKE_APP_B_PRF_DERIVE can take wither CK_NSS_IKE1_APP_B_PRF_DRIVE_PARAMS
 * or a single CK_MECHANISM_TYPE. In the latter cases bHashKeygx is assumed to
 * be false and ulExtraDataLen is assumed to be '0'.
 */
typedef struct CK_NSS_IKE1_APP_B_PRF_DERIVE_PARAMS {
    CK_MECHANISM_TYPE prfMechanism;
    CK_BBOOL bHasKeygxy;
    CK_OBJECT_HANDLE hKeygxy;
    CK_BYTE_PTR pExtraData;
    CK_ULONG ulExtraDataLen;
} CK_NSS_IKE1_APP_B_PRF_DERIVE_PARAMS;

/*
 * Parameter for the TLS extended master secret key derivation mechanisms:
 *
 *  * CKM_NSS_TLS_EXTENDED_MASTER_KEY_DERIVE
 *  * CKM_NSS_TLS_EXTENDED_MASTER_KEY_DERIVE_DH
 *
 * For the TLS 1.2 PRF, the prfHashMechanism parameter determines the hash
 * function used. For earlier versions of the PRF, set the prfHashMechanism
 * value to CKM_TLS_PRF.
 *
 * The session hash input is expected to be the output of the same hash
 * function as the PRF uses (as required by draft-ietf-tls-session-hash).  So
 * the ulSessionHashLen member must be equal the output length of the hash
 * function specified by the prfHashMechanism member (or, for pre-TLS 1.2 PRF,
 * the length of concatenated MD5 and SHA-1 digests).
 *
 */
typedef struct CK_NSS_TLS_EXTENDED_MASTER_KEY_DERIVE_PARAMS {
    CK_MECHANISM_TYPE prfHashMechanism;
    CK_BYTE_PTR pSessionHash;
    CK_ULONG ulSessionHashLen;
    CK_VERSION_PTR pVersion;
} CK_NSS_TLS_EXTENDED_MASTER_KEY_DERIVE_PARAMS;

/*
 * Trust info
 *
 * This isn't part of the Cryptoki standard (yet), so I'm putting
 * all the definitions here.  Some of this would move to nssckt.h
 * if trust info were made part of the standard.  In view of this
 * possibility, I'm putting my (NSS) values in the NSS
 * vendor space, like everything else.
 */

typedef CK_ULONG CK_TRUST;

/* The following trust types are defined: */
#define CKT_VENDOR_DEFINED 0x80000000

#define CKT_NSS (CKT_VENDOR_DEFINED | NSSCK_VENDOR_NSS)

/* If trust goes standard, these'll probably drop out of vendor space. */
#define CKT_NSS_TRUSTED (CKT_NSS + 1)
#define CKT_NSS_TRUSTED_DELEGATOR (CKT_NSS + 2)
#define CKT_NSS_MUST_VERIFY_TRUST (CKT_NSS + 3)
#define CKT_NSS_NOT_TRUSTED (CKT_NSS + 10)
#define CKT_NSS_TRUST_UNKNOWN (CKT_NSS + 5) /* default */

/*
 * These may well remain NSS-specific; I'm only using them
 * to cache resolution data.
 */
#define CKT_NSS_VALID_DELEGATOR (CKT_NSS + 11)

/*
 * old definitions. They still exist, but the plain meaning of the
 * labels have never been accurate to what was really implemented.
 * The new labels correctly reflect what the values effectively mean.
 */
#if defined(__GNUC__) && (__GNUC__ > 3)
/* make GCC warn when we use these #defines */
/*
 *  This is really painful because GCC doesn't allow us to mark random
 *  #defines as deprecated. We can only mark the following:
 *      functions, variables, and types.
 *  const variables will create extra storage for everyone including this
 *       header file, so it's undesirable.
 *  functions could be inlined to prevent storage creation, but will fail
 *       when constant values are expected (like switch statements).
 *  enum types do not seem to pay attention to the deprecated attribute.
 *
 *  That leaves typedefs. We declare new types that we then deprecate, then
 *  cast the resulting value to the deprecated type in the #define, thus
 *  producting the warning when the #define is used.
 */
#if (__GNUC__ == 4) && (__GNUC_MINOR__ < 5)
/* The mac doesn't like the friendlier deprecate messages. I'm assuming this
 * is a gcc version issue rather than mac or ppc specific */
typedef CK_TRUST __CKT_NSS_UNTRUSTED __attribute__((deprecated));
typedef CK_TRUST __CKT_NSS_VALID __attribute__((deprecated));
typedef CK_TRUST __CKT_NSS_MUST_VERIFY __attribute__((deprecated));
#else
/* when possible, get a full deprecation warning. This works on gcc 4.5
 * it may work on earlier versions of gcc */
typedef CK_TRUST __CKT_NSS_UNTRUSTED __attribute__((deprecated("CKT_NSS_UNTRUSTED really means CKT_NSS_MUST_VERIFY_TRUST")));
typedef CK_TRUST __CKT_NSS_VALID __attribute__((deprecated("CKT_NSS_VALID really means CKT_NSS_NOT_TRUSTED")));
typedef CK_TRUST __CKT_NSS_MUST_VERIFY __attribute__((deprecated("CKT_NSS_MUST_VERIFY really functions as CKT_NSS_TRUST_UNKNOWN")));
#endif
#define CKT_NSS_UNTRUSTED ((__CKT_NSS_UNTRUSTED)CKT_NSS_MUST_VERIFY_TRUST)
#define CKT_NSS_VALID ((__CKT_NSS_VALID)CKT_NSS_NOT_TRUSTED)
/* keep the old value for compatibility reasons*/
#define CKT_NSS_MUST_VERIFY ((__CKT_NSS_MUST_VERIFY)(CKT_NSS + 4))
#else
#ifdef _WIN32
/* This magic gets the windows compiler to give us a deprecation
 * warning */
#pragma deprecated(CKT_NSS_UNTRUSTED, CKT_NSS_MUST_VERIFY, CKT_NSS_VALID)
#endif
/* CKT_NSS_UNTRUSTED really means CKT_NSS_MUST_VERIFY_TRUST */
#define CKT_NSS_UNTRUSTED CKT_NSS_MUST_VERIFY_TRUST
/* CKT_NSS_VALID really means CKT_NSS_NOT_TRUSTED */
#define CKT_NSS_VALID CKT_NSS_NOT_TRUSTED
/* CKT_NSS_MUST_VERIFY was always treated as CKT_NSS_TRUST_UNKNOWN */
#define CKT_NSS_MUST_VERIFY (CKT_NSS + 4) /*really means trust unknown*/
#endif

/*
 * These are not really PKCS #11 values specifically. They are the 'loadable'
 * module spec NSS uses. They are available for others to use as well, but not
 * part of the formal PKCS #11 spec.
 *
 * The function 'FIND' returns an array of PKCS #11 initialization strings
 * The function 'ADD' takes a PKCS #11 initialization string and stores it.
 * The function 'DEL' takes a 'name= library=' value and deletes the associated
 *  string.
 * The function 'RELEASE' frees the array returned by 'FIND'
 */
#define SECMOD_MODULE_DB_FUNCTION_FIND 0
#define SECMOD_MODULE_DB_FUNCTION_ADD 1
#define SECMOD_MODULE_DB_FUNCTION_DEL 2
#define SECMOD_MODULE_DB_FUNCTION_RELEASE 3
typedef char **(PR_CALLBACK *SECMODModuleDBFunc)(unsigned long function,
                                                 char *parameters, void *moduleSpec);

/* softoken slot ID's */
#define SFTK_MIN_USER_SLOT_ID 4
#define SFTK_MAX_USER_SLOT_ID 100
#define SFTK_MIN_FIPS_USER_SLOT_ID 101
#define SFTK_MAX_FIPS_USER_SLOT_ID 127

/* Module Interface. This is the old NSS private module interface, now exported
 * as a PKCS #11 v3 interface. It's interface name is
 * "Vendor NSS Module Interface" */
typedef char **(*CK_NSS_ModuleDBFunc)(unsigned long function,
                                      char *parameters, void *args);
typedef struct CK_NSS_MODULE_FUNCTIONS {
    CK_VERSION version;
    CK_NSS_ModuleDBFunc NSC_ModuleDBFunc;
} CK_NSS_MODULE_FUNCTIONS;

/* There was an inconsistency between the spec and the header file in defining
 * the CK_GCM_PARAMS structure. The authoritative reference is the header file,
 * but NSS used the spec when adding it to its own header. In V3 we've
 * corrected it, but we need to handle the old case for devices that followed
 * us in using the incorrect specification. */
typedef struct CK_NSS_GCM_PARAMS {
    CK_BYTE_PTR pIv;
    CK_ULONG ulIvLen;
    CK_BYTE_PTR pAAD;
    CK_ULONG ulAADLen;
    CK_ULONG ulTagBits;
} CK_NSS_GCM_PARAMS;

typedef CK_NSS_GCM_PARAMS CK_PTR CK_NSS_GCM_PARAMS_PTR;

/* deprecated #defines. Drop in future NSS releases */
#ifdef NSS_PKCS11_2_0_COMPAT

/* defines that were changed between NSS's PKCS #11 and the Oasis headers */
#define CKF_EC_FP CKF_EC_F_P
#define CKO_KG_PARAMETERS CKO_DOMAIN_PARAMETERS
#define CK_INVALID_SESSION CK_INVALID_HANDLE
#define CKR_KEY_PARAMS_INVALID 0x0000006B

/* use the old wrong CK_GCM_PARAMS is NSS_PCKS11_2_0_COMPAT is defined */
typedef struct CK_NSS_GCM_PARAMS CK_GCM_PARAMS;
typedef CK_NSS_GCM_PARAMS CK_PTR CK_GCM_PARAMS_PTR;

/* don't leave old programs in a lurch just yet, give them the old NETSCAPE
 * synonym if NSS_PKCS11_2_0_COMPAT is defined*/
#define CKO_NETSCAPE_CRL CKO_NSS_CRL
#define CKO_NETSCAPE_SMIME CKO_NSS_SMIME
#define CKO_NETSCAPE_TRUST CKO_NSS_TRUST
#define CKO_NETSCAPE_BUILTIN_ROOT_LIST CKO_NSS_BUILTIN_ROOT_LIST
#define CKO_NETSCAPE_NEWSLOT CKO_NSS_NEWSLOT
#define CKO_NETSCAPE_DELSLOT CKO_NSS_DELSLOT
#define CKK_NETSCAPE_PKCS8 CKK_NSS_PKCS8
#define CKA_NETSCAPE_URL CKA_NSS_URL
#define CKA_NETSCAPE_EMAIL CKA_NSS_EMAIL
#define CKA_NETSCAPE_SMIME_INFO CKA_NSS_SMIME_INFO
#define CKA_NETSCAPE_SMIME_TIMESTAMP CKA_NSS_SMIME_TIMESTAMP
#define CKA_NETSCAPE_PKCS8_SALT CKA_NSS_PKCS8_SALT
#define CKA_NETSCAPE_PASSWORD_CHECK CKA_NSS_PASSWORD_CHECK
#define CKA_NETSCAPE_EXPIRES CKA_NSS_EXPIRES
#define CKA_NETSCAPE_KRL CKA_NSS_KRL
#define CKA_NETSCAPE_PQG_COUNTER CKA_NSS_PQG_COUNTER
#define CKA_NETSCAPE_PQG_SEED CKA_NSS_PQG_SEED
#define CKA_NETSCAPE_PQG_H CKA_NSS_PQG_H
#define CKA_NETSCAPE_PQG_SEED_BITS CKA_NSS_PQG_SEED_BITS
#define CKA_NETSCAPE_MODULE_SPEC CKA_NSS_MODULE_SPEC
#define CKA_NETSCAPE_DB CKA_NSS_DB
#define CKA_NETSCAPE_TRUST CKA_NSS_TRUST
#define CKM_NETSCAPE_AES_KEY_WRAP CKM_NSS_AES_KEY_WRAP
#define CKM_NETSCAPE_AES_KEY_WRAP_PAD CKM_NSS_AES_KEY_WRAP_PAD
#define CKM_NETSCAPE_PBE_SHA1_DES_CBC CKM_NSS_PBE_SHA1_DES_CBC
#define CKM_NETSCAPE_PBE_SHA1_TRIPLE_DES_CBC CKM_NSS_PBE_SHA1_TRIPLE_DES_CBC
#define CKM_NETSCAPE_PBE_SHA1_40_BIT_RC2_CBC CKM_NSS_PBE_SHA1_40_BIT_RC2_CBC
#define CKM_NETSCAPE_PBE_SHA1_128_BIT_RC2_CBC CKM_NSS_PBE_SHA1_128_BIT_RC2_CBC
#define CKM_NETSCAPE_PBE_SHA1_40_BIT_RC4 CKM_NSS_PBE_SHA1_40_BIT_RC4
#define CKM_NETSCAPE_PBE_SHA1_128_BIT_RC4 CKM_NSS_PBE_SHA1_128_BIT_RC4
#define CKM_NETSCAPE_PBE_SHA1_FAULTY_3DES_CBC CKM_NSS_PBE_SHA1_FAULTY_3DES_CBC
#define CKM_NETSCAPE_PBE_SHA1_HMAC_KEY_GEN CKM_NSS_PBE_SHA1_HMAC_KEY_GEN
#define CKM_NETSCAPE_PBE_MD5_HMAC_KEY_GEN CKM_NSS_PBE_MD5_HMAC_KEY_GEN
#define CKM_NETSCAPE_PBE_MD2_HMAC_KEY_GEN CKM_NSS_PBE_MD2_HMAC_KEY_GEN
#define CKR_NETSCAPE_CERTDB_FAILED CKR_NSS_CERTDB_FAILED
#define CKR_NETSCAPE_KEYDB_FAILED CKR_NSS_KEYDB_FAILED

#define CKT_NETSCAPE_TRUSTED CKT_NSS_TRUSTED
#define CKT_NETSCAPE_TRUSTED_DELEGATOR CKT_NSS_TRUSTED_DELEGATOR
#define CKT_NETSCAPE_UNTRUSTED CKT_NSS_UNTRUSTED
#define CKT_NETSCAPE_MUST_VERIFY CKT_NSS_MUST_VERIFY
#define CKT_NETSCAPE_TRUST_UNKNOWN CKT_NSS_TRUST_UNKNOWN
#define CKT_NETSCAPE_VALID CKT_NSS_VALID
#define CKT_NETSCAPE_VALID_DELEGATOR CKT_NSS_VALID_DELEGATOR
#else
/* use the new CK_GCM_PARAMS if NSS_PKCS11_2_0_COMPAT is not defined */
typedef struct CK_GCM_PARAMS_V3 CK_GCM_PARAMS;
typedef CK_GCM_PARAMS_V3 CK_PTR CK_GCM_PARAMS_PTR;
#endif

#endif /* _PKCS11N_H_ */
