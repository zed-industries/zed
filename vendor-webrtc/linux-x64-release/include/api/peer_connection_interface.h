/*
 *  Copyright 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

// This file contains the PeerConnection interface as defined in
// https://w3c.github.io/webrtc-pc/#peer-to-peer-connections
//
// The PeerConnectionFactory class provides factory methods to create
// PeerConnection, MediaStream and MediaStreamTrack objects.
//
// The following steps are needed to setup a typical call using WebRTC:
//
// 1. Create a PeerConnectionFactoryInterface. Check constructors for more
// information about input parameters.
//
// 2. Create a PeerConnection object. Provide a configuration struct which
// points to STUN and/or TURN servers used to generate ICE candidates, and
// provide an object that implements the PeerConnectionObserver interface,
// which is used to receive callbacks from the PeerConnection.
//
// 3. Create local MediaStreamTracks using the PeerConnectionFactory and add
// them to PeerConnection by calling AddTrack (or legacy method, AddStream).
//
// 4. Create an offer, call SetLocalDescription with it, serialize it, and send
// it to the remote peer
//
// 5. Once an ICE candidate has been gathered, the PeerConnection will call the
// observer function OnIceCandidate. The candidates must also be serialized and
// sent to the remote peer.
//
// 6. Once an answer is received from the remote peer, call
// SetRemoteDescription with the remote answer.
//
// 7. Once a remote candidate is received from the remote peer, provide it to
// the PeerConnection by calling AddIceCandidate.
//
// The receiver of a call (assuming the application is "call"-based) can decide
// to accept or reject the call; this decision will be taken by the application,
// not the PeerConnection.
//
// If the application decides to accept the call, it should:
//
// 1. Create PeerConnectionFactoryInterface if it doesn't exist.
//
// 2. Create a new PeerConnection.
//
// 3. Provide the remote offer to the new PeerConnection object by calling
// SetRemoteDescription.
//
// 4. Generate an answer to the remote offer by calling CreateAnswer and send it
// back to the remote peer.
//
// 5. Provide the local answer to the new PeerConnection by calling
// SetLocalDescription with the answer.
//
// 6. Provide the remote ICE candidates by calling AddIceCandidate.
//
// 7. Once a candidate has been gathered, the PeerConnection will call the
// observer function OnIceCandidate. Send these candidates to the remote peer.

#ifndef API_PEER_CONNECTION_INTERFACE_H_
#define API_PEER_CONNECTION_INTERFACE_H_
// IWYU pragma: no_include "pc/media_factory.h"

#include <stdint.h>
#include <stdio.h>

#include <functional>
#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "absl/base/attributes.h"
#include "absl/strings/string_view.h"
#include "api/adaptation/resource.h"
#include "api/async_dns_resolver.h"
#include "api/audio/audio_device.h"
#include "api/audio/audio_mixer.h"
#include "api/audio/audio_processing.h"
#include "api/audio_codecs/audio_decoder_factory.h"
#include "api/audio_codecs/audio_encoder_factory.h"
#include "api/audio_options.h"
#include "api/candidate.h"
#include "api/crypto/crypto_options.h"
#include "api/data_channel_event_observer_interface.h"
#include "api/data_channel_interface.h"
#include "api/dtls_transport_interface.h"
#include "api/fec_controller.h"
#include "api/field_trials_view.h"
#include "api/ice_transport_interface.h"
#include "api/jsep.h"
#include "api/legacy_stats_types.h"
#include "api/media_stream_interface.h"
#include "api/media_types.h"
#include "api/metronome/metronome.h"
#include "api/neteq/neteq_factory.h"
#include "api/network_state_predictor.h"
#include "api/packet_socket_factory.h"
#include "api/rtc_error.h"
#include "api/rtc_event_log/rtc_event_log_factory_interface.h"
#include "api/rtc_event_log_output.h"
#include "api/rtp_parameters.h"
#include "api/rtp_receiver_interface.h"
#include "api/rtp_sender_interface.h"
#include "api/rtp_transceiver_interface.h"
#include "api/scoped_refptr.h"
#include "api/sctp_transport_interface.h"
#include "api/set_local_description_observer_interface.h"
#include "api/set_remote_description_observer_interface.h"
#include "api/stats/rtc_stats_collector_callback.h"
#include "api/task_queue/task_queue_factory.h"
#include "api/transport/bandwidth_estimation_settings.h"
#include "api/transport/bitrate_settings.h"
#include "api/transport/enums.h"
#include "api/transport/network_control.h"
#include "api/transport/sctp_transport_factory_interface.h"
#include "api/turn_customizer.h"
#include "api/video/video_bitrate_allocator_factory.h"
#include "api/video_codecs/video_decoder_factory.h"
#include "api/video_codecs/video_encoder_factory.h"
#include "call/rtp_transport_controller_send_factory_interface.h"
#include "media/base/media_config.h"
// TODO(bugs.webrtc.org/7447): We plan to provide a way to let applications
// inject a PacketSocketFactory and/or NetworkManager, and not expose
// PortAllocator in the PeerConnection api.
#include "api/audio/audio_frame_processor.h"
#include "api/ref_count.h"
#include "api/units/time_delta.h"
#include "p2p/base/port.h"
#include "p2p/base/port_allocator.h"
#include "rtc_base/checks.h"
#include "rtc_base/network.h"
#include "rtc_base/network_constants.h"
#include "rtc_base/network_monitor_factory.h"
#include "rtc_base/rtc_certificate.h"
#include "rtc_base/rtc_certificate_generator.h"
#include "rtc_base/socket_factory.h"
#include "rtc_base/ssl_certificate.h"
#include "rtc_base/ssl_stream_adapter.h"
#include "rtc_base/system/rtc_export.h"
#include "rtc_base/thread.h"

namespace webrtc {
// IWYU pragma: begin_keep
// MediaFactory class definition is not part of the api.
class MediaFactory;

// IWYU pragma: end_keep
// MediaStream container interface.
class StreamCollectionInterface : public webrtc::RefCountInterface {
 public:
  // TODO(ronghuawu): Update the function names to c++ style, e.g. find -> Find.
  virtual size_t count() = 0;
  virtual MediaStreamInterface* at(size_t index) = 0;
  virtual MediaStreamInterface* find(const std::string& label) = 0;
  virtual MediaStreamTrackInterface* FindAudioTrack(const std::string& id) = 0;
  virtual MediaStreamTrackInterface* FindVideoTrack(const std::string& id) = 0;

 protected:
  // Dtor protected as objects shouldn't be deleted via this interface.
  ~StreamCollectionInterface() override = default;
};

class StatsObserver : public webrtc::RefCountInterface {
 public:
  virtual void OnComplete(const StatsReports& reports) = 0;

 protected:
  ~StatsObserver() override = default;
};

enum class SdpSemantics {
  // TODO(https://crbug.com/webrtc/13528): Remove support for kPlanB.
  kPlanB_DEPRECATED,
  kPlanB [[deprecated]] = kPlanB_DEPRECATED,
  kUnifiedPlan,
};

class RTC_EXPORT PeerConnectionInterface : public webrtc::RefCountInterface {
 public:
  // See https://w3c.github.io/webrtc-pc/#dom-rtcsignalingstate
  enum SignalingState {
    kStable,
    kHaveLocalOffer,
    kHaveLocalPrAnswer,
    kHaveRemoteOffer,
    kHaveRemotePrAnswer,
    kClosed,
  };
  static constexpr absl::string_view AsString(SignalingState);

  // See https://w3c.github.io/webrtc-pc/#dom-rtcicegatheringstate
  enum IceGatheringState {
    kIceGatheringNew,
    kIceGatheringGathering,
    kIceGatheringComplete
  };
  static constexpr absl::string_view AsString(IceGatheringState state);

  // See https://w3c.github.io/webrtc-pc/#dom-rtcpeerconnectionstate
  enum class PeerConnectionState {
    kNew,
    kConnecting,
    kConnected,
    kDisconnected,
    kFailed,
    kClosed,
  };
  static constexpr absl::string_view AsString(PeerConnectionState state);

  // See https://w3c.github.io/webrtc-pc/#dom-rtciceconnectionstate
  enum IceConnectionState {
    kIceConnectionNew,
    kIceConnectionChecking,
    kIceConnectionConnected,
    kIceConnectionCompleted,
    kIceConnectionFailed,
    kIceConnectionDisconnected,
    kIceConnectionClosed,
    kIceConnectionMax,
  };
  static constexpr absl::string_view AsString(IceConnectionState state);

  // TLS certificate policy.
  enum TlsCertPolicy {
    // For TLS based protocols, ensure the connection is secure by not
    // circumventing certificate validation.
    kTlsCertPolicySecure,
    // For TLS based protocols, disregard security completely by skipping
    // certificate validation. This is insecure and should never be used unless
    // security is irrelevant in that particular context.
    kTlsCertPolicyInsecureNoCheck,
  };

  struct RTC_EXPORT IceServer {
    IceServer();
    IceServer(const IceServer&);
    ~IceServer();

    // TODO(jbauch): Remove uri when all code using it has switched to urls.
    // List of URIs associated with this server. Valid formats are described
    // in RFC7064 and RFC7065, and more may be added in the future. The "host"
    // part of the URI may contain either an IP address or a hostname.
    std::string uri;
    std::vector<std::string> urls;
    std::string username;
    std::string password;
    TlsCertPolicy tls_cert_policy = kTlsCertPolicySecure;
    // If the URIs in `urls` only contain IP addresses, this field can be used
    // to indicate the hostname, which may be necessary for TLS (using the SNI
    // extension). If `urls` itself contains the hostname, this isn't
    // necessary.
    std::string hostname;
    // List of protocols to be used in the TLS ALPN extension.
    std::vector<std::string> tls_alpn_protocols;
    // List of elliptic curves to be used in the TLS elliptic curves extension.
    std::vector<std::string> tls_elliptic_curves;

    bool operator==(const IceServer& o) const {
      return uri == o.uri && urls == o.urls && username == o.username &&
             password == o.password && tls_cert_policy == o.tls_cert_policy &&
             hostname == o.hostname &&
             tls_alpn_protocols == o.tls_alpn_protocols &&
             tls_elliptic_curves == o.tls_elliptic_curves;
    }
    bool operator!=(const IceServer& o) const { return !(*this == o); }
  };
  typedef std::vector<IceServer> IceServers;

  enum IceTransportsType {
    // TODO(pthatcher): Rename these kTransporTypeXXX, but update
    // Chromium at the same time.
    kNone,
    kRelay,
    kNoHost,
    kAll
  };

  // https://tools.ietf.org/html/draft-ietf-rtcweb-jsep-24#section-4.1.1
  enum BundlePolicy {
    kBundlePolicyBalanced,
    kBundlePolicyMaxBundle,
    kBundlePolicyMaxCompat
  };

  // https://tools.ietf.org/html/draft-ietf-rtcweb-jsep-24#section-4.1.1
  enum RtcpMuxPolicy {
    kRtcpMuxPolicyNegotiate,
    kRtcpMuxPolicyRequire,
  };

  enum TcpCandidatePolicy {
    kTcpCandidatePolicyEnabled,
    kTcpCandidatePolicyDisabled
  };

  enum CandidateNetworkPolicy {
    kCandidateNetworkPolicyAll,
    kCandidateNetworkPolicyLowCost
  };

  enum ContinualGatheringPolicy { GATHER_ONCE, GATHER_CONTINUALLY };

  struct PortAllocatorConfig {
    // For min_port and max_port, 0 means not specified.
    int min_port = 0;
    int max_port = 0;
    uint32_t flags = 0;  // Same as kDefaultPortAllocatorFlags.
  };

  enum class RTCConfigurationType {
    // A configuration that is safer to use, despite not having the best
    // performance. Currently this is the default configuration.
    kSafe,
    // An aggressive configuration that has better performance, although it
    // may be riskier and may need extra support in the application.
    kAggressive
  };

  // TODO(hbos): Change into class with private data and public getters.
  // TODO(nisse): In particular, accessing fields directly from an
  // application is brittle, since the organization mirrors the
  // organization of the implementation, which isn't stable. So we
  // need getters and setters at least for fields which applications
  // are interested in.
  struct RTC_EXPORT RTCConfiguration {
    // This struct is subject to reorganization, both for naming
    // consistency, and to group settings to match where they are used
    // in the implementation. To do that, we need getter and setter
    // methods for all settings which are of interest to applications,
    // Chrome in particular.

    RTCConfiguration();
    RTCConfiguration(const RTCConfiguration&);
    explicit RTCConfiguration(RTCConfigurationType type);
    ~RTCConfiguration();

    bool operator==(const RTCConfiguration& o) const;
    bool operator!=(const RTCConfiguration& o) const;

    bool dscp() const { return media_config.enable_dscp; }
    void set_dscp(bool enable) { media_config.enable_dscp = enable; }

    bool stats_timestamp_with_environment_clock() const {
      return media_config.stats_timestamp_with_environment_clock;
    }
    void set_stats_timestamp_with_environment_clock(bool enable) {
      media_config.stats_timestamp_with_environment_clock = enable;
    }

    bool cpu_adaptation() const {
      return media_config.video.enable_cpu_adaptation;
    }
    void set_cpu_adaptation(bool enable) {
      media_config.video.enable_cpu_adaptation = enable;
    }

    bool suspend_below_min_bitrate() const {
      return media_config.video.suspend_below_min_bitrate;
    }
    void set_suspend_below_min_bitrate(bool enable) {
      media_config.video.suspend_below_min_bitrate = enable;
    }

    bool prerenderer_smoothing() const {
      return media_config.video.enable_prerenderer_smoothing;
    }
    void set_prerenderer_smoothing(bool enable) {
      media_config.video.enable_prerenderer_smoothing = enable;
    }

    bool experiment_cpu_load_estimator() const {
      return media_config.video.experiment_cpu_load_estimator;
    }
    void set_experiment_cpu_load_estimator(bool enable) {
      media_config.video.experiment_cpu_load_estimator = enable;
    }

    int audio_rtcp_report_interval_ms() const {
      return media_config.audio.rtcp_report_interval_ms;
    }
    void set_audio_rtcp_report_interval_ms(int audio_rtcp_report_interval_ms) {
      media_config.audio.rtcp_report_interval_ms =
          audio_rtcp_report_interval_ms;
    }

    int video_rtcp_report_interval_ms() const {
      return media_config.video.rtcp_report_interval_ms;
    }
    void set_video_rtcp_report_interval_ms(int video_rtcp_report_interval_ms) {
      media_config.video.rtcp_report_interval_ms =
          video_rtcp_report_interval_ms;
    }

    // Settings for the port allcoator. Applied only if the port allocator is
    // created by PeerConnectionFactory, not if it is injected with
    // PeerConnectionDependencies
    int min_port() const { return port_allocator_config.min_port; }
    void set_min_port(int port) { port_allocator_config.min_port = port; }
    int max_port() const { return port_allocator_config.max_port; }
    void set_max_port(int port) { port_allocator_config.max_port = port; }
    uint32_t port_allocator_flags() { return port_allocator_config.flags; }
    void set_port_allocator_flags(uint32_t flags) {
      port_allocator_config.flags = flags;
    }

    static const int kUndefined = -1;
    // Default maximum number of packets in the audio jitter buffer.
    static const int kAudioJitterBufferMaxPackets = 200;
    // ICE connection receiving timeout for aggressive configuration.
    static const int kAggressiveIceConnectionReceivingTimeout = 1000;

    ////////////////////////////////////////////////////////////////////////
    // The below few fields mirror the standard RTCConfiguration dictionary:
    // https://w3c.github.io/webrtc-pc/#rtcconfiguration-dictionary
    ////////////////////////////////////////////////////////////////////////

    // TODO(pthatcher): Rename this ice_servers, but update Chromium
    // at the same time.
    IceServers servers;
    // TODO(pthatcher): Rename this ice_transport_type, but update
    // Chromium at the same time.
    IceTransportsType type = kAll;
    BundlePolicy bundle_policy = kBundlePolicyBalanced;
    RtcpMuxPolicy rtcp_mux_policy = kRtcpMuxPolicyRequire;
    std::vector<scoped_refptr<RTCCertificate>> certificates;
    int ice_candidate_pool_size = 0;

    //////////////////////////////////////////////////////////////////////////
    // The below fields correspond to constraints from the deprecated
    // constraints interface for constructing a PeerConnection.
    //
    // std::optional fields can be "missing", in which case the implementation
    // default will be used.
    //////////////////////////////////////////////////////////////////////////

    // If set to true, don't gather IPv6 ICE candidates on Wi-Fi.
    // Only intended to be used on specific devices. Certain phones disable IPv6
    // when the screen is turned off and it would be better to just disable the
    // IPv6 ICE candidates on Wi-Fi in those cases.
    bool disable_ipv6_on_wifi = false;

    // By default, the PeerConnection will use a limited number of IPv6 network
    // interfaces, in order to avoid too many ICE candidate pairs being created
    // and delaying ICE completion.
    //
    // Can be set to INT_MAX to effectively disable the limit.
    int max_ipv6_networks = kDefaultMaxIPv6Networks;

    // Exclude link-local network interfaces
    // from consideration for gathering ICE candidates.
    bool disable_link_local_networks = false;

    // Minimum bitrate at which screencast video tracks will be encoded at.
    // This means adding padding bits up to this bitrate, which can help
    // when switching from a static scene to one with motion.
    std::optional<int> screencast_min_bitrate;

    /////////////////////////////////////////////////
    // The below fields are not part of the standard.
    /////////////////////////////////////////////////

    // Can be used to disable TCP candidate generation.
    TcpCandidatePolicy tcp_candidate_policy = kTcpCandidatePolicyEnabled;

    // Can be used to avoid gathering candidates for a "higher cost" network,
    // if a lower cost one exists. For example, if both Wi-Fi and cellular
    // interfaces are available, this could be used to avoid using the cellular
    // interface.
    CandidateNetworkPolicy candidate_network_policy =
        kCandidateNetworkPolicyAll;

    // The maximum number of packets that can be stored in the NetEq audio
    // jitter buffer. Can be reduced to lower tolerated audio latency.
    int audio_jitter_buffer_max_packets = kAudioJitterBufferMaxPackets;

    // Whether to use the NetEq "fast mode" which will accelerate audio quicker
    // if it falls behind.
    bool audio_jitter_buffer_fast_accelerate = false;

    // The minimum delay in milliseconds for the audio jitter buffer.
    int audio_jitter_buffer_min_delay_ms = 0;

    // Timeout in milliseconds before an ICE candidate pair is considered to be
    // "not receiving", after which a lower priority candidate pair may be
    // selected.
    int ice_connection_receiving_timeout = kUndefined;

    // Interval in milliseconds at which an ICE "backup" candidate pair will be
    // pinged. This is a candidate pair which is not actively in use, but may
    // be switched to if the active candidate pair becomes unusable.
    //
    // This is relevant mainly to Wi-Fi/cell handoff; the application may not
    // want this backup cellular candidate pair pinged frequently, since it
    // consumes data/battery.
    int ice_backup_candidate_pair_ping_interval = kUndefined;

    // Can be used to enable continual gathering, which means new candidates
    // will be gathered as network interfaces change. Note that if continual
    // gathering is used, the candidate removal API should also be used, to
    // avoid an ever-growing list of candidates.
    ContinualGatheringPolicy continual_gathering_policy = GATHER_ONCE;

    // If set to true, candidate pairs will be pinged in order of most likely
    // to work (which means using a TURN server, generally), rather than in
    // standard priority order.
    bool prioritize_most_likely_ice_candidate_pairs = false;

    // Implementation defined settings. A public member only for the benefit of
    // the implementation. Applications must not access it directly, and should
    // instead use provided accessor methods, e.g., set_cpu_adaptation.
    struct MediaConfig media_config;

    // If set to true, only one preferred TURN allocation will be used per
    // network interface. UDP is preferred over TCP and IPv6 over IPv4. This
    // can be used to cut down on the number of candidate pairings.
    // Deprecated. TODO(webrtc:11026) Remove this flag once the downstream
    // dependency is removed.
    bool prune_turn_ports = false;

    // The policy used to prune turn port.
    PortPrunePolicy turn_port_prune_policy = NO_PRUNE;

    PortPrunePolicy GetTurnPortPrunePolicy() const {
      return prune_turn_ports ? PRUNE_BASED_ON_PRIORITY
                              : turn_port_prune_policy;
    }

    // If set to true, this means the ICE transport should presume TURN-to-TURN
    // candidate pairs will succeed, even before a binding response is received.
    // This can be used to optimize the initial connection time, since the DTLS
    // handshake can begin immediately.
    bool presume_writable_when_fully_relayed = false;

    // If true, "renomination" will be added to the ice options in the transport
    // description.
    // See: https://tools.ietf.org/html/draft-thatcher-ice-renomination-00
    bool enable_ice_renomination = false;

    // If true, the ICE role is re-determined when the PeerConnection sets a
    // local transport description that indicates an ICE restart.
    //
    // This is standard RFC5245 ICE behavior, but causes unnecessary role
    // thrashing, so an application may wish to avoid it. This role
    // re-determining was removed in ICEbis (ICE v2).
    bool redetermine_role_on_ice_restart = true;

    // This flag is only effective when `continual_gathering_policy` is
    // GATHER_CONTINUALLY.
    //
    // If true, after the ICE transport type is changed such that new types of
    // ICE candidates are allowed by the new transport type, e.g. from
    // IceTransportsType::kRelay to IceTransportsType::kAll, candidates that
    // have been gathered by the ICE transport but not matching the previous
    // transport type and as a result not observed by PeerConnectionObserver,
    // will be surfaced to the observer.
    bool surface_ice_candidates_on_ice_transport_type_changed = false;

    // The following fields define intervals in milliseconds at which ICE
    // connectivity checks are sent.
    //
    // We consider ICE is "strongly connected" for an agent when there is at
    // least one candidate pair that currently succeeds in connectivity check
    // from its direction i.e. sending a STUN ping and receives a STUN ping
    // response, AND all candidate pairs have sent a minimum number of pings for
    // connectivity (this number is implementation-specific). Otherwise, ICE is
    // considered in "weak connectivity".
    //
    // Note that the above notion of strong and weak connectivity is not defined
    // in RFC 5245, and they apply to our current ICE implementation only.
    //
    // 1) ice_check_interval_strong_connectivity defines the interval applied to
    // ALL candidate pairs when ICE is strongly connected, and it overrides the
    // default value of this interval in the ICE implementation;
    // 2) ice_check_interval_weak_connectivity defines the counterpart for ALL
    // pairs when ICE is weakly connected, and it overrides the default value of
    // this interval in the ICE implementation;
    // 3) ice_check_min_interval defines the minimal interval (equivalently the
    // maximum rate) that overrides the above two intervals when either of them
    // is less.
    std::optional<int> ice_check_interval_strong_connectivity;
    std::optional<int> ice_check_interval_weak_connectivity;
    std::optional<int> ice_check_min_interval;

    // The min time period for which a candidate pair must wait for response to
    // connectivity checks before it becomes unwritable. This parameter
    // overrides the default value in the ICE implementation if set.
    std::optional<int> ice_unwritable_timeout;

    // The min number of connectivity checks that a candidate pair must sent
    // without receiving response before it becomes unwritable. This parameter
    // overrides the default value in the ICE implementation if set.
    std::optional<int> ice_unwritable_min_checks;

    // The min time period for which a candidate pair must wait for response to
    // connectivity checks it becomes inactive. This parameter overrides the
    // default value in the ICE implementation if set.
    std::optional<int> ice_inactive_timeout;

    // The interval in milliseconds at which STUN candidates will resend STUN
    // binding requests to keep NAT bindings open.
    std::optional<int> stun_candidate_keepalive_interval;

    // Optional TurnCustomizer.
    // With this class one can modify outgoing TURN messages.
    // The object passed in must remain valid until PeerConnection::Close() is
    // called.
    webrtc::TurnCustomizer* turn_customizer = nullptr;

    // Preferred network interface.
    // A candidate pair on a preferred network has a higher precedence in ICE
    // than one on an un-preferred network, regardless of priority or network
    // cost.
    std::optional<AdapterType> network_preference;

    // Configure the SDP semantics used by this PeerConnection. By default, this
    // is Unified Plan which is compliant to the WebRTC 1.0 specification. It is
    // possible to overrwite this to the deprecated Plan B SDP format, but note
    // that kPlanB will be deleted at some future date, see
    // https://crbug.com/webrtc/13528.
    //
    // kUnifiedPlan will cause the PeerConnection to create offers and answers
    // with multiple m= sections where each m= section maps to one RtpSender and
    // one RtpReceiver (an RtpTransceiver), either both audio or both video.
    // This will also cause the PeerConnection to ignore all but the first
    // a=ssrc lines that form a Plan B streams (if the PeerConnection is given
    // Plan B SDP to process).
    //
    // kPlanB will cause the PeerConnection to create offers and answers with at
    // most one audio and one video m= section with multiple RtpSenders and
    // RtpReceivers specified as multiple a=ssrc lines within the section. This
    // will also cause PeerConnection to ignore all but the first m= section of
    // the same media type (if the PeerConnection is given Unified Plan SDP to
    // process).
    SdpSemantics sdp_semantics = SdpSemantics::kUnifiedPlan;

    // TODO(bugs.webrtc.org/9891) - Move to crypto_options or remove.
    // Actively reset the SRTP parameters whenever the DTLS transports
    // underneath are reset for every offer/answer negotiation.
    // This is only intended to be a workaround for crbug.com/835958
    // WARNING: This would cause RTP/RTCP packets decryption failure if not used
    // correctly. This flag will be deprecated soon. Do not rely on it.
    bool active_reset_srtp_params = false;

    // Defines advanced optional cryptographic settings related to SRTP and
    // frame encryption for native WebRTC. Setting this will overwrite any
    // settings set in PeerConnectionFactory (which is deprecated).
    std::optional<CryptoOptions> crypto_options;

    // Configure if we should include the SDP attribute extmap-allow-mixed in
    // our offer on session level.
    bool offer_extmap_allow_mixed = true;

    // TURN logging identifier.
    // This identifier is added to a TURN allocation
    // and it intended to be used to be able to match client side
    // logs with TURN server logs. It will not be added if it's an empty string.
    std::string turn_logging_id;

    // Added to be able to control rollout of this feature.
    bool enable_implicit_rollback = false;

    // The delay before doing a usage histogram report for long-lived
    // PeerConnections. Used for testing only.
    std::optional<int> report_usage_pattern_delay_ms;

    // The ping interval (ms) when the connection is stable and writable. This
    // parameter overrides the default value in the ICE implementation if set.
    std::optional<int> stable_writable_connection_ping_interval_ms;

    // Whether this PeerConnection will avoid VPNs (kAvoidVpn), prefer VPNs
    // (kPreferVpn), only work over VPN (kOnlyUseVpn) or only work over non-VPN
    // (kNeverUseVpn) interfaces. This controls which local interfaces the
    // PeerConnection will prefer to connect over. Since VPN detection is not
    // perfect, adherence to this preference cannot be guaranteed.
    VpnPreference vpn_preference = VpnPreference::kDefault;

    // List of address/length subnets that should be treated like
    // VPN (in case webrtc fails to auto detect them).
    std::vector<NetworkMask> vpn_list;

    PortAllocatorConfig port_allocator_config;

    // The burst interval of the pacer, see TaskQueuePacedSender constructor.
    std::optional<TimeDelta> pacer_burst_interval;

    // When this flag is set, ports not bound to any specific network interface
    // will be used, in addition to normal ports bound to the enumerated
    // interfaces. Without this flag, these "any address" ports would only be
    // used when network enumeration fails or is disabled. But under certain
    // conditions, these ports may succeed where others fail, so they may allow
    // the application to work in a wider variety of environments, at the expense
    // of having to allocate additional candidates.
    bool enable_any_address_ports = false;

    //
    // Don't forget to update operator== if adding something.
    //
  };

  // See: https://www.w3.org/TR/webrtc/#idl-def-rtcofferansweroptions
  struct RTCOfferAnswerOptions {
    static const int kUndefined = -1;
    static const int kMaxOfferToReceiveMedia = 1;

    // The default value for constraint offerToReceiveX:true.
    static const int kOfferToReceiveMediaTrue = 1;

    // These options are left as backwards compatibility for clients who need
    // "Plan B" semantics. Clients who have switched to "Unified Plan" semantics
    // should use the RtpTransceiver API (AddTransceiver) instead.
    //
    // offer_to_receive_X set to 1 will cause a media description to be
    // generated in the offer, even if no tracks of that type have been added.
    // Values greater than 1 are treated the same.
    //
    // If set to 0, the generated directional attribute will not include the
    // "recv" direction (meaning it will be "sendonly" or "inactive".
    int offer_to_receive_video = kUndefined;
    int offer_to_receive_audio = kUndefined;

    bool voice_activity_detection = true;
    bool ice_restart = false;

    // If true, will offer to BUNDLE audio/video/data together. Not to be
    // confused with RTCP mux (multiplexing RTP and RTCP together).
    bool use_rtp_mux = true;

    // If true, "a=packetization:<payload_type> raw" attribute will be offered
    // in the SDP for all video payload and accepted in the answer if offered.
    bool raw_packetization_for_video = false;

    // This will apply to all video tracks with a Plan B SDP offer/answer.
    int num_simulcast_layers = 1;

    // If true: Use SDP format from draft-ietf-mmusic-scdp-sdp-03
    // If false: Use SDP format from draft-ietf-mmusic-sdp-sdp-26 or later
    bool use_obsolete_sctp_sdp = false;

    RTCOfferAnswerOptions() = default;

    RTCOfferAnswerOptions(int offer_to_receive_video,
                          int offer_to_receive_audio,
                          bool voice_activity_detection,
                          bool ice_restart,
                          bool use_rtp_mux)
        : offer_to_receive_video(offer_to_receive_video),
          offer_to_receive_audio(offer_to_receive_audio),
          voice_activity_detection(voice_activity_detection),
          ice_restart(ice_restart),
          use_rtp_mux(use_rtp_mux) {}
  };

  // Used by GetStats to decide which stats to include in the stats reports.
  // `kStatsOutputLevelStandard` includes the standard stats for Javascript API;
  // `kStatsOutputLevelDebug` includes both the standard stats and additional
  // stats for debugging purposes.
  enum StatsOutputLevel {
    kStatsOutputLevelStandard,
    kStatsOutputLevelDebug,
  };

  // Accessor methods to active local streams.
  // This method is not supported with kUnifiedPlan semantics. Please use
  // GetSenders() instead.
  virtual scoped_refptr<StreamCollectionInterface> local_streams() = 0;

  // Accessor methods to remote streams.
  // This method is not supported with kUnifiedPlan semantics. Please use
  // GetReceivers() instead.
  virtual scoped_refptr<StreamCollectionInterface> remote_streams() = 0;

  // Add a new MediaStream to be sent on this PeerConnection.
  // Note that a SessionDescription negotiation is needed before the
  // remote peer can receive the stream.
  //
  // This has been removed from the standard in favor of a track-based API. So,
  // this is equivalent to simply calling AddTrack for each track within the
  // stream, with the one difference that if "stream->AddTrack(...)" is called
  // later, the PeerConnection will automatically pick up the new track. Though
  // this functionality will be deprecated in the future.
  //
  // This method is not supported with kUnifiedPlan semantics. Please use
  // AddTrack instead.
  virtual bool AddStream(MediaStreamInterface* stream) = 0;

  // Remove a MediaStream from this PeerConnection.
  // Note that a SessionDescription negotiation is needed before the
  // remote peer is notified.
  //
  // This method is not supported with kUnifiedPlan semantics. Please use
  // RemoveTrack instead.
  virtual void RemoveStream(MediaStreamInterface* stream) = 0;

  // Add a new MediaStreamTrack to be sent on this PeerConnection, and return
  // the newly created RtpSender. The RtpSender will be associated with the
  // streams specified in the `stream_ids` list.
  //
  // Errors:
  // - INVALID_PARAMETER: `track` is null, has a kind other than audio or video,
  //       or a sender already exists for the track.
  // - INVALID_STATE: The PeerConnection is closed.
  virtual RTCErrorOr<scoped_refptr<RtpSenderInterface>> AddTrack(
      scoped_refptr<MediaStreamTrackInterface> track,
      const std::vector<std::string>& stream_ids) = 0;

  // Add a new MediaStreamTrack as above, but with an additional parameter,
  // `init_send_encodings` : initial RtpEncodingParameters for RtpSender,
  // similar to init_send_encodings in RtpTransceiverInit.
  // Note that a new transceiver will always be created.
  //
  virtual RTCErrorOr<scoped_refptr<RtpSenderInterface>> AddTrack(
      scoped_refptr<MediaStreamTrackInterface> track,
      const std::vector<std::string>& stream_ids,
      const std::vector<RtpEncodingParameters>& init_send_encodings) = 0;

  // Removes the connection between a MediaStreamTrack and the PeerConnection.
  // Stops sending on the RtpSender and marks the
  // corresponding RtpTransceiver direction as no longer sending.
  // https://w3c.github.io/webrtc-pc/#dom-rtcpeerconnection-removetrack
  //
  // Errors:
  // - INVALID_PARAMETER: `sender` is null or (Plan B only) the sender is not
  //       associated with this PeerConnection.
  // - INVALID_STATE: PeerConnection is closed.
  //
  // Plan B semantics: Removes the RtpSender from this PeerConnection.
  //
  // TODO(bugs.webrtc.org/9534): Rename to RemoveTrack once the other signature
  // is removed; remove default implementation once upstream is updated.
  virtual RTCError RemoveTrackOrError(
      scoped_refptr<RtpSenderInterface> /* sender */) {
    RTC_CHECK_NOTREACHED();
    return RTCError();
  }

  // AddTransceiver creates a new RtpTransceiver and adds it to the set of
  // transceivers. Adding a transceiver will cause future calls to CreateOffer
  // to add a media description for the corresponding transceiver.
  //
  // The initial value of `mid` in the returned transceiver is null. Setting a
  // new session description may change it to a non-null value.
  //
  // https://w3c.github.io/webrtc-pc/#dom-rtcpeerconnection-addtransceiver
  //
  // Optionally, an RtpTransceiverInit structure can be specified to configure
  // the transceiver from construction. If not specified, the transceiver will
  // default to having a direction of kSendRecv and not be part of any streams.
  //
  // These methods are only available when Unified Plan is enabled (see
  // RTCConfiguration).
  //
  // Common errors:
  // - INTERNAL_ERROR: The configuration does not have Unified Plan enabled.

  // Adds a transceiver with a sender set to transmit the given track. The kind
  // of the transceiver (and sender/receiver) will be derived from the kind of
  // the track.
  // Errors:
  // - INVALID_PARAMETER: `track` is null.
  virtual RTCErrorOr<scoped_refptr<RtpTransceiverInterface>> AddTransceiver(
      scoped_refptr<MediaStreamTrackInterface> track) = 0;
  virtual RTCErrorOr<scoped_refptr<RtpTransceiverInterface>> AddTransceiver(
      scoped_refptr<MediaStreamTrackInterface> track,
      const RtpTransceiverInit& init) = 0;

  // Adds a transceiver with the given kind. Can either be
  // webrtc::MediaType::AUDIO or webrtc::MediaType::VIDEO. Errors:
  // - INVALID_PARAMETER: `media_type` is not webrtc::MediaType::AUDIO or
  //                      webrtc::MediaType::VIDEO.
  virtual RTCErrorOr<scoped_refptr<RtpTransceiverInterface>> AddTransceiver(
      webrtc::MediaType media_type) = 0;
  virtual RTCErrorOr<scoped_refptr<RtpTransceiverInterface>> AddTransceiver(
      webrtc::MediaType media_type,
      const RtpTransceiverInit& init) = 0;

  // Creates a sender without a track. Can be used for "early media"/"warmup"
  // use cases, where the application may want to negotiate video attributes
  // before a track is available to send.
  //
  // The standard way to do this would be through "addTransceiver", but we
  // don't support that API yet.
  //
  // `kind` must be "audio" or "video".
  //
  // `stream_id` is used to populate the msid attribute; if empty, one will
  // be generated automatically.
  //
  // This method is not supported with kUnifiedPlan semantics. Please use
  // AddTransceiver instead.
  virtual scoped_refptr<RtpSenderInterface> CreateSender(
      const std::string& kind,
      const std::string& stream_id) = 0;

  // If Plan B semantics are specified, gets all RtpSenders, created either
  // through AddStream, AddTrack, or CreateSender. All senders of a specific
  // media type share the same media description.
  //
  // If Unified Plan semantics are specified, gets the RtpSender for each
  // RtpTransceiver.
  virtual std::vector<scoped_refptr<RtpSenderInterface>> GetSenders() const = 0;

  // If Plan B semantics are specified, gets all RtpReceivers created when a
  // remote description is applied. All receivers of a specific media type share
  // the same media description. It is also possible to have a media description
  // with no associated RtpReceivers, if the directional attribute does not
  // indicate that the remote peer is sending any media.
  //
  // If Unified Plan semantics are specified, gets the RtpReceiver for each
  // RtpTransceiver.
  virtual std::vector<scoped_refptr<RtpReceiverInterface>> GetReceivers()
      const = 0;

  // Get all RtpTransceivers, created either through AddTransceiver, AddTrack or
  // by a remote description applied with SetRemoteDescription.
  //
  // Note: This method is only available when Unified Plan is enabled (see
  // RTCConfiguration).
  virtual std::vector<scoped_refptr<RtpTransceiverInterface>> GetTransceivers()
      const = 0;

  // The legacy non-compliant GetStats() API. This correspond to the
  // callback-based version of getStats() in JavaScript. The returned metrics
  // are UNDOCUMENTED and many of them rely on implementation-specific details.
  // The goal is to DELETE THIS VERSION but we can't today because it is heavily
  // relied upon by third parties. See https://crbug.com/822696.
  //
  // This version is wired up into Chrome. Any stats implemented are
  // automatically exposed to the Web Platform. This has BYPASSED the Chrome
  // release processes for years and lead to cross-browser incompatibility
  // issues and web application reliance on Chrome-only behavior.
  //
  // This API is in "maintenance mode", serious regressions should be fixed but
  // adding new stats is highly discouraged.
  //
  // TODO(hbos): Deprecate and remove this when third parties have migrated to
  // the spec-compliant GetStats() API. https://crbug.com/822696
  virtual bool GetStats(StatsObserver* observer,
                        MediaStreamTrackInterface* track,  // Optional
                        StatsOutputLevel level) = 0;
  // The spec-compliant GetStats() API. This correspond to the promise-based
  // version of getStats() in JavaScript. Implementation status is described in
  // api/stats/rtcstats_objects.h. For more details on stats, see spec:
  // https://w3c.github.io/webrtc-pc/#dom-rtcpeerconnection-getstats
  // TODO(hbos): Takes shared ownership, use webrtc::scoped_refptr<> instead.
  // This requires stop overriding the current version in third party or making
  // third party calls explicit to avoid ambiguity during switch. Make the
  // future version abstract as soon as third party projects implement it.
  virtual void GetStats(RTCStatsCollectorCallback* callback) = 0;
  // Spec-compliant getStats() performing the stats selection algorithm with the
  // sender. https://w3c.github.io/webrtc-pc/#dom-rtcrtpsender-getstats
  virtual void GetStats(scoped_refptr<RtpSenderInterface> selector,
                        scoped_refptr<RTCStatsCollectorCallback> callback) = 0;
  // Spec-compliant getStats() performing the stats selection algorithm with the
  // receiver. https://w3c.github.io/webrtc-pc/#dom-rtcrtpreceiver-getstats
  virtual void GetStats(scoped_refptr<RtpReceiverInterface> selector,
                        scoped_refptr<RTCStatsCollectorCallback> callback) = 0;
  // Clear cached stats in the RTCStatsCollector.
  virtual void ClearStatsCache() {}

  // Create a data channel with the provided config, or default config if none
  // is provided. Note that an offer/answer negotiation is still necessary
  // before the data channel can be used.
  //
  // Also, calling CreateDataChannel is the only way to get a data "m=" section
  // in SDP, so it should be done before CreateOffer is called, if the
  // application plans to use data channels.
  virtual RTCErrorOr<scoped_refptr<DataChannelInterface>>
  CreateDataChannelOrError(const std::string& /* label */,
                           const DataChannelInit* /* config */) {
    return RTCError(RTCErrorType::INTERNAL_ERROR, "dummy function called");
  }
  // TODO(crbug.com/788659): Remove "virtual" below and default implementation
  // above once mock in Chrome is fixed.
  ABSL_DEPRECATED("Use CreateDataChannelOrError")
  virtual scoped_refptr<DataChannelInterface> CreateDataChannel(
      const std::string& label,
      const DataChannelInit* config) {
    auto result = CreateDataChannelOrError(label, config);
    if (!result.ok()) {
      return nullptr;
    } else {
      return result.MoveValue();
    }
  }

  // NOTE: For the following 6 methods, it's only safe to dereference the
  // SessionDescriptionInterface on signaling_thread() (for example, calling
  // ToString).

  // Returns the more recently applied description; "pending" if it exists, and
  // otherwise "current". See below.
  virtual const SessionDescriptionInterface* local_description() const = 0;
  virtual const SessionDescriptionInterface* remote_description() const = 0;

  // A "current" description the one currently negotiated from a complete
  // offer/answer exchange.
  virtual const SessionDescriptionInterface* current_local_description()
      const = 0;
  virtual const SessionDescriptionInterface* current_remote_description()
      const = 0;

  // A "pending" description is one that's part of an incomplete offer/answer
  // exchange (thus, either an offer or a pranswer). Once the offer/answer
  // exchange is finished, the "pending" description will become "current".
  virtual const SessionDescriptionInterface* pending_local_description()
      const = 0;
  virtual const SessionDescriptionInterface* pending_remote_description()
      const = 0;

  // Tells the PeerConnection that ICE should be restarted. This triggers a need
  // for negotiation and subsequent CreateOffer() calls will act as if
  // RTCOfferAnswerOptions::ice_restart is true.
  // https://w3c.github.io/webrtc-pc/#dom-rtcpeerconnection-restartice
  virtual void RestartIce() = 0;

  // Create a new offer.
  // The CreateSessionDescriptionObserver callback will be called when done.
  virtual void CreateOffer(CreateSessionDescriptionObserver* observer,
                           const RTCOfferAnswerOptions& options) = 0;

  // Create an answer to an offer.
  // The CreateSessionDescriptionObserver callback will be called when done.
  virtual void CreateAnswer(CreateSessionDescriptionObserver* observer,
                            const RTCOfferAnswerOptions& options) = 0;

  // Sets the local session description.
  //
  // According to spec, the local session description MUST be the same as was
  // returned by CreateOffer() or CreateAnswer() or else the operation should
  // fail. Our implementation however allows some amount of "SDP munging", but
  // please note that this is HIGHLY DISCOURAGED. If you do not intent to munge
  // SDP, the method below that doesn't take `desc` as an argument will create
  // the offer or answer for you.
  //
  // The observer is invoked as soon as the operation completes, which could be
  // before or after the SetLocalDescription() method has exited.
  virtual void SetLocalDescription(
      std::unique_ptr<SessionDescriptionInterface> /* desc */,
      scoped_refptr<SetLocalDescriptionObserverInterface> /* observer */) {}
  // Creates an offer or answer (depending on current signaling state) and sets
  // it as the local session description.
  //
  // The observer is invoked as soon as the operation completes, which could be
  // before or after the SetLocalDescription() method has exited.
  virtual void SetLocalDescription(
      scoped_refptr<SetLocalDescriptionObserverInterface> /* observer */) {}
  // Like SetLocalDescription() above, but the observer is invoked with a delay
  // after the operation completes. This helps avoid recursive calls by the
  // observer but also makes it possible for states to change in-between the
  // operation completing and the observer getting called. This makes them racy
  // for synchronizing peer connection states to the application.
  // TODO(https://crbug.com/webrtc/11798): Delete these methods in favor of the
  // ones taking SetLocalDescriptionObserverInterface as argument.
  virtual void SetLocalDescription(SetSessionDescriptionObserver* observer,
                                   SessionDescriptionInterface* desc) = 0;
  virtual void SetLocalDescription(
      SetSessionDescriptionObserver* /* observer */) {}

  // Sets the remote session description.
  //
  // (Unlike "SDP munging" before SetLocalDescription(), modifying a remote
  // offer or answer is allowed by the spec.)
  //
  // The observer is invoked as soon as the operation completes, which could be
  // before or after the SetRemoteDescription() method has exited.
  virtual void SetRemoteDescription(
      std::unique_ptr<SessionDescriptionInterface> desc,
      scoped_refptr<SetRemoteDescriptionObserverInterface> observer) = 0;
  // Like SetRemoteDescription() above, but the observer is invoked with a delay
  // after the operation completes. This helps avoid recursive calls by the
  // observer but also makes it possible for states to change in-between the
  // operation completing and the observer getting called. This makes them racy
  // for synchronizing peer connection states to the application.
  // TODO(https://crbug.com/webrtc/11798): Delete this method in favor of the
  // ones taking SetRemoteDescriptionObserverInterface as argument.
  virtual void SetRemoteDescription(
      SetSessionDescriptionObserver* /* observer */,
      SessionDescriptionInterface* /* desc */) {}

  // According to spec, we must only fire "negotiationneeded" if the Operations
  // Chain is empty. This method takes care of validating an event previously
  // generated with PeerConnectionObserver::OnNegotiationNeededEvent() to make
  // sure that even if there was a delay (e.g. due to a PostTask) between the
  // event being generated and the time of firing, the Operations Chain is empty
  // and the event is still valid to be fired.
  virtual bool ShouldFireNegotiationNeededEvent(uint32_t event_id) = 0;

  virtual PeerConnectionInterface::RTCConfiguration GetConfiguration() = 0;

  // Sets the PeerConnection's global configuration to `config`.
  //
  // The members of `config` that may be changed are `type`, `servers`,
  // `ice_candidate_pool_size` and `prune_turn_ports` (though the candidate
  // pool size can't be changed after the first call to SetLocalDescription).
  // Note that this means the BUNDLE and RTCP-multiplexing policies cannot be
  // changed with this method.
  //
  // Any changes to STUN/TURN servers or ICE candidate policy will affect the
  // next gathering phase, and cause the next call to createOffer to generate
  // new ICE credentials, as described in JSEP. This also occurs when
  // `prune_turn_ports` changes, for the same reasoning.
  //
  // If an error occurs, returns false and populates `error` if non-null:
  // - INVALID_MODIFICATION if `config` contains a modified parameter other
  //   than one of the parameters listed above.
  // - INVALID_RANGE if `ice_candidate_pool_size` is out of range.
  // - SYNTAX_ERROR if parsing an ICE server URL failed.
  // - INVALID_PARAMETER if a TURN server is missing `username` or `password`.
  // - INTERNAL_ERROR if an unexpected error occurred.
  virtual RTCError SetConfiguration(
      const PeerConnectionInterface::RTCConfiguration& config) = 0;

  // Provides a remote candidate to the ICE Agent.
  // A copy of the `candidate` will be created and added to the remote
  // description. So the caller of this method still has the ownership of the
  // `candidate`.
  // TODO(hbos): The spec mandates chaining this operation onto the operations
  // chain; deprecate and remove this version in favor of the callback-based
  // signature.
  virtual bool AddIceCandidate(const IceCandidateInterface* candidate) = 0;
  // TODO(hbos): Remove default implementation once implemented by downstream
  // projects.
  virtual void AddIceCandidate(
      std::unique_ptr<IceCandidateInterface> /* candidate */,
      std::function<void(RTCError)> /* callback */) {}

  // Removes a group of remote candidates from the ICE agent. Needed mainly for
  // continual gathering, to avoid an ever-growing list of candidates as
  // networks come and go. Note that the candidates' transport_name must be set
  // to the MID of the m= section that generated the candidate.
  // TODO(bugs.webrtc.org/8395): Use IceCandidateInterface instead of
  // webrtc::Candidate, which would avoid the transport_name oddity.
  virtual bool RemoveIceCandidates(
      const std::vector<Candidate>& candidates) = 0;

  // SetBitrate limits the bandwidth allocated for all RTP streams sent by
  // this PeerConnection. Other limitations might affect these limits and
  // are respected (for example "b=AS" in SDP).
  //
  // Setting `current_bitrate_bps` will reset the current bitrate estimate
  // to the provided value.
  virtual RTCError SetBitrate(const BitrateSettings& bitrate) = 0;

  // Allows an application to reconfigure bandwidth estimation.
  // The method can be called both before and after estimation has started.
  // Estimation starts when the first RTP packet is sent.
  // Estimation will be restarted if already started.
  virtual void ReconfigureBandwidthEstimation(
      const BandwidthEstimationSettings& settings) = 0;

  // Enable/disable playout of received audio streams. Enabled by default. Note
  // that even if playout is enabled, streams will only be played out if the
  // appropriate SDP is also applied. Setting `playout` to false will stop
  // playout of the underlying audio device but starts a task which will poll
  // for audio data every 10ms to ensure that audio processing happens and the
  // audio statistics are updated.
  virtual void SetAudioPlayout(bool playout) = 0;

  // Enable/disable recording of transmitted audio streams. Enabled by default.
  // Note that even if recording is enabled, streams will only be recorded if
  // the appropriate SDP is also applied.
  virtual void SetAudioRecording(bool recording) = 0;

  // Looks up the DtlsTransport associated with a MID value.
  // In the Javascript API, DtlsTransport is a property of a sender, but
  // because the PeerConnection owns the DtlsTransport in this implementation,
  // it is better to look them up on the PeerConnection.
  virtual scoped_refptr<DtlsTransportInterface> LookupDtlsTransportByMid(
      const std::string& mid) = 0;

  // Returns the SCTP transport, if any.
  virtual scoped_refptr<SctpTransportInterface> GetSctpTransport() const = 0;

  // Returns the current SignalingState.
  virtual SignalingState signaling_state() = 0;

  // Returns an aggregate state of all ICE *and* DTLS transports.
  // This is left in place to avoid breaking native clients who expect our old,
  // nonstandard behavior.
  // TODO(jonasolsson): deprecate and remove this.
  virtual IceConnectionState ice_connection_state() = 0;

  // Returns an aggregated state of all ICE transports.
  virtual IceConnectionState standardized_ice_connection_state() = 0;

  // Returns an aggregated state of all ICE and DTLS transports.
  virtual PeerConnectionState peer_connection_state() = 0;

  virtual IceGatheringState ice_gathering_state() = 0;

  // Returns the current state of canTrickleIceCandidates per
  // https://w3c.github.io/webrtc-pc/#attributes-1
  virtual std::optional<bool> can_trickle_ice_candidates() = 0;

  // When a resource is overused, the PeerConnection will try to reduce the load
  // on the sysem, for example by reducing the resolution or frame rate of
  // encoded streams. The Resource API allows injecting platform-specific usage
  // measurements. The conditions to trigger kOveruse or kUnderuse are up to the
  // implementation.
  virtual void AddAdaptationResource(scoped_refptr<Resource> resource) = 0;

  // Start RtcEventLog using an existing output-sink. Takes ownership of
  // `output` and passes it on to Call, which will take the ownership. If
  // the operation fails the output will be closed and deallocated. The
  // event log will send serialized events to the output object every
  // `output_period_ms`. Applications using the event log should generally
  // make their own trade-off regarding the output period. A long period is
  // generally more efficient, with potential drawbacks being more bursty
  // thread usage, and more events lost in case the application crashes. If
  // the `output_period_ms` argument is omitted, webrtc selects a default
  // deemed to be workable in most cases.
  virtual bool StartRtcEventLog(std::unique_ptr<RtcEventLogOutput> output,
                                int64_t output_period_ms) = 0;
  virtual bool StartRtcEventLog(std::unique_ptr<RtcEventLogOutput> output) = 0;

  // Stops logging the RtcEventLog.
  virtual void StopRtcEventLog() = 0;

  virtual void SetDataChannelEventObserver(
      std::unique_ptr<DataChannelEventObserverInterface> observer) = 0;

  // Terminates all media, closes the transports, and in general releases any
  // resources used by the PeerConnection. This is an irreversible operation.
  //
  // Note that after this method completes, the PeerConnection will no longer
  // use the PeerConnectionObserver interface passed in on construction, and
  // thus the observer object can be safely destroyed.
  virtual void Close() = 0;

  // The thread on which all PeerConnectionObserver callbacks will be invoked,
  // as well as callbacks for other classes such as DataChannelObserver.
  //
  // Also the only thread on which it's safe to use SessionDescriptionInterface
  // pointers.
  virtual Thread* signaling_thread() const = 0;

  // NetworkController instance being used by this PeerConnection, to be used
  // to identify instances when using a custom NetworkControllerFactory.
  virtual NetworkControllerInterface* GetNetworkController() = 0;

 protected:
  // Dtor protected as objects shouldn't be deleted via this interface.
  ~PeerConnectionInterface() override = default;
};

