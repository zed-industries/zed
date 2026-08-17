/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef CALL_RTP_TRANSPORT_CONTROLLER_SEND_H_
#define CALL_RTP_TRANSPORT_CONTROLLER_SEND_H_

#include <cstddef>
#include <cstdint>
#include <map>
#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "api/environment/environment.h"
#include "api/fec_controller.h"
#include "api/frame_transformer_interface.h"
#include "api/scoped_refptr.h"
#include "api/sequence_checker.h"
#include "api/task_queue/pending_task_safety_flag.h"
#include "api/task_queue/task_queue_base.h"
#include "api/transport/bandwidth_estimation_settings.h"
#include "api/transport/bitrate_settings.h"
#include "api/transport/network_control.h"
#include "api/transport/network_types.h"
#include "api/units/data_rate.h"
#include "api/units/data_size.h"
#include "api/units/time_delta.h"
#include "api/units/timestamp.h"
#include "call/rtp_bitrate_configurator.h"
#include "call/rtp_config.h"
#include "call/rtp_transport_config.h"
#include "call/rtp_transport_controller_send_interface.h"
#include "call/rtp_video_sender.h"
#include "modules/congestion_controller/rtp/control_handler.h"
#include "modules/congestion_controller/rtp/transport_feedback_adapter.h"
#include "modules/congestion_controller/rtp/transport_feedback_demuxer.h"
#include "modules/pacing/packet_router.h"
#include "modules/pacing/task_queue_paced_sender.h"
#include "modules/rtp_rtcp/include/report_block_data.h"
#include "modules/rtp_rtcp/include/rtp_rtcp_defines.h"
#include "modules/rtp_rtcp/source/rtcp_packet/congestion_control_feedback.h"
#include "rtc_base/experiments/field_trial_parser.h"
#include "rtc_base/network_route.h"
#include "rtc_base/rate_limiter.h"
#include "rtc_base/task_utils/repeating_task.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {
class FrameEncryptorInterface;

