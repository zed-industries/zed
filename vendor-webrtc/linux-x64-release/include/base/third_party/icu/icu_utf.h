// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/*
******************************************************************************
*
*   Copyright (C) 1999-2015, International Business Machines
*   Corporation and others.  All Rights Reserved.
*
******************************************************************************
*/

#ifndef BASE_THIRD_PARTY_ICU_ICU_UTF_H_
#define BASE_THIRD_PARTY_ICU_ICU_UTF_H_

#include <stdint.h>

namespace base_icu {

// source/common/unicode/umachine.h

/** The ICU boolean type @stable ICU 2.0 */
typedef int8_t UBool;

/**
 * Define UChar32 as a type for single Unicode code points.
 * UChar32 is a signed 32-bit integer (same as int32_t).
 *
 * The Unicode code point range is 0..0x10ffff.
 * All other values (negative or >=0x110000) are illegal as Unicode code points.
 * They may be used as sentinel values to indicate "done", "error"
 * or similar non-code point conditions.
 *
 * Before ICU 2.4 (Jitterbug 2146), UChar32 was defined
 * to be wchar_t if that is 32 bits wide (wchar_t may be signed or unsigned)
 * or else to be uint32_t.
 * That is, the definition of UChar32 was platform-dependent.
 *
 * @see U_SENTINEL
 * @stable ICU 2.4
 */
typedef int32_t UChar32;

/**
 * This value is intended for sentinel values for APIs that
 * (take or) return single code points (UChar32).
 * It is outside of the Unicode code point range 0..0x10ffff.
 *
 * For example, a "done" or "error" value in a new API
 * could be indicated with U_SENTINEL.
 *
 * ICU APIs designed before ICU 2.4 usually define service-specific "done"
 * values, mostly 0xffff.
 * Those may need to be distinguished from
 * actual U+ffff text contents by calling functions like
 * CharacterIterator::hasNext() or UnicodeString::length().
 *
 * @return -1
 * @see UChar32
 * @stable ICU 2.4
 */
#define CBU_SENTINEL (-1)

/**
 * \def UPRV_BLOCK_MACRO_BEGIN
 * Defined as the "do" keyword by default.
 * @internal
 */
#ifndef CBUPRV_BLOCK_MACRO_BEGIN
#define CBUPRV_BLOCK_MACRO_BEGIN do
#endif

/**
 * \def UPRV_BLOCK_MACRO_END
 * Defined as "while (FALSE)" by default.
 * @internal
 */
#ifndef CBUPRV_BLOCK_MACRO_END
#define CBUPRV_BLOCK_MACRO_END while (0)
#endif


// source/common/unicode/utf.h

/**
 * Is this code point a Unicode noncharacter?
 * @param c 32-bit code point
 * @return TRUE or FALSE
 * @stable ICU 2.4
 */
#define CBU_IS_UNICODE_NONCHAR(c) \
    ((c)>=0xfdd0 && \
     ((c)<=0xfdef || ((c)&0xfffe)==0xfffe) && (c)<=0x10ffff)

/**
 * Is c a Unicode code point value (0..U+10ffff)
 * that can be assigned a character?
 *
 * Code points that are not characters include:
 * - single surrogate code points (U+d800..U+dfff, 2048 code points)
 * - the last two code points on each plane (U+__fffe and U+__ffff, 34 code points)
 * - U+fdd0..U+fdef (new with Unicode 3.1, 32 code points)
 * - the highest Unicode code point value is U+10ffff
 *
 * This means that all code points below U+d800 are character code points,
 * and that boundary is tested first for performance.
 *
 * @param c 32-bit code point
 * @return TRUE or FALSE
 * @stable ICU 2.4
 */
#define CBU_IS_UNICODE_CHAR(c) \
    ((uint32_t)(c)<0xd800 || \
        (0xdfff<(c) && (c)<=0x10ffff && !CBU_IS_UNICODE_NONCHAR(c)))

/**
 * Is this code point a surrogate (U+d800..U+dfff)?
 * @param c 32-bit code point
 * @return TRUE or FALSE
 * @stable ICU 2.4
 */
#define CBU_IS_SURROGATE(c) (((uint32_t)(c)&0xfffff800) == 0xd800)

/**
 * Assuming c is a surrogate code point (U_IS_SURROGATE(c)),
 * is it a lead surrogate?
 * @param c 32-bit code point
 * @return TRUE or FALSE
 * @stable ICU 2.4
 */
#define CBU_IS_SURROGATE_LEAD(c) (((c)&0x400)==0)

// source/common/unicode/utf8.h

/**
 * Internal bit vector for 3-byte UTF-8 validity check, for use in U8_IS_VALID_LEAD3_AND_T1.
 * Each bit indicates whether one lead byte + first trail byte pair starts a valid sequence.
 * Lead byte E0..EF bits 3..0 are used as byte index,
 * first trail byte bits 7..5 are used as bit index into that byte.
 * @see U8_IS_VALID_LEAD3_AND_T1
 * @internal
 */
#define CBU8_LEAD3_T1_BITS "\x20\x30\x30\x30\x30\x30\x30\x30\x30\x30\x30\x30\x30\x10\x30\x30"

/**
 * Internal 3-byte UTF-8 validity check.
 * Non-zero if lead byte E0..EF and first trail byte 00..FF start a valid sequence.
 * @internal
 */
#define CBU8_IS_VALID_LEAD3_AND_T1(lead, t1) (CBU8_LEAD3_T1_BITS[(lead)&0xf]&(1<<((uint8_t)(t1)>>5)))

/**
 * Internal bit vector for 4-byte UTF-8 validity check, for use in U8_IS_VALID_LEAD4_AND_T1.
 * Each bit indicates whether one lead byte + first trail byte pair starts a valid sequence.
 * First trail byte bits 7..4 are used as byte index,
 * lead byte F0..F4 bits 2..0 are used as bit index into that byte.
 * @see U8_IS_VALID_LEAD4_AND_T1
 * @internal
 */
#define CBU8_LEAD4_T1_BITS "\x00\x00\x00\x00\x00\x00\x00\x00\x1E\x0F\x0F\x0F\x00\x00\x00\x00"

/**
 * Internal 4-byte UTF-8 validity check.
 * Non-zero if lead byte F0..F4 and first trail byte 00..FF start a valid sequence.
 * @internal
 */
#define CBU8_IS_VALID_LEAD4_AND_T1(lead, t1) (CBU8_LEAD4_T1_BITS[(uint8_t)(t1)>>4]&(1<<((lead)&7)))

/**
 * Does this code unit (byte) encode a code point by itself (US-ASCII 0..0x7f)?
 * @param c 8-bit code unit (byte)
 * @return TRUE or FALSE
 * @stable ICU 2.4
 */
#define CBU8_IS_SINGLE(c) (((c)&0x80)==0)

/**
 * Is this code unit (byte) a UTF-8 lead byte? (0xC2..0xF4)
 * @param c 8-bit code unit (byte)
 * @return TRUE or FALSE
 * @stable ICU 2.4
 */
#define CBU8_IS_LEAD(c) ((uint8_t)((c)-0xc2)<=0x32)

/**
 * Is this code unit (byte) a UTF-8 trail byte? (0x80..0xBF)
 * @param c 8-bit code unit (byte)
 * @return TRUE or FALSE
 * @stable ICU 2.4
 */
#define CBU8_IS_TRAIL(c) ((int8_t)(c)<-0x40)

/**
 * How many code units (bytes) are used for the UTF-8 encoding
 * of this Unicode code point?
 * @param c 32-bit code point
 * @return 1..4, or 0 if c is a surrogate or not a Unicode code point
 * @stable ICU 2.4
 */
#define CBU8_LENGTH(c) \
    ((uint32_t)(c)<=0x7f ? 1 : \
        ((uint32_t)(c)<=0x7ff ? 2 : \
            ((uint32_t)(c)<=0xd7ff ? 3 : \
                ((uint32_t)(c)<=0xdfff || (uint32_t)(c)>0x10ffff ? 0 : \
                    ((uint32_t)(c)<=0xffff ? 3 : 4)\
                ) \
            ) \
        ) \
    )

/**
 * The maximum number of UTF-8 code units (bytes) per Unicode code point (U+0000..U+10ffff).
 * @return 4
 * @stable ICU 2.4
 */
#define CBU8_MAX_LENGTH 4

/**
 * Get a code point from a string at a code point boundary offset,
 * and advance the offset to the next code point boundary.
 * (Post-incrementing forward iteration.)
 * "Safe" macro, checks for illegal sequences and for string boundaries.
 *
 * The length can be negative for a NUL-terminated string.
 *
 * The offset may point to the lead byte of a multi-byte sequence,
 * in which case the macro will read the whole sequence.
 * If the offset points to a trail byte or an illegal UTF-8 sequence, then
 * c is set to a negative value.
 *
 * @param s const uint8_t * string
 * @param i int32_t string offset, must be i<length
 * @param length int32_t string length
 * @param c output UChar32 variable, set to <0 in case of an error
 * @see U8_NEXT_UNSAFE
 * @stable ICU 2.4
 */
#define CBU8_NEXT(s, i, length, c) CBU8_INTERNAL_NEXT_OR_SUB(s, i, length, c, CBU_SENTINEL)

/** @internal */
#define CBU8_INTERNAL_NEXT_OR_SUB(s, i, length, c, sub) CBUPRV_BLOCK_MACRO_BEGIN { \
    (c)=(uint8_t)(s)[(i)++]; \
    if(!CBU8_IS_SINGLE(c)) { \
        uint8_t __t = 0; \
        if((i)!=(length) && \
            /* fetch/validate/assemble all but last trail byte */ \
            ((c)>=0xe0 ? \
                ((c)<0xf0 ?  /* U+0800..U+FFFF except surrogates */ \
                    CBU8_LEAD3_T1_BITS[(c)&=0xf]&(1<<((__t=(s)[i])>>5)) && \
                    (__t&=0x3f, 1) \
                :  /* U+10000..U+10FFFF */ \
                    ((c)-=0xf0)<=4 && \
                    CBU8_LEAD4_T1_BITS[(__t=(s)[i])>>4]&(1<<(c)) && \
                    ((c)=((c)<<6)|(__t&0x3f), ++(i)!=(length)) && \
                    (__t=(s)[i]-0x80)<=0x3f) && \
                /* valid second-to-last trail byte */ \
                ((c)=((c)<<6)|__t, ++(i)!=(length)) \
            :  /* U+0080..U+07FF */ \
                (c)>=0xc2 && ((c)&=0x1f, 1)) && \
            /* last trail byte */ \
            (__t=(s)[i]-0x80)<=0x3f && \
            ((c)=((c)<<6)|__t, ++(i), 1)) { \
        } else { \
            (c)=(sub);  /* ill-formed*/ \
        } \
    } \
} CBUPRV_BLOCK_MACRO_END

