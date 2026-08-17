/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef _SECDER_H_
#define _SECDER_H_

#include "utilrename.h"

/*
 * secder.h - public data structures and prototypes for the DER encoding and
 *        decoding utilities library
 */

#include <time.h>

#include "plarena.h"
#include "prlong.h"

#include "seccomon.h"
#include "secdert.h"
#include "prtime.h"

SEC_BEGIN_PROTOS

/*
** Encode a data structure into DER.
**  "dest" will be filled in (and memory allocated) to hold the der
**     encoded structure in "src"
**  "t" is a template structure which defines the shape of the
**     stored data
**  "src" is a pointer to the structure that will be encoded
*/
extern SECStatus DER_Encode(PLArenaPool *arena, SECItem *dest, DERTemplate *t,
                            void *src);

/*
** This function is deprecated.
*/
extern SECStatus DER_Lengths(SECItem *item, int *header_len_p,
                             PRUint32 *contents_len_p);

/*
** Lower level der subroutine that stores the standard header into "to".
** The header is of variable length, based on encodingLen.
** The return value is the new value of "to" after skipping over the header.
**  "to" is where the header will be stored
**  "code" is the der code to write
**  "encodingLen" is the number of bytes of data that will follow
**     the header
*/
extern unsigned char *DER_StoreHeader(unsigned char *to, unsigned int code,
                                      PRUint32 encodingLen);

/*
** Return the number of bytes it will take to hold a der encoded length.
*/
extern int DER_LengthLength(PRUint32 len);

/*
** Store a der encoded *signed* integer (whose value is "src") into "dst".
** XXX This should really be enhanced to take a long.
*/
extern SECStatus DER_SetInteger(PLArenaPool *arena, SECItem *dst, PRInt32 src);

/*
** Store a der encoded *unsigned* integer (whose value is "src") into "dst".
** XXX This should really be enhanced to take an unsigned long.
*/
extern SECStatus DER_SetUInteger(PLArenaPool *arena, SECItem *dst, PRUint32 src);

/*
** Decode a der encoded *signed* integer that is stored in "src".
** If "-1" is returned, then the caller should check the error in
** XP_GetError() to see if an overflow occurred (SEC_ERROR_BAD_DER).
*/
extern long DER_GetInteger(const SECItem *src);

/*
** Decode a der encoded *unsigned* integer that is stored in "src".
** If the ULONG_MAX is returned, then the caller should check the error
** in XP_GetError() to see if an overflow occurred (SEC_ERROR_BAD_DER).
*/
extern unsigned long DER_GetUInteger(SECItem *src);

/*
** Convert an NSPR time value to a der encoded time value.
**  "result" is the der encoded time (memory is allocated)
**  "time" is the NSPR time value (Since Jan 1st, 1970).
**      time must be on or after January 1, 1950, and
**      before January 1, 2050
** The caller is responsible for freeing up the buffer which
** result->data points to upon a successful operation.
*/
extern SECStatus DER_TimeToUTCTime(SECItem *result, PRTime time);
extern SECStatus DER_TimeToUTCTimeArena(PLArenaPool *arenaOpt,
                                        SECItem *dst, PRTime gmttime);

/*
** Convert an ascii encoded time value (according to DER rules) into
** an NSPR time value.
**  "result" the resulting NSPR time
**  "string" the der notation ascii value to decode
*/
extern SECStatus DER_AsciiToTime(PRTime *result, const char *string);

/*
** Same as DER_AsciiToTime except takes an SECItem instead of a string
*/
extern SECStatus DER_UTCTimeToTime(PRTime *result, const SECItem *time);

/*
** Convert a DER encoded UTC time to an ascii time representation
** "utctime" is the DER encoded UTC time to be converted. The
** caller is responsible for deallocating the returned buffer.
*/
extern char *DER_UTCTimeToAscii(SECItem *utcTime);

/*
** Convert a DER encoded UTC time to an ascii time representation, but only
** include the day, not the time.
**  "utctime" is the DER encoded UTC time to be converted.
** The caller is responsible for deallocating the returned buffer.
*/
extern char *DER_UTCDayToAscii(SECItem *utctime);
/* same thing for DER encoded GeneralizedTime */
extern char *DER_GeneralizedDayToAscii(SECItem *gentime);
/* same thing for either DER UTCTime or GeneralizedTime */
extern char *DER_TimeChoiceDayToAscii(SECItem *timechoice);

/*
** Convert a PRTime to a DER encoded Generalized time
** gmttime must be on or after January 1, year 1 and
** before January 1, 10000.
*/
extern SECStatus DER_TimeToGeneralizedTime(SECItem *dst, PRTime gmttime);
extern SECStatus DER_TimeToGeneralizedTimeArena(PLArenaPool *arenaOpt,
                                                SECItem *dst, PRTime gmttime);

/*
** Convert a DER encoded Generalized time value into an NSPR time value.
**  "dst" the resulting NSPR time
**  "string" the der notation ascii value to decode
*/
extern SECStatus DER_GeneralizedTimeToTime(PRTime *dst, const SECItem *time);

/*
** Convert from a PRTime UTC time value to a formatted ascii value. The
** caller is responsible for deallocating the returned buffer.
*/
extern char *CERT_UTCTime2FormattedAscii(PRTime utcTime, char *format);
#define CERT_GeneralizedTime2FormattedAscii CERT_UTCTime2FormattedAscii

/*
** Convert from a PRTime Generalized time value to a formatted ascii value. The
** caller is responsible for deallocating the returned buffer.
*/
extern char *CERT_GenTime2FormattedAscii(PRTime genTime, char *format);

/*
** decode a SECItem containing either a SEC_ASN1_GENERALIZED_TIME
** or a SEC_ASN1_UTC_TIME
*/

extern SECStatus DER_DecodeTimeChoice(PRTime *output, const SECItem *input);

/* encode a PRTime to an ASN.1 DER SECItem containing either a
   SEC_ASN1_GENERALIZED_TIME or a SEC_ASN1_UTC_TIME */

extern SECStatus DER_EncodeTimeChoice(PLArenaPool *arena, SECItem *output,
                                      PRTime input);

SEC_END_PROTOS

#endif /* _SECDER_H_ */
