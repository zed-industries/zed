/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef TEST_RTP_RTCP_OBSERVER_H_
#define TEST_RTP_RTCP_OBSERVER_H_

#include <map>
#include <memory>
#include <utility>
#include <vector>

#include "absl/flags/flag.h"
#include "api/array_view.h"
#include "api/test/simulated_network.h"
#include "api/units/time_delta.h"
#include "call/simulated_packet_receiver.h"
#include "call/video_send_stream.h"
#include "modules/rtp_rtcp/source/rtp_util.h"
#include "rtc_base/event.h"
#include "system_wrappers/include/field_trial.h"
#include "test/direct_transport.h"
#include "test/gtest.h"
#include "test/test_flags.h"

namespace webrtc {
namespace test {

class PacketTransport;

class RtpRtcpObserver {
 public:
  enum Action {
    SEND_PACKET,
    DROP_PACKET,
  };

  virtual ~RtpRtcpObserver() {}

  virtual bool Wait() {
    if (absl::GetFlag(FLAGS_webrtc_quick_perf_test)) {
      observation_complete_.Wait(TimeDelta::Millis(500));
      return true;
    }
    return observation_complete_.Wait(timeout_);
  }

  virtual Action OnSendRtp(ArrayView<const uint8_t> packet) {
    return SEND_PACKET;
  }

  virtual Action OnSendRtcp(ArrayView<const uint8_t> packet) {
    return SEND_PACKET;
  }

  virtual Action OnReceiveRtp(ArrayView<const uint8_t> packet) {
    return SEND_PACKET;
  }

  virtual Action OnReceiveRtcp(ArrayView<const uint8_t> packet) {
    return SEND_PACKET;
  }

 protected:
  RtpRtcpObserver() : RtpRtcpObserver(TimeDelta::Zero()) {}
  explicit RtpRtcpObserver(TimeDelta event_timeout) : timeout_(event_timeout) {}

  Event observation_complete_;

 private:
  const TimeDelta timeout_;
};

class PacketTransport : public test::DirectTransport {
 public:
  enum TransportType { kReceiver, kSender };

  PacketTransport(TaskQueueBase* task_queue,
                  Call* send_call,
                  RtpRtcpObserver* observer,
                  TransportType transport_type,
                  const std::map<uint8_t, MediaType>& payload_type_map,
                  std::unique_ptr<SimulatedPacketReceiverInterface> nw_pipe,
                  ArrayView<const RtpExtension> audio_extensions,
                  ArrayView<const RtpExtension> video_extensions)
      : test::DirectTransport(task_queue,
                              std::move(nw_pipe),
                              send_call,
                              payload_type_map,
                              audio_extensions,
                              video_extensions),
        observer_(observer),
        transport_type_(transport_type) {}

 private:
  bool SendRtp(ArrayView<const uint8_t> packet,
               const PacketOptions& options) override {
    EXPECT_TRUE(IsRtpPacket(packet));
    RtpRtcpObserver::Action action = RtpRtcpObserver::SEND_PACKET;
    if (observer_) {
      if (transport_type_ == kSender) {
        action = observer_->OnSendRtp(packet);
      } else {
        action = observer_->OnReceiveRtp(packet);
      }
    }
    switch (action) {
      case RtpRtcpObserver::DROP_PACKET:
        // Drop packet silently.
        return true;
      case RtpRtcpObserver::SEND_PACKET:
        return test::DirectTransport::SendRtp(packet, options);
    }
    return true;  // Will never happen, makes compiler happy.
  }

  bool SendRtcp(ArrayView<const uint8_t> packet) override {
    EXPECT_TRUE(IsRtcpPacket(packet));
    RtpRtcpObserver::Action action = RtpRtcpObserver::SEND_PACKET;
    if (observer_) {
      if (transport_type_ == kSender) {
        action = observer_->OnSendRtcp(packet);
      } else {
        action = observer_->OnReceiveRtcp(packet);
      }
    }
    switch (action) {
      case RtpRtcpObserver::DROP_PACKET:
        // Drop packet silently.
        return true;
      case RtpRtcpObserver::SEND_PACKET:
        return test::DirectTransport::SendRtcp(packet);
    }
    return true;  // Will never happen, makes compiler happy.
  }

  RtpRtcpObserver* const observer_;
  TransportType transport_type_;
};
}  // namespace test
}  // namespace webrtc

#endif  // TEST_RTP_RTCP_OBSERVER_H_
