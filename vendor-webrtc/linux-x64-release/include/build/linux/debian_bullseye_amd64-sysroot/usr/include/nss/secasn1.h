/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/*
 * Support for encoding/decoding of ASN.1 using BER/DER (Basic/Distinguished
 * Encoding Rules).  The routines are found in and used extensively by the
 * security library, but exported for other use.
 */

#ifndef _SECASN1_H_
#define _SECASN1_H_

#include "utilrename.h"
#include "plarena.h"

#include "seccomon.h"
#include "secasn1t.h"

/************************************************************************/
SEC_BEGIN_PROTOS

/*
 * XXX These function prototypes need full, explanatory comments.
 */

/*
** Decoding.
*/

extern SEC_ASN1DecoderContext *SEC_ASN1DecoderStart(PLArenaPool *pool,
                                                    void *dest,
                                                    const SEC_ASN1Template *t);

/* XXX char or unsigned char? */
extern SECStatus SEC_ASN1DecoderUpdate(SEC_ASN1DecoderContext *cx,
                                       const char *buf,
                                       unsigned long len);

extern SECStatus SEC_ASN1DecoderFinish(SEC_ASN1DecoderContext *cx);

/* Higher level code detected an error, abort the rest of the processing */
extern void SEC_ASN1DecoderAbort(SEC_ASN1DecoderContext *cx, int error);

extern void SEC_ASN1DecoderSetFilterProc(SEC_ASN1DecoderContext *cx,
                                         SEC_ASN1WriteProc fn,
                                         void *arg, PRBool no_store);

extern void SEC_ASN1DecoderClearFilterProc(SEC_ASN1DecoderContext *cx);

extern void SEC_ASN1DecoderSetNotifyProc(SEC_ASN1DecoderContext *cx,
                                         SEC_ASN1NotifyProc fn,
                                         void *arg);

extern void SEC_ASN1DecoderClearNotifyProc(SEC_ASN1DecoderContext *cx);

/* Sets the maximum size that should be allocated for a single ASN.1
 * element. Set to 0 to indicate there is no limit.
 *
 * Note: This does not set the maximum size overall that may be allocated
 * while parsing, nor does it guarantee that the decoder won't allocate
 * more than |max_size| while parsing an individual element; rather, it
 * merely guarantees that any individual allocation for returned data
 * should not exceed |max_size|.
*/
extern void SEC_ASN1DecoderSetMaximumElementSize(SEC_ASN1DecoderContext *cx,
                                                 unsigned long max_size);

extern SECStatus SEC_ASN1Decode(PLArenaPool *pool, void *dest,
                                const SEC_ASN1Template *t,
                                const char *buf, long len);

/* Both classic ASN.1 and QuickDER have a feature that removes leading zeroes
   out of SEC_ASN1_INTEGER if the caller sets siUnsignedInteger in the type
   field of the target SECItem prior to calling the decoder. Otherwise, the
   type field is ignored and untouched. For SECItem that are dynamically
   allocated (from POINTER, SET OF, SEQUENCE OF) the decoder sets the type
   field to siBuffer. */

extern SECStatus SEC_ASN1DecodeItem(PLArenaPool *pool, void *dest,
                                    const SEC_ASN1Template *t,
                                    const SECItem *src);

extern SECStatus SEC_QuickDERDecodeItem(PLArenaPool *arena, void *dest,
                                        const SEC_ASN1Template *templateEntry,
                                        const SECItem *src);

/*
** Encoding.
*/

extern SEC_ASN1EncoderContext *SEC_ASN1EncoderStart(const void *src,
                                                    const SEC_ASN1Template *t,
                                                    SEC_ASN1WriteProc fn,
                                                    void *output_arg);

/* XXX char or unsigned char? */
extern SECStatus SEC_ASN1EncoderUpdate(SEC_ASN1EncoderContext *cx,
                                       const char *buf,
                                       unsigned long len);

extern void SEC_ASN1EncoderFinish(SEC_ASN1EncoderContext *cx);

