/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef _P12T_H_
#define _P12T_H_

#include "secoid.h"
#include "keythi.h"
#include "pkcs11.h"
#include "secpkcs7.h"
#include "secdig.h" /* for SGNDigestInfo */
#include "pkcs12t.h"

#define SEC_PKCS12_VERSION 3

/* structure declarations */
typedef struct sec_PKCS12PFXItemStr sec_PKCS12PFXItem;
typedef struct sec_PKCS12MacDataStr sec_PKCS12MacData;
typedef struct sec_PKCS12AuthenticatedSafeStr sec_PKCS12AuthenticatedSafe;
typedef struct sec_PKCS12SafeContentsStr sec_PKCS12SafeContents;
typedef struct sec_PKCS12SafeBagStr sec_PKCS12SafeBag;
typedef struct sec_PKCS12PKCS8ShroudedKeyBagStr sec_PKCS12PKCS8ShroudedKeyBag;
typedef struct sec_PKCS12CertBagStr sec_PKCS12CertBag;
typedef struct sec_PKCS12CRLBagStr sec_PKCS12CRLBag;
typedef struct sec_PKCS12SecretBag sec_PKCS12SecretBag;
typedef struct sec_PKCS12AttributeStr sec_PKCS12Attribute;

struct sec_PKCS12CertBagStr {
    /* what type of cert is stored? */
    SECItem bagID;

    /* certificate information */
    union {
        SECItem x509Cert;
        SECItem SDSICert;
    } value;
};

struct sec_PKCS12CRLBagStr {
    /* what type of cert is stored? */
    SECItem bagID;

    /* certificate information */
    union {
        SECItem x509CRL;
    } value;
};

struct sec_PKCS12SecretBag {
    /* what type of secret? */
    SECItem secretType;

    /* secret information.  ssshhhh be vewy vewy quiet. */
    SECItem secretContent;
};

struct sec_PKCS12AttributeStr {
    SECItem attrType;
    SECItem **attrValue;
};

struct sec_PKCS12SafeBagStr {

    /* What type of bag are we using? */
    SECItem safeBagType;

    /* Dependent upon the type of bag being used. */
    union {
        SECKEYPrivateKeyInfo *pkcs8KeyBag;
        SECKEYEncryptedPrivateKeyInfo *pkcs8ShroudedKeyBag;
        sec_PKCS12CertBag *certBag;
        sec_PKCS12CRLBag *crlBag;
        sec_PKCS12SecretBag *secretBag;
        sec_PKCS12SafeContents *safeContents;
        SECItem *unknownBag;
    } safeBagContent;

    sec_PKCS12Attribute **attribs;

    /* used locally */
    SECOidData *bagTypeTag;
    PLArenaPool *arena;
    unsigned int nAttribs;

    /* used for validation/importing */
    PRBool problem, noInstall, validated, hasKey, unused, installed;
    int error;

    PRBool swapUnicodeBytes;
    PK11SlotInfo *slot;
    SECItem *pwitem;
    PRBool oldBagType;
    SECPKCS12TargetTokenCAs tokenCAs;
};

struct sec_PKCS12SafeContentsStr {
    sec_PKCS12SafeBag **safeBags;
    SECItem **encodedSafeBags;

    /* used locally */
    PLArenaPool *arena;
    unsigned int bagCount;
};

struct sec_PKCS12MacDataStr {
    SGNDigestInfo safeMac;
    SECItem macSalt;
    SECItem iter;
};

struct sec_PKCS12PFXItemStr {

    SECItem version;

    /* Content type will either be Data (password integrity mode)
     * or signedData (public-key integrity mode)
     */
    SEC_PKCS7ContentInfo *authSafe;
    SECItem encodedAuthSafe;

    /* Only present in password integrity mode */
    sec_PKCS12MacData macData;
    SECItem encodedMacData;
};

struct sec_PKCS12AuthenticatedSafeStr {
    /* Content type will either be encryptedData (password privacy mode)
     * or envelopedData (public-key privacy mode)
     */
    SEC_PKCS7ContentInfo **safes;
    SECItem **encodedSafes;

    /* used locally */
    unsigned int safeCount;
    SECItem dummySafe;
};

extern const SEC_ASN1Template sec_PKCS12PFXItemTemplate[];
extern const SEC_ASN1Template sec_PKCS12MacDataTemplate[];
extern const SEC_ASN1Template sec_PKCS12AuthenticatedSafeTemplate[];
extern const SEC_ASN1Template sec_PKCS12SafeContentsTemplate[];
extern const SEC_ASN1Template sec_PKCS12SafeContentsDecodeTemplate[];
extern const SEC_ASN1Template sec_PKCS12NestedSafeContentsDecodeTemplate[];
extern const SEC_ASN1Template sec_PKCS12CertBagTemplate[];
extern const SEC_ASN1Template sec_PKCS12CRLBagTemplate[];
extern const SEC_ASN1Template sec_PKCS12SecretBagTemplate[];
extern const SEC_ASN1Template sec_PKCS12PointerToCertBagTemplate[];
extern const SEC_ASN1Template sec_PKCS12PointerToCRLBagTemplate[];
extern const SEC_ASN1Template sec_PKCS12PointerToSecretBagTemplate[];
extern const SEC_ASN1Template sec_PKCS12PointerToSafeContentsTemplate[];
extern const SEC_ASN1Template sec_PKCS12AttributeTemplate[];
extern const SEC_ASN1Template sec_PKCS12PointerToContentInfoTemplate[];
extern const SEC_ASN1Template sec_PKCS12SafeBagTemplate[];

#endif
