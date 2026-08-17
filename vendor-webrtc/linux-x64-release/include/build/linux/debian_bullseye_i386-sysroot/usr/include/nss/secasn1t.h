/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/*
 * Types for encoding/decoding of ASN.1 using BER/DER (Basic/Distinguished
 * Encoding Rules).
 */

#ifndef _SECASN1T_H_
#define _SECASN1T_H_

#include "utilrename.h"

/*
** An array of these structures defines a BER/DER encoding for an object.
**
** The array usually starts with a dummy entry whose kind is SEC_ASN1_SEQUENCE;
** such an array is terminated with an entry where kind == 0.  (An array
** which consists of a single component does not require a second dummy
** entry -- the array is only searched as long as previous component(s)
** instruct it.)
*/
typedef struct sec_ASN1Template_struct {
    /*
    ** Kind of item being decoded/encoded, including tags and modifiers.
    */
    unsigned long kind;

    /*
    ** The value is the offset from the base of the structure to the
    ** field that holds the value being decoded/encoded.
    */
    unsigned long offset;

    /*
    ** When kind suggests it (SEC_ASN1_POINTER, SEC_ASN1_GROUP, SEC_ASN1_INLINE,
    ** or a component that is *not* a SEC_ASN1_UNIVERSAL), this points to
    ** a sub-template for nested encoding/decoding,
    ** OR, iff SEC_ASN1_DYNAMIC is set, then this is a pointer to a pointer
    ** to a function which will return the appropriate template when called
    ** at runtime.  NOTE! that explicit level of indirection, which is
    ** necessary because ANSI does not allow you to store a function
    ** pointer directly as a "void *" so we must store it separately and
    ** dereference it to get at the function pointer itself.
    */
    const void *sub;

    /*
    ** In the first element of a template array, the value is the size
    ** of the structure to allocate when this template is being referenced
    ** by another template via SEC_ASN1_POINTER or SEC_ASN1_GROUP.
    ** In all other cases, the value is ignored.
    */
    unsigned int size;
} SEC_ASN1Template;

/* default size used for allocation of encoding/decoding stuff */
/* XXX what is the best value here? */
#define SEC_ASN1_DEFAULT_ARENA_SIZE (2048)

/*
** BER/DER values for ASN.1 identifier octets.
*/
#define SEC_ASN1_TAG_MASK 0xff

/*
 * BER/DER universal type tag numbers.
 * The values are defined by the X.208 standard; do not change them!
 * NOTE: if you add anything to this list, you must add code to secasn1d.c
 * to accept the tag, and probably also to secasn1e.c to encode it.
 * XXX It appears some have been added recently without being added to
 * the code; so need to go through the list now and double-check them all.
 * (Look especially at those added in revision 1.10.)
 */
#define SEC_ASN1_TAGNUM_MASK 0x1f
#define SEC_ASN1_BOOLEAN 0x01
#define SEC_ASN1_INTEGER 0x02
#define SEC_ASN1_BIT_STRING 0x03
#define SEC_ASN1_OCTET_STRING 0x04
#define SEC_ASN1_NULL 0x05
#define SEC_ASN1_OBJECT_ID 0x06
#define SEC_ASN1_OBJECT_DESCRIPTOR 0x07
/* External type and instance-of type   0x08 */
#define SEC_ASN1_REAL 0x09
#define SEC_ASN1_ENUMERATED 0x0a
#define SEC_ASN1_EMBEDDED_PDV 0x0b
#define SEC_ASN1_UTF8_STRING 0x0c
/*                                      0x0d */
/*                                      0x0e */
/*                                      0x0f */
#define SEC_ASN1_SEQUENCE 0x10
#define SEC_ASN1_SET 0x11
#define SEC_ASN1_NUMERIC_STRING 0x12
#define SEC_ASN1_PRINTABLE_STRING 0x13
#define SEC_ASN1_T61_STRING 0x14
#define SEC_ASN1_VIDEOTEX_STRING 0x15
#define SEC_ASN1_IA5_STRING 0x16
#define SEC_ASN1_UTC_TIME 0x17
#define SEC_ASN1_GENERALIZED_TIME 0x18
#define SEC_ASN1_GRAPHIC_STRING 0x19
#define SEC_ASN1_VISIBLE_STRING 0x1a
#define SEC_ASN1_GENERAL_STRING 0x1b
#define SEC_ASN1_UNIVERSAL_STRING 0x1c
/*                                      0x1d */
#define SEC_ASN1_BMP_STRING 0x1e
#define SEC_ASN1_HIGH_TAG_NUMBER 0x1f
#define SEC_ASN1_TELETEX_STRING SEC_ASN1_T61_STRING

