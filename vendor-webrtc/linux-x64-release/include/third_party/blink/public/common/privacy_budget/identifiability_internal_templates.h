// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_PRIVACY_BUDGET_IDENTIFIABILITY_INTERNAL_TEMPLATES_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_PRIVACY_BUDGET_IDENTIFIABILITY_INTERNAL_TEMPLATES_H_

#include <concepts>
#include <cstdint>
#include <cstring>
#include <type_traits>

#include "base/containers/span.h"
#include "base/numerics/byte_conversions.h"

namespace blink {

namespace internal {

// Calculate a digest of an object with a unique representation.
//
// In a perfect world, we should also require that this representation be
// portable or made to be portable. Such a transformation could be done, for
// example, by adopting a consistent byte ordering on all platforms.
//
// This function should only be invoked on a bare (sans qualifiers and
// references) type for the sake of simplicity.
//
// Should not be used as a primitive for manually constructing a unique
// representation. For such cases, use the byte-wise digest functions instead.
//
// Should not be used outside of the narrow use cases in this file.
//
// This implementation does not work for x86 extended precision floating point
// numbers. These are 80-bits wide, but in practice includes 6 bytes of padding
// in order to extend the size to 16 bytes. The extra bytes are uninitialized
// and will not contribute a stable digest.
template <typename T>
constexpr int64_t DigestOfObjectRepresentation(T in)
  requires(std::same_as<T, std::remove_cvref_t<T>> &&
           std::is_trivially_copyable_v<T> &&
           std::has_unique_object_representations_v<T> &&
           sizeof(T) <= sizeof(int64_t))
{
  if constexpr (std::is_integral<T>::value &&
                (std::is_signed<T>::value || sizeof(T) < sizeof(int64_t))) {
    // If |in| is small enough, the digest is itself. There's no point hashing
    // this value since the identity has all the properties we are looking for
    // in a digest.
    return in;
  } else {
    // Otherwise we treat the native byte representation as the digest. If the
    // type is a structure containing non-byte-sized integers, this would
    // produce a different absolute output on BE and LE machines (though BE
    // machines are not supported by Chromium).
    std::array<uint8_t, 8u> bytes = {};
    base::span(bytes).template first<sizeof(T)>().copy_from(
        base::byte_span_from_ref(in));
    return static_cast<int64_t>(base::U64FromNativeEndian(bytes));
  }
}

}  // namespace internal

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_PRIVACY_BUDGET_IDENTIFIABILITY_INTERNAL_TEMPLATES_H_