// PeerConnection callback interface, used for RTCPeerConnection events.
// Application should implement these methods.
class PeerConnectionObserver {
 public:
  virtual ~PeerConnectionObserver() = default;

  // Triggered when the SignalingState changed.
  virtual void OnSignalingChange(
      PeerConnectionInterface::SignalingState new_state) = 0;

  // Triggered when media is received on a new stream from remote peer.
  virtual void OnAddStream(scoped_refptr<MediaStreamInterface> /* stream */) {}

  // Triggered when a remote peer closes a stream.
  virtual void OnRemoveStream(
      scoped_refptr<MediaStreamInterface> /* stream */) {}

  // Triggered when a remote peer opens a data channel.
  virtual void OnDataChannel(
      scoped_refptr<DataChannelInterface> data_channel) = 0;

  // Triggered when renegotiation is needed. For example, an ICE restart
  // has begun.
  // TODO(hbos): Delete in favor of OnNegotiationNeededEvent() when downstream
  // projects have migrated.
  virtual void OnRenegotiationNeeded() {}
  // Used to fire spec-compliant onnegotiationneeded events, which should only
  // fire when the Operations Chain is empty. The observer is responsible for
  // queuing a task (e.g. Chromium: jump to main thread) to maybe fire the
  // event. The event identified using `event_id` must only fire if
  // PeerConnection::ShouldFireNegotiationNeededEvent() returns true since it is
  // possible for the event to become invalidated by operations subsequently
  // chained.
  virtual void OnNegotiationNeededEvent(uint32_t /* event_id */) {}

