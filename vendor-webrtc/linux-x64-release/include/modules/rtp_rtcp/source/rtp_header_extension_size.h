/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef MODULES_RTP_RTCP_SOURCE_RTP_HEADER_EXTENSION_SIZE_H_
#define MODULES_RTP_RTCP_SOURCE_RTP_HEADER_EXTENSION_SIZE_H_

#include "api/array_view.h"
#include "modules/rtp_rtcp/include/rtp_header_extension_map.h"
#include "modules/rtp_rtcp/include/rtp_rtcp_defines.h"

namespace webrtc {

struct RtpExtensionSize {
  RTPExtensionType type;
  int value_size;
};

// Calculates rtp header extension size in bytes assuming packet contain
// all `extensions` with provided `value_size`.
// Counts only extensions present among `registered_extensions`.
int RtpHeaderExtensionSize(ArrayView<const RtpExtensionSize> extensions,
                           const RtpHeaderExtensionMap& registered_extensions);

}  // namespace webrtc

#endif  // MODULES_RTP_RTCP_SOURCE_RTP_HEADER_EXTENSION_SIZE_H_
