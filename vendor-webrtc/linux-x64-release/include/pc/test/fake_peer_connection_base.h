/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_TEST_FAKE_PEER_CONNECTION_BASE_H_
#define PC_TEST_FAKE_PEER_CONNECTION_BASE_H_

#include <cstdint>
#include <map>
#include <memory>
#include <optional>
#include <set>
#include <string>
#include <type_traits>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/adaptation/resource.h"
#include "api/audio/audio_device.h"
#include "api/candidate.h"
#include "api/crypto/crypto_options.h"
#include "api/data_channel_event_observer_interface.h"
#include "api/data_channel_interface.h"
#include "api/dtls_transport_interface.h"
#include "api/field_trials_view.h"
#include "api/jsep.h"
#include "api/media_stream_interface.h"
#include "api/media_types.h"
#include "api/peer_connection_interface.h"
#include "api/rtc_error.h"
#include "api/rtc_event_log_output.h"
#include "api/rtp_parameters.h"
#include "api/rtp_receiver_interface.h"
#include "api/rtp_sender_interface.h"
#include "api/rtp_transceiver_interface.h"
#include "api/scoped_refptr.h"
#include "api/sctp_transport_interface.h"
#include "api/set_remote_description_observer_interface.h"
#include "api/stats/rtc_stats_collector_callback.h"
#include "api/transport/bandwidth_estimation_settings.h"
#include "api/transport/bitrate_settings.h"
#include "api/transport/network_control.h"
#include "call/call.h"
#include "call/payload_type_picker.h"
#include "p2p/base/port.h"
#include "p2p/base/port_allocator.h"
#include "pc/jsep_transport_controller.h"
#include "pc/peer_connection_internal.h"
#include "pc/peer_connection_message_handler.h"
#include "pc/rtp_transceiver.h"
#include "pc/rtp_transmission_manager.h"
#include "pc/session_description.h"
#include "pc/transport_stats.h"
#include "pc/usage_pattern.h"
#include "rtc_base/ref_counted_object.h"
#include "rtc_base/rtc_certificate.h"
#include "rtc_base/ssl_certificate.h"
#include "rtc_base/ssl_stream_adapter.h"
#include "rtc_base/thread.h"
#include "test/scoped_key_value_config.h"

namespace webrtc {

// Customized PeerConnection fakes can be created by subclassing
// FakePeerConnectionBase then overriding the interesting methods. This class
// takes care of providing default implementations for all the pure virtual
// functions specified in the interfaces.
class FakePeerConnectionBase : public PeerConnectionInternal {
 public:
  // PeerConnectionInterface implementation.

  scoped_refptr<StreamCollectionInterface> local_streams() override {
    return nullptr;
  }

  scoped_refptr<StreamCollectionInterface> remote_streams() override {
    return nullptr;
  }

  bool AddStream(MediaStreamInterface* stream) override { return false; }

  void RemoveStream(MediaStreamInterface* stream) override {}

  RTCErrorOr<scoped_refptr<RtpSenderInterface>> AddTrack(
      scoped_refptr<MediaStreamTrackInterface> track,
      const std::vector<std::string>& stream_ids) override {
    return RTCError(RTCErrorType::UNSUPPORTED_OPERATION, "Not implemented");
  }

  RTCErrorOr<scoped_refptr<RtpSenderInterface>> AddTrack(
      scoped_refptr<MediaStreamTrackInterface> track,
      const std::vector<std::string>& stream_ids,
      const std::vector<RtpEncodingParameters>& init_send_encodings) override {
    return RTCError(RTCErrorType::UNSUPPORTED_OPERATION, "Not implemented");
  }

  RTCError RemoveTrackOrError(
      scoped_refptr<RtpSenderInterface> sender) override {
    return RTCError(RTCErrorType::UNSUPPORTED_OPERATION);
  }

  RTCErrorOr<scoped_refptr<RtpTransceiverInterface>> AddTransceiver(
      scoped_refptr<MediaStreamTrackInterface> track) override {
    return RTCError(RTCErrorType::UNSUPPORTED_OPERATION, "Not implemented");
  }

  RTCErrorOr<scoped_refptr<RtpTransceiverInterface>> AddTransceiver(
      scoped_refptr<MediaStreamTrackInterface> track,
      const RtpTransceiverInit& init) override {
    return RTCError(RTCErrorType::UNSUPPORTED_OPERATION, "Not implemented");
  }

  RTCErrorOr<scoped_refptr<RtpTransceiverInterface>> AddTransceiver(
      webrtc::MediaType media_type) override {
    return RTCError(RTCErrorType::UNSUPPORTED_OPERATION, "Not implemented");
  }

  RTCErrorOr<scoped_refptr<RtpTransceiverInterface>> AddTransceiver(
      webrtc::MediaType media_type,
      const RtpTransceiverInit& init) override {
    return RTCError(RTCErrorType::UNSUPPORTED_OPERATION, "Not implemented");
  }

