/*
 *  Copyright 2019 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_SYSTEM_WARN_CURRENT_THREAD_IS_DEADLOCKED_H_
#define RTC_BASE_SYSTEM_WARN_CURRENT_THREAD_IS_DEADLOCKED_H_

namespace webrtc {

#if defined(WEBRTC_ANDROID) && !defined(WEBRTC_CHROMIUM_BUILD)
void WarnThatTheCurrentThreadIsProbablyDeadlocked();
#else
inline void WarnThatTheCurrentThreadIsProbablyDeadlocked() {}
#endif

}  // namespace webrtc

#endif  // RTC_BASE_SYSTEM_WARN_CURRENT_THREAD_IS_DEADLOCKED_H_