/**
 * Append a code point to a string, overwriting 1 to 4 bytes.
 * The offset points to the current end of the string contents
 * and is advanced (post-increment).
 * "Unsafe" macro, assumes a valid code point and sufficient space in the string.
 * Otherwise, the result is undefined.
 *
 * @param s const uint8_t * string buffer
 * @param i string offset
 * @param c code point to append
 * @see U8_APPEND
 * @stable ICU 2.4
 */
#define CBU8_APPEND_UNSAFE(s, i, c)                             \
  CBUPRV_BLOCK_MACRO_BEGIN {                                    \
    uint32_t __uc = (uint32_t)(c);                              \
    if (__uc <= 0x7f) {                                         \
      (s)[(i)++] = (uint8_t)__uc;                               \
    } else {                                                    \
      if (__uc <= 0x7ff) {                                      \
        (s)[(i)++] = (uint8_t)((__uc >> 6) | 0xc0);             \
      } else {                                                  \
        if (__uc <= 0xffff) {                                   \
          (s)[(i)++] = (uint8_t)((__uc >> 12) | 0xe0);          \
        } else {                                                \
          (s)[(i)++] = (uint8_t)((__uc >> 18) | 0xf0);          \
          (s)[(i)++] = (uint8_t)(((__uc >> 12) & 0x3f) | 0x80); \
        }                                                       \
        (s)[(i)++] = (uint8_t)(((__uc >> 6) & 0x3f) | 0x80);    \
      }                                                         \
      (s)[(i)++] = (uint8_t)((__uc & 0x3f) | 0x80);             \
    }                                                           \
  }                                                             \
  CBUPRV_BLOCK_MACRO_END

