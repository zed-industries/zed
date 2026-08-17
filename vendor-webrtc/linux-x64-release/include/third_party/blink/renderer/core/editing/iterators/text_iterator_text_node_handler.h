// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_ITERATORS_TEXT_ITERATOR_TEXT_NODE_HANDLER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_ITERATORS_TEXT_ITERATOR_TEXT_NODE_HANDLER_H_

#include "third_party/blink/renderer/core/dom/text.h"
#include "third_party/blink/renderer/core/editing/iterators/text_iterator_behavior.h"
#include "third_party/blink/renderer/core/layout/inline/offset_mapping.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class TextIteratorTextState;

// TextIteratorTextNodeHandler extracts plain text from a text node by calling
// HandleTextNode() function. It should be used only by TextIterator.
class TextIteratorTextNodeHandler {
  STACK_ALLOCATED();

 public:
  TextIteratorTextNodeHandler(const TextIteratorBehavior&,
                              TextIteratorTextState*);
  TextIteratorTextNodeHandler(const TextIteratorTextNodeHandler&) = delete;
  ~TextIteratorTextNodeHandler() { mapping_units_.clear(); }
  TextIteratorTextNodeHandler& operator=(const TextIteratorTextNodeHandler&) =
      delete;

  const Text* GetNode() const { return text_node_; }

  // Returns true if more text is emitted without traversing to the next node.
  bool HandleRemainingTextRuns();

  // Emit plain text from the given text node.
  void HandleTextNodeWhole(const Text*);

  // Variants that emit plain text within the given DOM offset range.
  void HandleTextNodeStartFrom(const Text*, unsigned start_offset);
  void HandleTextNodeEndAt(const Text*, unsigned end_offset);
  void HandleTextNodeInRange(const Text*,
                             unsigned start_offset,
                             unsigned end_offset);

 private:
  void HandleTextNodeWithLayoutNG();

  // Used when the visibility of the style should not affect text gathering.
  bool IgnoresStyleVisibility() const {
    return behavior_.IgnoresStyleVisibility();
  }

  // The current text node and offset range, from which text should be emitted.
  const Text* text_node_ = nullptr;
  unsigned offset_ = 0;
  unsigned end_offset_ = 0;

  // UnitVector for text_node_. This is available only if uses_layout_ng_.
  OffsetMapping::UnitVector mapping_units_;
  wtf_size_t mapping_units_index_ = 0;

  const TextIteratorBehavior behavior_;

  // Contains state of emitted text.
  TextIteratorTextState& text_state_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_ITERATORS_TEXT_ITERATOR_TEXT_NODE_HANDLER_H_
