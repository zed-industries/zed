/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_MOCK_TRANSPORT_H_
#define TEST_MOCK_TRANSPORT_H_

#include "api/call/transport.h"
#include "test/gmock.h"

namespace webrtc {

class MockTransport : public Transport {
 public:
  MockTransport();
  ~MockTransport();

  MOCK_METHOD(bool,
              SendRtp,
              (webrtc::ArrayView<const uint8_t>, const PacketOptions&),
              (override));
  MOCK_METHOD(bool, SendRtcp, (webrtc::ArrayView<const uint8_t>), (override));
};

}  // namespace webrtc

#endif  // TEST_MOCK_TRANSPORT_H_
