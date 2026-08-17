/*
 *  Copyright 2017 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_JSEP_TRANSPORT_CONTROLLER_H_
#define PC_JSEP_TRANSPORT_CONTROLLER_H_

#include <stdint.h>

#include <functional>
#include <map>
#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "absl/functional/any_invocable.h"
#include "absl/strings/string_view.h"
#include "api/async_dns_resolver.h"
#include "api/candidate.h"
#include "api/crypto/crypto_options.h"
#include "api/environment/environment.h"
#include "api/ice_transport_interface.h"
#include "api/jsep.h"
#include "api/peer_connection_interface.h"
#include "api/rtc_error.h"
#include "api/rtc_event_log/rtc_event_log.h"
#include "api/scoped_refptr.h"
#include "api/sequence_checker.h"
#include "api/transport/data_channel_transport_interface.h"
#include "api/transport/sctp_transport_factory_interface.h"
#include "call/payload_type.h"
#include "call/payload_type_picker.h"
#include "media/base/codec.h"
#include "modules/rtp_rtcp/source/rtp_packet_received.h"
#include "p2p/base/ice_transport_internal.h"
#include "p2p/base/p2p_transport_channel.h"
#include "p2p/base/packet_transport_internal.h"
#include "p2p/base/port.h"
#include "p2p/base/port_allocator.h"
#include "p2p/base/transport_description.h"
#include "p2p/base/transport_info.h"
#include "p2p/dtls/dtls_transport_factory.h"
#include "p2p/dtls/dtls_transport_internal.h"
#include "pc/dtls_srtp_transport.h"
#include "pc/dtls_transport.h"
#include "pc/jsep_transport.h"
#include "pc/jsep_transport_collection.h"
#include "pc/rtp_transport.h"
#include "pc/rtp_transport_internal.h"
#include "pc/sctp_transport.h"
#include "pc/session_description.h"
#include "pc/srtp_transport.h"
#include "pc/transport_stats.h"
#include "rtc_base/callback_list.h"
#include "rtc_base/copy_on_write_buffer.h"
#include "rtc_base/rtc_certificate.h"
#include "rtc_base/ssl_certificate.h"
#include "rtc_base/ssl_stream_adapter.h"
#include "rtc_base/third_party/sigslot/sigslot.h"
#include "rtc_base/thread.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {

class JsepTransportController : public PayloadTypeSuggester,
                                public sigslot::has_slots<> {
 public:
  // Used when the RtpTransport/DtlsTransport of the m= section is changed
  // because the section is rejected or BUNDLE is enabled.
  class Observer {
   public:
    virtual ~Observer() {}

    // Returns true if media associated with `mid` was successfully set up to be
    // demultiplexed on `rtp_transport`. Could return false if two bundled m=
    // sections use the same SSRC, for example.
    //
    // If a data channel transport must be negotiated, `data_channel_transport`
    // and `negotiation_state` indicate negotiation status.  If
    // `data_channel_transport` is null, the data channel transport should not
    // be used.  Otherwise, the value is a pointer to the transport to be used
    // for data channels on `mid`, if any.
    //
    // The observer should not send data on `data_channel_transport` until
    // `negotiation_state` is provisional or final.  It should not delete
    // `data_channel_transport` or any fallback transport until
    // `negotiation_state` is final.
    virtual bool OnTransportChanged(
        const std::string& mid,
        RtpTransportInternal* rtp_transport,
        scoped_refptr<DtlsTransport> dtls_transport,
        DataChannelTransportInterface* data_channel_transport) = 0;
  };

  struct Config {
    // If `redetermine_role_on_ice_restart` is true, ICE role is redetermined
    // upon setting a local transport description that indicates an ICE
    // restart.
    bool redetermine_role_on_ice_restart = true;
    SSLProtocolVersion ssl_max_version = SSL_PROTOCOL_DTLS_12;
    // `crypto_options` is used to determine if created DTLS transports
    // negotiate GCM crypto suites or not.
    CryptoOptions crypto_options;
    PeerConnectionInterface::BundlePolicy bundle_policy =
        PeerConnectionInterface::kBundlePolicyBalanced;
    PeerConnectionInterface::RtcpMuxPolicy rtcp_mux_policy =
        PeerConnectionInterface::kRtcpMuxPolicyRequire;
    bool disable_encryption = false;
    bool enable_external_auth = false;
    // Used to inject the ICE/DTLS transports created externally.
    IceTransportFactory* ice_transport_factory = nullptr;
    DtlsTransportFactory* dtls_transport_factory = nullptr;
    Observer* transport_observer = nullptr;
    // Must be provided and valid for the lifetime of the
    // JsepTransportController instance.
    absl::AnyInvocable<void(const webrtc::CopyOnWriteBuffer& packet,
                            int64_t packet_time_us) const>
        rtcp_handler;
    absl::AnyInvocable<void(const RtpPacketReceived& parsed_packet) const>
        un_demuxable_packet_handler;
    // Initial value for whether DtlsTransport reset causes a reset
    // of SRTP parameters.
    bool active_reset_srtp_params = false;
    RtcEventLog* event_log = nullptr;

    // Factory for SCTP transports.
    SctpTransportFactoryInterface* sctp_factory = nullptr;
    std::function<void(webrtc::SSLHandshakeError)> on_dtls_handshake_error_;
  };

  // The ICE related events are fired on the `network_thread`.
  // All the transport related methods are called on the `network_thread`
  // and destruction of the JsepTransportController must occur on the
  // `network_thread`.
  JsepTransportController(
      const Environment& env,
      Thread* network_thread,
      PortAllocator* port_allocator,
      AsyncDnsResolverFactoryInterface* async_dns_resolver_factory,
      PayloadTypePicker& payload_type_picker,
      Config config);
  virtual ~JsepTransportController();

  JsepTransportController(const JsepTransportController&) = delete;
  JsepTransportController& operator=(const JsepTransportController&) = delete;

  // The main method to be called; applies a description at the transport
  // level, creating/destroying transport objects as needed and updating their
  // properties. This includes RTP, DTLS, and ICE (but not SCTP). At least not
  // yet? May make sense to in the future.
  //
  // `local_desc` must always be valid. If a remote description has previously
  // been set via a call to `SetRemoteDescription()` then `remote_desc` should
  // point to that description object in order to keep the current local and
  // remote session descriptions in sync.
  RTCError SetLocalDescription(SdpType type,
                               const SessionDescription* local_desc,
                               const SessionDescription* remote_desc);

  // Call to apply a remote description (See `SetLocalDescription()` for local).
  //
  // `remote_desc` must always be valid. If a local description has previously
  // been set via a call to `SetLocalDescription()` then `local_desc` should
  // point to that description object in order to keep the current local and
  // remote session descriptions in sync.
  RTCError SetRemoteDescription(SdpType type,
                                const SessionDescription* local_desc,
                                const SessionDescription* remote_desc);

  // Get transports to be used for the provided `mid`. If bundling is enabled,
  // calling GetRtpTransport for multiple MIDs may yield the same object.
  RtpTransportInternal* GetRtpTransport(absl::string_view mid) const;
  DtlsTransportInternal* GetDtlsTransport(const std::string& mid);
  const DtlsTransportInternal* GetRtcpDtlsTransport(
      const std::string& mid) const;
  // Gets the externally sharable version of the DtlsTransport.
  scoped_refptr<DtlsTransport> LookupDtlsTransportByMid(const std::string& mid);
  scoped_refptr<SctpTransport> GetSctpTransport(const std::string& mid) const;

  DataChannelTransportInterface* GetDataChannelTransport(
      const std::string& mid) const;

  /*********************
   * ICE-related methods
   ********************/
  // This method is public to allow PeerConnection to update it from
  // SetConfiguration.
  void SetIceConfig(const IceConfig& config);
  // Set the "needs-ice-restart" flag as described in JSEP. After the flag is
  // set, offers should generate new ufrags/passwords until an ICE restart
  // occurs.
  void SetNeedsIceRestartFlag();
  // Returns true if the ICE restart flag above was set, and no ICE restart has
  // occurred yet for this transport (by applying a local description with
  // changed ufrag/password). If the transport has been deleted as a result of
  // bundling, returns false.
  bool NeedsIceRestart(const std::string& mid) const;
  // Start gathering candidates for any new transports, or transports doing an
  // ICE restart.
  void MaybeStartGathering();
  RTCError AddRemoteCandidates(const std::string& mid,
                               const std::vector<Candidate>& candidates);
  RTCError RemoveRemoteCandidates(const std::vector<Candidate>& candidates);

  /**********************
   * DTLS-related methods
   *********************/
  // Specifies the identity to use in this session.
  // Can only be called once.
  bool SetLocalCertificate(const scoped_refptr<RTCCertificate>& certificate);
  scoped_refptr<RTCCertificate> GetLocalCertificate(
      const std::string& mid) const;
  // Caller owns returned certificate chain. This method mainly exists for
  // stats reporting.
  std::unique_ptr<SSLCertChain> GetRemoteSSLCertChain(
      const std::string& mid) const;
  // Get negotiated role, if one has been negotiated.
  std::optional<SSLRole> GetDtlsRole(const std::string& mid) const;

  // Suggest a payload type for a given codec on a given media section.
  // Media section is indicated by MID.
  // The function will either return a PT already in use on the connection
  // or a newly suggested one.
  RTCErrorOr<PayloadType> SuggestPayloadType(const std::string& mid,
                                             Codec codec) override;
  RTCError AddLocalMapping(const std::string& mid,
                           PayloadType payload_type,
                           const Codec& codec) override;
  const PayloadTypePicker& PayloadTypePickerForTesting() const {
    return payload_type_picker_;
  }

  bool GetStats(const std::string& mid, TransportStats* stats) const;

  bool initial_offerer() const { return initial_offerer_ && *initial_offerer_; }

  void SetActiveResetSrtpParams(bool active_reset_srtp_params);

  RTCError RollbackTransports();

  // F: void(const std::string&, const std::vector<webrtc::Candidate>&)
  template <typename F>
  void SubscribeIceCandidateGathered(F&& callback) {
    RTC_DCHECK_RUN_ON(network_thread_);
    signal_ice_candidates_gathered_.AddReceiver(std::forward<F>(callback));
  }

  // F: void(webrtc::IceConnectionState)
  template <typename F>
  void SubscribeIceConnectionState(F&& callback) {
    RTC_DCHECK_RUN_ON(network_thread_);
    signal_ice_connection_state_.AddReceiver(std::forward<F>(callback));
  }

  // F: void(PeerConnectionInterface::PeerConnectionState)
  template <typename F>
  void SubscribeConnectionState(F&& callback) {
    RTC_DCHECK_RUN_ON(network_thread_);
    signal_connection_state_.AddReceiver(std::forward<F>(callback));
  }

  // F: void(PeerConnectionInterface::IceConnectionState)
  template <typename F>
  void SubscribeStandardizedIceConnectionState(F&& callback) {
    RTC_DCHECK_RUN_ON(network_thread_);
    signal_standardized_ice_connection_state_.AddReceiver(
        std::forward<F>(callback));
  }

  // F: void(webrtc::IceGatheringState)
  template <typename F>
  void SubscribeIceGatheringState(F&& callback) {
    RTC_DCHECK_RUN_ON(network_thread_);
    signal_ice_gathering_state_.AddReceiver(std::forward<F>(callback));
  }

  // F: void(const webrtc::IceCandidateErrorEvent&)
  template <typename F>
  void SubscribeIceCandidateError(F&& callback) {
    RTC_DCHECK_RUN_ON(network_thread_);
    signal_ice_candidate_error_.AddReceiver(std::forward<F>(callback));
  }

  // F: void(const std::vector<webrtc::Candidate>&)
  template <typename F>
  void SubscribeIceCandidatesRemoved(F&& callback) {
    RTC_DCHECK_RUN_ON(network_thread_);
    signal_ice_candidates_removed_.AddReceiver(std::forward<F>(callback));
  }

  // F: void(const webrtc::CandidatePairChangeEvent&)
  template <typename F>
  void SubscribeIceCandidatePairChanged(F&& callback) {
    RTC_DCHECK_RUN_ON(network_thread_);
    signal_ice_candidate_pair_changed_.AddReceiver(std::forward<F>(callback));
  }

 private:
  // All of these callbacks are fired on the network thread.

  // If any transport failed => failed,
  // Else if all completed => completed,
  // Else if all connected => connected,
  // Else => connecting
  CallbackList<IceConnectionState> signal_ice_connection_state_
      RTC_GUARDED_BY(network_thread_);

  CallbackList<PeerConnectionInterface::PeerConnectionState>
      signal_connection_state_ RTC_GUARDED_BY(network_thread_);

  CallbackList<PeerConnectionInterface::IceConnectionState>
      signal_standardized_ice_connection_state_ RTC_GUARDED_BY(network_thread_);

  // If all transports done gathering => complete,
  // Else if any are gathering => gathering,
  // Else => new
  CallbackList<IceGatheringState> signal_ice_gathering_state_
      RTC_GUARDED_BY(network_thread_);

  // [mid, candidates]
  CallbackList<const std::string&, const std::vector<Candidate>&>
      signal_ice_candidates_gathered_ RTC_GUARDED_BY(network_thread_);

  CallbackList<const IceCandidateErrorEvent&> signal_ice_candidate_error_
      RTC_GUARDED_BY(network_thread_);

  CallbackList<const std::vector<Candidate>&> signal_ice_candidates_removed_
      RTC_GUARDED_BY(network_thread_);

  CallbackList<const CandidatePairChangeEvent&>
      signal_ice_candidate_pair_changed_ RTC_GUARDED_BY(network_thread_);

  // Called from SetLocalDescription and SetRemoteDescription.
  // When `local` is true, local_desc must be valid. Similarly when
  // `local` is false, remote_desc must be valid. The description counterpart
  // to the one that's being applied, may be nullptr but when it's supplied
  // the counterpart description's content groups will  be kept up to date for
  // `type == SdpType::kAnswer`.
  RTCError ApplyDescription_n(bool local,
                              SdpType type,
                              const SessionDescription* local_desc,
                              const SessionDescription* remote_desc)
      RTC_RUN_ON(network_thread_);
  RTCError ValidateAndMaybeUpdateBundleGroups(
      bool local,
      SdpType type,
      const SessionDescription* local_desc,
      const SessionDescription* remote_desc) RTC_RUN_ON(network_thread_);
  RTCError ValidateContent(const ContentInfo& content_info);

  void HandleRejectedContent(const ContentInfo& content_info)
      RTC_RUN_ON(network_thread_);
  bool HandleBundledContent(const ContentInfo& content_info,
                            const ContentGroup& bundle_group)
      RTC_RUN_ON(network_thread_);

  JsepTransportDescription CreateJsepTransportDescription(
      const ContentInfo& content_info,
      const TransportInfo& transport_info,
      const std::vector<int>& encrypted_extension_ids,
      int rtp_abs_sendtime_extn_id);

  std::map<const ContentGroup*, std::vector<int>>
  MergeEncryptedHeaderExtensionIdsForBundles(
      const SessionDescription* description);
  std::vector<int> GetEncryptedHeaderExtensionIds(
      const ContentInfo& content_info);

  int GetRtpAbsSendTimeHeaderExtensionId(const ContentInfo& content_info);

  // This method takes the BUNDLE group into account. If the JsepTransport is
  // destroyed because of BUNDLE, it would return the transport which other
  // transports are bundled on (In current implementation, it is the first
  // content in the BUNDLE group).
  const JsepTransport* GetJsepTransportForMid(const std::string& mid) const
      RTC_RUN_ON(network_thread_);
  JsepTransport* GetJsepTransportForMid(const std::string& mid)
      RTC_RUN_ON(network_thread_);
  const JsepTransport* GetJsepTransportForMid(absl::string_view mid) const
      RTC_RUN_ON(network_thread_);
  JsepTransport* GetJsepTransportForMid(absl::string_view mid)
      RTC_RUN_ON(network_thread_);

  // Get the JsepTransport without considering the BUNDLE group. Return nullptr
  // if the JsepTransport is destroyed.
  const JsepTransport* GetJsepTransportByName(
      const std::string& transport_name) const RTC_RUN_ON(network_thread_);
  JsepTransport* GetJsepTransportByName(const std::string& transport_name)
      RTC_RUN_ON(network_thread_);

  // Creates jsep transport. Noop if transport is already created.
  // Transport is created either during SetLocalDescription (`local` == true) or
  // during SetRemoteDescription (`local` == false). Passing `local` helps to
  // differentiate initiator (caller) from answerer (callee).
  RTCError MaybeCreateJsepTransport(bool local,
                                    const ContentInfo& content_info,
                                    const SessionDescription& description)
      RTC_RUN_ON(network_thread_);

  void DestroyAllJsepTransports_n() RTC_RUN_ON(network_thread_);

  void SetIceRole_n(IceRole ice_role) RTC_RUN_ON(network_thread_);

  IceRole DetermineIceRole(JsepTransport* jsep_transport,
                           const TransportInfo& transport_info,
                           SdpType type,
                           bool local);

  std::unique_ptr<DtlsTransportInternal> CreateDtlsTransport(
      const ContentInfo& content_info,
      IceTransportInternal* ice);
  scoped_refptr<IceTransportInterface> CreateIceTransport(
      const std::string& transport_name,
      bool rtcp);

  std::unique_ptr<RtpTransport> CreateUnencryptedRtpTransport(
      const std::string& transport_name,
      PacketTransportInternal* rtp_packet_transport,
      PacketTransportInternal* rtcp_packet_transport);
  std::unique_ptr<SrtpTransport> CreateSdesTransport(
      const std::string& transport_name,
      DtlsTransportInternal* rtp_dtls_transport,
      DtlsTransportInternal* rtcp_dtls_transport);
  std::unique_ptr<DtlsSrtpTransport> CreateDtlsSrtpTransport(
      const std::string& transport_name,
      DtlsTransportInternal* rtp_dtls_transport,
      DtlsTransportInternal* rtcp_dtls_transport);

  // Collect all the DtlsTransports, including RTP and RTCP, from the
  // JsepTransports, including those not mapped to a MID because they are being
  // kept alive in case of rollback.
  std::vector<DtlsTransportInternal*> GetDtlsTransports();
  // Same as the above, but doesn't include rollback transports.
  // JsepTransportController can iterate all the DtlsTransports and update the
  // aggregate states.
  std::vector<DtlsTransportInternal*> GetActiveDtlsTransports();

  // Handlers for signals from Transport.
  void OnTransportWritableState_n(PacketTransportInternal* transport)
      RTC_RUN_ON(network_thread_);
  void OnTransportReceivingState_n(PacketTransportInternal* transport)
      RTC_RUN_ON(network_thread_);
  void OnTransportGatheringState_n(IceTransportInternal* transport)
      RTC_RUN_ON(network_thread_);
  void OnTransportCandidateGathered_n(IceTransportInternal* transport,
                                      const Candidate& candidate)
      RTC_RUN_ON(network_thread_);
  void OnTransportCandidateError_n(IceTransportInternal* transport,
                                   const IceCandidateErrorEvent& event)
      RTC_RUN_ON(network_thread_);
  void OnTransportCandidatesRemoved_n(IceTransportInternal* transport,
                                      const Candidates& candidates)
      RTC_RUN_ON(network_thread_);
  void OnTransportRoleConflict_n(IceTransportInternal* transport)
      RTC_RUN_ON(network_thread_);
  void OnTransportStateChanged_n(IceTransportInternal* transport)
      RTC_RUN_ON(network_thread_);
  void OnTransportCandidatePairChanged_n(const CandidatePairChangeEvent& event)
      RTC_RUN_ON(network_thread_);
  void UpdateAggregateStates_n() RTC_RUN_ON(network_thread_);

  void OnRtcpPacketReceived_n(CopyOnWriteBuffer* packet, int64_t packet_time_us)
      RTC_RUN_ON(network_thread_);
  void OnUnDemuxableRtpPacketReceived_n(const RtpPacketReceived& packet)
      RTC_RUN_ON(network_thread_);

  void OnDtlsHandshakeError(SSLHandshakeError error);

  bool OnTransportChanged(const std::string& mid, JsepTransport* transport);

  const Environment env_;
  Thread* const network_thread_ = nullptr;
  PortAllocator* const port_allocator_ = nullptr;
  AsyncDnsResolverFactoryInterface* const async_dns_resolver_factory_ = nullptr;

  JsepTransportCollection transports_ RTC_GUARDED_BY(network_thread_);
  // Aggregate states for Transports.
  // standardized_ice_connection_state_ is intended to replace
  // ice_connection_state, see bugs.webrtc.org/9308
  IceConnectionState ice_connection_state_ = kIceConnectionConnecting;
  PeerConnectionInterface::IceConnectionState
      standardized_ice_connection_state_ =
          PeerConnectionInterface::kIceConnectionNew;
  PeerConnectionInterface::PeerConnectionState combined_connection_state_ =
      PeerConnectionInterface::PeerConnectionState::kNew;
  IceGatheringState ice_gathering_state_ = kIceGatheringNew;

  const Config config_;
  bool active_reset_srtp_params_ RTC_GUARDED_BY(network_thread_);

  std::optional<bool> initial_offerer_;

  IceConfig ice_config_;
  IceRole ice_role_ = ICEROLE_CONTROLLING;
  scoped_refptr<RTCCertificate> certificate_;

  BundleManager bundles_;
  // Reference to the SdpOfferAnswerHandler's payload type picker.
  PayloadTypePicker& payload_type_picker_;
};

}  // namespace webrtc

#endif  // PC_JSEP_TRANSPORT_CONTROLLER_H_
