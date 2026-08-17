/*
 * cryptohi.h - public prototypes for the crypto library
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef _CRYPTOHI_H_
#define _CRYPTOHI_H_

#include "blapit.h"

#include "seccomon.h"
#include "secoidt.h"
#include "secdert.h"
#include "cryptoht.h"
#include "keythi.h"
#include "certt.h"

SEC_BEGIN_PROTOS

/****************************************/
/*
** DER encode/decode (EC)DSA signatures
*/

/* ANSI X9.57 defines DSA signatures as DER encoded data.  Our DSA1 code (and
 * most of the rest of the world) just generates 40 bytes of raw data.  These
 * functions convert between formats.
 */
extern SECStatus DSAU_EncodeDerSig(SECItem *dest, SECItem *src);
extern SECItem *DSAU_DecodeDerSig(const SECItem *item);

/*
 * Unlike DSA1, raw DSA2 and ECDSA signatures do not have a fixed length.
 * Rather they contain two integers r and s whose length depends
 * on the size of q or the EC key used for signing.
 *
 * We can reuse the DSAU_EncodeDerSig interface to DER encode
 * raw ECDSA signature keeping in mind that the length of r
 * is the same as that of s and exactly half of src->len.
 *
 * For decoding, we need to pass the length of the desired
 * raw signature (twice the key size) explicitly.
 */
extern SECStatus DSAU_EncodeDerSigWithLen(SECItem *dest, SECItem *src,
                                          unsigned int len);
extern SECItem *DSAU_DecodeDerSigToLen(const SECItem *item, unsigned int len);

/****************************************/
/*
** Signature creation operations
*/

/*
** Create a new signature context used for signing a data stream.
**      "alg" the signature algorithm to use (e.g. SEC_OID_PKCS1_MD5_WITH_RSA_ENCRYPTION)
**	"privKey" the private key to use
*/
extern SGNContext *SGN_NewContext(SECOidTag alg, SECKEYPrivateKey *privKey);

/*
** Create a new signature context from an algorithmID.
**      "alg" the signature algorithm to use
**	"privKey" the private key to use
*/
extern SGNContext *SGN_NewContextWithAlgorithmID(SECAlgorithmID *alg,
                                                 SECKEYPrivateKey *privKey);

/*
** Destroy a signature-context object
**	"cx" the object
**	"freeit" if PR_TRUE then free the object as well as its sub-objects
*/
extern void SGN_DestroyContext(SGNContext *cx, PRBool freeit);

/*
** Reset the signing context "cx" to its initial state, preparing it for
** another stream of data.
*/
extern SECStatus SGN_Begin(SGNContext *cx);

/*
** Update the signing context with more data to sign.
**	"cx" the context
**	"input" the input data to sign
**	"inputLen" the length of the input data
*/
extern SECStatus SGN_Update(SGNContext *cx, const unsigned char *input,
                            unsigned int inputLen);

/*
** Finish the signature process. Use either k0 or k1 to sign the data
** stream that was input using SGN_Update. The resulting signature is
** formatted using PKCS#1 and then encrypted using RSA private or public
** encryption.
**	"cx" the context
**	"result" the final signature data (memory is allocated)
*/
extern SECStatus SGN_End(SGNContext *cx, SECItem *result);

/*
** Sign a single block of data using private key encryption and given
** signature/hash algorithm.
**	"result" the final signature data (memory is allocated)
**	"buf" the input data to sign
**	"len" the amount of data to sign
**	"pk" the private key to encrypt with
**	"algid" the signature/hash algorithm to sign with
**		(must be compatible with the key type).
*/
extern SECStatus SEC_SignData(SECItem *result,
                              const unsigned char *buf, int len,
                              SECKEYPrivateKey *pk, SECOidTag algid);

/*
** Sign a single block of data using private key encryption and given
** signature/hash algorithm with parameters from an algorithmID.
**	"result" the final signature data (memory is allocated)
**	"buf" the input data to sign
**	"len" the amount of data to sign
**	"pk" the private key to encrypt with
**	"algid" the signature/hash algorithm to sign with
**		(must be compatible with the key type).
*/
extern SECStatus SEC_SignDataWithAlgorithmID(SECItem *result,
                                             const unsigned char *buf, int len,
                                             SECKEYPrivateKey *pk,
                                             SECAlgorithmID *algid);

