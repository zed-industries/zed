/*
 *  Copyright 2004 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef P2P_BASE_PORT_ALLOCATOR_H_
#define P2P_BASE_PORT_ALLOCATOR_H_

#include <stdint.h>

#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/candidate.h"
#include "api/sequence_checker.h"
#include "api/transport/enums.h"
#include "p2p/base/port.h"
#include "p2p/base/port_interface.h"
#include "p2p/base/transport_description.h"
#include "rtc_base/checks.h"
#include "rtc_base/network.h"
#include "rtc_base/socket_address.h"
#include "rtc_base/ssl_certificate.h"
#include "rtc_base/system/rtc_export.h"
#include "rtc_base/third_party/sigslot/sigslot.h"

namespace webrtc {

class TurnCustomizer;

// PortAllocator is responsible for allocating Port types for a given
// P2PSocket. It also handles port freeing.
//
// Clients can override this class to control port allocation, including
// what kinds of ports are allocated.

enum {
  // Disable local UDP ports. This doesn't impact how we connect to relay
  // servers.
  PORTALLOCATOR_DISABLE_UDP = 0x01,
  PORTALLOCATOR_DISABLE_STUN = 0x02,
  PORTALLOCATOR_DISABLE_RELAY = 0x04,
  // Disable local TCP ports. This doesn't impact how we connect to relay
  // servers.
  PORTALLOCATOR_DISABLE_TCP = 0x08,
  PORTALLOCATOR_ENABLE_IPV6 = 0x40,
  PORTALLOCATOR_ENABLE_SHARED_SOCKET = 0x100,
  PORTALLOCATOR_ENABLE_STUN_RETRANSMIT_ATTRIBUTE = 0x200,
  // When specified, we'll only allocate the STUN candidate for the public
  // interface as seen by regular http traffic and the HOST candidate associated
  // with the default local interface.
  PORTALLOCATOR_DISABLE_ADAPTER_ENUMERATION = 0x400,
  // When specified along with PORTALLOCATOR_DISABLE_ADAPTER_ENUMERATION, the
  // default local candidate mentioned above will not be allocated. Only the
  // STUN candidate will be.
  PORTALLOCATOR_DISABLE_DEFAULT_LOCAL_CANDIDATE = 0x800,
  // Disallow use of UDP when connecting to a relay server. Since proxy servers
  // usually don't handle UDP, using UDP will leak the IP address.
  PORTALLOCATOR_DISABLE_UDP_RELAY = 0x1000,

  // When multiple networks exist, do not gather candidates on the ones with
  // high cost. So if both Wi-Fi and cellular networks exist, gather only on the
  // Wi-Fi network. If a network type is "unknown", it has a cost lower than
  // cellular but higher than Wi-Fi/Ethernet. So if an unknown network exists,
  // cellular networks will not be used to gather candidates and if a Wi-Fi
  // network is present, "unknown" networks will not be usd to gather
  // candidates. Doing so ensures that even if a cellular network type was not
  // detected initially, it would not be used if a Wi-Fi network is present.
  PORTALLOCATOR_DISABLE_COSTLY_NETWORKS = 0x2000,

  // When specified, do not collect IPv6 ICE candidates on Wi-Fi.
  PORTALLOCATOR_ENABLE_IPV6_ON_WIFI = 0x4000,

  // When this flag is set, ports not bound to any specific network interface
  // will be used, in addition to normal ports bound to the enumerated
  // interfaces. Without this flag, these "any address" ports would only be
  // used when network enumeration fails or is disabled. But under certain
  // conditions, these ports may succeed where others fail, so they may allow
  // the application to work in a wider variety of environments, at the expense
  // of having to allocate additional candidates.
  PORTALLOCATOR_ENABLE_ANY_ADDRESS_PORTS = 0x8000,

  // Exclude link-local network interfaces
  // from considertaion after adapter enumeration.
  PORTALLOCATOR_DISABLE_LINK_LOCAL_NETWORKS = 0x10000,
};

// Defines various reasons that have caused ICE regathering.
enum class IceRegatheringReason {
  NETWORK_CHANGE,      // Network interfaces on the device changed
  NETWORK_FAILURE,     // Regather only on networks that have failed
  OCCASIONAL_REFRESH,  // Periodic regather on all networks
  MAX_VALUE
};

const uint32_t kDefaultPortAllocatorFlags = 0;

const uint32_t kDefaultStepDelay = 1000;  // 1 sec step delay.
// As per RFC 5245 Appendix B.1, STUN transactions need to be paced at certain
// internal. Less than 20ms is not acceptable. We choose 50ms as our default.
const uint32_t kMinimumStepDelay = 50;

// Turning on IPv6 could make many IPv6 interfaces available for connectivity
// check and delay the call setup time. kDefaultMaxIPv6Networks is the default
// upper limit of IPv6 networks but could be changed by
// set_max_ipv6_networks().
constexpr int kDefaultMaxIPv6Networks = 5;

// CF = CANDIDATE FILTER
enum : uint32_t {
  CF_NONE = 0x0,
  CF_HOST = 0x1,
  CF_REFLEXIVE = 0x2,
  CF_RELAY = 0x4,
  CF_ALL = 0x7,
};

// TLS certificate policy.
enum class TlsCertPolicy {
  // For TLS based protocols, ensure the connection is secure by not
  // circumventing certificate validation.
  TLS_CERT_POLICY_SECURE,
  // For TLS based protocols, disregard security completely by skipping
  // certificate validation. This is insecure and should never be used unless
  // security is irrelevant in that particular context.
  TLS_CERT_POLICY_INSECURE_NO_CHECK,
};

// TODO(deadbeef): Rename to TurnCredentials (and username to ufrag).
struct RelayCredentials {
  RelayCredentials() {}
  RelayCredentials(absl::string_view username, absl::string_view password)
      : username(username), password(password) {}

  bool operator==(const RelayCredentials& o) const {
    return username == o.username && password == o.password;
  }
  bool operator!=(const RelayCredentials& o) const { return !(*this == o); }

  std::string username;
  std::string password;
};

typedef std::vector<ProtocolAddress> PortList;
// TODO(deadbeef): Rename to TurnServerConfig.
struct RTC_EXPORT RelayServerConfig {
  RelayServerConfig();
  RelayServerConfig(const SocketAddress& address,
                    absl::string_view username,
                    absl::string_view password,
                    ProtocolType proto);
  RelayServerConfig(absl::string_view address,
                    int port,
                    absl::string_view username,
                    absl::string_view password,
                    ProtocolType proto);
  // Legacy constructor where "secure" and PROTO_TCP implies PROTO_TLS.
  RelayServerConfig(absl::string_view address,
                    int port,
                    absl::string_view username,
                    absl::string_view password,
                    ProtocolType proto,
                    bool secure);
  RelayServerConfig(const RelayServerConfig&);
  ~RelayServerConfig();

  bool operator==(const RelayServerConfig& o) const {
    return ports == o.ports && credentials == o.credentials;
  }
  bool operator!=(const RelayServerConfig& o) const { return !(*this == o); }

  PortList ports;
  RelayCredentials credentials;
  TlsCertPolicy tls_cert_policy = TlsCertPolicy::TLS_CERT_POLICY_SECURE;
  std::vector<std::string> tls_alpn_protocols;
  std::vector<std::string> tls_elliptic_curves;
  SSLCertificateVerifier* tls_cert_verifier = nullptr;
  std::string turn_logging_id;
};

class RTC_EXPORT PortAllocatorSession : public sigslot::has_slots<> {
 public:
  // Content name passed in mostly for logging and debugging.
  PortAllocatorSession(absl::string_view content_name,
                       int component,
                       absl::string_view ice_ufrag,
                       absl::string_view ice_pwd,
                       uint32_t flags);

  // Subclasses should clean up any ports created.
  ~PortAllocatorSession() override;

  uint32_t flags() const { return flags_; }
  void set_flags(uint32_t flags) { flags_ = flags; }
  std::string content_name() const { return content_name_; }
  int component() const { return component_; }
  const std::string& ice_ufrag() const { return ice_ufrag_; }
  const std::string& ice_pwd() const { return ice_pwd_; }
  bool pooled() const { return pooled_; }

  // Setting this filter should affect not only candidates gathered in the
  // future, but candidates already gathered and ports already "ready",
  // which would be returned by ReadyCandidates() and ReadyPorts().
  //
  // Default filter should be CF_ALL.
  virtual void SetCandidateFilter(uint32_t filter) = 0;

  // Starts gathering ports and ICE candidates.
  virtual void StartGettingPorts() = 0;
  // Completely stops gathering. Will not gather again unless StartGettingPorts
  // is called again.
  virtual void StopGettingPorts() = 0;
  // Whether the session is actively getting ports.
  virtual bool IsGettingPorts() = 0;

  //
  // NOTE: The group of methods below is only used for continual gathering.
  //

  // ClearGettingPorts should have the same immediate effect as
  // StopGettingPorts, but if the implementation supports continual gathering,
  // ClearGettingPorts allows additional ports/candidates to be gathered if the
  // network conditions change.
  virtual void ClearGettingPorts() = 0;
  // Whether it is in the state where the existing gathering process is stopped,
  // but new ones may be started (basically after calling ClearGettingPorts).
  virtual bool IsCleared() const;
  // Whether the session has completely stopped.
  virtual bool IsStopped() const;
  // Re-gathers candidates on networks that do not have any connections. More
  // precisely, a network interface may have more than one IP addresses (e.g.,
  // IPv4 and IPv6 addresses). Each address subnet will be used to create a
  // network. Only if all networks of an interface have no connection, the
  // implementation should start re-gathering on all networks of that interface.
  virtual void RegatherOnFailedNetworks() {}
  // Get candidate-level stats from all candidates on the ready ports and return
  // the stats to the given list.
  virtual void GetCandidateStatsFromReadyPorts(
      CandidateStatsList* /* candidate_stats_list */) const {}
  // Set the interval at which STUN candidates will resend STUN binding requests
  // on the underlying ports to keep NAT bindings open.
  // The default value of the interval in implementation is restored if a null
  // optional value is passed.
  virtual void SetStunKeepaliveIntervalForReadyPorts(
      const std::optional<int>& /* stun_keepalive_interval */) {}
  // Another way of getting the information provided by the signals below.
  //
  // Ports and candidates are not guaranteed to be in the same order as the
  // signals were emitted in.
  virtual std::vector<PortInterface*> ReadyPorts() const = 0;
  virtual std::vector<Candidate> ReadyCandidates() const = 0;
  virtual bool CandidatesAllocationDone() const = 0;
  // Marks all ports in the current session as "pruned" so that they may be
  // destroyed if no connection is using them.
  virtual void PruneAllPorts() {}

  sigslot::signal2<PortAllocatorSession*, PortInterface*> SignalPortReady;
  // Fires this signal when the network of the ports failed (either because the
  // interface is down, or because there is no connection on the interface),
  // or when TURN ports are pruned because a higher-priority TURN port becomes
  // ready(pairable).
  sigslot::signal2<PortAllocatorSession*, const std::vector<PortInterface*>&>
      SignalPortsPruned;
  sigslot::signal2<PortAllocatorSession*, const std::vector<Candidate>&>
      SignalCandidatesReady;
  sigslot::signal2<PortAllocatorSession*, const IceCandidateErrorEvent&>
      SignalCandidateError;
  // Candidates should be signaled to be removed when the port that generated
  // the candidates is removed.
  sigslot::signal2<PortAllocatorSession*, const std::vector<Candidate>&>
      SignalCandidatesRemoved;
  sigslot::signal1<PortAllocatorSession*> SignalCandidatesAllocationDone;

  sigslot::signal2<PortAllocatorSession*, IceRegatheringReason>
      SignalIceRegathering;

  virtual uint32_t generation();
  virtual void set_generation(uint32_t generation);

 protected:
  // This method is called when a pooled session (which doesn't have these
  // properties initially) is returned by PortAllocator::TakePooledSession,
  // and the content name, component, and ICE ufrag/pwd are updated.
  //
  // A subclass may need to override this method to perform additional actions,
  // such as applying the updated information to ports and candidates.
  virtual void UpdateIceParametersInternal() {}

  // TODO(deadbeef): Get rid of these when everyone switches to ice_ufrag and
  // ice_pwd.
  const std::string& username() const { return ice_ufrag_; }
  const std::string& password() const { return ice_pwd_; }

 private:
  void SetIceParameters(absl::string_view content_name,
                        int component,
                        absl::string_view ice_ufrag,
                        absl::string_view ice_pwd) {
    content_name_ = std::string(content_name);
    component_ = component;
    ice_ufrag_ = std::string(ice_ufrag);
    ice_pwd_ = std::string(ice_pwd);
    UpdateIceParametersInternal();
  }

  void set_pooled(bool value) { pooled_ = value; }

  uint32_t flags_;
  uint32_t generation_;
  std::string content_name_;
  int component_;
  std::string ice_ufrag_;
  std::string ice_pwd_;

  bool pooled_ = false;

  // SetIceParameters is an implementation detail which only PortAllocator
  // should be able to call.
  friend class PortAllocator;
};

