/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_NETEQ_MOCK_MOCK_HISTOGRAM_H_
#define MODULES_AUDIO_CODING_NETEQ_MOCK_MOCK_HISTOGRAM_H_

#include "modules/audio_coding/neteq/histogram.h"
#include "test/gmock.h"

namespace webrtc {

class MockHistogram : public Histogram {
 public:
  MockHistogram(size_t num_buckets, int forget_factor)
      : Histogram(num_buckets, forget_factor) {}
  virtual ~MockHistogram() {}

  MOCK_METHOD(void, Add, (int), (override));
  MOCK_METHOD(int, Quantile, (int), (override));
};

}  // namespace webrtc
#endif  // MODULES_AUDIO_CODING_NETEQ_MOCK_MOCK_HISTOGRAM_H_
