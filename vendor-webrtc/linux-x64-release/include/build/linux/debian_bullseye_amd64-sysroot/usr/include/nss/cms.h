/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/*
 * Interfaces of the CMS implementation.
 */

#ifndef _CMS_H_
#define _CMS_H_

#include "seccomon.h"

#include "secoidt.h"
#include "certt.h"
#include "keythi.h"
#include "hasht.h"
#include "cmst.h"

/************************************************************************/
SEC_BEGIN_PROTOS

/************************************************************************
 * cmsdecode.c - CMS decoding
 ************************************************************************/

/*
 * NSS_CMSDecoder_Start - set up decoding of a DER-encoded CMS message
 *
 * "poolp" - pointer to arena for message, or NULL if new pool should be created
 * "cb", "cb_arg" - callback function and argument for delivery of inner content
 *                  inner content will be stored in the message if cb is NULL.
 * "pwfn", pwfn_arg" - callback function for getting token password
 * "decrypt_key_cb", "decrypt_key_cb_arg" - callback function for getting bulk key for encryptedData
 */
extern NSSCMSDecoderContext *
NSS_CMSDecoder_Start(PLArenaPool *poolp,
                     NSSCMSContentCallback cb, void *cb_arg,
                     PK11PasswordFunc pwfn, void *pwfn_arg,
                     NSSCMSGetDecryptKeyCallback decrypt_key_cb, void *decrypt_key_cb_arg);

/*
 * NSS_CMSDecoder_Update - feed DER-encoded data to decoder
 */
extern SECStatus
NSS_CMSDecoder_Update(NSSCMSDecoderContext *p7dcx, const char *buf, unsigned long len);

/*
 * NSS_CMSDecoder_Cancel - cancel a decoding process
 */
extern void
NSS_CMSDecoder_Cancel(NSSCMSDecoderContext *p7dcx);

/*
 * NSS_CMSDecoder_Finish - mark the end of inner content and finish decoding
 */
extern NSSCMSMessage *
NSS_CMSDecoder_Finish(NSSCMSDecoderContext *p7dcx);

/*
 * NSS_CMSMessage_CreateFromDER - decode a CMS message from DER encoded data
 */
extern NSSCMSMessage *
NSS_CMSMessage_CreateFromDER(SECItem *DERmessage,
                             NSSCMSContentCallback cb, void *cb_arg,
                             PK11PasswordFunc pwfn, void *pwfn_arg,
                             NSSCMSGetDecryptKeyCallback decrypt_key_cb, void *decrypt_key_cb_arg);

/************************************************************************
 * cmsencode.c - CMS encoding
 ************************************************************************/

/*
 * NSS_CMSEncoder_Start - set up encoding of a CMS message
 *
 * "cmsg" - message to encode
 * "outputfn", "outputarg" - callback function for delivery of DER-encoded output
 *                           will not be called if NULL.
 * "dest" - if non-NULL, pointer to SECItem that will hold the DER-encoded output
 * "destpoolp" - pool to allocate DER-encoded output in
 * "pwfn", pwfn_arg" - callback function for getting token password
 * "decrypt_key_cb", "decrypt_key_cb_arg" - callback function for getting bulk key for encryptedData
 * "detached_digestalgs", "detached_digests" - digests from detached content
 */
extern NSSCMSEncoderContext *
NSS_CMSEncoder_Start(NSSCMSMessage *cmsg,
                     NSSCMSContentCallback outputfn, void *outputarg,
                     SECItem *dest, PLArenaPool *destpoolp,
                     PK11PasswordFunc pwfn, void *pwfn_arg,
                     NSSCMSGetDecryptKeyCallback decrypt_key_cb, void *decrypt_key_cb_arg,
                     SECAlgorithmID **detached_digestalgs, SECItem **detached_digests);

/*
 * NSS_CMSEncoder_Update - take content data delivery from the user
 *
 * "p7ecx" - encoder context
 * "data" - content data
 * "len" - length of content data
 */
extern SECStatus
NSS_CMSEncoder_Update(NSSCMSEncoderContext *p7ecx, const char *data, unsigned long len);

/*
 * NSS_CMSEncoder_Cancel - stop all encoding
 */
extern SECStatus
NSS_CMSEncoder_Cancel(NSSCMSEncoderContext *p7ecx);

/*
 * NSS_CMSEncoder_Finish - signal the end of data
 *
 * we need to walk down the chain of encoders and the finish them from the innermost out
 */
extern SECStatus
NSS_CMSEncoder_Finish(NSSCMSEncoderContext *p7ecx);

/************************************************************************
 * cmsmessage.c - CMS message object
 ************************************************************************/

/*
 * NSS_CMSMessage_Create - create a CMS message object
 *
 * "poolp" - arena to allocate memory from, or NULL if new arena should be created
 */
extern NSSCMSMessage *
NSS_CMSMessage_Create(PLArenaPool *poolp);

/*
 * NSS_CMSMessage_SetEncodingParams - set up a CMS message object for encoding or decoding
 *
 * "cmsg" - message object
 * "pwfn", pwfn_arg" - callback function for getting token password
 * "decrypt_key_cb", "decrypt_key_cb_arg" - callback function for getting bulk key for encryptedData
 * "detached_digestalgs", "detached_digests" - digests from detached content
 *
 * used internally.
 */
extern void
NSS_CMSMessage_SetEncodingParams(NSSCMSMessage *cmsg,
                                 PK11PasswordFunc pwfn, void *pwfn_arg,
                                 NSSCMSGetDecryptKeyCallback decrypt_key_cb, void *decrypt_key_cb_arg,
                                 SECAlgorithmID **detached_digestalgs, SECItem **detached_digests);

/*
 * NSS_CMSMessage_Destroy - destroy a CMS message and all of its sub-pieces.
 */