/*
** Modifiers to type tags.  These are also specified by a/the
** standard, and must not be changed.
*/

#define SEC_ASN1_METHOD_MASK 0x20
#define SEC_ASN1_PRIMITIVE 0x00
#define SEC_ASN1_CONSTRUCTED 0x20

#define SEC_ASN1_CLASS_MASK 0xc0
#define SEC_ASN1_UNIVERSAL 0x00
#define SEC_ASN1_APPLICATION 0x40
#define SEC_ASN1_CONTEXT_SPECIFIC 0x80
#define SEC_ASN1_PRIVATE 0xc0

/*
** Our additions, used for templates.
** These are not defined by any standard; the values are used internally only.
** Just be careful to keep them out of the low 8 bits.
** XXX finish comments
*/
#define SEC_ASN1_OPTIONAL 0x00100
#define SEC_ASN1_EXPLICIT 0x00200
#define SEC_ASN1_ANY 0x00400
#define SEC_ASN1_INLINE 0x00800
#define SEC_ASN1_POINTER 0x01000
#define SEC_ASN1_GROUP 0x02000        /* with SET or SEQUENCE means \
                                       * SET OF or SEQUENCE OF */
#define SEC_ASN1_DYNAMIC 0x04000      /* subtemplate is found by calling \
                                       * a function at runtime */
#define SEC_ASN1_SKIP 0x08000         /* skip a field; only for decoding */
#define SEC_ASN1_INNER 0x10000        /* with ANY means capture the      \
                                       * contents only (not the id, len, \
                                       * or eoc); only for decoding */
#define SEC_ASN1_SAVE 0x20000         /* stash away the encoded bytes first; \
                                       * only for decoding */
#define SEC_ASN1_MAY_STREAM 0x40000   /* field or one of its sub-fields may \
                                       * stream in and so should encode as  \
                                       * indefinite-length when streaming   \
                                       * has been indicated; only for       \
                                       * encoding */
#define SEC_ASN1_SKIP_REST 0x80000    /* skip all following fields; \
                                         only for decoding */
#define SEC_ASN1_CHOICE 0x100000      /* pick one from a template */
#define SEC_ASN1_NO_STREAM 0X200000   /* This entry will not stream          \
                                         even if the sub-template says       \
                                         streaming is possible.  Helps       \
                                         to solve ambiguities with potential \
                                         streaming entries that are          \
                                         optional */
#define SEC_ASN1_DEBUG_BREAK 0X400000 /* put this in your template and the \
                                         decoder will assert when it       \
                                         processes it. Only for use with   \
                                         SEC_QuickDERDecodeItem */

/* Shorthand/Aliases */
#define SEC_ASN1_SEQUENCE_OF (SEC_ASN1_GROUP | SEC_ASN1_SEQUENCE)
#define SEC_ASN1_SET_OF (SEC_ASN1_GROUP | SEC_ASN1_SET)
#define SEC_ASN1_ANY_CONTENTS (SEC_ASN1_ANY | SEC_ASN1_INNER)

/* Maximum depth of nested SEQUENCEs and SETs */
#define SEC_ASN1D_MAX_DEPTH 32

/*
** Function used for SEC_ASN1_DYNAMIC.
** "arg" is a pointer to the structure being encoded/decoded
** "enc", when true, means that we are encoding (false means decoding)
*/
typedef const SEC_ASN1Template *SEC_ASN1TemplateChooser(void *arg, PRBool enc);
typedef SEC_ASN1TemplateChooser *SEC_ASN1TemplateChooserPtr;

