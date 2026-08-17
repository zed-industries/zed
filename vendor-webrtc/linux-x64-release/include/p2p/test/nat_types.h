/*
 *  Copyright 2004 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef P2P_TEST_NAT_TYPES_H_
#define P2P_TEST_NAT_TYPES_H_

namespace webrtc {

/* Identifies each type of NAT that can be simulated. */
enum NATType {
  NAT_OPEN_CONE,
  NAT_ADDR_RESTRICTED,
  NAT_PORT_RESTRICTED,
  NAT_SYMMETRIC
};

// Implements the rules for each specific type of NAT.
class NAT {
 public:
  virtual ~NAT() {}

  // Determines whether this NAT uses both source and destination address when
  // checking whether a mapping already exists.
  virtual bool IsSymmetric() = 0;

  // Determines whether this NAT drops packets received from a different IP
  // the one last sent to.
  virtual bool FiltersIP() = 0;

  // Determines whether this NAT drops packets received from a different port
  // the one last sent to.
  virtual bool FiltersPort() = 0;

  // Returns an implementation of the given type of NAT.
  static NAT* Create(NATType type);
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::NAT;
using ::webrtc::NAT_ADDR_RESTRICTED;
using ::webrtc::NAT_OPEN_CONE;
using ::webrtc::NAT_PORT_RESTRICTED;
using ::webrtc::NAT_SYMMETRIC;
using ::webrtc::NATType;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // P2P_TEST_NAT_TYPES_H_
