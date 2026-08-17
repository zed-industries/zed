// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_IPCZ_NODE_H_
#define IPCZ_SRC_IPCZ_NODE_H_

#include <functional>
#include <memory>
#include <optional>
#include <utility>
#include <vector>

#include "ipcz/api_object.h"
#include "ipcz/driver_memory.h"
#include "ipcz/features.h"
#include "ipcz/ipcz.h"
#include "ipcz/link_side.h"
#include "ipcz/node_messages.h"
#include "ipcz/node_name.h"
#include "ipcz/node_type.h"
#include "third_party/abseil-cpp/absl/container/flat_hash_map.h"
#include "third_party/abseil-cpp/absl/container/flat_hash_set.h"
#include "third_party/abseil-cpp/absl/synchronization/mutex.h"
#include "third_party/abseil-cpp/absl/types/span.h"

namespace ipcz {

class NodeLink;
class NodeLinkMemory;

// A Node controls creation and interconnection of a collection of routers which
// can establish links to and from other routers in other nodes. Every node is
// assigned a globally unique name by a trusted broker node, and nodes may be
// introduced to each other exclusively through such brokers.
class Node : public APIObjectImpl<Node, APIObject::kNode> {
 public:
  using Type = NodeType;

  // State regarding a connection to a single remote node.
  struct Connection {
    // The NodeLink used to communicate with the remote node.
    Ref<NodeLink> link;

    // The NodeLink used to communicate with the broker of the remote node's
    // network. If the remote node belongs to the same network as the local
    // node, then this is the same link the local node's `broker_link_`. If the
    // local node *is* the broker for the remote node on `link`, then this link
    // is null.
    Ref<NodeLink> broker;
  };

  // Constructs a new node of the given `type`, using `driver` to support IPC.
  // Note that `driver` must outlive the Node.
  Node(Type type,
       const IpczDriver& driver,
       const IpczCreateNodeOptions* options = nullptr);

  Type type() const { return type_; }
  const IpczDriver& driver() const { return driver_; }
  const IpczCreateNodeOptions& options() const { return options_; }
  const Features& features() const { return features_; }

  // APIObject:
  IpczResult Close() override;

  // Connects to another node using `driver_transport` for I/O to and from the
  // other node. `initial_portals` is a collection of new portals who may
  // immediately begin to route parcels over a link to the new node, assuming
  // the link is established successfully.
  IpczResult ConnectNode(IpczDriverHandle driver_transport,
                         IpczConnectNodeFlags flags,
                         absl::Span<IpczHandle> initial_portals);

  // Retrieves the name assigned to this node, if any.
  NodeName GetAssignedName();

  // Gets a reference to the node's broker link, if it has one.
  Ref<NodeLink> GetBrokerLink();

  // Registers a new connection for the given `remote_node_name`.
  bool AddConnection(const NodeName& remote_node_name, Connection connection);

  // Returns a copy of the Connection to the remote node named by `name`, or
  // null if this node has no connection to that node.
  std::optional<Node::Connection> GetConnection(const NodeName& name);

  // Returns a reference to the NodeLink used by this Node to communicate with
  // the remote node identified by `name`; or null if this node has no NodeLink
  // connected to that node. This is shorthand for GetConnection() in the common
  // case where the caller only wants the underlying NodeLink.
  Ref<NodeLink> GetLink(const NodeName& name);

  // Generates a new random NodeName using this node's driver as a source of
  // randomness.
  NodeName GenerateRandomName() const;

  // Sets a NodeLink to use for asynchronous shared memory allocation requests.
  // This is configured when the ConnectNode() API is called with
  // IPCZ_CONNECT_NODE_TO_ALLOCATION_DELEGATE. Typically this is combined with
  // IPCZ_CONNECT_NODE_TO_BROKER when connecting from a sandboxed process which
  // cannot allocate its own shared memory regions.
  void SetAllocationDelegate(Ref<NodeLink> link);

  // Requests allocation of a new shared memory object of the given size.
  // `callback` is invoked with the new object when allocation is complete.
  // This operation is asynchronous if allocation is delegated to another node,
  // but if this node can allocate directly through the driver, `callback` is
  // invoked with the result before this method returns.
  using AllocateSharedMemoryCallback = std::function<void(DriverMemory)>;
  void AllocateSharedMemory(size_t size, AllocateSharedMemoryCallback callback);

  // Asynchronously attempts to establish a new NodeLink directly to the named
  // node, invoking `callback` when complete. On success, this node will retain
  // a new NodeLink to the named node, and `callback` will be invoked with a
  // reference to that link. Otherwise `callback` will be invoked with a null
  // reference.
  //
  // If the calling node already has a link to the named node, `callback` may
  // be invoked synchronously with a link to that node before this method
  // returns.
  using EstablishLinkCallback = std::function<void(NodeLink*)>;
  void EstablishLink(const NodeName& name, EstablishLinkCallback callback);

  // Handles an incoming introduction request. Must only be called on a broker
  // node. If this broker has a NodeLink to the node named by `for_node`, it
  // will introduce that node and the remote node on `from_node_link`.
  void HandleIntroductionRequest(NodeLink& from_node_link,
                                 const NodeName& for_node);

  // Accepts an introduction received from the broker. `transport` and `memory`
  // can be used to establish a new NodeLink to the remote node, whose name is
  // `name`. The NodeLink must assume a role as the given `side` of the link.
  void AcceptIntroduction(NodeLink& from_node_link,
                          const NodeName& name,
                          LinkSide side,
                          Node::Type remote_node_type,
                          uint32_t remote_protocol_version,
                          const Features& remote_features,
                          Ref<DriverTransport> transport,
                          Ref<NodeLinkMemory> memory);