extern void
NSS_CMSMessage_Destroy(NSSCMSMessage *cmsg);

/*
 * NSS_CMSMessage_Copy - return a copy of the given message.
 *
 * The copy may be virtual or may be real -- either way, the result needs
 * to be passed to NSS_CMSMessage_Destroy later (as does the original).
 */
extern NSSCMSMessage *
NSS_CMSMessage_Copy(NSSCMSMessage *cmsg);

/*
 * NSS_CMSMessage_GetArena - return a pointer to the message's arena pool
 */
extern PLArenaPool *
NSS_CMSMessage_GetArena(NSSCMSMessage *cmsg);

/*
 * NSS_CMSMessage_GetContentInfo - return a pointer to the top level contentInfo
 */
extern NSSCMSContentInfo *
NSS_CMSMessage_GetContentInfo(NSSCMSMessage *cmsg);

/*
 * Return a pointer to the actual content.
 * In the case of those types which are encrypted, this returns the *plain* content.
 * In case of nested contentInfos, this descends and retrieves the innermost content.
 */
extern SECItem *
NSS_CMSMessage_GetContent(NSSCMSMessage *cmsg);

/*
 * NSS_CMSMessage_ContentLevelCount - count number of levels of CMS content objects in this message
 *
 * CMS data content objects do not count.
 */
extern int
NSS_CMSMessage_ContentLevelCount(NSSCMSMessage *cmsg);

/*
 * NSS_CMSMessage_ContentLevel - find content level #n
 *
 * CMS data content objects do not count.
 */
extern NSSCMSContentInfo *
NSS_CMSMessage_ContentLevel(NSSCMSMessage *cmsg, int n);

/*
 * NSS_CMSMessage_ContainsCertsOrCrls - see if message contains certs along the way
 */
extern PRBool
NSS_CMSMessage_ContainsCertsOrCrls(NSSCMSMessage *cmsg);

/*
 * NSS_CMSMessage_IsEncrypted - see if message contains a encrypted submessage
 */
extern PRBool
NSS_CMSMessage_IsEncrypted(NSSCMSMessage *cmsg);

/*
 * NSS_CMSMessage_IsSigned - see if message contains a signed submessage
 *
 * If the CMS message has a SignedData with a signature (not just a SignedData)
 * return true; false otherwise.  This can/should be called before calling
 * VerifySignature, which will always indicate failure if no signature is
 * present, but that does not mean there even was a signature!
 * Note that the content itself can be empty (detached content was sent
 * another way); it is the presence of the signature that matters.
 */
extern PRBool
NSS_CMSMessage_IsSigned(NSSCMSMessage *cmsg);

/*
 * NSS_CMSMessage_IsContentEmpty - see if content is empty
 *
 * returns PR_TRUE is innermost content length is < minLen
 * XXX need the encrypted content length (why?)
 */
extern PRBool
NSS_CMSMessage_IsContentEmpty(NSSCMSMessage *cmsg, unsigned int minLen);

/************************************************************************
 * cmscinfo.c - CMS contentInfo methods
 ************************************************************************/

/*
 * NSS_CMSContentInfo_Destroy - destroy a CMS contentInfo and all of its sub-pieces.
 */
extern void
NSS_CMSContentInfo_Destroy(NSSCMSContentInfo *cinfo);

/*
 * NSS_CMSContentInfo_GetChildContentInfo - get content's contentInfo (if it exists)
 */
extern NSSCMSContentInfo *
NSS_CMSContentInfo_GetChildContentInfo(NSSCMSContentInfo *cinfo);

/*
 * NSS_CMSContentInfo_SetContent - set cinfo's content type & content to CMS object
 */
extern SECStatus
NSS_CMSContentInfo_SetContent(NSSCMSMessage *cmsg, NSSCMSContentInfo *cinfo, SECOidTag type, void *ptr);

/*
 * NSS_CMSContentInfo_SetContent_XXXX - typesafe wrappers for NSS_CMSContentInfo_SetType
 *   set cinfo's content type & content to CMS object
 */
extern SECStatus
NSS_CMSContentInfo_SetContent_Data(NSSCMSMessage *cmsg, NSSCMSContentInfo *cinfo, SECItem *data, PRBool detached);

extern SECStatus
NSS_CMSContentInfo_SetContent_SignedData(NSSCMSMessage *cmsg, NSSCMSContentInfo *cinfo, NSSCMSSignedData *sigd);

extern SECStatus
NSS_CMSContentInfo_SetContent_EnvelopedData(NSSCMSMessage *cmsg, NSSCMSContentInfo *cinfo, NSSCMSEnvelopedData *envd);

extern SECStatus
NSS_CMSContentInfo_SetContent_DigestedData(NSSCMSMessage *cmsg, NSSCMSContentInfo *cinfo, NSSCMSDigestedData *digd);

extern SECStatus
NSS_CMSContentInfo_SetContent_EncryptedData(NSSCMSMessage *cmsg, NSSCMSContentInfo *cinfo, NSSCMSEncryptedData *encd);

/*
 * turn off streaming for this content type.
 * This could fail with SEC_ERROR_NO_MEMORY in memory constrained conditions.
 */
extern SECStatus
NSS_CMSContentInfo_SetDontStream(NSSCMSContentInfo *cinfo, PRBool dontStream);

/*
 * NSS_CMSContentInfo_GetContent - get pointer to inner content
 *
 * needs to be casted...
 */
extern void *
NSS_CMSContentInfo_GetContent(NSSCMSContentInfo *cinfo);

/*
 * NSS_CMSContentInfo_GetInnerContent - get pointer to innermost content
 *
 * this is typically only called by NSS_CMSMessage_GetContent()
 */