  scoped_refptr<RtpSenderInterface> CreateSender(
      const std::string& kind,
      const std::string& stream_id) override {
    return nullptr;
  }

  std::vector<scoped_refptr<RtpSenderInterface>> GetSenders() const override {
    return {};
  }

  std::vector<scoped_refptr<RtpReceiverInterface>> GetReceivers()
      const override {
    return {};
  }

  std::vector<scoped_refptr<RtpTransceiverInterface>> GetTransceivers()
      const override {
    return {};
  }

  bool GetStats(StatsObserver* observer,
                MediaStreamTrackInterface* track,
                StatsOutputLevel level) override {
    return false;
  }

  void GetStats(RTCStatsCollectorCallback* callback) override {}
  void GetStats(scoped_refptr<RtpSenderInterface> selector,
                scoped_refptr<RTCStatsCollectorCallback> callback) override {}
  void GetStats(scoped_refptr<RtpReceiverInterface> selector,
                scoped_refptr<RTCStatsCollectorCallback> callback) override {}

  void ClearStatsCache() override {}

  scoped_refptr<SctpTransportInterface> GetSctpTransport() const {
    return nullptr;
  }

  RTCErrorOr<scoped_refptr<DataChannelInterface>> CreateDataChannelOrError(
      const std::string& label,
      const DataChannelInit* config) override {
    return RTCError(RTCErrorType::UNSUPPORTED_OPERATION,
                    "Fake function called");
  }

  const SessionDescriptionInterface* local_description() const override {
    return nullptr;
  }
  const SessionDescriptionInterface* remote_description() const override {
    return nullptr;
  }

  const SessionDescriptionInterface* current_local_description()
      const override {
    return nullptr;
  }
  const SessionDescriptionInterface* current_remote_description()
      const override {
    return nullptr;
  }

  const SessionDescriptionInterface* pending_local_description()
      const override {
    return nullptr;
  }
  const SessionDescriptionInterface* pending_remote_description()
      const override {
    return nullptr;
  }

  void RestartIce() override {}

  void CreateOffer(CreateSessionDescriptionObserver* observer,
                   const RTCOfferAnswerOptions& options) override {}

  void CreateAnswer(CreateSessionDescriptionObserver* observer,
                    const RTCOfferAnswerOptions& options) override {}

  void SetLocalDescription(SetSessionDescriptionObserver* observer,
                           SessionDescriptionInterface* desc) override {}

  void SetRemoteDescription(SetSessionDescriptionObserver* observer,
                            SessionDescriptionInterface* desc) override {}

  void SetRemoteDescription(
      std::unique_ptr<SessionDescriptionInterface> desc,
      scoped_refptr<SetRemoteDescriptionObserverInterface> observer) override {}

  bool ShouldFireNegotiationNeededEvent(uint32_t event_id) { return true; }

  RTCConfiguration GetConfiguration() override { return RTCConfiguration(); }

  RTCError SetConfiguration(
      const PeerConnectionInterface::RTCConfiguration& config) override {
    return RTCError();
  }

  bool AddIceCandidate(const IceCandidateInterface* candidate) override {
    return false;
  }

  bool RemoveIceCandidates(const std::vector<Candidate>& candidates) override {
    return false;
  }

  RTCError SetBitrate(const BitrateSettings& bitrate) override {
    return RTCError(RTCErrorType::UNSUPPORTED_OPERATION, "Not implemented");
  }

  void ReconfigureBandwidthEstimation(
      const BandwidthEstimationSettings& settings) override {}

  void SetAudioPlayout(bool playout) override {}

  void SetAudioRecording(bool recording) override {}

  scoped_refptr<DtlsTransportInterface> LookupDtlsTransportByMid(
      const std::string& mid) {
    return nullptr;
  }

  SignalingState signaling_state() override { return SignalingState::kStable; }

  IceConnectionState ice_connection_state() override {
    return IceConnectionState::kIceConnectionNew;
  }

  IceConnectionState standardized_ice_connection_state() override {
    return IceConnectionState::kIceConnectionNew;
  }

  PeerConnectionState peer_connection_state() override {
    return PeerConnectionState::kNew;
  }

  IceGatheringState ice_gathering_state() override {
    return IceGatheringState::kIceGatheringNew;
  }

  std::optional<bool> can_trickle_ice_candidates() { return std::nullopt; }

  void AddAdaptationResource(scoped_refptr<Resource> resource) {}

  bool StartRtcEventLog(std::unique_ptr<RtcEventLogOutput> output,
                        int64_t output_period_ms) override {
    return false;
  }

  bool StartRtcEventLog(std::unique_ptr<RtcEventLogOutput> output) override {
    return false;
  }

  void SetDataChannelEventObserver(
      std::unique_ptr<DataChannelEventObserverInterface> observer) override {}

