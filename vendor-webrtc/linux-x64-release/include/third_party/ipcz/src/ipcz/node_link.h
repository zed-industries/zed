// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_IPCZ_NODE_LINK_H_
#define IPCZ_SRC_IPCZ_NODE_LINK_H_

#include <atomic>
#include <cstddef>
#include <cstdint>
#include <optional>
#include <type_traits>
#include <vector>

#include "ipcz/driver_memory.h"
#include "ipcz/driver_transport.h"
#include "ipcz/features.h"
#include "ipcz/fragment_ref.h"
#include "ipcz/link_side.h"
#include "ipcz/link_type.h"
#include "ipcz/node.h"
#include "ipcz/node_link_memory.h"
#include "ipcz/node_messages.h"
#include "ipcz/node_name.h"
#include "ipcz/sequence_number.h"
#include "ipcz/sublink_id.h"
#include "third_party/abseil-cpp/absl/container/flat_hash_map.h"
#include "third_party/abseil-cpp/absl/synchronization/mutex.h"
#include "third_party/abseil-cpp/absl/types/span.h"
#include "util/ref_counted.h"

namespace ipcz {

class Message;
class Parcel;
class ParcelWrapper;
class RemoteRouterLink;
class Router;

// A NodeLink instance encapsulates all communication from its owning node to
// exactly one other remote node in the sytem. Each NodeLink manages a
// DriverTransport for general-purpose I/O to and from the remote node.
//
// NodeLinks may also allocate an arbitrary number of sublinks which are used
// to multiplex the link and facilitate point-to-point communication between
// specific Router instances on either end.
class NodeLink : public msg::NodeMessageListener {
 public:
  struct Sublink {
    Sublink(Ref<RemoteRouterLink> link, Ref<Router> receiver);
    Sublink(Sublink&&);
    Sublink(const Sublink&);
    Sublink& operator=(Sublink&&);
    Sublink& operator=(const Sublink&);
    ~Sublink();

    Ref<RemoteRouterLink> router_link;
    Ref<Router> receiver;
  };

  // Creates a new NodeLink over a transport which is already active. This is
  // only safe to call from within the activity handler of that transport, and
  // the returned NodeLink will have effectively already been activated.
  static Ref<NodeLink> CreateActive(Ref<Node> node,
                                    LinkSide link_side,
                                    const NodeName& local_node_name,
                                    const NodeName& remote_node_name,
                                    Node::Type remote_node_type,
                                    uint32_t remote_protocol_version,
                                    const Features& remote_features,
                                    Ref<DriverTransport> transport,
                                    Ref<NodeLinkMemory> memory);

  // Creates a new NodeLink over a transport which is not yet active. This
  // NodeLink must be explicitly activated with Activate() before it can be
  // used.
  static Ref<NodeLink> CreateInactive(Ref<Node> node,
                                      LinkSide link_side,
                                      const NodeName& local_node_name,
                                      const NodeName& remote_node_name,
                                      Node::Type remote_node_type,
                                      uint32_t remote_protocol_version,
                                      const Features& remote_features,
                                      Ref<DriverTransport> transport,
                                      Ref<NodeLinkMemory> memory);

  const Ref<Node>& node() const { return node_; }
  LinkSide link_side() const { return link_side_; }
  const NodeName& local_node_name() const { return local_node_name_; }
  const NodeName& remote_node_name() const { return remote_node_name_; }
  Node::Type remote_node_type() const { return remote_node_type_; }
  uint32_t remote_protocol_version() const { return remote_protocol_version_; }
  const Features& remote_features() const { return remote_features_; }
  const Features& available_features() const { return available_features_; }

  const Ref<DriverTransport>& transport() const { return transport_; }

  NodeLinkMemory& memory() { return *memory_; }
  const NodeLinkMemory& memory() const { return *memory_; }

  // Activates this NodeLink. The NodeLink must have been created with
  // CreateInactive() and must not have already been activated.
  void Activate();