extern SECItem *
NSS_CMSContentInfo_GetInnerContent(NSSCMSContentInfo *cinfo);

/*
 * NSS_CMSContentInfo_GetContentType{Tag,OID} - find out (saving pointer to lookup result
 * for future reference) and return the inner content type.
 */
extern SECOidTag
NSS_CMSContentInfo_GetContentTypeTag(NSSCMSContentInfo *cinfo);

extern SECItem *
NSS_CMSContentInfo_GetContentTypeOID(NSSCMSContentInfo *cinfo);

/*
 * NSS_CMSContentInfo_GetContentEncAlgTag - find out (saving pointer to lookup result
 * for future reference) and return the content encryption algorithm tag.
 */
extern SECOidTag
NSS_CMSContentInfo_GetContentEncAlgTag(NSSCMSContentInfo *cinfo);

/*
 * NSS_CMSContentInfo_GetContentEncAlg - find out and return the content encryption algorithm tag.
 */
extern SECAlgorithmID *
NSS_CMSContentInfo_GetContentEncAlg(NSSCMSContentInfo *cinfo);

extern SECStatus
NSS_CMSContentInfo_SetContentEncAlg(PLArenaPool *poolp, NSSCMSContentInfo *cinfo,
                                    SECOidTag bulkalgtag, SECItem *parameters, int keysize);

extern SECStatus
NSS_CMSContentInfo_SetContentEncAlgID(PLArenaPool *poolp, NSSCMSContentInfo *cinfo,
                                      SECAlgorithmID *algid, int keysize);

extern void
NSS_CMSContentInfo_SetBulkKey(NSSCMSContentInfo *cinfo, PK11SymKey *bulkkey);

extern PK11SymKey *
NSS_CMSContentInfo_GetBulkKey(NSSCMSContentInfo *cinfo);

extern int
NSS_CMSContentInfo_GetBulkKeySize(NSSCMSContentInfo *cinfo);

/************************************************************************
 * cmsutil.c - CMS misc utility functions
 ************************************************************************/

/*
 * NSS_CMSArray_SortByDER - sort array of objects by objects' DER encoding
 *
 * make sure that the order of the objects guarantees valid DER (which must be
 * in lexigraphically ascending order for a SET OF); if reordering is necessary it
 * will be done in place (in objs).
 */
extern SECStatus
NSS_CMSArray_SortByDER(void **objs, const SEC_ASN1Template *objtemplate, void **objs2);

/*
 * NSS_CMSUtil_DERCompare - for use with NSS_CMSArray_Sort to
 *  sort arrays of SECItems containing DER
 */
extern int
NSS_CMSUtil_DERCompare(void *a, void *b);

/*
 * NSS_CMSAlgArray_GetIndexByAlgID - find a specific algorithm in an array of
 * algorithms.
 *
 * algorithmArray - array of algorithm IDs
 * algid - algorithmid of algorithm to pick
 *
 * Returns:
 *  An integer containing the index of the algorithm in the array or -1 if
 *  algorithm was not found.
 */
extern int
NSS_CMSAlgArray_GetIndexByAlgID(SECAlgorithmID **algorithmArray, SECAlgorithmID *algid);

/*
 * NSS_CMSAlgArray_GetIndexByAlgID - find a specific algorithm in an array of
 * algorithms.
 *
 * algorithmArray - array of algorithm IDs
 * algiddata - id of algorithm to pick
 *
 * Returns:
 *  An integer containing the index of the algorithm in the array or -1 if
 *  algorithm was not found.
 */
extern int
NSS_CMSAlgArray_GetIndexByAlgTag(SECAlgorithmID **algorithmArray, SECOidTag algtag);

extern const SECHashObject *
NSS_CMSUtil_GetHashObjByAlgID(SECAlgorithmID *algid);

extern const SEC_ASN1Template *
NSS_CMSUtil_GetTemplateByTypeTag(SECOidTag type);

extern size_t
NSS_CMSUtil_GetSizeByTypeTag(SECOidTag type);

extern NSSCMSContentInfo *
NSS_CMSContent_GetContentInfo(void *msg, SECOidTag type);

extern const char *
NSS_CMSUtil_VerificationStatusToString(NSSCMSVerificationStatus vs);

/************************************************************************
 * cmssigdata.c - CMS signedData methods
 ************************************************************************/

extern NSSCMSSignedData *
NSS_CMSSignedData_Create(NSSCMSMessage *cmsg);

extern void
NSS_CMSSignedData_Destroy(NSSCMSSignedData *sigd);

/*
 * NSS_CMSSignedData_Encode_BeforeStart - do all the necessary things to a SignedData
 *     before start of encoding.
 *
 * In detail:
 *  - find out about the right value to put into sigd->version
 *  - come up with a list of digestAlgorithms (which should be the union of the algorithms
 *         in the signerinfos).
 *         If we happen to have a pre-set list of algorithms (and digest values!), we
 *         check if we have all the signerinfos' algorithms. If not, this is an error.
 */
extern SECStatus
NSS_CMSSignedData_Encode_BeforeStart(NSSCMSSignedData *sigd);

extern SECStatus
NSS_CMSSignedData_Encode_BeforeData(NSSCMSSignedData *sigd);

/*
 * NSS_CMSSignedData_Encode_AfterData - do all the necessary things to a SignedData
 *     after all the encapsulated data was passed through the encoder.
 *
 * In detail:
 *  - create the signatures in all the SignerInfos
 *
 * Please note that nothing is done to the Certificates and CRLs in the message - this
 * is entirely the responsibility of our callers.
 */
extern SECStatus
NSS_CMSSignedData_Encode_AfterData(NSSCMSSignedData *sigd);

extern SECStatus
NSS_CMSSignedData_Decode_BeforeData(NSSCMSSignedData *sigd);

