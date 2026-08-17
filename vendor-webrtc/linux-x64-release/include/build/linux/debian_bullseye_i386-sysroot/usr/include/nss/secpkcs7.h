/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/*
 * Interface to the PKCS7 implementation.
 */

#ifndef _SECPKCS7_H_
#define _SECPKCS7_H_

#include "seccomon.h"

#include "secoidt.h"
#include "certt.h"
#include "keythi.h"
#include "hasht.h"
#include "pkcs7t.h"

extern const SEC_ASN1Template sec_PKCS7ContentInfoTemplate[];

/************************************************************************/
SEC_BEGIN_PROTOS

/************************************************************************
 *      Miscellaneous
 ************************************************************************/

/*
 * Returns the content type of the given contentInfo.
 */
extern SECOidTag SEC_PKCS7ContentType(SEC_PKCS7ContentInfo *cinfo);

/*
 * Destroy a PKCS7 contentInfo and all of its sub-pieces.
 */
extern void SEC_PKCS7DestroyContentInfo(SEC_PKCS7ContentInfo *contentInfo);

/*
 * Copy a PKCS7 contentInfo.  A Destroy is needed on *each* copy.
 */
extern SEC_PKCS7ContentInfo *
SEC_PKCS7CopyContentInfo(SEC_PKCS7ContentInfo *contentInfo);

/*
 * Return a pointer to the actual content.  In the case of those types
 * which are encrypted, this returns the *plain* content.
 */
extern SECItem *SEC_PKCS7GetContent(SEC_PKCS7ContentInfo *cinfo);

/************************************************************************
 *      PKCS7 Decoding, Verification, etc..
 ************************************************************************/

extern SEC_PKCS7DecoderContext *
SEC_PKCS7DecoderStart(SEC_PKCS7DecoderContentCallback callback,
                      void *callback_arg,
                      SECKEYGetPasswordKey pwfn, void *pwfn_arg,
                      SEC_PKCS7GetDecryptKeyCallback decrypt_key_cb,
                      void *decrypt_key_cb_arg,
                      SEC_PKCS7DecryptionAllowedCallback decrypt_allowed_cb);

extern SECStatus
SEC_PKCS7DecoderUpdate(SEC_PKCS7DecoderContext *p7dcx,
                       const char *buf, unsigned long len);

extern SEC_PKCS7ContentInfo *
SEC_PKCS7DecoderFinish(SEC_PKCS7DecoderContext *p7dcx);

/*  Abort the underlying ASN.1 stream & set an error  */
void SEC_PKCS7DecoderAbort(SEC_PKCS7DecoderContext *p7dcx, int error);

extern SEC_PKCS7ContentInfo *
SEC_PKCS7DecodeItem(SECItem *p7item,
                    SEC_PKCS7DecoderContentCallback cb, void *cb_arg,
                    SECKEYGetPasswordKey pwfn, void *pwfn_arg,
                    SEC_PKCS7GetDecryptKeyCallback decrypt_key_cb,
                    void *decrypt_key_cb_arg,
                    SEC_PKCS7DecryptionAllowedCallback decrypt_allowed_cb);

extern PRBool SEC_PKCS7ContainsCertsOrCrls(SEC_PKCS7ContentInfo *cinfo);

/* checks to see if the contents of the content info is
 * empty.  it so, PR_TRUE is returned.  PR_FALSE, otherwise.
 *
 * minLen is used to specify a minimum size.  if content size <= minLen,
 * content is assumed empty.
 */
extern PRBool
SEC_PKCS7IsContentEmpty(SEC_PKCS7ContentInfo *cinfo, unsigned int minLen);

extern PRBool SEC_PKCS7ContentIsEncrypted(SEC_PKCS7ContentInfo *cinfo);

/*
 * If the PKCS7 content has a signature (not just *could* have a signature)
 * return true; false otherwise.  This can/should be called before calling
 * VerifySignature, which will always indicate failure if no signature is
 * present, but that does not mean there even was a signature!
 * Note that the content itself can be empty (detached content was sent
 * another way); it is the presence of the signature that matters.
 */
extern PRBool SEC_PKCS7ContentIsSigned(SEC_PKCS7ContentInfo *cinfo);

