/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_SYSTEM_UNUSED_H_
#define RTC_BASE_SYSTEM_UNUSED_H_

// Prevent the compiler from warning about an unused variable. For example:
//   int result = DoSomething();
//   RTC_DCHECK(result == 17);
//   RTC_UNUSED(result);
// Note: In most cases it is better to remove the unused variable rather than
// suppressing the compiler warning.
#ifndef RTC_UNUSED
#ifdef __cplusplus
#define RTC_UNUSED(x) static_cast<void>(x)
#else
#define RTC_UNUSED(x) (void)(x)
#endif
#endif  // RTC_UNUSED

#endif  // RTC_BASE_SYSTEM_UNUSED_H_
