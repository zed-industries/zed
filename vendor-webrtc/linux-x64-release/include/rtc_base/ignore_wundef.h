/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_IGNORE_WUNDEF_H_
#define RTC_BASE_IGNORE_WUNDEF_H_

// If a header file uses #if on possibly undefined macros (and it's for some
// reason not possible to just fix the header file), include it like this:
//
//   RTC_PUSH_IGNORING_WUNDEF()
//   #include "misbehaving_header.h"
//   RTC_POP_IGNORING_WUNDEF()
//
// This will cause the compiler to not emit -Wundef warnings for that file.

#ifdef __clang__
#define RTC_PUSH_IGNORING_WUNDEF() \
  _Pragma("clang diagnostic push") \
      _Pragma("clang diagnostic ignored \"-Wundef\"")
#define RTC_POP_IGNORING_WUNDEF() _Pragma("clang diagnostic pop")
#else
#define RTC_PUSH_IGNORING_WUNDEF()
#define RTC_POP_IGNORING_WUNDEF()
#endif  // __clang__

#endif  // RTC_BASE_IGNORE_WUNDEF_H_
