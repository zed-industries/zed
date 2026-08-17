/*
 * Copyright (C) 2006, 2007, 2008 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_PLAIN_TEXT_RANGE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_PLAIN_TEXT_RANGE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/forward.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/wtf_size_t.h"

namespace blink {

class ContainerNode;
class Range;
class TextIteratorBehavior;

class CORE_EXPORT PlainTextRange {
  STACK_ALLOCATED();

 public:
  PlainTextRange();
  PlainTextRange(const PlainTextRange&);
  explicit PlainTextRange(wtf_size_t location);
  PlainTextRange(wtf_size_t start, wtf_size_t end);

  bool operator==(const PlainTextRange& other) const {
    return start_ == other.start_ && end_ == other.end_;
  }

  bool operator!=(const PlainTextRange& other) const {
    return !operator==(other);
  }

  wtf_size_t End() const {
    DCHECK(IsNotNull());
    return end_;
  }
  wtf_size_t Start() const {
    DCHECK(IsNotNull());
    return start_;
  }
  bool IsNull() const { return start_ == kNotFound; }
  bool IsNotNull() const { return start_ != kNotFound; }
  wtf_size_t length() const {
    DCHECK(IsNotNull());
    return end_ - start_;
  }

  EphemeralRange CreateRange(const ContainerNode& scope) const;
  EphemeralRange CreateRangeForSelection(const ContainerNode& scope) const;
  EphemeralRange CreateRangeForSelectionIndexing(
      const ContainerNode& scope) const;

  static PlainTextRange Create(const ContainerNode& scope,
                               const EphemeralRange&);
  static PlainTextRange Create(const ContainerNode& scope, const Range&);

 private:
  PlainTextRange& operator=(const PlainTextRange&) = delete;

  EphemeralRange CreateRangeFor(const ContainerNode& scope,
                                const TextIteratorBehavior&) const;

  const wtf_size_t start_;
  const wtf_size_t end_;
};

CORE_EXPORT std::ostream& operator<<(std::ostream&, const PlainTextRange&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_PLAIN_TEXT_RANGE_H_
