/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
#ifndef _LOWKEYTI_H_
#define _LOWKEYTI_H_ 1

#include "blapit.h"
#include "prtypes.h"
#include "plarena.h"
#include "secitem.h"
#include "secasn1t.h"
#include "secoidt.h"

/*
** Typedef for callback to get a password "key".
*/
extern const SEC_ASN1Template nsslowkey_PQGParamsTemplate[];
extern const SEC_ASN1Template nsslowkey_RSAPrivateKeyTemplate[];
extern const SEC_ASN1Template nsslowkey_DSAPrivateKeyTemplate[];
extern const SEC_ASN1Template nsslowkey_DSAPrivateKeyExportTemplate[];
extern const SEC_ASN1Template nsslowkey_DHPrivateKeyTemplate[];
extern const SEC_ASN1Template nsslowkey_DHPrivateKeyExportTemplate[];
#define NSSLOWKEY_EC_PRIVATE_KEY_VERSION 1 /* as per SECG 1 C.4 */
extern const SEC_ASN1Template nsslowkey_ECPrivateKeyTemplate[];

extern const SEC_ASN1Template nsslowkey_PrivateKeyInfoTemplate[];
extern const SEC_ASN1Template nsslowkey_EncryptedPrivateKeyInfoTemplate[];
extern const SEC_ASN1Template nsslowkey_SubjectPublicKeyInfoTemplate[];
extern const SEC_ASN1Template nsslowkey_RSAPublicKeyTemplate[];

/*
 * PKCS #8 attributes
 */
struct NSSLOWKEYAttributeStr {
    SECItem attrType;
    SECItem *attrValue;
};
typedef struct NSSLOWKEYAttributeStr NSSLOWKEYAttribute;

/*
** A PKCS#8 private key info object
*/
struct NSSLOWKEYPrivateKeyInfoStr {
    PLArenaPool *arena;
    SECItem version;
    SECAlgorithmID algorithm;
    SECItem privateKey;
    NSSLOWKEYAttribute **attributes;
};
typedef struct NSSLOWKEYPrivateKeyInfoStr NSSLOWKEYPrivateKeyInfo;
#define NSSLOWKEY_PRIVATE_KEY_INFO_VERSION 0 /* what we *create* */

struct NSSLOWKEYSubjectPublicKeyInfoStr {
    PLArenaPool *arena;
    SECAlgorithmID algorithm;
    SECItem subjectPublicKey;
};
typedef struct NSSLOWKEYSubjectPublicKeyInfoStr NSSLOWKEYSubjectPublicKeyInfo;

typedef enum {
    NSSLOWKEYNullKey = 0,
    NSSLOWKEYRSAKey = 1,
    NSSLOWKEYDSAKey = 2,
    NSSLOWKEYDHKey = 4,
    NSSLOWKEYECKey = 5
} NSSLOWKEYType;

/*
** An RSA public key object.
*/
struct NSSLOWKEYPublicKeyStr {
    PLArenaPool *arena;
    NSSLOWKEYType keyType;
    union {
        RSAPublicKey rsa;
        DSAPublicKey dsa;
        DHPublicKey dh;
        ECPublicKey ec;
    } u;
};
typedef struct NSSLOWKEYPublicKeyStr NSSLOWKEYPublicKey;

/*
** Low Level private key object
** This is only used by the raw Crypto engines (crypto), keydb (keydb),
** and PKCS #11. Everyone else uses the high level key structure.
*/
struct NSSLOWKEYPrivateKeyStr {
    PLArenaPool *arena;
    NSSLOWKEYType keyType;
    union {
        RSAPrivateKey rsa;
        DSAPrivateKey dsa;
        DHPrivateKey dh;
        ECPrivateKey ec;
    } u;
};
typedef struct NSSLOWKEYPrivateKeyStr NSSLOWKEYPrivateKey;

#endif /* _LOWKEYTI_H_ */
