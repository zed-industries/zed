/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/*
 * base64.h - prototypes for base64 encoding/decoding
 * Note: These functions are deprecated; see nssb64.h for new routines.
 */
#ifndef _BASE64_H_
#define _BASE64_H_

#include "utilrename.h"
#include "seccomon.h"

SEC_BEGIN_PROTOS

/*
** Return an PORT_Alloc'd ascii string which is the base64 encoded
** version of the input string.
*/
extern char *BTOA_DataToAscii(const unsigned char *data, unsigned int len);

/*
** Return an PORT_Alloc'd string which is the base64 decoded version
** of the input string; set *lenp to the length of the returned data.
*/
extern unsigned char *ATOB_AsciiToData(const char *string, unsigned int *lenp);

/*
** Convert from ascii to binary encoding of an item.
*/
extern SECStatus ATOB_ConvertAsciiToItem(SECItem *binary_item, const char *ascii);

/*
** Convert from binary encoding of an item to ascii.
*/
extern char *BTOA_ConvertItemToAscii(SECItem *binary_item);

SEC_END_PROTOS

#endif /* _BASE64_H_ */
