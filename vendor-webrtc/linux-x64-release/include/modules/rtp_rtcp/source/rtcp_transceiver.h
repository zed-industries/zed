/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_RTP_RTCP_SOURCE_RTCP_TRANSCEIVER_H_
#define MODULES_RTP_RTCP_SOURCE_RTCP_TRANSCEIVER_H_

#include <cstdint>
#include <memory>
#include <vector>

#include "absl/functional/any_invocable.h"
#include "api/task_queue/task_queue_base.h"
#include "modules/rtp_rtcp/include/rtp_rtcp_defines.h"
#include "modules/rtp_rtcp/source/rtcp_packet.h"
#include "modules/rtp_rtcp/source/rtcp_transceiver_config.h"
#include "modules/rtp_rtcp/source/rtcp_transceiver_impl.h"
#include "rtc_base/copy_on_write_buffer.h"
#include "system_wrappers/include/clock.h"

namespace webrtc {
//
// Manage incoming and outgoing rtcp messages for multiple BUNDLED streams.
//
// This class is thread-safe wrapper of RtcpTransceiverImpl
class RtcpTransceiver : public RtcpFeedbackSenderInterface {
 public:
  explicit RtcpTransceiver(const RtcpTransceiverConfig& config);
  RtcpTransceiver(const RtcpTransceiver&) = delete;
  RtcpTransceiver& operator=(const RtcpTransceiver&) = delete;
  // Note that interfaces provided in constructor still might be used after the
  // destructor. However they can only be used on the confic.task_queue.
  // Use Stop function to get notified when they are no longer used or
  // ensure those objects outlive the task queue.
  ~RtcpTransceiver() override;

  // Start asynchronious destruction of the RtcpTransceiver.
  // It is safe to call destructor right after Stop exits.
  // No other methods can be called.
  // Note that interfaces provided in constructor or registered with AddObserver
  // still might be used by the transceiver on the task queue
  // until `on_destroyed` runs.
  void Stop(absl::AnyInvocable<void() &&> on_destroyed);

  // Registers observer to be notified about incoming rtcp packets.
  // Calls to observer will be done on the `config.task_queue`.
  void AddMediaReceiverRtcpObserver(uint32_t remote_ssrc,
                                    MediaReceiverRtcpObserver* observer);
  // Deregisters the observer. Might return before observer is deregistered.
  // Runs `on_removed` when observer is deregistered.
  void RemoveMediaReceiverRtcpObserver(
      uint32_t remote_ssrc,
      MediaReceiverRtcpObserver* observer,
      absl::AnyInvocable<void() &&> on_removed);

  // Enables/disables sending rtcp packets eventually.
  // Packets may be sent after the SetReadyToSend(false) returns, but no new
  // packets will be scheduled.
  void SetReadyToSend(bool ready);

  // Handles incoming rtcp packets.
  void ReceivePacket(CopyOnWriteBuffer packet);

  // Sends RTCP packets starting with a sender or receiver report.
  void SendCompoundPacket();

  // (REMB) Receiver Estimated Max Bitrate.
  // Includes REMB in following compound packets and sends a REMB message
  // immediately if 'RtcpTransceiverConfig::send_remb_on_change' is set.
  void SetRemb(int64_t bitrate_bps, std::vector<uint32_t> ssrcs) override;
  // Stops sending REMB in following compound packets.
  void UnsetRemb() override;

  // TODO(bugs.webrtc.org/8239): Remove SendCombinedRtcpPacket
  // and move generating of the TransportFeedback message inside
  // RtcpTransceiverImpl when there is one RtcpTransceiver per rtp transport.
  void SendCombinedRtcpPacket(
      std::vector<std::unique_ptr<rtcp::RtcpPacket>> rtcp_packets) override;

  // Reports missing packets, https://tools.ietf.org/html/rfc4585#section-6.2.1
  void SendNack(uint32_t ssrc, std::vector<uint16_t> sequence_numbers);

  // Requests new key frame.
  // using PLI, https://tools.ietf.org/html/rfc4585#section-6.3.1.1
  void SendPictureLossIndication(uint32_t ssrc);
  // using FIR, https://tools.ietf.org/html/rfc5104#section-4.3.1.2
  // Use the SendFullIntraRequest(ssrcs, true) instead.
  void SendFullIntraRequest(std::vector<uint32_t> ssrcs);
  // If new_request is true then requested sequence no. will increase for each
  // requested ssrc.
  void SendFullIntraRequest(std::vector<uint32_t> ssrcs, bool new_request);

 private:
  Clock* const clock_;
  TaskQueueBase* const task_queue_;
  std::unique_ptr<RtcpTransceiverImpl> rtcp_transceiver_;
};

}  // namespace webrtc

#endif  // MODULES_RTP_RTCP_SOURCE_RTCP_TRANSCEIVER_H_
