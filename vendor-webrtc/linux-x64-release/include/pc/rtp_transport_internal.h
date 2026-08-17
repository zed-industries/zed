/*
 *  Copyright 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_RTP_TRANSPORT_INTERNAL_H_
#define PC_RTP_TRANSPORT_INTERNAL_H_

#include <cstdint>
#include <optional>
#include <string>
#include <utility>

#include "absl/functional/any_invocable.h"
#include "call/rtp_demuxer.h"
#include "pc/session_description.h"
#include "rtc_base/callback_list.h"
#include "rtc_base/copy_on_write_buffer.h"
#include "rtc_base/network/sent_packet.h"
#include "rtc_base/network_route.h"
#include "rtc_base/socket.h"
#include "rtc_base/third_party/sigslot/sigslot.h"

namespace webrtc {

class CopyOnWriteBuffer;

// This class is an internal interface; it is not accessible to API consumers
// but is accessible to internal classes in order to send and receive RTP and
// RTCP packets belonging to a single RTP session. Additional convenience and
// configuration methods are also provided.
class RtpTransportInternal : public sigslot::has_slots<> {
 public:
  virtual ~RtpTransportInternal() = default;

  virtual void SetRtcpMuxEnabled(bool enable) = 0;

  virtual const std::string& transport_name() const = 0;

  // Sets socket options on the underlying RTP or RTCP transports.
  virtual int SetRtpOption(Socket::Option opt, int value) = 0;
  virtual int SetRtcpOption(Socket::Option opt, int value) = 0;

  virtual bool rtcp_mux_enabled() const = 0;

  virtual bool IsReadyToSend() const = 0;

  // Called whenever a transport's ready-to-send state changes. The argument
  // is true if all used transports are ready to send. This is more specific
  // than just "writable"; it means the last send didn't return ENOTCONN.
  void SubscribeReadyToSend(const void* tag,
                            absl::AnyInvocable<void(bool)> callback) {
    callback_list_ready_to_send_.AddReceiver(tag, std::move(callback));
  }
  void UnsubscribeReadyToSend(const void* tag) {
    callback_list_ready_to_send_.RemoveReceivers(tag);
  }

  // Called whenever an RTCP packet is received. There is no equivalent signal
  // for demuxable RTP packets because they would be forwarded to the
  // BaseChannel through the RtpDemuxer callback.
  void SubscribeRtcpPacketReceived(
      const void* tag,
      absl::AnyInvocable<void(webrtc::CopyOnWriteBuffer*, int64_t)> callback) {
    callback_list_rtcp_packet_received_.AddReceiver(tag, std::move(callback));
  }
  // There doesn't seem to be a need to unsubscribe from this signal.

  // Called whenever a RTP packet that can not be demuxed by the transport is
  // received.
  void SetUnDemuxableRtpPacketReceivedHandler(
      absl::AnyInvocable<void(RtpPacketReceived&)> callback) {
    callback_undemuxable_rtp_packet_received_ = std::move(callback);
  }

  // Called whenever the network route of the P2P layer transport changes.
  // The argument is an optional network route.
  void SubscribeNetworkRouteChanged(
      const void* tag,
      absl::AnyInvocable<void(std::optional<webrtc::NetworkRoute>)> callback) {
    callback_list_network_route_changed_.AddReceiver(tag, std::move(callback));
  }
  void UnsubscribeNetworkRouteChanged(const void* tag) {
    callback_list_network_route_changed_.RemoveReceivers(tag);
  }

  // Called whenever a transport's writable state might change. The argument is
  // true if the transport is writable, otherwise it is false.
  void SubscribeWritableState(const void* tag,
                              absl::AnyInvocable<void(bool)> callback) {
    callback_list_writable_state_.AddReceiver(tag, std::move(callback));
  }
  void UnsubscribeWritableState(const void* tag) {
    callback_list_writable_state_.RemoveReceivers(tag);
  }
  void SubscribeSentPacket(
      const void* tag,
      absl::AnyInvocable<void(const webrtc::SentPacketInfo&)> callback) {
    callback_list_sent_packet_.AddReceiver(tag, std::move(callback));
  }
  void UnsubscribeSentPacket(const void* tag) {
    callback_list_sent_packet_.RemoveReceivers(tag);
  }

  virtual bool IsWritable(bool rtcp) const = 0;

  // TODO(zhihuang): Pass the `packet` by copy so that the original data
  // wouldn't be modified.
  virtual bool SendRtpPacket(CopyOnWriteBuffer* packet,
                             const AsyncSocketPacketOptions& options,
                             int flags) = 0;

  virtual bool SendRtcpPacket(CopyOnWriteBuffer* packet,
                              const AsyncSocketPacketOptions& options,
                              int flags) = 0;

  // This method updates the RTP header extension map so that the RTP transport
  // can parse the received packets and identify the MID. This is called by the
  // BaseChannel when setting the content description.
  //
  // TODO(zhihuang): Merging and replacing following methods handling header
  // extensions with SetParameters:
  //   UpdateRtpHeaderExtensionMap,
  //   UpdateSendEncryptedHeaderExtensionIds,
  //   UpdateRecvEncryptedHeaderExtensionIds,
  //   CacheRtpAbsSendTimeHeaderExtension,
  virtual void UpdateRtpHeaderExtensionMap(
      const RtpHeaderExtensions& header_extensions) = 0;

  virtual bool IsSrtpActive() const = 0;

  virtual bool RegisterRtpDemuxerSink(const RtpDemuxerCriteria& criteria,
                                      RtpPacketSinkInterface* sink) = 0;

  virtual bool UnregisterRtpDemuxerSink(RtpPacketSinkInterface* sink) = 0;

 protected:
  void SendReadyToSend(bool arg) { callback_list_ready_to_send_.Send(arg); }
  void SendRtcpPacketReceived(CopyOnWriteBuffer* buffer,
                              int64_t packet_time_us) {
    callback_list_rtcp_packet_received_.Send(buffer, packet_time_us);
  }
  void NotifyUnDemuxableRtpPacketReceived(RtpPacketReceived& packet) {
    callback_undemuxable_rtp_packet_received_(packet);
  }
  void SendNetworkRouteChanged(std::optional<NetworkRoute> route) {
    callback_list_network_route_changed_.Send(route);
  }
  void SendWritableState(bool state) {
    callback_list_writable_state_.Send(state);
  }
  void SendSentPacket(const SentPacketInfo& packet) {
    callback_list_sent_packet_.Send(packet);
  }

 private:
  CallbackList<bool> callback_list_ready_to_send_;
  CallbackList<CopyOnWriteBuffer*, int64_t> callback_list_rtcp_packet_received_;
  absl::AnyInvocable<void(RtpPacketReceived&)>
      callback_undemuxable_rtp_packet_received_ =
          [](RtpPacketReceived& packet) {};
  CallbackList<std::optional<NetworkRoute>>
      callback_list_network_route_changed_;
  CallbackList<bool> callback_list_writable_state_;
  CallbackList<const SentPacketInfo&> callback_list_sent_packet_;
};

}  // namespace webrtc

#endif  // PC_RTP_TRANSPORT_INTERNAL_H_
