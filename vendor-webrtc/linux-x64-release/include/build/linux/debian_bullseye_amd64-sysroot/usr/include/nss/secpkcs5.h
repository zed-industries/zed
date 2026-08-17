/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
#ifndef _SECPKCS5_H_
#define _SECPKCS5_H_
#include "seccomon.h"
#include "secmodt.h"

/* used for V2 PKCS 12 Draft Spec */
typedef enum {
    pbeBitGenIDNull = 0,
    pbeBitGenCipherKey = 0x01,
    pbeBitGenCipherIV = 0x02,
    pbeBitGenIntegrityKey = 0x03
} PBEBitGenID;

typedef struct PBEBitGenContextStr PBEBitGenContext;

SEC_BEGIN_PROTOS

/* private */
SECAlgorithmID *
sec_pkcs5CreateAlgorithmID(SECOidTag algorithm, SECOidTag cipherAlgorithm,
                           SECOidTag prfAlg, SECOidTag *pPbeAlgorithm,
                           int keyLengh, SECItem *salt, int iteration);

/* Get the initialization vector.  The password is passed in, hashing
 * is performed, and the initialization vector is returned.
 *  algid is a pointer to a PBE algorithm ID
 *  pwitem is the password
 * If an error occurs or the algorithm id is not a PBE algrithm,
 * NULL is returned.  Otherwise, the iv is returned in a secitem.
 */
SECItem *
SEC_PKCS5GetIV(SECAlgorithmID *algid, SECItem *pwitem, PRBool faulty3DES);

SECOidTag SEC_PKCS5GetCryptoAlgorithm(SECAlgorithmID *algid);
PRBool SEC_PKCS5IsAlgorithmPBEAlg(SECAlgorithmID *algid);
PRBool SEC_PKCS5IsAlgorithmPBEAlgTag(SECOidTag algTag);
SECOidTag SEC_PKCS5GetPBEAlgorithm(SECOidTag algTag, int keyLen);
int SEC_PKCS5GetKeyLength(SECAlgorithmID *algid);

/**********************************************************************
 * Deprecated PBE functions.  Use the PBE functions in pk11func.h
 * instead.
 **********************************************************************/

PBEBitGenContext *
PBE_CreateContext(SECOidTag hashAlgorithm, PBEBitGenID bitGenPurpose,
                  SECItem *pwitem, SECItem *salt, unsigned int bitsNeeded,
                  unsigned int iterations);

void
PBE_DestroyContext(PBEBitGenContext *context);

SECItem *
PBE_GenerateBits(PBEBitGenContext *context);

SEC_END_PROTOS

#endif /* _SECPKS5_H_ */
