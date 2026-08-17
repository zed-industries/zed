/*
 *  Copyright (c) 2023 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_TOOLS_RTC_EVENT_LOG_VISUALIZER_ANALYZER_BINDINGS_H_
#define RTC_TOOLS_RTC_EVENT_LOG_VISUALIZER_ANALYZER_BINDINGS_H_

#include <cstddef>
#include <cstdint>

void analyze_rtc_event_log(const char* log_contents,
                           size_t log_size,
                           const char* selection,
                           size_t selection_size,
                           char* output,
                           uint32_t* output_size);

#endif  // RTC_TOOLS_RTC_EVENT_LOG_VISUALIZER_ANALYZER_BINDINGS_H_