/* Higher level code detected an error, abort the rest of the processing */
extern void SEC_ASN1EncoderAbort(SEC_ASN1EncoderContext *cx, int error);

extern void SEC_ASN1EncoderSetNotifyProc(SEC_ASN1EncoderContext *cx,
                                         SEC_ASN1NotifyProc fn,
                                         void *arg);

extern void SEC_ASN1EncoderClearNotifyProc(SEC_ASN1EncoderContext *cx);

extern void SEC_ASN1EncoderSetStreaming(SEC_ASN1EncoderContext *cx);

extern void SEC_ASN1EncoderClearStreaming(SEC_ASN1EncoderContext *cx);

extern void sec_ASN1EncoderSetDER(SEC_ASN1EncoderContext *cx);

extern void sec_ASN1EncoderClearDER(SEC_ASN1EncoderContext *cx);

extern void SEC_ASN1EncoderSetTakeFromBuf(SEC_ASN1EncoderContext *cx);

extern void SEC_ASN1EncoderClearTakeFromBuf(SEC_ASN1EncoderContext *cx);

extern SECStatus SEC_ASN1Encode(const void *src, const SEC_ASN1Template *t,
                                SEC_ASN1WriteProc output_proc,
                                void *output_arg);

/*
 * If both pool and dest are NULL, the caller should free the returned SECItem
 * with a SECITEM_FreeItem(..., PR_TRUE) call.  If pool is NULL but dest is
 * not NULL, the caller should free the data buffer pointed to by dest with a
 * SECITEM_FreeItem(dest, PR_FALSE) or PORT_Free(dest->data) call.
 */
extern SECItem *SEC_ASN1EncodeItem(PLArenaPool *pool, SECItem *dest,
                                   const void *src, const SEC_ASN1Template *t);

extern SECItem *SEC_ASN1EncodeInteger(PLArenaPool *pool,
                                      SECItem *dest, long value);

extern SECItem *SEC_ASN1EncodeUnsignedInteger(PLArenaPool *pool,
                                              SECItem *dest,
                                              unsigned long value);

extern SECStatus SEC_ASN1DecodeInteger(SECItem *src,
                                       unsigned long *value);

/*
** Utilities.
*/

/*
 * We have a length that needs to be encoded; how many bytes will the
 * encoding take?
 */
extern int SEC_ASN1LengthLength(unsigned long len);

/* encode the length and return the number of bytes we encoded. Buffer
 * must be pre allocated  */
extern int SEC_ASN1EncodeLength(unsigned char *buf, int value);

/*
 * Find the appropriate subtemplate for the given template.
 * This may involve calling a "chooser" function, or it may just
 * be right there.  In either case, it is expected to *have* a
 * subtemplate; this is asserted in debug builds (in non-debug
 * builds, NULL will be returned).
 *
 * "thing" is a pointer to the structure being encoded/decoded
 * "encoding", when true, means that we are in the process of encoding
 *  (as opposed to in the process of decoding)
 */
extern const SEC_ASN1Template *
SEC_ASN1GetSubtemplate(const SEC_ASN1Template *inTemplate, void *thing,
                       PRBool encoding);

/* whether the template is for a primitive type or a choice of
 * primitive types
 */
extern PRBool SEC_ASN1IsTemplateSimple(const SEC_ASN1Template *theTemplate);

/************************************************************************/

/*
 * Generic Templates
 * One for each of the simple types, plus a special one for ANY, plus:
 *  - a pointer to each one of those
 *  - a set of each one of those
 *  - a sequence of each one of those
 *
 * Note that these are alphabetical (case insensitive); please add new
 * ones in the appropriate place.
 */

