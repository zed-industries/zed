/*
 * Copyright (C) 2004, 2006, 2009 Apple Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_ITERATORS_TEXT_ITERATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_ITERATORS_TEXT_ITERATOR_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/range.h"
#include "third_party/blink/renderer/core/editing/finder/find_options.h"
#include "third_party/blink/renderer/core/editing/forward.h"
#include "third_party/blink/renderer/core/editing/iterators/fully_clipped_state_stack.h"
#include "third_party/blink/renderer/core/editing/iterators/text_iterator_behavior.h"
#include "third_party/blink/renderer/core/editing/iterators/text_iterator_text_node_handler.h"
#include "third_party/blink/renderer/core/editing/iterators/text_iterator_text_state.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

CORE_EXPORT String
PlainText(const EphemeralRange&,
          const TextIteratorBehavior& = TextIteratorBehavior());

CORE_EXPORT String
PlainText(const EphemeralRangeInFlatTree&,
          const TextIteratorBehavior& = TextIteratorBehavior());

// Iterates through the DOM range, returning all the text, and 0-length
// boundaries at points where replaced elements break up the text flow.  The
// text comes back in chunks so as to optimize for performance of the iteration.

template <typename Strategy>
class TextIteratorAlgorithm {
  STACK_ALLOCATED();

 public:
  // [start, end] indicates the document range that the iteration should take
  // place within (both ends inclusive).
  TextIteratorAlgorithm(const PositionTemplate<Strategy>& start,
                        const PositionTemplate<Strategy>& end,
                        const TextIteratorBehavior& = TextIteratorBehavior());

  // Same behavior as previous constructor but takes an EphemeralRange instead
  // of two Positions
  TextIteratorAlgorithm(const EphemeralRangeTemplate<Strategy>&,
                        const TextIteratorBehavior& = TextIteratorBehavior());

  ~TextIteratorAlgorithm();

  bool AtEnd() const { return !text_state_.PositionNode() || should_stop_; }
  void Advance();
  bool IsInsideAtomicInlineElement() const;

  EphemeralRangeTemplate<Strategy> Range() const;
  const Node* GetNode() const;

  const Document& OwnerDocument() const;
  const Node& CurrentContainer() const;
  int StartOffsetInCurrentContainer() const;
  int EndOffsetInCurrentContainer() const;
  PositionTemplate<Strategy> StartPositionInCurrentContainer() const;
  PositionTemplate<Strategy> EndPositionInCurrentContainer() const;

  // Returns the position before |char16_offset| in current text run.
  PositionTemplate<Strategy> GetPositionBefore(int char16_offset) const;

  // Returns the position after |char16_offset| in current text run.
  PositionTemplate<Strategy> GetPositionAfter(int char16_offset) const;

  const TextIteratorTextState& GetTextState() const { return text_state_; }
  int length() const { return text_state_.length(); }
  UChar CharacterAt(unsigned index) const {
    return text_state_.CharacterAt(index);
  }

  bool BreaksAtReplacedElement() {
    return !behavior_.DoesNotBreakAtReplacedElement();
  }

  // Computes the length of the given range using a text iterator according to
  // the specified iteration behavior. The default iteration behavior is to
  // always emit object replacement characters for replaced elements.
  // TODO(editing-dev): We should remove start/end version of |RangeLength()|.
  static int RangeLength(
      const PositionTemplate<Strategy>& start,
      const PositionTemplate<Strategy>& end,
      const TextIteratorBehavior& =
          TextIteratorBehavior::DefaultRangeLengthBehavior());

  static int RangeLength(
      const EphemeralRangeTemplate<Strategy>&,
      const TextIteratorBehavior& =
          TextIteratorBehavior::DefaultRangeLengthBehavior());

  static bool ShouldEmitTabBeforeNode(const Node&);
  static bool ShouldEmitNewlineBeforeNode(const Node&);
  static bool ShouldEmitNewlineAfterNode(const Node&);
  static bool ShouldEmitNewlineForNode(const Node&, bool emits_original_text);

  static bool SupportsAltText(const Node&);

 private:
  enum IterationProgress {
    kHandledNone,
    kHandledOpenShadowRoots,
    kHandledUserAgentShadowRoot,
    kHandledNode,
    kHandledChildren
  };

  void EmitChar16AfterNode(UChar code_unit, const Node& node);
  void EmitChar16AsNode(UChar code_unit, const Node& node);
  void EmitChar16BeforeNode(UChar code_unit, const Node& node);

  void ExitNode();
  bool ShouldRepresentNodeOffsetZero();
  bool ShouldEmitSpaceBeforeAndAfterNode(const Node&);
  void RepresentNodeOffsetZero();

  // Returns true if text is emitted from the remembered progress (if any).
  bool HandleRememberedProgress();

  void HandleTextNode();
  void HandleReplacedElement();
  void HandleNonTextNode();

  // Used by selection preservation code. There should be one character emitted
  // between every VisiblePosition in the Range used to create the TextIterator.
  // FIXME <rdar://problem/6028818>: This functionality should eventually be
  // phased out when we rewrite moveParagraphs to not clone/destroy moved
  // content.
  bool EmitsCharactersBetweenAllVisiblePositions() const {
    return behavior_.EmitsCharactersBetweenAllVisiblePositions();
  }

  bool EntersTextControls() const { return behavior_.EntersTextControls(); }

  // Used in pasting inside password field.
  bool EmitsOriginalText() const { return behavior_.EmitsOriginalText(); }

  // Used when the visibility of the style should not affect text gathering.
  bool IgnoresStyleVisibility() const {
    return behavior_.IgnoresStyleVisibility();
  }

  // Used when the iteration should stop if form controls are reached.
  bool StopsOnFormControls() const { return behavior_.StopsOnFormControls(); }

  bool EmitsImageAltText() const { return behavior_.EmitsImageAltText(); }

  bool EntersOpenShadowRoots() const {
    return behavior_.EntersOpenShadowRoots();
  }

  bool EmitsObjectReplacementCharacter() const {
    return behavior_.EmitsObjectReplacementCharacter();
  }

  bool ExcludesAutofilledValue() const {
    return behavior_.ExcludeAutofilledValue();
  }

  bool DoesNotBreakAtReplacedElement() const {
    return behavior_.DoesNotBreakAtReplacedElement();
  }

  // Clipboard should respect user-select style attribute
  bool SkipsUnselectableContent() const {
    return behavior_.SkipsUnselectableContent();
  }

  bool IsBetweenSurrogatePair(unsigned position) const;

  // Ensure container node of current text run for computing position.
  void EnsurePositionContainer() const;

  // The range.
  const Node* const start_container_;
  const unsigned start_offset_;
  const Node* const end_container_;
  const unsigned end_offset_;
  // |end_node_| stores |Strategy::ChildAt(*end_container_, end_offfset_ - 1)|,
  // if it exists, or |nullptr| otherwise.
  const Node* const end_node_;
  const Node* const past_end_node_;

  // Current position, not necessarily of the text being returned, but position
  // as we walk through the DOM tree.
  const Node* node_;
  IterationProgress iteration_progress_;
  FullyClippedStateStackAlgorithm<Strategy> fully_clipped_stack_;
  unsigned shadow_depth_;

  // Used when there is still some pending text from the current node; when
  // these are false, we go back to normal iterating.
  bool needs_another_newline_ = false;
  bool needs_handle_replaced_element_ = false;

  const Text* last_text_node_ = nullptr;

  const TextIteratorBehavior behavior_;

  // Used when stopsOnFormControls() is true to determine if the iterator should
  // keep advancing.
  bool should_stop_ = false;
  // Used for use counter |InnerTextWithShadowTree| and
  // |SelectionToStringWithShadowTree|, we should not use other purpose.
  bool handle_shadow_root_ = false;

  // Contains state of emitted text.
  TextIteratorTextState text_state_;

  // Helper for extracting text content from text nodes.
  TextIteratorTextNodeHandler text_node_handler_;
};

extern template class CORE_EXTERN_TEMPLATE_EXPORT
    TextIteratorAlgorithm<EditingStrategy>;
extern template class CORE_EXTERN_TEMPLATE_EXPORT
    TextIteratorAlgorithm<EditingInFlatTreeStrategy>;

using TextIterator = TextIteratorAlgorithm<EditingStrategy>;
using TextIteratorInFlatTree = TextIteratorAlgorithm<EditingInFlatTreeStrategy>;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_ITERATORS_TEXT_ITERATOR_H_