#if defined(_WIN32) || defined(ANDROID)
#define SEC_ASN1_GET(x) NSS_Get_##x(NULL, PR_FALSE)
#define SEC_ASN1_SUB(x) &p_NSS_Get_##x
#define SEC_ASN1_XTRN SEC_ASN1_DYNAMIC
#define SEC_ASN1_MKSUB(x) \
    static const SEC_ASN1TemplateChooserPtr p_NSS_Get_##x = &NSS_Get_##x;
#else
#define SEC_ASN1_GET(x) x
#define SEC_ASN1_SUB(x) x
#define SEC_ASN1_XTRN 0
#define SEC_ASN1_MKSUB(x)
#endif

#define SEC_ASN1_CHOOSER_DECLARE(x) \
    extern const SEC_ASN1Template *NSS_Get_##x(void *arg, PRBool enc);

#define SEC_ASN1_CHOOSER_IMPLEMENT(x)                          \
    const SEC_ASN1Template *NSS_Get_##x(void *arg, PRBool enc) \
    {                                                          \
        return x;                                              \
    }

/*
** Opaque object used by the decoder to store state.
*/
typedef struct sec_DecoderContext_struct SEC_ASN1DecoderContext;

/*
** Opaque object used by the encoder to store state.
*/
typedef struct sec_EncoderContext_struct SEC_ASN1EncoderContext;

/*
 * This is used to describe to a filter function the bytes that are
 * being passed to it.  This is only useful when the filter is an "outer"
 * one, meaning it expects to get *all* of the bytes not just the
 * contents octets.
 */
typedef enum {
    SEC_ASN1_Identifier = 0,
    SEC_ASN1_Length = 1,
    SEC_ASN1_Contents = 2,
    SEC_ASN1_EndOfContents = 3
} SEC_ASN1EncodingPart;

/*
 * Type of the function pointer used either for decoding or encoding,
 * when doing anything "funny" (e.g. manipulating the data stream)
 */
typedef void (*SEC_ASN1NotifyProc)(void *arg, PRBool before,
                                   void *dest, int real_depth);

/*
 * Type of the function pointer used for grabbing encoded bytes.
 * This can be used during either encoding or decoding, as follows...
 *
 * When decoding, this can be used to filter the encoded bytes as they
 * are parsed.  This is what you would do if you wanted to process the data
 * along the way (like to decrypt it, or to perform a hash on it in order
 * to do a signature check later).  See SEC_ASN1DecoderSetFilterProc().
 * When processing only part of the encoded bytes is desired, you "watch"
 * for the field(s) you are interested in with a "notify proc" (see
 * SEC_ASN1DecoderSetNotifyProc()) and for even finer granularity (e.g. to
 * ignore all by the contents bytes) you pay attention to the "data_kind"
 * parameter.
 *
 * When encoding, this is the specification for the output function which
 * will receive the bytes as they are encoded.  The output function can
 * perform any postprocessing necessary (like hashing (some of) the data
 * to create a digest that gets included at the end) as well as shoving
 * the data off wherever it needs to go.  (In order to "tune" any processing,
 * you can set a "notify proc" as described above in the decoding case.)
 *
 * The parameters:
 * - "arg" is an opaque pointer that you provided at the same time you
 *   specified a function of this type
 * - "data" is a buffer of length "len", containing the encoded bytes
 * - "depth" is how deep in a nested encoding we are (it is not usually
 *   valuable, but can be useful sometimes so I included it)
 * - "data_kind" tells you if these bytes are part of the ASN.1 encoded
 *   octets for identifier, length, contents, or end-of-contents
 */
typedef void (*SEC_ASN1WriteProc)(void *arg,
                                  const char *data, unsigned long len,
                                  int depth, SEC_ASN1EncodingPart data_kind);

#endif /* _SECASN1T_H_ */