  // Binds `sublink` on this NodeLink to the given `router`. `link_side`
  // specifies which side of the link this end identifies as (A or B), and
  // `type` specifies the type of link this is, from the perspective of
  // `router`.
  //
  // If `link_state_fragment` is non-null, the given fragment contains the
  // shared RouterLinkState structure for the new link. Only central links
  // require a RouterLinkState.
  Ref<RemoteRouterLink> AddRemoteRouterLink(
      SublinkId sublink,
      FragmentRef<RouterLinkState> link_state,
      LinkType type,
      LinkSide side,
      Ref<Router> router);

  // Removes the route specified by `sublink`. Once removed, any messages
  // received for that sublink are ignored.
  void RemoveRemoteRouterLink(SublinkId sublink);

  // Retrieves the Router and RemoteRouterLink currently bound to `sublink`
  // on this NodeLink.
  std::optional<Sublink> GetSublink(SublinkId sublink);

  // Retrieves only the Router currently bound to `sublink` on this NodeLink.
  Ref<Router> GetRouter(SublinkId sublink);

  // Sends a new driver memory object to the remote endpoint to be associated
  // with BufferId within the peer NodeLink's associated NodeLinkMemory, and to
  // be used to dynamically allocate blocks of `block_size` bytes. The BufferId
  // must have already been reserved locally by this NodeLink using
  // AllocateNewBufferId().
  void AddBlockBuffer(BufferId id, uint32_t block_size, DriverMemory memory);

  // Asks the broker on the other end of this link to introduce the local node
  // to the node identified by `name`. This will always elicit a response from
  // the broker in the form of either an AcceptIntroduction or
  // RejectIntroduction message.
  void RequestIntroduction(const NodeName& name);

  // Introduces the remote node to the node named `name`, with details needed to
  // construct a new NodeLink to that node.
  void AcceptIntroduction(const NodeName& name,
                          LinkSide side,
                          Node::Type remote_node_type,
                          uint32_t remote_protocol_version,
                          const Features& remote_features,
                          Ref<DriverTransport> transport,
                          DriverMemory memory);

  // Rejects an introduction request previously sent by the remote node for the
  // node identified by `name`.
  void RejectIntroduction(const NodeName& name);

  // May be called on a link from a non-broker to a broker in order to refer a
  // new node to the remote broker. `transport` is a transport whose peer
  // endpoint belongs to the referred node, and `num_initial_portals` is the
  // number of initial portals expected on the resulting link from this side.
  // Upon success, `callback` is invoked with a new NodeLink to the referred
  // node and the number of initial portals expected by that side. On failure,
  // `callback` is invoked with a null link.
  using ReferralCallback = std::function<void(Ref<NodeLink>, uint32_t)>;
  void ReferNonBroker(Ref<DriverTransport> transport,
                      uint32_t num_initial_portals,
                      ReferralCallback callback);

  // Sends a request to the remote node to establish a new RouterLink over this
  // this NodeLink, to replace an existing RouterLink between the remote node
  // and `current_peer_node`. `current_peer_sublink` identifies the specific
  // RouterLink between them which is to be replaced.
  //
  // `inbound_sequence_length_from_bypassed_link` is the final length of the
  // parcel sequence to be routed over the link which is being bypassed.
  // `new_sublink` and (optionally null) `new_link_state` can be used to
  // establish the new link over the NodeLink transmitting this message.
  void AcceptBypassLink(
      const NodeName& current_peer_node,
      SublinkId current_peer_sublink,
      SequenceNumber inbound_sequence_length_from_bypassed_link,
      SublinkId new_sublink,
      FragmentRef<RouterLinkState> new_link_state);

  // Sends a request to allocate a new shared memory region and invokes
  // `callback` once the request succeeds or fails. On failure, `callback` is
  // invoke with an invalid DriverMemory object.
  using RequestMemoryCallback = std::function<void(DriverMemory)>;
  void RequestMemory(size_t size, RequestMemoryCallback callback);

