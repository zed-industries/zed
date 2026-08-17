/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef COMMON_AUDIO_VAD_VAD_UNITTEST_H_
#define COMMON_AUDIO_VAD_VAD_UNITTEST_H_

#include <stddef.h>  // size_t

#include "test/gtest.h"

namespace webrtc {
namespace test {

// Modes we support
const int kModes[] = {0, 1, 2, 3};
const size_t kModesSize = sizeof(kModes) / sizeof(*kModes);

// Rates we support.
const int kRates[] = {8000, 12000, 16000, 24000, 32000, 48000};
const size_t kRatesSize = sizeof(kRates) / sizeof(*kRates);

// Frame lengths we support.
const size_t kMaxFrameLength = 1440;
const size_t kFrameLengths[] = {
    80, 120, 160, 240, 320, 480, 640, 960, kMaxFrameLength};
const size_t kFrameLengthsSize = sizeof(kFrameLengths) / sizeof(*kFrameLengths);

}  // namespace test
}  // namespace webrtc

class VadTest : public ::testing::Test {
 protected:
  VadTest();
  void SetUp() override;
  void TearDown() override;

  // Returns true if the rate and frame length combination is valid.
  bool ValidRatesAndFrameLengths(int rate, size_t frame_length);
};

#endif  // COMMON_AUDIO_VAD_VAD_UNITTEST_H_
