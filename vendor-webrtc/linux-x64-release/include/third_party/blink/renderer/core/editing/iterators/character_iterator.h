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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_ITERATORS_CHARACTER_ITERATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_ITERATORS_CHARACTER_ITERATOR_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/range.h"
#include "third_party/blink/renderer/core/editing/forward.h"
#include "third_party/blink/renderer/core/editing/iterators/text_iterator.h"
#include "third_party/blink/renderer/core/editing/iterators/text_iterator_behavior.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

// Builds on the text iterator, adding a character position so we can walk one
// character at a time, or faster, as needed. Useful for searching.
template <typename Strategy>
class CharacterIteratorAlgorithm {
  STACK_ALLOCATED();

 public:
  CharacterIteratorAlgorithm(
      const PositionTemplate<Strategy>& start,
      const PositionTemplate<Strategy>& end,
      const TextIteratorBehavior& = TextIteratorBehavior());
  explicit CharacterIteratorAlgorithm(
      const EphemeralRangeTemplate<Strategy>&,
      const TextIteratorBehavior& = TextIteratorBehavior());

  void Advance(int num_characters);

  bool AtBreak() const { return at_break_; }
  bool AtEnd() const { return text_iterator_.AtEnd(); }

  int length() const { return text_iterator_.length() - run_offset_; }
  UChar CharacterAt(unsigned index) const {
    return text_iterator_.CharacterAt(run_offset_ + index);
  }

  int CharacterOffset() const { return offset_; }

  const Document& OwnerDocument() const;
  const Node& CurrentContainer() const;
  int StartOffset() const;
  int EndOffset() const;

  PositionTemplate<Strategy> GetPositionBefore() const;
  PositionTemplate<Strategy> GetPositionAfter() const;

  // TDOO(editing-dev): We should rename |StartPosition()| to
  // |GetPositionBeforeDeprecated()| and use |GetPositionBefore()| to
  // avoid using |EditingPositionOf()|.
  // Note: Following two tests are failed when using |GetPositionBefore()|
  // instead of |StartPosition()|:
  //  1. extend-by-sentence-002.html
  //  2. move_forward_sentence_empty_line_break.html
  PositionTemplate<Strategy> StartPosition() const;
  // TDOO(editing-dev): We should rename |EndPosition()| to
  // |GetPositionAfterDeprecated()| and use |GetPositionAfter()| to
  // avoid using |EditingPositionOf()|.
  PositionTemplate<Strategy> EndPosition() const;

  EphemeralRangeTemplate<Strategy> CalculateCharacterSubrange(int offset,
                                                              int length);

 private:
  void Initialize();

  int offset_;
  int run_offset_;
  bool at_break_;

  TextIteratorAlgorithm<Strategy> text_iterator_;
};

extern template class CORE_EXTERN_TEMPLATE_EXPORT
    CharacterIteratorAlgorithm<EditingStrategy>;
using CharacterIterator = CharacterIteratorAlgorithm<EditingStrategy>;

extern template class CORE_EXTERN_TEMPLATE_EXPORT
    CharacterIteratorAlgorithm<EditingInFlatTreeStrategy>;
using CharacterIteratorInFlatTree =
    CharacterIteratorAlgorithm<EditingInFlatTreeStrategy>;

CORE_EXPORT EphemeralRange CalculateCharacterSubrange(const EphemeralRange&,
                                                      int character_offset,
                                                      int character_count);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_ITERATORS_CHARACTER_ITERATOR_H_