  // Asks the remote node (which must be a broker) to relay `message` over to
  // `to_node`. This is used to transmit driver objects between non-broker nodes
  // whenever direct transmission is unsupported by the driver.
  void RelayMessage(const NodeName& to_node, Message& message);

  // Simulates receipt of a new message from the remote node on this link. This
  // is called by the local Node with a message that was relayed to it by its
  // broker. All relayed messages land on their destination node through this
  // method.
  bool DispatchRelayedMessage(msg::AcceptRelayedMessage& relay);

  // Permanently deactivates this NodeLink. Once this call returns the NodeLink
  // will no longer receive transport messages. It may still be used to transmit
  // outgoing messages, but it cannot be reactivated. Transmissions over a
  // deactivated transport may or may not guarantee delivery to the peer
  // transport, as this is left to driver's discretion.
  //
  // Must only be called on an activated NodeLink, either one which was created
  // with CreateActive(), or one which was activated later by calling
  // Activate().
  void Deactivate();

  // Finalizes serialization of DriverObjects within `message` and transmits it
  // to the NodeLink's peer, either over the DriverTransport or through shared
  // memory.
  void Transmit(Message& message);

 private:
  friend class RefCounted<NodeLink>;

  enum ActivationState {
    kNeverActivated,
    kActive,
    kDeactivated,
  };

  NodeLink(Ref<Node> node,
           LinkSide link_side,
           const NodeName& local_node_name,
           const NodeName& remote_node_name,
           Node::Type remote_node_type,
           uint32_t remote_protocol_version,
           const Features& remote_features,
           Ref<DriverTransport> transport,
           Ref<NodeLinkMemory> memory,
           ActivationState initial_activation_state);
  ~NodeLink() override;

  SequenceNumber GenerateOutgoingSequenceNumber();

  // NodeMessageListener overrides:
  bool OnReferNonBroker(msg::ReferNonBroker& refer) override;
  bool OnNonBrokerReferralAccepted(
      msg::NonBrokerReferralAccepted& accepted) override;
  bool OnNonBrokerReferralRejected(
      msg::NonBrokerReferralRejected& rejected) override;
  bool OnRequestIntroduction(msg::RequestIntroduction& request) override;
  bool OnAcceptIntroduction(msg::AcceptIntroduction& accept) override;
  bool OnRejectIntroduction(msg::RejectIntroduction& reject) override;
  bool OnRequestIndirectIntroduction(
      msg::RequestIndirectIntroduction& request) override;
  bool OnAddBlockBuffer(msg::AddBlockBuffer& add) override;
  bool OnAcceptParcel(msg::AcceptParcel& accept) override;
  bool OnAcceptParcelDriverObjects(
      msg::AcceptParcelDriverObjects& accept) override;
  bool OnRouteClosed(msg::RouteClosed& route_closed) override;
  bool OnRouteDisconnected(msg::RouteDisconnected& route_disconnected) override;
  bool OnBypassPeer(msg::BypassPeer& bypass) override;
  bool OnAcceptBypassLink(msg::AcceptBypassLink& accept) override;
  bool OnStopProxying(msg::StopProxying& stop) override;
  bool OnProxyWillStop(msg::ProxyWillStop& will_stop) override;
  bool OnBypassPeerWithLink(msg::BypassPeerWithLink& bypass) override;
  bool OnStopProxyingToLocalPeer(msg::StopProxyingToLocalPeer& stop) override;
  bool OnFlushRouter(msg::FlushRouter& flush) override;
  bool OnRequestMemory(msg::RequestMemory& request) override;
  bool OnProvideMemory(msg::ProvideMemory& provide) override;
  bool OnRelayMessage(msg::RelayMessage& relay) override;
  bool OnAcceptRelayedMessage(msg::AcceptRelayedMessage& accept) override;
  void OnTransportError() override;

  void HandleTransportError();