/*
** Sign a pre-digested block of data using private key encryption, encoding
**  The given signature/hash algorithm.
**	"result" the final signature data (memory is allocated)
**	"digest" the digest to sign
**	"privKey" the private key to encrypt with
**	"algtag" The algorithm tag to encode (need for RSA only)
*/
extern SECStatus SGN_Digest(SECKEYPrivateKey *privKey,
                            SECOidTag algtag, SECItem *result, SECItem *digest);

/*
** DER sign a single block of data using private key encryption and the
** MD5 hashing algorithm. This routine first computes a digital signature
** using SEC_SignData, then wraps it with an CERTSignedData and then der
** encodes the result.
**	"arena" is the memory arena to use to allocate data from
** 	"result" the final der encoded data (memory is allocated)
** 	"buf" the input data to sign
** 	"len" the amount of data to sign
** 	"pk" the private key to encrypt with
*/
extern SECStatus SEC_DerSignData(PLArenaPool *arena, SECItem *result,
                                 const unsigned char *buf, int len,
                                 SECKEYPrivateKey *pk, SECOidTag algid);

/*
** DER sign a single block of data using private key encryption and
** the given signature/hash algorithm with parameters from an
** algorithmID. This routine first computes a digital signature using
** SEC_SignData, then wraps it with an CERTSignedData and then der
** encodes the result.
**	"arena" is the memory arena to use to allocate data from
** 	"result" the final der encoded data (memory is allocated)
** 	"buf" the input data to sign
** 	"len" the amount of data to sign
** 	"pk" the private key to encrypt with
**	"algid" the signature/hash algorithm to sign with
**		(must be compatible with the key type).
*/
extern SECStatus SEC_DerSignDataWithAlgorithmID(PLArenaPool *arena,
                                                SECItem *result,
                                                const unsigned char *buf,
                                                int len,
                                                SECKEYPrivateKey *pk,
                                                SECAlgorithmID *algid);

/*
** Destroy a signed-data object.
**	"sd" the object
**	"freeit" if PR_TRUE then free the object as well as its sub-objects
*/
extern void SEC_DestroySignedData(CERTSignedData *sd, PRBool freeit);

/*
** Get the signature algorithm tag number for the given key type and hash
** algorithm tag. Returns SEC_OID_UNKNOWN if key type and hash algorithm
** do not match or are not supported.
*/
extern SECOidTag SEC_GetSignatureAlgorithmOidTag(KeyType keyType,
                                                 SECOidTag hashAlgTag);

/*
** Create algorithm parameters for signing. Return a new item
** allocated from arena, or NULL on failure.
**	"arena" is the memory arena to use to allocate data from
**	"result" the encoded parameters (memory is allocated)
**	"signAlgTag" is the signing algorithm
**	"hashAlgTag" is the preferred hash algorithm
**	"params" is the default parameters
**	"key" is the private key
*/
extern SECItem *SEC_CreateSignatureAlgorithmParameters(PLArenaPool *arena,
                                                       SECItem *result,
                                                       SECOidTag signAlgTag,
                                                       SECOidTag hashAlgTag,
                                                       const SECItem *params,
                                                       const SECKEYPrivateKey *key);

/****************************************/
/*
** Signature verification operations
*/

/*
** Create a signature verification context. This version is deprecated,
**  This function is deprecated. Use VFY_CreateContextDirect or
**  VFY_CreateContextWithAlgorithmID instead.
**	"key" the public key to verify with
**	"sig" the encrypted signature data if sig is NULL then
**	   VFY_EndWithSignature must be called with the correct signature at
**	   the end of the processing.
**	"sigAlg" specifies the signing algorithm to use (including the
**         hash algorthim).  This must match the key type.
**	"wincx" void pointer to the window context
*/
extern VFYContext *VFY_CreateContext(SECKEYPublicKey *key, SECItem *sig,
                                     SECOidTag sigAlg, void *wincx);
