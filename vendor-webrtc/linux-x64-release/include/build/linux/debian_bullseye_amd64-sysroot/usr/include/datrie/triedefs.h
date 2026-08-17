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
 * triedefs.h - General typedefs for trie
 * Created: 2006-08-11
 * Author:  Theppitak Karoonboonyanan <theppitak@gmail.com>
 */

#ifndef __TRIEDEFS_H
#define __TRIEDEFS_H

#include <datrie/typedefs.h>

/**
 * @file triedefs.h
 * @brief General typedefs for trie
 */

/**
 * @brief Alphabet character type for use as input/output strings of trie keys
 */
typedef uint32         AlphaChar;

/**
 * @brief Error value for alphabet character
 */
#define ALPHA_CHAR_ERROR   (~(AlphaChar)0)

/**
 * @brief Raw character type mapped into packed set from AlphaChar,
 * for use in actual trie transition calculations
 */
typedef unsigned char  TrieChar;
/**
 * @brief Trie terminator character
 */
#define TRIE_CHAR_TERM    '\0'
#define TRIE_CHAR_MAX     255

/**
 * @brief Type of index into Trie double-array and tail structures
 */
typedef int32          TrieIndex;
/**
 * @brief Trie error index
 */
#define TRIE_INDEX_ERROR  0
/**
 * @brief Maximum trie index value
 */
#define TRIE_INDEX_MAX    0x7fffffff

/**
 * @brief Type of value associated to trie entries
 */
typedef int32          TrieData;
/**
 * @brief Trie error data
 */
#define TRIE_DATA_ERROR  -1

#endif  /* __TRIEDEFS_H */

/*
vi:ts=4:ai:expandtab
*/
