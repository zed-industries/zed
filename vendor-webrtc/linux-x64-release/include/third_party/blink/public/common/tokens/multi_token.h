// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Helper class for a strongly-typed multiple variant token. Allows creating
// a token that can represent one of a collection of distinct token types.
// It would be great to replace this with a much simpler C++17 std::variant
// when that is available.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_TOKENS_MULTI_TOKEN_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_TOKENS_MULTI_TOKEN_H_

#include <stdint.h>

#include <compare>
#include <limits>
#include <type_traits>
#include <variant>

#include "base/types/variant_util.h"
#include "base/unguessable_token.h"
#include "third_party/blink/public/common/tokens/multi_token_internal.h"

namespace blink {

// `MultiToken<Tokens...>` is a variant of 2 or more token types. Each token
// type must be an instantiation of `base::TokenType`, and each token type must
// be unique within `Tokens...`. Unlike `base::UnguessableToken`, a `MultiToken`
// is always valid: there is no null state. Default constructing a `MultiToken`
// will create a `MultiToken` containing an instance of the first token type in
// `Tokens`.
//
// Usage:
//
// using CowToken = base::TokenType<class CowTokenTag>;
// using GoatToken = base::TokenType<class GoatTokenTag>;
// using UngulateToken = blink::MultiToken<CowToken, GoatToken>;
//
// void TeleportCow(const CowToken&);
// void TeleportGoat(const GoatToken&);
//
// void TeleportUngulate(const UngulateToken& token) {
//   token.Visit(base::Overloaded(
//         [](const CowToken& cow_token) { TeleportCow(cow_token); },
//         [](const GoatToken& goat_token) { TeleportGoat(goat_token); }));
// }
template <typename... Tokens>
  requires(sizeof...(Tokens) > 1 &&
           sizeof...(Tokens) <= std::numeric_limits<uint32_t>::max() &&
           (internal::IsBaseToken<Tokens> && ...) &&
           internal::AreAllUnique<Tokens...>)
class MultiToken {
 public:
  using Storage = std::variant<Tokens...>;

  // In an ideal world, this would use StrongAlias, but a StrongAlias is not
  // usable in a switch statement, even when the underlying type is integral.
  enum class Tag : uint32_t {};

  // A default constructed token will hold a default-constructed instance (i.e.
  // randomly initialised) of the first token type in `Tokens...`.
  MultiToken() = default;

  template <typename T>
    requires(internal::IsBaseToken<T> && internal::IsCompatible<T, Tokens...>)
  // NOLINTNEXTLINE(google-explicit-constructor)
  MultiToken(const T& token) : storage_(token) {}
  MultiToken(const MultiToken&) = default;

  // Construct from another compatible MultiToken.
  template <typename... Ts>
    requires(internal::IsCompatible<Ts, Tokens...> && ...)
  explicit MultiToken(const MultiToken<Ts...>& multi_token)
      : MultiToken(multi_token.Visit(
            [](const auto& token) { return MultiToken(token); })) {}

  template <typename T>
    requires(internal::IsBaseToken<T> && internal::IsCompatible<T, Tokens...>)
  MultiToken& operator=(const T& token) {
    storage_ = token;
    return *this;
  }
  MultiToken& operator=(const MultiToken&) = default;

  // Assign from another compatible MultiToken.
  template <typename... Ts>
    requires(internal::IsCompatible<Ts, Tokens...> && ...)
  MultiToken& operator=(const MultiToken<Ts...>& multi_token) {
    return *this = multi_token.Visit(
               [](const auto& token) { return MultiToken(token); });
  }

  ~MultiToken() = default;

  // Returns true iff `this` currently holds a token of type `T`.
  template <typename T>
    requires(internal::IsBaseToken<T> && internal::IsCompatible<T, Tokens...>)
  bool Is() const {
    return std::holds_alternative<T>(storage_);
  }

  // Returns `T` if `this` currently holds a token of type `T`; otherwise,
  // crashes.
  template <typename T>
    requires(internal::IsBaseToken<T> && internal::IsCompatible<T, Tokens...>)
  const T& GetAs() const {
    return std::get<T>(storage_);
  }

  // Wrapper around std::visit() which invokes the provided functor on this
  // MultiToken. The functor must return the same type when called with any of
  // the MultiToken's alternatives.
  template <typename Visitor>
  decltype(auto) Visit(Visitor&& visitor) const {
    return std::visit(std::forward<Visitor>(visitor), this->storage_);
  }

  // Comparison operators
  constexpr friend auto operator<=>(const MultiToken& lhs,
                                    const MultiToken& rhs) = default;
  constexpr friend bool operator==(const MultiToken& lhs,
                                   const MultiToken& rhs) = default;

  template <typename T>
    requires(internal::IsBaseToken<T> && internal::IsCompatible<T, Tokens...>)
  friend auto operator<=>(const MultiToken& lhs, const T& rhs) {
    return lhs <=> MultiToken(rhs);
  }

  template <typename T>
    requires(internal::IsBaseToken<T> && internal::IsCompatible<T, Tokens...>)
  friend bool operator==(const MultiToken& lhs, const T& rhs) {
    return lhs == MultiToken(rhs);
  }

  // Hash functor for use in unordered containers.
  struct Hasher {
    using argument_type = MultiToken;
    using result_type = size_t;
    result_type operator()(const MultiToken& token) const {
      return base::UnguessableTokenHash()(token.value());
    }
  };

  // Prefer the above helpers where possible. These methods are primarily useful
  // for serialization/deserialization.

  // Returns the underlying `base::UnguessableToken` of the currently held
  // token.
  const base::UnguessableToken& value() const {
    return Visit([](const auto& token) -> const base::UnguessableToken& {
      return token.value();
    });
  }

  // 0-based index of the currently held token's type, based on its position in
  // `Tokens...`.
  Tag variant_index() const { return static_cast<Tag>(storage_.index()); }

  // Returns the 0-based index that a token of type `T` would have if it were
  // currently held.
  template <typename T>
    requires(internal::IsBaseToken<T> && internal::IsCompatible<T, Tokens...>)
  static constexpr Tag IndexOf() {
    return static_cast<Tag>(base::VariantIndexOfType<Storage, T>());
  }

  // Equivalent to `value().ToString()`.
  std::string ToString() const {
    return Visit([](const auto& token) { return token.ToString(); });
  }

 private:
  Storage storage_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_TOKENS_MULTI_TOKEN_H_
