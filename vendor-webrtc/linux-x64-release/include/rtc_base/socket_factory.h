/*
 *  Copyright 2004 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_SOCKET_FACTORY_H_
#define RTC_BASE_SOCKET_FACTORY_H_

#include "rtc_base/socket.h"

namespace webrtc {

class SocketFactory {
 public:
  virtual ~SocketFactory() {}

  // Returns a new socket.  The type can be SOCK_DGRAM and SOCK_STREAM.
  virtual Socket* CreateSocket(int family, int type) = 0;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::SocketFactory;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // RTC_BASE_SOCKET_FACTORY_H_
