// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_MARKER_RANGE_MAPPING_CONTEXT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_MARKER_RANGE_MAPPING_CONTEXT_H_

#include "third_party/blink/renderer/core/layout/inline/fragment_item.h"
#include "third_party/blink/renderer/core/layout/inline/offset_mapping.h"
#include "third_party/blink/renderer/core/layout/inline/text_offset_range.h"

namespace blink {

class DocumentMarker;

// Helper for mapping from DOM offset (range) to text content offset.
//
// Exploits the fact that DocumentMarkers are sorted in DOM offset order, to
// maintain a cached starting point within the unit mapping range and thus
// amortize the cost of unit lookup.
class CORE_EXPORT MarkerRangeMappingContext {
  STACK_ALLOCATED();

 private:
  // The internal class that implements the mapping.
  class CORE_EXPORT DOMToTextContentOffsetMapper {
    STACK_ALLOCATED();

   public:
    explicit DOMToTextContentOffsetMapper(const Text& text_node);

    unsigned GetTextContentOffset(unsigned dom_offset) const;

    unsigned GetTextContentOffsetNoCache(unsigned dom_offset) const;

    void Reset() const { units_begin_ = units_.begin(); }

   private:
    base::span<const OffsetMappingUnit> GetMappingUnits(
        const LayoutObject* layout_object);

    // Find the mapping unit for `dom_offset`, starting from `begin`.
    base::span<const OffsetMappingUnit>::iterator FindUnit(
        base::span<const OffsetMappingUnit>::iterator begin,
        unsigned dom_offset) const;

    base::span<const OffsetMappingUnit> units_;
    mutable base::span<const OffsetMappingUnit>::iterator units_begin_;
  };

 public:
  MarkerRangeMappingContext() = delete;

  explicit MarkerRangeMappingContext(const Text& text_node,
                                     const TextOffsetRange& fragment_dom_range)
      : mapper_(DOMToTextContentOffsetMapper(text_node)),
        fragment_dom_range_(fragment_dom_range),
        text_length_(text_node.length()) {}

  // Computes the text fragment offsets for the given marker’s start and end,
  // or returns nullopt if the marker is completely outside the fragment.
  std::optional<TextOffsetRange> GetTextContentOffsets(
      const DocumentMarker&) const;

  void Reset() const { mapper_.Reset(); }

 private:
  const DOMToTextContentOffsetMapper mapper_;
  const TextOffsetRange fragment_dom_range_;
  const unsigned text_length_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_MARKER_RANGE_MAPPING_CONTEXT_H_