  // Called any time the legacy IceConnectionState changes.
  //
  // Note that our ICE states lag behind the standard slightly. The most
  // notable differences include the fact that "failed" occurs after 15
  // seconds, not 30, and this actually represents a combination ICE + DTLS
  // state, so it may be "failed" if DTLS fails while ICE succeeds.
  //
  // TODO(jonasolsson): deprecate and remove this.
  virtual void OnIceConnectionChange(
      PeerConnectionInterface::IceConnectionState /* new_state */) {}

  // Called any time the standards-compliant IceConnectionState changes.
  virtual void OnStandardizedIceConnectionChange(
      PeerConnectionInterface::IceConnectionState /* new_state */) {}

  // Called any time the PeerConnectionState changes.
  virtual void OnConnectionChange(
      PeerConnectionInterface::PeerConnectionState /* new_state */) {}

  // Called any time the IceGatheringState changes.
  virtual void OnIceGatheringChange(
      PeerConnectionInterface::IceGatheringState new_state) = 0;

  // A new ICE candidate has been gathered.
  virtual void OnIceCandidate(const IceCandidateInterface* candidate) = 0;

  // Gathering of an ICE candidate failed.
  // See https://w3c.github.io/webrtc-pc/#event-icecandidateerror
  virtual void OnIceCandidateError(const std::string& /* address */,
                                   int /* port */,
                                   const std::string& /* url */,
                                   int /* error_code */,
                                   const std::string& /* error_text */) {}