// Every method of PortAllocator (including the destructor) must be called on
// the same thread after Initialize is called.
//
// This allows a PortAllocator subclass to be constructed and configured on one
// thread, and passed into an object that uses it on a different thread.
class RTC_EXPORT PortAllocator : public sigslot::has_slots<> {
 public:
  PortAllocator();
  ~PortAllocator() override;

  // This MUST be called on the PortAllocator's thread after finishing
  // constructing and configuring the PortAllocator subclasses.
  virtual void Initialize();

  // Set to true if some Ports need to know the ICE credentials when they are
  // created. This will ensure that the PortAllocator will only match pooled
  // allocator sessions to the ICE transport with the same credentials.
  virtual void set_restrict_ice_credentials_change(bool value);

  // Set STUN and TURN servers to be used in future sessions, and set
  // candidate pool size, as described in JSEP.
  //
  // If the servers are changing, and the candidate pool size is nonzero, and
  // FreezeCandidatePool hasn't been called, existing pooled sessions will be
  // destroyed and new ones created.
  //
  // If the servers are not changing but the candidate pool size is, and
  // FreezeCandidatePool hasn't been called, pooled sessions will be either
  // created or destroyed as necessary.
  //
  // Returns true if the configuration could successfully be changed.
  // Deprecated
  bool SetConfiguration(const ServerAddresses& stun_servers,
                        const std::vector<RelayServerConfig>& turn_servers,
                        int candidate_pool_size,
                        bool prune_turn_ports,
                        TurnCustomizer* turn_customizer = nullptr,
                        const std::optional<int>&
                            stun_candidate_keepalive_interval = std::nullopt);
  bool SetConfiguration(const ServerAddresses& stun_servers,
                        const std::vector<RelayServerConfig>& turn_servers,
                        int candidate_pool_size,
                        PortPrunePolicy turn_port_prune_policy,
                        TurnCustomizer* turn_customizer = nullptr,
                        const std::optional<int>&
                            stun_candidate_keepalive_interval = std::nullopt);

