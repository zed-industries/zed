/*
 * blapit.h - public data structures for the freebl library
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef _BLAPIT_H_
#define _BLAPIT_H_

#include "seccomon.h"
#include "prlink.h"
#include "plarena.h"
#include "ecl-exp.h"

/* RC2 operation modes */
#define NSS_RC2 0
#define NSS_RC2_CBC 1

/* RC5 operation modes */
#define NSS_RC5 0
#define NSS_RC5_CBC 1

/* DES operation modes */
#define NSS_DES 0
#define NSS_DES_CBC 1
#define NSS_DES_EDE3 2
#define NSS_DES_EDE3_CBC 3

#define DES_KEY_LENGTH 8 /* Bytes */

/* AES operation modes */
#define NSS_AES 0
#define NSS_AES_CBC 1
#define NSS_AES_CTS 2
#define NSS_AES_CTR 3
#define NSS_AES_GCM 4

/* Camellia operation modes */
#define NSS_CAMELLIA 0
#define NSS_CAMELLIA_CBC 1

/* SEED operation modes */
#define NSS_SEED 0
#define NSS_SEED_CBC 1

#define DSA1_SUBPRIME_LEN 20                             /* Bytes */
#define DSA1_SIGNATURE_LEN (DSA1_SUBPRIME_LEN * 2)       /* Bytes */
#define DSA_MAX_SUBPRIME_LEN 32                          /* Bytes */
#define DSA_MAX_SIGNATURE_LEN (DSA_MAX_SUBPRIME_LEN * 2) /* Bytes */

/*
 * Mark the old defines as deprecated. This will warn code that expected
 * DSA1 only that they need to change if the are to support DSA2.
 */
#if defined(__GNUC__) && (__GNUC__ > 3)
/* make GCC warn when we use these #defines */
typedef int __BLAPI_DEPRECATED __attribute__((deprecated));
#define DSA_SUBPRIME_LEN ((__BLAPI_DEPRECATED)DSA1_SUBPRIME_LEN)
#define DSA_SIGNATURE_LEN ((__BLAPI_DEPRECATED)DSA1_SIGNATURE_LEN)
#define DSA_Q_BITS ((__BLAPI_DEPRECATED)(DSA1_SUBPRIME_LEN * 8))
#else
#ifdef _WIN32
/* This magic gets the windows compiler to give us a deprecation
 * warning */
#pragma deprecated(DSA_SUBPRIME_LEN, DSA_SIGNATURE_LEN, DSA_QBITS)
#endif
#define DSA_SUBPRIME_LEN DSA1_SUBPRIME_LEN
#define DSA_SIGNATURE_LEN DSA1_SIGNATURE_LEN
#define DSA_Q_BITS (DSA1_SUBPRIME_LEN * 8)
#endif

/* XXX We shouldn't have to hard code this limit. For
 * now, this is the quickest way to support ECDSA signature
 * processing (ECDSA signature lengths depend on curve
 * size). This limit is sufficient for curves upto
 * 576 bits.
 */
#define MAX_ECKEY_LEN 72 /* Bytes */

#define EC_MAX_KEY_BITS 521 /* in bits */
#define EC_MIN_KEY_BITS 256 /* in bits */

/* EC point compression format */
#define EC_POINT_FORM_COMPRESSED_Y0 0x02
#define EC_POINT_FORM_COMPRESSED_Y1 0x03
#define EC_POINT_FORM_UNCOMPRESSED 0x04
#define EC_POINT_FORM_HYBRID_Y0 0x06
#define EC_POINT_FORM_HYBRID_Y1 0x07

/*
 * Number of bytes each hash algorithm produces
 */
#define MD2_LENGTH 16        /* Bytes */
#define MD5_LENGTH 16        /* Bytes */
#define SHA1_LENGTH 20       /* Bytes */
#define SHA256_LENGTH 32     /* bytes */
#define SHA384_LENGTH 48     /* bytes */
#define SHA512_LENGTH 64     /* bytes */
#define BLAKE2B512_LENGTH 64 /* Bytes */
#define HASH_LENGTH_MAX SHA512_LENGTH

