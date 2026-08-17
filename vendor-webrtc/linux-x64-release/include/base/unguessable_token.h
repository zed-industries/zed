// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_UNGUESSABLE_TOKEN_H_
#define BASE_UNGUESSABLE_TOKEN_H_

#include <stdint.h>
#include <string.h>

#include <compare>
#include <iosfwd>
#include <string_view>
#include <utility>

#include "base/base_export.h"
#include "base/check.h"
#include "base/containers/span.h"
#include "base/token.h"

namespace base {

struct UnguessableTokenHash;

// UnguessableToken is, like Token, a randomly chosen 128-bit value. Unlike
// Token, a new UnguessableToken is always generated at runtime from a
// cryptographically strong random source (or copied or serialized and
// deserialized from another such UnguessableToken). Also unlike Token, the ==
// and != operators are constant time. It can be used as part of a larger
// aggregate type, or as an ID in and of itself.
//
// An UnguessableToken is a strong *bearer token*. Bearer tokens are like HTTP
// cookies: if a caller has the token, the callee thereby considers the caller
// authorized to request the operation the callee performs.
//
// UnguessableToken can be used when the resource associated with the ID needs
// to be protected against manipulation by other untrusted agents in the system,
// and there is no other convenient way to verify the authority of the agent to
// do so (because the resource is part of a table shared across processes, for
// instance). In such a scheme, knowledge of the token value in and of itself is
// sufficient proof of authority to carry out an operation on the associated
// resource.
//
// Use Create() for creating new UnguessableTokens.
//
// NOTE: It is illegal to send empty UnguessableTokens across processes, and
// sending/receiving empty tokens should be treated as a security issue. If
// there is a valid scenario for sending "no token" across processes, use
// std::optional instead of an empty token.

class BASE_EXPORT UnguessableToken {
 public:
  // Create a unique UnguessableToken. It's guaranteed to be nonempty.
  static UnguessableToken Create();

  // Returns a reference to a global null UnguessableToken. This should only be
  // used for functions that need to return a reference to an UnguessableToken,
  // and should not be used as a general-purpose substitute for invoking the
  // default constructor.
  static const UnguessableToken& Null();

  // Return an UnguessableToken built from the high/low bytes provided.
  // It should only be used in deserialization scenarios.
  //
  // NOTE: If the returned `std::optional` does not have a value, it means that
  // `high` and `low` correspond to an `UnguesssableToken` that was never
  // initialized via Create(). This is a security issue, and should be handled.
  static std::optional<UnguessableToken> Deserialize(uint64_t high,
                                                     uint64_t low);

  // Returns an `UnguessableToken` built from its string representation. It
  // should only be used in deserialization scenarios.
  //
  // NOTE: If the returned `std::optional` does not have a value, it means that
  // the given string does not represent a valid serialized `UnguessableToken`.
  // This should be handled as a security issue.
  static std::optional<UnguessableToken> DeserializeFromString(
      std::string_view string_representation);

  // Creates an empty UnguessableToken.
  // Assign to it with Create() before using it.
  constexpr UnguessableToken() = default;

  constexpr UnguessableToken(const UnguessableToken&) = default;
  constexpr UnguessableToken& operator=(const UnguessableToken&) = default;
  constexpr UnguessableToken(UnguessableToken&&) noexcept = default;
  constexpr UnguessableToken& operator=(UnguessableToken&&) = default;

  // NOTE: Serializing an empty UnguessableToken is an illegal operation.
  uint64_t GetHighForSerialization() const {
    DCHECK(!is_empty());
    return token_.high();
  }

  // NOTE: Serializing an empty UnguessableToken is an illegal operation.
  uint64_t GetLowForSerialization() const {
    DCHECK(!is_empty());
    return token_.low();
  }

  constexpr bool is_empty() const { return token_.is_zero(); }

  // Hex representation of the unguessable token.
  std::string ToString() const { return token_.ToString(); }

  explicit constexpr operator bool() const { return !is_empty(); }

  span<const uint8_t, 16> AsBytes() const { return token_.AsBytes(); }

  friend constexpr auto operator<=>(const UnguessableToken& lhs,
                                    const UnguessableToken& rhs) = default;

  // operator== uses constant-time comparison for security where available.
  friend BASE_EXPORT bool operator==(const UnguessableToken& lhs,
                                     const UnguessableToken& rhs);

  template <typename H>
  friend H AbslHashValue(H h, const UnguessableToken& token) {
    return H::combine(std::move(h), token.token_);
  }

#if defined(UNIT_TEST)
  static UnguessableToken CreateForTesting(uint64_t high, uint64_t low) {
    std::optional<UnguessableToken> token = Deserialize(high, low);
    DCHECK(token.has_value());
    return token.value();
  }
#endif

 private:
  friend struct UnguessableTokenHash;
  explicit UnguessableToken(const Token& token);

  base::Token token_;
};

BASE_EXPORT bool operator==(const UnguessableToken& lhs,
                            const UnguessableToken& rhs);

BASE_EXPORT std::ostream& operator<<(std::ostream& out,
                                     const UnguessableToken& token);

// For use in std::unordered_map.
struct UnguessableTokenHash {
  size_t operator()(const base::UnguessableToken& token) const {
    DCHECK(token);
    return TokenHash()(token.token_);
  }
};

}  // namespace base

#endif  // BASE_UNGUESSABLE_TOKEN_H_
