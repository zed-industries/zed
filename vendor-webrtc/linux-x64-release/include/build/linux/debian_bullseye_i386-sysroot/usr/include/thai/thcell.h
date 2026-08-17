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
 * thcell.h - Thai string cell custering
 * Created: 2001-08-08 (split from thrend.h)
 */

#ifndef THAI_THCELL_H
#define THAI_THCELL_H

#include <thai/thailib.h>

BEGIN_CDECL

/**
 * @file   thcell.h
 * @brief  Thai string cell custering
 */

/**
 * @brief  Thai char cell representation
 */
struct thcell_t {
    thchar_t base;      /**< base character */
    thchar_t hilo;      /**< upper/lower vowel/diacritic */
    thchar_t top;       /**< top-level mark */
};

extern void th_init_cell(struct thcell_t *cell);

extern size_t th_next_cell(const thchar_t *s, size_t len,
                           struct thcell_t *cell, int is_decomp_am);

extern size_t th_prev_cell(const thchar_t *s, size_t pos,
                           struct thcell_t *cell, int is_decomp_am);
extern size_t th_make_cells(const thchar_t *s, size_t len,
                            struct thcell_t cells[], size_t *ncells,
                            int is_decomp_am);

END_CDECL

#endif  /* THAI_THCELL_H */

