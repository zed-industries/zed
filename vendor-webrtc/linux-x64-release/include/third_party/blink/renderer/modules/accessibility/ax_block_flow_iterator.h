// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_AX_BLOCK_FLOW_ITERATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_AX_BLOCK_FLOW_ITERATOR_H_

#include "third_party/blink/renderer/core/layout/geometry/physical_rect.h"
#include "third_party/blink/renderer/core/layout/inline/text_offset_range.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/geometry/physical_direction.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class AXObject;
class FragmentItem;
class FragmentItems;
class LayoutBlockFlow;
class LayoutObject;
class LayoutText;
class PhysicalBoxFragment;

// This class collects and organizes data to facilitate construction of
// AXInlineTextBoxes for text fragments.
class AXBlockFlowData : public GarbageCollected<AXBlockFlowData> {
 public:
  // Inline formatted content is expressed as fragments for block flow layout.
  // See third_party/blink/renderer/core/layout/inline/fragment_item.h for
  // defails.
  //
  // Fragments are restricted to the following types:
  //   LineItem
  // -   * Groups a series of fragments that are on the same line.
  //   TextItem
  //     * Represents a substring of text is a LayoutText object.
  //   GeneratedTextItem
  //     * Text that is auto-generated rather than explicit such as ellipsis
  //   BoxItem
  //     * Grouping of other content, such as images, anonymous containers, ...
  //
  // Typically the fragmentation is in a flattened form, so if a line has
  // a descendant count of N, then the next N-1 siblings are grouped into the
  // line. BoxItems, may have their descendants as siblings, or children. In the
  // latter case, the BoxItem has a PhysicalBoxFragment pointer to the children.
  // For generation of AXInlineTextBoxes, LineItem and TextItem are the main
  // fragments of interest. Each TextItem corresponds 1:1 with an
  // AXInlineTextBox. The LineItem is used for determining if the next or
  // previous fragment is on the same line, and to determine if a trailing
  // space is required in the text content for an AXInlineTextBox.
  //
  // Each LineItem in the layout fragmentation has a corresponding Line instant
  // containing pertinent details for the construction of AXInlineTextBox
  // objects.
  struct Line {
    // Index of the first fragment on the line. See FindFirstFragment for index
    // calculation.
    wtf_size_t start_index = 0;
    // Total number of fragments on the line.
    wtf_size_t length = 0;
    // Whether or not the line ends on a forced linebreak (e.g. <BR>). In the
    // event of a soft linebreak (wrapping a long line to fit width
    // constraints), inclusion of a trailing space may be required.
    bool forced_break = false;
    // Index within the LayoutText's text content of the break.
    std::optional<wtf_size_t> break_index;
  };

  // A LayoutBlockFlow may contain multiple PhysicalBoxFragments (e.g. due to
  // print pagination). Each PhysicalBoxFragment contains a FragmentItems
  // object, which contains the actual fragments. See FragmentProperties for
  // details on the types of fragments. To uniquely identify a fragment within
  // the block flow, we require the index to indicate the PhysicalBoxFragment,
  // as well as the index within the box fragment's items.
  struct Position {
    // Index of the physical box fragment in the layout block flow container.
    wtf_size_t fragmentainer_index = 0;
    // Index of the fragment item in the physical box fragment.
    wtf_size_t item_index = 0;
  };

  using FragmentIndex = wtf_size_t;

  // Identifies uniquely a text fragment within a LayoutObject.
  using TextFragmentKey = std::pair<const LayoutObject*, FragmentIndex>;

  enum class FailureReason {
    kAtLineBoundary,  // The text fragment is at the beginning / end of a line
    kAtBoxFragment,   // The text fragment is the neighbor of a box fragment
                      // whose fragment items are not part of this BlockFlow.
  };

  using Neighbor = std::variant<AXBlockFlowData::TextFragmentKey,
                                AXBlockFlowData::FailureReason>;

  explicit AXBlockFlowData(LayoutBlockFlow* container);
  virtual ~AXBlockFlowData();

  void Trace(Visitor*) const;

  // Retrieves the position in the flattened indexing of fragment items of the
  // first fragment associated with the object.
  // A block flow layout can contain multiple physical box fragments (e.g.
  // print pagination). The flattened indexing is computed by iterating
  // through each of the box fragments associated with the blow flow layout.
  // In other words, the returned index for the first fragment in the second
  // box is equal to the number of items in the first box. This indexing
  // strategy is used through the API except where explicitly noted to the
  // contrary.
  // Returns std::nullopt if no fragments reference the layout object.
  const std::optional<wtf_size_t> FindFirstFragment(
      const LayoutObject* layout_object) const;
  const Position GetPosition(wtf_size_t index) const;
  const FragmentItem* ItemAt(wtf_size_t index) const;

