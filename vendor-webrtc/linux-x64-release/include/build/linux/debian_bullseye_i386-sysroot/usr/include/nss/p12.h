/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef _P12_H_
#define _P12_H_

#include "secoid.h"
#include "keyhi.h"
#include "secpkcs7.h"
#include "p12t.h"

typedef int(PR_CALLBACK *PKCS12OpenFunction)(void *arg);
typedef int(PR_CALLBACK *PKCS12ReadFunction)(void *arg,
                                             unsigned char *buffer,
                                             unsigned int *lenRead,
                                             unsigned int maxLen);
typedef int(PR_CALLBACK *PKCS12WriteFunction)(void *arg,
                                              unsigned char *buffer,
                                              unsigned int *bufLen,
                                              unsigned int *lenWritten);
typedef int(PR_CALLBACK *PKCS12CloseFunction)(void *arg);
typedef SECStatus(PR_CALLBACK *PKCS12UnicodeConvertFunction)(
    PLArenaPool *arena,
    SECItem *dest, SECItem *src,
    PRBool toUnicode,
    PRBool swapBytes);
typedef void(PR_CALLBACK *SEC_PKCS12EncoderOutputCallback)(
    void *arg, const char *buf,
    unsigned long len);
typedef void(PR_CALLBACK *SEC_PKCS12DecoderOutputCallback)(
    void *arg, const char *buf,
    unsigned long len);
/*
 * In NSS 3.12 or later, 'arg' actually points to a CERTCertificate,
 * the 'leafCert' variable in sec_pkcs12_validate_cert in p12d.c.
 * See r1.35 of p12d.c ("Patch 2" in bug 321584).
 *
 * This callback might be called by SEC_PKCS12DecoderValidateBags each time
 * a nickname collission is detected. The callback must return a new
 * nickname. The returned SECItem should be of type siAsciiString,
 * it should be allocated using:
 *     SECITEM_AllocItem(NULL, NULL, LENGTH_OF_NEW_NICKNAME + 1)
 * and data must contain the new nickname as a zero terminated string.
 */
typedef SECItem *(PR_CALLBACK *SEC_PKCS12NicknameCollisionCallback)(
    SECItem *old_nickname,
    PRBool *cancel,
    void *arg);
/*
 * This callback is called by SEC_PKCS12DecoderRenameCertNicknames for each
 * certificate found in the p12 source data.
 *
 * cert: A decoded certificate.
 * default_nickname: The nickname as found in the source data.
 *                   Will be NULL if source data doesn't have nickname.
 * new_nickname: Output parameter that may contain the renamed nickname.
 * arg: The user data that was passed to SEC_PKCS12DecoderRenameCertNicknames.
 *
 * If the callback accept that NSS will use a nickname based on the
 * default_nickname (potentially resolving conflicts), then the callback
 * must set *new_nickname to NULL.
 *
 * If the callback wishes to override the nickname, it must set *new_nickname
 * to a new SECItem which should be allocated using
 *     SECITEM_AllocItem(NULL, NULL, LENGTH_OF_NEW_NICKNAME + 1)
 * new_nickname->type should be set to siAsciiString, and new_nickname->data
 * must contain the new nickname as a zero terminated string.
 *
 * A return value of SECFailure indicates that the renaming operation failed,
 * and callback should release new_nickname before returning if it's already
 * being allocated.
 * Otherwise, the callback function must return SECSuccess, including use
 * default nickname as mentioned above.
 */
typedef SECStatus(PR_CALLBACK *SEC_PKCS12NicknameRenameCallback)(
    const CERTCertificate *cert,
    const SECItem *default_nickname,
    SECItem **new_nickname,
    void *arg);

typedef SECStatus(PR_CALLBACK *digestOpenFn)(void *arg, PRBool readData);
typedef SECStatus(PR_CALLBACK *digestCloseFn)(void *arg, PRBool removeFile);
typedef int(PR_CALLBACK *digestIOFn)(void *arg, unsigned char *buf,
                                     unsigned long len);

typedef struct SEC_PKCS12ExportContextStr SEC_PKCS12ExportContext;
typedef struct SEC_PKCS12SafeInfoStr SEC_PKCS12SafeInfo;
typedef struct SEC_PKCS12DecoderContextStr SEC_PKCS12DecoderContext;
typedef struct SEC_PKCS12DecoderItemStr SEC_PKCS12DecoderItem;

struct sec_PKCS12PasswordModeInfo {
    SECItem *password;
    SECOidTag algorithm;
};

struct sec_PKCS12PublicKeyModeInfo {
    CERTCertificate *cert;
    CERTCertDBHandle *certDb;
    SECOidTag algorithm;
    int keySize;
};

struct SEC_PKCS12DecoderItemStr {
    SECItem *der;
    SECOidTag type;
    PRBool hasKey;
    SECItem *friendlyName; /* UTF-8 string */
    SECAlgorithmID *shroudAlg;
};

SEC_BEGIN_PROTOS

SEC_PKCS12SafeInfo *
SEC_PKCS12CreatePubKeyEncryptedSafe(SEC_PKCS12ExportContext *p12ctxt,
                                    CERTCertDBHandle *certDb,
                                    CERTCertificate *signer,
                                    CERTCertificate **recipients,
                                    SECOidTag algorithm, int keysize);

extern SEC_PKCS12SafeInfo *
SEC_PKCS12CreatePasswordPrivSafe(SEC_PKCS12ExportContext *p12ctxt,
                                 SECItem *pwitem, SECOidTag privAlg);

