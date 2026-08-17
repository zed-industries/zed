/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_RTP_RTCP_INCLUDE_RTP_RTCP_H_
#define MODULES_RTP_RTCP_INCLUDE_RTP_RTCP_H_

#include <memory>

#include "api/environment/environment.h"
#include "modules/rtp_rtcp/source/rtp_rtcp_interface.h"

namespace webrtc {

// A deprecated version of the RtpRtcp module.
class [[deprecated("bugs.webrtc.org/42224904")]] RtpRtcp
    : public RtpRtcpInterface {
 public:
  [[deprecated("bugs.webrtc.org/42224904")]]  //
  static std::unique_ptr<RtpRtcp>
  Create(const Environment& env, const Configuration& configuration);

  // Process any pending tasks such as timeouts.
  virtual void Process() = 0;
};

}  // namespace webrtc

#endif  // MODULES_RTP_RTCP_INCLUDE_RTP_RTCP_H_
