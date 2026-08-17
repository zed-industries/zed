/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_SYSTEM_IGNORE_WARNINGS_H_
#define RTC_BASE_SYSTEM_IGNORE_WARNINGS_H_

#ifdef __clang__
#define RTC_PUSH_IGNORING_WFRAME_LARGER_THAN() \
  _Pragma("clang diagnostic push")             \
      _Pragma("clang diagnostic ignored \"-Wframe-larger-than=\"")
#define RTC_POP_IGNORING_WFRAME_LARGER_THAN() _Pragma("clang diagnostic pop")
#elif __GNUC__
#define RTC_PUSH_IGNORING_WFRAME_LARGER_THAN() \
  _Pragma("GCC diagnostic push")               \
      _Pragma("GCC diagnostic ignored \"-Wframe-larger-than=\"")
#define RTC_POP_IGNORING_WFRAME_LARGER_THAN() _Pragma("GCC diagnostic pop")
#else
#define RTC_PUSH_IGNORING_WFRAME_LARGER_THAN()
#define RTC_POP_IGNORING_WFRAME_LARGER_THAN()
#endif

#endif  // RTC_BASE_SYSTEM_IGNORE_WARNINGS_H_
