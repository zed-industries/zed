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
 * trie.h - Trie data type and functions
 * Created: 2006-08-11
 * Author:  Theppitak Karoonboonyanan <theppitak@gmail.com>
 */

#ifndef __TRIE_H
#define __TRIE_H

#include <datrie/triedefs.h>
#include <datrie/alpha-map.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * @file trie.h
 * @brief Trie data type and functions
 *
 * Trie is a kind of digital search tree, an efficient indexing method with
 * O(1) time complexity for searching. Comparably as efficient as hashing,
 * trie also provides flexibility on incremental matching and key spelling
 * manipulation. This makes it ideal for lexical analyzers, as well as
 * spelling dictionaries.
 *
 * This library is an implementation of double-array structure for representing
 * trie, as proposed by Junichi Aoe. The details of the implementation can be
 * found at http://linux.thai.net/~thep/datrie/datrie.html
 *
 * A Trie is associated with an AlphaMap, a map between actual alphabet
 * characters and the raw characters used to walk through trie.
 * You can define the alphabet set by adding ranges of character codes
 * to it before associating it to a trie. And the keys to be added to the trie
 * must comprise only characters in such ranges. Note that the size of the
 * alphabet set is limited to 256 (TRIE_CHAR_MAX + 1), and the AlphaMap
 * will map the alphabet characters to raw codes in the range 0..255
 * (0..TRIE_CHAR_MAX). The alphabet character ranges need not be continuous,
 * but the mapped raw codes will be continuous, for the sake of compactness
 * of the trie.
 *
 * A new Trie can be created in memory using trie_new(), saved to file using
 * trie_save(), and loaded later with trie_new_from_file().
 * It can even be embeded in another file using trie_fwrite() and read back
 * using trie_fread().
 * After use, Trie objects must be freed using trie_free().
 *
 * Operations on trie include:
 *
 * - Add/delete entries with trie_store() and trie_delete()
 * - Retrieve entries with trie_retrieve()
 * - Walk through trie stepwise with TrieState and its functions
 *   (trie_root(), trie_state_walk(), trie_state_rewind(),
 *   trie_state_clone(), trie_state_copy(),
 *   trie_state_is_walkable(), trie_state_walkable_chars(),
 *   trie_state_is_single(), trie_state_get_data().
 *   And do not forget to free TrieState objects with trie_state_free()
 *   after use.)
 * - Enumerate all keys using trie_enumerate()
 * - Iterate entries using TrieIterator and its functions
 *   (trie_iterator_new(), trie_iterator_next(), trie_iterator_get_key(),
 *   trie_iterator_get_data().
 *   And do not forget to free TrieIterator objects with trie_iterator_free()
 *   after use.)
 */

/**
 * @brief Trie data type
 */
typedef struct _Trie   Trie;

/**
 * @brief Trie enumeration function
 *
 * @param key  : the key of the entry
 * @param data : the data of the entry
 * @param user_data : the user-supplied data on enumerate call
 *
 * @return TRUE to continue enumeration, FALSE to stop
 */
typedef Bool (*TrieEnumFunc) (const AlphaChar  *key,
                              TrieData          key_data,
                              void             *user_data);

/**
 * @brief Trie walking state
 */
typedef struct _TrieState TrieState;


/**
 * @brief Trie iteration state
 */
typedef struct _TrieIterator TrieIterator;

/*-----------------------*
 *   GENERAL FUNCTIONS   *
 *-----------------------*/

Trie *  trie_new (const AlphaMap *alpha_map);

Trie *  trie_new_from_file (const char *path);

Trie *  trie_fread (FILE *file);

void    trie_free (Trie *trie);

size_t  trie_get_serialized_size (Trie *trie);

void    trie_serialize (Trie *trie, uint8 *ptr);

int     trie_save (Trie *trie, const char *path);

int     trie_fwrite (Trie *trie, FILE *file);

Bool    trie_is_dirty (const Trie *trie);


/*------------------------------*
 *   GENERAL QUERY OPERATIONS   *
 *------------------------------*/

Bool    trie_retrieve (const Trie      *trie,
                       const AlphaChar *key,
                       TrieData        *o_data);

Bool    trie_store (Trie *trie, const AlphaChar *key, TrieData data);

Bool    trie_store_if_absent (Trie *trie, const AlphaChar *key, TrieData data);

Bool    trie_delete (Trie *trie, const AlphaChar *key);

Bool    trie_enumerate (const Trie     *trie,
                        TrieEnumFunc    enum_func,
                        void           *user_data);


/*-------------------------------*
 *   STEPWISE QUERY OPERATIONS   *
 *-------------------------------*/

TrieState * trie_root (const Trie *trie);


/*----------------*
 *   TRIE STATE   *
 *----------------*/

TrieState * trie_state_clone (const TrieState *s);

void        trie_state_copy (TrieState *dst, const TrieState *src);

void      trie_state_free (TrieState *s);

void      trie_state_rewind (TrieState *s);

Bool      trie_state_walk (TrieState *s, AlphaChar c);

Bool      trie_state_is_walkable (const TrieState *s, AlphaChar c);

int       trie_state_walkable_chars (const TrieState  *s,
                                     AlphaChar         chars[],
                                     int               chars_nelm);

/**
 * @brief Check for terminal state
 *
 * @param s    : the state to check
 *
 * @return boolean value indicating whether it is a terminal state
 *
 * Check if the given state is a terminal state. A terminal state is a trie
 * state that terminates a key, and stores a value associated with it.
 */
#define   trie_state_is_terminal(s) trie_state_is_walkable((s),0)

Bool      trie_state_is_single (const TrieState *s);

/**
 * @brief Check for leaf state
 *
 * @param s    : the state to check
 *
 * @return boolean value indicating whether it is a leaf state
 *
 * Check if the given state is a leaf state. A leaf state is a terminal state
 * that has no other branch.
 */
#define   trie_state_is_leaf(s) \
    (trie_state_is_single(s) && trie_state_is_terminal(s))

TrieData trie_state_get_data (const TrieState *s);


/*----------------------*
 *    ENTRY ITERATION   *
 *----------------------*/

TrieIterator *  trie_iterator_new (TrieState *s);

void            trie_iterator_free (TrieIterator *iter);

Bool            trie_iterator_next (TrieIterator *iter);

AlphaChar *     trie_iterator_get_key (const TrieIterator *iter);

TrieData        trie_iterator_get_data (const TrieIterator *iter);


#ifdef __cplusplus
}
#endif

#endif  /* __TRIE_H */

/*
vi:ts=4:ai:expandtab
*/
