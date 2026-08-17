/*
 *  Copyright 2020 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_SYSTEM_GCD_HELPERS_H_
#define RTC_BASE_SYSTEM_GCD_HELPERS_H_

#include <dispatch/dispatch.h>

#ifdef __cplusplus
extern "C" {
#endif

DISPATCH_RETURNS_RETAINED DISPATCH_WARN_RESULT DISPATCH_NOTHROW dispatch_queue_t
RTCDispatchQueueCreateWithTarget(const char* label,
                                 dispatch_queue_attr_t attr,
                                 dispatch_queue_t target);

#ifdef __cplusplus
}
#endif

#endif  // RTC_BASE_SYSTEM_GCD_HELPERS_H_
