/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/*
 * Header for pkcs7 types.
 */

#ifndef _PKCS7T_H_
#define _PKCS7T_H_

#include "plarena.h"

#include "seccomon.h"
#include "secoidt.h"
#include "certt.h"
#include "secmodt.h"

/* Opaque objects */
typedef struct SEC_PKCS7DecoderContextStr SEC_PKCS7DecoderContext;
typedef struct SEC_PKCS7EncoderContextStr SEC_PKCS7EncoderContext;

/* legacy defines that haven't been active for years */
typedef void *(*SECKEYGetPasswordKey)(void *arg, void *handle);

/* Non-opaque objects.  NOTE, though: I want them to be treated as
 * opaque as much as possible.  If I could hide them completely,
 * I would.  (I tried, but ran into trouble that was taking me too
 * much time to get out of.)  I still intend to try to do so.
 * In fact, the only type that "outsiders" should even *name* is
 * SEC_PKCS7ContentInfo, and they should not reference its fields.
 */
/* rjr: PKCS #11 cert handling (pk11cert.c) does use SEC_PKCS7RecipientInfo's.
 * This is because when we search the recipient list for the cert and key we
 * want, we need to invert the order of the loops we used to have. The old
 * loops were:
 *
 *  For each recipient {
 *       find_cert = PK11_Find_AllCert(recipient->issuerSN);
 *       [which unrolls to... ]
 *       For each slot {
 *            Log into slot;
 *            search slot for cert;
 *      }
 *  }
 *
 *  the new loop searchs all the recipients at once on a slot. this allows
 *  PKCS #11 to order slots in such a way that logout slots don't get checked
 *  if we can find the cert on a logged in slot. This eliminates lots of
 *  spurious password prompts when smart cards are installed... so why this
 *  comment? If you make SEC_PKCS7RecipientInfo completely opaque, you need
 *  to provide a non-opaque list of issuerSN's (the only field PKCS#11 needs
 *  and fix up pk11cert.c first. NOTE: Only S/MIME calls this special PKCS #11
 *  function.
 */
typedef struct SEC_PKCS7ContentInfoStr SEC_PKCS7ContentInfo;
typedef struct SEC_PKCS7SignedDataStr SEC_PKCS7SignedData;
typedef struct SEC_PKCS7EncryptedContentInfoStr SEC_PKCS7EncryptedContentInfo;
typedef struct SEC_PKCS7EnvelopedDataStr SEC_PKCS7EnvelopedData;
typedef struct SEC_PKCS7SignedAndEnvelopedDataStr
    SEC_PKCS7SignedAndEnvelopedData;
typedef struct SEC_PKCS7SignerInfoStr SEC_PKCS7SignerInfo;
typedef struct SEC_PKCS7RecipientInfoStr SEC_PKCS7RecipientInfo;
typedef struct SEC_PKCS7DigestedDataStr SEC_PKCS7DigestedData;
typedef struct SEC_PKCS7EncryptedDataStr SEC_PKCS7EncryptedData;
/*
 * The following is not actually a PKCS7 type, but for now it is only
 * used by PKCS7, so we have adopted it.  If someone else *ever* needs
 * it, its name should be changed and it should be moved out of here.
 * Do not dare to use it without doing so!
 */
typedef struct SEC_PKCS7AttributeStr SEC_PKCS7Attribute;

struct SEC_PKCS7ContentInfoStr {
    PLArenaPool *poolp;         /* local; not part of encoding */
    PRBool created;             /* local; not part of encoding */
    int refCount;               /* local; not part of encoding */
    SECOidData *contentTypeTag; /* local; not part of encoding */
    SECKEYGetPasswordKey pwfn;  /* local; not part of encoding */
    void *pwfn_arg;             /* local; not part of encoding */
    SECItem contentType;
    union {
        SECItem *data;
        SEC_PKCS7DigestedData *digestedData;
        SEC_PKCS7EncryptedData *encryptedData;
        SEC_PKCS7EnvelopedData *envelopedData;
        SEC_PKCS7SignedData *signedData;
        SEC_PKCS7SignedAndEnvelopedData *signedAndEnvelopedData;
    } content;
};

struct SEC_PKCS7SignedDataStr {
    SECItem version;
    SECAlgorithmID **digestAlgorithms;
    SEC_PKCS7ContentInfo contentInfo;
    SECItem **rawCerts;
    CERTSignedCrl **crls;
    SEC_PKCS7SignerInfo **signerInfos;
    SECItem **digests;               /* local; not part of encoding */
    CERTCertificate **certs;         /* local; not part of encoding */
    CERTCertificateList **certLists; /* local; not part of encoding */
};
#define SEC_PKCS7_SIGNED_DATA_VERSION 1 /* what we *create* */

struct SEC_PKCS7EncryptedContentInfoStr {
    SECOidData *contentTypeTag; /* local; not part of encoding */
    SECItem contentType;
    SECAlgorithmID contentEncAlg;
    SECItem encContent;
    SECItem plainContent; /* local; not part of encoding */
                          /* bytes not encrypted, but encoded */
    int keysize;          /* local; not part of encoding */
                          /* size of bulk encryption key
                           * (only used by creation code) */
    SECOidTag encalg;     /* local; not part of encoding */
                          /* oid tag of encryption algorithm
                           * (only used by creation code) */
};

