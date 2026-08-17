/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef _HASH_H_
#define _HASH_H_

#include "seccomon.h"
#include "hasht.h"
#include "secoidt.h"

SEC_BEGIN_PROTOS

/*
** Generic hash api.
*/

extern unsigned int HASH_ResultLen(HASH_HashType type);

extern unsigned int HASH_ResultLenContext(HASHContext *context);

extern unsigned int HASH_ResultLenByOidTag(SECOidTag hashOid);

extern SECStatus HASH_HashBuf(HASH_HashType type,
                              unsigned char *dest,
                              const unsigned char *src,
                              PRUint32 src_len);

extern HASHContext *HASH_Create(HASH_HashType type);

extern HASHContext *HASH_Clone(HASHContext *context);

extern void HASH_Destroy(HASHContext *context);

extern void HASH_Begin(HASHContext *context);

extern void HASH_Update(HASHContext *context,
                        const unsigned char *src,
                        unsigned int len);

extern void HASH_End(HASHContext *context,
                     unsigned char *result,
                     unsigned int *result_len,
                     unsigned int max_result_len);

extern HASH_HashType HASH_GetType(HASHContext *context);

extern const SECHashObject *HASH_GetHashObject(HASH_HashType type);

extern const SECHashObject *HASH_GetHashObjectByOidTag(SECOidTag hashOid);

extern HASH_HashType HASH_GetHashTypeByOidTag(SECOidTag hashOid);
extern SECOidTag HASH_GetHashOidTagByHMACOidTag(SECOidTag hmacOid);
extern SECOidTag HASH_GetHMACOidTagByHashOidTag(SECOidTag hashOid);

extern SECOidTag HASH_GetHashOidTagByHashType(HASH_HashType type);

SEC_END_PROTOS

#endif /* _HASH_H_ */
