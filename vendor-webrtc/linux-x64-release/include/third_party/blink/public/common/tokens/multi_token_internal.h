// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Internal implementation details for MultiToken. Only intended to be included
// from multi_token.h.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_TOKENS_MULTI_TOKEN_INTERNAL_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_TOKENS_MULTI_TOKEN_INTERNAL_H_

#include <type_traits>

#include "base/types/token_type.h"

namespace blink::internal {

template <typename T>
struct IsBaseTokenType : std::false_type {};

template <typename T>
struct IsBaseTokenType<base::TokenType<T>> : std::true_type {};

template <typename T>
concept IsBaseToken = IsBaseTokenType<T>::value;

template <typename... Types>
bool AreAllUnique;
template <>
inline constexpr bool AreAllUnique<> = true;
template <typename T, typename... Ts>
inline constexpr bool AreAllUnique<T, Ts...> =
    (!std::is_same_v<T, Ts> && ...) && AreAllUnique<Ts...>;

template <typename T, typename... Ts>
concept IsCompatible = (std::is_same_v<T, Ts> || ...);

}  // namespace blink::internal

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_TOKENS_MULTI_TOKEN_INTERNAL_H_