  // Handles a rejected introduction for the node named `name` from the
  // identified broker. This is called on a node that previously requested an
  // introduction if the broker is unable or unwilling to satisfy the request.
  void NotifyIntroductionFailed(NodeLink& from_broker, const NodeName& name);

  // Relays a message to its destination on behalf of `from_node`.
  bool RelayMessage(const NodeName& from_node, msg::RelayMessage& relay);

  // Attempts to dispatch a relayed message from the broker as if it came from
  // the relay source directly.
  bool AcceptRelayedMessage(msg::AcceptRelayedMessage& accept);

  // Drops the connection running over `connection_link` between this node and
  // another.
  void DropConnection(const NodeLink& connection_link);

  // Asynchronously waits for this Node to acquire a broker link and then
  // invokes `callback` with it. If this node already has a broker link then the
  // callback is invoked immediately, before this method returns.
  using BrokerLinkCallback = std::function<void(Ref<NodeLink>)>;
  void WaitForBrokerLinkAsync(BrokerLinkCallback callback);

  // Processes a request for an indirect cross-network node introduction. The
  // request was sent by `from_node_link` (a link to another broker) and is
  // asking us to introduce `our_node` within our network to `their_node` in the
  // the requestor's network.
  bool HandleIndirectIntroductionRequest(NodeLink& from_node_link,
                                         const NodeName& our_node,
                                         const NodeName& their_node);

 private:
  class PendingIntroduction;

  ~Node() override;

  // Deactivates all NodeLinks and their underlying driver transports in
  // preparation for this node's imminent destruction.
  void ShutDown();

  // Resolves all pending introduction requests with a null link, implying
  // failure.
  void CancelAllIntroductions();

  // Creates a new transport and link memory and sends introduction messages to
  // introduce the remote node on `first` to the remote node on `second`.
  void IntroduceRemoteNodes(NodeLink& first, NodeLink& second);

  // Called when dropping or failing to establish a broker link.
  void NotifyBrokerLinkDropped();

  const Type type_;
  const IpczDriver& driver_;
  const IpczCreateNodeOptions options_;
  const Features features_;

  absl::Mutex mutex_;

  // The name assigned to this node by the first broker it connected to, or
  // self-assigned if this is a broker node. Once assigned, this name remains
  // constant through the lifetime of the node.
  NodeName assigned_name_ ABSL_GUARDED_BY(mutex_);

  // A link to the first broker this node connected to. If this link is broken,
  // the node will lose all its other links too. This is always null on broker
  // nodes, though brokers may keep track of links to other brokers within
  // `other_brokers_`.
  Ref<NodeLink> broker_link_ ABSL_GUARDED_BY(mutex_);

  // A link over which all internal shared memory allocation is delegated. If
  // null, this Node will always attempt to allocate shared memory directly
  // through its ipcz driver.
  Ref<NodeLink> allocation_delegate_link_ ABSL_GUARDED_BY(mutex_);

  // Lookup table of broker-assigned node names and links to those nodes. All of
  // these links and their associated names are received by the `broker_link_`
  // if this is a non-broker node. If this is a broker node, these links are
  // either assigned by this node itself, or received from other brokers in the
  // system.
  using ConnectionMap = absl::flat_hash_map<NodeName, Connection>;
  ConnectionMap connections_ ABSL_GUARDED_BY(mutex_);

  // A map of other nodes to which this node is waiting for an introduction,
  // either from its own broker or (if we are a broker) all the other known
  // brokers we're connected to.
  using PendingIntroductionMap =
      absl::flat_hash_map<NodeName, std::unique_ptr<PendingIntroduction>>;
  PendingIntroductionMap pending_introductions_ ABSL_GUARDED_BY(mutex_);

  // Nodes may race to request introductions to each other from the same broker.
  // This can lead to redundant introductions being sent which the requesting
  // nodes should be able to ignore. However, the following could occur on a
  // broker which is processing a request from node A on Thread 1 while also
  // processing a request from node B on thread 2:
  //
  //    Thread 1                       Thread 2                      Time
  //    ---                            ---                             |
  //    A requests intro to B          B requests intro to A           v
  //    Send B intro X to A
  //                                   Send A intro Y to B
  //    Send A intro X to B
  //                                   Send B intro Y to A
  //
  // Each unique intro shares either end of a transport with its recipients,
  // so both A and B must accept the same introduction (either X or Y). In this
  // scenario however, A will first receive and accept intro X, and will ignore
  // intro Y as redundant. But B will receive intro Y first and ignore intro X
  // as redundant. This is bad.
  //
  // The set of `in_progress_introductions_` allows this (broker) node to guard
  // against such interleaved introductions. Immediately before sending an intro
  // to both recipients, a key identifying them is placed into the set. This key
  // is removed immediately after both introductions are sent. If another thread
  // is asked to introduce the same two nodes while the key is still present, it
  // will ignore the request and send nothing.
  using IntroductionKey = std::pair<NodeName, NodeName>;
  absl::flat_hash_set<IntroductionKey> in_progress_introductions_
      ABSL_GUARDED_BY(mutex_);

  // Set of callbacks waiting to be invoked as soon as this Node acquires a
  // broker link.
  std::vector<BrokerLinkCallback> broker_link_callbacks_
      ABSL_GUARDED_BY(mutex_);

  // Mapping of links to other known brokers in the system. This is the subset
  // of `links_` which corresponds to remote broker nodes NOT in this node's own
  // network. This map can only be non-empty on broker nodes.
  absl::flat_hash_map<NodeName, Ref<NodeLink>> other_brokers_
      ABSL_GUARDED_BY(mutex_);
};

}  // namespace ipcz

#endif  // IPCZ_SRC_IPCZ_NODE_H_
