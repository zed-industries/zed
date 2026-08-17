/*
 *  Copyright 2013 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_IFADDRS_ANDROID_H_
#define RTC_BASE_IFADDRS_ANDROID_H_

#include <stdio.h>
#include <sys/socket.h>  // no-presubmit-check

// Implementation of getifaddrs for Android.
// Fills out a list of ifaddr structs (see below) which contain information
// about every network interface available on the host.
// See 'man getifaddrs' on Linux or OS X (nb: it is not a POSIX function).
struct ifaddrs {
  struct ifaddrs* ifa_next;
  char* ifa_name;
  unsigned int ifa_flags;
  struct sockaddr* ifa_addr;
  struct sockaddr* ifa_netmask;
  // Real ifaddrs has broadcast, point to point and data members.
  // We don't need them (yet?).
};

namespace webrtc {

int getifaddrs(struct ifaddrs** result);
void freeifaddrs(struct ifaddrs* addrs);

}  // namespace webrtc

#endif  // RTC_BASE_IFADDRS_ANDROID_H_
