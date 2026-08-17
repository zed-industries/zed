/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_NETEQ_TOOLS_NETEQ_EVENT_LOG_INPUT_H_
#define MODULES_AUDIO_CODING_NETEQ_TOOLS_NETEQ_EVENT_LOG_INPUT_H_

#include <optional>

#include "absl/strings/string_view.h"
#include "logging/rtc_event_log/rtc_event_log_parser.h"
#include "modules/audio_coding/neteq/tools/neteq_input.h"

namespace webrtc {
namespace test {

std::unique_ptr<NetEqInput> CreateNetEqEventLogInput(
    const ParsedRtcEventLog& parsed_log,
    std::optional<uint32_t> ssrc);

}  // namespace test
}  // namespace webrtc
#endif  // MODULES_AUDIO_CODING_NETEQ_TOOLS_NETEQ_EVENT_LOG_INPUT_H_