/*
 * NSS_CMSSignedData_Decode_AfterData - do all the necessary things to a SignedData
 *     after all the encapsulated data was passed through the decoder.
 */
extern SECStatus
NSS_CMSSignedData_Decode_AfterData(NSSCMSSignedData *sigd);

/*
 * NSS_CMSSignedData_Decode_AfterEnd - do all the necessary things to a SignedData
 *     after all decoding is finished.
 */
extern SECStatus
NSS_CMSSignedData_Decode_AfterEnd(NSSCMSSignedData *sigd);

/*
 * NSS_CMSSignedData_GetSignerInfos - retrieve the SignedData's signer list
 */
extern NSSCMSSignerInfo **
NSS_CMSSignedData_GetSignerInfos(NSSCMSSignedData *sigd);

extern int
NSS_CMSSignedData_SignerInfoCount(NSSCMSSignedData *sigd);

extern NSSCMSSignerInfo *
NSS_CMSSignedData_GetSignerInfo(NSSCMSSignedData *sigd, int i);

/*
 * NSS_CMSSignedData_GetDigestAlgs - retrieve the SignedData's digest algorithm list
 */
extern SECAlgorithmID **
NSS_CMSSignedData_GetDigestAlgs(NSSCMSSignedData *sigd);

/*
 * NSS_CMSSignedData_GetContentInfo - return pointer to this signedData's contentinfo
 */
extern NSSCMSContentInfo *
NSS_CMSSignedData_GetContentInfo(NSSCMSSignedData *sigd);

/*
 * NSS_CMSSignedData_GetCertificateList - retrieve the SignedData's certificate list
 */
extern SECItem **
NSS_CMSSignedData_GetCertificateList(NSSCMSSignedData *sigd);

extern SECStatus
NSS_CMSSignedData_ImportCerts(NSSCMSSignedData *sigd, CERTCertDBHandle *certdb,
                              SECCertUsage certusage, PRBool keepcerts);

/*
 * NSS_CMSSignedData_HasDigests - see if we have digests in place
 */
extern PRBool
NSS_CMSSignedData_HasDigests(NSSCMSSignedData *sigd);

/*
 * NSS_CMSSignedData_VerifySignerInfo - check a signature.
 *
 * The digests were either calculated during decoding (and are stored in the
 * signedData itself) or set after decoding using NSS_CMSSignedData_SetDigests.
 *
 * The verification checks if the signing cert is valid and has a trusted chain
 * for the purpose specified by "certusage".
 */
extern SECStatus
NSS_CMSSignedData_VerifySignerInfo(NSSCMSSignedData *sigd, int i, CERTCertDBHandle *certdb,
                                   SECCertUsage certusage);

/*
 * NSS_CMSSignedData_VerifyCertsOnly - verify the certs in a certs-only message
*/
extern SECStatus
NSS_CMSSignedData_VerifyCertsOnly(NSSCMSSignedData *sigd,
                                  CERTCertDBHandle *certdb,
                                  SECCertUsage usage);

extern SECStatus
NSS_CMSSignedData_AddCertList(NSSCMSSignedData *sigd, CERTCertificateList *certlist);

/*
 * NSS_CMSSignedData_AddCertChain - add cert and its entire chain to the set of certs
 */
extern SECStatus
NSS_CMSSignedData_AddCertChain(NSSCMSSignedData *sigd, CERTCertificate *cert);

extern SECStatus
NSS_CMSSignedData_AddCertificate(NSSCMSSignedData *sigd, CERTCertificate *cert);

extern PRBool
NSS_CMSSignedData_ContainsCertsOrCrls(NSSCMSSignedData *sigd);

extern SECStatus
NSS_CMSSignedData_AddSignerInfo(NSSCMSSignedData *sigd,
                                NSSCMSSignerInfo *signerinfo);

extern SECStatus
NSS_CMSSignedData_SetDigests(NSSCMSSignedData *sigd,
                             SECAlgorithmID **digestalgs,
                             SECItem **digests);

extern SECStatus
NSS_CMSSignedData_SetDigestValue(NSSCMSSignedData *sigd,
                                 SECOidTag digestalgtag,
                                 SECItem *digestdata);

extern SECStatus
NSS_CMSSignedData_AddDigest(PLArenaPool *poolp,
                            NSSCMSSignedData *sigd,
                            SECOidTag digestalgtag,
                            SECItem *digest);

extern SECItem *
NSS_CMSSignedData_GetDigestValue(NSSCMSSignedData *sigd, SECOidTag digestalgtag);

/*
 * NSS_CMSSignedData_CreateCertsOnly - create a certs-only SignedData.
 *
 * cert          - base certificates that will be included
 * include_chain - if true, include the complete cert chain for cert
 *
 * More certs and chains can be added via AddCertificate and AddCertChain.
 *
 * An error results in a return value of NULL and an error set.
 */
extern NSSCMSSignedData *
NSS_CMSSignedData_CreateCertsOnly(NSSCMSMessage *cmsg, CERTCertificate *cert, PRBool include_chain);

/************************************************************************
 * cmssiginfo.c - signerinfo methods
 ************************************************************************/

extern NSSCMSSignerInfo *
NSS_CMSSignerInfo_Create(NSSCMSMessage *cmsg, CERTCertificate *cert, SECOidTag digestalgtag);
extern NSSCMSSignerInfo *
NSS_CMSSignerInfo_CreateWithSubjKeyID(NSSCMSMessage *cmsg, SECItem *subjKeyID, SECKEYPublicKey *pubKey, SECKEYPrivateKey *signingKey, SECOidTag digestalgtag);

/*
 * NSS_CMSSignerInfo_Destroy - destroy a SignerInfo data structure
 */