  void StopRtcEventLog() override {}

  void Close() override {}

  // PeerConnectionInternal implementation.

  Thread* network_thread() const override { return nullptr; }
  Thread* worker_thread() const override { return nullptr; }
  Thread* signaling_thread() const override { return nullptr; }

  std::string session_id() const override { return ""; }

  bool initial_offerer() const override { return false; }

  std::vector<scoped_refptr<RtpTransceiverProxyWithInternal<RtpTransceiver>>>
  GetTransceiversInternal() const override {
    return {};
  }

  std::optional<std::string> sctp_transport_name() const override {
    return std::nullopt;
  }

  std::optional<std::string> sctp_mid() const override { return std::nullopt; }

  std::map<std::string, TransportStats> GetTransportStatsByNames(
      const std::set<std::string>& transport_names) override {
    return {};
  }

  Call::Stats GetCallStats() override { return Call::Stats(); }

  std::optional<AudioDeviceModule::Stats> GetAudioDeviceStats() override {
    return std::nullopt;
  }

  bool GetLocalCertificate(
      const std::string& transport_name,
      scoped_refptr<RTCCertificate>* certificate) override {
    return false;
  }

  std::unique_ptr<SSLCertChain> GetRemoteSSLCertChain(
      const std::string& transport_name) override {
    return nullptr;
  }

  bool IceRestartPending(const std::string& content_name) const override {
    return false;
  }

  bool NeedsIceRestart(const std::string& content_name) const override {
    return false;
  }

  bool GetSslRole(const std::string& content_name, SSLRole* role) override {
    return false;
  }
  const PeerConnectionInterface::RTCConfiguration* configuration()
      const override {
    return nullptr;
  }

  void ReportSdpBundleUsage(
      const SessionDescriptionInterface& remote_description) override {}

  PeerConnectionMessageHandler* message_handler() override { return nullptr; }
  RtpTransmissionManager* rtp_manager() override { return nullptr; }
  const RtpTransmissionManager* rtp_manager() const override { return nullptr; }
  bool dtls_enabled() const override { return false; }
  const PeerConnectionFactoryInterface::Options* options() const override {
    return nullptr;
  }

  CryptoOptions GetCryptoOptions() override { return CryptoOptions(); }
  JsepTransportController* transport_controller_s() override { return nullptr; }
  JsepTransportController* transport_controller_n() override { return nullptr; }
  DataChannelController* data_channel_controller() override { return nullptr; }
  PortAllocator* port_allocator() override { return nullptr; }
  LegacyStatsCollector* legacy_stats() override { return nullptr; }
  PeerConnectionObserver* Observer() const override { return nullptr; }
  std::optional<SSLRole> GetSctpSslRole_n() override { return std::nullopt; }
  PeerConnectionInterface::IceConnectionState ice_connection_state_internal()
      override {
    return PeerConnectionInterface::IceConnectionState::kIceConnectionNew;
  }
  void SetIceConnectionState(
      PeerConnectionInterface::IceConnectionState new_state) override {}
  void NoteUsageEvent(UsageEvent event) override {}
  bool IsClosed() const override { return false; }
  bool IsUnifiedPlan() const override { return true; }
  bool ValidateBundleSettings(const SessionDescription* desc,
                              const std::map<std::string, const ContentGroup*>&
                                  bundle_groups_by_mid) override {
    return false;
  }

  RTCErrorOr<scoped_refptr<RtpTransceiverInterface>> AddTransceiver(
      webrtc::MediaType media_type,
      scoped_refptr<MediaStreamTrackInterface> track,
      const RtpTransceiverInit& init,
      bool fire_callback = true) override {
    return RTCError(RTCErrorType::INTERNAL_ERROR, "");
  }
  RTCError StartSctpTransport(const SctpOptions& options) override {
    return RTCError::OK();
  }

  void AddRemoteCandidate(absl::string_view mid,
                          const Candidate& candidate) override {}

  Call* call_ptr() override { return nullptr; }
  bool SrtpRequired() const override { return false; }
  bool CreateDataChannelTransport(absl::string_view mid) override {
    return false;
  }
  void DestroyDataChannelTransport(RTCError error) override {}

  const FieldTrialsView& trials() const override { return field_trials_; }

  NetworkControllerInterface* GetNetworkController() override {
    return nullptr;
  }

  PayloadTypePicker& payload_type_picker() override {
    return payload_type_picker_;
  }

  CandidateStatsList GetPooledCandidateStats() const override { return {}; }

 protected:
  test::ScopedKeyValueConfig field_trials_;
  PayloadTypePicker payload_type_picker_;
};

static_assert(
    !std::is_abstract_v<webrtc::RefCountedObject<FakePeerConnectionBase>>,
    "");

}  // namespace webrtc

#endif  // PC_TEST_FAKE_PEER_CONNECTION_BASE_H_