/*
 * SEC_PKCS7VerifySignature
 *      Look at a PKCS7 contentInfo and check if the signature is good.
 *      The verification checks that the signing cert is valid and trusted
 *      for the purpose specified by "certusage".
 *
 *      In addition, if "keepcerts" is true, add any new certificates found
 *      into our local database.
 */
extern PRBool SEC_PKCS7VerifySignature(SEC_PKCS7ContentInfo *cinfo,
                                       SECCertUsage certusage,
                                       PRBool keepcerts);

/*
 * SEC_PKCS7VerifyDetachedSignature
 *      Look at a PKCS7 contentInfo and check if the signature matches
 *      a passed-in digest (calculated, supposedly, from detached contents).
 *      The verification checks that the signing cert is valid and trusted
 *      for the purpose specified by "certusage".
 *
 *      In addition, if "keepcerts" is true, add any new certificates found
 *      into our local database.
 */
extern PRBool SEC_PKCS7VerifyDetachedSignature(SEC_PKCS7ContentInfo *cinfo,
                                               SECCertUsage certusage,
                                               const SECItem *detached_digest,
                                               HASH_HashType digest_type,
                                               PRBool keepcerts);

/*
 * SEC_PKCS7VerifyDetachedSignatureAtTime
 *      Look at a PKCS7 contentInfo and check if the signature matches
 *      a passed-in digest (calculated, supposedly, from detached contents).
 *      The verification checks that the signing cert is valid and trusted
 *      for the purpose specified by "certusage" at time "atTime".
 *
 *      In addition, if "keepcerts" is true, add any new certificates found
 *      into our local database.
 */
extern PRBool
SEC_PKCS7VerifyDetachedSignatureAtTime(SEC_PKCS7ContentInfo *cinfo,
                                       SECCertUsage certusage,
                                       const SECItem *detached_digest,
                                       HASH_HashType digest_type,
                                       PRBool keepcerts,
                                       PRTime atTime);

/*
 * SEC_PKCS7GetSignerCommonName, SEC_PKCS7GetSignerEmailAddress
 *      The passed-in contentInfo is espected to be Signed, and these
 *      functions return the specified portion of the full signer name.
 *
 *      Returns a pointer to allocated memory, which must be freed.
 *      A NULL return value is an error.
 */
extern char *SEC_PKCS7GetSignerCommonName(SEC_PKCS7ContentInfo *cinfo);
extern char *SEC_PKCS7GetSignerEmailAddress(SEC_PKCS7ContentInfo *cinfo);

/*
 * Return the the signing time, in UTCTime format, of a PKCS7 contentInfo.
 */
extern SECItem *SEC_PKCS7GetSigningTime(SEC_PKCS7ContentInfo *cinfo);

/************************************************************************
 *      PKCS7 Creation and Encoding.
 ************************************************************************/

/*
 * Start a PKCS7 signing context.
 *
 * "cert" is the cert that will be used to sign the data.  It will be
 * checked for validity.
 *
 * "certusage" describes the signing usage (e.g. certUsageEmailSigner)
 * XXX Maybe SECCertUsage should be split so that our caller just says
 * "email" and *we* add the "signing" part -- otherwise our caller
 * could be lying about the usage; we do not want to allow encryption
 * certs for signing or vice versa.
 *
 * "certdb" is the cert database to use for verifying the cert.
 * It can be NULL if a default database is available (like in the client).
 *
 * "digestalg" names the digest algorithm (e.g. SEC_OID_SHA1).
 *
 * "digest" is the actual digest of the data.  It must be provided in
 * the case of detached data or NULL if the content will be included.
 *
 * The return value can be passed to functions which add things to
 * it like attributes, then eventually to SEC_PKCS7Encode() or to
 * SEC_PKCS7EncoderStart() to create the encoded data, and finally to
 * SEC_PKCS7DestroyContentInfo().
 *
 * An error results in a return value of NULL and an error set.
 * (Retrieve specific errors via PORT_GetError()/XP_GetError().)
 */
extern SEC_PKCS7ContentInfo *
SEC_PKCS7CreateSignedData(CERTCertificate *cert,
                          SECCertUsage certusage,
                          CERTCertDBHandle *certdb,
                          SECOidTag digestalg,
                          SECItem *digest,
                          SECKEYGetPasswordKey pwfn, void *pwfn_arg);