struct SEC_PKCS7EnvelopedDataStr {
    SECItem version;
    SEC_PKCS7RecipientInfo **recipientInfos;
    SEC_PKCS7EncryptedContentInfo encContentInfo;
};
#define SEC_PKCS7_ENVELOPED_DATA_VERSION 0 /* what we *create* */

struct SEC_PKCS7SignedAndEnvelopedDataStr {
    SECItem version;
    SEC_PKCS7RecipientInfo **recipientInfos;
    SECAlgorithmID **digestAlgorithms;
    SEC_PKCS7EncryptedContentInfo encContentInfo;
    SECItem **rawCerts;
    CERTSignedCrl **crls;
    SEC_PKCS7SignerInfo **signerInfos;
    SECItem **digests;               /* local; not part of encoding */
    CERTCertificate **certs;         /* local; not part of encoding */
    CERTCertificateList **certLists; /* local; not part of encoding */
    PK11SymKey *sigKey;              /* local; not part of encoding */
};
#define SEC_PKCS7_SIGNED_AND_ENVELOPED_DATA_VERSION 1 /* what we *create* */

struct SEC_PKCS7SignerInfoStr {
    SECItem version;
    CERTIssuerAndSN *issuerAndSN;
    SECAlgorithmID digestAlg;
    SEC_PKCS7Attribute **authAttr;
    SECAlgorithmID digestEncAlg;
    SECItem encDigest;
    SEC_PKCS7Attribute **unAuthAttr;
    CERTCertificate *cert;         /* local; not part of encoding */
    CERTCertificateList *certList; /* local; not part of encoding */
};
#define SEC_PKCS7_SIGNER_INFO_VERSION 1 /* what we *create* */

struct SEC_PKCS7RecipientInfoStr {
    SECItem version;
    CERTIssuerAndSN *issuerAndSN;
    SECAlgorithmID keyEncAlg;
    SECItem encKey;
    CERTCertificate *cert; /* local; not part of encoding */
};
#define SEC_PKCS7_RECIPIENT_INFO_VERSION 0 /* what we *create* */

struct SEC_PKCS7DigestedDataStr {
    SECItem version;
    SECAlgorithmID digestAlg;
    SEC_PKCS7ContentInfo contentInfo;
    SECItem digest;
};
#define SEC_PKCS7_DIGESTED_DATA_VERSION 0 /* what we *create* */

struct SEC_PKCS7EncryptedDataStr {
    SECItem version;
    SEC_PKCS7EncryptedContentInfo encContentInfo;
};
#define SEC_PKCS7_ENCRYPTED_DATA_VERSION 0 /* what we *create* */

/*
 * See comment above about this type not really belonging to PKCS7.
 */
struct SEC_PKCS7AttributeStr {
    /* The following fields make up an encoded Attribute: */
    SECItem type;
    SECItem **values; /* data may or may not be encoded */
    /* The following fields are not part of an encoded Attribute: */
    SECOidData *typeTag;
    PRBool encoded; /* when true, values are encoded */
};

/*
 * Type of function passed to SEC_PKCS7Decode or SEC_PKCS7DecoderStart.
 * If specified, this is where the content bytes (only) will be "sent"
 * as they are recovered during the decoding.
 *
 * XXX Should just combine this with SEC_PKCS7EncoderContentCallback type
 * and use a simpler, common name.
 */
typedef void (*SEC_PKCS7DecoderContentCallback)(void *arg,
                                                const char *buf,
                                                unsigned long len);

/*
 * Type of function passed to SEC_PKCS7Encode or SEC_PKCS7EncoderStart.
 * This is where the encoded bytes will be "sent".
 *
 * XXX Should just combine this with SEC_PKCS7DecoderContentCallback type
 * and use a simpler, common name.
 */
typedef void (*SEC_PKCS7EncoderOutputCallback)(void *arg,
                                               const char *buf,
                                               unsigned long len);

/*
 * Type of function passed to SEC_PKCS7Decode or SEC_PKCS7DecoderStart
 * to retrieve the decryption key.  This function is inteded to be
 * used for EncryptedData content info's which do not have a key available
 * in a certificate, etc.
 */
typedef PK11SymKey *(*SEC_PKCS7GetDecryptKeyCallback)(void *arg,
                                                      SECAlgorithmID *algid);

/*
 * Type of function passed to SEC_PKCS7Decode or SEC_PKCS7DecoderStart.
 * This function in intended to be used to verify that decrypting a
 * particular crypto algorithm is allowed.  Content types which do not
 * require decryption will not need the callback.  If the callback
 * is not specified for content types which require decryption, the
 * decryption will be disallowed.
 */
typedef PRBool (*SEC_PKCS7DecryptionAllowedCallback)(SECAlgorithmID *algid,
                                                     PK11SymKey *bulkkey);

#endif /* _PKCS7T_H_ */
