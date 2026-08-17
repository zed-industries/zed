// Copyright 2011 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_HUNSPELL_GOOGLE_BDICT_H_
#define THIRD_PARTY_HUNSPELL_GOOGLE_BDICT_H_

#include <stddef.h>
#include <stdint.h>

#include "base/containers/span.h"
#include "base/hash/md5.h"

// BDict (binary dictionary) format. All offsets are little endian.
//
// Header (28 bytes).
//   "BDic" Signature (4 bytes)
//   Version (little endian 4 bytes)
//   Absolute offset in file of the aff info. (4 bytes)
//   Absolute offset in file of the dic table. (4 bytes)
//   (Added by v2.0) MD5 checksum of the aff info and the dic table. (16 bytes)
//
// Aff information:
//   Absolute offset in file of the affix group table (4 bytes)
//   Absolute offset in file of the affix rules table (4 bytes)
//   Absolute offset in file of the replacements table (4 bytes)
//   Absolute offset in file of the "other rules" table (4 bytes)
//
//   The data between the aff header and the affix rules table is the comment
//   from the beginning of the .aff file which often contains copyrights, etc.
//
//   Affix group table:
//     Array of NULL terminated strings. It will end in a double-NULL.
//
//   Affix rules table:
//     List of LF terminated lines. NULL terminated.
//
//   Replacements table:
//     List of pairs of NULL teminated words. The end is indicated by a
//     double-NULL. The first word in the pair is the replacement source, the
//     second is what to replace it with. Example:
//       foo\0bar\0a\0b\0\0
//     for replacing ("foo" with "bar") and ("a" with "b").
//
//   Other rules table:
//     List of LF terminated lines. NULL terminated.
//
//
// Dic table. This stores the .dic file which contains the words in the
// dictionary, and indices for each one that indicate a set of suffixes or
// prefixes that can be applied. We store it in a trie to save space. It
// replaces Hunspell's hash manager.
//
//   0abxxxxx xxxxxxxx (in binary) Leaf node:
//     The number stored in the bits represented by x is the affix index.
//
//     If bit <a> is set, the leaf node has an additional string. Following the
//     2 byte header is a NULL-terminated (possibly 0-length) string that should
//     be appended to the node. This allows long unique endings to be handled
//     efficiently.
//
//     If bit <b> is set, the leaf node has a supplimental list of affix IDs
//     following the ordinary data for the leaf node. These affix group IDs are
//     additional rules for the same word. For example, two prefixes may go
//     with distinct sets of suffixes.
//
//     If the affix index is all 1's, then that means that there is only the
//     supplimental list, and the 13-bit of affix built-in to the node don't
//     count. This is used to represent numbers greater than 13 bits, since
//     the supplimentary list has 16 bits per entry. The node must have a
//     supplimenal list if this is set.
//
//     This additional array is an array of 16-bit little-endian values,
//     terminated by 0xFFFF (since 0 is an affix ID meaning "no affix ID".
//
//   0x110000ab: Lookup node.
//     When <a> is set, addresses are 32-bits relative to the beginning of the
//     dictionary data. When unset, addresses are 16-bits relative to the
//     beginning of this node. All values are little endian.
//
//     When <b> is set, there is one additional entry before the table begins.
//     This is the 0th character. 0 is a common addition (meaning no more data)
//     and this prevents us from having to store entries for all the control
//     characters. This magic element is not counted in the table size.
//
//     The ID byte is followeed by two bytes:
//       XX: First character value in the lookup table.
//       XX: Number of characters in the lookup table.
//
//     This is followed optionally by the entry for 0, and then by a table of
//     size indicated by the second charatcer after the ID.
//
//   1110xxxx: List node with 8-bit addresses.
//     The number of items (max 16) in the list is stored in the bits xxxx.
//     Followed by N (character byte, 8-bit offset) pairs. These offsets are
//     relative to the end of the list of pairs.
//   1111xxxx: List node with 16-bit addresses. Same as above but offsets are
//     2-bytes each. LITTLE ENDIAN!

namespace hunspell {

#pragma pack(push, 1)

class BDict {
 public:
  // File header.
  enum { SIGNATURE = 0x63694442 };
  enum {
    MAJOR_VERSION = 2,
    MINOR_VERSION = 0
  };
  struct Header {
    uint32_t signature;

    // Major versions are incompatible with other major versions. Minor versions
    // should be readable by older programs expecting the same major version.
    uint16_t major_version;
    uint16_t minor_version;

    uint32_t aff_offset;  // Offset of the aff data.
    uint32_t dic_offset;  // Offset of the dic data.