extern void
NSS_CMSSignerInfo_Destroy(NSSCMSSignerInfo *si);

/*
 * NSS_CMSSignerInfo_Sign - sign something
 *
 */
extern SECStatus
NSS_CMSSignerInfo_Sign(NSSCMSSignerInfo *signerinfo, SECItem *digest, SECItem *contentType);

extern SECStatus
NSS_CMSSignerInfo_VerifyCertificate(NSSCMSSignerInfo *signerinfo, CERTCertDBHandle *certdb,
                                    SECCertUsage certusage);

/*
 * NSS_CMSSignerInfo_Verify - verify the signature of a single SignerInfo
 *
 * Just verifies the signature. The assumption is that verification of the certificate
 * is done already.
 */
extern SECStatus
NSS_CMSSignerInfo_Verify(NSSCMSSignerInfo *signerinfo, SECItem *digest, SECItem *contentType);

extern NSSCMSVerificationStatus
NSS_CMSSignerInfo_GetVerificationStatus(NSSCMSSignerInfo *signerinfo);

extern SECOidData *
NSS_CMSSignerInfo_GetDigestAlg(NSSCMSSignerInfo *signerinfo);

extern SECOidTag
NSS_CMSSignerInfo_GetDigestAlgTag(NSSCMSSignerInfo *signerinfo);

extern int
NSS_CMSSignerInfo_GetVersion(NSSCMSSignerInfo *signerinfo);

extern CERTCertificateList *
NSS_CMSSignerInfo_GetCertList(NSSCMSSignerInfo *signerinfo);

/*
 * NSS_CMSSignerInfo_GetSigningTime - return the signing time,
 *                                    in UTCTime format, of a CMS signerInfo.
 *
 * sinfo - signerInfo data for this signer
 *
 * Returns a pointer to XXXX (what?)
 * A return value of NULL is an error.
 */
extern SECStatus
NSS_CMSSignerInfo_GetSigningTime(NSSCMSSignerInfo *sinfo, PRTime *stime);

/*
 * Return the signing cert of a CMS signerInfo.
 *
 * the certs in the enclosing SignedData must have been imported already
 */
extern CERTCertificate *
NSS_CMSSignerInfo_GetSigningCertificate(NSSCMSSignerInfo *signerinfo, CERTCertDBHandle *certdb);

/*
 * NSS_CMSSignerInfo_GetSignerCommonName - return the common name of the signer
 *
 * sinfo - signerInfo data for this signer
 *
 * Returns a pointer to allocated memory, which must be freed with PORT_Free.
 * A return value of NULL is an error.
 */
extern char *
NSS_CMSSignerInfo_GetSignerCommonName(NSSCMSSignerInfo *sinfo);

/*
 * NSS_CMSSignerInfo_GetSignerEmailAddress - return the common name of the signer
 *
 * sinfo - signerInfo data for this signer
 *
 * Returns a pointer to allocated memory, which must be freed.
 * A return value of NULL is an error.
 */
extern char *
NSS_CMSSignerInfo_GetSignerEmailAddress(NSSCMSSignerInfo *sinfo);

/*
 * NSS_CMSSignerInfo_AddAuthAttr - add an attribute to the
 * authenticated (i.e. signed) attributes of "signerinfo".
 */
extern SECStatus
NSS_CMSSignerInfo_AddAuthAttr(NSSCMSSignerInfo *signerinfo, NSSCMSAttribute *attr);

/*
 * NSS_CMSSignerInfo_AddUnauthAttr - add an attribute to the
 * unauthenticated attributes of "signerinfo".
 */
extern SECStatus
NSS_CMSSignerInfo_AddUnauthAttr(NSSCMSSignerInfo *signerinfo, NSSCMSAttribute *attr);

/*
 * NSS_CMSSignerInfo_AddSigningTime - add the signing time to the
 * authenticated (i.e. signed) attributes of "signerinfo".
 *
 * This is expected to be included in outgoing signed
 * messages for email (S/MIME) but is likely useful in other situations.
 *
 * This should only be added once; a second call will do nothing.
 *
 * XXX This will probably just shove the current time into "signerinfo"
 * but it will not actually get signed until the entire item is
 * processed for encoding.  Is this (expected to be small) delay okay?
 */
extern SECStatus
NSS_CMSSignerInfo_AddSigningTime(NSSCMSSignerInfo *signerinfo, PRTime t);

/*
 * NSS_CMSSignerInfo_AddSMIMECaps - add a SMIMECapabilities attribute to the
 * authenticated (i.e. signed) attributes of "signerinfo".
 *
 * This is expected to be included in outgoing signed
 * messages for email (S/MIME).
 */
extern SECStatus
NSS_CMSSignerInfo_AddSMIMECaps(NSSCMSSignerInfo *signerinfo);

/*
 * NSS_CMSSignerInfo_AddSMIMEEncKeyPrefs - add a SMIMEEncryptionKeyPreferences attribute to the
 * authenticated (i.e. signed) attributes of "signerinfo".
 *
 * This is expected to be included in outgoing signed messages for email (S/MIME).
 */
SECStatus
NSS_CMSSignerInfo_AddSMIMEEncKeyPrefs(NSSCMSSignerInfo *signerinfo, CERTCertificate *cert, CERTCertDBHandle *certdb);

/*
 * NSS_CMSSignerInfo_AddMSSMIMEEncKeyPrefs - add a SMIMEEncryptionKeyPreferences attribute to the
 * authenticated (i.e. signed) attributes of "signerinfo", using the OID preferred by Microsoft.
 *
 * This is expected to be included in outgoing signed messages for email (S/MIME),
 * if compatibility with Microsoft mail clients is wanted.
 */