// source/common/unicode/utf16.h

/**
 * Does this code unit alone encode a code point (BMP, not a surrogate)?
 * @param c 16-bit code unit
 * @return TRUE or FALSE
 * @stable ICU 2.4
 */
#define CBU16_IS_SINGLE(c) !CBU_IS_SURROGATE(c)

/**
 * Is this code unit a lead surrogate (U+d800..U+dbff)?
 * @param c 16-bit code unit
 * @return TRUE or FALSE
 * @stable ICU 2.4
 */
#define CBU16_IS_LEAD(c) (((uint32_t)(c)&0xfffffc00) == 0xd800)

/**
 * Is this code unit a trail surrogate (U+dc00..U+dfff)?
 * @param c 16-bit code unit
 * @return TRUE or FALSE
 * @stable ICU 2.4
 */
#define CBU16_IS_TRAIL(c) (((uint32_t)(c)&0xfffffc00) == 0xdc00)

/**
 * Is this code unit a surrogate (U+d800..U+dfff)?
 * @param c 16-bit code unit
 * @return TRUE or FALSE
 * @stable ICU 2.4
 */
#define CBU16_IS_SURROGATE(c) CBU_IS_SURROGATE(c)

/**
 * Assuming c is a surrogate code point (U16_IS_SURROGATE(c)),
 * is it a lead surrogate?
 * @param c 16-bit code unit
 * @return TRUE or FALSE
 * @stable ICU 2.4
 */
