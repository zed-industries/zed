/*
 * Copyright (C) 2010 Apple Inc. All rights reserved.
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
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. AND ITS CONTRIBUTORS ``AS IS''
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO,
 * THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL APPLE INC. OR ITS CONTRIBUTORS
 * BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF
 * THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_FINDER_FIND_OPTIONS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_FINDER_FIND_OPTIONS_H_

namespace blink {

// This represents a set of flags for string search.
// An object of this class is very small, and we don't need to use
// `const FindOptions&` to pass an object to a function.
class FindOptions {
 public:
  constexpr FindOptions() = default;
  FindOptions(const FindOptions& another) = default;
  FindOptions& operator=(const FindOptions& another) = default;

  bool IsCaseInsensitive() const { return case_insensitive_; }
  constexpr FindOptions& SetCaseInsensitive(bool flag) {
    case_insensitive_ = flag;
    return *this;
  }

  bool IsBackwards() const { return backwards_; }
  constexpr FindOptions& SetBackwards(bool flag) {
    backwards_ = flag;
    return *this;
  }

  bool IsWrappingAround() const { return wrapping_around_; }
  constexpr FindOptions& SetWrappingAround(bool flag) {
    wrapping_around_ = flag;
    return *this;
  }

  bool IsStartingInSelection() const { return starting_in_selection_; }
  constexpr FindOptions& SetStartingInSelection(bool flag) {
    starting_in_selection_ = flag;
    return *this;
  }

  bool IsWholeWord() const { return whole_word_; }
  constexpr FindOptions& SetWholeWord(bool flag) {
    whole_word_ = flag;
    return *this;
  }

  // Used for window.find() or execCommand('find').
  // TODO(yosin) Once find UI works on flat tree and it doesn't use
  // `rangeOfString()`, we should get rid of the FindApiCall flag.
  bool IsFindApiCall() const { return find_api_call_; }
  constexpr FindOptions& SetFindApiCall(bool flag) {
    find_api_call_ = flag;
    return *this;
  }

  bool IsRubySupported() const { return ruby_supported_; }
  constexpr FindOptions& SetRubySupported(bool flag) {
    ruby_supported_ = flag;
    return *this;
  }

 private:
  bool case_insensitive_ : 1 = false;
  bool backwards_ : 1 = false;
  bool wrapping_around_ : 1 = false;
  bool starting_in_selection_ : 1 = false;
  bool whole_word_ : 1 = false;
  bool find_api_call_ : 1 = false;
  bool ruby_supported_ : 1 = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_FINDER_FIND_OPTIONS_H_
