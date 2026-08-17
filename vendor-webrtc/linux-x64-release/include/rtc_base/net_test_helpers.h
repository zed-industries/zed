/*
 *  Copyright 2023 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_NET_TEST_HELPERS_H_
#define RTC_BASE_NET_TEST_HELPERS_H_

#include "rtc_base/system/rtc_export.h"

namespace webrtc {

RTC_EXPORT bool HasIPv4Enabled();
RTC_EXPORT bool HasIPv6Enabled();

}  // namespace webrtc

#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using webrtc::HasIPv4Enabled;
using webrtc::HasIPv6Enabled;
}
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // RTC_BASE_NET_TEST_HELPERS_H_
