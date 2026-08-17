// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAGMENT_DIRECTIVE_SAME_BLOCK_WORD_ITERATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAGMENT_DIRECTIVE_SAME_BLOCK_WORD_ITERATOR_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/node.h"
#include "third_party/blink/renderer/core/editing/ephemeral_range.h"
#include "third_party/blink/renderer/core/editing/finder/find_buffer.h"
#include "third_party/blink/renderer/core/editing/iterators/text_iterator.h"
#include "third_party/blink/renderer/core/editing/position.h"
#include "third_party/blink/renderer/platform/text/text_boundaries.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class CORE_EXPORT ForwardDirection {
  STATIC_ONLY(ForwardDirection);

 public:
  static Node* Next(const Node& node) { return FlatTreeTraversal::Next(node); }
  static Node* Next(const Node& node, const Node* stay_within) {
    return FlatTreeTraversal::Next(node, stay_within);
  }
  // Returns given node if it's visible or next closest visible node.
  static Node* AdvanceUntilVisibleTextNode(Node& start_node) {
    return FindBuffer::ForwardVisibleTextNode(start_node);
  }
  // |IsInSameUninterruptedBlock| is diraction specific because |start| and
  // |end| should be in right order.
  static bool IsInSameUninterruptedBlock(Node& start, Node& end) {
    return FindBuffer::IsInSameUninterruptedBlock(start, end);
  }

  // Returns first position of the text according to the direction.
  static int FirstPosition(const String& text) { return 0; }

  // Returns the text in the node before/after the offset.
  static String NodeTextFromOffset(const PositionInFlatTree& position) {
    return RangeText(position, PositionInFlatTree::LastPositionInNode(
                                   *position.ComputeContainerNode()));
  }

  // Returns the position of the next word.
  static int FindNextWordPos(const String& text, int position) {
    return FindNextWordForward(text.Span16(), position);
  }

  // Returns substring of the given text for the given positions.
  static String Substring(const String& text,
                          int start_position,
                          int end_position) {
    return text.Substring(start_position, end_position - start_position);
  }

  // Returns the text within given positions.
  static String RangeText(const PositionInFlatTree& start,
                          const PositionInFlatTree& end) {
    return PlainText(EphemeralRangeInFlatTree(start, end));
  }

  // Returns concatenated string of the given strings.
  static String Concat(const String& start, String end) { return start + end; }
};

class CORE_EXPORT BackwardDirection {
  STATIC_ONLY(BackwardDirection);

 public:
  static Node* Next(const Node& node) {
    return FlatTreeTraversal::Previous(node);
  }
  // Returns given node if it's visible or closest previous visible node.
  static Node* AdvanceUntilVisibleTextNode(Node& start_node) {
    return FindBuffer::BackwardVisibleTextNode(start_node);
  }
  // |IsInSameUninterruptedBlock| is diraction specific because |start| and
  // |end| should be in right order.
  static bool IsInSameUninterruptedBlock(Node& start, Node& end) {
    return FindBuffer::IsInSameUninterruptedBlock(end, start);
  }

  // Returns first position of the text according to the direction.
  static int FirstPosition(const String& text) { return text.length(); }

  // Returns the text in the node before/after the offset.
  static String NodeTextFromOffset(const PositionInFlatTree position) {
    return RangeText(position, PositionInFlatTree::FirstPositionInNode(
                                   *position.ComputeContainerNode()));
  }

  // Returns the position of the previous word.
  static int FindNextWordPos(String text, int position) {
    return FindNextWordBackward(text.Span16(), position);
  }

  // Returns substring of the given text for the given positions.
  static String Substring(const String& text,
                          int start_position,
                          int end_position) {
    return text.Substring(end_position, start_position - end_position);
  }

  // Returns the text within given positions.
  static String RangeText(const PositionInFlatTree& start,
                          const PositionInFlatTree& end) {
    return PlainText(EphemeralRangeInFlatTree(end, start));
  }

  // Returns concatenated string of the given strings.
  static String Concat(const String& start, const String& end) {
    return end + start;
  }
};

// Word iterator that doesn't cross block boundaries.
// Note that if the start position given to this class is in a middle of a word
// then the first word retuned by this class will be a partial word.
template <typename Direction>
class SameBlockWordIterator
    : public GarbageCollected<SameBlockWordIterator<Direction>> {
 public:
  explicit SameBlockWordIterator(const PositionInFlatTree& position);

  // Returns the text from the position that `SameBlockWordIterator` started
  // with until the current position.
  String TextFromStart() const;

  // Moves the current position to the beginning of the next word. Returns true
  // if the text advanced successfully, and false if the iterator reached the
  // end of the block.
  bool AdvanceNextWord();

  void Trace(Visitor* visitor) const;

 private:
  // Moves fields tracking current position to the next node.
  bool NextNode();

  // Returns the next visible text node in the same block as the given node.
  Node* NextVisibleTextNodeWithinBlock(Node& node);

  // Text and offset in text in the current node. The offset is valid only for a
  // text and might not be correct as a position in a node.
  String current_node_text_;
  int current_text_offset_;
  Member<Node> current_node_;

  // Start position
  PositionInFlatTree start_position_;
};

extern template class CORE_EXTERN_TEMPLATE_EXPORT
    SameBlockWordIterator<ForwardDirection>;
extern template class CORE_EXTERN_TEMPLATE_EXPORT
    SameBlockWordIterator<BackwardDirection>;
using ForwardSameBlockWordIterator = SameBlockWordIterator<ForwardDirection>;
using BackwardSameBlockWordIterator = SameBlockWordIterator<BackwardDirection>;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAGMENT_DIRECTIVE_SAME_BLOCK_WORD_ITERATOR_H_
