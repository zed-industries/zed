/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef _PKCS12_H_
#define _PKCS12_H_

#include "pkcs12t.h"
#include "p12.h"

SEC_BEGIN_PROTOS

typedef SECItem *(*SEC_PKCS12GetPassword)(void *arg);

/* Decode functions */
/* Import a PFX item.
 *      der_pfx is the der-encoded pfx item to import.
 *      pbef, and pbefarg are used to retrieve passwords for the HMAC,
 *          and any passwords needed for passing to PKCS5 encryption
 *          routines.
 *      algorithm is the algorithm by which private keys are stored in
 *          the key database.  this could be a specific algorithm or could
 *          be based on a global setting.
 *      slot is the slot to where the certificates will be placed.  if NULL,
 *          the internal key slot is used.
 * If the process is successful, a SECSuccess is returned, otherwise
 * a failure occurred.
 */
SECStatus
SEC_PKCS12PutPFX(SECItem *der_pfx, SECItem *pwitem,
                 SEC_PKCS12NicknameCollisionCallback ncCall,
                 PK11SlotInfo *slot, void *wincx);

/* check the first two bytes of a file to make sure that it matches
 * the desired header for a PKCS 12 file
 */
PRBool SEC_PKCS12ValidData(char *buf, int bufLen, long int totalLength);

SEC_END_PROTOS

#endif