  const ServerAddresses& stun_servers() const {
    CheckRunOnValidThreadIfInitialized();
    return stun_servers_;
  }

  const std::vector<RelayServerConfig>& turn_servers() const {
    CheckRunOnValidThreadIfInitialized();
    return turn_servers_;
  }

  int candidate_pool_size() const {
    CheckRunOnValidThreadIfInitialized();
    return candidate_pool_size_;
  }

  const std::optional<int>& stun_candidate_keepalive_interval() const {
    CheckRunOnValidThreadIfInitialized();
    return stun_candidate_keepalive_interval_;
  }

  // Sets the network types to ignore.
  // Values are defined by the AdapterType enum.
  // For instance, calling this with
  // ADAPTER_TYPE_ETHERNET | ADAPTER_TYPE_LOOPBACK will ignore Ethernet and
  // loopback interfaces.
  virtual void SetNetworkIgnoreMask(int network_ignore_mask) = 0;

  // Set whether VPN connections should be preferred, avoided, mandated or
  // blocked.
  virtual void SetVpnPreference(VpnPreference preference) {
    vpn_preference_ = preference;
  }

  // Set list of <ipaddress, mask> that shall be categorized as VPN.
  // Implemented by BasicPortAllocator.
  virtual void SetVpnList(const std::vector<NetworkMask>& /* vpn_list */) {}

