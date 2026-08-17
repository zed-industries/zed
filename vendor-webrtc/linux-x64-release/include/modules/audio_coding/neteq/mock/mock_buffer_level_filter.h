/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_NETEQ_MOCK_MOCK_BUFFER_LEVEL_FILTER_H_
#define MODULES_AUDIO_CODING_NETEQ_MOCK_MOCK_BUFFER_LEVEL_FILTER_H_

#include "modules/audio_coding/neteq/buffer_level_filter.h"
#include "test/gmock.h"

namespace webrtc {

class MockBufferLevelFilter : public BufferLevelFilter {
 public:
  MOCK_METHOD(void,
              Update,
              (size_t buffer_size_samples, int time_stretched_samples));
  MOCK_METHOD(int, filtered_current_level, (), (const));
};

}  // namespace webrtc
#endif  // MODULES_AUDIO_CODING_NETEQ_MOCK_MOCK_BUFFER_LEVEL_FILTER_H_
