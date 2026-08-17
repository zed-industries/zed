/*
 *  Copyright 2023 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_LOCATION_H_
#define API_LOCATION_H_

#include "rtc_base/system/rtc_export.h"

namespace webrtc {

// Location provides basic info where of an object was constructed, or was
// significantly brought to life. This is a stripped down version of
// https://source.chromium.org/chromium/chromium/src/+/main:base/location.h
// that only specifies an interface compatible to how base::Location is
// supposed to be used.
// The declaration is overriden inside the Chromium build.
class RTC_EXPORT Location {
 public:
  static Location Current() { return Location(); }
};

}  // namespace webrtc

#endif  // API_LOCATION_H_
