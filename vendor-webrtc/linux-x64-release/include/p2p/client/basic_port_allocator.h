/*
 *  Copyright 2004 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef P2P_CLIENT_BASIC_PORT_ALLOCATOR_H_
#define P2P_CLIENT_BASIC_PORT_ALLOCATOR_H_

#include <cstdint>
#include <functional>
#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "absl/base/attributes.h"
#include "absl/base/nullability.h"
#include "absl/strings/string_view.h"
#include "api/candidate.h"
#include "api/environment/environment.h"
#include "api/field_trials_view.h"
#include "api/packet_socket_factory.h"
#include "api/task_queue/pending_task_safety_flag.h"
#include "api/transport/enums.h"
#include "api/turn_customizer.h"
#include "p2p/base/port.h"
#include "p2p/base/port_allocator.h"
#include "p2p/base/port_interface.h"
#include "p2p/base/stun_port.h"
#include "p2p/base/turn_port.h"
#include "p2p/client/relay_port_factory_interface.h"
#include "p2p/client/turn_port_factory.h"
#include "rtc_base/async_packet_socket.h"
#include "rtc_base/checks.h"
#include "rtc_base/ip_address.h"
#include "rtc_base/memory/always_valid_pointer.h"
#include "rtc_base/network.h"
#include "rtc_base/network/received_packet.h"
#include "rtc_base/socket_address.h"
#include "rtc_base/system/rtc_export.h"
#include "rtc_base/thread.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {

class RTC_EXPORT BasicPortAllocator : public PortAllocator {
 public:
  BasicPortAllocator(
      const Environment& env,
      NetworkManager* absl_nonnull network_manager,
      PacketSocketFactory* absl_nonnull socket_factory,
      TurnCustomizer* absl_nullable turn_customizer = nullptr,
      RelayPortFactoryInterface* absl_nullable relay_port_factory = nullptr);

  BasicPortAllocator(const BasicPortAllocator&) = delete;
  BasicPortAllocator& operator=(const BasicPortAllocator&) = delete;

  ~BasicPortAllocator() override;

  // Set to kDefaultNetworkIgnoreMask by default.
  void SetNetworkIgnoreMask(int network_ignore_mask) override;
  int GetNetworkIgnoreMask() const;

  NetworkManager* network_manager() const {
    CheckRunOnValidThreadIfInitialized();
    return network_manager_;
  }

  // If socket_factory() is set to NULL each PortAllocatorSession
  // creates its own socket factory.
  PacketSocketFactory* socket_factory() {
    CheckRunOnValidThreadIfInitialized();
    return socket_factory_;
  }

  PortAllocatorSession* CreateSessionInternal(
      absl::string_view content_name,
      int component,
      absl::string_view ice_ufrag,
      absl::string_view ice_pwd) override;

  // Convenience method that adds a TURN server to the configuration.
  void AddTurnServerForTesting(const RelayServerConfig& turn_server);

  RelayPortFactoryInterface* relay_port_factory() {
    CheckRunOnValidThreadIfInitialized();
    return relay_port_factory_.get();
  }

  void SetVpnList(const std::vector<NetworkMask>& vpn_list) override;

  const Environment& env() const { return env_; }

 private:
  bool MdnsObfuscationEnabled() const override;

  const Environment env_;
  NetworkManager* network_manager_;
  // Always externally-owned pointer to a socket factory.
  PacketSocketFactory* const socket_factory_;
  int network_ignore_mask_ = webrtc::kDefaultNetworkIgnoreMask;

  AlwaysValidPointer<RelayPortFactoryInterface, TurnPortFactory>
      relay_port_factory_;
};

struct PortConfiguration;
class AllocationSequence;

enum class SessionState {
  GATHERING,  // Actively allocating ports and gathering candidates.
  CLEARED,    // Current allocation process has been stopped but may start
              // new ones.
  STOPPED     // This session has completely stopped, no new allocation
              // process will be started.
};

// This class is thread-compatible and assumes it's created, operated upon and
// destroyed on the network thread.
class RTC_EXPORT BasicPortAllocatorSession : public PortAllocatorSession {
 public:
  BasicPortAllocatorSession(BasicPortAllocator* allocator,
                            absl::string_view content_name,
                            int component,
                            absl::string_view ice_ufrag,
                            absl::string_view ice_pwd);
  ~BasicPortAllocatorSession() override;

  virtual BasicPortAllocator* allocator();
  Thread* network_thread() { return network_thread_; }
  PacketSocketFactory* socket_factory() { return socket_factory_; }

  // If the new filter allows new types of candidates compared to the previous
  // filter, gathered candidates that were discarded because of not matching the
  // previous filter will be signaled if they match the new one.
  //
  // We do not perform any regathering since the port allocator flags decide
  // the type of candidates to gather and the candidate filter only controls the
  // signaling of candidates. As a result, with the candidate filter changed
  // alone, all newly allowed candidates for signaling should already be
  // gathered by the respective webrtc::Port.
  void SetCandidateFilter(uint32_t filter) override;
  void StartGettingPorts() override;
  void StopGettingPorts() override;
  void ClearGettingPorts() override;
  bool IsGettingPorts() override;
  bool IsCleared() const override;
  bool IsStopped() const override;
  // These will all be webrtc::Ports.
  std::vector<PortInterface*> ReadyPorts() const override;
  std::vector<Candidate> ReadyCandidates() const override;
  bool CandidatesAllocationDone() const override;
  void RegatherOnFailedNetworks() override;
  void GetCandidateStatsFromReadyPorts(
      CandidateStatsList* candidate_stats_list) const override;
  void SetStunKeepaliveIntervalForReadyPorts(
      const std::optional<int>& stun_keepalive_interval) override;
  void PruneAllPorts() override;
  static std::vector<const Network*> SelectIPv6Networks(
      std::vector<const Network*>& all_ipv6_networks,
      int max_ipv6_networks);

 protected:
  void UpdateIceParametersInternal() override;

  // Starts the process of getting the port configurations.
  virtual void GetPortConfigurations();

  // Adds a port configuration that is now ready.  Once we have one for each
  // network (or a timeout occurs), we will start allocating ports.
  void ConfigReady(std::unique_ptr<PortConfiguration> config);
  // TODO(bugs.webrtc.org/12840) Remove once unused in downstream projects.
  ABSL_DEPRECATED(
      "Use ConfigReady(std::unique_ptr<PortConfiguration>) instead!")
  void ConfigReady(PortConfiguration* config);

 private:
  class PortData {
   public:
    enum State {
      STATE_INPROGRESS,  // Still gathering candidates.
      STATE_COMPLETE,    // All candidates allocated and ready for process.
      STATE_ERROR,       // Error in gathering candidates.
      STATE_PRUNED       // Pruned by higher priority ports on the same network
                         // interface. Only TURN ports may be pruned.
    };

    PortData() {}
    PortData(Port* port, AllocationSequence* seq)
        : port_(port), sequence_(seq) {}

    Port* port() const { return port_; }
    AllocationSequence* sequence() const { return sequence_; }
    bool has_pairable_candidate() const { return has_pairable_candidate_; }
    State state() const { return state_; }
    bool complete() const { return state_ == STATE_COMPLETE; }
    bool error() const { return state_ == STATE_ERROR; }
    bool pruned() const { return state_ == STATE_PRUNED; }
    bool inprogress() const { return state_ == STATE_INPROGRESS; }
    // Returns true if this port is ready to be used.
    bool ready() const {
      return has_pairable_candidate_ && state_ != STATE_ERROR &&
             state_ != STATE_PRUNED;
    }
    // Sets the state to "PRUNED" and prunes the Port.
    void Prune() {
      state_ = STATE_PRUNED;
      if (port()) {
        port()->Prune();
      }
    }
    void set_has_pairable_candidate(bool has_pairable_candidate) {
      if (has_pairable_candidate) {
        RTC_DCHECK(state_ == STATE_INPROGRESS);
      }
      has_pairable_candidate_ = has_pairable_candidate;
    }
    void set_state(State state) {
      RTC_DCHECK(state != STATE_ERROR || state_ == STATE_INPROGRESS);
      state_ = state;
    }

   private:
    Port* port_ = nullptr;
    AllocationSequence* sequence_ = nullptr;
    bool has_pairable_candidate_ = false;
    State state_ = STATE_INPROGRESS;
  };

  void OnConfigReady(std::unique_ptr<PortConfiguration> config);
  void OnConfigStop();
  void AllocatePorts();
  void OnAllocate(int allocation_epoch);
  void DoAllocate(bool disable_equivalent_phases);
  void OnNetworksChanged();
  void OnAllocationSequenceObjectsCreated();
  void DisableEquivalentPhases(const Network* network,
                               PortConfiguration* config,
                               uint32_t* flags);
  void AddAllocatedPort(Port* port, AllocationSequence* seq);
  void OnCandidateReady(Port* port, const Candidate& c);
  void OnCandidateError(Port* port, const IceCandidateErrorEvent& event);
  void OnPortComplete(Port* port);
  void OnPortError(Port* port);
  void OnProtocolEnabled(AllocationSequence* seq, ProtocolType proto);
  void OnPortDestroyed(PortInterface* port);
  void MaybeSignalCandidatesAllocationDone();
  void OnPortAllocationComplete();
  PortData* FindPort(Port* port);
  std::vector<const Network*> GetNetworks();
  std::vector<const Network*> GetFailedNetworks();
  void Regather(const std::vector<const Network*>& networks,
                bool disable_equivalent_phases,
                IceRegatheringReason reason);

  bool CheckCandidateFilter(const Candidate& c) const;
  bool CandidatePairable(const Candidate& c, const Port* port) const;

  std::vector<PortData*> GetUnprunedPorts(
      const std::vector<const Network*>& networks);
  // Prunes ports and signal the remote side to remove the candidates that
  // were previously signaled from these ports.
  void PrunePortsAndRemoveCandidates(
      const std::vector<PortData*>& port_data_list);
  // Gets filtered and sanitized candidates generated from a port and
  // append to `candidates`.
  void GetCandidatesFromPort(const PortData& data,
                             std::vector<Candidate>* candidates) const;
  Port* GetBestTurnPortForNetwork(absl::string_view network_name) const;
  // Returns true if at least one TURN port is pruned.
  bool PruneTurnPorts(Port* newly_pairable_turn_port);
  bool PruneNewlyPairableTurnPort(PortData* newly_pairable_turn_port);

  BasicPortAllocator* allocator_;
  Thread* network_thread_;
  PacketSocketFactory* socket_factory_;
  bool allocation_started_;
  bool network_manager_started_;
  bool allocation_sequences_created_;
  std::vector<std::unique_ptr<PortConfiguration>> configs_;
  std::vector<AllocationSequence*> sequences_;
  std::vector<PortData> ports_;
  std::vector<IceCandidateErrorEvent> candidate_error_events_;
  uint32_t candidate_filter_ = webrtc::CF_ALL;
  // Policy on how to prune turn ports, taken from the port allocator.
  PortPrunePolicy turn_port_prune_policy_;
  SessionState state_ = SessionState::CLEARED;
  int allocation_epoch_ RTC_GUARDED_BY(network_thread_) = 0;
  ScopedTaskSafety network_safety_;

  friend class AllocationSequence;
};

// Records configuration information useful in creating ports.
// TODO(deadbeef): Rename "relay" to "turn_server" in this struct.
struct RTC_EXPORT PortConfiguration {
  // TODO(jiayl): remove `stun_address` when Chrome is updated.
  SocketAddress stun_address;
  ServerAddresses stun_servers;
  std::string username;
  std::string password;
  bool use_turn_server_as_stun_server_disabled = false;

  typedef std::vector<RelayServerConfig> RelayList;
  RelayList relays;

  PortConfiguration(const ServerAddresses& stun_servers,
                    absl::string_view username,
                    absl::string_view password,
                    const FieldTrialsView* field_trials = nullptr);

  // Returns addresses of both the explicitly configured STUN servers,
  // and TURN servers that should be used as STUN servers.
  ServerAddresses StunServers();

  // Adds another relay server, with the given ports and modifier, to the list.
  void AddRelay(const RelayServerConfig& config);

  // Determines whether the given relay server supports the given protocol.
  bool SupportsProtocol(const RelayServerConfig& relay,
                        ProtocolType type) const;
  bool SupportsProtocol(ProtocolType type) const;
  // Helper method returns the server addresses for the matching RelayType and
  // Protocol type.
  ServerAddresses GetRelayServerAddresses(ProtocolType type) const;
};

// Performs the allocation of ports, in a sequenced (timed) manner, for a given
// network and IP address.
// This class is thread-compatible.
class AllocationSequence {
 public:
  enum State {
    kInit,       // Initial state.
    kRunning,    // Started allocating ports.
    kStopped,    // Stopped from running.
    kCompleted,  // All ports are allocated.

    // kInit --> kRunning --> {kCompleted|kStopped}
  };
  // `port_allocation_complete_callback` is called when AllocationSequence is
  // done with allocating ports. This signal is useful when port allocation
  // fails which doesn't result in any candidates. Using this signal
  // BasicPortAllocatorSession can send its candidate discovery conclusion
  // signal. Without this signal, BasicPortAllocatorSession doesn't have any
  // event to trigger signal. This can also be achieved by starting a timer in
  // BPAS, but this is less deterministic.
  AllocationSequence(BasicPortAllocatorSession* session,
                     const Network* network,
                     PortConfiguration* config,
                     uint32_t flags,
                     std::function<void()> port_allocation_complete_callback);
  void Init();
  void Clear();
  void OnNetworkFailed();

  State state() const { return state_; }
  const Network* network() const { return network_; }

  bool network_failed() const { return network_failed_; }
  void set_network_failed() { network_failed_ = true; }

  // Disables the phases for a new sequence that this one already covers for an
  // equivalent network setup.
  void DisableEquivalentPhases(const Network* network,
                               PortConfiguration* config,
                               uint32_t* flags);

  // Starts and stops the sequence.  When started, it will continue allocating
  // new ports on its own timed schedule.
  void Start();
  void Stop();

 private:
  void CreateTurnPort(const RelayServerConfig& config, int relative_priority);

  typedef std::vector<ProtocolType> ProtocolList;

  void Process(int epoch);
  bool IsFlagSet(uint32_t flag) { return ((flags_ & flag) != 0); }
  void CreateUDPPorts();
  void CreateTCPPorts();
  void CreateStunPorts();
  void CreateRelayPorts();

  void OnReadPacket(AsyncPacketSocket* socket, const ReceivedIpPacket& packet);

  void OnPortDestroyed(PortInterface* port);

  BasicPortAllocatorSession* session_;
  bool network_failed_ = false;
  const Network* network_;
  // Compared with the new best IP in DisableEquivalentPhases.
  IPAddress previous_best_ip_;
  PortConfiguration* config_;
  State state_;
  uint32_t flags_;
  ProtocolList protocols_;
  std::unique_ptr<AsyncPacketSocket> udp_socket_;
  // There will be only one udp port per AllocationSequence.
  UDPPort* udp_port_;
  std::vector<Port*> relay_ports_;
  int phase_;
  std::function<void()> port_allocation_complete_callback_;
  // This counter is sampled and passed together with tasks when tasks are
  // posted. If the sampled counter doesn't match `epoch_` on reception, the
  // posted task is ignored.
  int epoch_ = 0;
  ScopedTaskSafety safety_;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::AllocationSequence;
using ::webrtc::BasicPortAllocator;
using ::webrtc::BasicPortAllocatorSession;
using ::webrtc::PortConfiguration;
using ::webrtc::SessionState;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // P2P_CLIENT_BASIC_PORT_ALLOCATOR_H_
