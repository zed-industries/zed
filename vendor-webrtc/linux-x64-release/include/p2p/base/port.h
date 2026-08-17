/*
 *  Copyright 2004 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef P2P_BASE_PORT_H_
#define P2P_BASE_PORT_H_

#include <stddef.h>
#include <stdint.h>

#include <functional>
#include <map>
#include <memory>
#include <optional>
#include <set>
#include <string>
#include <utility>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/candidate.h"
#include "api/environment/environment.h"
#include "api/field_trials_view.h"
#include "api/packet_socket_factory.h"
#include "api/sequence_checker.h"
#include "api/task_queue/task_queue_base.h"
#include "api/transport/stun.h"
#include "p2p/base/candidate_pair_interface.h"
#include "p2p/base/connection.h"
#include "p2p/base/p2p_constants.h"  // IWYU pragma: keep
#include "p2p/base/port_interface.h"
#include "p2p/base/transport_description.h"
#include "rtc_base/async_packet_socket.h"
#include "rtc_base/callback_list.h"
#include "rtc_base/dscp.h"
#include "rtc_base/network.h"
#include "rtc_base/network/received_packet.h"
#include "rtc_base/network/sent_packet.h"
#include "rtc_base/socket_address.h"
#include "rtc_base/system/rtc_export.h"
#include "rtc_base/third_party/sigslot/sigslot.h"
#include "rtc_base/thread_annotations.h"
#include "rtc_base/weak_ptr.h"

namespace webrtc {

// RFC 6544, TCP candidate encoding rules.
extern const int DISCARD_PORT;
extern const char TCPTYPE_ACTIVE_STR[];
extern const char TCPTYPE_PASSIVE_STR[];
extern const char TCPTYPE_SIMOPEN_STR[];

enum class MdnsNameRegistrationStatus {
  // IP concealment with mDNS is not enabled or the name registration process is
  // not started yet.
  kNotStarted,
  // A request to create and register an mDNS name for a local IP address of a
  // host candidate is sent to the mDNS responder.
  kInProgress,
  // The name registration is complete and the created name is returned by the
  // mDNS responder.
  kCompleted,
};

// Stats that we can return about the port of a STUN candidate.
class StunStats {
 public:
  StunStats() = default;
  StunStats(const StunStats&) = default;
  ~StunStats() = default;

  StunStats& operator=(const StunStats& other) = default;

  int stun_binding_requests_sent = 0;
  int stun_binding_responses_received = 0;
  double stun_binding_rtt_ms_total = 0;
  double stun_binding_rtt_ms_squared_total = 0;
};

// Stats that we can return about a candidate.
class CandidateStats {
 public:
  CandidateStats() = default;
  CandidateStats(const CandidateStats&) = default;
  CandidateStats(CandidateStats&&) = default;
  CandidateStats(Candidate candidate,
                 std::optional<StunStats> stats = std::nullopt)
      : candidate_(std::move(candidate)), stun_stats_(std::move(stats)) {}
  ~CandidateStats() = default;

  CandidateStats& operator=(const CandidateStats& other) = default;

  const Candidate& candidate() const { return candidate_; }

  const std::optional<StunStats>& stun_stats() const { return stun_stats_; }

 private:
  Candidate candidate_;
  // STUN port stats if this candidate is a STUN candidate.
  std::optional<StunStats> stun_stats_;
};

typedef std::vector<CandidateStats> CandidateStatsList;

const char* ProtoToString(ProtocolType proto);
std::optional<ProtocolType> StringToProto(absl::string_view proto_name);

struct ProtocolAddress {
  SocketAddress address;
  ProtocolType proto;

  ProtocolAddress(const SocketAddress& a, ProtocolType p)
      : address(a), proto(p) {}

  bool operator==(const ProtocolAddress& o) const {
    return address == o.address && proto == o.proto;
  }
  bool operator!=(const ProtocolAddress& o) const { return !(*this == o); }
};

struct IceCandidateErrorEvent {
  IceCandidateErrorEvent() = default;
  IceCandidateErrorEvent(absl::string_view address,
                         int port,
                         absl::string_view url,
                         int error_code,
                         absl::string_view error_text)
      : address(std::move(address)),
        port(port),
        url(std::move(url)),
        error_code(error_code),
        error_text(std::move(error_text)) {}

  std::string address;
  int port = 0;
  std::string url;
  int error_code = 0;
  std::string error_text;
};

struct CandidatePairChangeEvent {
  CandidatePair selected_candidate_pair;
  int64_t last_data_received_ms;
  std::string reason;
  // How long do we estimate that we've been disconnected.
  int64_t estimated_disconnected_time_ms;
};

typedef std::set<SocketAddress> ServerAddresses;

// Represents a local communication mechanism that can be used to create
// connections to similar mechanisms of the other client.  Subclasses of this
// one add support for specific mechanisms like local UDP ports.
class RTC_EXPORT Port : public PortInterface, public sigslot::has_slots<> {
 public:
  // A struct containing common arguments to creating a port. See also
  // CreateRelayPortArgs.
  struct PortParametersRef {
    Environment env;
    TaskQueueBase* network_thread;
    PacketSocketFactory* socket_factory;
    const ::webrtc::Network* network;
    absl::string_view ice_username_fragment;
    absl::string_view ice_password;
  };

 protected:
  // Constructors for use only by via constructors in derived classes.
  Port(const PortParametersRef& args, IceCandidateType type);
  Port(const PortParametersRef& args,
       IceCandidateType type,
       uint16_t min_port,
       uint16_t max_port,
       bool shared_socket = false);

 public:
  ~Port() override;

  // Note that the port type does NOT uniquely identify different subclasses of
  // Port. Use the 2-tuple of the port type AND the protocol (GetProtocol()) to
  // uniquely identify subclasses. Whenever a new subclass of Port introduces a
  // conflict in the value of the 2-tuple, make sure that the implementation
  // that relies on this 2-tuple for RTTI is properly changed.
  IceCandidateType Type() const override;
  const ::webrtc::Network* Network() const override;

  // Methods to set/get ICE role and tiebreaker values.
  IceRole GetIceRole() const override;
  void SetIceRole(IceRole role) override;

  void SetIceTiebreaker(uint64_t tiebreaker) override;
  uint64_t IceTiebreaker() const override;

  bool SharedSocket() const override;
  void ResetSharedSocket() { shared_socket_ = false; }

  // Should not destroy the port even if no connection is using it. Called when
  // a port is ready to use.
  void KeepAliveUntilPruned();
  // Allows a port to be destroyed if no connection is using it.
  void Prune();

  // Call to stop any currently pending operations from running.
  void CancelPendingTasks();

  // The thread on which this port performs its I/O.
  TaskQueueBase* thread() override { return thread_; }

  // The factory used to create the sockets of this port.
  PacketSocketFactory* socket_factory() const override { return factory_; }

  // For debugging purposes.
  const std::string& content_name() const override { return content_name_; }
  void set_content_name(absl::string_view content_name) {
    content_name_ = std::string(content_name);
  }

  int component() const { return component_; }
  void set_component(int component) { component_ = component; }

  bool send_retransmit_count_attribute() const override {
    return send_retransmit_count_attribute_;
  }
  void set_send_retransmit_count_attribute(bool enable) {
    send_retransmit_count_attribute_ = enable;
  }

  // Identifies the generation that this port was created in.
  uint32_t generation() const override { return generation_; }
  void set_generation(uint32_t generation) override {
    generation_ = generation;
  }

  const std::string& username_fragment() const;
  const std::string& password() const { return password_; }

  // May be called when this port was initially created by a pooled
  // PortAllocatorSession, and is now being assigned to an ICE transport.
  // Updates the information for candidates as well.
  void SetIceParameters(int component,
                        absl::string_view username_fragment,
                        absl::string_view password);

  // Fired when candidates are discovered by the port. When all candidates
  // are discovered that belong to port SignalAddressReady is fired.
  sigslot::signal2<Port*, const Candidate&> SignalCandidateReady;
  // Provides all of the above information in one handy object.
  const std::vector<Candidate>& Candidates() const override;
  // Fired when candidate discovery failed using certain server.
  sigslot::signal2<Port*, const IceCandidateErrorEvent&> SignalCandidateError;

  // SignalPortComplete is sent when port completes the task of candidates
  // allocation.
  sigslot::signal1<Port*> SignalPortComplete;
  // This signal sent when port fails to allocate candidates and this port
  // can't be used in establishing the connections. When port is in shared mode
  // and port fails to allocate one of the candidates, port shouldn't send
  // this signal as other candidates might be usefull in establishing the
  // connection.
  sigslot::signal1<Port*> SignalPortError;

  void SubscribePortDestroyed(
      std::function<void(PortInterface*)> callback) override;
  void SendPortDestroyed(Port* port);
  // Returns a map containing all of the connections of this port, keyed by the
  // remote address.
  typedef std::map<SocketAddress, Connection*> AddressMap;
  const AddressMap& connections() { return connections_; }

  // Returns the connection to the given address or NULL if none exists.
  Connection* GetConnection(const SocketAddress& remote_addr) override;

  // Removes and deletes a connection object. `DestroyConnection` will
  // delete the connection object directly whereas `DestroyConnectionAsync`
  // defers the `delete` operation to when the call stack has been unwound.
  // Async may be needed when deleting a connection object from within a
  // callback.
  void DestroyConnection(Connection* conn) override {
    DestroyConnectionInternal(conn, false);
  }

  void DestroyConnectionAsync(Connection* conn) override {
    DestroyConnectionInternal(conn, true);
  }

  // In a shared socket mode each port which shares the socket will decide
  // to accept the packet based on the `remote_addr`. Currently only UDP
  // port implemented this method.
  // TODO(mallinath) - Make it pure virtual.
  virtual bool HandleIncomingPacket(AsyncPacketSocket* socket,
                                    const ReceivedIpPacket& packet);

  // Shall the port handle packet from this `remote_addr`.
  // This method is overridden by TurnPort.
  virtual bool CanHandleIncomingPacketsFrom(
      const SocketAddress& remote_addr) const;

  // Sends a response error to the given request.
  void SendBindingErrorResponse(StunMessage* message,
                                const SocketAddress& addr,
                                int error_code,
                                absl::string_view reason) override;
  void SendUnknownAttributesErrorResponse(
      StunMessage* message,
      const SocketAddress& addr,
      const std::vector<uint16_t>& unknown_types);

  void EnablePortPackets() override;

  // Called if the port has no connections and is no longer useful.
  void Destroy();

  // Debugging description of this port
  std::string ToString() const override;
  uint16_t min_port() { return min_port_; }
  uint16_t max_port() { return max_port_; }

  // Timeout shortening function to speed up unit tests.
  void set_timeout_delay(int delay);

  // This method will return local and remote username fragments from the
  // stun username attribute if present.
  bool ParseStunUsername(const StunMessage* stun_msg,
                         std::string* local_username,
                         std::string* remote_username) const override;
  std::string CreateStunUsername(
      absl::string_view remote_username) const override;

  bool MaybeIceRoleConflict(const SocketAddress& addr,
                            IceMessage* stun_msg,
                            absl::string_view remote_ufrag) override;

  // Called when a packet has been sent to the socket.
  // This is made pure virtual to notify subclasses of Port that they MUST
  // listen to AsyncPacketSocket::SignalSentPacket and then call
  // PortInterface::OnSentPacket.
  virtual void OnSentPacket(AsyncPacketSocket* socket,
                            const SentPacketInfo& sent_packet) = 0;

  // Called when the socket is currently able to send.
  void OnReadyToSend();

  // Called when the Connection discovers a local peer reflexive candidate.
  void AddPrflxCandidate(const Candidate& local) override;

  int16_t network_cost() const override { return network_cost_; }

  void GetStunStats(std::optional<StunStats>* /* stats */) override {}

 protected:
  void UpdateNetworkCost() override;

  WeakPtr<Port> NewWeakPtr() { return weak_factory_.GetWeakPtr(); }

  void AddAddress(const SocketAddress& address,
                  const SocketAddress& base_address,
                  const SocketAddress& related_address,
                  absl::string_view protocol,
                  absl::string_view relay_protocol,
                  absl::string_view tcptype,
                  IceCandidateType type,
                  uint32_t type_preference,
                  uint32_t relay_preference,
                  absl::string_view url,
                  bool is_final);

  void FinishAddingAddress(const Candidate& c, bool is_final)
      RTC_RUN_ON(thread_);

  virtual void PostAddAddress(bool is_final);

  // Adds the given connection to the map keyed by the remote candidate address.
  // If an existing connection has the same address, the existing one will be
  // replaced and destroyed.
  void AddOrReplaceConnection(Connection* conn);

  // Called when a packet is received from an unknown address that is not
  // currently a connection.  If this is an authenticated STUN binding request,
  // then we will signal the client.
  void OnReadPacket(const ReceivedIpPacket& packet, ProtocolType proto);

  [[deprecated(
      "Use OnReadPacket(const webrtc::ReceivedIpPacket& packet, ProtocolType "
      "proto)")]] void
  OnReadPacket(const char* data,
               size_t size,
               const SocketAddress& addr,
               ProtocolType proto) {
    OnReadPacket(ReceivedIpPacket::CreateFromLegacy(
                     data, size, /*packet_time_us = */ -1, addr),
                 proto);
  }

  // If the given data comprises a complete and correct STUN message then the
  // return value is true, otherwise false. If the message username corresponds
  // with this port's username fragment, msg will contain the parsed STUN
  // message.  Otherwise, the function may send a STUN response internally.
  // remote_username contains the remote fragment of the STUN username.
  bool GetStunMessage(const char* data,
                      size_t size,
                      const SocketAddress& addr,
                      std::unique_ptr<IceMessage>* out_msg,
                      std::string* out_username) override;

  // Checks if the address in addr is compatible with the port's ip.
  bool IsCompatibleAddress(const SocketAddress& addr);

  // Returns DSCP value packets generated by the port itself should use.
  DiffServCodePoint StunDscpValue() const override;

  // Extra work to be done in subclasses when a connection is destroyed.
  virtual void HandleConnectionDestroyed(Connection* /* conn */) {}

  void DestroyAllConnections();

  void CopyPortInformationToPacketInfo(PacketInfo* info) const;

  MdnsNameRegistrationStatus mdns_name_registration_status() const {
    return mdns_name_registration_status_;
  }
  void set_mdns_name_registration_status(MdnsNameRegistrationStatus status) {
    mdns_name_registration_status_ = status;
  }

  const FieldTrialsView& field_trials() const { return env_.field_trials(); }

  IceCandidateType type() const { return type_; }

 private:
  bool MaybeObfuscateAddress(const Candidate& c, bool is_final)
      RTC_RUN_ON(thread_);

  void PostDestroyIfDead(bool delayed);
  void DestroyIfDead();

  // Called internally when deleting a connection object.
  // Returns true if the connection object was removed from the `connections_`
  // list and the state updated accordingly. If the connection was not found
  // in the list, the return value is false. Note that this may indicate
  // incorrect behavior of external code that might be attempting to delete
  // connection objects from within a 'on destroyed' callback notification
  // for the connection object itself.
  bool OnConnectionDestroyed(Connection* conn);

  // Private implementation of DestroyConnection to keep the async usage
  // distinct.
  void DestroyConnectionInternal(Connection* conn, bool async);

  void OnNetworkTypeChanged(const ::webrtc::Network* network);

  const Environment env_;
  TaskQueueBase* const thread_;
  PacketSocketFactory* const factory_;
  const IceCandidateType type_;
  bool send_retransmit_count_attribute_;
  const ::webrtc::Network* network_;
  uint16_t min_port_;
  uint16_t max_port_;
  std::string content_name_;
  int component_;
  uint32_t generation_;
  // In order to establish a connection to this Port (so that real data can be
  // sent through), the other side must send us a STUN binding request that is
  // authenticated with this username_fragment and password.
  // PortAllocatorSession will provide these username_fragment and password.
  std::string ice_username_fragment_ RTC_GUARDED_BY(thread_);
  std::string password_ RTC_GUARDED_BY(thread_);
  std::vector<Candidate> candidates_ RTC_GUARDED_BY(thread_);
  AddressMap connections_;
  int timeout_delay_;
  bool enable_port_packets_;
  IceRole ice_role_;
  uint64_t tiebreaker_;
  bool shared_socket_;

  // A virtual cost perceived by the user, usually based on the network type
  // (WiFi. vs. Cellular). It takes precedence over the priority when
  // comparing two connections.
  int16_t network_cost_;
  // INIT: The state when a port is just created.
  // KEEP_ALIVE_UNTIL_PRUNED: A port should not be destroyed even if no
  // connection is using it.
  // PRUNED: It will be destroyed if no connection is using it for a period of
  // 30 seconds.
  enum class State { INIT, KEEP_ALIVE_UNTIL_PRUNED, PRUNED };
  State state_ = State::INIT;
  int64_t last_time_all_connections_removed_ = 0;
  MdnsNameRegistrationStatus mdns_name_registration_status_ =
      MdnsNameRegistrationStatus::kNotStarted;

  CallbackList<PortInterface*> port_destroyed_callback_list_;

  // Keep as the last member variable.
  WeakPtrFactory<Port> weak_factory_;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::CandidatePairChangeEvent;
using ::webrtc::CandidateStats;
using ::webrtc::CandidateStatsList;
using ::webrtc::DISCARD_PORT;
using ::webrtc::IceCandidateErrorEvent;
using ::webrtc::MdnsNameRegistrationStatus;
using ::webrtc::Port;
using ::webrtc::ProtocolAddress;
using ::webrtc::ProtoToString;
using ::webrtc::ServerAddresses;
using ::webrtc::StringToProto;
using ::webrtc::StunStats;
using ::webrtc::TCPTYPE_ACTIVE_STR;
using ::webrtc::TCPTYPE_PASSIVE_STR;
using ::webrtc::TCPTYPE_SIMOPEN_STR;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // P2P_BASE_PORT_H_