SECStatus
NSS_CMSSignerInfo_AddMSSMIMEEncKeyPrefs(NSSCMSSignerInfo *signerinfo, CERTCertificate *cert, CERTCertDBHandle *certdb);

/*
 * NSS_CMSSignerInfo_AddCounterSignature - countersign a signerinfo
 */
extern SECStatus
NSS_CMSSignerInfo_AddCounterSignature(NSSCMSSignerInfo *signerinfo,
                                      SECOidTag digestalg, CERTCertificate signingcert);

/*
 * XXXX the following needs to be done in the S/MIME layer code
 * after signature of a signerinfo is verified
 */
extern SECStatus
NSS_SMIMESignerInfo_SaveSMIMEProfile(NSSCMSSignerInfo *signerinfo);

/*
 * NSS_CMSSignerInfo_IncludeCerts - set cert chain inclusion mode for this signer
 */
extern SECStatus
NSS_CMSSignerInfo_IncludeCerts(NSSCMSSignerInfo *signerinfo, NSSCMSCertChainMode cm, SECCertUsage usage);

/************************************************************************
 * cmsenvdata.c - CMS envelopedData methods
 ************************************************************************/

/*
 * NSS_CMSEnvelopedData_Create - create an enveloped data message
 */
extern NSSCMSEnvelopedData *
NSS_CMSEnvelopedData_Create(NSSCMSMessage *cmsg, SECOidTag algorithm, int keysize);

/*
 * NSS_CMSEnvelopedData_Destroy - destroy an enveloped data message
 */
extern void
NSS_CMSEnvelopedData_Destroy(NSSCMSEnvelopedData *edp);

/*
 * NSS_CMSEnvelopedData_GetContentInfo - return pointer to this envelopedData's contentinfo
 */
extern NSSCMSContentInfo *
NSS_CMSEnvelopedData_GetContentInfo(NSSCMSEnvelopedData *envd);

/*
 * NSS_CMSEnvelopedData_AddRecipient - add a recipientinfo to the enveloped data msg
 *
 * rip must be created on the same pool as edp - this is not enforced, though.
 */
extern SECStatus
NSS_CMSEnvelopedData_AddRecipient(NSSCMSEnvelopedData *edp, NSSCMSRecipientInfo *rip);

/*
 * NSS_CMSEnvelopedData_Encode_BeforeStart - prepare this envelopedData for encoding
 *
 * at this point, we need
 * - recipientinfos set up with recipient's certificates
 * - a content encryption algorithm (if none, 3DES will be used)
 *
 * this function will generate a random content encryption key (aka bulk key),
 * initialize the recipientinfos with certificate identification and wrap the bulk key
 * using the proper algorithm for every certificiate.
 * it will finally set the bulk algorithm and key so that the encode step can find it.
 */
extern SECStatus
NSS_CMSEnvelopedData_Encode_BeforeStart(NSSCMSEnvelopedData *envd);

/*
 * NSS_CMSEnvelopedData_Encode_BeforeData - set up encryption
 */
extern SECStatus
NSS_CMSEnvelopedData_Encode_BeforeData(NSSCMSEnvelopedData *envd);

/*
 * NSS_CMSEnvelopedData_Encode_AfterData - finalize this envelopedData for encoding
 */
extern SECStatus
NSS_CMSEnvelopedData_Encode_AfterData(NSSCMSEnvelopedData *envd);

/*
 * NSS_CMSEnvelopedData_Decode_BeforeData - find our recipientinfo,
 * derive bulk key & set up our contentinfo
 */
extern SECStatus
NSS_CMSEnvelopedData_Decode_BeforeData(NSSCMSEnvelopedData *envd);

/*
 * NSS_CMSEnvelopedData_Decode_AfterData - finish decrypting this envelopedData's content
 */
extern SECStatus
NSS_CMSEnvelopedData_Decode_AfterData(NSSCMSEnvelopedData *envd);

/*
 * NSS_CMSEnvelopedData_Decode_AfterEnd - finish decoding this envelopedData
 */
extern SECStatus
NSS_CMSEnvelopedData_Decode_AfterEnd(NSSCMSEnvelopedData *envd);

/************************************************************************
 * cmsrecinfo.c - CMS recipientInfo methods
 ************************************************************************/

/*
 * NSS_CMSRecipientInfo_Create - create a recipientinfo
 *
 * we currently do not create KeyAgreement recipientinfos with multiple recipientEncryptedKeys
 * the certificate is supposed to have been verified by the caller
 */
extern NSSCMSRecipientInfo *
NSS_CMSRecipientInfo_Create(NSSCMSMessage *cmsg, CERTCertificate *cert);

extern NSSCMSRecipientInfo *
NSS_CMSRecipientInfo_CreateWithSubjKeyID(NSSCMSMessage *cmsg,
                                         SECItem *subjKeyID,
                                         SECKEYPublicKey *pubKey);

extern NSSCMSRecipientInfo *
NSS_CMSRecipientInfo_CreateWithSubjKeyIDFromCert(NSSCMSMessage *cmsg,
                                                 CERTCertificate *cert);

/*
 * NSS_CMSRecipientInfo_CreateNew - create a blank recipientinfo for
 * applications which want to encode their own CMS structures and
 * key exchange types.
 */
extern NSSCMSRecipientInfo *
NSS_CMSRecipientInfo_CreateNew(void *pwfn_arg);

/*
 * NSS_CMSRecipientInfo_CreateFromDER - create a recipientinfo  from partially
 * decoded DER data for applications which want to encode their own CMS
 * structures and key exchange types.
 */
extern NSSCMSRecipientInfo *
NSS_CMSRecipientInfo_CreateFromDER(SECItem *input, void *pwfn_arg);

extern void
NSS_CMSRecipientInfo_Destroy(NSSCMSRecipientInfo *ri);

