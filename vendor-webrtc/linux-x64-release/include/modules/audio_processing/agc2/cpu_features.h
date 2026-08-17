/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AGC2_CPU_FEATURES_H_
#define MODULES_AUDIO_PROCESSING_AGC2_CPU_FEATURES_H_

#include <string>

namespace webrtc {

// Collection of flags indicating which CPU features are available on the
// current platform. True means available.
struct AvailableCpuFeatures {
  AvailableCpuFeatures(bool sse2, bool avx2, bool neon)
      : sse2(sse2), avx2(avx2), neon(neon) {}
  // Intel.
  bool sse2;
  bool avx2;
  // ARM.
  bool neon;
  std::string ToString() const;
};

// Detects what CPU features are available.
AvailableCpuFeatures GetAvailableCpuFeatures();

// Returns the CPU feature flags all set to false.
AvailableCpuFeatures NoAvailableCpuFeatures();

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AGC2_CPU_FEATURES_H_
