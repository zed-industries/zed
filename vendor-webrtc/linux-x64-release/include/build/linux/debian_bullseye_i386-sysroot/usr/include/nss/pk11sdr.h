/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef _PK11SDR_H_
#define _PK11SDR_H_

#include "seccomon.h"

SEC_BEGIN_PROTOS

/*
 * PK11SDR_Encrypt - encrypt data using the specified key id or SDR default
 * result should be freed with SECItem_ZfreeItem
 */
SECStatus
PK11SDR_Encrypt(SECItem *keyid, SECItem *data, SECItem *result, void *cx);

/*
 * PK11SDR_Decrypt - decrypt data previously encrypted with PK11SDR_Encrypt
 * result should be freed with SECItem_ZfreeItem
 */
SECStatus
PK11SDR_Decrypt(SECItem *data, SECItem *result, void *cx);

SEC_END_PROTOS

#endif