/*
 * Input block size for each hash algorithm.
 */

#define MD2_BLOCK_LENGTH 64      /* bytes */
#define MD5_BLOCK_LENGTH 64      /* bytes */
#define SHA1_BLOCK_LENGTH 64     /* bytes */
#define SHA224_BLOCK_LENGTH 64   /* bytes */
#define SHA256_BLOCK_LENGTH 64   /* bytes */
#define SHA384_BLOCK_LENGTH 128  /* bytes */
#define SHA512_BLOCK_LENGTH 128  /* bytes */
#define BLAKE2B_BLOCK_LENGTH 128 /* Bytes */
#define HASH_BLOCK_LENGTH_MAX SHA512_BLOCK_LENGTH

#define AES_BLOCK_SIZE 16 /* bytes */
#define AES_KEY_WRAP_BLOCK_SIZE (AES_BLOCK_SIZE / 2)
#define AES_KEY_WRAP_IV_BYTES AES_KEY_WRAP_BLOCK_SIZE

#define AES_128_KEY_LENGTH 16 /* bytes */
#define AES_192_KEY_LENGTH 24 /* bytes */
#define AES_256_KEY_LENGTH 32 /* bytes */

#define CAMELLIA_BLOCK_SIZE 16 /* bytes */

#define SEED_BLOCK_SIZE 16 /* bytes */
#define SEED_KEY_LENGTH 16 /* bytes */

#define NSS_FREEBL_DEFAULT_CHUNKSIZE 2048

#define BLAKE2B_KEY_SIZE 64

/*
 * These values come from the initial key size limits from the PKCS #11
 * module. They may be arbitrarily adjusted to any value freebl supports.
 */
#define RSA_MIN_MODULUS_BITS 128
#define RSA_MAX_MODULUS_BITS 16384
#define RSA_MAX_EXPONENT_BITS 64
#define DH_MIN_P_BITS 128
#define DH_MAX_P_BITS 16384

/*
 * The FIPS 186-1 algorithm for generating primes P and Q allows only 9
 * distinct values for the length of P, and only one value for the
 * length of Q.
 * The algorithm uses a variable j to indicate which of the 9 lengths
 * of P is to be used.
 * The following table relates j to the lengths of P and Q in bits.
 *
 *  j   bits in P   bits in Q
 *  _   _________   _________
 *  0    512        160
 *  1    576        160
 *  2    640        160
 *  3    704        160
 *  4    768        160
 *  5    832        160
 *  6    896        160
 *  7    960        160
 *  8   1024        160
 *
 * The FIPS-186-1 compliant PQG generator takes j as an input parameter.
 *
 * FIPS 186-3 algorithm specifies 4 distinct P and Q sizes:
 *
 *     bits in P       bits in Q
 *     _________       _________
 *      1024           160
 *      2048           224
 *      2048           256
 *      3072           256
 *
 * The FIPS-186-3 complaiant PQG generator (PQG V2) takes arbitrary p and q
 * lengths as input and returns an error if they aren't in this list.
 */

#define DSA1_Q_BITS 160
#define DSA_MAX_P_BITS 3072
#define DSA_MIN_P_BITS 512
#define DSA_MAX_Q_BITS 256
#define DSA_MIN_Q_BITS 160

#if DSA_MAX_Q_BITS != DSA_MAX_SUBPRIME_LEN * 8
#error "Inconsistent declaration of DSA SUBPRIME/Q parameters in blapit.h"
#endif

/*
 * function takes desired number of bits in P,
 * returns index (0..8) or -1 if number of bits is invalid.
 */
#define PQG_PBITS_TO_INDEX(bits) \
    (((bits) < 512 || (bits) > 1024 || (bits) % 64) ? -1 : (int)((bits)-512) / 64)

/*
 * function takes index (0-8)
 * returns number of bits in P for that index, or -1 if index is invalid.
 */
#define PQG_INDEX_TO_PBITS(j) (((unsigned)(j) > 8) ? -1 : (512 + 64 * (j)))