  // Ice candidates have been removed.
  // TODO(honghaiz): Make this a pure virtual method when all its subclasses
  // implement it.
  virtual void OnIceCandidatesRemoved(
      const std::vector<Candidate>& /* candidates */) {}

  // Called when the ICE connection receiving status changes.
  virtual void OnIceConnectionReceivingChange(bool /* receiving */) {}

  // Called when the selected candidate pair for the ICE connection changes.
  virtual void OnIceSelectedCandidatePairChanged(
      const CandidatePairChangeEvent& /* event */) {}

  // This is called when a receiver and its track are created.
  // TODO(zhihuang): Make this pure virtual when all subclasses implement it.
  // Note: This is called with both Plan B and Unified Plan semantics. Unified
  // Plan users should prefer OnTrack, OnAddTrack is only called as backwards
  // compatibility (and is called in the exact same situations as OnTrack).
  virtual void OnAddTrack(
      scoped_refptr<RtpReceiverInterface> /* receiver */,
      const std::vector<scoped_refptr<MediaStreamInterface>>& /* streams */) {}

  // This is called when signaling indicates a transceiver will be receiving
  // media from the remote endpoint. This is fired during a call to
  // SetRemoteDescription. The receiving track can be accessed by:
  // `transceiver->receiver()->track()` and its associated streams by
  // `transceiver->receiver()->streams()`.
  // Note: This will only be called if Unified Plan semantics are specified.
  // This behavior is specified in section 2.2.8.2.5 of the "Set the
  // RTCSessionDescription" algorithm:
  // https://w3c.github.io/webrtc-pc/#set-description
  virtual void OnTrack(
      scoped_refptr<RtpTransceiverInterface> /* transceiver */) {}