extern const SEC_ASN1Template SEC_AnyTemplate[];
extern const SEC_ASN1Template SEC_BitStringTemplate[];
extern const SEC_ASN1Template SEC_BMPStringTemplate[];
extern const SEC_ASN1Template SEC_BooleanTemplate[];
extern const SEC_ASN1Template SEC_EnumeratedTemplate[];
extern const SEC_ASN1Template SEC_GeneralizedTimeTemplate[];
extern const SEC_ASN1Template SEC_IA5StringTemplate[];
extern const SEC_ASN1Template SEC_IntegerTemplate[];
extern const SEC_ASN1Template SEC_NullTemplate[];
extern const SEC_ASN1Template SEC_ObjectIDTemplate[];
extern const SEC_ASN1Template SEC_OctetStringTemplate[];
extern const SEC_ASN1Template SEC_PrintableStringTemplate[];
extern const SEC_ASN1Template SEC_T61StringTemplate[];
extern const SEC_ASN1Template SEC_UniversalStringTemplate[];
extern const SEC_ASN1Template SEC_UTCTimeTemplate[];
extern const SEC_ASN1Template SEC_UTF8StringTemplate[];
extern const SEC_ASN1Template SEC_VisibleStringTemplate[];

extern const SEC_ASN1Template SEC_PointerToAnyTemplate[];
extern const SEC_ASN1Template SEC_PointerToBitStringTemplate[];
extern const SEC_ASN1Template SEC_PointerToBMPStringTemplate[];
extern const SEC_ASN1Template SEC_PointerToBooleanTemplate[];
extern const SEC_ASN1Template SEC_PointerToEnumeratedTemplate[];
extern const SEC_ASN1Template SEC_PointerToGeneralizedTimeTemplate[];
extern const SEC_ASN1Template SEC_PointerToIA5StringTemplate[];
extern const SEC_ASN1Template SEC_PointerToIntegerTemplate[];
extern const SEC_ASN1Template SEC_PointerToNullTemplate[];
extern const SEC_ASN1Template SEC_PointerToObjectIDTemplate[];
extern const SEC_ASN1Template SEC_PointerToOctetStringTemplate[];
extern const SEC_ASN1Template SEC_PointerToPrintableStringTemplate[];
extern const SEC_ASN1Template SEC_PointerToT61StringTemplate[];
extern const SEC_ASN1Template SEC_PointerToUniversalStringTemplate[];
extern const SEC_ASN1Template SEC_PointerToUTCTimeTemplate[];
extern const SEC_ASN1Template SEC_PointerToUTF8StringTemplate[];
extern const SEC_ASN1Template SEC_PointerToVisibleStringTemplate[];

extern const SEC_ASN1Template SEC_SequenceOfAnyTemplate[];
extern const SEC_ASN1Template SEC_SequenceOfBitStringTemplate[];
extern const SEC_ASN1Template SEC_SequenceOfBMPStringTemplate[];
extern const SEC_ASN1Template SEC_SequenceOfBooleanTemplate[];
extern const SEC_ASN1Template SEC_SequenceOfEnumeratedTemplate[];
extern const SEC_ASN1Template SEC_SequenceOfGeneralizedTimeTemplate[];
extern const SEC_ASN1Template SEC_SequenceOfIA5StringTemplate[];
extern const SEC_ASN1Template SEC_SequenceOfIntegerTemplate[];
extern const SEC_ASN1Template SEC_SequenceOfNullTemplate[];
extern const SEC_ASN1Template SEC_SequenceOfObjectIDTemplate[];
extern const SEC_ASN1Template SEC_SequenceOfOctetStringTemplate[];
extern const SEC_ASN1Template SEC_SequenceOfPrintableStringTemplate[];
extern const SEC_ASN1Template SEC_SequenceOfT61StringTemplate[];
extern const SEC_ASN1Template SEC_SequenceOfUniversalStringTemplate[];
extern const SEC_ASN1Template SEC_SequenceOfUTCTimeTemplate[];
extern const SEC_ASN1Template SEC_SequenceOfUTF8StringTemplate[];
extern const SEC_ASN1Template SEC_SequenceOfVisibleStringTemplate[];

