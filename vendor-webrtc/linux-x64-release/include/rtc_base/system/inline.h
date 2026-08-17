/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_SYSTEM_INLINE_H_
#define RTC_BASE_SYSTEM_INLINE_H_

#if defined(_MSC_VER)

#define RTC_FORCE_INLINE __forceinline
#define RTC_NO_INLINE __declspec(noinline)

#elif defined(__GNUC__)

#define RTC_FORCE_INLINE __attribute__((__always_inline__))
#define RTC_NO_INLINE __attribute__((__noinline__))

#else

#define RTC_FORCE_INLINE
#define RTC_NO_INLINE

#endif

#endif  // RTC_BASE_SYSTEM_INLINE_H_
