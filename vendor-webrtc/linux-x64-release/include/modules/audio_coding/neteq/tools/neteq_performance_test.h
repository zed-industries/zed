/*
 *  Copyright (c) 2014 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_NETEQ_TOOLS_NETEQ_PERFORMANCE_TEST_H_
#define MODULES_AUDIO_CODING_NETEQ_TOOLS_NETEQ_PERFORMANCE_TEST_H_

#include <stdint.h>

namespace webrtc {
namespace test {

class NetEqPerformanceTest {
 public:
  // Runs a performance test with parameters as follows:
  //   `runtime_ms`: the simulation time, i.e., the duration of the audio data.
  //   `lossrate`: drop one out of `lossrate` packets, e.g., one out of 10.
  //   `drift_factor`: clock drift in [0, 1].
  // Returns the runtime in ms.
  static int64_t Run(int runtime_ms, int lossrate, double drift_factor);
};

}  // namespace test
}  // namespace webrtc

#endif  // MODULES_AUDIO_CODING_NETEQ_TOOLS_NETEQ_PERFORMANCE_TEST_H_
