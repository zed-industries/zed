// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_NUMBER_PARSING_OPTIONS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_NUMBER_PARSING_OPTIONS_H_

#include <ostream>

#include "base/check_op.h"

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace WTF {

// Copyable and immutable object representing number parsing flags.
class NumberParsingOptions final {
  STACK_ALLOCATED();

 public:
  // 'Strict' behavior for WTF::String.
  static constexpr NumberParsingOptions Strict() {
    return NumberParsingOptions().SetAcceptLeadingPlus().SetAcceptWhiteSpace();
  }
  // Non-'Strict' behavior for WTF::String.
  static constexpr NumberParsingOptions Loose() {
    return Strict().SetAcceptTrailingGarbage();
  }

  // Construct an instance without any flags set.
  constexpr NumberParsingOptions()
      : accept_trailing_garbage_(false),
        accept_leading_plus_(false),
        accept_leading_trailing_whitespace_(false),
        accept_minus_zero_for_unsigned_(false) {}

  // Returns a new instance by merging |this| and AcceptTrailingGarbage flag.
  constexpr NumberParsingOptions SetAcceptTrailingGarbage() const {
    NumberParsingOptions copy = *this;
    copy.accept_trailing_garbage_ = true;
    return copy;
  }

  // Returns a new instance by merging |this| and AcceptLeadingPlus flag.
  constexpr NumberParsingOptions SetAcceptLeadingPlus() const {
    NumberParsingOptions copy = *this;
    copy.accept_leading_plus_ = true;
    return copy;
  }

  // Returns a new instance by merging |this| and AcceptWhiteSpace flag.
  constexpr NumberParsingOptions SetAcceptWhiteSpace() const {
    NumberParsingOptions copy = *this;
    copy.accept_leading_trailing_whitespace_ = true;
    return copy;
  }

  // Returns a new instance by merging |this| and AcceptMinusZeroForUnsigned
  // flag.
  constexpr NumberParsingOptions SetAcceptMinusZeroForUnsigned() const {
    NumberParsingOptions copy = *this;
    copy.accept_minus_zero_for_unsigned_ = true;
    return copy;
  }

  bool AcceptTrailingGarbage() const { return accept_trailing_garbage_; }
  bool AcceptLeadingPlus() const { return accept_leading_plus_; }
  bool AcceptWhitespace() const { return accept_leading_trailing_whitespace_; }
  bool AcceptMinusZeroForUnsigned() const {
    return accept_minus_zero_for_unsigned_;
  }

 private:
  unsigned accept_trailing_garbage_ : 1;
  unsigned accept_leading_plus_ : 1;
  unsigned accept_leading_trailing_whitespace_ : 1;
  unsigned accept_minus_zero_for_unsigned_ : 1;
};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_NUMBER_PARSING_OPTIONS_H_