  // Called when signaling indicates that media will no longer be received on a
  // track.
  // With Plan B semantics, the given receiver will have been removed from the
  // PeerConnection and the track muted.
  // With Unified Plan semantics, the receiver will remain but the transceiver
  // will have changed direction to either sendonly or inactive.
  // https://w3c.github.io/webrtc-pc/#process-remote-track-removal
  // TODO(hbos,deadbeef): Make pure virtual when all subclasses implement it.
  virtual void OnRemoveTrack(
      scoped_refptr<RtpReceiverInterface> /* receiver */) {}

  // Called when an interesting usage is detected by WebRTC.
  // An appropriate action is to add information about the context of the
  // PeerConnection and write the event to some kind of "interesting events"
  // log function.
  // The heuristics for defining what constitutes "interesting" are
  // implementation-defined.
  virtual void OnInterestingUsage(int /* usage_pattern */) {}
};

// PeerConnectionDependencies holds all of PeerConnections dependencies.
// A dependency is distinct from a configuration as it defines significant
// executable code that can be provided by a user of the API.
//
// All new dependencies should be added as a unique_ptr to allow the
// PeerConnection object to be the definitive owner of the dependencies
// lifetime making injection safer.
struct RTC_EXPORT PeerConnectionDependencies final {
  explicit PeerConnectionDependencies(PeerConnectionObserver* observer_in);
  // This object is not copyable or assignable.
  PeerConnectionDependencies(const PeerConnectionDependencies&) = delete;
  PeerConnectionDependencies& operator=(const PeerConnectionDependencies&) =
      delete;
  // This object is only moveable.
  PeerConnectionDependencies(PeerConnectionDependencies&&);
  PeerConnectionDependencies& operator=(PeerConnectionDependencies&&) = default;
  ~PeerConnectionDependencies();
  // Mandatory dependencies
  PeerConnectionObserver* observer = nullptr;
  // Optional dependencies
  // TODO(bugs.webrtc.org/7447): remove port allocator once downstream is
  // updated. The recommended way to inject networking components is to pass a
  // PacketSocketFactory when creating the PeerConnectionFactory.
  std::unique_ptr<PortAllocator> allocator;
  // Factory for creating resolvers that look up hostnames in DNS
  std::unique_ptr<webrtc::AsyncDnsResolverFactoryInterface>
      async_dns_resolver_factory;
  std::unique_ptr<webrtc::IceTransportFactory> ice_transport_factory;
  std::unique_ptr<RTCCertificateGeneratorInterface> cert_generator;
  std::unique_ptr<SSLCertificateVerifier> tls_cert_verifier;
  std::unique_ptr<webrtc::VideoBitrateAllocatorFactory>
      video_bitrate_allocator_factory;
  // Optional network controller factory to use.
  // Overrides that set in PeerConnectionFactoryDependencies.
  std::unique_ptr<NetworkControllerFactoryInterface> network_controller_factory;