extern SEC_PKCS12SafeInfo *
SEC_PKCS12CreateUnencryptedSafe(SEC_PKCS12ExportContext *p12ctxt);

extern SECStatus
SEC_PKCS12AddPasswordIntegrity(SEC_PKCS12ExportContext *p12ctxt,
                               SECItem *pwitem, SECOidTag integAlg);
extern SECStatus
SEC_PKCS12AddPublicKeyIntegrity(SEC_PKCS12ExportContext *p12ctxt,
                                CERTCertificate *cert, CERTCertDBHandle *certDb,
                                SECOidTag algorithm, int keySize);

extern SEC_PKCS12ExportContext *
SEC_PKCS12CreateExportContext(SECKEYGetPasswordKey pwfn, void *pwfnarg,
                              PK11SlotInfo *slot, void *wincx);

extern SECStatus
SEC_PKCS12AddCert(SEC_PKCS12ExportContext *p12ctxt,
                  SEC_PKCS12SafeInfo *safe, void *nestedDest,
                  CERTCertificate *cert, CERTCertDBHandle *certDb,
                  SECItem *keyId, PRBool includeCertChain);

extern SECStatus
SEC_PKCS12AddKeyForCert(SEC_PKCS12ExportContext *p12ctxt,
                        SEC_PKCS12SafeInfo *safe,
                        void *nestedDest, CERTCertificate *cert,
                        PRBool shroudKey, SECOidTag algorithm, SECItem *pwitem,
                        SECItem *keyId, SECItem *nickName);

extern SECStatus
SEC_PKCS12AddCertOrChainAndKey(SEC_PKCS12ExportContext *p12ctxt,
                               void *certSafe, void *certNestedDest,
                               CERTCertificate *cert, CERTCertDBHandle *certDb,
                               void *keySafe, void *keyNestedDest, PRBool shroudKey,
                               SECItem *pwitem, SECOidTag algorithm,
                               PRBool includeCertChain);

extern SECStatus
SEC_PKCS12AddCertAndKey(SEC_PKCS12ExportContext *p12ctxt,
                        void *certSafe, void *certNestedDest,
                        CERTCertificate *cert, CERTCertDBHandle *certDb,
                        void *keySafe, void *keyNestedDest,
                        PRBool shroudKey, SECItem *pwitem, SECOidTag algorithm);

extern void *
SEC_PKCS12CreateNestedSafeContents(SEC_PKCS12ExportContext *p12ctxt,
                                   void *baseSafe, void *nestedDest);

extern SECStatus
SEC_PKCS12Encode(SEC_PKCS12ExportContext *p12exp,
                 SEC_PKCS12EncoderOutputCallback output, void *outputarg);

extern void
SEC_PKCS12DestroyExportContext(SEC_PKCS12ExportContext *p12exp);

extern SEC_PKCS12DecoderContext *
SEC_PKCS12DecoderStart(SECItem *pwitem, PK11SlotInfo *slot, void *wincx,
                       digestOpenFn dOpen, digestCloseFn dClose,
                       digestIOFn dRead, digestIOFn dWrite, void *dArg);

extern SECStatus
SEC_PKCS12DecoderSetTargetTokenCAs(SEC_PKCS12DecoderContext *p12dcx,
                                   SECPKCS12TargetTokenCAs tokenCAs);

extern SECStatus
SEC_PKCS12DecoderUpdate(SEC_PKCS12DecoderContext *p12dcx, unsigned char *data,
                        unsigned long len);

extern void
SEC_PKCS12DecoderFinish(SEC_PKCS12DecoderContext *p12dcx);

extern SECStatus
SEC_PKCS12DecoderVerify(SEC_PKCS12DecoderContext *p12dcx);

extern SECStatus
SEC_PKCS12DecoderValidateBags(SEC_PKCS12DecoderContext *p12dcx,
                              SEC_PKCS12NicknameCollisionCallback nicknameCb);

/*
 * SEC_PKCS12DecoderRenameCertNicknames() can be used to change
 * certificate nicknames in SEC_PKCS12DecoderContext, prior to calling
 * SEC_PKCS12DecoderImportBags.
 *
 * arg: User-defined data that will be passed to nicknameCb.
 *
 * If SEC_PKCS12DecoderRenameCertNicknames() is called after calling
 * SEC_PKCS12DecoderValidateBags(), then only the certificate nickname
 * will be changed.
 * If SEC_PKCS12DecoderRenameCertNicknames() is called prior to calling
 * SEC_PKCS12DecoderValidateBags(), then SEC_PKCS12DecoderValidateBags()
 * will change the nickname of the corresponding private key, too.
 */
extern SECStatus
SEC_PKCS12DecoderRenameCertNicknames(SEC_PKCS12DecoderContext *p12dcx,
                                     SEC_PKCS12NicknameRenameCallback nicknameCb,
                                     void *arg);

extern SECStatus
SEC_PKCS12DecoderImportBags(SEC_PKCS12DecoderContext *p12dcx);

CERTCertList *
SEC_PKCS12DecoderGetCerts(SEC_PKCS12DecoderContext *p12dcx);

SECStatus
SEC_PKCS12DecoderIterateInit(SEC_PKCS12DecoderContext *p12dcx);

SECStatus
SEC_PKCS12DecoderIterateNext(SEC_PKCS12DecoderContext *p12dcx,
                             const SEC_PKCS12DecoderItem **ipp);

SEC_END_PROTOS

#endif