  std::unique_ptr<PortAllocatorSession> CreateSession(
      absl::string_view content_name,
      int component,
      absl::string_view ice_ufrag,
      absl::string_view ice_pwd);

  // Get an available pooled session and set the transport information on it.
  //
  // Caller takes ownership of the returned session.
  //
  // If restrict_ice_credentials_change is TRUE, then it will only
  //   return a pooled session with matching ice credentials.
  // If no pooled sessions are available, returns null.
  std::unique_ptr<PortAllocatorSession> TakePooledSession(
      absl::string_view content_name,
      int component,
      absl::string_view ice_ufrag,
      absl::string_view ice_pwd);

  // Returns the next session that would be returned by TakePooledSession
  // optionally restricting it to sessions with specified ice credentials.
  const PortAllocatorSession* GetPooledSession(
      const IceParameters* ice_credentials = nullptr) const;

  // Discard any remaining pooled sessions.
  void DiscardCandidatePool();

  // Clears the address and the related address fields of a local candidate to
  // avoid IP leakage. This is applicable in several scenarios:
  // 1. Sanitization is configured via the candidate filter.
  // 2. Sanitization is configured via the port allocator flags.
  // 3. mDNS concealment of private IPs is enabled.
  Candidate SanitizeCandidate(const Candidate& c) const;