/*
 * NSS_CMSRecipientInfo_GetCertAndKey - retrieve the cert and key from the
 * recipientInfo struct. If retcert or retkey are NULL, the cert or
 * key (respectively) would not be returned). This function is a no-op if both
 * retcert and retkey are NULL. Caller inherits ownership of the cert and key
 * he requested (and is responsible to free them).
 */
SECStatus NSS_CMSRecipientInfo_GetCertAndKey(NSSCMSRecipientInfo *ri,
                                             CERTCertificate **retcert,
                                             SECKEYPrivateKey **retkey);

extern int
NSS_CMSRecipientInfo_GetVersion(NSSCMSRecipientInfo *ri);

extern SECItem *
NSS_CMSRecipientInfo_GetEncryptedKey(NSSCMSRecipientInfo *ri, int subIndex);

/*
 * NSS_CMSRecipientInfo_Encode - encode an NSS_CMSRecipientInfo as ASN.1
 */
SECStatus NSS_CMSRecipientInfo_Encode(PLArenaPool *poolp,
                                      const NSSCMSRecipientInfo *src,
                                      SECItem *returned);

extern SECOidTag
NSS_CMSRecipientInfo_GetKeyEncryptionAlgorithmTag(NSSCMSRecipientInfo *ri);

extern SECStatus
NSS_CMSRecipientInfo_WrapBulkKey(NSSCMSRecipientInfo *ri, PK11SymKey *bulkkey,
                                 SECOidTag bulkalgtag);

extern PK11SymKey *
NSS_CMSRecipientInfo_UnwrapBulkKey(NSSCMSRecipientInfo *ri, int subIndex,
                                   CERTCertificate *cert, SECKEYPrivateKey *privkey,
                                   SECOidTag bulkalgtag);

/************************************************************************
 * cmsencdata.c - CMS encryptedData methods
 ************************************************************************/
/*
 * NSS_CMSEncryptedData_Create - create an empty encryptedData object.
 *
 * "algorithm" specifies the bulk encryption algorithm to use.
 * "keysize" is the key size.
 *
 * An error results in a return value of NULL and an error set.
 * (Retrieve specific errors via PORT_GetError()/XP_GetError().)
 */
extern NSSCMSEncryptedData *
NSS_CMSEncryptedData_Create(NSSCMSMessage *cmsg, SECOidTag algorithm, int keysize);

/*
 * NSS_CMSEncryptedData_Destroy - destroy an encryptedData object
 */
extern void
NSS_CMSEncryptedData_Destroy(NSSCMSEncryptedData *encd);

/*
 * NSS_CMSEncryptedData_GetContentInfo - return pointer to encryptedData object's contentInfo
 */
extern NSSCMSContentInfo *
NSS_CMSEncryptedData_GetContentInfo(NSSCMSEncryptedData *encd);

/*
 * NSS_CMSEncryptedData_Encode_BeforeStart - do all the necessary things to a EncryptedData
 *     before encoding begins.
 *
 * In particular:
 *  - set the correct version value.
 *  - get the encryption key
 */
extern SECStatus
NSS_CMSEncryptedData_Encode_BeforeStart(NSSCMSEncryptedData *encd);

/*
 * NSS_CMSEncryptedData_Encode_BeforeData - set up encryption
 */
extern SECStatus
NSS_CMSEncryptedData_Encode_BeforeData(NSSCMSEncryptedData *encd);

/*
 * NSS_CMSEncryptedData_Encode_AfterData - finalize this encryptedData for encoding
 */
extern SECStatus
NSS_CMSEncryptedData_Encode_AfterData(NSSCMSEncryptedData *encd);

/*
 * NSS_CMSEncryptedData_Decode_BeforeData - find bulk key & set up decryption
 */
extern SECStatus
NSS_CMSEncryptedData_Decode_BeforeData(NSSCMSEncryptedData *encd);

/*
 * NSS_CMSEncryptedData_Decode_AfterData - finish decrypting this encryptedData's content
 */
extern SECStatus
NSS_CMSEncryptedData_Decode_AfterData(NSSCMSEncryptedData *encd);

/*
 * NSS_CMSEncryptedData_Decode_AfterEnd - finish decoding this encryptedData
 */
extern SECStatus
NSS_CMSEncryptedData_Decode_AfterEnd(NSSCMSEncryptedData *encd);

/************************************************************************
 * cmsdigdata.c - CMS encryptedData methods
 ************************************************************************/
/*
 * NSS_CMSDigestedData_Create - create a digestedData object (presumably for encoding)
 *
 * version will be set by NSS_CMSDigestedData_Encode_BeforeStart
 * digestAlg is passed as parameter
 * contentInfo must be filled by the user
 * digest will be calculated while encoding
 */
extern NSSCMSDigestedData *
NSS_CMSDigestedData_Create(NSSCMSMessage *cmsg, SECAlgorithmID *digestalg);

/*
 * NSS_CMSDigestedData_Destroy - destroy a digestedData object
 */
extern void
NSS_CMSDigestedData_Destroy(NSSCMSDigestedData *digd);

/*
 * NSS_CMSDigestedData_GetContentInfo - return pointer to digestedData object's contentInfo
 */
extern NSSCMSContentInfo *
NSS_CMSDigestedData_GetContentInfo(NSSCMSDigestedData *digd);

/*
 * NSS_CMSDigestedData_Encode_BeforeStart - do all the necessary things to a DigestedData
 *     before encoding begins.
 *
 * In particular:
 *  - set the right version number. The contentInfo's content type must be set up already.
 */
extern SECStatus
NSS_CMSDigestedData_Encode_BeforeStart(NSSCMSDigestedData *digd);

