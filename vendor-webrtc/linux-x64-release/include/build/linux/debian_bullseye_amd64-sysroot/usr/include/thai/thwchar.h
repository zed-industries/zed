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
 * thwchar.h - wide char support for Thai
 * Created: 2001-05-17
 */

#ifndef THAI_THWCHAR_H
#define THAI_THWCHAR_H

#include <thai/thailib.h>
#include <wchar.h>

BEGIN_CDECL

/**
 * @file   thwchar.h
 * @brief  Wide char support for Thai
 */

/**
 * @brief  Wide-character value indicating error
 */
#define THWCHAR_ERR  (~(thwchar_t)0)

/**
 * @brief  Thai character type for storing Unicode character
 */
typedef wchar_t thwchar_t;

extern thwchar_t th_tis2uni(thchar_t c);

extern int th_tis2uni_line(const thchar_t* s, thwchar_t* result, size_t n);

extern thwchar_t th_winthai2uni(thchar_t c);
extern thwchar_t th_macthai2uni(thchar_t c);

extern thchar_t th_uni2tis(thwchar_t wc);

extern int th_uni2tis_line(const thwchar_t* s, thchar_t* result, size_t n);

extern thchar_t th_uni2winthai(thwchar_t wc);
extern thchar_t th_uni2macthai(thwchar_t wc);

END_CDECL

#endif  /* THAI_THWCHAR_H */