#define CBU16_IS_SURROGATE_LEAD(c) (((c)&0x400)==0)

/**
 * Helper constant for U16_GET_SUPPLEMENTARY.
 * @internal
 */
#define CBU16_SURROGATE_OFFSET ((0xd800<<10UL)+0xdc00-0x10000)

/**
 * Get a supplementary code point value (U+10000..U+10ffff)
 * from its lead and trail surrogates.
 * The result is undefined if the input values are not
 * lead and trail surrogates.
 *
 * @param lead lead surrogate (U+d800..U+dbff)
 * @param trail trail surrogate (U+dc00..U+dfff)
 * @return supplementary code point (U+10000..U+10ffff)
 * @stable ICU 2.4
 */
#define CBU16_GET_SUPPLEMENTARY(lead, trail) \
    (((::base_icu::UChar32)(lead)<<10UL)+(::base_icu::UChar32)(trail)-CBU16_SURROGATE_OFFSET)

/**
 * Get the lead surrogate (0xd800..0xdbff) for a
 * supplementary code point (0x10000..0x10ffff).
 * @param supplementary 32-bit code point (U+10000..U+10ffff)
 * @return lead surrogate (U+d800..U+dbff) for supplementary
 * @stable ICU 2.4
 */
#define CBU16_LEAD(supplementary) (::base_icu::UChar)(((supplementary)>>10)+0xd7c0)

/**
 * Get the trail surrogate (0xdc00..0xdfff) for a
 * supplementary code point (0x10000..0x10ffff).
 * @param supplementary 32-bit code point (U+10000..U+10ffff)
 * @return trail surrogate (U+dc00..U+dfff) for supplementary
 * @stable ICU 2.4
 */
