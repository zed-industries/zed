/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef CALL_TEST_MOCK_RTP_TRANSPORT_CONTROLLER_SEND_H_
#define CALL_TEST_MOCK_RTP_TRANSPORT_CONTROLLER_SEND_H_

#include <cstddef>
#include <cstdint>
#include <map>
#include <memory>
#include <optional>

#include "absl/strings/string_view.h"
#include "api/fec_controller.h"
#include "api/frame_transformer_interface.h"
#include "api/scoped_refptr.h"
#include "api/transport/bandwidth_estimation_settings.h"
#include "api/transport/bitrate_settings.h"
#include "api/transport/network_control.h"
#include "api/transport/network_types.h"
#include "api/units/timestamp.h"
#include "call/rtp_config.h"
#include "call/rtp_transport_controller_send_interface.h"
#include "modules/pacing/packet_router.h"
#include "modules/rtp_rtcp/include/rtp_rtcp_defines.h"
#include "modules/rtp_rtcp/source/rtp_rtcp_interface.h"
#include "rtc_base/network/sent_packet.h"
#include "rtc_base/network_route.h"
#include "test/gmock.h"

namespace webrtc {

class MockRtpTransportControllerSend
    : public RtpTransportControllerSendInterface {
 public:
  MOCK_METHOD(RtpVideoSenderInterface*,
              CreateRtpVideoSender,
              ((const std::map<uint32_t, RtpState>&),
               (const std::map<uint32_t, RtpPayloadState>&),
               const RtpConfig&,
               int rtcp_report_interval_ms,
               Transport*,
               const RtpSenderObservers&,
               std::unique_ptr<FecController>,
               const RtpSenderFrameEncryptionConfig&,
               scoped_refptr<FrameTransformerInterface>),
              (override));
  MOCK_METHOD(void,
              DestroyRtpVideoSender,
              (RtpVideoSenderInterface*),
              (override));
  MOCK_METHOD(void, RegisterSendingRtpStream, (RtpRtcpInterface&), (override));
  MOCK_METHOD(void,
              DeRegisterSendingRtpStream,
              (RtpRtcpInterface&),
              (override));
  MOCK_METHOD(PacketRouter*, packet_router, (), (override));
  MOCK_METHOD(NetworkStateEstimateObserver*,
              network_state_estimate_observer,
              (),
              (override));
  MOCK_METHOD(RtpPacketSender*, packet_sender, (), (override));
  MOCK_METHOD(void,
              SetAllocatedSendBitrateLimits,
              (BitrateAllocationLimits),
              (override));
  MOCK_METHOD(void,
              ReconfigureBandwidthEstimation,
              (const BandwidthEstimationSettings&),
              (override));
  MOCK_METHOD(void, SetPacingFactor, (float), (override));
  MOCK_METHOD(void, SetQueueTimeLimit, (int), (override));
  MOCK_METHOD(StreamFeedbackProvider*,
              GetStreamFeedbackProvider,
              (),
              (override));
  MOCK_METHOD(void,
              RegisterTargetTransferRateObserver,
              (TargetTransferRateObserver*),
              (override));
  MOCK_METHOD(void,
              OnNetworkRouteChanged,
              (absl::string_view, const webrtc::NetworkRoute&),
              (override));
  MOCK_METHOD(void, OnNetworkAvailability, (bool), (override));
  MOCK_METHOD(NetworkLinkRtcpObserver*, GetRtcpObserver, (), (override));
  MOCK_METHOD(int64_t, GetPacerQueuingDelayMs, (), (const, override));
  MOCK_METHOD(std::optional<Timestamp>,
              GetFirstPacketTime,
              (),
              (const, override));
  MOCK_METHOD(void, EnablePeriodicAlrProbing, (bool), (override));
  MOCK_METHOD(void, OnSentPacket, (const SentPacketInfo&), (override));
  MOCK_METHOD(void,
              SetSdpBitrateParameters,
              (const BitrateConstraints&),
              (override));
  MOCK_METHOD(void,
              SetClientBitratePreferences,
              (const BitrateSettings&),
              (override));
  MOCK_METHOD(void, OnTransportOverheadChanged, (size_t), (override));
  MOCK_METHOD(void, AccountForAudioPacketsInPacedSender, (bool), (override));
  MOCK_METHOD(void, IncludeOverheadInPacedSender, (), (override));
  MOCK_METHOD(void, OnReceivedPacket, (const ReceivedPacket&), (override));
  MOCK_METHOD(void, EnsureStarted, (), (override));
  MOCK_METHOD(NetworkControllerInterface*,
              GetNetworkController,
              (),
              (override));
  MOCK_METHOD(void,
              EnableCongestionControlFeedbackAccordingToRfc8888,
              (),
              (override));
  MOCK_METHOD(int,
              ReceivedCongestionControlFeedbackCount,
              (),
              (const, override));
  MOCK_METHOD(int, ReceivedTransportCcFeedbackCount, (), (const, override));
};
}  // namespace webrtc
#endif  // CALL_TEST_MOCK_RTP_TRANSPORT_CONTROLLER_SEND_H_