/*
 * NSS_CMSDigestedData_Encode_BeforeData - do all the necessary things to a DigestedData
 *     before the encapsulated data is passed through the encoder.
 *
 * In detail:
 *  - set up the digests if necessary
 */
extern SECStatus
NSS_CMSDigestedData_Encode_BeforeData(NSSCMSDigestedData *digd);

/*
 * NSS_CMSDigestedData_Encode_AfterData - do all the necessary things to a DigestedData
 *     after all the encapsulated data was passed through the encoder.
 *
 * In detail:
 *  - finish the digests
 */
extern SECStatus
NSS_CMSDigestedData_Encode_AfterData(NSSCMSDigestedData *digd);

/*
 * NSS_CMSDigestedData_Decode_BeforeData - do all the necessary things to a DigestedData
 *     before the encapsulated data is passed through the encoder.
 *
 * In detail:
 *  - set up the digests if necessary
 */
extern SECStatus
NSS_CMSDigestedData_Decode_BeforeData(NSSCMSDigestedData *digd);

/*
 * NSS_CMSDigestedData_Decode_AfterData - do all the necessary things to a DigestedData
 *     after all the encapsulated data was passed through the encoder.
 *
 * In detail:
 *  - finish the digests
 */
extern SECStatus
NSS_CMSDigestedData_Decode_AfterData(NSSCMSDigestedData *digd);

/*
 * NSS_CMSDigestedData_Decode_AfterEnd - finalize a digestedData.
 *
 * In detail:
 *  - check the digests for equality
 */
extern SECStatus
NSS_CMSDigestedData_Decode_AfterEnd(NSSCMSDigestedData *digd);

/************************************************************************
 * cmsdigest.c - digestion routines
 ************************************************************************/

/*
 * NSS_CMSDigestContext_StartMultiple - start digest calculation using all the
 *  digest algorithms in "digestalgs" in parallel.
 */
extern NSSCMSDigestContext *
NSS_CMSDigestContext_StartMultiple(SECAlgorithmID **digestalgs);

/*
 * NSS_CMSDigestContext_StartSingle - same as NSS_CMSDigestContext_StartMultiple, but
 *  only one algorithm.
 */
extern NSSCMSDigestContext *
NSS_CMSDigestContext_StartSingle(SECAlgorithmID *digestalg);

/*
 * NSS_CMSDigestContext_Update - feed more data into the digest machine
 */
extern void
NSS_CMSDigestContext_Update(NSSCMSDigestContext *cmsdigcx, const unsigned char *data, int len);

/*
 * NSS_CMSDigestContext_Cancel - cancel digesting operation
 */
extern void
NSS_CMSDigestContext_Cancel(NSSCMSDigestContext *cmsdigcx);

/*
 * NSS_CMSDigestContext_FinishMultiple - finish the digests and put them
 *  into an array of SECItems (allocated on poolp)
 */
extern SECStatus
NSS_CMSDigestContext_FinishMultiple(NSSCMSDigestContext *cmsdigcx, PLArenaPool *poolp,
                                    SECItem ***digestsp);

/*
 * NSS_CMSDigestContext_FinishSingle - same as NSS_CMSDigestContext_FinishMultiple,
 *  but for one digest.
 */
extern SECStatus
NSS_CMSDigestContext_FinishSingle(NSSCMSDigestContext *cmsdigcx, PLArenaPool *poolp,
                                  SECItem *digest);

/************************************************************************
 *
 ************************************************************************/

/* shortcuts for basic use */

/*
 * NSS_CMSDEREncode - DER Encode a CMS message, with input being
 *                    the plaintext message and derOut being the output,
 *                    stored in arena's pool.
 */
extern SECStatus
NSS_CMSDEREncode(NSSCMSMessage *cmsg, SECItem *input, SECItem *derOut,
                 PLArenaPool *arena);

/************************************************************************
 *
 ************************************************************************/

/*
 *  define new S/MIME content type entries
 *
 *  S/MIME uses the builtin PKCS7 oid types for encoding and decoding the
 *  various S/MIME content. Some applications have their own content type
 *  which is different from the standard content type defined by S/MIME.
 *
 *  This function allows you to register new content types. There are basically
 *  Two different types of content, Wrappping content, and Data.
 *
 *  For data types, All the functions below can be zero or NULL excext
 *  type and is isData, which should be your oid tag and PR_FALSE respectively
 *
 *  For wrapping types, everything must be provided, or you will get encoder
 *  failures.
 *
 *  If NSS doesn't already define the OID that you need, you can register
 *  your own with SECOID_AddEntry.
 *
 *  Once you have defined your new content type, you can pass your new content
 *  type to NSS_CMSContentInfo_SetContent().
 *
 *  If you are using a wrapping type you can pass your own data structure in
 *  the ptr field, but it must contain and embedded NSSCMSGenericWrappingData
 *  structure as the first element. The size you pass to
 *  NSS_CMSType_RegisterContentType is the total size of your self defined
 *  data structure. NSS_CMSContentInfo_GetContent will return that data
 *  structure from the content info. Your ASN1Template will be evaluated
 *  against that data structure.
 */
SECStatus NSS_CMSType_RegisterContentType(SECOidTag type,
                                          SEC_ASN1Template *asn1Template, size_t size,
                                          NSSCMSGenericWrapperDataDestroy destroy,
                                          NSSCMSGenericWrapperDataCallback decode_before,
                                          NSSCMSGenericWrapperDataCallback decode_after,
                                          NSSCMSGenericWrapperDataCallback decode_end,
                                          NSSCMSGenericWrapperDataCallback encode_start,
                                          NSSCMSGenericWrapperDataCallback encode_before,
                                          NSSCMSGenericWrapperDataCallback encode_after,
                                          PRBool isData);

/************************************************************************/
SEC_END_PROTOS

#endif /* _CMS_H_ */
