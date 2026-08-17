// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_FINDER_FIND_BUFFER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_FINDER_FIND_BUFFER_H_

#include <optional>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/finder/find_options.h"
#include "third_party/blink/renderer/core/editing/iterators/text_searcher_icu.h"
#include "third_party/blink/renderer/core/editing/position.h"

namespace blink {

class CorpusChunk;
class FindResults;
class LayoutBlockFlow;
class Node;
class OffsetMapping;

enum class RubySupport {
  // No support.
  kDisabled,
  // Support if a target IFC contains a <ruby>.
  kEnabledIfNecessary,
  // Support always.
  kEnabledForcefully,
};

// Buffer for find-in-page, collects text until it meets a block/other
// delimiters. Uses TextSearcherICU to find match in buffer.
// See doc at https://goo.gl/rnXjBu
class CORE_EXPORT FindBuffer {
  STACK_ALLOCATED();

 public:
  explicit FindBuffer(const EphemeralRangeInFlatTree& range,
                      RubySupport ruby_support = RubySupport::kDisabled);

  static EphemeralRangeInFlatTree FindMatchInRange(
      const EphemeralRangeInFlatTree& range,
      String search_text,
      const FindOptions,
      std::optional<base::TimeDelta> timeout_ms = std::nullopt);

  // Returns the closest ancestor of |start_node| (including the node itself)
  // that is block level.
  static const Node& GetFirstBlockLevelAncestorInclusive(
      const Node& start_node);

  // Returns true if start and end nodes are in the same layout block flow and
  // there are no nodes in between that can be considered blocks. Otherwise,
  // returns false.
  static bool IsInSameUninterruptedBlock(const Node& start_node,
                                         const Node& end_node);

  // See |GetVisibleTextNode|.
  static Node* ForwardVisibleTextNode(Node& start_node,
                                      Node* past_last_node = nullptr);
  static Node* BackwardVisibleTextNode(Node& start_node);

  static bool ShouldIgnoreContents(const Node& node);
  static std::optional<UChar> CharConstantForNode(const Node& node);

  // Finds all the match for |search_text| in |buffer_|.
  FindResults FindMatches(const String& search_text,
                          const blink::FindOptions options);

  // Gets a flat tree range corresponding to text in the [start_index,
  // end_index) of |buffer|.
  EphemeralRangeInFlatTree RangeFromBufferIndex(unsigned start_index,
                                                unsigned end_index) const;

  // Returns a position at which the next FindBuffer should start.
  //
  // This function returns the node next to the end node of the specified
  // `range`. If the end position of the `range` points the middle of a Text
  // node, this function skips a part of the Text after the position. Usually
  // this behavior won't cause problems because we don't need to search text
  // after the end position.
  PositionInFlatTree PositionAfterBlock() const {
    if (!node_after_block_)
      return PositionInFlatTree();
    return PositionInFlatTree::FirstPositionInNode(*node_after_block_);
  }

  bool IsInvalidMatch(MatchResultICU match) const;

  Vector<String> BuffersForTesting() const;

 private:
  // Collects text for one LayoutBlockFlow located within |range| to |buffer_|,
  // might be stopped without finishing one full LayoutBlockFlow  if we
  // encountered another LayoutBLockFlow, or if the end of |range| is
  // surpassed. Saves the next starting node after the block (first node in
  // another LayoutBlockFlow or after |end_position|) to |node_after_block_|.
  void CollectTextUntilBlockBoundary(const EphemeralRangeInFlatTree& range,
                                     RubySupport ruby_support);

  // Replaces nodes that should be ignored with appropriate char constants.
  static void ReplaceNodeWithCharConstants(const Node& node,
                                           Vector<UChar>& buffer);

  // Mapping for position in buffer -> actual node where the text came from,
  // along with the offset in the OffsetMapping of this find_buffer.
  // This is needed because when we find a match in the buffer, we want to know
  // where it's located in the OffsetMapping for this FindBuffer.
  // Example: (assume there are no whitespace)
  // <div>
  //  aaa
  //  <span style="float:right;">bbb<span>ccc</span></span>
  //  ddd
  // </div>
  // We can finish FIP with three FindBuffer runs:
  // Run #1, 1 BufferNodeMapping with mapping text = "aaa\uFFFCddd",
  // The "\uFFFC" is the object replacement character created by the float.
  // For text node "aaa", oib = 0, oim = 0.
  // Content of |buffer_| = "aaa".
  // Run #2, 2 BufferNodeMappings, with mapping text = "bbbccc",
  //  1. For text node "bbb", oib = 0, oim = 0.
  //  2. For text node "ccc", oib = 3, oim = 3.
  // Content of |buffer_| = "bbbccc".
  // Run #3, 1 BufferNodeMapping with mapping text = "aaa\uFFFCddd",
  // For text node "ddd", oib = 0, oim = 4.
  // Content of |buffer_| = "ddd".
  // Since the LayoutBlockFlow for "aaa" and "ddd" is the same, they have the
  // same OffsetMapping, the |offset_in_mapping_| for the BufferNodeMapping in
  // run #3 is 4 (the index of first "d" character in the mapping text).
  struct BufferNodeMapping {
    const unsigned offset_in_buffer;
    const unsigned offset_in_mapping;
  };

  const BufferNodeMapping* MappingForIndex(unsigned index) const;

  PositionInFlatTree PositionAtStartOfCharacterAtIndex(unsigned index) const;

  PositionInFlatTree PositionAtEndOfCharacterAtIndex(unsigned index) const;

  Vector<UChar> SerializeLevelInGraph(
      const HeapVector<Member<CorpusChunk>>& chunk_list,
      const String& level,
      const EphemeralRangeInFlatTree& range);

  // Adds text in |text_node| that are located within |range| to |buffer|.
  void AddTextToBuffer(const Text& text_node,
                       const EphemeralRangeInFlatTree& range,
                       Vector<UChar>& buffer,
                       Vector<BufferNodeMapping>* mappings);

  const Node* node_after_block_ = nullptr;
  Vector<UChar> buffer_;
  // buffer_list_ is usually empty. It contains items only if an element
  // with display:ruby-text exists.
  Vector<Vector<UChar>> buffer_list_;
  Vector<BufferNodeMapping> buffer_node_mappings_;
  TextSearcherICU text_searcher_;

  const OffsetMapping* offset_mapping_ = nullptr;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_FINDER_FIND_BUFFER_H_