  // Optional field trials to use.
  // Overrides those from PeerConnectionFactoryDependencies.
  std::unique_ptr<FieldTrialsView> trials;
};

// PeerConnectionFactoryDependencies holds all of the PeerConnectionFactory
// dependencies. All new dependencies should be added here instead of
// overloading the function. This simplifies dependency injection and makes it
// clear which are mandatory and optional. If possible please allow the peer
// connection factory to take ownership of the dependency by adding a unique_ptr
// to this structure.
struct RTC_EXPORT PeerConnectionFactoryDependencies final {
  PeerConnectionFactoryDependencies();
  // This object is not copyable or assignable.
  PeerConnectionFactoryDependencies(const PeerConnectionFactoryDependencies&) =
      delete;
  PeerConnectionFactoryDependencies& operator=(
      const PeerConnectionFactoryDependencies&) = delete;
  // This object is only moveable.
  PeerConnectionFactoryDependencies(PeerConnectionFactoryDependencies&&);
  PeerConnectionFactoryDependencies& operator=(
      PeerConnectionFactoryDependencies&&) = default;
  ~PeerConnectionFactoryDependencies();

  // Optional dependencies
  Thread* network_thread = nullptr;
  Thread* worker_thread = nullptr;
  Thread* signaling_thread = nullptr;
  SocketFactory* socket_factory = nullptr;
  // The `packet_socket_factory` will only be used if CreatePeerConnection is
  // called without a `port_allocator`.
  std::unique_ptr<PacketSocketFactory> packet_socket_factory;
  std::unique_ptr<TaskQueueFactory> task_queue_factory;
  std::unique_ptr<RtcEventLogFactoryInterface> event_log_factory;
  std::unique_ptr<FecControllerFactoryInterface> fec_controller_factory;
  std::unique_ptr<NetworkStatePredictorFactoryInterface>
      network_state_predictor_factory;
  std::unique_ptr<NetworkControllerFactoryInterface> network_controller_factory;
  // The `network_manager` will only be used if CreatePeerConnection is called
  // without a `port_allocator`, causing the default allocator and network
  // manager to be used.
  std::unique_ptr<NetworkManager> network_manager;
  // The `network_monitor_factory` will only be used if CreatePeerConnection is
  // called without a `port_allocator`, and the above `network_manager' is null.
  std::unique_ptr<NetworkMonitorFactory> network_monitor_factory;
  std::unique_ptr<NetEqFactory> neteq_factory;
  std::unique_ptr<SctpTransportFactoryInterface> sctp_factory;
  std::unique_ptr<FieldTrialsView> trials;
  std::unique_ptr<RtpTransportControllerSendFactoryInterface>
      transport_controller_send_factory;
  // Metronome used for decoding, must be called on the worker thread.
  std::unique_ptr<Metronome> decode_metronome;
  // Metronome used for encoding, must be called on the worker thread.
  // TODO(b/304158952): Consider merging into a single metronome for all codec
  // usage.
  std::unique_ptr<Metronome> encode_metronome;