/*
** Create a signature verification context.
**	"key" the public key to verify with
**	"sig" the encrypted signature data if sig is NULL then
**	   VFY_EndWithSignature must be called with the correct signature at
**	   the end of the processing.
**	"pubkAlg" specifies the cryptographic signing algorithm to use (the
**         raw algorithm without any hash specified.  This must match the key
**         type.
**	"hashAlg" specifies the hashing algorithm used. If the key is an
**	   RSA key, and sig is not NULL, then hashAlg can be SEC_OID_UNKNOWN.
**	   the hash is selected from data in the sig.
**	"hash" optional pointer to return the actual hash algorithm used.
**	   in practice this should always match the passed in hashAlg (the
**	   exception is the case where hashAlg is SEC_OID_UNKNOWN above).
**         If this value is NULL no, hash oid is returned.
**	"wincx" void pointer to the window context
*/
extern VFYContext *VFY_CreateContextDirect(const SECKEYPublicKey *key,
                                           const SECItem *sig,
                                           SECOidTag pubkAlg,
                                           SECOidTag hashAlg,
                                           SECOidTag *hash, void *wincx);
/*
** Create a signature verification context from a algorithm ID.
**	"key" the public key to verify with
**	"sig" the encrypted signature data if sig is NULL then
**	   VFY_EndWithSignature must be called with the correct signature at
**	   the end of the processing.
**	"algid" specifies the signing algorithm and parameters to use.
**         This must match the key type.
**      "hash" optional pointer to return the oid of the actual hash used in
**         the signature. If this value is NULL no, hash oid is returned.
**	"wincx" void pointer to the window context
*/
extern VFYContext *VFY_CreateContextWithAlgorithmID(const SECKEYPublicKey *key,
                                                    const SECItem *sig,
                                                    const SECAlgorithmID *algid,
                                                    SECOidTag *hash,
                                                    void *wincx);

/*
** Destroy a verification-context object.
**	"cx" the context to destroy
**	"freeit" if PR_TRUE then free the object as well as its sub-objects
*/
extern void VFY_DestroyContext(VFYContext *cx, PRBool freeit);

extern SECStatus VFY_Begin(VFYContext *cx);

/*
** Update a verification context with more input data. The input data
** is fed to a secure hash function (depending on what was in the
** encrypted signature data).
**	"cx" the context
**	"input" the input data
**	"inputLen" the amount of input data
*/
extern SECStatus VFY_Update(VFYContext *cx, const unsigned char *input,
                            unsigned int inputLen);

/*
** Finish the verification process. The return value is a status which
** indicates success or failure. On success, the SECSuccess value is
** returned. Otherwise, SECFailure is returned and the error code found
** using PORT_GetError() indicates what failure occurred.
** 	"cx" the context
*/
extern SECStatus VFY_End(VFYContext *cx);

/*
** Finish the verification process. The return value is a status which
** indicates success or failure. On success, the SECSuccess value is
** returned. Otherwise, SECFailure is returned and the error code found
** using PORT_GetError() indicates what failure occurred. If signature is
** supplied the verification uses this signature to verify, otherwise the
** signature passed in VFY_CreateContext() is used.
** VFY_EndWithSignature(cx,NULL); is identical to VFY_End(cx);.
** 	"cx" the context
** 	"sig" the encrypted signature data
*/
extern SECStatus VFY_EndWithSignature(VFYContext *cx, SECItem *sig);

/*
** Verify the signature on a block of data for which we already have
** the digest. The signature data is an RSA private key encrypted
** block of data formatted according to PKCS#1.
**  This function is deprecated. Use VFY_VerifyDigestDirect or
**  VFY_VerifyDigestWithAlgorithmID instead.
** 	"dig" the digest
** 	"key" the public key to check the signature with
** 	"sig" the encrypted signature data
**	"sigAlg" specifies the signing algorithm to use.  This must match
**	    the key type.
**	"wincx" void pointer to the window context
**/
extern SECStatus VFY_VerifyDigest(SECItem *dig, SECKEYPublicKey *key,
                                  SECItem *sig, SECOidTag sigAlg, void *wincx);
