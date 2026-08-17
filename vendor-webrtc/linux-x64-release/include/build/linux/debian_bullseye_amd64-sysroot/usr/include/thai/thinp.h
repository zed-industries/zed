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
 * thinp.h - Thai string input sequence filtering
 * Created: 2001-05-17
 */

#ifndef THAI_THINP_H
#define THAI_THINP_H

#include <thai/thailib.h>
#include <thai/thcell.h>

BEGIN_CDECL

/**
 * @file   thinp.h
 * @brief  Thai string input sequence filtering
 */

/**
 * @brief  Strictness of input sequence checking, according to WTT 2.0
 */
typedef enum {
    ISC_PASSTHROUGH = 0,        /**< No check */
    ISC_BASICCHECK  = 1,        /**< Basic check */
    ISC_STRICT      = 2         /**< Strict check */
} thstrict_t;

extern int th_isaccept(thchar_t c1, thchar_t c2, thstrict_t s);

/**
 * @brief  Input sequence correction info
 */
struct thinpconv_t {
    thchar_t conv[4];  /**< (null-terminated) string to put into input buffer */
    int      offset;   /**< offset (<=0) from cur pos where the conv begin */
};

extern int th_validate(struct thcell_t context, thchar_t c,
                       struct thinpconv_t *conv);

extern int th_validate_leveled(struct thcell_t context, thchar_t c,
                               struct thinpconv_t *conv, thstrict_t s);

END_CDECL

#endif  /* THAI_THINP_H */

