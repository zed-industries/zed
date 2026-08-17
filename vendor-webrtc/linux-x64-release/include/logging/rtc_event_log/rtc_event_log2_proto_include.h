/*
 *  Copyright (c) 2023 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef LOGGING_RTC_EVENT_LOG_RTC_EVENT_LOG2_PROTO_INCLUDE_H_
#define LOGGING_RTC_EVENT_LOG_RTC_EVENT_LOG2_PROTO_INCLUDE_H_

// *.pb.h files are generated at build-time by the protobuf compiler.
#ifdef WEBRTC_ANDROID_PLATFORM_BUILD
#include "external/webrtc/webrtc/logging/rtc_event_log/rtc_event_log2.pb.h"
#else
#include "logging/rtc_event_log/rtc_event_log2.pb.h"
#endif

#endif  //  LOGGING_RTC_EVENT_LOG_RTC_EVENT_LOG2_PROTO_INCLUDE_H_