/* When we are generating a gcm iv from a random number, we need to calculate
 * an acceptable iteration count to avoid birthday attacks. (randomly
 * generating the same IV twice).
 *
 * We use the approximation n = sqrt(2*m*p) to find an acceptable n given m
 * and p.
 * where n is the number of iterations.
 *       m is the number of possible random values.
 *       p is the probability of collision (0-1).
 *
 * We want to calculate the constant number GCM_IV_RANDOM_BIRTHDAY_BITS, which
 * is the number of bits we subtract off of the length of the iv (in bits) to
 * get a safe count value (log2).
 *
 * Since we do the calculation in bits, so we need to take the whole
 * equation log2:
 *       log2 n = (1+(log2 m)+(log2 p))/2
 * Since p < 1, log2 p is negative. Also note that the length of the iv in
 * bits is log2 m, so if we set GCMIV_RANDOM_BIRTHDAY_BITS =- log2 p - 1.
 * then we can calculate a safe counter value with:
 *        n = 2^((ivLenBits - GCMIV_RANDOM_BIRTHDAY_BITS)/2)
 *
 * If we arbitrarily set p = 10^-18 (1 chance in trillion trillion operation)
 * we get GCMIV_RANDOM_BIRTHDAY_BITS = -(-18)/.301 -1 = 59 (.301 = log10 2)
 * GCMIV_RANDOM_BIRTHDAY_BITS should be at least 59, call it a round 64. NOTE:
 * the variable IV size for TLS is 64 bits, which explains why it's not safe
 * to use a random value for the nonce in TLS. */
#define GCMIV_RANDOM_BIRTHDAY_BITS 64

/***************************************************************************
** Opaque objects
*/

struct DESContextStr;
struct RC2ContextStr;
struct RC4ContextStr;
struct RC5ContextStr;
struct AESContextStr;
struct CamelliaContextStr;
struct MD2ContextStr;
struct MD5ContextStr;
struct SHA1ContextStr;
struct SHA256ContextStr;
struct SHA512ContextStr;
struct AESKeyWrapContextStr;
struct SEEDContextStr;
struct ChaCha20Poly1305ContextStr;
struct Blake2bContextStr;

typedef struct DESContextStr DESContext;
typedef struct RC2ContextStr RC2Context;
typedef struct RC4ContextStr RC4Context;
typedef struct RC5ContextStr RC5Context;
typedef struct AESContextStr AESContext;
typedef struct CamelliaContextStr CamelliaContext;
typedef struct MD2ContextStr MD2Context;
typedef struct MD5ContextStr MD5Context;
typedef struct SHA1ContextStr SHA1Context;
typedef struct SHA256ContextStr SHA256Context;
/* SHA224Context is really a SHA256ContextStr.  This is not a mistake. */
typedef struct SHA256ContextStr SHA224Context;
typedef struct SHA512ContextStr SHA512Context;
/* SHA384Context is really a SHA512ContextStr.  This is not a mistake. */
typedef struct SHA512ContextStr SHA384Context;
typedef struct AESKeyWrapContextStr AESKeyWrapContext;
typedef struct SEEDContextStr SEEDContext;
typedef struct ChaCha20Poly1305ContextStr ChaCha20Poly1305Context;
typedef struct Blake2bContextStr BLAKE2BContext;

/***************************************************************************
** RSA Public and Private Key structures
*/

/* member names from PKCS#1, section 7.1 */
struct RSAPublicKeyStr {
    PLArenaPool *arena;
    SECItem modulus;
    SECItem publicExponent;
};
typedef struct RSAPublicKeyStr RSAPublicKey;

/* member names from PKCS#1, section 7.2 */
struct RSAPrivateKeyStr {
    PLArenaPool *arena;
    SECItem version;
    SECItem modulus;
    SECItem publicExponent;
    SECItem privateExponent;
    SECItem prime1;
    SECItem prime2;
    SECItem exponent1;
    SECItem exponent2;
    SECItem coefficient;
};
typedef struct RSAPrivateKeyStr RSAPrivateKey;

/***************************************************************************
** DSA Public and Private Key and related structures
*/

struct PQGParamsStr {
    PLArenaPool *arena;
    SECItem prime;    /* p */
    SECItem subPrime; /* q */
    SECItem base;     /* g */
    /* XXX chrisk: this needs to be expanded to hold j and validationParms (RFC2459 7.3.2) */
};
typedef struct PQGParamsStr PQGParams;

