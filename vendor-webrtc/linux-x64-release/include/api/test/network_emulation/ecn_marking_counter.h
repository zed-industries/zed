/*
 *  Copyright 2024 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef API_TEST_NETWORK_EMULATION_ECN_MARKING_COUNTER_H_
#define API_TEST_NETWORK_EMULATION_ECN_MARKING_COUNTER_H_

#include "api/transport/ecn_marking.h"

namespace webrtc {

// Counts Explicit Congestion Notifaction marks in IP packets.
// https://www.rfc-editor.org/rfc/rfc9331.html
class EcnMarkingCounter {
 public:
  // Number of packets without ECT explicitly set sent through the network.
  int not_ect() const { return not_ect_; }
  // Number of packets with ECT(1) sent through the network.
  int ect_0() const { return ect_0_; }
  // Number of packets with ECT(1) sent through the network.
  int ect_1() const { return ect_1_; }
  // Number of packets the network has marked as CE (congestion experienced).
  int ce() const { return ce_; }

  void Add(EcnMarking ecn);
  EcnMarkingCounter& operator+=(const EcnMarkingCounter& counter);

 private:
  int not_ect_ = 0;
  int ect_0_ = 0;  // Not used by WebRTC or L4S.
  int ect_1_ = 0;
  int ce_ = 0;
};

}  // namespace webrtc
#endif  // API_TEST_NETWORK_EMULATION_ECN_MARKING_COUNTER_H_