  uint64_t ice_tiebreaker() const { return tiebreaker_; }

  uint32_t flags() const {
    CheckRunOnValidThreadIfInitialized();
    return flags_;
  }

  void set_flags(uint32_t flags) {
    CheckRunOnValidThreadIfInitialized();
    flags_ = flags;
  }

  // Gets/Sets the port range to use when choosing client ports.
  int min_port() const {
    CheckRunOnValidThreadIfInitialized();
    return min_port_;
  }

  int max_port() const {
    CheckRunOnValidThreadIfInitialized();
    return max_port_;
  }

  bool SetPortRange(int min_port, int max_port) {
    CheckRunOnValidThreadIfInitialized();
    if (min_port > max_port) {
      return false;
    }

    min_port_ = min_port;
    max_port_ = max_port;
    return true;
  }

  // Can be used to change the default numer of IPv6 network interfaces used
  // (5). Can set to INT_MAX to effectively disable the limit.
  //
  // TODO(deadbeef): Applications shouldn't have to arbitrarily limit the
  // number of available IPv6 network interfaces just because they could slow
  // ICE down. We should work on making our ICE logic smarter (for example,
  // prioritizing pinging connections that are most likely to work) so that
  // every network interface can be used without impacting ICE's speed.
  void set_max_ipv6_networks(int networks) {
    CheckRunOnValidThreadIfInitialized();
    max_ipv6_networks_ = networks;
  }