/*
 * Create a PKCS7 certs-only container.
 *
 * "cert" is the (first) cert that will be included.
 *
 * "include_chain" specifies whether the entire chain for "cert" should
 * be included.
 *
 * "certdb" is the cert database to use for finding the chain.
 * It can be NULL in when "include_chain" is false, or when meaning
 * use the default database.
 *
 * More certs and chains can be added via AddCertficate and AddCertChain.
 *
 * An error results in a return value of NULL and an error set.
 * (Retrieve specific errors via PORT_GetError()/XP_GetError().)
 */
extern SEC_PKCS7ContentInfo *
SEC_PKCS7CreateCertsOnly(CERTCertificate *cert,
                         PRBool include_chain,
                         CERTCertDBHandle *certdb);

/*
 * Start a PKCS7 enveloping context.
 *
 * "cert" is the cert for the recipient.  It will be checked for validity.
 *
 * "certusage" describes the encryption usage (e.g. certUsageEmailRecipient)
 * XXX Maybe SECCertUsage should be split so that our caller just says
 * "email" and *we* add the "recipient" part -- otherwise our caller
 * could be lying about the usage; we do not want to allow encryption
 * certs for signing or vice versa.
 *
 * "certdb" is the cert database to use for verifying the cert.
 * It can be NULL if a default database is available (like in the client).
 *
 * "encalg" specifies the bulk encryption algorithm to use (e.g. SEC_OID_RC2).
 *
 * "keysize" specifies the bulk encryption key size, in bits.
 *
 * The return value can be passed to functions which add things to
 * it like more recipients, then eventually to SEC_PKCS7Encode() or to
 * SEC_PKCS7EncoderStart() to create the encoded data, and finally to
 * SEC_PKCS7DestroyContentInfo().
 *
 * An error results in a return value of NULL and an error set.
 * (Retrieve specific errors via PORT_GetError()/XP_GetError().)
 */
extern SEC_PKCS7ContentInfo *
SEC_PKCS7CreateEnvelopedData(CERTCertificate *cert,
                             SECCertUsage certusage,
                             CERTCertDBHandle *certdb,
                             SECOidTag encalg,
                             int keysize,
                             SECKEYGetPasswordKey pwfn, void *pwfn_arg);

/*
 * XXX There will be a similar routine for creating signedAndEnvelopedData.
 * But its parameters will be different and I have no plans to implement
 * it any time soon because we have no current need for it.
 */

/*
 * Create an empty PKCS7 data content info.
 *
 * An error results in a return value of NULL and an error set.
 * (Retrieve specific errors via PORT_GetError()/XP_GetError().)
 */
extern SEC_PKCS7ContentInfo *SEC_PKCS7CreateData(void);

/*
 * Create an empty PKCS7 encrypted content info.
 *
 * "algorithm" specifies the bulk encryption algorithm to use.
 *
 * An error results in a return value of NULL and an error set.
 * (Retrieve specific errors via PORT_GetError()/XP_GetError().)
 */
extern SEC_PKCS7ContentInfo *
SEC_PKCS7CreateEncryptedData(SECOidTag algorithm, int keysize,
                             SECKEYGetPasswordKey pwfn, void *pwfn_arg);

/*
 * Create an empty PKCS7 encrypted content info.
 *
 * Similar to SEC_PKCS7CreateEncryptedData(), but this is capable of
 * creating encrypted content for PKCS #5 v2 algorithms.
 *
 * "pbe_algorithm" specifies the PBE algorithm to use.
 * "cipher_algorithm" specifies the bulk encryption algorithm to use.
 * "prf_algorithm" specifies the PRF algorithm which pbe_algorithm uses.
 *
 * An error results in a return value of NULL and an error set.
 * (Retrieve specific errors via PORT_GetError()/XP_GetError().)
 */
extern SEC_PKCS7ContentInfo *
SEC_PKCS7CreateEncryptedDataWithPBEV2(SECOidTag pbe_algorithm,
                                      SECOidTag cipher_algorithm,
                                      SECOidTag prf_algorithm,
                                      int keysize,
                                      SECKEYGetPasswordKey pwfn, void *pwfn_arg);

/*
 * All of the following things return SECStatus to signal success or failure.
 * Failure should have a more specific error status available via
 * PORT_GetError()/XP_GetError().
 */

