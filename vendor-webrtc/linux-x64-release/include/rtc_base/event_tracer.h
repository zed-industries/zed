/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_EVENT_TRACER_H_
#define RTC_BASE_EVENT_TRACER_H_

// This file defines the interface for event tracing in WebRTC.
//
// Event log handlers are set through SetupEventTracer(). User of this API will
// provide two function pointers to handle event tracing calls.
//
// * GetCategoryEnabledPtr
//   Event tracing system calls this function to determine if a particular
//   event category is enabled.
//
// * AddTraceEventPtr
//   Adds a tracing event. It is the user's responsibility to log the data
//   provided.
//
// Parameters for the above two functions are described in trace_event.h.

#include <stdio.h>

#include "absl/strings/string_view.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

#if defined(RTC_USE_PERFETTO)
void RegisterPerfettoTrackEvents();
#else
typedef const unsigned char* (*GetCategoryEnabledPtr)(const char* name);
typedef void (*AddTraceEventPtr)(char phase,
                                 const unsigned char* category_enabled,
                                 const char* name,
                                 unsigned long long id,
                                 int num_args,
                                 const char** arg_names,
                                 const unsigned char* arg_types,
                                 const unsigned long long* arg_values,
                                 unsigned char flags);

// User of WebRTC can call this method to setup event tracing.
//
// This method must be called before any WebRTC methods. Functions
// provided should be thread-safe.
void SetupEventTracer(GetCategoryEnabledPtr get_category_enabled_ptr,
                      AddTraceEventPtr add_trace_event_ptr);

// This class defines interface for the event tracing system to call
// internally. Do not call these methods directly.
class EventTracer {
 public:
  static const unsigned char* GetCategoryEnabled(const char* name);

  static void AddTraceEvent(char phase,
                            const unsigned char* category_enabled,
                            const char* name,
                            unsigned long long id,
                            int num_args,
                            const char** arg_names,
                            const unsigned char* arg_types,
                            const unsigned long long* arg_values,
                            unsigned char flags);
};
#endif

namespace tracing {
// Set up internal event tracer.
// TODO(webrtc:15917): Implement for perfetto.
RTC_EXPORT void SetupInternalTracer(bool enable_all_categories = true);
RTC_EXPORT bool StartInternalCapture(absl::string_view filename);
RTC_EXPORT void StartInternalCaptureToFile(FILE* file);
RTC_EXPORT void StopInternalCapture();
// Make sure we run this, this will tear down the internal tracing.
RTC_EXPORT void ShutdownInternalTracer();
}  // namespace tracing

}  // namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
namespace tracing {
using ::webrtc::tracing::SetupInternalTracer;
using ::webrtc::tracing::ShutdownInternalTracer;
using ::webrtc::tracing::StartInternalCapture;
using ::webrtc::tracing::StartInternalCaptureToFile;
using ::webrtc::tracing::StopInternalCapture;
}  // namespace tracing
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // RTC_BASE_EVENT_TRACER_H_
