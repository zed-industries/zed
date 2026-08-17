/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef CALL_RTP_TRANSPORT_CONTROLLER_SEND_INTERFACE_H_
#define CALL_RTP_TRANSPORT_CONTROLLER_SEND_INTERFACE_H_
#include <stddef.h>
#include <stdint.h>

#include <map>
#include <memory>
#include <optional>

#include "absl/strings/string_view.h"
#include "api/crypto/crypto_options.h"
#include "api/fec_controller.h"
#include "api/frame_transformer_interface.h"
#include "api/rtp_packet_sender.h"
#include "api/scoped_refptr.h"
#include "api/transport/bandwidth_estimation_settings.h"
#include "api/transport/bitrate_settings.h"
#include "api/transport/network_control.h"
#include "api/transport/network_types.h"
#include "api/units/timestamp.h"
#include "call/rtp_config.h"
#include "common_video/frame_counts.h"
#include "modules/rtp_rtcp/include/report_block_data.h"
#include "modules/rtp_rtcp/include/rtcp_statistics.h"
#include "modules/rtp_rtcp/include/rtp_rtcp_defines.h"
#include "rtc_base/network_route.h"

namespace webrtc {

struct SentPacketInfo;
class FrameEncryptorInterface;
class TargetTransferRateObserver;
class Transport;
class PacketRouter;
class RtpVideoSenderInterface;
class RtpPacketSender;
class RtpRtcpInterface;

struct RtpSenderObservers {
  RtcpRttStats* rtcp_rtt_stats;
  RtcpIntraFrameObserver* intra_frame_callback;
  RtcpLossNotificationObserver* rtcp_loss_notification_observer;
  ReportBlockDataObserver* report_block_data_observer;
  StreamDataCountersCallback* rtp_stats;
  BitrateStatisticsObserver* bitrate_observer;
  FrameCountObserver* frame_count_observer;
  RtcpPacketTypeCounterObserver* rtcp_type_observer;
  SendPacketObserver* send_packet_observer;
};

struct RtpSenderFrameEncryptionConfig {
  FrameEncryptorInterface* frame_encryptor = nullptr;
  CryptoOptions crypto_options;
};

// An RtpTransportController should own everything related to the RTP
// transport to/from a remote endpoint. We should have separate
// interfaces for send and receive side, even if they are implemented
// by the same class. This is an ongoing refactoring project. At some
// point, this class should be promoted to a public api under
// webrtc/api/rtp/.
//
// For a start, this object is just a collection of the objects needed
// by the VideoSendStream constructor. The plan is to move ownership
// of all RTP-related objects here, and add methods to create per-ssrc
// objects which would then be passed to VideoSendStream. Eventually,
// direct accessors like packet_router() should be removed.
//
// This should also have a reference to the underlying
// webrtc::Transport(s). Currently, webrtc::Transport is implemented by
// WebRtcVideoChannel and WebRtcVoiceMediaChannel, and owned by
// WebrtcSession. Video and audio always uses different transport
// objects, even in the common case where they are bundled over the
// same underlying transport.
//
// Extracting the logic of the webrtc::Transport from BaseChannel and
// subclasses into a separate class seems to be a prerequesite for
// moving the transport here.
class RtpTransportControllerSendInterface {
 public:
  virtual ~RtpTransportControllerSendInterface() {}
  virtual PacketRouter* packet_router() = 0;

  virtual RtpVideoSenderInterface* CreateRtpVideoSender(
      const std::map<uint32_t, RtpState>& suspended_ssrcs,
      // TODO(holmer): Move states into RtpTransportControllerSend.
      const std::map<uint32_t, RtpPayloadState>& states,
      const RtpConfig& rtp_config,
      int rtcp_report_interval_ms,
      Transport* send_transport,
      const RtpSenderObservers& observers,
      std::unique_ptr<FecController> fec_controller,
      const RtpSenderFrameEncryptionConfig& frame_encryption_config,
      scoped_refptr<FrameTransformerInterface> frame_transformer) = 0;
  virtual void DestroyRtpVideoSender(
      RtpVideoSenderInterface* rtp_video_sender) = 0;

  // Register a specific RTP stream as sending. This means that the pacer and
  // packet router can send packets using this RTP stream.
  virtual void RegisterSendingRtpStream(RtpRtcpInterface& rtp_module) = 0;
  // Pacer and PacketRouter stop using this RTP stream.
  virtual void DeRegisterSendingRtpStream(RtpRtcpInterface& rtp_module) = 0;

  virtual NetworkStateEstimateObserver* network_state_estimate_observer() = 0;

  virtual RtpPacketSender* packet_sender() = 0;

  // SetAllocatedSendBitrateLimits sets bitrates limits imposed by send codec
  // settings.
  virtual void SetAllocatedSendBitrateLimits(
      BitrateAllocationLimits limits) = 0;

  virtual void ReconfigureBandwidthEstimation(
      const BandwidthEstimationSettings& settings) = 0;

  virtual void SetPacingFactor(float pacing_factor) = 0;
  virtual void SetQueueTimeLimit(int limit_ms) = 0;

  virtual StreamFeedbackProvider* GetStreamFeedbackProvider() = 0;
  virtual void RegisterTargetTransferRateObserver(
      TargetTransferRateObserver* observer) = 0;
  virtual void OnNetworkRouteChanged(absl::string_view transport_name,
                                     const NetworkRoute& network_route) = 0;
  virtual void OnNetworkAvailability(bool network_available) = 0;
  virtual NetworkLinkRtcpObserver* GetRtcpObserver() = 0;
  virtual int64_t GetPacerQueuingDelayMs() const = 0;
  virtual std::optional<Timestamp> GetFirstPacketTime() const = 0;
  virtual void EnablePeriodicAlrProbing(bool enable) = 0;

  // Called when a packet has been sent.
  // The call should arrive on the network thread, but may not in all cases
  // (some tests don't adhere to this). Implementations today should not block
  // the calling thread or make assumptions about the thread context.
  virtual void OnSentPacket(const SentPacketInfo& sent_packet) = 0;

  virtual void OnReceivedPacket(const ReceivedPacket& received_packet) = 0;

  virtual void SetSdpBitrateParameters(
      const BitrateConstraints& constraints) = 0;
  virtual void SetClientBitratePreferences(
      const BitrateSettings& preferences) = 0;

  virtual void OnTransportOverheadChanged(
      size_t transport_overhead_per_packet) = 0;

  virtual void AccountForAudioPacketsInPacedSender(bool account_for_audio) = 0;
  virtual void IncludeOverheadInPacedSender() = 0;

  virtual void EnsureStarted() = 0;
  virtual NetworkControllerInterface* GetNetworkController() = 0;

  // Called once it's known that the remote end supports RFC 8888.
  virtual void EnableCongestionControlFeedbackAccordingToRfc8888() = 0;
  // Count of RFC8888 feedback reports received
  virtual int ReceivedCongestionControlFeedbackCount() const = 0;
  // Count of transport-cc feedback reports received
  virtual int ReceivedTransportCcFeedbackCount() const = 0;
};

}  // namespace webrtc

#endif  // CALL_RTP_TRANSPORT_CONTROLLER_SEND_INTERFACE_H_
