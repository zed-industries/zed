/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
#ifndef _KEYTHI_H_
#define _KEYTHI_H_ 1

#include "eccutil.h"
#include "plarena.h"
#include "pkcs11t.h"
#include "secmodt.h"
#include "prclist.h"

/*
** RFC 4055 Section 1.2 specifies three different RSA key types.
**
** rsaKey maps to keys with SEC_OID_PKCS1_RSA_ENCRYPTION and can be used for
** both encryption and signatures with old (PKCS #1 v1.5) and new (PKCS #1
** v2.1) padding schemes.
**
** rsaPssKey maps to keys with SEC_OID_PKCS1_RSA_PSS_SIGNATURE and may only
** be used for signatures with PSS padding (PKCS #1 v2.1).
**
** rsaOaepKey maps to keys with SEC_OID_PKCS1_RSA_OAEP_ENCRYPTION and may only
** be used for encryption with OAEP padding (PKCS #1 v2.1).
*/

typedef enum {
    nullKey = 0,
    rsaKey = 1,
    dsaKey = 2,
    fortezzaKey = 3, /* deprecated */
    dhKey = 4,
    keaKey = 5, /* deprecated */
    ecKey = 6,
    rsaPssKey = 7,
    rsaOaepKey = 8
} KeyType;

/*
** Template Definitions
**/

SEC_BEGIN_PROTOS
extern const SEC_ASN1Template SECKEY_RSAPublicKeyTemplate[];
extern const SEC_ASN1Template SECKEY_RSAPSSParamsTemplate[];
extern const SEC_ASN1Template SECKEY_DSAPublicKeyTemplate[];
extern const SEC_ASN1Template SECKEY_DHPublicKeyTemplate[];
extern const SEC_ASN1Template SECKEY_DHParamKeyTemplate[];
extern const SEC_ASN1Template SECKEY_PQGParamsTemplate[];
extern const SEC_ASN1Template SECKEY_DSAPrivateKeyExportTemplate[];

/* Windows DLL accessor functions */
SEC_ASN1_CHOOSER_DECLARE(SECKEY_DSAPublicKeyTemplate)
SEC_ASN1_CHOOSER_DECLARE(SECKEY_RSAPublicKeyTemplate)
SEC_ASN1_CHOOSER_DECLARE(SECKEY_RSAPSSParamsTemplate)
SEC_END_PROTOS

/*
** RSA Public Key structures
** member names from PKCS#1, section 7.1
*/

struct SECKEYRSAPublicKeyStr {
    PLArenaPool *arena;
    SECItem modulus;
    SECItem publicExponent;
};
typedef struct SECKEYRSAPublicKeyStr SECKEYRSAPublicKey;

/*
** RSA-PSS parameters
*/
struct SECKEYRSAPSSParamsStr {
    SECAlgorithmID *hashAlg;
    SECAlgorithmID *maskAlg;
    SECItem saltLength;
    SECItem trailerField;
};
typedef struct SECKEYRSAPSSParamsStr SECKEYRSAPSSParams;

/*
** DSA Public Key and related structures
*/

struct SECKEYPQGParamsStr {
    PLArenaPool *arena;
    SECItem prime;    /* p */
    SECItem subPrime; /* q */
    SECItem base;     /* g */
    /* XXX chrisk: this needs to be expanded to hold j and validationParms (RFC2459 7.3.2) */
};
typedef struct SECKEYPQGParamsStr SECKEYPQGParams;

struct SECKEYDSAPublicKeyStr {
    SECKEYPQGParams params;
    SECItem publicValue;
};
typedef struct SECKEYDSAPublicKeyStr SECKEYDSAPublicKey;

/*
** Diffie-Hellman Public Key structure
** Structure member names suggested by PKCS#3.
*/
struct SECKEYDHParamsStr {
    PLArenaPool *arena;
    SECItem prime; /* p */
    SECItem base;  /* g */
};
typedef struct SECKEYDHParamsStr SECKEYDHParams;

struct SECKEYDHPublicKeyStr {
    PLArenaPool *arena;
    SECItem prime;
    SECItem base;
    SECItem publicValue;
};
typedef struct SECKEYDHPublicKeyStr SECKEYDHPublicKey;

/*
** Elliptic curve Public Key structure
** The PKCS#11 layer needs DER encoding of ANSI X9.62
** parameters value
*/
typedef SECItem SECKEYECParams;

struct SECKEYECPublicKeyStr {
    SECKEYECParams DEREncodedParams;
    int size;                 /* size in bits */
    SECItem publicValue;      /* encoded point */
    ECPointEncoding encoding; /* deprecated, ignored */
};
typedef struct SECKEYECPublicKeyStr SECKEYECPublicKey;