  // Returns the index-th box fragment for the layout block flow container.
  // Typically the container will have only one box fragment, but will have
  // multiple fragments in cases such as when paginating for printing.
  const PhysicalBoxFragment* BoxFragment(wtf_size_t index) const;

  // Total sum of the fragments in the block flow container.
  wtf_size_t Size() const { return total_fragment_count_; }

  const HeapVector<Line>& GetLines() const { return lines_; }

  WTF::String GetText(wtf_size_t index) const;

  std::optional<Line> GetLine(wtf_size_t index) const;

  Neighbor ComputeNeighborOnLine(FragmentIndex index, bool forward) const;

 private:
  void ProcessLayoutBlock(LayoutBlockFlow* container);
  void ProcessBoxFragment(const PhysicalBoxFragment* box_fragment,
                          wtf_size_t starting_fragment_index);
  bool OnLine(const Line& line, wtf_size_t index) const;
  LayoutText* GetFirstLetterPseudoLayoutText(FragmentIndex index) const;

  Member<LayoutBlockFlow> block_flow_container_;

  // Mapping of a layout object to the first fragment for the object.
  HeapHashMap<Member<const LayoutObject>, wtf_size_t> layout_fragment_map_;

  HeapVector<Line> lines_;

  wtf_size_t total_fragment_count_ = 0;
};

// Iterates through fragments associated with a layout object.
//
// Usage:
//   AXBlockFlowIterator it(ax_object);
//   while(it.Next()) {
//     // process text fragment, extracting the text, character offsets and
//     // next or previous on line fragments.
//   }
//
// Calling a method to extract fragment information such as GetText before
// calling Next() triggers an exception.
//
class MODULES_EXPORT AXBlockFlowIterator {
  STACK_ALLOCATED();

 public:
  AXBlockFlowIterator() = default;
  explicit AXBlockFlowIterator(const AXObject* object);
  virtual ~AXBlockFlowIterator() = default;

  // A FragmentItem pointer can't be a key because FragmentItem instances
  // are stored in HeapVector instances, and Oilpan heap compaction changes
  // addresses of FragmentItem instances. The FragmentItems pointer is stable,
  // and we can find the fragment using the items + index pair.
  using FragmentIndex = wtf_size_t;
  using MapKey = std::pair<const FragmentItems*, FragmentIndex>;

  // Advances to the next fragment item for the same LayoutObject. Returns true
  // if successful, and false if already at the trailing fragment item.
  bool Next();

  const WTF::String& GetText();

  // Returns the offset of each character in the text, may be used to compute
  // bounding boxes of arbitrary text selections. Note: multiple characters may
  // be represented by a single glyph (rendering unit) such as "fi" in a font
  // that supports ligatures. All characters corresponding to the same glyph
  // will share the same character offset. The length of the vector is padded
  // to align with the length of the text, to minimize the potential for
  // misalignment when the text contains characters that cannot be rendered.
  void GetCharacterLayoutPixelOffsets(Vector<int>& offsets);

  AXBlockFlowData::Neighbor NextOnLineAsIndex();
  AXBlockFlowData::Neighbor PreviousOnLineAsIndex();
  const std::optional<MapKey> NextOnLine();
  const std::optional<MapKey> PreviousOnLine();
  const MapKey GetMapKey() const;

  PhysicalRect LocalBounds() const;

  FragmentIndex CurrentFragmentIndex() const { return current_index_.value(); }
  void MoveToIndex(FragmentIndex index) { current_index_ = index; }

  TextOffsetRange TextOffset() const;
  wtf_size_t TextStartOffset() const { return TextOffset().start; }
  wtf_size_t TextEndOffset() const { return TextOffset().end; }
  PhysicalDirection GetDirection() const;

  bool IsLineBreak() const;

  // This version returns the text associated with the fragment without checking
  // if trailing whitespace is needed for serialization.
  static WTF::String GetTextForTesting(MapKey map_key);

 private:
  const AXBlockFlowData* block_flow_data_ = nullptr;
  const LayoutObject* layout_object_ = nullptr;

  std::optional<wtf_size_t> start_index_;
  std::optional<wtf_size_t> current_index_;

  std::optional<WTF::String> text_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_AX_BLOCK_FLOW_ITERATOR_H_
