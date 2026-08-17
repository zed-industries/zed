/*
 *  Copyright 2009 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_WIN32_SOCKET_INIT_H_
#define RTC_BASE_WIN32_SOCKET_INIT_H_

#ifndef WEBRTC_WIN
#error "Only #include this header in Windows builds"
#endif

#include "rtc_base/win32.h"

namespace webrtc {

class WinsockInitializer {
 public:
  WinsockInitializer() {
    WSADATA wsaData;
    WORD wVersionRequested = MAKEWORD(1, 0);
    err_ = WSAStartup(wVersionRequested, &wsaData);
  }
  ~WinsockInitializer() {
    if (!err_)
      WSACleanup();
  }
  int error() { return err_; }

 private:
  int err_;
};

}  // namespace webrtc

#endif  // RTC_BASE_WIN32_SOCKET_INIT_H_