/*
 * Add the specified attribute to the authenticated (i.e. signed) attributes
 * of "cinfo" -- "oidtag" describes the attribute and "value" is the
 * value to be associated with it.  NOTE! "value" must already be encoded;
 * no interpretation of "oidtag" is done.  Also, it is assumed that this
 * signedData has only one signer -- if we ever need to add attributes
 * when there is more than one signature, we need a way to specify *which*
 * signature should get the attribute.
 *
 * XXX Technically, a signed attribute can have multiple values; if/when
 * we ever need to support an attribute which takes multiple values, we
 * either need to change this interface or create an AddSignedAttributeValue
 * which can be called subsequently, and would then append a value.
 *
 * "cinfo" should be of type signedData (the only kind of pkcs7 data
 * that is allowed authenticated attributes); SECFailure will be returned
 * if it is not.
 */
extern SECStatus SEC_PKCS7AddSignedAttribute(SEC_PKCS7ContentInfo *cinfo,
                                             SECOidTag oidtag,
                                             SECItem *value);

/*
 * Add "cert" and its entire chain to the set of certs included in "cinfo".
 *
 * "certdb" is the cert database to use for finding the chain.
 * It can be NULL, meaning use the default database.
 *
 * "cinfo" should be of type signedData or signedAndEnvelopedData;
 * SECFailure will be returned if it is not.
 */
extern SECStatus SEC_PKCS7AddCertChain(SEC_PKCS7ContentInfo *cinfo,
                                       CERTCertificate *cert,
                                       CERTCertDBHandle *certdb);

/*
 * Add "cert" to the set of certs included in "cinfo".
 *
 * "cinfo" should be of type signedData or signedAndEnvelopedData;
 * SECFailure will be returned if it is not.
 */
extern SECStatus SEC_PKCS7AddCertificate(SEC_PKCS7ContentInfo *cinfo,
                                         CERTCertificate *cert);

/*
 * Add another recipient to an encrypted message.
 *
 * "cinfo" should be of type envelopedData or signedAndEnvelopedData;
 * SECFailure will be returned if it is not.
 *
 * "cert" is the cert for the recipient.  It will be checked for validity.
 *
 * "certusage" describes the encryption usage (e.g. certUsageEmailRecipient)
 * XXX Maybe SECCertUsage should be split so that our caller just says
 * "email" and *we* add the "recipient" part -- otherwise our caller
 * could be lying about the usage; we do not want to allow encryption
 * certs for signing or vice versa.
 *
 * "certdb" is the cert database to use for verifying the cert.
 * It can be NULL if a default database is available (like in the client).
 */
extern SECStatus SEC_PKCS7AddRecipient(SEC_PKCS7ContentInfo *cinfo,
                                       CERTCertificate *cert,
                                       SECCertUsage certusage,
                                       CERTCertDBHandle *certdb);

/*
 * Add the signing time to the authenticated (i.e. signed) attributes
 * of "cinfo".  This is expected to be included in outgoing signed
 * messages for email (S/MIME) but is likely useful in other situations.
 *
 * This should only be added once; a second call will either do
 * nothing or replace an old signing time with a newer one.
 *
 * XXX This will probably just shove the current time into "cinfo"
 * but it will not actually get signed until the entire item is
 * processed for encoding.  Is this (expected to be small) delay okay?
 *
 * "cinfo" should be of type signedData (the only kind of pkcs7 data
 * that is allowed authenticated attributes); SECFailure will be returned
 * if it is not.
 */
extern SECStatus SEC_PKCS7AddSigningTime(SEC_PKCS7ContentInfo *cinfo);

/*
 * Add the signer's symmetric capabilities to the authenticated
 * (i.e. signed) attributes of "cinfo".  This is expected to be
 * included in outgoing signed messages for email (S/MIME).
 *
 * This can only be added once; a second call will return SECFailure.
 *
 * "cinfo" should be of type signedData or signedAndEnvelopedData;
 * SECFailure will be returned if it is not.
 */
extern SECStatus SEC_PKCS7AddSymmetricCapabilities(SEC_PKCS7ContentInfo *cinfo);

