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
 * thbrk.h - Thai word segmentation
 * Created: 2001-05-17
 */

#ifndef THAI_THBRK_H
#define THAI_THBRK_H

#include <thai/thailib.h>

BEGIN_CDECL

/**
 * @file   thbrk.h
 * @brief  Thai word segmentation
 */

typedef struct _ThBrk ThBrk;

extern ThBrk *th_brk_new(const char *dictpath);

extern void th_brk_delete(ThBrk *brk);

extern int th_brk_find_breaks(ThBrk *brk, const thchar_t *s,
                              int pos[], size_t pos_sz);

extern int th_brk_insert_breaks(ThBrk *brk, const thchar_t *in,
                                thchar_t *out, size_t out_sz,
                                const char *delim);

TH_DEPRECATED_FOR(th_brk_find_breaks)
extern int th_brk(const thchar_t *s, int pos[], size_t pos_sz);

TH_DEPRECATED_FOR(th_brk_insert_breaks)
extern int th_brk_line(const thchar_t *in, thchar_t *out, size_t out_sz,
                       const char *delim);

END_CDECL

#endif  /* THAI_THBRK_H */

