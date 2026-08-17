/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_NETEQ_MOCK_MOCK_DTMF_BUFFER_H_
#define MODULES_AUDIO_CODING_NETEQ_MOCK_MOCK_DTMF_BUFFER_H_

#include "modules/audio_coding/neteq/dtmf_buffer.h"
#include "test/gmock.h"

namespace webrtc {

class MockDtmfBuffer : public DtmfBuffer {
 public:
  MockDtmfBuffer(int fs) : DtmfBuffer(fs) {}
  ~MockDtmfBuffer() override { Die(); }
  MOCK_METHOD(void, Die, ());
  MOCK_METHOD(void, Flush, (), (override));
  MOCK_METHOD(int, InsertEvent, (const DtmfEvent& event), (override));
  MOCK_METHOD(bool,
              GetEvent,
              (uint32_t current_timestamp, DtmfEvent* event),
              (override));
  MOCK_METHOD(size_t, Length, (), (const, override));
  MOCK_METHOD(bool, Empty, (), (const, override));
};

}  // namespace webrtc
#endif  // MODULES_AUDIO_CODING_NETEQ_MOCK_MOCK_DTMF_BUFFER_H_