#define CBU16_TRAIL(supplementary) (::base_icu::UChar)(((supplementary)&0x3ff)|0xdc00)

/**
 * How many 16-bit code units are used to encode this Unicode code point? (1 or 2)
 * The result is not defined if c is not a Unicode code point (U+0000..U+10ffff).
 * @param c 32-bit code point
 * @return 1 or 2
 * @stable ICU 2.4
 */
#define CBU16_LENGTH(c) ((uint32_t)(c)<=0xffff ? 1 : 2)

/**
 * The maximum number of 16-bit code units per Unicode code point (U+0000..U+10ffff).
 * @return 2
 * @stable ICU 2.4
 */
#define CBU16_MAX_LENGTH 2

/**
 * Get a code point from a string at a random-access offset,
 * without changing the offset.
 * "Safe" macro, handles unpaired surrogates and checks for string boundaries.
 *
 * The offset may point to either the lead or trail surrogate unit
 * for a supplementary code point, in which case the macro will read
 * the adjacent matching surrogate as well.
 *
 * The length can be negative for a NUL-terminated string.
 *
 * If the offset points to a single, unpaired surrogate, then
 * c is set to that unpaired surrogate.
 * Iteration through a string is more efficient with U16_NEXT_UNSAFE or U16_NEXT.
 *
 * @param s const UChar * string
 * @param start starting string offset (usually 0)
 * @param i string offset, must be start<=i<length
 * @param length string length
 * @param c output UChar32 variable
 * @see U16_GET_UNSAFE
 * @stable ICU 2.4
 */
#define CBU16_GET(s, start, i, length, c) CBUPRV_BLOCK_MACRO_BEGIN { \
    (c)=(s)[i]; \
    if(CBU16_IS_SURROGATE(c)) { \
        uint16_t __c2; \
        if(CBU16_IS_SURROGATE_LEAD(c)) { \
            if((i)+1!=(length) && CBU16_IS_TRAIL(__c2=(s)[(i)+1])) { \
                (c)=CBU16_GET_SUPPLEMENTARY((c), __c2); \
            } \
        } else { \
            if((i)>(start) && CBU16_IS_LEAD(__c2=(s)[(i)-1])) { \
                (c)=CBU16_GET_SUPPLEMENTARY(__c2, (c)); \
            } \
        } \
    } \
} CBUPRV_BLOCK_MACRO_END

/**
 * Get a code point from a string at a code point boundary offset,
 * and advance the offset to the next code point boundary.
 * (Post-incrementing forward iteration.)
 * "Safe" macro, handles unpaired surrogates and checks for string boundaries.
 *
 * The length can be negative for a NUL-terminated string.
 *
 * The offset may point to the lead surrogate unit
 * for a supplementary code point, in which case the macro will read
 * the following trail surrogate as well.
 * If the offset points to a trail surrogate or
 * to a single, unpaired lead surrogate, then c is set to that unpaired surrogate.
 *
 * @param s const UChar * string
 * @param i string offset, must be i<length
 * @param length string length
 * @param c output UChar32 variable
 * @see U16_NEXT_UNSAFE
 * @stable ICU 2.4
 */
#define CBU16_NEXT(s, i, length, c) CBUPRV_BLOCK_MACRO_BEGIN { \
    (c)=(s)[(i)++]; \
    if(CBU16_IS_LEAD(c)) { \
        uint16_t __c2; \
        if((i)!=(length) && CBU16_IS_TRAIL(__c2=(s)[(i)])) { \
            ++(i); \
            (c)=CBU16_GET_SUPPLEMENTARY((c), __c2); \
        } \
    } \
} CBUPRV_BLOCK_MACRO_END

/**
 * Append a code point to a string, overwriting 1 or 2 code units.
 * The offset points to the current end of the string contents
 * and is advanced (post-increment).
 * "Unsafe" macro, assumes a valid code point and sufficient space in the string.
 * Otherwise, the result is undefined.
 *
 * @param s const UChar * string buffer
 * @param i string offset
 * @param c code point to append
 * @see U16_APPEND
 * @stable ICU 2.4
 */
