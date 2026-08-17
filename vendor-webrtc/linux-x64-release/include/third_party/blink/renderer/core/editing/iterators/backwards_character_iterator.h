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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_ITERATORS_BACKWARDS_CHARACTER_ITERATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_ITERATORS_BACKWARDS_CHARACTER_ITERATOR_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/forward.h"
#include "third_party/blink/renderer/core/editing/iterators/simplified_backwards_text_iterator.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

template <typename Strategy>
class BackwardsCharacterIteratorAlgorithm {
  STACK_ALLOCATED();

 public:
  BackwardsCharacterIteratorAlgorithm(
      const EphemeralRangeTemplate<Strategy>&,
      const TextIteratorBehavior& = TextIteratorBehavior());

  void Advance(int);

  bool AtEnd() const { return text_iterator_.AtEnd(); }

  int length() const { return text_iterator_.length() - run_offset_; }
  UChar CharacterAt(unsigned index) const {
    return text_iterator_.CharacterAt(run_offset_ + index);
  }

  PositionTemplate<Strategy> EndPosition() const;

 private:
  int offset_;
  int run_offset_;
  bool at_break_;

  SimplifiedBackwardsTextIteratorAlgorithm<Strategy> text_iterator_;
};

extern template class CORE_EXTERN_TEMPLATE_EXPORT
    BackwardsCharacterIteratorAlgorithm<EditingStrategy>;
extern template class CORE_EXTERN_TEMPLATE_EXPORT
    BackwardsCharacterIteratorAlgorithm<EditingInFlatTreeStrategy>;

using BackwardsCharacterIterator =
    BackwardsCharacterIteratorAlgorithm<EditingStrategy>;

using BackwardsCharacterIteratorInFlatTree =
    BackwardsCharacterIteratorAlgorithm<EditingInFlatTreeStrategy>;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_ITERATORS_BACKWARDS_CHARACTER_ITERATOR_H_
