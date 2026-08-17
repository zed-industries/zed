// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_TOKENS_TOKEN_MOJOM_TRAITS_HELPER_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_TOKENS_TOKEN_MOJOM_TRAITS_HELPER_H_

#include "base/unguessable_token.h"
#include "mojo/public/cpp/base/unguessable_token_mojom_traits.h"

namespace blink {

// Defines Mojo StructTraits that convert between the given |MojomDataViewType|
// and the given |TokenType|. It is assumed that TokenType is an instance of
// base::TokenType<...> and that MojomDataViewType is a simple mojom struct
// containing only a "base.mojom.UnguessableToken value" field.
template <typename MojomDataViewType, typename TokenType>
struct TokenMojomTraitsHelper {
  // For converting from MojomDataViewType to TokenType.
  static bool Read(MojomDataViewType& input, TokenType* output) {
    base::UnguessableToken token;
    if (!input.ReadValue(&token))
      return false;
    // UnguessableToken's StructTraits ensures that `token` will never be
    // empty.
    CHECK(!token.is_empty());
    *output = TokenType(token);
    return true;
  }

  // For converting from TokenType to MojomDataViewType.
  static const base::UnguessableToken& value(const TokenType& input) {
    return input.value();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_TOKENS_TOKEN_MOJOM_TRAITS_HELPER_H_