/*
** Verify the signature on a block of data for which we already have
** the digest. The signature data is an RSA private key encrypted
** block of data formatted according to PKCS#1.
** 	"dig" the digest
** 	"key" the public key to check the signature with
** 	"sig" the encrypted signature data
**	"pubkAlg" specifies the cryptographic signing algorithm to use (the
**         raw algorithm without any hash specified.  This must match the key
**         type.
**	"hashAlg" specifies the hashing algorithm used.
**	"wincx" void pointer to the window context
**/
extern SECStatus VFY_VerifyDigestDirect(const SECItem *dig,
                                        const SECKEYPublicKey *key,
                                        const SECItem *sig, SECOidTag pubkAlg,
                                        SECOidTag hashAlg, void *wincx);
/*
** Verify the signature on a block of data for which we already have
** the digest. The signature data is an RSA private key encrypted
** block of data formatted according to PKCS#1.
**	"key" the public key to verify with
**	"sig" the encrypted signature data if sig is NULL then
**	   VFY_EndWithSignature must be called with the correct signature at
**	   the end of the processing.
**	"algid" specifies the signing algorithm and parameters to use.
**         This must match the key type.
**      "hash" oid of the actual hash used to create digest. If this  value is
**         not set to SEC_OID_UNKNOWN, it must match the hash of the signature.
**	"wincx" void pointer to the window context
*/
extern SECStatus VFY_VerifyDigestWithAlgorithmID(const SECItem *dig,
                                                 const SECKEYPublicKey *key, const SECItem *sig,
                                                 const SECAlgorithmID *algid, SECOidTag hash,
                                                 void *wincx);

/*
** Verify the signature on a block of data. The signature data is an RSA
** private key encrypted block of data formatted according to PKCS#1.
**   This function is deprecated. Use VFY_VerifyDataDirect or
**   VFY_VerifyDataWithAlgorithmID instead.
** 	"buf" the input data
** 	"len" the length of the input data
** 	"key" the public key to check the signature with
** 	"sig" the encrypted signature data
**	"sigAlg" specifies the signing algorithm to use.  This must match
**	    the key type.
**	"wincx" void pointer to the window context
*/
extern SECStatus VFY_VerifyData(const unsigned char *buf, int len,
                                const SECKEYPublicKey *key, const SECItem *sig,
                                SECOidTag sigAlg, void *wincx);
/*
** Verify the signature on a block of data. The signature data is an RSA
** private key encrypted block of data formatted according to PKCS#1.
** 	"buf" the input data
** 	"len" the length of the input data
** 	"key" the public key to check the signature with
** 	"sig" the encrypted signature data
**	"pubkAlg" specifies the cryptographic signing algorithm to use (the
**         raw algorithm without any hash specified.  This must match the key
**         type.
**	"hashAlg" specifies the hashing algorithm used. If the key is an
**	   RSA key, and sig is not NULL, then hashAlg can be SEC_OID_UNKNOWN.
**	   the hash is selected from data in the sig.
**	"hash" optional pointer to return the actual hash algorithm used.
**	   in practice this should always match the passed in hashAlg (the
**	   exception is the case where hashAlg is SEC_OID_UNKNOWN above).
**         If this value is NULL no, hash oid is returned.
**	"wincx" void pointer to the window context
*/
extern SECStatus VFY_VerifyDataDirect(const unsigned char *buf, int len,
                                      const SECKEYPublicKey *key,
                                      const SECItem *sig,
                                      SECOidTag pubkAlg, SECOidTag hashAlg,
                                      SECOidTag *hash, void *wincx);

/*
** Verify the signature on a block of data. The signature data is an RSA
** private key encrypted block of data formatted according to PKCS#1.
** 	"buf" the input data
** 	"len" the length of the input data
** 	"key" the public key to check the signature with
** 	"sig" the encrypted signature data
**	"algid" specifies the signing algorithm and parameters to use.
**         This must match the key type.
**      "hash" optional pointer to return the oid of the actual hash used in
**         the signature. If this value is NULL no, hash oid is returned.
**	"wincx" void pointer to the window context
*/
extern SECStatus VFY_VerifyDataWithAlgorithmID(const unsigned char *buf,
                                               int len, const SECKEYPublicKey *key,
                                               const SECItem *sig,
                                               const SECAlgorithmID *algid, SECOidTag *hash,
                                               void *wincx);

SEC_END_PROTOS

#endif /* _CRYPTOHI_H_ */
