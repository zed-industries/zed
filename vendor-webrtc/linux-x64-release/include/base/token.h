// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TOKEN_H_
#define BASE_TOKEN_H_

#include <stdint.h>

#include <array>
#include <compare>
#include <optional>
#include <string>
#include <string_view>
#include <utility>

#include "base/base_export.h"
#include "base/containers/span.h"

namespace base {

// A Token is a randomly chosen 128-bit integer. This class supports generation
// from a cryptographically strong random source, or constexpr construction over
// fixed values (e.g. to store a pre-generated constant value). Tokens are
// similar in spirit and purpose to UUIDs, without many of the constraints and
// expectations (such as byte layout and string representation) clasically
// associated with UUIDs.
class BASE_EXPORT Token {
 public:
  // Constructs a zero Token.
  constexpr Token() = default;

  // Constructs a Token with |high| and |low| as its contents.
  constexpr Token(uint64_t high, uint64_t low) : words_{high, low} {}

  constexpr Token(const Token&) = default;
  constexpr Token& operator=(const Token&) = default;
  constexpr Token(Token&&) noexcept = default;
  constexpr Token& operator=(Token&&) = default;

  // Constructs a new Token with random |high| and |low| values taken from a
  // cryptographically strong random source. The result's |is_zero()| is
  // guaranteed to be false.
  static Token CreateRandom();

  // The high and low 64 bits of this Token.
  constexpr uint64_t high() const { return words_[0]; }
  constexpr uint64_t low() const { return words_[1]; }

  constexpr bool is_zero() const { return words_[0] == 0 && words_[1] == 0; }

  span<const uint8_t, 16> AsBytes() const { return as_byte_span(words_); }

  friend constexpr auto operator<=>(const Token& lhs,
                                    const Token& rhs) = default;
  friend constexpr bool operator==(const Token& lhs,
                                   const Token& rhs) = default;

  template <typename H>
  friend H AbslHashValue(H h, const Token& token) {
    return H::combine(std::move(h), token.words_);
  }

  // Generates a string representation of this Token useful for e.g. logging.
  std::string ToString() const;

  // FromString is the opposite of ToString. It returns std::nullopt if the
  // |string_representation| is invalid.
  static std::optional<Token> FromString(
      std::string_view string_representation);

 private:
  // Note: Two uint64_t are used instead of uint8_t[16] in order to have a
  // simpler implementation, particularly for |ToString()|, |is_zero()|, and
  // constexpr value construction.

  std::array<uint64_t, 2> words_ = {0, 0};
};

// For use in std::unordered_map.
struct BASE_EXPORT TokenHash {
  size_t operator()(const Token& token) const;
};

class Pickle;
class PickleIterator;

// For serializing and deserializing Token values.
BASE_EXPORT void WriteTokenToPickle(Pickle* pickle, const Token& token);
BASE_EXPORT std::optional<Token> ReadTokenFromPickle(
    PickleIterator* pickle_iterator);

}  // namespace base

#endif  // BASE_TOKEN_H_
