// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_IPCZ_NODE_CONNECTOR_H_
#define IPCZ_SRC_IPCZ_NODE_CONNECTOR_H_

#include <cstdint>
#include <functional>
#include <vector>

#include "ipcz/driver_transport.h"
#include "ipcz/ipcz.h"
#include "ipcz/node.h"
#include "ipcz/node_messages.h"
#include "util/ref_counted.h"

namespace ipcz {

class NodeLink;
class Router;

// A NodeConnector activates and temporarily attaches itself to a
// DriverTransport to listen for and transmit introductory messages between two
// nodes invoking ConnectNode() on opposite ends of the same transport pair.
//
// Once an initial handshake is complete the underlying transport is adopted by
// a new NodeLink and handed off to the local Node to communicate with the
// remote node, and this object is destroyed.
class NodeConnector : public msg::NodeMessageListener {
 public:
  // Constructs a new NodeConnector to send and receive a handshake on
  // `transport`. The specific type of connector to create is determined by a
  // combination of the Node::Type of `node` and the value of `flags`.
  //
  // If a connection is successfully established, `transport` will eventually
  // be adopted by a NodeLink and passed to `node` for use. Otherwise, all
  // `initial_portals` will observe peer closure.
  //
  // In either case this object invokes `callback` if non-null and then destroys
  // itself once the handshake is complete. If this fails, the NodeLink given
  // to the callback will be null.
  using ConnectCallback = std::function<void(Ref<NodeLink> new_link)>;
  static IpczResult ConnectNode(Ref<Node> node,
                                Ref<DriverTransport> transport,
                                IpczConnectNodeFlags flags,
                                const std::vector<Ref<Router>>& initial_routers,
                                ConnectCallback callback = nullptr);

  // Handles a request on `node` (which must be a broker) to accept a new
  // non-broker node referral from `referrer`, referring a new non-broker node
  // on the remote end of `transport_to_referred_node`. This performs a
  // handshake with the referred node before introducing it and the referrer to
  // each other. `link_memory` and `client_link_memory` must be valid and will
  // be passed respectively to the referred node (for its link to the broker)
  // and the referring node (for its link to the referred node).
  static bool HandleNonBrokerReferral(
      Ref<Node> node,
      uint64_t referral_id,
      uint32_t num_initial_portals,
      Ref<NodeLink> referrer,
      Ref<DriverTransport> transport_to_referred_node,
      DriverMemoryWithMapping link_memory,
      DriverMemoryWithMapping client_link_memory);

  virtual bool Connect() = 0;

 protected:
  NodeConnector(Ref<Node> node,
                Ref<DriverTransport> transport,
                IpczConnectNodeFlags flags,
                std::vector<Ref<Router>> waiting_routers,
                ConnectCallback callback);
  ~NodeConnector() override;

  size_t num_portals() const { return waiting_routers_.size(); }

  // Invoked once by the implementation when it has completed its handshake.
  // Destroys `this`.
  void AcceptConnection(Node::Connection connection,
                        uint32_t num_remote_portals);

  // Invoked if the transport observes an error before receiving the expected
  // handshake message, or if the implementation receives any message other than
  // the one handshake message it expects to see first.
  void RejectConnection();

  const Ref<Node> node_;
  const Ref<DriverTransport> transport_;
  const IpczConnectNodeFlags flags_;
  const std::vector<Ref<Router>> waiting_routers_;

  // NodeMessageListener overrides:
  void OnTransportError() override;

 private:
  bool ActivateTransport();
  void EstablishWaitingRouters(Ref<NodeLink> to_link, size_t max_valid_portals);

  const ConnectCallback callback_;
};

}  // namespace ipcz

#endif  // IPCZ_SRC_IPCZ_NODE_CONNECTOR_H_
