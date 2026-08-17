/*
 *  Copyright (c) 2014 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_NETEQ_MOCK_MOCK_EXPAND_H_
#define MODULES_AUDIO_CODING_NETEQ_MOCK_MOCK_EXPAND_H_

#include "modules/audio_coding/neteq/expand.h"
#include "test/gmock.h"

namespace webrtc {

class MockExpand : public Expand {
 public:
  MockExpand(BackgroundNoise* background_noise,
             SyncBuffer* sync_buffer,
             RandomVector* random_vector,
             StatisticsCalculator* statistics,
             int fs,
             size_t num_channels)
      : Expand(background_noise,
               sync_buffer,
               random_vector,
               statistics,
               fs,
               num_channels) {}
  ~MockExpand() override { Die(); }
  MOCK_METHOD(void, Die, ());
  MOCK_METHOD(void, Reset, (), (override));
  MOCK_METHOD(int, Process, (AudioMultiVector * output), (override));
  MOCK_METHOD(void, SetParametersForNormalAfterExpand, (), (override));
  MOCK_METHOD(void, SetParametersForMergeAfterExpand, (), (override));
  MOCK_METHOD(size_t, overlap_length, (), (const, override));
};

}  // namespace webrtc

namespace webrtc {

class MockExpandFactory : public ExpandFactory {
 public:
  MOCK_METHOD(Expand*,
              Create,
              (BackgroundNoise * background_noise,
               SyncBuffer* sync_buffer,
               RandomVector* random_vector,
               StatisticsCalculator* statistics,
               int fs,
               size_t num_channels),
              (const, override));
};

}  // namespace webrtc
#endif  // MODULES_AUDIO_CODING_NETEQ_MOCK_MOCK_EXPAND_H_