  int max_ipv6_networks() {
    CheckRunOnValidThreadIfInitialized();
    return max_ipv6_networks_;
  }

  // Delay between different candidate gathering phases (UDP, TURN, TCP).
  // Defaults to 1 second, but PeerConnection sets it to 50ms.
  // TODO(deadbeef): Get rid of this. Its purpose is to avoid sending too many
  // STUN transactions at once, but that's already happening if you configure
  // multiple STUN servers or have multiple network interfaces. We should
  // implement some global pacing logic instead if that's our goal.
  uint32_t step_delay() const {
    CheckRunOnValidThreadIfInitialized();
    return step_delay_;
  }

  void set_step_delay(uint32_t delay) {
    CheckRunOnValidThreadIfInitialized();
    step_delay_ = delay;
  }

  bool allow_tcp_listen() const {
    CheckRunOnValidThreadIfInitialized();
    return allow_tcp_listen_;
  }

  void set_allow_tcp_listen(bool allow_tcp_listen) {
    CheckRunOnValidThreadIfInitialized();
    allow_tcp_listen_ = allow_tcp_listen;
  }

  uint32_t candidate_filter() {
    CheckRunOnValidThreadIfInitialized();
    return candidate_filter_;
  }

  // The new filter value will be populated to future allocation sessions, when
  // they are created via CreateSession, and also pooled sessions when one is
  // taken via TakePooledSession.
  //
  // A change in the candidate filter also fires a signal
  // `SignalCandidateFilterChanged`, so that objects subscribed to this signal
  // can, for example, update the candidate filter for sessions created by this
  // allocator and already taken by the object.
  //
  // Specifically for the session taken by the ICE transport, we currently do
  // not support removing candidate pairs formed with local candidates from this
  // session that are disabled by the new candidate filter.
  void SetCandidateFilter(uint32_t filter);
  // Deprecated.
  // TODO(qingsi): Remove this after Chromium migrates to the new method.
  void set_candidate_filter(uint32_t filter) { SetCandidateFilter(filter); }

  // Deprecated (by the next method).
  bool prune_turn_ports() const {
    CheckRunOnValidThreadIfInitialized();
    return turn_port_prune_policy_ == webrtc::PRUNE_BASED_ON_PRIORITY;
  }

  PortPrunePolicy turn_port_prune_policy() const {
    CheckRunOnValidThreadIfInitialized();
    return turn_port_prune_policy_;
  }

  TurnCustomizer* turn_customizer() {
    CheckRunOnValidThreadIfInitialized();
    return turn_customizer_;
  }

  // Collect candidate stats from pooled allocator sessions. This can be used to
  // collect candidate stats without creating an offer/answer or setting local
  // description. After the local description is set, the ownership of the
  // pooled session is taken by P2PTransportChannel, and the
  // candidate stats can be collected from P2PTransportChannel::GetStats.
  virtual void GetCandidateStatsFromPooledSessions(
      CandidateStatsList* candidate_stats_list);

  // Return IceParameters of the pooled sessions.
  std::vector<IceParameters> GetPooledIceCredentials();

  // Fired when `candidate_filter_` changes.
  sigslot::signal2<uint32_t /* prev_filter */, uint32_t /* cur_filter */>
      SignalCandidateFilterChanged;

 protected:
  // TODO(webrtc::13579): Remove std::string version once downstream users have
  // migrated to the absl::string_view version.
  virtual PortAllocatorSession* CreateSessionInternal(
      absl::string_view content_name,
      int component,
      absl::string_view ice_ufrag,
      absl::string_view ice_pwd) = 0;

  const std::vector<std::unique_ptr<PortAllocatorSession>>& pooled_sessions() {
    return pooled_sessions_;
  }

  // Returns true if there is an mDNS responder attached to the network manager.
  virtual bool MdnsObfuscationEnabled() const { return false; }