/*
 * Mark that the signer's certificate and its issuing chain should
 * be included in the encoded data.  This is expected to be used
 * in outgoing signed messages for email (S/MIME).
 *
 * "certdb" is the cert database to use for finding the chain.
 * It can be NULL, meaning use the default database.
 *
 * "cinfo" should be of type signedData or signedAndEnvelopedData;
 * SECFailure will be returned if it is not.
 */
extern SECStatus SEC_PKCS7IncludeCertChain(SEC_PKCS7ContentInfo *cinfo,
                                           CERTCertDBHandle *certdb);

/*
 * Set the content; it will be included and also hashed and/or encrypted
 * as appropriate.  This is for in-memory content (expected to be "small")
 * that will be included in the PKCS7 object.  All others should stream the
 * content through when encoding (see SEC_PKCS7Encoder{Start,Update,Finish}).
 *
 * "buf" points to data of length "len"; it will be copied.
 */
extern SECStatus SEC_PKCS7SetContent(SEC_PKCS7ContentInfo *cinfo,
                                     const char *buf, unsigned long len);

/*
 * Encode a PKCS7 object, in one shot.  All necessary components
 * of the object must already be specified.  Either the data has
 * already been included (via SetContent), or the data is detached,
 * or there is no data at all (certs-only).
 *
 * "cinfo" specifies the object to be encoded.
 *
 * "outputfn" is where the encoded bytes will be passed.
 *
 * "outputarg" is an opaque argument to the above callback.
 *
 * "bulkkey" specifies the bulk encryption key to use.   This argument
 * can be NULL if no encryption is being done, or if the bulk key should
 * be generated internally (usually the case for EnvelopedData but never
 * for EncryptedData, which *must* provide a bulk encryption key).
 *
 * "pwfn" is a callback for getting the password which protects the
 * private key of the signer.  This argument can be NULL if it is known
 * that no signing is going to be done.
 *
 * "pwfnarg" is an opaque argument to the above callback.
 */
extern SECStatus SEC_PKCS7Encode(SEC_PKCS7ContentInfo *cinfo,
                                 SEC_PKCS7EncoderOutputCallback outputfn,
                                 void *outputarg,
                                 PK11SymKey *bulkkey,
                                 SECKEYGetPasswordKey pwfn,
                                 void *pwfnarg);

/*
 * Encode a PKCS7 object, in one shot.  All necessary components
 * of the object must already be specified.  Either the data has
 * already been included (via SetContent), or the data is detached,
 * or there is no data at all (certs-only).  The output, rather than
 * being passed to an output function as is done above, is all put
 * into a SECItem.
 *
 * "pool" specifies a pool from which to allocate the result.
 * It can be NULL, in which case memory is allocated generically.
 *
 * "dest" specifies a SECItem in which to put the result data.
 * It can be NULL, in which case the entire item is allocated, too.
 *
 * "cinfo" specifies the object to be encoded.
 *
 * "bulkkey" specifies the bulk encryption key to use.   This argument
 * can be NULL if no encryption is being done, or if the bulk key should
 * be generated internally (usually the case for EnvelopedData but never
 * for EncryptedData, which *must* provide a bulk encryption key).
 *
 * "pwfn" is a callback for getting the password which protects the
 * private key of the signer.  This argument can be NULL if it is known
 * that no signing is going to be done.
 *
 * "pwfnarg" is an opaque argument to the above callback.
 */
extern SECItem *SEC_PKCS7EncodeItem(PLArenaPool *pool,
                                    SECItem *dest,
                                    SEC_PKCS7ContentInfo *cinfo,
                                    PK11SymKey *bulkkey,
                                    SECKEYGetPasswordKey pwfn,
                                    void *pwfnarg);

/*
 * For those who want to simply point to the pkcs7 contentInfo ASN.1
 * template, and *not* call the encoding functions directly, the
 * following function can be used -- after it is called, the entire
 * PKCS7 contentInfo is ready to be encoded.
 */
extern SECStatus SEC_PKCS7PrepareForEncode(SEC_PKCS7ContentInfo *cinfo,
                                           PK11SymKey *bulkkey,
                                           SECKEYGetPasswordKey pwfn,
                                           void *pwfnarg);

