// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_ORIGIN_TRIALS_ORIGIN_TRIAL_PUBLIC_KEY_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_ORIGIN_TRIALS_ORIGIN_TRIAL_PUBLIC_KEY_H_

#include <array>
#include <cstdint>

namespace blink {

// Public key must be 32 bytes long for Ed25519.
using OriginTrialPublicKey = std::array<uint8_t, 32>;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_ORIGIN_TRIALS_ORIGIN_TRIAL_PUBLIC_KEY_H_
