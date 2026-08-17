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
 * thwcoll.h - Thai wide-char string collation
 * Created: 2001-05-17
 */

#ifndef THAI_THWCOLL_H
#define THAI_THWCOLL_H

#include <thai/thailib.h>
#include <thai/thwchar.h>

BEGIN_CDECL

/**
 * @file   thwcoll.h
 * @brief  Thai wide-char string collation
 */


/**
 * @brief  Compare two Thai wide-char strings for collation
 *
 * @param  s1 : first wide-char string (null-terminated)
 * @param  s2 : second wide-char string (null-terminated)
 *
 * @return  < 0 if @a s1 precedes @a s2; 0 if @a s1 equals @a s2;
 *          > 0 if @a s1 succeeds @a s2
 */
extern int    th_wcstrcoll(const thwchar_t *s1, const thwchar_t *s2);

/**
 * @brief  Transform wide-char string into collatable form
 *
 * @param  dest : the destination wide-char string buffer
 * @param  src  : the wide-char string to transform
 * @param  n    : the size of @a dest buffer (as number of elements)
 *
 * @returns  the number of elements required to store the result in @a dest,
 *           excluding the terminating '\\0'.
 *           If the return value >= @a n, @a dest is undefined.
 *
 * Transforms @a src into a form from which comparison using wcscmp() yields
 * the same result as that from th_wcstrcoll(). At most @a n characters are
 * put in @a dest.
 */
extern size_t th_wcstrxfrm(thwchar_t dest[], const thwchar_t *src, size_t n);

END_CDECL

#endif  /* THAI_THCOLL_H */

