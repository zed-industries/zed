/*
 *  Copyright 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_RTP_PARAMETERS_CONVERSION_H_
#define PC_RTP_PARAMETERS_CONVERSION_H_

#include <optional>
#include <vector>

#include "api/rtc_error.h"
#include "api/rtp_parameters.h"
#include "media/base/codec.h"
#include "media/base/stream_params.h"
#include "pc/session_description.h"

namespace webrtc {

//*****************************************************************************
// Functions for converting from old webrtc:: structures to new webrtc::
// structures. These are permissive with regards to
// input validation; it's assumed that any necessary validation already
// occurred.
//
// These are expected to be used to convert from audio/video engine
// capabilities to RtpCapabilities.
//*****************************************************************************

// Returns empty value if `cricket_feedback` is a feedback type not
// supported/recognized.
std::optional<RtcpFeedback> ToRtcpFeedback(
    const FeedbackParam& cricket_feedback);

RtpCodecCapability ToRtpCodecCapability(const Codec& cricket_codec);

RtpCapabilities ToRtpCapabilities(
    const std::vector<Codec>& cricket_codecs,
    const RtpHeaderExtensions& cricket_extensions);

}  // namespace webrtc

#endif  // PC_RTP_PARAMETERS_CONVERSION_H_
