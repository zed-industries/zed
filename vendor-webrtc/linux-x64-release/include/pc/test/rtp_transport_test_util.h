/*
 *  Copyright 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_TEST_RTP_TRANSPORT_TEST_UTIL_H_
#define PC_TEST_RTP_TRANSPORT_TEST_UTIL_H_

#include <cstdint>
#include <utility>

#include "absl/functional/any_invocable.h"
#include "call/rtp_packet_sink_interface.h"
#include "modules/rtp_rtcp/source/rtp_packet_received.h"
#include "pc/rtp_transport_internal.h"
#include "rtc_base/copy_on_write_buffer.h"
#include "rtc_base/network/sent_packet.h"

namespace webrtc {

// Used to handle the signals when the RtpTransport receives an RTP/RTCP packet.
// Used in Rtp/Srtp/DtlsTransport unit tests.
class TransportObserver : public RtpPacketSinkInterface {
 public:
  TransportObserver() {}

  explicit TransportObserver(RtpTransportInternal* rtp_transport) {
    rtp_transport->SubscribeRtcpPacketReceived(
        this, [this](CopyOnWriteBuffer* buffer, int64_t packet_time_ms) {
          OnRtcpPacketReceived(buffer, packet_time_ms);
        });
    rtp_transport->SubscribeReadyToSend(
        this, [this](bool arg) { OnReadyToSend(arg); });
    rtp_transport->SetUnDemuxableRtpPacketReceivedHandler(
        [this](RtpPacketReceived& packet) { OnUndemuxableRtpPacket(packet); });
    rtp_transport->SubscribeSentPacket(this,
                                       [this](const SentPacketInfo& packet) {
                                         sent_packet_count_++;
                                         if (action_on_sent_packet_) {
                                           action_on_sent_packet_();
                                         }
                                       });
  }

  // RtpPacketInterface override.
  void OnRtpPacket(const RtpPacketReceived& packet) override {
    rtp_count_++;
    last_recv_rtp_packet_ = packet;
  }

  void OnUndemuxableRtpPacket(const RtpPacketReceived& packet) {
    un_demuxable_rtp_count_++;
  }

  void OnRtcpPacketReceived(CopyOnWriteBuffer* packet, int64_t packet_time_us) {
    rtcp_count_++;
    last_recv_rtcp_packet_ = *packet;
  }

  int rtp_count() const { return rtp_count_; }
  int un_demuxable_rtp_count() const { return un_demuxable_rtp_count_; }
  int rtcp_count() const { return rtcp_count_; }
  int sent_packet_count() const { return sent_packet_count_; }

  const RtpPacketReceived& last_recv_rtp_packet() {
    return last_recv_rtp_packet_;
  }

  CopyOnWriteBuffer last_recv_rtcp_packet() { return last_recv_rtcp_packet_; }

  void OnReadyToSend(bool ready) {
    if (action_on_ready_to_send_) {
      action_on_ready_to_send_(ready);
    }
    ready_to_send_signal_count_++;
    ready_to_send_ = ready;
  }

  bool ready_to_send() { return ready_to_send_; }

  int ready_to_send_signal_count() { return ready_to_send_signal_count_; }

  void SetActionOnReadyToSend(absl::AnyInvocable<void(bool)> action) {
    action_on_ready_to_send_ = std::move(action);
  }
  void SetActionOnSentPacket(absl::AnyInvocable<void()> action) {
    action_on_sent_packet_ = std::move(action);
  }

 private:
  bool ready_to_send_ = false;
  int rtp_count_ = 0;
  int un_demuxable_rtp_count_ = 0;
  int rtcp_count_ = 0;
  int sent_packet_count_ = 0;
  int ready_to_send_signal_count_ = 0;
  RtpPacketReceived last_recv_rtp_packet_;
  CopyOnWriteBuffer last_recv_rtcp_packet_;
  absl::AnyInvocable<void(bool)> action_on_ready_to_send_;
  absl::AnyInvocable<void()> action_on_sent_packet_;
};

}  // namespace webrtc

#endif  // PC_TEST_RTP_TRANSPORT_TEST_UTIL_H_