  // The following thread checks are only done in DCHECK for the consistency
  // with the exsiting thread checks.
  void CheckRunOnValidThreadIfInitialized() const {
    RTC_DCHECK(!initialized_ || thread_checker_.IsCurrent());
  }

  void CheckRunOnValidThreadAndInitialized() const {
    RTC_DCHECK(initialized_ && thread_checker_.IsCurrent());
  }

  bool initialized_ = false;
  uint32_t flags_;
  int min_port_;
  int max_port_;
  int max_ipv6_networks_;
  uint32_t step_delay_;
  bool allow_tcp_listen_;
  uint32_t candidate_filter_;
  std::string origin_;
  SequenceChecker thread_checker_;
  VpnPreference vpn_preference_ = VpnPreference::kDefault;

 private:
  ServerAddresses stun_servers_;
  std::vector<RelayServerConfig> turn_servers_;
  int candidate_pool_size_ = 0;  // Last value passed into SetConfiguration.
  std::vector<std::unique_ptr<PortAllocatorSession>> pooled_sessions_;
  PortPrunePolicy turn_port_prune_policy_ = webrtc::NO_PRUNE;

  // Customizer for TURN messages.
  // The instance is owned by application and will be shared among
  // all TurnPort(s) created.
  TurnCustomizer* turn_customizer_ = nullptr;

  std::optional<int> stun_candidate_keepalive_interval_;

  // If true, TakePooledSession() will only return sessions that has same ice
  // credentials as requested.
  bool restrict_ice_credentials_change_ = false;

  // Returns iterator to pooled session with specified ice_credentials or first
  // if ice_credentials is nullptr.
  std::vector<std::unique_ptr<PortAllocatorSession>>::const_iterator
  FindPooledSession(const IceParameters* ice_credentials = nullptr) const;

  // ICE tie breaker.
  uint64_t tiebreaker_;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::CF_ALL;
using ::webrtc::CF_HOST;
using ::webrtc::CF_NONE;
using ::webrtc::CF_REFLEXIVE;
using ::webrtc::CF_RELAY;
using ::webrtc::IceRegatheringReason;
using ::webrtc::kDefaultMaxIPv6Networks;
using ::webrtc::kDefaultPortAllocatorFlags;
using ::webrtc::kDefaultStepDelay;
using ::webrtc::kMinimumStepDelay;
using ::webrtc::PortAllocator;
using ::webrtc::PORTALLOCATOR_DISABLE_ADAPTER_ENUMERATION;
using ::webrtc::PORTALLOCATOR_DISABLE_COSTLY_NETWORKS;
using ::webrtc::PORTALLOCATOR_DISABLE_DEFAULT_LOCAL_CANDIDATE;
using ::webrtc::PORTALLOCATOR_DISABLE_LINK_LOCAL_NETWORKS;
using ::webrtc::PORTALLOCATOR_DISABLE_RELAY;
using ::webrtc::PORTALLOCATOR_DISABLE_STUN;
using ::webrtc::PORTALLOCATOR_DISABLE_TCP;
using ::webrtc::PORTALLOCATOR_DISABLE_UDP;
using ::webrtc::PORTALLOCATOR_DISABLE_UDP_RELAY;
using ::webrtc::PORTALLOCATOR_ENABLE_ANY_ADDRESS_PORTS;
using ::webrtc::PORTALLOCATOR_ENABLE_IPV6;
using ::webrtc::PORTALLOCATOR_ENABLE_IPV6_ON_WIFI;
using ::webrtc::PORTALLOCATOR_ENABLE_SHARED_SOCKET;
using ::webrtc::PORTALLOCATOR_ENABLE_STUN_RETRANSMIT_ATTRIBUTE;
using ::webrtc::PortAllocatorSession;
using ::webrtc::PortList;
using ::webrtc::RelayCredentials;
using ::webrtc::RelayServerConfig;
using ::webrtc::TlsCertPolicy;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // P2P_BASE_PORT_ALLOCATOR_H_