/*
 * Start the process of encoding a PKCS7 object.  The first part of
 * the encoded object will be passed to the output function right away;
 * after that it is expected that SEC_PKCS7EncoderUpdate will be called,
 * streaming in the actual content that is getting included as well as
 * signed or encrypted (or both).
 *
 * "cinfo" specifies the object to be encoded.
 *
 * "outputfn" is where the encoded bytes will be passed.
 *
 * "outputarg" is an opaque argument to the above callback.
 *
 * "bulkkey" specifies the bulk encryption key to use.   This argument
 * can be NULL if no encryption is being done, or if the bulk key should
 * be generated internally (usually the case for EnvelopedData but never
 * for EncryptedData, which *must* provide a bulk encryption key).
 *
 * Returns an object to be passed to EncoderUpdate and EncoderFinish.
 */
extern SEC_PKCS7EncoderContext *
SEC_PKCS7EncoderStart(SEC_PKCS7ContentInfo *cinfo,
                      SEC_PKCS7EncoderOutputCallback outputfn,
                      void *outputarg,
                      PK11SymKey *bulkkey);

/*
 * Encode more contents, hashing and/or encrypting along the way.
 */
extern SECStatus SEC_PKCS7EncoderUpdate(SEC_PKCS7EncoderContext *p7ecx,
                                        const char *buf,
                                        unsigned long len);

/*
 * No more contents; finish the signature creation, if appropriate,
 * and then the encoding.
 *
 * "pwfn" is a callback for getting the password which protects the
 * signer's private key.  This argument can be NULL if it is known
 * that no signing is going to be done.
 *
 * "pwfnarg" is an opaque argument to the above callback.
 */
extern SECStatus SEC_PKCS7EncoderFinish(SEC_PKCS7EncoderContext *p7ecx,
                                        SECKEYGetPasswordKey pwfn,
                                        void *pwfnarg);

/*  Abort the underlying ASN.1 stream & set an error  */
void SEC_PKCS7EncoderAbort(SEC_PKCS7EncoderContext *p7dcx, int error);

/* retrieve the algorithm ID used to encrypt the content info
 * for encrypted and enveloped data.  The SECAlgorithmID pointer
 * returned needs to be freed as it is a copy of the algorithm
 * id in the content info.
 */
extern SECAlgorithmID *
SEC_PKCS7GetEncryptionAlgorithm(SEC_PKCS7ContentInfo *cinfo);

/* the content of an encrypted data content info is encrypted.
 * it is assumed that for encrypted data, that the data has already
 * been set and is in the "plainContent" field of the content info.
 *
 * cinfo is the content info to encrypt
 *
 * key is the key with which to perform the encryption.  if the
 *     algorithm is a password based encryption algorithm, the
 *     key is actually a password which will be processed per
 *     PKCS #5.
 *
 * in the event of an error, SECFailure is returned.  SECSuccess
 * indicates a success.
 */
extern SECStatus
SEC_PKCS7EncryptContents(PLArenaPool *poolp,
                         SEC_PKCS7ContentInfo *cinfo,
                         SECItem *key,
                         void *wincx);

/* the content of an encrypted data content info is decrypted.
 * it is assumed that for encrypted data, that the data has already
 * been set and is in the "encContent" field of the content info.
 *
 * cinfo is the content info to decrypt
 *
 * key is the key with which to perform the decryption.  if the
 *     algorithm is a password based encryption algorithm, the
 *     key is actually a password which will be processed per
 *     PKCS #5.
 *
 * in the event of an error, SECFailure is returned.  SECSuccess
 * indicates a success.
 */
extern SECStatus
SEC_PKCS7DecryptContents(PLArenaPool *poolp,
                         SEC_PKCS7ContentInfo *cinfo,
                         SECItem *key,
                         void *wincx);

/* retrieve the certificate list from the content info.  the list
 * is a pointer to the list in the content info.  this should not
 * be deleted or freed in any way short of calling
 * SEC_PKCS7DestroyContentInfo
 */
extern SECItem **
SEC_PKCS7GetCertificateList(SEC_PKCS7ContentInfo *cinfo);

/* Returns the key length (in bits) of the algorithm used to encrypt
   this object.  Returns 0 if it's not encrypted, or the key length is
   irrelevant. */
extern int
SEC_PKCS7GetKeyLength(SEC_PKCS7ContentInfo *cinfo);

/************************************************************************/
SEC_END_PROTOS

#endif /* _SECPKCS7_H_ */
