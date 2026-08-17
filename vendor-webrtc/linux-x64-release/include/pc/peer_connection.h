/*
 *  Copyright 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_PEER_CONNECTION_H_
#define PC_PEER_CONNECTION_H_

#include <stdint.h>

#include <functional>
#include <map>
#include <memory>
#include <optional>
#include <set>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/adaptation/resource.h"
#include "api/async_dns_resolver.h"
#include "api/audio/audio_device.h"
#include "api/candidate.h"
#include "api/crypto/crypto_options.h"
#include "api/data_channel_event_observer_interface.h"
#include "api/data_channel_interface.h"
#include "api/dtls_transport_interface.h"
#include "api/environment/environment.h"
#include "api/field_trials_view.h"
#include "api/ice_transport_interface.h"
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
#include "api/sequence_checker.h"
#include "api/set_local_description_observer_interface.h"
#include "api/set_remote_description_observer_interface.h"
#include "api/stats/rtc_stats_collector_callback.h"
#include "api/task_queue/pending_task_safety_flag.h"
#include "api/transport/bandwidth_estimation_settings.h"
#include "api/transport/bitrate_settings.h"
#include "api/transport/data_channel_transport_interface.h"
#include "api/transport/enums.h"
#include "api/transport/network_control.h"
#include "api/turn_customizer.h"
#include "call/call.h"
#include "call/payload_type_picker.h"
#include "modules/rtp_rtcp/source/rtp_packet_received.h"
#include "p2p/base/ice_transport_internal.h"
#include "p2p/base/port.h"
#include "p2p/base/port_allocator.h"
#include "p2p/base/transport_description.h"
#include "pc/channel_interface.h"
#include "pc/codec_vendor.h"
#include "pc/connection_context.h"
#include "pc/data_channel_controller.h"
#include "pc/data_channel_utils.h"
#include "pc/dtls_transport.h"
#include "pc/jsep_transport_controller.h"
#include "pc/legacy_stats_collector.h"
#include "pc/peer_connection_internal.h"
#include "pc/peer_connection_message_handler.h"
#include "pc/rtc_stats_collector.h"
#include "pc/rtp_transceiver.h"
#include "pc/rtp_transmission_manager.h"
#include "pc/rtp_transport_internal.h"
#include "pc/sdp_offer_answer.h"
#include "pc/session_description.h"
#include "pc/transceiver_list.h"
#include "pc/transport_stats.h"
#include "pc/usage_pattern.h"
#include "rtc_base/checks.h"
#include "rtc_base/copy_on_write_buffer.h"
#include "rtc_base/rtc_certificate.h"
#include "rtc_base/ssl_certificate.h"
#include "rtc_base/ssl_stream_adapter.h"
#include "rtc_base/thread.h"
#include "rtc_base/thread_annotations.h"
#include "rtc_base/weak_ptr.h"

namespace webrtc {

// PeerConnection is the implementation of the PeerConnection object as defined
// by the PeerConnectionInterface API surface.
// The class currently is solely responsible for the following:
// - Managing the session state machine (signaling state).
// - Creating and initializing lower-level objects, like PortAllocator and
//   BaseChannels.
// - Owning and managing the life cycle of the RtpSender/RtpReceiver and track
//   objects.
// - Tracking the current and pending local/remote session descriptions.
// The class currently is jointly responsible for the following:
// - Parsing and interpreting SDP.
// - Generating offers and answers based on the current state.
// - The ICE state machine.
// - Generating stats.
class PeerConnection : public PeerConnectionInternal,
                       public JsepTransportController::Observer {
 public:
  // Creates a PeerConnection and initializes it with the given values.
  // If the initialization fails, the function releases the PeerConnection
  // and returns nullptr.
  //
  // Note that the function takes ownership of dependencies, and will
  // either use them or release them, whether it succeeds or fails.
  static scoped_refptr<PeerConnection> Create(
      const Environment& env,
      scoped_refptr<ConnectionContext> context,
      const PeerConnectionFactoryInterface::Options& options,
      std::unique_ptr<Call> call,
      const PeerConnectionInterface::RTCConfiguration& configuration,
      PeerConnectionDependencies& dependencies,
      const ServerAddresses& stun_servers,
      const std::vector<RelayServerConfig>& turn_servers);

  scoped_refptr<StreamCollectionInterface> local_streams() override;
  scoped_refptr<StreamCollectionInterface> remote_streams() override;
  bool AddStream(MediaStreamInterface* local_stream) override;
  void RemoveStream(MediaStreamInterface* local_stream) override;

  RTCErrorOr<scoped_refptr<RtpSenderInterface>> AddTrack(
      scoped_refptr<MediaStreamTrackInterface> track,
      const std::vector<std::string>& stream_ids) override;
  RTCErrorOr<scoped_refptr<RtpSenderInterface>> AddTrack(
      scoped_refptr<MediaStreamTrackInterface> track,
      const std::vector<std::string>& stream_ids,
      const std::vector<RtpEncodingParameters>& init_send_encodings) override;
  RTCErrorOr<scoped_refptr<RtpSenderInterface>> AddTrack(
      scoped_refptr<MediaStreamTrackInterface> track,
      const std::vector<std::string>& stream_ids,
      const std::vector<RtpEncodingParameters>* init_send_encodings);
  RTCError RemoveTrackOrError(
      scoped_refptr<RtpSenderInterface> sender) override;

  RTCErrorOr<scoped_refptr<RtpTransceiverInterface>> AddTransceiver(
      scoped_refptr<MediaStreamTrackInterface> track) override;
  RTCErrorOr<scoped_refptr<RtpTransceiverInterface>> AddTransceiver(
      scoped_refptr<MediaStreamTrackInterface> track,
      const RtpTransceiverInit& init) override;
  RTCErrorOr<scoped_refptr<RtpTransceiverInterface>> AddTransceiver(
      webrtc::MediaType media_type) override;
  RTCErrorOr<scoped_refptr<RtpTransceiverInterface>> AddTransceiver(
      webrtc::MediaType media_type,
      const RtpTransceiverInit& init) override;

  scoped_refptr<RtpSenderInterface> CreateSender(
      const std::string& kind,
      const std::string& stream_id) override;

  std::vector<scoped_refptr<RtpSenderInterface>> GetSenders() const override;
  std::vector<scoped_refptr<RtpReceiverInterface>> GetReceivers()
      const override;
  std::vector<scoped_refptr<RtpTransceiverInterface>> GetTransceivers()
      const override;

  RTCErrorOr<scoped_refptr<DataChannelInterface>> CreateDataChannelOrError(
      const std::string& label,
      const DataChannelInit* config) override;
  // WARNING: LEGACY. See peerconnectioninterface.h
  bool GetStats(StatsObserver* observer,
                MediaStreamTrackInterface* track,
                StatsOutputLevel level) override;
  // Spec-complaint GetStats(). See peerconnectioninterface.h
  void GetStats(RTCStatsCollectorCallback* callback) override;
  void GetStats(scoped_refptr<RtpSenderInterface> selector,
                scoped_refptr<RTCStatsCollectorCallback> callback) override;
  void GetStats(scoped_refptr<RtpReceiverInterface> selector,
                scoped_refptr<RTCStatsCollectorCallback> callback) override;
  void ClearStatsCache() override;

  SignalingState signaling_state() override;

  IceConnectionState ice_connection_state() override;
  IceConnectionState ice_connection_state_internal() override {
    return ice_connection_state();
  }
  IceConnectionState standardized_ice_connection_state() override;
  PeerConnectionState peer_connection_state() override;
  IceGatheringState ice_gathering_state() override;
  std::optional<bool> can_trickle_ice_candidates() override;

  const SessionDescriptionInterface* local_description() const override;
  const SessionDescriptionInterface* remote_description() const override;
  const SessionDescriptionInterface* current_local_description() const override;
  const SessionDescriptionInterface* current_remote_description()
      const override;
  const SessionDescriptionInterface* pending_local_description() const override;
  const SessionDescriptionInterface* pending_remote_description()
      const override;

  void RestartIce() override;

  // JSEP01
  void CreateOffer(CreateSessionDescriptionObserver* observer,
                   const RTCOfferAnswerOptions& options) override;
  void CreateAnswer(CreateSessionDescriptionObserver* observer,
                    const RTCOfferAnswerOptions& options) override;

  void SetLocalDescription(
      std::unique_ptr<SessionDescriptionInterface> desc,
      scoped_refptr<SetLocalDescriptionObserverInterface> observer) override;
  void SetLocalDescription(
      scoped_refptr<SetLocalDescriptionObserverInterface> observer) override;
  // TODO(https://crbug.com/webrtc/11798): Delete these methods in favor of the
  // ones taking SetLocalDescriptionObserverInterface as argument.
  void SetLocalDescription(SetSessionDescriptionObserver* observer,
                           SessionDescriptionInterface* desc) override;
  void SetLocalDescription(SetSessionDescriptionObserver* observer) override;

  void SetRemoteDescription(
      std::unique_ptr<SessionDescriptionInterface> desc,
      scoped_refptr<SetRemoteDescriptionObserverInterface> observer) override;
  // TODO(https://crbug.com/webrtc/11798): Delete this methods in favor of the
  // ones taking SetRemoteDescriptionObserverInterface as argument.
  void SetRemoteDescription(SetSessionDescriptionObserver* observer,
                            SessionDescriptionInterface* desc) override;

  PeerConnectionInterface::RTCConfiguration GetConfiguration() override;
  RTCError SetConfiguration(
      const PeerConnectionInterface::RTCConfiguration& configuration) override;
  bool AddIceCandidate(const IceCandidateInterface* candidate) override;
  void AddIceCandidate(std::unique_ptr<IceCandidateInterface> candidate,
                       std::function<void(RTCError)> callback) override;
  bool RemoveIceCandidates(const std::vector<Candidate>& candidates) override;

  RTCError SetBitrate(const BitrateSettings& bitrate) override;
  void ReconfigureBandwidthEstimation(
      const BandwidthEstimationSettings& settings) override;

  void SetAudioPlayout(bool playout) override;
  void SetAudioRecording(bool recording) override;

  scoped_refptr<DtlsTransportInterface> LookupDtlsTransportByMid(
      const std::string& mid) override;
  scoped_refptr<DtlsTransport> LookupDtlsTransportByMidInternal(
      const std::string& mid);

  scoped_refptr<SctpTransportInterface> GetSctpTransport() const override;

  void AddAdaptationResource(scoped_refptr<Resource> resource) override;

  bool StartRtcEventLog(std::unique_ptr<RtcEventLogOutput> output,
                        int64_t output_period_ms) override;
  bool StartRtcEventLog(std::unique_ptr<RtcEventLogOutput> output) override;
  void StopRtcEventLog() override;

  void SetDataChannelEventObserver(
      std::unique_ptr<DataChannelEventObserverInterface> observer) override;

  void Close() override;

  Thread* signaling_thread() const final {
    return context_->signaling_thread();
  }

  Thread* network_thread() const final { return context_->network_thread(); }
  Thread* worker_thread() const final { return context_->worker_thread(); }

  std::string session_id() const override { return session_id_; }

  bool initial_offerer() const override {
    RTC_DCHECK_RUN_ON(signaling_thread());
    return sdp_handler_->initial_offerer();
  }

  std::vector<scoped_refptr<RtpTransceiverProxyWithInternal<RtpTransceiver>>>
  GetTransceiversInternal() const override {
    RTC_DCHECK_RUN_ON(signaling_thread());
    if (!ConfiguredForMedia()) {
      return {};
    }
    return rtp_manager()->transceivers()->List();
  }

  std::vector<DataChannelStats> GetDataChannelStats() const override;

  std::optional<std::string> sctp_transport_name() const override;
  std::optional<std::string> sctp_mid() const override;

  CandidateStatsList GetPooledCandidateStats() const override;
  std::map<std::string, TransportStats> GetTransportStatsByNames(
      const std::set<std::string>& transport_names) override;
  Call::Stats GetCallStats() override;

  std::optional<AudioDeviceModule::Stats> GetAudioDeviceStats() override;

  bool GetLocalCertificate(const std::string& transport_name,
                           scoped_refptr<RTCCertificate>* certificate) override;
  std::unique_ptr<SSLCertChain> GetRemoteSSLCertChain(
      const std::string& transport_name) override;
  bool IceRestartPending(const std::string& content_name) const override;
  bool NeedsIceRestart(const std::string& content_name) const override;
  bool GetSslRole(const std::string& content_name, SSLRole* role) override;

  // Functions needed by DataChannelController
  void NoteDataAddedEvent() override { NoteUsageEvent(UsageEvent::DATA_ADDED); }
  // Returns the observer. Will crash on CHECK if the observer is removed.
  PeerConnectionObserver* Observer() const override;
  bool IsClosed() const override {
    RTC_DCHECK_RUN_ON(signaling_thread());
    return !sdp_handler_ ||
           sdp_handler_->signaling_state() == PeerConnectionInterface::kClosed;
  }
  // Get current SSL role used by SCTP's underlying transport.
  std::optional<SSLRole> GetSctpSslRole_n() override;

  void OnSctpDataChannelStateChanged(
      int channel_id,
      DataChannelInterface::DataState state) override;

  bool ShouldFireNegotiationNeededEvent(uint32_t event_id) override;

  // Functions needed by SdpOfferAnswerHandler
  LegacyStatsCollector* legacy_stats() override {
    RTC_DCHECK_RUN_ON(signaling_thread());
    return legacy_stats_.get();
  }
  DataChannelController* data_channel_controller() override {
    RTC_DCHECK_RUN_ON(signaling_thread());
    return &data_channel_controller_;
  }
  bool dtls_enabled() const override {
    RTC_DCHECK_RUN_ON(signaling_thread());
    return dtls_enabled_;
  }
  const PeerConnectionInterface::RTCConfiguration* configuration()
      const override {
    RTC_DCHECK_RUN_ON(signaling_thread());
    return &configuration_;
  }
  PeerConnectionMessageHandler* message_handler() override {
    RTC_DCHECK_RUN_ON(signaling_thread());
    return &message_handler_;
  }

  RtpTransmissionManager* rtp_manager() override { return rtp_manager_.get(); }
  const RtpTransmissionManager* rtp_manager() const override {
    return rtp_manager_.get();
  }

  JsepTransportController* transport_controller_s() override {
    RTC_DCHECK_RUN_ON(signaling_thread());
    return transport_controller_copy_;
  }
  JsepTransportController* transport_controller_n() override {
    RTC_DCHECK_RUN_ON(network_thread());
    return transport_controller_.get();
  }
  PortAllocator* port_allocator() override { return port_allocator_.get(); }
  Call* call_ptr() override { return call_ptr_; }

  ConnectionContext* context() { return context_.get(); }
  const PeerConnectionFactoryInterface::Options* options() const override {
    return &options_;
  }
  void SetIceConnectionState(IceConnectionState new_state) override;
  void NoteUsageEvent(UsageEvent event) override;

  // Asynchronously adds a remote candidate on the network thread.
  void AddRemoteCandidate(absl::string_view mid,
                          const Candidate& candidate) override;

  // Report the UMA metric BundleUsage for the given remote description.
  void ReportSdpBundleUsage(
      const SessionDescriptionInterface& remote_description) override;

  // Report several UMA metrics on establishing the connection.
  void ReportFirstConnectUsageMetrics() RTC_RUN_ON(signaling_thread());
  // Report several UMA metrics for established connections when the connection
  // is closed.
  void ReportCloseUsageMetrics() RTC_RUN_ON(signaling_thread());

  // Returns true if the PeerConnection is configured to use Unified Plan
  // semantics for creating offers/answers and setting local/remote
  // descriptions. If this is true the RtpTransceiver API will also be available
  // to the user. If this is false, Plan B semantics are assumed.
  // TODO(bugs.webrtc.org/8530): Flip the default to be Unified Plan once
  // sufficient time has passed.
  bool IsUnifiedPlan() const override {
    return is_unified_plan_;
  }
  bool ValidateBundleSettings(const SessionDescription* desc,
                              const std::map<std::string, const ContentGroup*>&
                                  bundle_groups_by_mid) override;

  bool CreateDataChannelTransport(absl::string_view mid) override;
  void DestroyDataChannelTransport(RTCError error) override;

  // Asynchronously calls SctpTransport::Start() on the network thread for
  // `sctp_mid()` if set. Called as part of setting the local description.
  RTCError StartSctpTransport(const SctpOptions& options) override;

  // Returns the CryptoOptions for this PeerConnection. This will always
  // return the RTCConfiguration.crypto_options if set and will only default
  // back to the PeerConnectionFactory settings if nothing was set.
  CryptoOptions GetCryptoOptions() override;

  // Internal implementation for AddTransceiver family of methods. If
  // `fire_callback` is set, fires OnRenegotiationNeeded callback if successful.
  RTCErrorOr<scoped_refptr<RtpTransceiverInterface>> AddTransceiver(
      webrtc::MediaType media_type,
      scoped_refptr<MediaStreamTrackInterface> track,
      const RtpTransceiverInit& init,
      bool fire_callback = true) override;

  // Returns true if SRTP (either using DTLS-SRTP or SDES) is required by
  // this session.
  bool SrtpRequired() const override;

  std::optional<std::string> SetupDataChannelTransport_n(absl::string_view mid)
      RTC_RUN_ON(network_thread());
  void TeardownDataChannelTransport_n(RTCError error)
      RTC_RUN_ON(network_thread());

  const FieldTrialsView& trials() const override { return env_.field_trials(); }

  bool ConfiguredForMedia() const;

  // Functions made public for testing.
  void ReturnHistogramVeryQuicklyForTesting() {
    RTC_DCHECK_RUN_ON(signaling_thread());
    return_histogram_very_quickly_ = true;
  }
  void RequestUsagePatternReportForTesting();
  int FeedbackAccordingToRfc8888CountForTesting() const;
  int FeedbackAccordingToTransportCcCountForTesting() const;

  NetworkControllerInterface* GetNetworkController() override {
    if (!worker_thread()->IsCurrent()) {
      return worker_thread()->BlockingCall(
          [this]() { return GetNetworkController(); });
    }
    RTC_DCHECK_RUN_ON(worker_thread());
    RTC_DCHECK(call_);
    return call_->GetTransportControllerSend()->GetNetworkController();
  }
  PayloadTypePicker& payload_type_picker() override {
    return payload_type_picker_;
  }
  void DisableSdpMungingChecksForTesting() {
    if (!signaling_thread()->IsCurrent()) {
      signaling_thread()->BlockingCall(
          [&]() { DisableSdpMungingChecksForTesting(); });
      return;
    }
    RTC_DCHECK_RUN_ON(signaling_thread());
    sdp_handler_->DisableSdpMungingChecksForTesting();
  }

 protected:
  // Available for webrtc::scoped_refptr creation
  PeerConnection(const PeerConnectionInterface::RTCConfiguration& configuration,
                 const Environment& env,
                 scoped_refptr<ConnectionContext> context,
                 const PeerConnectionFactoryInterface::Options& options,
                 bool is_unified_plan,
                 std::unique_ptr<Call> call,
                 PeerConnectionDependencies& dependencies,
                 const ServerAddresses& stun_servers,
                 const std::vector<RelayServerConfig>& turn_servers,
                 bool dtls_enabled);

  ~PeerConnection() override;

 private:
  // Called from the constructor to apply the server configuration on the
  // network thread and initialize network thread related state (see
  // InitializeTransportController_n). The return value of this function is used
  // to set the initial value of `transport_controller_copy_`.
  JsepTransportController* InitializeNetworkThread(
      const ServerAddresses& stun_servers,
      const std::vector<RelayServerConfig>& turn_servers);
  JsepTransportController* InitializeTransportController_n(
      const RTCConfiguration& configuration) RTC_RUN_ON(network_thread());

  scoped_refptr<RtpTransceiverProxyWithInternal<RtpTransceiver>>
  FindTransceiverBySender(scoped_refptr<RtpSenderInterface> sender)
      RTC_RUN_ON(signaling_thread());

  void SetStandardizedIceConnectionState(
      PeerConnectionInterface::IceConnectionState new_state)
      RTC_RUN_ON(signaling_thread());
  void SetConnectionState(
      PeerConnectionInterface::PeerConnectionState new_state)
      RTC_RUN_ON(signaling_thread());

  // Called any time the IceGatheringState changes.
  void OnIceGatheringChange(IceGatheringState new_state)
      RTC_RUN_ON(signaling_thread());
  // New ICE candidate has been gathered.
  void OnIceCandidate(std::unique_ptr<IceCandidateInterface> candidate)
      RTC_RUN_ON(signaling_thread());
  // Gathering of an ICE candidate failed.
  void OnIceCandidateError(const std::string& address,
                           int port,
                           const std::string& url,
                           int error_code,
                           const std::string& error_text)
      RTC_RUN_ON(signaling_thread());
  // Some local ICE candidates have been removed.
  void OnIceCandidatesRemoved(const std::vector<Candidate>& candidates)
      RTC_RUN_ON(signaling_thread());

  void OnSelectedCandidatePairChanged(const CandidatePairChangeEvent& event)
      RTC_RUN_ON(signaling_thread());

  void OnNegotiationNeeded();

  // Called when first configuring the port allocator.
  struct InitializePortAllocatorResult {
    bool enable_ipv6;
  };
  InitializePortAllocatorResult InitializePortAllocator_n(
      const ServerAddresses& stun_servers,
      const std::vector<RelayServerConfig>& turn_servers,
      const RTCConfiguration& configuration);
  // Called when SetConfiguration is called to apply the supported subset
  // of the configuration on the network thread.
  bool ReconfigurePortAllocator_n(
      const ServerAddresses& stun_servers,
      const std::vector<RelayServerConfig>& turn_servers,
      IceTransportsType type,
      int candidate_pool_size,
      PortPrunePolicy turn_port_prune_policy,
      TurnCustomizer* turn_customizer,
      std::optional<int> stun_candidate_keepalive_interval,
      bool have_local_description);

  // Starts output of an RTC event log to the given output object.
  // This function should only be called from the worker thread.
  bool StartRtcEventLog_w(std::unique_ptr<RtcEventLogOutput> output,
                          int64_t output_period_ms);

  // Stops recording an RTC event log.
  // This function should only be called from the worker thread.
  void StopRtcEventLog_w();

  // Returns true and the TransportInfo of the given `content_name`
  // from `description`. Returns false if it's not available.
  static bool GetTransportDescription(const SessionDescription* description,
                                      const std::string& content_name,
                                      TransportDescription* info);

  // Returns the media index for a local ice candidate given the content name.
  // Returns false if the local session description does not have a media
  // content called  `content_name`.
  bool GetLocalCandidateMediaIndex(const std::string& content_name,
                                   int* sdp_mline_index)
      RTC_RUN_ON(signaling_thread());

  // JsepTransportController signal handlers.
  void OnTransportControllerConnectionState(::webrtc::IceConnectionState state)
      RTC_RUN_ON(signaling_thread());
  void OnTransportControllerGatheringState(::webrtc::IceGatheringState state)
      RTC_RUN_ON(signaling_thread());
  void OnTransportControllerCandidatesGathered(
      const std::string& transport_name,
      const std::vector<Candidate>& candidates) RTC_RUN_ON(signaling_thread());
  void OnTransportControllerCandidateError(const IceCandidateErrorEvent& event)
      RTC_RUN_ON(signaling_thread());
  void OnTransportControllerCandidatesRemoved(
      const std::vector<Candidate>& candidates) RTC_RUN_ON(signaling_thread());
  void OnTransportControllerCandidateChanged(
      const CandidatePairChangeEvent& event) RTC_RUN_ON(signaling_thread());
  void OnTransportControllerDtlsHandshakeError(SSLHandshakeError error);

  // Invoked when TransportController connection completion is signaled.
  // Reports stats for all transports in use.
  void ReportTransportStats(std::vector<RtpTransceiverProxyRefPtr> transceivers)
      RTC_RUN_ON(network_thread());

  // Gather the usage of IPv4/IPv6 as best connection.
  static void ReportBestConnectionState(const TransportStats& stats);

  static void ReportNegotiatedCiphers(
      bool dtls_enabled,
      const TransportStats& stats,
      const std::set<webrtc::MediaType>& media_types);
  void ReportIceCandidateCollected(const Candidate& candidate)
      RTC_RUN_ON(signaling_thread());

  void ReportUsagePattern() const RTC_RUN_ON(signaling_thread());

  void ReportRemoteIceCandidateAdded(const Candidate& candidate);

  // JsepTransportController::Observer override.
  //
  // Called by `transport_controller_` when processing transport information
  // from a session description, and the mapping from m= sections to transports
  // changed (as a result of BUNDLE negotiation, or m= sections being
  // rejected).
  bool OnTransportChanged(
      const std::string& mid,
      RtpTransportInternal* rtp_transport,
      scoped_refptr<DtlsTransport> dtls_transport,
      DataChannelTransportInterface* data_channel_transport) override;

  void SetSctpTransportName(std::string sctp_transport_name);

  std::function<void(const webrtc::CopyOnWriteBuffer& packet,
                     int64_t packet_time_us)>
  InitializeRtcpCallback();

  std::function<void(const RtpPacketReceived& parsed_packet)>
  InitializeUnDemuxablePacketHandler();

  bool CanAttemptDtlsStunPiggybacking(const RTCConfiguration& configuration);

  const Environment env_;
  const scoped_refptr<ConnectionContext> context_;
  const PeerConnectionFactoryInterface::Options options_;
  PeerConnectionObserver* observer_ RTC_GUARDED_BY(signaling_thread()) =
      nullptr;

  const bool is_unified_plan_;
  const bool dtls_enabled_;
  bool return_histogram_very_quickly_ RTC_GUARDED_BY(signaling_thread()) =
      false;
  // Did the connectionState ever change to `connected`?
  // Used to gather metrics only the first such state change.
  bool was_ever_connected_ RTC_GUARDED_BY(signaling_thread()) = false;

  IceConnectionState ice_connection_state_ RTC_GUARDED_BY(signaling_thread()) =
      kIceConnectionNew;
  PeerConnectionInterface::IceConnectionState standardized_ice_connection_state_
      RTC_GUARDED_BY(signaling_thread()) = kIceConnectionNew;
  PeerConnectionInterface::PeerConnectionState connection_state_
      RTC_GUARDED_BY(signaling_thread()) = PeerConnectionState::kNew;

  IceGatheringState ice_gathering_state_ RTC_GUARDED_BY(signaling_thread()) =
      kIceGatheringNew;
  PeerConnectionInterface::RTCConfiguration configuration_
      RTC_GUARDED_BY(signaling_thread());

  const std::unique_ptr<AsyncDnsResolverFactoryInterface>
      async_dns_resolver_factory_;
  std::unique_ptr<PortAllocator>
      port_allocator_;  // TODO(bugs.webrtc.org/9987): Accessed on both
                        // signaling and network thread.
  const std::unique_ptr<IceTransportFactory>
      ice_transport_factory_;  // TODO(bugs.webrtc.org/9987): Accessed on the
                               // signaling thread but the underlying raw
                               // pointer is given to
                               // `jsep_transport_controller_` and used on the
                               // network thread.
  const std::unique_ptr<SSLCertificateVerifier> tls_cert_verifier_
      RTC_GUARDED_BY(network_thread());

  // The unique_ptr belongs to the worker thread, but the Call object manages
  // its own thread safety.
  std::unique_ptr<Call> call_ RTC_GUARDED_BY(worker_thread());
  ScopedTaskSafety signaling_thread_safety_;
  scoped_refptr<PendingTaskSafetyFlag> network_thread_safety_;
  scoped_refptr<PendingTaskSafetyFlag> worker_thread_safety_;

  // Points to the same thing as `call_`. Since it's const, we may read the
  // pointer from any thread.
  // TODO(bugs.webrtc.org/11992): Remove this workaround (and potential dangling
  // pointer).
  Call* const call_ptr_;

  std::unique_ptr<LegacyStatsCollector> legacy_stats_
      RTC_GUARDED_BY(signaling_thread());  // A pointer is passed to senders_
  scoped_refptr<RTCStatsCollector> stats_collector_
      RTC_GUARDED_BY(signaling_thread());

  const std::string session_id_;

  // `sctp_mid_` is the content name (MID) in SDP.
  // Note: this is used as the data channel MID by both SCTP and data channel
  // transports.  It is set when either transport is initialized and unset when
  // both transports are deleted.
  // There is one copy on the signaling thread and another copy on the
  // networking thread. Changes are always initiated from the signaling
  // thread, but applied first on the networking thread via an invoke().
  std::optional<std::string> sctp_mid_s_ RTC_GUARDED_BY(signaling_thread());
  std::optional<std::string> sctp_mid_n_ RTC_GUARDED_BY(network_thread());
  std::string sctp_transport_name_s_ RTC_GUARDED_BY(signaling_thread());

  UsagePattern usage_pattern_ RTC_GUARDED_BY(signaling_thread());

  // The DataChannelController is accessed from both the signaling thread
  // and networking thread. It is a thread-aware object.
  DataChannelController data_channel_controller_;

  // Machinery for handling messages posted to oneself
  PeerConnectionMessageHandler message_handler_
      RTC_GUARDED_BY(signaling_thread());

  PayloadTypePicker payload_type_picker_;

  // The transport controller is set and used on the network thread.
  // Some functions pass the value of the transport_controller_ pointer
  // around as arguments while running on the signaling thread; these
  // use the transport_controller_copy.
  std::unique_ptr<JsepTransportController> transport_controller_
      RTC_GUARDED_BY(network_thread());
  JsepTransportController* transport_controller_copy_
      RTC_GUARDED_BY(signaling_thread()) = nullptr;

  // The machinery for handling offers and answers. Const after initialization.
  std::unique_ptr<SdpOfferAnswerHandler> sdp_handler_
      RTC_GUARDED_BY(signaling_thread()) RTC_PT_GUARDED_BY(signaling_thread());

  // Administration of senders, receivers and transceivers
  // Accessed on both signaling and network thread. Const after Initialize().
  std::unique_ptr<RtpTransmissionManager> rtp_manager_;

  std::unique_ptr<CodecLookupHelper> codec_lookup_helper_;

  // This variable needs to be the last one in the class.
  WeakPtrFactory<PeerConnection> weak_factory_;
};

}  // namespace webrtc

#endif  // PC_PEER_CONNECTION_H_
