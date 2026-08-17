/*
 *  Copyright (c) 2014 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef COMMON_AUDIO_WINDOW_GENERATOR_H_
#define COMMON_AUDIO_WINDOW_GENERATOR_H_

#include <stddef.h>

namespace webrtc {

// Helper class with generators for various signal transform windows.
class WindowGenerator {
 public:
  WindowGenerator() = delete;
  WindowGenerator(const WindowGenerator&) = delete;
  WindowGenerator& operator=(const WindowGenerator&) = delete;

  static void Hanning(int length, float* window);
  static void KaiserBesselDerived(float alpha, size_t length, float* window);
};

}  // namespace webrtc

#endif  // COMMON_AUDIO_WINDOW_GENERATOR_H_