class RtpTransportControllerSend final
    : public RtpTransportControllerSendInterface,
      public NetworkLinkRtcpObserver,
      public NetworkStateEstimateObserver {
 public:
  explicit RtpTransportControllerSend(const RtpTransportConfig& config);
  ~RtpTransportControllerSend() override;

  RtpTransportControllerSend(const RtpTransportControllerSend&) = delete;
  RtpTransportControllerSend& operator=(const RtpTransportControllerSend&) =
      delete;

  // TODO(tommi): Change to std::unique_ptr<>.
  RtpVideoSenderInterface* CreateRtpVideoSender(
      const std::map<uint32_t, RtpState>& suspended_ssrcs,
      const std::map<uint32_t, RtpPayloadState>&
          states,  // move states into RtpTransportControllerSend
      const RtpConfig& rtp_config,
      int rtcp_report_interval_ms,
      Transport* send_transport,
      const RtpSenderObservers& observers,
      std::unique_ptr<FecController> fec_controller,
      const RtpSenderFrameEncryptionConfig& frame_encryption_config,
      scoped_refptr<FrameTransformerInterface> frame_transformer) override;
  void DestroyRtpVideoSender(
      RtpVideoSenderInterface* rtp_video_sender) override;

  // Implements RtpTransportControllerSendInterface
  void RegisterSendingRtpStream(RtpRtcpInterface& rtp_module) override;
  void DeRegisterSendingRtpStream(RtpRtcpInterface& rtp_module) override;
  PacketRouter* packet_router() override;

  NetworkStateEstimateObserver* network_state_estimate_observer() override;
  RtpPacketSender* packet_sender() override;

  void SetAllocatedSendBitrateLimits(BitrateAllocationLimits limits) override;
  void ReconfigureBandwidthEstimation(
      const BandwidthEstimationSettings& settings) override;

  void SetPacingFactor(float pacing_factor) override;
  void SetQueueTimeLimit(int limit_ms) override;
  StreamFeedbackProvider* GetStreamFeedbackProvider() override;
  void RegisterTargetTransferRateObserver(
      TargetTransferRateObserver* observer) override;
  void OnNetworkRouteChanged(absl::string_view transport_name,
                             const NetworkRoute& network_route) override;
  void OnNetworkAvailability(bool network_available) override;
  NetworkLinkRtcpObserver* GetRtcpObserver() override;
  int64_t GetPacerQueuingDelayMs() const override;
  std::optional<Timestamp> GetFirstPacketTime() const override;
  void EnablePeriodicAlrProbing(bool enable) override;
  void OnSentPacket(const SentPacketInfo& sent_packet) override;
  void OnReceivedPacket(const ReceivedPacket& packet_msg) override;

  void SetSdpBitrateParameters(const BitrateConstraints& constraints) override;
  void SetClientBitratePreferences(const BitrateSettings& preferences) override;

  void OnTransportOverheadChanged(
      size_t transport_overhead_bytes_per_packet) override;

  void AccountForAudioPacketsInPacedSender(bool account_for_audio) override;
  void IncludeOverheadInPacedSender() override;
  void EnsureStarted() override;

  // Implements NetworkLinkRtcpObserver interface
  void OnReceiverEstimatedMaxBitrate(Timestamp receive_time,
                                     DataRate bitrate) override;
  void OnReport(Timestamp receive_time,
                ArrayView<const ReportBlockData> report_blocks) override;
  void OnRttUpdate(Timestamp receive_time, TimeDelta rtt) override;
  void OnTransportFeedback(Timestamp receive_time,
                           const rtcp::TransportFeedback& feedback) override;
  void OnCongestionControlFeedback(
      Timestamp receive_time,
      const rtcp::CongestionControlFeedback& feedback) override;

  // Implements NetworkStateEstimateObserver interface
  void OnRemoteNetworkEstimate(NetworkStateEstimate estimate) override;

  NetworkControllerInterface* GetNetworkController() override {
    RTC_DCHECK_RUN_ON(&sequence_checker_);
    return controller_.get();
  }

  // Called once it's known that the remote end supports RFC 8888.
  void EnableCongestionControlFeedbackAccordingToRfc8888() override;

  int ReceivedCongestionControlFeedbackCount() const override {
    RTC_DCHECK_RUN_ON(&sequence_checker_);
    return feedback_count_;
  }
  int ReceivedTransportCcFeedbackCount() const override {
    RTC_DCHECK_RUN_ON(&sequence_checker_);
    return transport_cc_feedback_count_;
  }

 private:
  void MaybeCreateControllers() RTC_RUN_ON(sequence_checker_);
  void HandleTransportPacketsFeedback(const TransportPacketsFeedback& feedback)
      RTC_RUN_ON(sequence_checker_);
  void UpdateNetworkAvailability() RTC_RUN_ON(sequence_checker_);
  void UpdateInitialConstraints(TargetRateConstraints new_contraints)
      RTC_RUN_ON(sequence_checker_);

  void StartProcessPeriodicTasks() RTC_RUN_ON(sequence_checker_);
  void UpdateControllerWithTimeInterval() RTC_RUN_ON(sequence_checker_);

  std::optional<BitrateConstraints> ApplyOrLiftRelayCap(bool is_relayed);
  bool IsRelevantRouteChange(const NetworkRoute& old_route,
                             const NetworkRoute& new_route) const;
  void UpdateBitrateConstraints(const BitrateConstraints& updated);
  void UpdateStreamsConfig() RTC_RUN_ON(sequence_checker_);
  void PostUpdates(NetworkControlUpdate update) RTC_RUN_ON(sequence_checker_);
  void UpdateControlState() RTC_RUN_ON(sequence_checker_);
  void UpdateCongestedState() RTC_RUN_ON(sequence_checker_);
  std::optional<bool> GetCongestedStateUpdate() const
      RTC_RUN_ON(sequence_checker_);

  // Called by packet router just before packet is sent to the RTP modules.
  void NotifyBweOfPacedSentPacket(const RtpPacketToSend& packet,
                                  const PacedPacketInfo& pacing_info);
  void ProcessSentPacket(const SentPacketInfo& sent_packet)
      RTC_RUN_ON(sequence_checker_);
  void ProcessSentPacketUpdates(NetworkControlUpdate updates)
      RTC_RUN_ON(sequence_checker_);

  const Environment env_;
  SequenceChecker sequence_checker_;
  TaskQueueBase* task_queue_;
  PacketRouter packet_router_;

  std::vector<std::unique_ptr<RtpVideoSenderInterface>> video_rtp_senders_
      RTC_GUARDED_BY(&sequence_checker_);
  RtpBitrateConfigurator bitrate_configurator_;
  std::map<std::string, NetworkRoute> network_routes_
      RTC_GUARDED_BY(sequence_checker_);
  BandwidthEstimationSettings bwe_settings_ RTC_GUARDED_BY(sequence_checker_);
  bool pacer_started_ RTC_GUARDED_BY(sequence_checker_);
  TaskQueuePacedSender pacer_;

  TargetTransferRateObserver* observer_ RTC_GUARDED_BY(sequence_checker_);
  TransportFeedbackDemuxer feedback_demuxer_;

  TransportFeedbackAdapter transport_feedback_adapter_
      RTC_GUARDED_BY(sequence_checker_);

  NetworkControllerFactoryInterface* const controller_factory_override_
      RTC_PT_GUARDED_BY(sequence_checker_);
  const std::unique_ptr<NetworkControllerFactoryInterface>
      controller_factory_fallback_ RTC_PT_GUARDED_BY(sequence_checker_);

  std::unique_ptr<CongestionControlHandler> control_handler_
      RTC_GUARDED_BY(sequence_checker_) RTC_PT_GUARDED_BY(sequence_checker_);

  std::unique_ptr<NetworkControllerInterface> controller_
      RTC_GUARDED_BY(sequence_checker_) RTC_PT_GUARDED_BY(sequence_checker_);

  TimeDelta process_interval_ RTC_GUARDED_BY(sequence_checker_);

  struct LossReport {
    uint32_t extended_highest_sequence_number = 0;
    int cumulative_lost = 0;
  };
  std::map<uint32_t, LossReport> last_report_blocks_
      RTC_GUARDED_BY(sequence_checker_);
  Timestamp last_report_block_time_ RTC_GUARDED_BY(sequence_checker_);

  NetworkControllerConfig initial_config_ RTC_GUARDED_BY(sequence_checker_);
  StreamsConfig streams_config_ RTC_GUARDED_BY(sequence_checker_);

  const bool reset_feedback_on_route_change_;
  const bool add_pacing_to_cwin_;
  const bool reset_bwe_on_adapter_id_change_;

  FieldTrialParameter<DataRate> relay_bandwidth_cap_;

  size_t transport_overhead_bytes_per_packet_ RTC_GUARDED_BY(sequence_checker_);
  bool network_available_ RTC_GUARDED_BY(sequence_checker_);
  RepeatingTaskHandle pacer_queue_update_task_
      RTC_GUARDED_BY(sequence_checker_);
  RepeatingTaskHandle controller_task_ RTC_GUARDED_BY(sequence_checker_);

  DataSize congestion_window_size_ RTC_GUARDED_BY(sequence_checker_);
  bool is_congested_ RTC_GUARDED_BY(sequence_checker_);
  bool transport_maybe_support_ecn_ =
      false;  // True if RFC8888 has been negotiated.
  bool sending_packets_as_ect1_ = false;
  // Count of feedback messages received.
  int feedback_count_ RTC_GUARDED_BY(sequence_checker_) = 0;
  int transport_cc_feedback_count_ RTC_GUARDED_BY(sequence_checker_) = 0;

  // Protected by internal locks.
  RateLimiter retransmission_rate_limiter_;

  ScopedTaskSafety safety_;
};

}  // namespace webrtc

#endif  // CALL_RTP_TRANSPORT_CONTROLLER_SEND_H_
