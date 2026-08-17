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
 * thrend.h - Thai string rendering
 * Created: 2001-05-17
 */

#ifndef THAI_THREND_H
#define THAI_THREND_H

#include <thai/thailib.h>
#include <thai/thcell.h>

BEGIN_CDECL

/**
 * @file   thrend.h
 * @brief  Thai string rendering
 */

/**
 * @brief  Glyph code type
 */
typedef unsigned char thglyph_t;

/**
 * @brief  Blank base glyph, for floating upper/lower vowel
 */
#define TH_BLANK_BASE_GLYPH  0xdd

extern int th_render_cell_tis(struct thcell_t cell,
                              thglyph_t res[], size_t res_sz,
                              int is_decomp_am);

extern int th_render_cell_win(struct thcell_t cell,
                              thglyph_t res[], size_t res_sz,
                              int is_decomp_am);

extern int th_render_cell_mac(struct thcell_t cell,
                              thglyph_t res[], size_t res_sz,
                              int is_decomp_am);


extern int th_render_text_tis(const thchar_t *s,
                              thglyph_t res[], size_t res_sz,
                              int is_decomp_am);

extern int th_render_text_win(const thchar_t *s,
                              thglyph_t res[], size_t res_sz,
                              int is_decomp_am);

extern int th_render_text_mac(const thchar_t *s,
                              thglyph_t res[], size_t res_sz,
                              int is_decomp_am);

END_CDECL

#endif  /* THAI_THREND_H */

