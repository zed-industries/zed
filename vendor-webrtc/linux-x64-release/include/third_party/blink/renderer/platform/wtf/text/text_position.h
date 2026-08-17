/*
 * Copyright (C) 2010, Google Inc. All rights reserved.
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
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. AND ITS CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL APPLE INC. OR ITS CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH
 * DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_TEXT_POSITION_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_TEXT_POSITION_H_

#include <memory>
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "third_party/blink/renderer/platform/wtf/wtf_export.h"
#include "third_party/blink/renderer/platform/wtf/wtf_size_t.h"

namespace WTF {

// An abstract number of element in a sequence. The sequence has a first
// element.  This type should be used instead of integer because 2
// contradicting traditions can call a first element '0' or '1' which makes
// integer type ambiguous.
class OrdinalNumber final {
  DISALLOW_NEW();

 public:
  static OrdinalNumber FromZeroBasedInt(int zero_based_int) {
    return OrdinalNumber(zero_based_int);
  }
  static OrdinalNumber FromOneBasedInt(int one_based_int) {
    return OrdinalNumber(one_based_int - 1);
  }

  // Use First() instead.
  OrdinalNumber() = delete;

  int ZeroBasedInt() const { return zero_based_value_; }
  int OneBasedInt() const { return zero_based_value_ + 1; }

  bool operator==(OrdinalNumber other) const {
    return zero_based_value_ == other.zero_based_value_;
  }
  bool operator!=(OrdinalNumber other) const { return !((*this) == other); }

  static OrdinalNumber First() { return OrdinalNumber(0); }
  static OrdinalNumber BeforeFirst() { return OrdinalNumber(-1); }

 private:
  OrdinalNumber(int zero_based_int) : zero_based_value_(zero_based_int) {}
  int zero_based_value_;
};

// TextPosition structure specifies coordinates within an text resource. It is
// used mostly
// for saving script source position.
class TextPosition final {
  DISALLOW_NEW();

 public:
  TextPosition(OrdinalNumber line, OrdinalNumber column)
      : line_(line), column_(column) {}

  // Use MinimumPosition() instead.
  TextPosition() = delete;

  bool operator==(const TextPosition& other) const {
    return line_ == other.line_ && column_ == other.column_;
  }
  bool operator!=(const TextPosition& other) const {
    return !((*this) == other);
  }
  WTF_EXPORT OrdinalNumber ToOffset(const Vector<unsigned>&);

  // A 'minimum' value of position, used as a default value.
  static TextPosition MinimumPosition() {
    return TextPosition(OrdinalNumber::First(), OrdinalNumber::First());
  }

  // A value with line value less than a minimum; used as an impossible
  // position.
  static TextPosition BelowRangePosition() {
    return TextPosition(OrdinalNumber::BeforeFirst(),
                        OrdinalNumber::BeforeFirst());
  }

  // A value corresponding to a position with given offset within text having
  // the specified line ending offsets.
  WTF_EXPORT static TextPosition FromOffsetAndLineEndings(
      unsigned,
      const Vector<unsigned>&);

  OrdinalNumber line_;
  OrdinalNumber column_;
};

WTF_EXPORT std::unique_ptr<Vector<wtf_size_t>> GetLineEndings(const String&);

}  // namespace WTF

using WTF::OrdinalNumber;

using WTF::TextPosition;

using WTF::GetLineEndings;

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_TEXT_POSITION_H_
