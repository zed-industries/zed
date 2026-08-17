/* -*- Mode: C; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 4 -*- */
/*
 * libdatrie - Double-Array Trie Library
 * Copyright (C) 2006  Theppitak Karoonboonyanan <theppitak@gmail.com>
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
 * alpha-map.h - map between character codes and trie alphabet
 * Created: 2006-08-19
 * Author:  Theppitak Karoonboonyanan <theppitak@gmail.com>
 */

#ifndef __ALPHA_MAP_H
#define __ALPHA_MAP_H

#include <stdio.h>

#include <datrie/triedefs.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * @file alpha-map.h
 * @brief AlphaMap data type and functions
 *
 * AlphaMap is a mapping between AlphaChar and TrieChar. AlphaChar is the
 * alphabet character used in words of a target language, while TrieChar
 * is a small integer with packed range of values and is actually used in
 * trie state transition calculations.
 *
 * Since double-array trie relies on sparse state transition table,
 * a small set of input characters can make the table small, i.e. with
 * small number of columns. But in real life, alphabet characters can be
 * of non-continuous range of values. The unused slots between them can
 * waste the space in the table, and can increase the chance of unused
 * array cells.
 *
 * AlphaMap is thus defined for mapping between non-continuous ranges of
 * values of AlphaChar and packed and continuous range of Triechar.
 *
 * In this implementation, TrieChar is defined as a single-byte integer,
 * which means the largest AlphaChar set that is supported is of 255
 * values, as the special value of 0 is reserved for null-termination code.
 */

/**
 * @brief AlphaMap data type
 */
typedef struct _AlphaMap    AlphaMap;

AlphaMap *  alpha_map_new (void);

AlphaMap *  alpha_map_clone (const AlphaMap *a_map);

void        alpha_map_free (AlphaMap *alpha_map);

int         alpha_map_add_range (AlphaMap  *alpha_map,
                                 AlphaChar  begin,
                                 AlphaChar  end);

int         alpha_char_strlen (const AlphaChar *str);
int         alpha_char_strcmp (const AlphaChar *str1, const AlphaChar *str2);

#ifdef __cplusplus
}
#endif

#endif /* __ALPHA_MAP_H */


/*
vi:ts=4:ai:expandtab
*/