  // Invoked when we receive a Parcel whose data fragment resides in a buffer
  // not yet known to the local node. This schedules the parcel for acceptance
  // as soon as that buffer is available.
  void WaitForParcelFragmentToResolve(SublinkId for_sublink,
                                      std::unique_ptr<Parcel> parcel,
                                      const FragmentDescriptor& descriptor,
                                      bool is_split_parcel);

  bool AcceptParcelWithoutDriverObjects(SublinkId for_sublink,
                                        std::unique_ptr<Parcel> parcel);
  bool AcceptParcelDriverObjects(SublinkId for_sublink,
                                 std::unique_ptr<Parcel> parcel);
  bool AcceptSplitParcel(SublinkId for_sublink,
                         std::unique_ptr<Parcel> parcel_without_driver_objects,
                         std::unique_ptr<Parcel> parcel_with_driver_objects);
  bool AcceptCompleteParcel(SublinkId for_sublink,
                            std::unique_ptr<Parcel> parcel);

  const Ref<Node> node_;
  const LinkSide link_side_;
  const NodeName local_node_name_;
  const NodeName remote_node_name_;
  const Node::Type remote_node_type_;
  const uint32_t remote_protocol_version_;
  const Features remote_features_;
  const Features available_features_;
  const Ref<DriverTransport> transport_;
  const Ref<NodeLinkMemory> memory_;

  absl::Mutex mutex_;
  ActivationState activation_state_ ABSL_GUARDED_BY(mutex_);

  // Messages transmitted from this NodeLink may traverse either the driver
  // transport OR a shared memory queue. Messages transmitted from the same
  // thread need to be processed in the same order by the receiving node. This
  // is used to generate a sequence number for every message so they can be
  // reordered on the receiving end.
  std::atomic<uint64_t> next_outgoing_sequence_number_generator_{0};

  using SublinkMap = absl::flat_hash_map<SublinkId, Sublink>;
  SublinkMap sublinks_ ABSL_GUARDED_BY(mutex_);

  // Pending memory allocation request callbacks. Keyed by request size, when
  // an incoming ProvideMemory message is received, the front of the list for
  // that size is removed from the map and invoked with the new memory object.
  using MemoryRequestMap =
      absl::flat_hash_map<uint32_t, std::list<RequestMemoryCallback>>;
  MemoryRequestMap pending_memory_requests_ ABSL_GUARDED_BY(mutex_);

  // Tracks partially received contents of split parcels so they can be
  // reconstructed for dispatch.
  using PartialParcelKey = std::tuple<SublinkId, SequenceNumber>;
  using PartialParcelMap =
      absl::flat_hash_map<PartialParcelKey, std::unique_ptr<Parcel>>;
  PartialParcelMap partial_parcels_ ABSL_GUARDED_BY(mutex_);

  // Mapping from subparcel index to Parcel object.
  using SubparcelMap = absl::flat_hash_map<size_t, Parcel>;

  // Tracks complete subparcels received for a given sequence number. Only once
  // all expected subparcels are received can the parcel be made available for
  // consumption.
  using SubparcelVector = std::vector<Ref<ParcelWrapper>>;
  struct SubparcelTracker {
    size_t num_subparcels_received = 0;
    SubparcelVector subparcels;
  };
  using SubparcelTrackerKey = std::tuple<SublinkId, SequenceNumber>;
  using SubparcelTrackerMap =
      absl::flat_hash_map<SubparcelTrackerKey, SubparcelTracker>;
  SubparcelTrackerMap subparcel_trackers_ ABSL_GUARDED_BY(mutex_);

  // Tracks pending referrals sent to the broker.
  uint64_t next_referral_id_ = 0;
  using ReferralCallbackMap = absl::flat_hash_map<uint64_t, ReferralCallback>;
  ReferralCallbackMap pending_referrals_ ABSL_GUARDED_BY(mutex_);
};

}  // namespace ipcz

#endif  // IPCZ_SRC_IPCZ_NODE_LINK_H_
