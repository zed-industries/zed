/*
 * secdig.h - public prototypes for digest-info functions
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef _SECDIG_H_
#define _SECDIG_H_

#include "utilrename.h"
#include "secdigt.h"

#include "seccomon.h"
#include "secasn1t.h"
#include "secdert.h"

SEC_BEGIN_PROTOS

extern const SEC_ASN1Template sgn_DigestInfoTemplate[];

SEC_ASN1_CHOOSER_DECLARE(sgn_DigestInfoTemplate)

/****************************************/
/*
** Digest-info functions
*/

/*
** Create a new digest-info object
**  "algorithm" one of SEC_OID_MD2, SEC_OID_MD5, or SEC_OID_SHA1
**  "sig" the raw signature data (from MD2 or MD5)
**  "sigLen" the length of the signature data
**
** NOTE: this is a low level routine used to prepare some data for PKCS#1
** digital signature formatting.
**
** XXX It might be nice to combine the create and encode functions.
** I think that is all anybody ever wants to do anyway.
*/
extern SGNDigestInfo *SGN_CreateDigestInfo(SECOidTag algorithm,
                                           const unsigned char *sig,
                                           unsigned int sigLen);

/*
** Destroy a digest-info object
*/
extern void SGN_DestroyDigestInfo(SGNDigestInfo *info);

/*
** Encode a digest-info object
**  "poolp" is where to allocate the result from; it can be NULL in
**      which case generic heap allocation (XP_ALLOC) will be used
**  "dest" is where to store the result; it can be NULL, in which case
**      it will be allocated (from poolp or heap, as explained above)
**  "diginfo" is the object to be encoded
** The return value is NULL if any error occurred, otherwise it is the
** resulting SECItem (either allocated or the same as the "dest" parameter).
**
** XXX It might be nice to combine the create and encode functions.
** I think that is all anybody ever wants to do anyway.
*/
extern SECItem *SGN_EncodeDigestInfo(PLArenaPool *poolp, SECItem *dest,
                                     SGNDigestInfo *diginfo);

/*
** Decode a DER encoded digest info objct.
**  didata is thr source of the encoded digest.
** The return value is NULL if an error occurs.  Otherwise, a
** digest info object which is allocated within it's own
** pool is returned.  The digest info should be deleted
** by later calling SGN_DestroyDigestInfo.
*/
extern SGNDigestInfo *SGN_DecodeDigestInfo(SECItem *didata);

/*
** Copy digest info.
**  poolp is the arena to which the digest will be copied.
**  a is the destination digest, it must be non-NULL.
**  b is the source digest
** This function is for copying digests.  It allows digests
** to be copied into a specified pool.  If the digest is in
** the same pool as other data, you do not want to delete
** the digest by calling SGN_DestroyDigestInfo.
** A return value of SECFailure indicates an error.  A return
** of SECSuccess indicates no error occurred.
*/
extern SECStatus SGN_CopyDigestInfo(PLArenaPool *poolp,
                                    SGNDigestInfo *a,
                                    SGNDigestInfo *b);

/*
** Compare two digest-info objects, returning the difference between
** them.
*/
extern SECComparison SGN_CompareDigestInfo(SGNDigestInfo *a, SGNDigestInfo *b);

SEC_END_PROTOS

#endif /* _SECDIG_H_ */
