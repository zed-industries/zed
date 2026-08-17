// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TYPES_TOKEN_TYPE_H_
#define BASE_TYPES_TOKEN_TYPE_H_

#include <compare>
#include <type_traits>
#include <utility>

#include "base/check.h"
#include "base/types/strong_alias.h"
#include "base/unguessable_token.h"

namespace base {

// A specialization of StrongAlias for UnguessableToken. Unlike
// UnguessableToken, a TokenType<...> does not default to null and does not
// expose the concept of null tokens. If you need to indicate a null token,
// please use std::optional<TokenType<...>>.
template <typename TypeMarker>
class TokenType : public StrongAlias<TypeMarker, UnguessableToken> {
 private:
  using Super = StrongAlias<TypeMarker, UnguessableToken>;

 public:
  TokenType() : Super(UnguessableToken::Create()) {}
  explicit TokenType(const UnguessableToken& token) : Super(token) {
    // Disallow attempts to force a null UnguessableToken into a strongly-typed
    // token. Allowing in-place nullability of UnguessableToken was a design
    // mistake; do not propagate that mistake here as well.
    CHECK(!token.is_empty());
  }

  // This object allows default assignment operators for compatibility with
  // STL containers.
  TokenType(const TokenType& token) = default;
  TokenType(TokenType&& token) noexcept = default;
  TokenType& operator=(const TokenType& token) = default;
  TokenType& operator=(TokenType&& token) noexcept = default;

  // StrongAlias doesn't define <=> because not all underlying types will
  // implement it. TokenType can define it using UnguessableToken's
  // implementation, though.
  friend constexpr auto operator<=>(const TokenType& lhs,
                                    const TokenType& rhs) {
    return lhs.value() <=> rhs.value();
  }
  friend constexpr bool operator==(const TokenType& lhs, const TokenType& rhs) {
    return lhs.value() == rhs.value();
  }

  template <typename H>
  friend H AbslHashValue(H h, const TokenType& token_type) {
    return H::combine(std::move(h), token_type.value());
  }

  // Hash functor for use in unordered containers.
  struct Hasher {
    using argument_type = TokenType;
    using result_type = size_t;
    result_type operator()(const argument_type& token) const {
      return UnguessableTokenHash()(token.value());
    }
  };

  // Mimic the UnguessableToken API for ease and familiarity of use.
  std::string ToString() const { return this->value().ToString(); }
};

}  // namespace base

#endif  // BASE_TYPES_TOKEN_TYPE_H_
