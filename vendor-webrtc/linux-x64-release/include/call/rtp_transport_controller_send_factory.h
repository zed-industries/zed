/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef CALL_RTP_TRANSPORT_CONTROLLER_SEND_FACTORY_H_
#define CALL_RTP_TRANSPORT_CONTROLLER_SEND_FACTORY_H_

#include <memory>

#include "call/rtp_transport_config.h"
#include "call/rtp_transport_controller_send.h"
#include "call/rtp_transport_controller_send_factory_interface.h"
#include "call/rtp_transport_controller_send_interface.h"

namespace webrtc {
class RtpTransportControllerSendFactory
    : public RtpTransportControllerSendFactoryInterface {
 public:
  std::unique_ptr<RtpTransportControllerSendInterface> Create(
      const RtpTransportConfig& config) override {
    return std::make_unique<RtpTransportControllerSend>(config);
  }

  virtual ~RtpTransportControllerSendFactory() {}
};
}  // namespace webrtc
#endif  // CALL_RTP_TRANSPORT_CONTROLLER_SEND_FACTORY_H_