    // Added by version 2.0.
    base::MD5Digest digest;  // MD5 digest of the aff data and the dic data.
  };

  // AFF section ===============================================================

  struct AffHeader {
    uint32_t affix_group_offset;
    uint32_t affix_rule_offset;
    uint32_t rep_offset;  // Replacements table.
    uint32_t other_offset;
  };

  // DIC section ===============================================================

  // Leaf ----------------------------------------------------------------------

  // Leaf nodes have the high bit set to 0.
  enum { LEAF_NODE_TYPE_MASK = 0x80 };  // 10000000
  enum { LEAF_NODE_TYPE_VALUE = 0 };    // 00000000

  // Leaf nodes with additional strings have the next-to-high bit set to 1.
  // This mask/value pair also includes the high bit set to 0 which is the leaf
  // indicator.
  enum { LEAF_NODE_ADDITIONAL_MASK = 0xC0 };   // 11000000
  enum { LEAF_NODE_ADDITIONAL_VALUE = 0x40 };  // 01000000

  // Leaf nodes with an additional array of affix rules following it.
  enum { LEAF_NODE_FOLLOWING_MASK = 0xA0 };  // 10100000
  enum { LEAF_NODE_FOLLOWING_VALUE = 0x20 }; // 00100000

  // The low 5 bits of the leaf node ID byte are the first 5 bits of the affix
  // ID. The following byte is used for the low bits of the affix ID (we don't
  // specify as mask for that).
  enum { LEAF_NODE_FIRST_BYTE_AFFIX_MASK = 0x1F };  // 00011111

  // The maximum affix value that can be stored in the first entry (not in the
  // following list). We reserve all 1's to be a magic value (see next entry)
  // so we can store large numbers somewhere else.
  enum { LEAF_NODE_MAX_FIRST_AFFIX_ID = 0x1FFE };  // 00011111 11111110

  // When the affix built-in to the leaf node (the first one) has too many bits
  // for the space reserved for it (13 bits), then we fill it with this value.
  // This means that the affix doesn't count. The affix will instead be stored
  // in the "following list" which allows up to 16 bits per entry.
  enum { FIRST_AFFIX_IS_UNUSED = 0x1FFF };  // 00011111 11111111

  // The maximum number of leaf nodes we'll read that have the same word and
  // follow each other (the FOLLOWING bit is set).
  enum { MAX_AFFIXES_PER_WORD = 32 };

  // The terminator for the list of following affix group IDs.
  enum { LEAF_NODE_FOLLOWING_LIST_TERMINATOR = 0xFFFF };

  // Lookup --------------------------------------------------------------------

  // Lookup nodes have the first 6 bits set to 110000.
  enum { LOOKUP_NODE_TYPE_MASK = 0xFC };   // 11111100
  enum { LOOKUP_NODE_TYPE_VALUE = 0xC0 };  // 11000000

  // Lookup nodes have the low bit meaning it has a 0th entry, and the
  // next-to-lowest bit indicating whether the offsets are 32-bits. Included
  // in these masks are the lookup ID above.
  enum { LOOKUP_NODE_0TH_MASK = 0xFD };    // 11111110
  enum { LOOKUP_NODE_0TH_VALUE = 0xC1 };   // 11000010
  enum { LOOKUP_NODE_32BIT_MASK = 0xFE};   // 11111110
  enum { LOOKUP_NODE_32BIT_VALUE = 0xC2};  // 11000001

  // List ----------------------------------------------------------------------

  // List nodes have the first 3 bits set to 1.
  enum { LIST_NODE_TYPE_MASK = 0xE0 };   // 11100000
  enum { LIST_NODE_TYPE_VALUE = 0xE0 };  // 11100000

  // The 4th from highest bit indicates a 16 bit (as opposed to 8 bit) list.
  // This mask/value also includes the list ID in the high 3 bits.
  enum { LIST_NODE_16BIT_MASK = 0xF0 };   // 11110000
  enum { LIST_NODE_16BIT_VALUE = 0xF0 };  // 11110000

  // The low 4 bits of the list ID byte are the count.
  enum { LIST_NODE_COUNT_MASK = 0xF };  // 00001111

  // Verifies the specified BDICT is sane. This function checks the BDICT header
  // and compares the MD5 digest of the data with the one in the header.
  static bool Verify(base::span<const uint8_t> bdict_data);
};

#pragma pack(pop)

}  // namespace hunspell

#endif  // THIRD_PARTY_HUNSPELL_GOOGLE_BDICT_H_
