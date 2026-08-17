/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_NETEQ_TOOLS_NETEQ_RTP_DUMP_INPUT_H_
#define MODULES_AUDIO_CODING_NETEQ_TOOLS_NETEQ_RTP_DUMP_INPUT_H_

#include <map>
#include <memory>
#include <optional>

#include "absl/strings/string_view.h"
#include "modules/audio_coding/neteq/tools/neteq_input.h"
#include "modules/rtp_rtcp/include/rtp_rtcp_defines.h"

namespace webrtc {
namespace test {

std::unique_ptr<NetEqInput> CreateNetEqRtpDumpInput(
    absl::string_view file_name,
    const std::map<int, RTPExtensionType>& hdr_ext_map,
    std::optional<uint32_t> ssrc_filter);

}  // namespace test
}  // namespace webrtc
#endif  // MODULES_AUDIO_CODING_NETEQ_TOOLS_NETEQ_RTP_DUMP_INPUT_H_
