/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/*
 * Header file for routines specific to S/MIME.  Keep things that are pure
 * pkcs7 out of here; this is for S/MIME policy, S/MIME interoperability, etc.
 */

#ifndef _SECMIME_H_
#define _SECMIME_H_ 1

#include "secpkcs7.h"

/************************************************************************/
SEC_BEGIN_PROTOS

/*
 * Initialize the local recording of the user S/MIME cipher preferences.
 * This function is called once for each cipher, the order being
 * important (first call records greatest preference, and so on).
 * When finished, it is called with a "which" of CIPHER_FAMILID_MASK.
 * If the function is called again after that, it is assumed that
 * the preferences are being reset, and the old preferences are
 * discarded.
 *
 * XXX This is for a particular user, and right now the storage is
 * XXX local, static.  The preference should be stored elsewhere to allow
 * XXX for multiple uses of one library?  How does SSL handle this;
 * XXX it has something similar?
 *
 *  - The "which" values are defined in ciferfam.h (the SMIME_* values,
 *    for example SMIME_DES_CBC_56).
 *  - If "on" is non-zero then the named cipher is enabled, otherwise
 *    it is disabled.  (It is not necessary to call the function for
 *    ciphers that are disabled, however, as that is the default.)
 *
 * If the cipher preference is successfully recorded, SECSuccess
 * is returned.  Otherwise SECFailure is returned.  The only errors
 * are due to failure allocating memory or bad parameters/calls:
 *	SEC_ERROR_XXX ("which" is not in the S/MIME cipher family)
 *	SEC_ERROR_XXX (function is being called more times than there
 *		are known/expected ciphers)
 */
extern SECStatus SECMIME_EnableCipher(long which, int on);

/*
 * Initialize the local recording of the S/MIME policy.
 * This function is called to enable/disable a particular cipher.
 * (S/MIME encryption or decryption using a particular cipher is only
 * allowed if that cipher is currently enabled.)  At startup, all S/MIME
 * ciphers are disabled.  From that point, this function can be called
 * to enable a cipher -- it is not necessary to call this to disable
 * a cipher unless that cipher was previously, explicitly enabled via
 * this function.
 *
 * XXX This is for a the current module, I think, so local, static storage
 * XXX is okay.  Is that correct, or could multiple uses of the same
 * XXX library expect to operate under different policies?
 *
 *  - The "which" values are defined in ciferfam.h (the SMIME_* values,
 *    for example SMIME_DES_CBC_56).
 *  - If "on" is non-zero then the named cipher is enabled, otherwise
 *    it is disabled.
 *
 * If the cipher is successfully enabled/disabled, SECSuccess is
 * returned.  Otherwise SECFailure is returned.  The only errors
 * are due to bad parameters:
 *	SEC_ERROR_XXX ("which" is not in the S/MIME cipher family)
 *	SEC_ERROR_XXX ("which" exceeds expected maximum cipher; this is
 *		really an internal error)
 */
extern SECStatus SECMIME_SetPolicy(long which, int on);

/*
 * Does the current policy allow S/MIME decryption of this particular
 * algorithm and keysize?
 */
extern PRBool SECMIME_DecryptionAllowed(SECAlgorithmID *algid, PK11SymKey *key);

/*
 * Does the current policy allow *any* S/MIME encryption (or decryption)?
 *
 * This tells whether or not *any* S/MIME encryption can be done,
 * according to policy.  Callers may use this to do nicer user interface
 * (say, greying out a checkbox so a user does not even try to encrypt
 * a message when they are not allowed to) or for any reason they want
 * to check whether S/MIME encryption (or decryption, for that matter)
 * may be done.
 *
 * It takes no arguments.  The return value is a simple boolean:
 *   PR_TRUE means encryption (or decryption) is *possible*
 *	(but may still fail due to other reasons, like because we cannot
 *	find all the necessary certs, etc.; PR_TRUE is *not* a guarantee)
 *   PR_FALSE means encryption (or decryption) is not permitted
 *
 * There are no errors from this routine.
 */
extern PRBool SECMIME_EncryptionPossible(void);

/*
 * Start an S/MIME encrypting context.
 *
 * "scert" is the cert for the sender.  It will be checked for validity.
 * "rcerts" are the certs for the recipients.  They will also be checked.
 *
 * "certdb" is the cert database to use for verifying the certs.
 * It can be NULL if a default database is available (like in the client).
 *
 * This function already does all of the stuff specific to S/MIME protocol
 * and local policy; the return value just needs to be passed to
 * SEC_PKCS7Encode() or to SEC_PKCS7EncoderStart() to create the encoded data,
 * and finally to SEC_PKCS7DestroyContentInfo().
 *
 * An error results in a return value of NULL and an error set.
 * (Retrieve specific errors via PORT_GetError()/XP_GetError().)
 */
extern SEC_PKCS7ContentInfo *SECMIME_CreateEncrypted(CERTCertificate *scert,
                                                     CERTCertificate **rcerts,
                                                     CERTCertDBHandle *certdb,
                                                     SECKEYGetPasswordKey pwfn,
                                                     void *pwfn_arg);

/*
 * Start an S/MIME signing context.
 *
 * "scert" is the cert that will be used to sign the data.  It will be
 * checked for validity.
 *
 * "certdb" is the cert database to use for verifying the cert.
 * It can be NULL if a default database is available (like in the client).
 *
 * "digestalg" names the digest algorithm.  (It should be SEC_OID_SHA1;
 * XXX There should be SECMIME functions for hashing, or the hashing should
 * be built into this interface, which we would like because we would
 * support more smartcards that way, and then this argument should go away.)
 *
 * "digest" is the actual digest of the data.  It must be provided in
 * the case of detached data or NULL if the content will be included.
 *
 * This function already does all of the stuff specific to S/MIME protocol
 * and local policy; the return value just needs to be passed to
 * SEC_PKCS7Encode() or to SEC_PKCS7EncoderStart() to create the encoded data,
 * and finally to SEC_PKCS7DestroyContentInfo().
 *
 * An error results in a return value of NULL and an error set.
 * (Retrieve specific errors via PORT_GetError()/XP_GetError().)
 */
extern SEC_PKCS7ContentInfo *SECMIME_CreateSigned(CERTCertificate *scert,
                                                  CERTCertificate *ecert,
                                                  CERTCertDBHandle *certdb,
                                                  SECOidTag digestalg,
                                                  SECItem *digest,
                                                  SECKEYGetPasswordKey pwfn,
                                                  void *pwfn_arg);

/************************************************************************/
SEC_END_PROTOS

#endif /* _SECMIME_H_ */
