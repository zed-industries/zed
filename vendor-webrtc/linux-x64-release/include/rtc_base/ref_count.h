/*
 *  Copyright 2011 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef RTC_BASE_REF_COUNT_H_
#define RTC_BASE_REF_COUNT_H_

// Transition file for backwards compatibility with source code
// that includes the non-API file.

#include "api/ref_count.h"

#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {

// TODO(bugs.webrtc.org/15622): Deprecate and remove these aliases.
using RefCountInterface [[deprecated("Use webrtc::RefCountInterface")]] =
    webrtc::RefCountInterface;
using RefCountReleaseStatus
    [[deprecated("Use webrtc::RefCountReleaseStatus")]] =
        webrtc::RefCountReleaseStatus;

}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // RTC_BASE_REF_COUNT_H_
