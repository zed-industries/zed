/*
 *  Copyright 2004 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

// P2PTransportChannel wraps up the state management of the connection between
// two P2P clients.  Clients have candidate ports for connecting, and
// connections which are combinations of candidates from each end (Alice and
// Bob each have candidates, one candidate from Alice and one candidate from
// Bob are used to make a connection, repeat to make many connections).
//
// When all of the available connections become invalid (non-writable), we
// kick off a process of determining more candidates and more connections.
//
#ifndef P2P_BASE_P2P_TRANSPORT_CHANNEL_H_
#define P2P_BASE_P2P_TRANSPORT_CHANNEL_H_

#include <stddef.h>
#include <stdint.h>

#include <functional>
#include <map>
#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "api/async_dns_resolver.h"
#include "api/candidate.h"
#include "api/ice_transport_interface.h"
#include "api/rtc_error.h"
#include "api/sequence_checker.h"
#include "api/transport/enums.h"
#include "api/transport/stun.h"
#include "logging/rtc_event_log/events/rtc_event_ice_candidate_pair_config.h"
#include "logging/rtc_event_log/ice_logger.h"
#include "p2p/base/active_ice_controller_factory_interface.h"
#include "p2p/base/active_ice_controller_interface.h"
#include "p2p/base/candidate_pair_interface.h"
#include "p2p/base/connection.h"
#include "p2p/base/ice_agent_interface.h"
#include "p2p/base/ice_controller_factory_interface.h"
#include "p2p/base/ice_switch_reason.h"
#include "p2p/base/ice_transport_internal.h"
#include "p2p/base/p2p_constants.h"
#include "p2p/base/p2p_transport_channel_ice_field_trials.h"
#include "p2p/base/port.h"
#include "p2p/base/port_allocator.h"
#include "p2p/base/port_interface.h"
#include "p2p/base/regathering_controller.h"
#include "p2p/base/stun_dictionary.h"
#include "p2p/base/transport_description.h"
#include "p2p/dtls/dtls_stun_piggyback_callbacks.h"
#include "rtc_base/async_packet_socket.h"
#include "rtc_base/checks.h"
#include "rtc_base/dscp.h"
#include "rtc_base/network/received_packet.h"
#include "rtc_base/network/sent_packet.h"
#include "rtc_base/network_route.h"
#include "rtc_base/socket.h"
#include "rtc_base/socket_address.h"
#include "rtc_base/strings/string_builder.h"
#include "rtc_base/system/rtc_export.h"
#include "rtc_base/thread.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {
class RtcEventLog;
}  // namespace webrtc

namespace webrtc {

bool IceCredentialsChanged(absl::string_view old_ufrag,
                           absl::string_view old_pwd,
                           absl::string_view new_ufrag,
                           absl::string_view new_pwd);

// Adds the port on which the candidate originated.
class RemoteCandidate : public Candidate {
 public:
  RemoteCandidate(const Candidate& c, PortInterface* origin_port)
      : Candidate(c), origin_port_(origin_port) {}

  PortInterface* origin_port() { return origin_port_; }

 private:
  PortInterface* origin_port_;
};

// P2PTransportChannel manages the candidates and connection process to keep
// two P2P clients connected to each other.
class RTC_EXPORT P2PTransportChannel : public IceTransportInternal,
                                       public IceAgentInterface {
 public:
  static std::unique_ptr<P2PTransportChannel> Create(
      absl::string_view transport_name,
      int component,
      IceTransportInit init);

  // For testing only.
  // TODO(zstein): Remove once AsyncDnsResolverFactory is required.
  P2PTransportChannel(absl::string_view transport_name,
                      int component,
                      PortAllocator* allocator,
                      const FieldTrialsView* field_trials = nullptr);

  ~P2PTransportChannel() override;

  P2PTransportChannel(const P2PTransportChannel&) = delete;
  P2PTransportChannel& operator=(const P2PTransportChannel&) = delete;

  // From TransportChannelImpl:
  IceTransportStateInternal GetState() const override;
  IceTransportState GetIceTransportState() const override;

  const std::string& transport_name() const override;
  int component() const override;
  bool writable() const override;
  bool receiving() const override;
  void SetIceRole(IceRole role) override;
  IceRole GetIceRole() const override;
  void SetIceParameters(const IceParameters& ice_params) override;
  void SetRemoteIceParameters(const IceParameters& ice_params) override;
  void SetRemoteIceMode(IceMode mode) override;
  // TODO(deadbeef): Deprecated. Remove when Chromium's
  // IceTransportChannel does not depend on this.
  void Connect() {}
  void MaybeStartGathering() override;
  IceGatheringState gathering_state() const override;
  void ResolveHostnameCandidate(const Candidate& candidate);
  void AddRemoteCandidate(const Candidate& candidate) override;
  void RemoveRemoteCandidate(const Candidate& candidate) override;
  void RemoveAllRemoteCandidates() override;
  // Sets the parameters in IceConfig. We do not set them blindly. Instead, we
  // only update the parameter if it is considered set in `config`. For example,
  // a negative value of receiving_timeout will be considered "not set" and we
  // will not use it to update the respective parameter in `config_`.
  // TODO(deadbeef): Use std::optional instead of negative values.
  void SetIceConfig(const IceConfig& config) override;
  const IceConfig& config() const override;

  // From TransportChannel:
  int SendPacket(const char* data,
                 size_t len,
                 const AsyncSocketPacketOptions& options,
                 int flags) override;
  int SetOption(Socket::Option opt, int value) override;
  bool GetOption(Socket::Option opt, int* value) override;
  int GetError() override;
  bool GetStats(IceTransportStats* ice_transport_stats) override;
  std::optional<int> GetRttEstimate() override;
  const Connection* selected_connection() const override;
  std::optional<const CandidatePair> GetSelectedCandidatePair() const override;

  // From IceAgentInterface
  void OnStartedPinging() override;
  int64_t GetLastPingSentMs() const override;
  void UpdateConnectionStates() override;
  void UpdateState() override;
  void SendPingRequest(const Connection* connection) override;
  void SwitchSelectedConnection(const Connection* connection,
                                IceSwitchReason reason) override;
  void ForgetLearnedStateForConnections(
      ArrayView<const Connection* const> connections) override;
  bool PruneConnections(
      ArrayView<const Connection* const> connections) override;

  // TODO(honghaiz): Remove this method once the reference of it in
  // Chromoting is removed.
  const Connection* best_connection() const {
    RTC_DCHECK_RUN_ON(network_thread_);
    return selected_connection_;
  }

  void set_incoming_only(bool value) {
    RTC_DCHECK_RUN_ON(network_thread_);
    incoming_only_ = value;
  }

  // Note: These are only for testing purpose.
  // `ports_` and `pruned_ports` should not be changed from outside.
  const std::vector<PortInterface*>& ports() {
    RTC_DCHECK_RUN_ON(network_thread_);
    return ports_;
  }
  const std::vector<PortInterface*>& pruned_ports() {
    RTC_DCHECK_RUN_ON(network_thread_);
    return pruned_ports_;
  }

  IceMode remote_ice_mode() const {
    RTC_DCHECK_RUN_ON(network_thread_);
    return remote_ice_mode_;
  }

  void PruneAllPorts();
  int check_receiving_interval() const;
  std::optional<NetworkRoute> network_route() const override;

  void RemoveConnection(Connection* connection);

  // Helper method used only in unittest.
  DiffServCodePoint DefaultDscpValue() const;

  // Public for unit tests.
  Connection* FindNextPingableConnection();
  void MarkConnectionPinged(Connection* conn);

  // Public for unit tests.
  ArrayView<Connection* const> connections() const;
  void RemoveConnectionForTest(Connection* connection);

  // Public for unit tests.
  PortAllocatorSession* allocator_session() const {
    RTC_DCHECK_RUN_ON(network_thread_);
    if (allocator_sessions_.empty()) {
      return nullptr;
    }
    return allocator_sessions_.back().get();
  }

  // Public for unit tests.
  const std::vector<RemoteCandidate>& remote_candidates() const {
    RTC_DCHECK_RUN_ON(network_thread_);
    return remote_candidates_;
  }

  std::string ToString() const {
    RTC_DCHECK_RUN_ON(network_thread_);
    const std::string RECEIVING_ABBREV[2] = {"_", "R"};
    const std::string WRITABLE_ABBREV[2] = {"_", "W"};
    StringBuilder ss;
    ss << "Channel[" << transport_name_ << "|" << component_ << "|"
       << RECEIVING_ABBREV[receiving_] << WRITABLE_ABBREV[writable_] << "]";
    return ss.Release();
  }

  std::optional<std::reference_wrapper<StunDictionaryWriter>>
  GetDictionaryWriter() override {
    return stun_dict_writer_;
  }

  const FieldTrialsView* field_trials() const override { return field_trials_; }

  void ResetDtlsStunPiggybackCallbacks() override;
  void SetDtlsStunPiggybackCallbacks(
      DtlsStunPiggybackCallbacks&& callbacks) override;

  // Returns the local ICE parameters.
  const IceParameters* local_ice_parameters() const override {
    RTC_DCHECK_RUN_ON(network_thread_);
    return &ice_parameters_;
  }
  // Returns the latest remote ICE parameters or nullptr if there are no remote
  // ICE parameters yet.
  const IceParameters* remote_ice_parameters() const override {
    RTC_DCHECK_RUN_ON(network_thread_);
    return remote_ice_parameters_.empty() ? nullptr
                                          : &remote_ice_parameters_.back();
  }

 private:
  P2PTransportChannel(
      absl::string_view transport_name,
      int component,
      PortAllocator* allocator,
      // DNS resolver factory
      AsyncDnsResolverFactoryInterface* async_dns_resolver_factory,
      // If the P2PTransportChannel has to delete the DNS resolver factory
      // on release, this pointer is set.
      std::unique_ptr<AsyncDnsResolverFactoryInterface>
          owned_dns_resolver_factory,
      RtcEventLog* event_log,
      IceControllerFactoryInterface* ice_controller_factory,
      ActiveIceControllerFactoryInterface* active_ice_controller_factory,
      const FieldTrialsView* field_trials);

  bool IsGettingPorts() {
    RTC_DCHECK_RUN_ON(network_thread_);
    return allocator_session()->IsGettingPorts();
  }

  // Returns true if it's possible to send packets on `connection`.
  bool ReadyToSend(const Connection* connection) const;
  bool PresumedWritable(const Connection* conn) const;
  void SendPingRequestInternal(Connection* connection);

  NetworkRoute ConfigureNetworkRoute(const Connection* conn);
  void SwitchSelectedConnectionInternal(Connection* conn,
                                        IceSwitchReason reason);
  void UpdateTransportState();
  void HandleAllTimedOut();
  void MaybeStopPortAllocatorSessions();
  void OnSelectedConnectionDestroyed() RTC_RUN_ON(network_thread_);

  // ComputeIceTransportState computes the RTCIceTransportState as described in
  // https://w3c.github.io/webrtc-pc/#dom-rtcicetransportstate. ComputeState
  // computes the value we currently export as RTCIceTransportState.
  // TODO(bugs.webrtc.org/9308): Remove ComputeState once it's no longer used.
  IceTransportStateInternal ComputeState() const;
  IceTransportState ComputeIceTransportState() const;

  bool CreateConnections(const Candidate& remote_candidate,
                         PortInterface* origin_port);
  bool CreateConnection(PortInterface* port,
                        const Candidate& remote_candidate,
                        PortInterface* origin_port);
  bool FindConnection(const Connection* connection) const;

  uint32_t GetRemoteCandidateGeneration(const Candidate& candidate);
  bool IsDuplicateRemoteCandidate(const Candidate& candidate);
  void RememberRemoteCandidate(const Candidate& remote_candidate,
                               PortInterface* origin_port);
  void PingConnection(Connection* conn);
  void AddAllocatorSession(std::unique_ptr<PortAllocatorSession> session);
  void AddConnection(Connection* connection);

  void OnPortReady(PortAllocatorSession* session, PortInterface* port);
  void OnPortsPruned(PortAllocatorSession* session,
                     const std::vector<PortInterface*>& ports);
  void OnCandidatesReady(PortAllocatorSession* session,
                         const std::vector<Candidate>& candidates);
  void OnCandidateError(PortAllocatorSession* session,
                        const IceCandidateErrorEvent& event);
  void OnCandidatesRemoved(PortAllocatorSession* session,
                           const std::vector<Candidate>& candidates);
  void OnCandidatesAllocationDone(PortAllocatorSession* session);
  void OnUnknownAddress(PortInterface* port,
                        const SocketAddress& addr,
                        ProtocolType proto,
                        IceMessage* stun_msg,
                        const std::string& remote_username,
                        bool port_muxed);
  void OnCandidateFilterChanged(uint32_t prev_filter, uint32_t cur_filter);

  // When a port is destroyed, remove it from both lists `ports_`
  // and `pruned_ports_`.
  void OnPortDestroyed(PortInterface* port);
  // When pruning a port, move it from `ports_` to `pruned_ports_`.
  // Returns true if the port is found and removed from `ports_`.
  bool PrunePort(PortInterface* port);
  void OnRoleConflict(PortInterface* port);

  void OnConnectionStateChange(Connection* connection);
  void OnReadPacket(Connection* connection, const ReceivedIpPacket& packet);
  void OnSentPacket(const SentPacketInfo& sent_packet);
  void OnReadyToSend(Connection* connection);
  void OnConnectionDestroyed(Connection* connection);

  void OnNominated(Connection* conn);

  void LogCandidatePairConfig(Connection* conn,
                              IceCandidatePairConfigType type);

  uint32_t GetNominationAttr(Connection* conn) const;
  bool GetUseCandidateAttr(Connection* conn) const;

  bool AllowedToPruneConnections() const;

  // Returns the remote IceParameters and generation that match `ufrag`
  // if found, and returns nullptr otherwise.
  const IceParameters* FindRemoteIceFromUfrag(absl::string_view ufrag,
                                              uint32_t* generation) const;
  // Returns the index of the latest remote ICE parameters, or 0 if no remote
  // ICE parameters have been received.
  uint32_t remote_ice_generation() {
    RTC_DCHECK_RUN_ON(network_thread_);
    return remote_ice_parameters_.empty()
               ? 0
               : static_cast<uint32_t>(remote_ice_parameters_.size() - 1);
  }

  // Indicates if the given local port has been pruned.
  bool IsPortPruned(const PortInterface* port) const;

  // Indicates if the given remote candidate has been pruned.
  bool IsRemoteCandidatePruned(const Candidate& cand) const;

  // Sets the writable state, signaling if necessary.
  void SetWritable(bool writable);
  // Sets the receiving state, signaling if necessary.
  void SetReceiving(bool receiving);
  // Clears the address and the related address fields of a local candidate to
  // avoid IP leakage. This is applicable in several scenarios as commented in
  // `PortAllocator::SanitizeCandidate`.
  Candidate SanitizeLocalCandidate(const Candidate& c) const;
  // Clears the address field of a remote candidate to avoid IP leakage. This is
  // applicable in the following scenarios:
  // 1. mDNS candidates are received.
  // 2. Peer-reflexive remote candidates.
  Candidate SanitizeRemoteCandidate(const Candidate& c) const;

  // Cast a Connection returned from IceController and verify that it exists.
  // (P2P owns all Connections, and only gives const pointers to IceController,
  // see IceControllerInterface).
  Connection* FromIceController(const Connection* conn) {
    // Verify that IceController does not return a connection
    // that we have destroyed.
    RTC_DCHECK(FindConnection(conn));
    return const_cast<Connection*>(conn);
  }

  int64_t ComputeEstimatedDisconnectedTimeMs(int64_t now,
                                             Connection* old_connection);

  void ParseFieldTrials(const FieldTrialsView* field_trials);

  std::string transport_name_ RTC_GUARDED_BY(network_thread_);
  int component_ RTC_GUARDED_BY(network_thread_);
  PortAllocator* allocator_ RTC_GUARDED_BY(network_thread_);
  AsyncDnsResolverFactoryInterface* const async_dns_resolver_factory_
      RTC_GUARDED_BY(network_thread_);
  const std::unique_ptr<AsyncDnsResolverFactoryInterface>
      owned_dns_resolver_factory_;
  Thread* const network_thread_;
  bool incoming_only_ RTC_GUARDED_BY(network_thread_);
  int error_ RTC_GUARDED_BY(network_thread_);
  std::vector<std::unique_ptr<PortAllocatorSession>> allocator_sessions_
      RTC_GUARDED_BY(network_thread_);
  // `ports_` contains ports that are used to form new connections when
  // new remote candidates are added.
  std::vector<PortInterface*> ports_ RTC_GUARDED_BY(network_thread_);
  // `pruned_ports_` contains ports that have been removed from `ports_` and
  // are not being used to form new connections, but that aren't yet destroyed.
  // They may have existing connections, and they still fire signals such as
  // SignalUnknownAddress.
  std::vector<PortInterface*> pruned_ports_ RTC_GUARDED_BY(network_thread_);

  Connection* selected_connection_ RTC_GUARDED_BY(network_thread_) = nullptr;
  std::vector<Connection*> connections_ RTC_GUARDED_BY(network_thread_);

  std::vector<RemoteCandidate> remote_candidates_
      RTC_GUARDED_BY(network_thread_);
  bool had_connection_ RTC_GUARDED_BY(network_thread_) =
      false;  // if connections_ has ever been nonempty
  typedef std::map<Socket::Option, int> OptionMap;
  OptionMap options_ RTC_GUARDED_BY(network_thread_);
  IceParameters ice_parameters_ RTC_GUARDED_BY(network_thread_);
  std::vector<IceParameters> remote_ice_parameters_
      RTC_GUARDED_BY(network_thread_);
  IceMode remote_ice_mode_ RTC_GUARDED_BY(network_thread_);
  IceRole ice_role_ RTC_GUARDED_BY(network_thread_);
  IceGatheringState gathering_state_ RTC_GUARDED_BY(network_thread_);
  std::unique_ptr<BasicRegatheringController> regathering_controller_
      RTC_GUARDED_BY(network_thread_);
  int64_t last_ping_sent_ms_ RTC_GUARDED_BY(network_thread_) = 0;
  int weak_ping_interval_ RTC_GUARDED_BY(network_thread_) = WEAK_PING_INTERVAL;
  // TODO(jonasolsson): Remove state_ and rename standardized_state_ once state_
  // is no longer used to compute the ICE connection state.
  IceTransportStateInternal state_ RTC_GUARDED_BY(network_thread_) =
      IceTransportStateInternal::STATE_INIT;
  IceTransportState standardized_state_ RTC_GUARDED_BY(network_thread_) =
      IceTransportState::kNew;
  IceConfig config_ RTC_GUARDED_BY(network_thread_);
  int last_sent_packet_id_ RTC_GUARDED_BY(network_thread_) =
      -1;  // -1 indicates no packet was sent before.
  // The value put in the "nomination" attribute for the next nominated
  // connection. A zero-value indicates the connection will not be nominated.
  uint32_t nomination_ RTC_GUARDED_BY(network_thread_) = 0;
  bool receiving_ RTC_GUARDED_BY(network_thread_) = false;
  bool writable_ RTC_GUARDED_BY(network_thread_) = false;
  bool has_been_writable_ RTC_GUARDED_BY(network_thread_) =
      false;  // if writable_ has ever been true

  std::optional<NetworkRoute> network_route_ RTC_GUARDED_BY(network_thread_);
  IceEventLog ice_event_log_ RTC_GUARDED_BY(network_thread_);

  std::unique_ptr<ActiveIceControllerInterface> ice_controller_
      RTC_GUARDED_BY(network_thread_);

  struct CandidateAndResolver final {
    CandidateAndResolver(const Candidate& candidate,
                         std::unique_ptr<AsyncDnsResolverInterface>&& resolver);
    ~CandidateAndResolver();
    // Moveable, but not copyable.
    CandidateAndResolver(CandidateAndResolver&&) = default;
    CandidateAndResolver& operator=(CandidateAndResolver&&) = default;

    Candidate candidate_;
    std::unique_ptr<AsyncDnsResolverInterface> resolver_;
  };
  std::vector<CandidateAndResolver> resolvers_ RTC_GUARDED_BY(network_thread_);
  void FinishAddingRemoteCandidate(const Candidate& new_remote_candidate);
  void OnCandidateResolved(AsyncDnsResolverInterface* resolver);
  void AddRemoteCandidateWithResult(Candidate candidate,
                                    const AsyncDnsResolverResult& result);

  std::unique_ptr<StunAttribute> GoogDeltaReceived(
      const StunByteStringAttribute*);
  void GoogDeltaAckReceived(RTCErrorOr<const StunUInt64Attribute*>);

  // Bytes/packets sent/received on this channel.
  uint64_t bytes_sent_ = 0;
  uint64_t bytes_received_ = 0;
  uint64_t packets_sent_ = 0;
  uint64_t packets_received_ = 0;

  // Number of times the selected_connection_ has been modified.
  uint32_t selected_candidate_pair_changes_ = 0;

  // When was last data received on a existing connection,
  // from connection->last_data_received() that uses webrtc::TimeMillis().
  int64_t last_data_received_ms_ = 0;

  // Parsed field trials.
  IceFieldTrials ice_field_trials_;
  // Unparsed field trials.
  const FieldTrialsView* field_trials_;

  // A dictionary of attributes that will be reflected to peer.
  StunDictionaryWriter stun_dict_writer_;

  // A dictionary that tracks attributes from peer.
  StunDictionaryView stun_dict_view_;

  // DTLS-STUN piggybacking callbacks.
  DtlsStunPiggybackCallbacks dtls_stun_piggyback_callbacks_;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::IceCredentialsChanged;
using ::webrtc::P2PTransportChannel;
using ::webrtc::RemoteCandidate;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // P2P_BASE_P2P_TRANSPORT_CHANNEL_H_
