// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_PRIVACY_BUDGET_IDENTIFIABILITY_METRICS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_PRIVACY_BUDGET_IDENTIFIABILITY_METRICS_H_

#include <cstdint>
#include <cstring>
#include <type_traits>

#include "base/containers/span.h"
#include "third_party/blink/public/common/common_export.h"

namespace blink {

// IdentifiabilityDigestOfBytes, which is NOT a cryptographic hash function,
// takes a span of bytes as input and calculates a digest that can be used with
// identifiability metric reporting functions.
//
// The returned digest ...:
//
// * Is Stable: The returned digest will be consistent across different versions
//   of Chromium. Thus it can be persisted and meaningfully aggregated across
//   browser versions.
//
// * Is approximately uniformly distributed when the input is uniformly
//   distributed.
//
// * Is NOT optimized for any other distribution of input including narrow
//   integral ranges.
//
// * Is NOT collision resistant: Callers should assume that it is easy to come
//   up with collisions, and to come up with a pre-image given a digest.
//
// Note: This is NOT a cryptographic hash function.
BLINK_COMMON_EXPORT uint64_t
IdentifiabilityDigestOfBytes(base::span<const uint8_t> in);

// The zero-length digest, i.e. the digest computed for no bytes.
static constexpr uint64_t kIdentifiabilityDigestOfNoBytes =
    UINT64_C(0x9ae16a3b2f90404f);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_PRIVACY_BUDGET_IDENTIFIABILITY_METRICS_H_
