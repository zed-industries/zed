// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_FONT_UNIQUE_NAME_LOOKUP_FONT_TABLE_MATCHER_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_FONT_UNIQUE_NAME_LOOKUP_FONT_TABLE_MATCHER_H_

#include <stddef.h>
#include <stdint.h>

#include <optional>

#include "base/memory/read_only_shared_memory_region.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/font_unique_name_lookup/font_unique_name_table.pb.h"

namespace blink {

// Parses a protobuf received in memory_mapping to build a font lookup
// structure. Allows case-insensitively matching full font names or postscript
// font names against the parsed table by calling MatchName. Used in Blink for
// looking up
// @font-face { src: local(<font_name>) } CSS font face src references.
class BLINK_COMMON_EXPORT FontTableMatcher {
 public:
  // Constructs a FontTableMatcher from a ReadOnlySharedMemoryMapping returned
  // by FontUniqueNameLookup.  Internally parses the Protobuf structure in
  // memory_mapping to build a list of unique font names, which can then be
  // matched using the MatchName method. The ReadOnlySharedMemoryMapping passed
  // in memory_mapping only needs to be alive for the initial construction of
  // FontTableMatcher. After that, FontTableMatcher no longer accesses it.
  explicit FontTableMatcher(
      const base::ReadOnlySharedMemoryMapping& memory_mapping);

  // Takes a FontUniqueNameTable protobuf and serializes it into a newly created
  // ReadonlySharedMemoryMapping. Used only for testing.
  static base::ReadOnlySharedMemoryMapping MemoryMappingFromFontUniqueNameTable(
      const FontUniqueNameTable& font_unique_name_table);

  struct MatchResult {
    std::string font_path;
    uint32_t ttc_index;
  };

  // Given a font full name or font potscript name, match case insensitively
  // against the internal list of unique font names.
  // Return a font filesystem path and a TrueType collection index to identify a
  // font binary to uniquely identify instantiate a font.
  std::optional<MatchResult> MatchName(const std::string& name_request) const;

  // Returns the number of fonts available after parsing the
  // ReadOnlySharedMemoryMapping.
  size_t AvailableFonts() const;

  // Compares this FontTableMatcher to other for whether
  // their internal list of fonts is disjoint. Used only for testing.
  bool FontListIsDisjointFrom(const FontTableMatcher& other) const;

  // When building a FontUniqueNameTable, use this function to prepare and sort
  // the font names in the protobuf datastructure so that the binary search used
  // by calls to MatchName succeeds on ReadOnlySharedMemoryMappings that are
  // handed out to renderers.
  static void SortUniqueNameTableForSearch(FontUniqueNameTable* font_table);

 private:
  FontUniqueNameTable font_table_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_FONT_UNIQUE_NAME_LOOKUP_FONT_TABLE_MATCHER_H_