  // Media specific dependencies. Unused when `media_factory == nullptr`.
  scoped_refptr<AudioDeviceModule> adm;
  scoped_refptr<AudioEncoderFactory> audio_encoder_factory;
  scoped_refptr<AudioDecoderFactory> audio_decoder_factory;
  scoped_refptr<AudioMixer> audio_mixer;
  // TODO: bugs.webrtc.org/369904700 - Delete `audio_processing` in favor
  // of `audio_processing_builder`.
  [[deprecated]] scoped_refptr<AudioProcessing> audio_processing;
  std::unique_ptr<AudioProcessingBuilderInterface> audio_processing_builder;
  std::unique_ptr<AudioFrameProcessor> audio_frame_processor;
  std::unique_ptr<VideoEncoderFactory> video_encoder_factory;
  std::unique_ptr<VideoDecoderFactory> video_decoder_factory;

  // The `media_factory` members allows webrtc to be optionally built without
  // media support (i.e., if only being used for data channels).
  // By default media is disabled. To enable media call
  // `EnableMedia(PeerConnectionFactoryDependencies&)`. Definition of the
  // `MediaFactory` interface is a webrtc implementation detail.
  std::unique_ptr<MediaFactory> media_factory;
};

// PeerConnectionFactoryInterface is the factory interface used for creating
// PeerConnection, MediaStream and MediaStreamTrack objects.
//
// The simplest method for obtaiing one, CreatePeerConnectionFactory will
// create the required libjingle threads, socket and network manager factory
// classes for networking if none are provided, though it requires that the
// application runs a message loop on the thread that called the method (see
// explanation below)
//
// If an application decides to provide its own threads and/or implementation
// of networking classes, it should use the alternate
// CreatePeerConnectionFactory method which accepts threads as input, and use
// the CreatePeerConnection version that takes a PortAllocator as an argument.
class RTC_EXPORT PeerConnectionFactoryInterface
    : public webrtc::RefCountInterface {
 public:
  class Options {
   public:
    Options() {}

    // If set to true, created PeerConnections won't enforce any SRTP
    // requirement, allowing unsecured media. Should only be used for
    // testing/debugging.
    bool disable_encryption = false;

    // If set to true, any platform-supported network monitoring capability
    // won't be used, and instead networks will only be updated via polling.
    //
    // This only has an effect if a PeerConnection is created with the default
    // PortAllocator implementation.
    bool disable_network_monitor = false;

    // Sets the network types to ignore. For instance, calling this with
    // ADAPTER_TYPE_ETHERNET | ADAPTER_TYPE_LOOPBACK will ignore Ethernet and
    // loopback interfaces.
    int network_ignore_mask = kDefaultNetworkIgnoreMask;

    // Sets the maximum supported protocol version. The highest version
    // supported by both ends will be used for the connection, i.e. if one
    // party supports DTLS 1.0 and the other DTLS 1.2, DTLS 1.0 will be used.
    SSLProtocolVersion ssl_max_version = SSL_PROTOCOL_DTLS_12;

    // Sets crypto related options, e.g. enabled cipher suites.
    CryptoOptions crypto_options = {};
  };

  // Set the options to be used for subsequently created PeerConnections.
  virtual void SetOptions(const Options& options) = 0;

  // The preferred way to create a new peer connection. Simply provide the
  // configuration and a PeerConnectionDependencies structure.
  virtual RTCErrorOr<scoped_refptr<PeerConnectionInterface>>
  CreatePeerConnectionOrError(
      const PeerConnectionInterface::RTCConfiguration& configuration,
      PeerConnectionDependencies dependencies) = 0;

  // Returns the capabilities of an RTP sender of type `kind`.
  // If for some reason you pass in webrtc::MediaType::DATA, returns an empty
  // structure.
  virtual RtpCapabilities GetRtpSenderCapabilities(
      webrtc::MediaType kind) const = 0;

  // Returns the capabilities of an RTP receiver of type `kind`.
  // If for some reason you pass in webrtc::MediaType::DATA, returns an empty
  // structure.
  virtual RtpCapabilities GetRtpReceiverCapabilities(
      webrtc::MediaType kind) const = 0;

  virtual scoped_refptr<MediaStreamInterface> CreateLocalMediaStream(
      const std::string& stream_id) = 0;

  // Creates an AudioSourceInterface.
  // `options` decides audio processing settings.
  virtual scoped_refptr<AudioSourceInterface> CreateAudioSource(
      const AudioOptions& options) = 0;

  // Creates a new local VideoTrack. The same `source` can be used in several
  // tracks.
  virtual scoped_refptr<VideoTrackInterface> CreateVideoTrack(
      scoped_refptr<VideoTrackSourceInterface> source,
      absl::string_view label) = 0;
  ABSL_DEPRECATED("Use version with scoped_refptr")
  virtual scoped_refptr<VideoTrackInterface> CreateVideoTrack(
      const std::string& label,
      VideoTrackSourceInterface* source) {
    return CreateVideoTrack(scoped_refptr<VideoTrackSourceInterface>(source),
                            label);
  }

  // Creates an new AudioTrack. At the moment `source` can be null.
  virtual scoped_refptr<AudioTrackInterface> CreateAudioTrack(
      const std::string& label,
      AudioSourceInterface* source) = 0;

  // Starts AEC dump using existing file. Takes ownership of `file` and passes
  // it on to VoiceEngine (via other objects) immediately, which will take
  // the ownerhip. If the operation fails, the file will be closed.
  // A maximum file size in bytes can be specified. When the file size limit is
  // reached, logging is stopped automatically. If max_size_bytes is set to a
  // value <= 0, no limit will be used, and logging will continue until the
  // StopAecDump function is called.
  // TODO(webrtc:6463): Delete default implementation when downstream mocks
  // classes are updated.
  virtual bool StartAecDump(FILE* /* file */, int64_t /* max_size_bytes */) {
    return false;
  }

  // Stops logging the AEC dump.
  virtual void StopAecDump() = 0;

 protected:
  // Dtor and ctor protected as objects shouldn't be created or deleted via
  // this interface.
  PeerConnectionFactoryInterface() {}
  ~PeerConnectionFactoryInterface() override = default;
};

