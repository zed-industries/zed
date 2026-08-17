/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef TEST_NULL_TRANSPORT_H_
#define TEST_NULL_TRANSPORT_H_

#include "api/call/transport.h"

namespace webrtc {

class PacketReceiver;

namespace test {
class NullTransport : public Transport {
 public:
  bool SendRtp(ArrayView<const uint8_t> packet,
               const PacketOptions& options) override;
  bool SendRtcp(ArrayView<const uint8_t> packet) override;
};
}  // namespace test
}  // namespace webrtc

#endif  // TEST_NULL_TRANSPORT_H_
