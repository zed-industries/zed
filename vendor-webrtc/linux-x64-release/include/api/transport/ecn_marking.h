/*
 *  Copyright 2024 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_TRANSPORT_ECN_MARKING_H_
#define API_TRANSPORT_ECN_MARKING_H_

namespace webrtc {

// TODO: bugs.webrtc.org/42225697 - L4S support is slowly being developed.
// Help is appreciated.

// L4S Explicit Congestion Notification (ECN) .
// https://www.rfc-editor.org/rfc/rfc9331.html ECT stands for ECN-Capable
// Transport and CE stands for Congestion Experienced.

// RFC-3168, Section 5
// +-----+-----+
// | ECN FIELD |
// +-----+-----+
//   ECT   CE         [Obsolete] RFC 2481 names for the ECN bits.
//    0     0         Not-ECT
//    0     1         ECT(1)
//    1     0         ECT(0)
//    1     1         CE

enum class EcnMarking {
  kNotEct = 0,  // Not ECN-Capable Transport
  kEct1 = 1,    // ECN-Capable Transport
  kEct0 = 2,    // Not used by L4s (or webrtc.)
  kCe = 3,      // Congestion experienced
};

}  // namespace webrtc

#endif  // API_TRANSPORT_ECN_MARKING_H_
