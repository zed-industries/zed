/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_TOOLS_RTC_EVENT_LOG_TO_TEXT_CONVERTER_H_
#define RTC_TOOLS_RTC_EVENT_LOG_TO_TEXT_CONVERTER_H_

#include <stdio.h>

#include <string>

#include "absl/base/attributes.h"
#include "logging/rtc_event_log/rtc_event_log_parser.h"

namespace webrtc {

// Parses events from file `inputfile` and prints human readable text
// representations to `output`. The output is sorted by log time.
// `handle_unconfigured_extensions` controls the policy for parsing RTP
// header extensions if the log doesn't contain a mapping between the
// header extensions and numerical IDs.
ABSL_MUST_USE_RESULT bool Convert(
    std::string inputfile,
    FILE* output,
    ParsedRtcEventLog::UnconfiguredHeaderExtensions
        handle_unconfigured_extensions);

}  // namespace webrtc

#endif  // RTC_TOOLS_RTC_EVENT_LOG_TO_TEXT_CONVERTER_H_