struct PQGVerifyStr {
    PLArenaPool *arena; /* includes this struct, seed, & h. */
    unsigned int counter;
    SECItem seed;
    SECItem h;
};
typedef struct PQGVerifyStr PQGVerify;

struct DSAPublicKeyStr {
    PQGParams params;
    SECItem publicValue;
};
typedef struct DSAPublicKeyStr DSAPublicKey;

struct DSAPrivateKeyStr {
    PQGParams params;
    SECItem publicValue;
    SECItem privateValue;
};
typedef struct DSAPrivateKeyStr DSAPrivateKey;

/***************************************************************************
** Diffie-Hellman Public and Private Key and related structures
** Structure member names suggested by PKCS#3.
*/

struct DHParamsStr {
    PLArenaPool *arena;
    SECItem prime; /* p */
    SECItem base;  /* g */
};
typedef struct DHParamsStr DHParams;

struct DHPublicKeyStr {
    PLArenaPool *arena;
    SECItem prime;
    SECItem base;
    SECItem publicValue;
};
typedef struct DHPublicKeyStr DHPublicKey;

struct DHPrivateKeyStr {
    PLArenaPool *arena;
    SECItem prime;
    SECItem base;
    SECItem publicValue;
    SECItem privateValue;
};
typedef struct DHPrivateKeyStr DHPrivateKey;

/***************************************************************************
** Data structures used for elliptic curve parameters and
** public and private keys.
*/

/*
** The ECParams data structures can encode elliptic curve
** parameters for both GFp and GF2m curves.
*/

typedef enum { ec_params_explicit,
               ec_params_named
} ECParamsType;

typedef enum { ec_field_GFp = 1,
               ec_field_GF2m,
               ec_field_plain
} ECFieldType;

struct ECFieldIDStr {
    int size; /* field size in bits */
    ECFieldType type;
    union {
        SECItem prime; /* prime p for (GFp) */
        SECItem poly;  /* irreducible binary polynomial for (GF2m) */
    } u;
    int k1; /* first coefficient of pentanomial or
                         * the only coefficient of trinomial
                         */
    int k2; /* two remaining coefficients of pentanomial */
    int k3;
};
typedef struct ECFieldIDStr ECFieldID;

struct ECCurveStr {
    SECItem a; /* contains octet stream encoding of
                         * field element (X9.62 section 4.3.3)
             */
    SECItem b;
    SECItem seed;
};
typedef struct ECCurveStr ECCurve;

struct ECParamsStr {
    PLArenaPool *arena;
    ECParamsType type;
    ECFieldID fieldID;
    ECCurve curve;
    SECItem base;
    SECItem order;
    int cofactor;
    SECItem DEREncoding;
    ECCurveName name;
    SECItem curveOID;
};
typedef struct ECParamsStr ECParams;

struct ECPublicKeyStr {
    ECParams ecParams;
    SECItem publicValue; /* elliptic curve point encoded as
                * octet stream.
                */
};
typedef struct ECPublicKeyStr ECPublicKey;

struct ECPrivateKeyStr {
    ECParams ecParams;
    SECItem publicValue;  /* encoded ec point */
    SECItem privateValue; /* private big integer */
    SECItem version;      /* As per SEC 1, Appendix C, Section C.4 */
};
typedef struct ECPrivateKeyStr ECPrivateKey;

typedef void *(*BLapiAllocateFunc)(void);
typedef void (*BLapiDestroyContextFunc)(void *cx, PRBool freeit);
typedef SECStatus (*BLapiInitContextFunc)(void *cx,
                                          const unsigned char *key,
                                          unsigned int keylen,
                                          const unsigned char *,
                                          int,
                                          unsigned int,
                                          unsigned int);
typedef SECStatus (*BLapiEncrypt)(void *cx, unsigned char *output,
                                  unsigned int *outputLen,
                                  unsigned int maxOutputLen,
                                  const unsigned char *input,
                                  unsigned int inputLen);

#endif /* _BLAPIT_H_ */
