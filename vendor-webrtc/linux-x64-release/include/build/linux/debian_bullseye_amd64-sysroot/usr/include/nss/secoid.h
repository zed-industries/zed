/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef _SECOID_H_
#define _SECOID_H_

#include "utilrename.h"

/*
 * secoid.h - public data structures and prototypes for ASN.1 OID functions
 */

#include "plarena.h"

#include "seccomon.h"
#include "secoidt.h"
#include "secasn1t.h"

SEC_BEGIN_PROTOS

extern const SEC_ASN1Template SECOID_AlgorithmIDTemplate[];

/* This functions simply returns the address of the above-declared template. */
SEC_ASN1_CHOOSER_DECLARE(SECOID_AlgorithmIDTemplate)

/*
 * OID handling routines
 */
extern SECOidData *SECOID_FindOID(const SECItem *oid);
extern SECOidTag SECOID_FindOIDTag(const SECItem *oid);
extern SECOidData *SECOID_FindOIDByTag(SECOidTag tagnum);
extern SECOidData *SECOID_FindOIDByMechanism(unsigned long mechanism);

/****************************************/
/*
** Algorithm id handling operations
*/

/*
** Fill in an algorithm-ID object given a tag and some parameters.
**      "aid" where the DER encoded algorithm info is stored (memory
**         is allocated)
**      "tag" the tag number defining the algorithm
**      "params" if not NULL, the parameters to go with the algorithm
*/
extern SECStatus SECOID_SetAlgorithmID(PLArenaPool *arena, SECAlgorithmID *aid,
                                       SECOidTag tag, SECItem *params);

/*
** Copy the "src" object to "dest". Memory is allocated in "dest" for
** each of the appropriate sub-objects. Memory in "dest" is not freed
** before memory is allocated (use SECOID_DestroyAlgorithmID(dest, PR_FALSE)
** to do that).
*/
extern SECStatus SECOID_CopyAlgorithmID(PLArenaPool *arena, SECAlgorithmID *dest,
                                        const SECAlgorithmID *src);

/*
** Get the tag number for the given algorithm-id object.
*/
extern SECOidTag SECOID_GetAlgorithmTag(const SECAlgorithmID *aid);

/*
** Destroy an algorithm-id object.
**      "aid" the certificate-request to destroy
**      "freeit" if PR_TRUE then free the object as well as its sub-objects
*/
extern void SECOID_DestroyAlgorithmID(SECAlgorithmID *aid, PRBool freeit);

/*
** Compare two algorithm-id objects, returning the difference between
** them.
*/
extern SECComparison SECOID_CompareAlgorithmID(SECAlgorithmID *a,
                                               SECAlgorithmID *b);

extern PRBool SECOID_KnownCertExtenOID(SECItem *extenOid);

/* Given a tag number, return a string describing it.
 */
extern const char *SECOID_FindOIDTagDescription(SECOidTag tagnum);

/* Add a dynamic SECOidData to the dynamic OID table.
** Routine copies the src entry, and returns the new SECOidTag.
** Returns SEC_OID_INVALID if failed to add for some reason.
*/
extern SECOidTag SECOID_AddEntry(const SECOidData *src);

/*
 * initialize the oid data structures.
 */
extern SECStatus SECOID_Init(void);

/*
 * free up the oid data structures.
 */
extern SECStatus SECOID_Shutdown(void);

/* if to->data is not NULL, and to->len is large enough to hold the result,
 * then the resultant OID will be copyed into to->data, and to->len will be
 * changed to show the actual OID length.
 * Otherwise, memory for the OID will be allocated (from the caller's
 * PLArenaPool, if pool is non-NULL) and to->data will receive the address
 * of the allocated data, and to->len will receive the OID length.
 * The original value of to->data is not freed when a new buffer is allocated.
 *
 * The input string may begin with "OID." and this still be ignored.
 * The length of the input string is given in len.  If len == 0, then
 * len will be computed as strlen(from), meaning it must be NUL terminated.
 * It is an error if from == NULL, or if *from == '\0'.
 */
extern SECStatus SEC_StringToOID(PLArenaPool *pool, SECItem *to,
                                 const char *from, PRUint32 len);

extern void UTIL_SetForkState(PRBool forked);

/*
 * Accessor functions for new opaque extended SECOID table.
 * Any of these functions may return SECSuccess or SECFailure with the error
 * code set to SEC_ERROR_UNKNOWN_OBJECT_TYPE if the SECOidTag is out of range.
 */

/* The Get function outputs the 32-bit value associated with the SECOidTag.
 * Flags bits are the NSS_USE_ALG_ #defines in "secoidt.h".
 * Default value for any algorithm is 0xffffffff (enabled for all purposes).
 * No value is output if function returns SECFailure.
 */
extern SECStatus NSS_GetAlgorithmPolicy(SECOidTag tag, PRUint32 *pValue);

/* The Set function modifies the stored value according to the following
 * algorithm:
 *   policy[tag] = (policy[tag] & ~clearBits) | setBits;
 */
extern SECStatus
NSS_SetAlgorithmPolicy(SECOidTag tag, PRUint32 setBits, PRUint32 clearBits);

/* Lock the policy so NSS_SetAlgorithmPolicy (and other policy functions)
 * No longer function */
void
NSS_LockPolicy(void);

/* return true if policy changes are now locked out */
PRBool
NSS_IsPolicyLocked(void);

SEC_END_PROTOS

#endif /* _SECOID_H_ */