#define CBU16_APPEND_UNSAFE(s, i, c) CBUPRV_BLOCK_MACRO_BEGIN { \
    if((uint32_t)(c)<=0xffff) { \
        (s)[(i)++]=(uint16_t)(c); \
    } else { \
        (s)[(i)++]=(uint16_t)(((c)>>10)+0xd7c0); \
        (s)[(i)++]=(uint16_t)(((c)&0x3ff)|0xdc00); \
    } \
} CBUPRV_BLOCK_MACRO_END

/**
 * Adjust a random-access offset to a code point boundary
 * at the start of a code point.
 * If the offset points to the trail surrogate of a surrogate pair,
 * then the offset is decremented.
 * Otherwise, it is not modified.
 * "Safe" macro, handles unpaired surrogates and checks for string boundaries.
 *
 * @param s const UChar * string
 * @param start starting string offset (usually 0)
 * @param i string offset, must be start<=i
 * @see U16_SET_CP_START_UNSAFE
 * @stable ICU 2.4
 */
#define CBU16_SET_CP_START(s, start, i) CBUPRV_BLOCK_MACRO_BEGIN { \
    if(CBU16_IS_TRAIL((s)[i]) && (i)>(start) && CBU16_IS_LEAD((s)[(i)-1])) { \
        --(i); \
    } \
} CBUPRV_BLOCK_MACRO_END

/**
 * Move the string offset from one code point boundary to the previous one
 * and get the code point between them.
 * (Pre-decrementing backward iteration.)
 * "Safe" macro, handles unpaired surrogates and checks for string boundaries.
 *
 * The input offset may be the same as the string length.
 * If the offset is behind a trail surrogate unit
 * for a supplementary code point, then the macro will read
 * the preceding lead surrogate as well.
 * If the offset is behind a lead surrogate or behind a single, unpaired
 * trail surrogate, then c is set to that unpaired surrogate.
 *
 * @param s const UChar * string
 * @param start starting string offset (usually 0)
 * @param i string offset, must be start<i
 * @param c output UChar32 variable
 * @see U16_PREV_UNSAFE
 * @stable ICU 2.4
 */
#define CBU16_PREV(s, start, i, c) CBUPRV_BLOCK_MACRO_BEGIN { \
    (c)=(s)[--(i)]; \
    if(CBU16_IS_TRAIL(c)) { \
        uint16_t __c2; \
        if((i)>(start) && CBU16_IS_LEAD(__c2=(s)[(i)-1])) { \
            --(i); \
            (c)=CBU16_GET_SUPPLEMENTARY(__c2, (c)); \
        } \
    } \
} CBUPRV_BLOCK_MACRO_END

/**
 * Adjust a random-access offset to a code point boundary after a code point.
 * If the offset is behind the lead surrogate of a surrogate pair,
 * then the offset is incremented.
 * Otherwise, it is not modified.
 * The input offset may be the same as the string length.
 * "Safe" macro, handles unpaired surrogates and checks for string boundaries.
 *
 * The length can be negative for a NUL-terminated string.
 *
 * @param s const UChar * string
 * @param start int32_t starting string offset (usually 0)
 * @param i int32_t string offset, start<=i<=length
 * @param length int32_t string length
 * @see U16_SET_CP_LIMIT_UNSAFE
 * @stable ICU 2.4
 */
#define CBU16_SET_CP_LIMIT(s, start, i, length) CBUPRV_BLOCK_MACRO_BEGIN { \
    if((start)<(i) && ((i)<(length) || (length)<0) && CBU16_IS_LEAD((s)[(i)-1]) && CBU16_IS_TRAIL((s)[i])) { \
        ++(i); \
    } \
} CBUPRV_BLOCK_MACRO_END

}  // namesapce base_icu

#endif  // BASE_THIRD_PARTY_ICU_ICU_UTF_H_
