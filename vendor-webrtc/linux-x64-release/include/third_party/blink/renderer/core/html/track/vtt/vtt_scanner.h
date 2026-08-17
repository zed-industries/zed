/*
 * Copyright (c) 2013, Opera Software ASA. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the name of Opera Software ASA nor the names of its
 *    contributors may be used to endorse or promote products derived
 *    from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
 * FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
 * COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT,
 * INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
 * STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
 * OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_VTT_VTT_SCANNER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_VTT_VTT_SCANNER_H_

#include <variant>

#include "base/check_op.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/parsing_utilities.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

// Helper class for "scanning" an input string and performing parsing of
// "micro-syntax"-like constructs.
//
// There's two primary operations: match and scan.
//
// The 'match' operation matches an explicitly or implicitly specified sequence
// against the characters ahead of the current input pointer, and returns true
// if the sequence can be matched.
//
// The 'scan' operation performs a 'match', and if the match is successful it
// advance the input pointer past the matched sequence.
class CORE_EXPORT VTTScanner {
  STACK_ALLOCATED();

 private:
  using LCharSpan = base::span<const LChar>;
  using UCharSpan = base::span<const UChar>;
  using State = std::variant<LCharSpan, UCharSpan>;

  template <typename Functor>
  decltype(auto) Invoke(Functor functor, State& state) {
    return std::holds_alternative<LCharSpan>(state_)
               ? functor(std::get<LCharSpan>(state))
               : functor(std::get<UCharSpan>(state));
  }
  template <typename Functor>
  decltype(auto) Invoke(Functor functor, const State& state) const {
    return std::holds_alternative<LCharSpan>(state_)
               ? functor(std::get<LCharSpan>(state))
               : functor(std::get<UCharSpan>(state));
  }
  template <typename Functor>
  decltype(auto) Invoke(Functor functor) {
    return Invoke(functor, state_);
  }
  template <typename Functor>
  decltype(auto) Invoke(Functor functor) const {
    return Invoke(functor, state_);
  }

 public:
  explicit VTTScanner(const String& line);
  VTTScanner(const VTTScanner&) = delete;
  VTTScanner& operator=(const VTTScanner&) = delete;

  // Return the number of remaining characters.
  size_t Remaining() const {
    return Invoke([](const auto& buf) { return buf.size(); });
  }
  // Check if the input pointer points at the end of the input.
  bool IsAtEnd() const {
    return Invoke([](const auto& buf) { return buf.empty(); });
  }
  // Match the character |c| against the character at the input pointer
  // (~lookahead).
  bool Match(char c) const { return !IsAtEnd() && CurrentChar() == c; }
  // Scan the character |c|.
  bool Scan(char);
  // Scan the string |str|.
  bool Scan(StringView str);

  // Return the count of characters for which the specified
  // |characterPredicate| returns true. The start of the run will be the
  // current input pointer.
  template <bool predicate(UChar)>
  size_t CountWhile() const;

  // Like CountWhile, but using a negated predicate.
  template <bool predicate(UChar)>
  size_t CountUntil() const;

  // Skip (advance the input pointer) as long as the specified
  // |characterPredicate| returns true, and the input pointer is not passed
  // the end of the input.
  template <bool predicate(UChar)>
  void SkipWhile();

  // Like SkipWhile, but using a negated predicate.
  template <bool predicate(UChar)>
  void SkipUntil();

  // Return a scanner with a buffer containing the run of characters for which
  // the specified `predicate` returns true. The characters will be consumed in
  // this scanner.
  template <bool predicate(UChar)>
  VTTScanner SubrangeWhile();

  // Like SubrangeWhile, but using a negated predicate.
  template <bool predicate(UChar)>
  VTTScanner SubrangeUntil();

  // Return the String made up of the first `length` characters, and advance the
  // input pointer accordingly.
  String ExtractString(size_t length);

  // Return a String constructed from the rest of the input (between input
  // pointer and end of input), and advance the input pointer accordingly.
  String RestOfInputAsString();

  // Scan a set of ASCII digits from the input. Return the number of digits
  // scanned, and set |number| to the computed value. If the digits make up a
  // number that does not fit the 'unsigned' type, |number| is set to UINT_MAX.
  // Note: Does not handle sign.
  size_t ScanDigits(unsigned& number);

  // Scan a floating point value on one of the forms: \d+\.? \d+\.\d+ \.\d+
  bool ScanDouble(double& number);

  // Scan a floating point value (per ScanDouble) followed by a '%'.
  bool ScanPercentage(double& percentage);

 protected:
  explicit VTTScanner(State state) : state_(std::move(state)) {}

  UChar CurrentChar() const;
  void Advance(size_t amount = 1);
  void AdvanceIfNonZero(size_t amount);

  State state_;
};

template <bool predicate(UChar)>
inline size_t VTTScanner::CountWhile() const {
  return Invoke([](const auto& buf) {
    auto it = std::ranges::find_if_not(buf, predicate);
    return std::distance(buf.begin(), it);
  });
}

template <bool predicate(UChar)>
inline size_t VTTScanner::CountUntil() const {
  return Invoke([](const auto& buf) {
    auto it = std::ranges::find_if(buf, predicate);
    return std::distance(buf.begin(), it);
  });
}

template <bool predicate(UChar)>
inline void VTTScanner::SkipWhile() {
  AdvanceIfNonZero(CountWhile<predicate>());
}

template <bool predicate(UChar)>
inline void VTTScanner::SkipUntil() {
  AdvanceIfNonZero(CountUntil<predicate>());
}

template <bool predicate(UChar)>
inline VTTScanner VTTScanner::SubrangeWhile() {
  const size_t count = CountWhile<predicate>();
  return VTTScanner(Invoke([count](auto& buf) {
    State collected;
    std::tie(collected, buf) = buf.split_at(count);
    return collected;
  }));
}

template <bool predicate(UChar)>
inline VTTScanner VTTScanner::SubrangeUntil() {
  const size_t count = CountUntil<predicate>();
  return VTTScanner(Invoke([count](auto& buf) {
    State collected;
    std::tie(collected, buf) = buf.split_at(count);
    return collected;
  }));
}

inline UChar VTTScanner::CurrentChar() const {
  return Invoke([](const auto& buf) { return buf.front(); });
}

inline void VTTScanner::Advance(size_t amount) {
  Invoke([amount](auto& buf) { buf = buf.subspan(amount); });
}

inline void VTTScanner::AdvanceIfNonZero(size_t amount) {
  if (amount) {
    Advance(amount);
  }
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_VTT_VTT_SCANNER_H_