/*
** FORTEZZA Public Key structures
*/
struct SECKEYFortezzaPublicKeyStr {
    int KEAversion;
    int DSSversion;
    unsigned char KMID[8];
    SECItem clearance;
    SECItem KEApriviledge;
    SECItem DSSpriviledge;
    SECItem KEAKey;
    SECItem DSSKey;
    SECKEYPQGParams params;
    SECKEYPQGParams keaParams;
};
typedef struct SECKEYFortezzaPublicKeyStr SECKEYFortezzaPublicKey;
#define KEAprivilege KEApriviledge /* corrected spelling */
#define DSSprivilege DSSpriviledge /* corrected spelling */

struct SECKEYDiffPQGParamsStr {
    SECKEYPQGParams DiffKEAParams;
    SECKEYPQGParams DiffDSAParams;
};
typedef struct SECKEYDiffPQGParamsStr SECKEYDiffPQGParams;

struct SECKEYPQGDualParamsStr {
    SECKEYPQGParams CommParams;
    SECKEYDiffPQGParams DiffParams;
};
typedef struct SECKEYPQGDualParamsStr SECKEYPQGDualParams;

struct SECKEYKEAParamsStr {
    PLArenaPool *arena;
    SECItem hash;
};
typedef struct SECKEYKEAParamsStr SECKEYKEAParams;

struct SECKEYKEAPublicKeyStr {
    SECKEYKEAParams params;
    SECItem publicValue;
};
typedef struct SECKEYKEAPublicKeyStr SECKEYKEAPublicKey;

/*
** A Generic  public key object.
*/
struct SECKEYPublicKeyStr {
    PLArenaPool *arena;
    KeyType keyType;
    PK11SlotInfo *pkcs11Slot;
    CK_OBJECT_HANDLE pkcs11ID;
    union {
        SECKEYRSAPublicKey rsa;
        SECKEYDSAPublicKey dsa;
        SECKEYDHPublicKey dh;
        SECKEYKEAPublicKey kea;
        SECKEYFortezzaPublicKey fortezza;
        SECKEYECPublicKey ec;
    } u;
};
typedef struct SECKEYPublicKeyStr SECKEYPublicKey;

/* bit flag definitions for staticflags */
#define SECKEY_Attributes_Cached 0x1 /* bit 0 states \
                                        whether attributes are cached */
#define SECKEY_CKA_PRIVATE (1U << 1) /* bit 1 is the value of CKA_PRIVATE */
#define SECKEY_CKA_ALWAYS_AUTHENTICATE (1U << 2)

#define SECKEY_ATTRIBUTES_CACHED(key) \
    (0 != (key->staticflags & SECKEY_Attributes_Cached))

#define SECKEY_ATTRIBUTE_VALUE(key, attribute) \
    (0 != (key->staticflags & SECKEY_##attribute))

#define SECKEY_HAS_ATTRIBUTE_SET(key, attribute) \
    (0 != (key->staticflags & SECKEY_Attributes_Cached)) ? (0 != (key->staticflags & SECKEY_##attribute)) : PK11_HasAttributeSet(key->pkcs11Slot, key->pkcs11ID, attribute, PR_FALSE)

#define SECKEY_HAS_ATTRIBUTE_SET_LOCK(key, attribute, haslock) \
    (0 != (key->staticflags & SECKEY_Attributes_Cached)) ? (0 != (key->staticflags & SECKEY_##attribute)) : pk11_HasAttributeSet_Lock(key->pkcs11Slot, key->pkcs11ID, attribute, haslock)

/*
** A generic key structure
*/
struct SECKEYPrivateKeyStr {
    PLArenaPool *arena;
    KeyType keyType;
    PK11SlotInfo *pkcs11Slot;  /* pkcs11 slot this key lives in */
    CK_OBJECT_HANDLE pkcs11ID; /* ID of pkcs11 object */
    PRBool pkcs11IsTemp;       /* temp pkcs11 object, delete it when done */
    void *wincx;               /* context for errors and pw prompts */
    PRUint32 staticflags;      /* bit flag of cached PKCS#11 attributes */
};
typedef struct SECKEYPrivateKeyStr SECKEYPrivateKey;

typedef struct {
    PRCList links;
    SECKEYPrivateKey *key;
} SECKEYPrivateKeyListNode;

typedef struct {
    PRCList list;
    PLArenaPool *arena;
} SECKEYPrivateKeyList;

typedef struct {
    PRCList links;
    SECKEYPublicKey *key;
} SECKEYPublicKeyListNode;

typedef struct {
    PRCList list;
    PLArenaPool *arena;
} SECKEYPublicKeyList;
#endif /* _KEYTHI_H_ */