extern const SEC_ASN1Template SEC_SetOfAnyTemplate[];
extern const SEC_ASN1Template SEC_SetOfBitStringTemplate[];
extern const SEC_ASN1Template SEC_SetOfBMPStringTemplate[];
extern const SEC_ASN1Template SEC_SetOfBooleanTemplate[];
extern const SEC_ASN1Template SEC_SetOfEnumeratedTemplate[];
extern const SEC_ASN1Template SEC_SetOfGeneralizedTimeTemplate[];
extern const SEC_ASN1Template SEC_SetOfIA5StringTemplate[];
extern const SEC_ASN1Template SEC_SetOfIntegerTemplate[];
extern const SEC_ASN1Template SEC_SetOfNullTemplate[];
extern const SEC_ASN1Template SEC_SetOfObjectIDTemplate[];
extern const SEC_ASN1Template SEC_SetOfOctetStringTemplate[];
extern const SEC_ASN1Template SEC_SetOfPrintableStringTemplate[];
extern const SEC_ASN1Template SEC_SetOfT61StringTemplate[];
extern const SEC_ASN1Template SEC_SetOfUniversalStringTemplate[];
extern const SEC_ASN1Template SEC_SetOfUTCTimeTemplate[];
extern const SEC_ASN1Template SEC_SetOfUTF8StringTemplate[];
extern const SEC_ASN1Template SEC_SetOfVisibleStringTemplate[];

/*
 * Template for skipping a subitem; this only makes sense when decoding.
 */
extern const SEC_ASN1Template SEC_SkipTemplate[];

/* These functions simply return the address of the above-declared templates.
** This is necessary for Windows DLLs.  Sigh.
*/
SEC_ASN1_CHOOSER_DECLARE(SEC_AnyTemplate)
SEC_ASN1_CHOOSER_DECLARE(SEC_BMPStringTemplate)
SEC_ASN1_CHOOSER_DECLARE(SEC_BooleanTemplate)
SEC_ASN1_CHOOSER_DECLARE(SEC_BitStringTemplate)
SEC_ASN1_CHOOSER_DECLARE(SEC_GeneralizedTimeTemplate)
SEC_ASN1_CHOOSER_DECLARE(SEC_IA5StringTemplate)
SEC_ASN1_CHOOSER_DECLARE(SEC_IntegerTemplate)
SEC_ASN1_CHOOSER_DECLARE(SEC_NullTemplate)
SEC_ASN1_CHOOSER_DECLARE(SEC_ObjectIDTemplate)
SEC_ASN1_CHOOSER_DECLARE(SEC_OctetStringTemplate)
SEC_ASN1_CHOOSER_DECLARE(SEC_UTCTimeTemplate)
SEC_ASN1_CHOOSER_DECLARE(SEC_UTF8StringTemplate)

SEC_ASN1_CHOOSER_DECLARE(SEC_PointerToAnyTemplate)
SEC_ASN1_CHOOSER_DECLARE(SEC_PointerToOctetStringTemplate)

SEC_ASN1_CHOOSER_DECLARE(SEC_SetOfAnyTemplate)

SEC_ASN1_CHOOSER_DECLARE(SEC_EnumeratedTemplate)
SEC_ASN1_CHOOSER_DECLARE(SEC_PointerToEnumeratedTemplate)
SEC_ASN1_CHOOSER_DECLARE(SEC_SequenceOfAnyTemplate)
SEC_ASN1_CHOOSER_DECLARE(SEC_SequenceOfObjectIDTemplate)
SEC_ASN1_CHOOSER_DECLARE(SEC_SkipTemplate)
SEC_ASN1_CHOOSER_DECLARE(SEC_UniversalStringTemplate)
SEC_ASN1_CHOOSER_DECLARE(SEC_PrintableStringTemplate)
SEC_ASN1_CHOOSER_DECLARE(SEC_T61StringTemplate)
SEC_ASN1_CHOOSER_DECLARE(SEC_PointerToGeneralizedTimeTemplate)
SEC_END_PROTOS
#endif /* _SECASN1_H_ */
