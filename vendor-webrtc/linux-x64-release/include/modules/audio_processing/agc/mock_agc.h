/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AGC_MOCK_AGC_H_
#define MODULES_AUDIO_PROCESSING_AGC_MOCK_AGC_H_

#include "api/array_view.h"
#include "modules/audio_processing/agc/agc.h"
#include "test/gmock.h"

namespace webrtc {

class MockAgc : public Agc {
 public:
  virtual ~MockAgc() {}
  MOCK_METHOD(void,
              Process,
              (webrtc::ArrayView<const int16_t> audio),
              (override));
  MOCK_METHOD(bool, GetRmsErrorDb, (int* error), (override));
  MOCK_METHOD(void, Reset, (), (override));
  MOCK_METHOD(int, set_target_level_dbfs, (int level), (override));
  MOCK_METHOD(int, target_level_dbfs, (), (const, override));
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AGC_MOCK_AGC_H_
