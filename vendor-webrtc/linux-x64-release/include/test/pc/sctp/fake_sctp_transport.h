/*
 *  Copyright 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_PC_SCTP_FAKE_SCTP_TRANSPORT_H_
#define TEST_PC_SCTP_FAKE_SCTP_TRANSPORT_H_

#include <cstddef>
#include <functional>
#include <memory>
#include <optional>

#include "api/environment/environment.h"
#include "api/priority.h"
#include "api/rtc_error.h"
#include "api/sctp_transport_interface.h"
#include "api/transport/data_channel_transport_interface.h"
#include "api/transport/sctp_transport_factory_interface.h"
#include "media/sctp/sctp_transport_internal.h"
#include "rtc_base/checks.h"
#include "rtc_base/copy_on_write_buffer.h"

// Used for tests in this file to verify that PeerConnection responds to signals
// from the SctpTransport correctly, and calls Start with the correct
// local/remote ports.
class FakeSctpTransport : public webrtc::SctpTransportInternal {
 public:
  void SetOnConnectedCallback(std::function<void()> callback) override {}
  void SetDataChannelSink(webrtc::DataChannelSink* sink) override {}
  void SetDtlsTransport(webrtc::DtlsTransportInternal* transport) override {}
  bool Start(const webrtc::SctpOptions& options) override {
    local_port_.emplace(options.local_port);
    remote_port_.emplace(options.remote_port);
    max_message_size_ = options.max_message_size;
    return true;
  }
  bool OpenStream(int sid, webrtc::PriorityValue priority) override {
    return true;
  }
  bool ResetStream(int sid) override { return true; }
  webrtc::RTCError SendData(int sid,
                            const webrtc::SendDataParams& params,
                            const webrtc::CopyOnWriteBuffer& payload) override {
    return webrtc::RTCError::OK();
  }
  bool ReadyToSendData() override { return true; }
  void set_debug_name_for_testing(const char* debug_name) override {}

  int max_message_size() const override { return max_message_size_; }
  std::optional<int> max_outbound_streams() const override {
    return std::nullopt;
  }
  std::optional<int> max_inbound_streams() const override {
    return std::nullopt;
  }
  size_t buffered_amount(int sid) const override { return 0; }
  size_t buffered_amount_low_threshold(int sid) const override { return 0; }
  void SetBufferedAmountLowThreshold(int sid, size_t bytes) override {}
  int local_port() const {
    RTC_DCHECK(local_port_);
    return *local_port_;
  }
  int remote_port() const {
    RTC_DCHECK(remote_port_);
    return *remote_port_;
  }

 private:
  std::optional<int> local_port_;
  std::optional<int> remote_port_;
  int max_message_size_;
};

class FakeSctpTransportFactory : public webrtc::SctpTransportFactoryInterface {
 public:
  std::unique_ptr<webrtc::SctpTransportInternal> CreateSctpTransport(
      const webrtc::Environment& env,
      webrtc::DtlsTransportInternal*) override {
    last_fake_sctp_transport_ = new FakeSctpTransport();
    return std::unique_ptr<webrtc::SctpTransportInternal>(
        last_fake_sctp_transport_);
  }

  FakeSctpTransport* last_fake_sctp_transport() {
    return last_fake_sctp_transport_;
  }

 private:
  FakeSctpTransport* last_fake_sctp_transport_ = nullptr;
};

#endif  // TEST_PC_SCTP_FAKE_SCTP_TRANSPORT_H_
