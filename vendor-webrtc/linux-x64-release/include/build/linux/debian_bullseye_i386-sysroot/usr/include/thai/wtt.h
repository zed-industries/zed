/*
 * libthai - Thai Language Support Library
 * Copyright (C) 2001  Theppitak Karoonboonyanan <theppitak@gmail.com>
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
 */

/*
 * wtt.h - WTT I/O implementation
 * Created: 2001-08-04
 */

#ifndef THAI_WTT_H
#define THAI_WTT_H

/**
 * @file   wtt.h
 * @brief  WTT I/O implementation
 *
 * WTT stands for Wing Tuk Tee (in Thai, Runs everywhere). It was defined by 
 * TACTIS (Thai API Consortium/Thai Industrial Standard) in the NECTEC Thai 
 * Software Standard Project (1989-1991), and later endorsed by Thai 
 * Industrial Standard Institute (TISI) as TIS 1566-2541 in 1998.
 *
 * WTT classifies Thai chracter(TIS-620) into 17 types below.
 *
 * <pre>
 * <b>ITYPE  VALUE SHORT_DESCRIPTION</b>
 * CTRL    0    control characters
 * NON     1    non composible characters
 * CONS    2    consonants
 * LV      3    leading vowels
 * FV1     4    following vowels 1
 * FV2     5    following vowels 2
 * FV3     6    following vowels 3
 * BV1     7    below vowels 1
 * BV2     8    below vowels 2
 * BD      9    below diacritics
 * TONE    10   tonemarks
 * AD1     11   above diacritics 1
 * AD2     12   above diacritics 2
 * AD3     13   above diacritics 3
 * AV1     14   above vowels 1
 * AV2     15   above vowels 2
 * AV3     16   above vowels 3
 * </pre>
 *
 * Functions in thctype.h do basic character classifications while 
 * wtt.h classifies a chracter in detail. Please refer to the reference.
 *
 * TACio_op() checks how to compose two given chracters. The possiblities are 
 * Composible (CP), Non-display (XC), Accept (AC), Reject (RJ) and Strict 
 * Reject (SR). The values of CP, XC, AC, RJ and SR are difined in wtt.h. 
 * And their meanings are:
 *
 *   @li  CP : second character is displayed in the same cell as the first, 
 *             also implies an acceptance. 
 *   @li  XC : Do nothing.
 *   @li  AC : Display second character in the next cell.
 *   @li  RJ : Discard second character.
 *   @li  SR : Reject second character only in strict mode.
 *
 */

#include <thai/thailib.h>

BEGIN_CDECL

/**
 * @brief  Classification of characters in TIS620 according to WTT
 */
typedef enum {
    CTRL    = 0,   /**< control chars */
    NON     = 1,   /**< non composibles */
    CONS    = 2,   /**< consonants */
    LV      = 3,   /**< leading vowels */
    FV1     = 4,   /**< following vowels 1 */
    FV2     = 5,   /**< following vowels 2 */
    FV3     = 6,   /**< following vowels 3 */
    BV1     = 7,   /**< below vowels 1 */
    BV2     = 8,   /**< below vowels 2 */
    BD      = 9,   /**< below diacritics */
    TONE    = 10,  /**< tonemarks */
    AD1     = 11,  /**< above diacritics 1 */
    AD2     = 12,  /**< above diacritics 2 */
    AD3     = 13,  /**< above diacritics 3 */
    AV1     = 14,  /**< above vowels 1 */
    AV2     = 15,  /**< above vowels 2 */
    AV3     = 16   /**< above vowels 3 */
} WTTClass;

/**
 * @brief  Composibility checking tables
 */
typedef enum {
    CP  = 1,  /**< COMPOSIBLE - following char is displayed in the same cell
                              as leading char, also implies ACCEPT */
    XC  = 2,  /**< Non-display */
    AC  = 3,  /**< ACCEPT - display the following char in the next cell */
    RJ  = 4,  /**< REJECT - discard that following char, ignore it */
    SR  = 5   /**< STRICT REJECT - REJECT only if in strict mode */
} WTTOp;

extern WTTClass TACchtype(thchar_t c);

extern WTTOp    TACio_op(thchar_t c1, thchar_t c2);

/*
 * implementation parts
 */

extern short TACchtype_[256];
extern short TACio_op_[17][17];

#define TACchtype(c)     (TACchtype_[c])
#define TACio_op(c1, c2) (TACio_op_[TACchtype(c1)][TACchtype(c2)])

END_CDECL

#endif  /* THAI_WTT_H */

