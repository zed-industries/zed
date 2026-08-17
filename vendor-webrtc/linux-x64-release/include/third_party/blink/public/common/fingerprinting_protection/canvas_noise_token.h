// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_FINGERPRINTING_PROTECTION_CANVAS_NOISE_TOKEN_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_FINGERPRINTING_PROTECTION_CANVAS_NOISE_TOKEN_H_

#include <cstdint>

#include "third_party/blink/public/common/common_export.h"

namespace blink {

// A static class that stores a per-renderer process noise token sent by the
// browser. Noise tokens are one of many attributes that handle canvas noising
// for fingerprinting protection. This class simply stores the 64 bit token and
// makes it available across Blink.
class BLINK_COMMON_EXPORT CanvasNoiseToken final {
 public:
  static void Set(uint64_t token);
  static uint64_t Get();
};
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_FINGERPRINTING_PROTECTION_CANVAS_NOISE_TOKEN_H_