// CreateModularPeerConnectionFactory is implemented in the "peerconnection"
// build target, which doesn't pull in the implementations of every module
// webrtc may use.
//
// If an application knows it will only require certain modules, it can reduce
// webrtc's impact on its binary size by depending only on the "peerconnection"
// target and the modules the application requires, using
// CreateModularPeerConnectionFactory. For example, if an application
// only uses WebRTC for audio, it can pass in null pointers for the
// video-specific interfaces, and omit the corresponding modules from its
// build.
//
// If `network_thread` or `worker_thread` are null, the PeerConnectionFactory
// will create the necessary thread internally. If `signaling_thread` is null,
// the PeerConnectionFactory will use the thread on which this method is called
// as the signaling thread, wrapping it in an webrtc::Thread object if needed.
RTC_EXPORT scoped_refptr<PeerConnectionFactoryInterface>
CreateModularPeerConnectionFactory(
    PeerConnectionFactoryDependencies dependencies);

// https://w3c.github.io/webrtc-pc/#dom-rtcsignalingstate
inline constexpr absl::string_view PeerConnectionInterface::AsString(
    SignalingState state) {
  switch (state) {
    case SignalingState::kStable:
      return "stable";
    case SignalingState::kHaveLocalOffer:
      return "have-local-offer";
    case SignalingState::kHaveLocalPrAnswer:
      return "have-local-pranswer";
    case SignalingState::kHaveRemoteOffer:
      return "have-remote-offer";
    case SignalingState::kHaveRemotePrAnswer:
      return "have-remote-pranswer";
    case SignalingState::kClosed:
      return "closed";
  }
  // This cannot happen.
  // Not using "RTC_CHECK_NOTREACHED()" because AsString() is constexpr.
  return "";
}

// https://w3c.github.io/webrtc-pc/#dom-rtcicegatheringstate
inline constexpr absl::string_view PeerConnectionInterface::AsString(
    IceGatheringState state) {
  switch (state) {
    case IceGatheringState::kIceGatheringNew:
      return "new";
    case IceGatheringState::kIceGatheringGathering:
      return "gathering";
    case IceGatheringState::kIceGatheringComplete:
      return "complete";
  }
  // This cannot happen.
  // Not using "RTC_CHECK_NOTREACHED()" because AsString() is constexpr.
  return "";
}

// https://w3c.github.io/webrtc-pc/#dom-rtciceconnectionstate
inline constexpr absl::string_view PeerConnectionInterface::AsString(
    PeerConnectionState state) {
  switch (state) {
    case PeerConnectionState::kNew:
      return "new";
    case PeerConnectionState::kConnecting:
      return "connecting";
    case PeerConnectionState::kConnected:
      return "connected";
    case PeerConnectionState::kDisconnected:
      return "disconnected";
    case PeerConnectionState::kFailed:
      return "failed";
    case PeerConnectionState::kClosed:
      return "closed";
  }
  // This cannot happen.
  // Not using "RTC_CHECK_NOTREACHED()" because AsString() is constexpr.
  return "";
}

inline constexpr absl::string_view PeerConnectionInterface::AsString(
    IceConnectionState state) {
  switch (state) {
    case kIceConnectionNew:
      return "new";
    case kIceConnectionChecking:
      return "checking";
    case kIceConnectionConnected:
      return "connected";
    case kIceConnectionCompleted:
      return "completed";
    case kIceConnectionFailed:
      return "failed";
    case kIceConnectionDisconnected:
      return "disconnected";
    case kIceConnectionClosed:
      return "closed";
    case kIceConnectionMax:
      // This cannot happen.
      // Not using "RTC_CHECK_NOTREACHED()" because AsString() is constexpr.
      return "";
  }
  // This cannot happen.
  // Not using "RTC_CHECK_NOTREACHED()" because AsString() is constexpr.
  return "";
}

}  // namespace webrtc

#endif  // API_PEER_CONNECTION_INTERFACE_H_
