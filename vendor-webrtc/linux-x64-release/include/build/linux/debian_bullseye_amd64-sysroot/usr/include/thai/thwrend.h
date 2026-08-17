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
 * thwrend.h - Thai wide-char string rendering
 * Created: 2001-05-17
 */

#ifndef THAI_THWREND_H
#define THAI_THWREND_H

#include <thai/thailib.h>
#include <thai/thrend.h>
#include <thai/thwchar.h>

BEGIN_CDECL

extern const thwchar_t *th_wcnext_cell(const thwchar_t *s, size_t len,
                                       struct thcell *cell, size_t *nchars);
extern const thwchar_t *th_wcmake_cells(const thwchar_t *s, size_t len,
                                        struct thcell cells[], size_t *ncells);

extern void th_wcrender_cell_tis(struct thcell cell,
                                 thglyph_t res[], size_t res_sz);
extern void th_wcrender_cell_win(struct thcell cell,
                                 thglyph_t res[], size_t res_sz);
extern void th_wcrender_cell_mac(struct thcell cell,
                                 thglyph_t res[], size_t res_sz);

extern int th_wcrender_text_tis(const thwchar_t *s,
                                thglyph_t res[], size_t res_sz);
extern int th_wcrender_text_win(const thwchar_t *s,
                                thglyph_t res[], size_t res_sz);
extern int th_wcrender_text_mac(const thwchar_t *s,
                                thglyph_t res[], size_t res_sz);

END_CDECL

#endif  /* THAI_THWREND_H */

