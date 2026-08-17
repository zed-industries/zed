/*
 *  Copyright 2013 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_DSCP_H_
#define RTC_BASE_DSCP_H_

namespace webrtc {
// Differentiated Services Code Point.
// See http://tools.ietf.org/html/rfc2474 for details.
enum DiffServCodePoint {
  DSCP_NO_CHANGE = -1,
  DSCP_DEFAULT = 0,  // Same as DSCP_CS0
  DSCP_CS0 = 0,      // The default
  DSCP_CS1 = 8,      // Bulk/background traffic
  DSCP_AF11 = 10,
  DSCP_AF12 = 12,
  DSCP_AF13 = 14,
  DSCP_CS2 = 16,
  DSCP_AF21 = 18,
  DSCP_AF22 = 20,
  DSCP_AF23 = 22,
  DSCP_CS3 = 24,
  DSCP_AF31 = 26,
  DSCP_AF32 = 28,
  DSCP_AF33 = 30,
  DSCP_CS4 = 32,
  DSCP_AF41 = 34,  // Video
  DSCP_AF42 = 36,  // Video
  DSCP_AF43 = 38,  // Video
  DSCP_CS5 = 40,   // Video
  DSCP_EF = 46,    // Voice
  DSCP_CS6 = 48,   // Voice
  DSCP_CS7 = 56,   // Control messages
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::DiffServCodePoint;
using ::webrtc::DSCP_AF11;
using ::webrtc::DSCP_AF12;
using ::webrtc::DSCP_AF13;
using ::webrtc::DSCP_AF21;
using ::webrtc::DSCP_AF22;
using ::webrtc::DSCP_AF23;
using ::webrtc::DSCP_AF31;
using ::webrtc::DSCP_AF32;
using ::webrtc::DSCP_AF33;
using ::webrtc::DSCP_AF41;
using ::webrtc::DSCP_AF42;
using ::webrtc::DSCP_AF43;
using ::webrtc::DSCP_CS0;
using ::webrtc::DSCP_CS1;
using ::webrtc::DSCP_CS2;
using ::webrtc::DSCP_CS3;
using ::webrtc::DSCP_CS4;
using ::webrtc::DSCP_CS5;
using ::webrtc::DSCP_CS6;
using ::webrtc::DSCP_CS7;
using ::webrtc::DSCP_DEFAULT;
using ::webrtc::DSCP_EF;
using ::webrtc::DSCP_NO_CHANGE;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // RTC_BASE_DSCP_H_
