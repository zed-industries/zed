/*
 *  Copyright 2016 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef P2P_BASE_ICE_TRANSPORT_INTERNAL_H_
#define P2P_BASE_ICE_TRANSPORT_INTERNAL_H_

#include <stdint.h>

#include <functional>
#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "absl/functional/any_invocable.h"
#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "api/candidate.h"
#include "api/field_trials_view.h"
#include "api/peer_connection_interface.h"
#include "api/rtc_error.h"
#include "api/transport/enums.h"
#include "p2p/base/candidate_pair_interface.h"
#include "p2p/base/connection.h"
#include "p2p/base/connection_info.h"
#include "p2p/base/packet_transport_internal.h"
#include "p2p/base/port.h"
#include "p2p/base/stun_dictionary.h"
#include "p2p/base/transport_description.h"
#include "p2p/dtls/dtls_stun_piggyback_callbacks.h"
#include "rtc_base/callback_list.h"
#include "rtc_base/checks.h"
#include "rtc_base/network_constants.h"
#include "rtc_base/system/rtc_export.h"
#include "rtc_base/third_party/sigslot/sigslot.h"

namespace webrtc {

// TODO(zhihuang): Replace this with
// PeerConnectionInterface::IceConnectionState.
enum class IceTransportStateInternal {
  STATE_INIT,
  STATE_CONNECTING,  // Will enter this state once a connection is created
  STATE_COMPLETED,
  STATE_FAILED
};

enum IceConnectionState {
  kIceConnectionConnecting = 0,
  kIceConnectionFailed,
  kIceConnectionConnected,  // Writable, but still checking one or more
                            // connections
  kIceConnectionCompleted,
};

struct IceTransportStats {
  CandidateStatsList candidate_stats_list;
  ConnectionInfos connection_infos;
  // Number of times the selected candidate pair has changed
  // Initially 0 and 1 once the first candidate pair has been selected.
  // The counter is increase also when "unselecting" a connection.
  uint32_t selected_candidate_pair_changes = 0;

  // Bytes/packets sent/received.
  // note: Is not the same as sum(connection_infos.bytes_sent)
  // as connections are created and destroyed while the ICE transport
  // is alive.
  uint64_t bytes_sent = 0;
  uint64_t bytes_received = 0;
  uint64_t packets_sent = 0;
  uint64_t packets_received = 0;

  IceRole ice_role = ICEROLE_UNKNOWN;
  std::string ice_local_username_fragment;
  IceTransportState ice_state = IceTransportState::kNew;
};

typedef std::vector<Candidate> Candidates;

// TODO(deadbeef): Unify with PeerConnectionInterface::IceConnectionState
// once /talk/ and /webrtc/ are combined, and also switch to ENUM_NAME naming
// style.
enum IceGatheringState {
  kIceGatheringNew = 0,
  kIceGatheringGathering,
  kIceGatheringComplete,
};

enum ContinualGatheringPolicy {
  // All port allocator sessions will stop after a writable connection is found.
  GATHER_ONCE = 0,
  // The most recent port allocator session will keep on running.
  GATHER_CONTINUALLY,
};

// ICE Nomination mode.
enum class NominationMode {
  REGULAR,         // Nominate once per ICE restart (Not implemented yet).
  AGGRESSIVE,      // Nominate every connection except that it will behave as if
                   // REGULAR when the remote is an ICE-LITE endpoint.
  SEMI_AGGRESSIVE  // Our current implementation of the nomination algorithm.
                   // The details are described in P2PTransportChannel.
};

// Utility method that checks if various required Candidate fields are filled in
// and contain valid values. If conditions are not met, an RTCError with the
// appropriated error number and description is returned. If the configuration
// is valid RTCError::OK() is returned.
RTCError VerifyCandidate(const Candidate& cand);

// Runs through a list of webrtc::Candidate instances and calls VerifyCandidate
// for each one, stopping on the first error encounted and returning that error
// value if so. On success returns RTCError::OK().
RTCError VerifyCandidates(const Candidates& candidates);

// Information about ICE configuration.
// TODO(bugs.webrtc.org/15609): Define a public API for this.
struct RTC_EXPORT IceConfig {
  // The ICE connection receiving timeout value in milliseconds.
  std::optional<int> receiving_timeout;
  // Time interval in milliseconds to ping a backup connection when the ICE
  // channel is strongly connected.
  std::optional<int> backup_connection_ping_interval;

  ContinualGatheringPolicy continual_gathering_policy = GATHER_ONCE;

  bool gather_continually() const {
    return continual_gathering_policy == GATHER_CONTINUALLY;
  }

  // Whether we should prioritize Relay/Relay candidate when nothing
  // is writable yet.
  bool prioritize_most_likely_candidate_pairs = false;

  // Writable connections are pinged at a slower rate once stablized.
  std::optional<int> stable_writable_connection_ping_interval;

  // If set to true, this means the ICE transport should presume TURN-to-TURN
  // candidate pairs will succeed, even before a binding response is received.
  bool presume_writable_when_fully_relayed = false;

  // If true, after the ICE transport type (as the candidate filter used by the
  // port allocator) is changed such that new types of ICE candidates are
  // allowed by the new filter, e.g. from CF_RELAY to CF_ALL, candidates that
  // have been gathered by the ICE transport but filtered out and not signaled
  // to the upper layers, will be surfaced.
  bool surface_ice_candidates_on_ice_transport_type_changed = false;

  // Interval to check on all networks and to perform ICE regathering on any
  // active network having no connection on it.
  std::optional<int> regather_on_failed_networks_interval;

  // The time period in which we will not switch the selected connection
  // when a new connection becomes receiving but the selected connection is not
  // in case that the selected connection may become receiving soon.
  std::optional<int> receiving_switching_delay;

  // TODO(honghaiz): Change the default to regular nomination.
  // Default nomination mode if the remote does not support renomination.
  NominationMode default_nomination_mode = NominationMode::SEMI_AGGRESSIVE;

  // The interval in milliseconds at which ICE checks (STUN pings) will be sent
  // for a candidate pair when it is both writable and receiving (strong
  // connectivity). This parameter overrides the default value given by
  // `STRONG_PING_INTERVAL` in p2ptransport.h if set.
  std::optional<int> ice_check_interval_strong_connectivity;
  // The interval in milliseconds at which ICE checks (STUN pings) will be sent
  // for a candidate pair when it is either not writable or not receiving (weak
  // connectivity). This parameter overrides the default value given by
  // `WEAK_PING_INTERVAL` in p2ptransport.h if set.
  std::optional<int> ice_check_interval_weak_connectivity;
  // ICE checks (STUN pings) will not be sent at higher rate (lower interval)
  // than this, no matter what other settings there are.
  // Measure in milliseconds.
  //
  // Note that this parameter overrides both the above check intervals for
  // candidate pairs with strong or weak connectivity, if either of the above
  // interval is shorter than the min interval.
  std::optional<int> ice_check_min_interval;
  // The min time period for which a candidate pair must wait for response to
  // connectivity checks before it becomes unwritable. This parameter
  // overrides the default value given by `CONNECTION_WRITE_CONNECT_TIMEOUT`
  // in port.h if set, when determining the writability of a candidate pair.
  std::optional<int> ice_unwritable_timeout;

  // The min number of connectivity checks that a candidate pair must sent
  // without receiving response before it becomes unwritable. This parameter
  // overrides the default value given by `CONNECTION_WRITE_CONNECT_FAILURES` in
  // port.h if set, when determining the writability of a candidate pair.
  std::optional<int> ice_unwritable_min_checks;

  // The min time period for which a candidate pair must wait for response to
  // connectivity checks it becomes inactive. This parameter overrides the
  // default value given by `CONNECTION_WRITE_TIMEOUT` in port.h if set, when
  // determining the writability of a candidate pair.
  std::optional<int> ice_inactive_timeout;

  // The interval in milliseconds at which STUN candidates will resend STUN
  // binding requests to keep NAT bindings open.
  std::optional<int> stun_keepalive_interval;

  std::optional<AdapterType> network_preference;

  VpnPreference vpn_preference = VpnPreference::kDefault;

  // Experimental feature to transport the DTLS handshake in STUN packets.
  bool dtls_handshake_in_stun = false;

  IceConfig();
  IceConfig(int receiving_timeout_ms,
            int backup_connection_ping_interval,
            ContinualGatheringPolicy gathering_policy,
            bool prioritize_most_likely_candidate_pairs,
            int stable_writable_connection_ping_interval_ms,
            bool presume_writable_when_fully_relayed,
            int regather_on_failed_networks_interval_ms,
            int receiving_switching_delay_ms);
  // Construct an IceConfig object from an RTCConfiguration object.
  // This will check the `config` settings and set the associated IceConfig
  // member properties.
  explicit IceConfig(const PeerConnectionInterface::RTCConfiguration& config);
  ~IceConfig();

  // Checks if the current configuration values are consistent.
  RTCError IsValid() const;

  // Helper getters for parameters with implementation-specific default value.
  // By convention, parameters with default value are represented by
  // std::optional and setting a parameter to null restores its default value.
  int receiving_timeout_or_default() const;
  int backup_connection_ping_interval_or_default() const;
  int stable_writable_connection_ping_interval_or_default() const;
  int regather_on_failed_networks_interval_or_default() const;
  int receiving_switching_delay_or_default() const;
  int ice_check_interval_strong_connectivity_or_default() const;
  int ice_check_interval_weak_connectivity_or_default() const;
  int ice_check_min_interval_or_default() const;
  int ice_unwritable_timeout_or_default() const;
  int ice_unwritable_min_checks_or_default() const;
  int ice_inactive_timeout_or_default() const;
  int stun_keepalive_interval_or_default() const;
};

// IceTransportInternal is an internal abstract class that does ICE.
// Once the public interface is supported,
// (https://www.w3.org/TR/webrtc/#rtcicetransport)
// the IceTransportInterface will be split from this class.
//
// TODO(bugs.webrtc.org/15609): Define a public API for this.
class RTC_EXPORT IceTransportInternal : public PacketTransportInternal {
 public:
  IceTransportInternal();
  ~IceTransportInternal() override;

  // TODO(bugs.webrtc.org/9308): Remove GetState once all uses have been
  // migrated to GetIceTransportState.
  virtual IceTransportStateInternal GetState() const = 0;
  virtual IceTransportState GetIceTransportState() const = 0;

  virtual int component() const = 0;

  virtual IceRole GetIceRole() const = 0;

  virtual void SetIceRole(IceRole role) = 0;

  // Default implementation in order to allow downstream usage deletion.
  // TODO: bugs.webrtc.org/42224914 - Remove when all downstream overrides are
  // gone.
  virtual void SetIceTiebreaker(uint64_t /* tiebreaker */) {
    RTC_CHECK_NOTREACHED();
  }

  virtual void SetIceCredentials(absl::string_view ice_ufrag,
                                 absl::string_view ice_pwd);

  virtual void SetRemoteIceCredentials(absl::string_view ice_ufrag,
                                       absl::string_view ice_pwd);

  // TODO: bugs.webrtc.org/367395350 - Make virtual when all downstream
  // overrides are gone.
  // Returns the current local ICE parameters.
  virtual const IceParameters* local_ice_parameters() const {
    RTC_CHECK_NOTREACHED();
  }
  // Returns the latest remote ICE parameters or nullptr if there are no remote
  // ICE parameters yet.
  virtual const IceParameters* remote_ice_parameters() const {
    RTC_CHECK_NOTREACHED();
  }

  // The ufrag and pwd in `ice_params` must be set
  // before candidate gathering can start.
  virtual void SetIceParameters(const IceParameters& ice_params) = 0;

  virtual void SetRemoteIceParameters(const IceParameters& ice_params) = 0;

  virtual void SetRemoteIceMode(IceMode mode) = 0;

  virtual void SetIceConfig(const IceConfig& config) = 0;
  // Default implementation in order to allow downstream usage deletion.
  // TODO: bugs.webrtc.org/367395350 - Make virtual when all downstream
  // overrides are gone.
  virtual const IceConfig& config() const { RTC_CHECK_NOTREACHED(); }

  // Start gathering candidates if not already started, or if an ICE restart
  // occurred.
  virtual void MaybeStartGathering() = 0;

  virtual void AddRemoteCandidate(const Candidate& candidate) = 0;

  virtual void RemoveRemoteCandidate(const Candidate& candidate) = 0;

  virtual void RemoveAllRemoteCandidates() = 0;

  virtual IceGatheringState gathering_state() const = 0;

  // Returns the current stats for this connection.
  virtual bool GetStats(IceTransportStats* ice_transport_stats) = 0;

  // Returns RTT estimate over the currently active connection, or an empty
  // std::optional if there is none.
  virtual std::optional<int> GetRttEstimate() = 0;

  // TODO(qingsi): Remove this method once Chrome does not depend on it anymore.
  virtual const Connection* selected_connection() const = 0;

  // Returns the selected candidate pair, or an empty std::optional if there is
  // none.
  virtual std::optional<const CandidatePair> GetSelectedCandidatePair()
      const = 0;

  virtual std::optional<std::reference_wrapper<StunDictionaryWriter>>
  GetDictionaryWriter() {
    return std::nullopt;
  }

  void AddGatheringStateCallback(
      const void* removal_tag,
      absl::AnyInvocable<void(webrtc::IceTransportInternal*)> callback);
  void RemoveGatheringStateCallback(const void* removal_tag);

  // Handles sending and receiving of candidates.
  sigslot::signal2<IceTransportInternal*, const Candidate&>
      SignalCandidateGathered;

  void SetCandidateErrorCallback(
      absl::AnyInvocable<void(webrtc::IceTransportInternal*,
                              const webrtc::IceCandidateErrorEvent&)>
          callback) {
    RTC_DCHECK(!candidate_error_callback_);
    candidate_error_callback_ = std::move(callback);
  }

  void SetCandidatesRemovedCallback(
      absl::AnyInvocable<void(webrtc::IceTransportInternal*, const Candidates&)>
          callback) {
    RTC_DCHECK(!candidates_removed_callback_);
    candidates_removed_callback_ = std::move(callback);
  }

  // Deprecated by PacketTransportInternal::SignalNetworkRouteChanged.
  // This signal occurs when there is a change in the way that packets are
  // being routed, i.e. to a different remote location. The candidate
  // indicates where and how we are currently sending media.
  // TODO(zhihuang): Update the Chrome remoting to use the new
  // SignalNetworkRouteChanged.
  sigslot::signal2<IceTransportInternal*, const Candidate&> SignalRouteChange;

  void SetCandidatePairChangeCallback(
      absl::AnyInvocable<void(const webrtc::CandidatePairChangeEvent&)>
          callback) {
    RTC_DCHECK(!candidate_pair_change_callback_);
    candidate_pair_change_callback_ = std::move(callback);
  }

  // Invoked when there is conflict in the ICE role between local and remote
  // agents.
  sigslot::signal1<IceTransportInternal*> SignalRoleConflict;

  // Emitted whenever the transport state changed.
  // TODO(bugs.webrtc.org/9308): Remove once all uses have migrated to the new
  // IceTransportState.
  sigslot::signal1<IceTransportInternal*> SignalStateChanged;

  // Emitted whenever the new standards-compliant transport state changed.
  sigslot::signal1<IceTransportInternal*> SignalIceTransportStateChanged;

  // Invoked when the transport is being destroyed.
  sigslot::signal1<IceTransportInternal*> SignalDestroyed;

  // Invoked when remote dictionary has been updated,
  // i.e. modifications to attributes from remote ice agent has
  // reflected in our StunDictionaryView.
  template <typename F>
  void AddDictionaryViewUpdatedCallback(const void* tag, F&& callback) {
    dictionary_view_updated_callback_list_.AddReceiver(
        tag, std::forward<F>(callback));
  }
  void RemoveDictionaryViewUpdatedCallback(const void* tag) {
    dictionary_view_updated_callback_list_.RemoveReceivers(tag);
  }

  // Invoked when local dictionary has been synchronized,
  // i.e. remote ice agent has reported acknowledged updates from us.
  template <typename F>
  void AddDictionaryWriterSyncedCallback(const void* tag, F&& callback) {
    dictionary_writer_synced_callback_list_.AddReceiver(
        tag, std::forward<F>(callback));
  }
  void RemoveDictionaryWriterSyncedCallback(const void* tag) {
    dictionary_writer_synced_callback_list_.RemoveReceivers(tag);
  }

  virtual const FieldTrialsView* field_trials() const { return nullptr; }

  virtual void ResetDtlsStunPiggybackCallbacks() {}
  virtual void SetDtlsStunPiggybackCallbacks(
      DtlsStunPiggybackCallbacks&& callbacks) {}

 protected:
  void SendGatheringStateEvent() { gathering_state_callback_list_.Send(this); }

  CallbackList<IceTransportInternal*,
               const StunDictionaryView&,
               ArrayView<uint16_t>>
      dictionary_view_updated_callback_list_;
  CallbackList<IceTransportInternal*, const StunDictionaryWriter&>
      dictionary_writer_synced_callback_list_;

  CallbackList<IceTransportInternal*> gathering_state_callback_list_;

  absl::AnyInvocable<void(webrtc::IceTransportInternal*,
                          const webrtc::IceCandidateErrorEvent&)>
      candidate_error_callback_;

  absl::AnyInvocable<void(webrtc::IceTransportInternal*, const Candidates&)>
      candidates_removed_callback_;

  absl::AnyInvocable<void(const webrtc::CandidatePairChangeEvent&)>
      candidate_pair_change_callback_;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::Candidates;
using ::webrtc::ContinualGatheringPolicy;
using ::webrtc::GATHER_CONTINUALLY;
using ::webrtc::GATHER_ONCE;
using ::webrtc::IceConfig;
using ::webrtc::IceConnectionState;
using ::webrtc::IceGatheringState;
using ::webrtc::IceTransportInternal;
using ::webrtc::IceTransportStats;
using ::webrtc::kIceConnectionCompleted;
using ::webrtc::kIceConnectionConnected;
using ::webrtc::kIceConnectionConnecting;
using ::webrtc::kIceConnectionFailed;
using ::webrtc::kIceGatheringComplete;
using ::webrtc::kIceGatheringGathering;
using ::webrtc::kIceGatheringNew;
using ::webrtc::NominationMode;
using ::webrtc::VerifyCandidate;
using ::webrtc::VerifyCandidates;

using IceTransportState = ::webrtc::IceTransportStateInternal;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // P2P_BASE_ICE_TRANSPORT_INTERNAL_H_
