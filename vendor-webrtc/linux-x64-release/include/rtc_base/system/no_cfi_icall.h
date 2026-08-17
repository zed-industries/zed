/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_SYSTEM_NO_CFI_ICALL_H_
#define RTC_BASE_SYSTEM_NO_CFI_ICALL_H_

#include "rtc_base/sanitizer.h"

// DISABLE_CFI_ICALL -- Disable Control Flow Integrity indirect call checks.
// Note that the same macro is defined in "base/compiler_specific.h".
// Only use this when building standalone WebRTC.
#if !defined(WEBRTC_CHROMIUM_BUILD)
#if !defined(DISABLE_CFI_ICALL)
#if defined(WEBRTC_WIN)
// Windows also needs __declspec(guard(nocf)).
#define DISABLE_CFI_ICALL RTC_NO_SANITIZE("cfi-icall") __declspec(guard(nocf))
#else
#define DISABLE_CFI_ICALL RTC_NO_SANITIZE("cfi-icall")
#endif  // defined(WEBRTC_WIN)
#endif  // !defined(DISABLE_CFI_ICALL)
#if !defined(DISABLE_CFI_ICALL)
#define DISABLE_CFI_ICALL
#endif
#endif  // !defined(WEBRTC_CHROMIUM_BUILD)

#endif  // RTC_BASE_SYSTEM_NO_CFI_ICALL_H_
