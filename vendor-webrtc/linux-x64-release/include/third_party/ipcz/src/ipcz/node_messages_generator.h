// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// no-include-guard-because-multiply-included

// This file defines the internal messages which can be sent on a NodeLink
// between two ipcz nodes.

IPCZ_MSG_BEGIN_INTERFACE(Node)

// Initial greeting sent by a broker node when a ConnectNode() is issued without
// the IPCZ_CONNECT_NODE_TO_BROKER flag, implying that the receiving node is a
// non-broker.
IPCZ_MSG_BEGIN(ConnectFromBrokerToNonBroker, IPCZ_MSG_ID(0))
  IPCZ_MSG_BEGIN_VERSION(0)
    // The name of the broker node.
    IPCZ_MSG_PARAM(NodeName, broker_name)

    // The name of the receiving non-broker node, assigned randomly by the
    // broker.
    IPCZ_MSG_PARAM(NodeName, receiver_name)

    // The highest protocol version known and desired by the broker.
    IPCZ_MSG_PARAM(uint32_t, protocol_version)

    // The number of initial portals assumed on the broker's end of the
    // connection. If there is a mismatch between the number sent by each node
    // on an initial connection, the node which sent the larger number should
    // behave as if its excess portals have observed peer closure. This may
    // occur for example as a result of version skew between one application
    // node and another, where one end tries to establish more initial portals
    // than the other supports.
    IPCZ_MSG_PARAM(uint32_t, num_initial_portals)

    // A driver memory object to serve as the new NodeLink's primary shared
    // buffer. That is, BufferId 0 within its NodeLinkMemory's BufferPool.
    IPCZ_MSG_PARAM_DRIVER_OBJECT(buffer)

    // Explicit padding to preserve 8-byte size alignemnt.
    IPCZ_MSG_PARAM(uint32_t, padding)
  IPCZ_MSG_END_VERSION(0)

  IPCZ_MSG_BEGIN_VERSION(1)
    // Features enabled on the sending node.
    IPCZ_MSG_PARAM_ARRAY(Features::Bitfield, features)
  IPCZ_MSG_END_VERSION(1)
IPCZ_MSG_END()

// Initial greeting sent by a non-broker node when ConnectNode() is invoked with
// IPCZ_CONNECT_NODE_TO_BROKER. The sending non-broker node expects to receive a
// corresponding ConnectFromBrokerToNonBroker
IPCZ_MSG_BEGIN(ConnectFromNonBrokerToBroker, IPCZ_MSG_ID(1))
  IPCZ_MSG_BEGIN_VERSION(0)
    // The highest protocol version known and desired by the sender.
    IPCZ_MSG_PARAM(uint32_t, protocol_version)

    // The number of initial portals assumed on the sender's end of the
    // connection. If there is a mismatch between the number sent by each node
    // on an initial connection, the node which sent the larger number should
    // behave as if its excess portals have observed peer closure.
    IPCZ_MSG_PARAM(uint32_t, num_initial_portals)
  IPCZ_MSG_END_VERSION(0)

  IPCZ_MSG_BEGIN_VERSION(1)
    // Features enabled on the sending node.
    IPCZ_MSG_PARAM_ARRAY(Features::Bitfield, features)
  IPCZ_MSG_END_VERSION(1)
IPCZ_MSG_END()

// Sent from a non-broker to its broker when calling ConnectNode() with
// IPCZ_CONNECT_NODE_SHARE_BROKER. In this case the transport given to
// ConnectNode() is passed along to the broker via this message, and the broker
// assumes the other end of that transport belongs to a new non-broker node who
// wishes to join the network.
//
// The broker performs an initial handshake with the referred node -- it waits
// for a ConnectToReferredBroker message on the new transport and then sends a
// ConnectToReferredNonBroker over the same transport, as well as a
// NonBrokerReferralAccepted message back to the original referrer who sent this
// request.
//
// If this request is invalid or the broker otherwise fails to establish a link
// to the referred node, the broker instead responds to the referrer with a
// NonBrokerReferralRejected message.
IPCZ_MSG_BEGIN(ReferNonBroker, IPCZ_MSG_ID(2))
  IPCZ_MSG_BEGIN_VERSION(0)
    // A unique (for the transmitting NodeLink) identifier for this referral,
    // used to associate a corresponding NonBrokerReferralAccepted/Rejected
    // response from the broker.
    IPCZ_MSG_PARAM(uint64_t, referral_id)

    // The number of initial portals the referrer will assume on its own
    // transport to the referred node if the referral is successful and the
    // broker responds with NonBrokerReferralAccepted. This value is passed
    // along to the referred node via ConnectToReferredNonBroker.
    IPCZ_MSG_PARAM(uint32_t, num_initial_portals)

    // The transport given to ConnectNode() with IPCZ_CONNECT_NODE_SHARE_BROKER.
    IPCZ_MSG_PARAM_DRIVER_OBJECT(transport)
  IPCZ_MSG_END_VERSION(0)
IPCZ_MSG_END()

// Sent from a non-broker to its tentative broker when calling ConnectNode()
// with IPCZ_CONNECT_NODE_INHERIT_BROKER. The other end of the transport given
// to that ConnectNode() call must itself be given to ConnectNode() by some
// other non-broker calling with IPCZ_CONNECT_NODE_SHARE_BROKER. That other node
// will pass the transport to the broker using a ReferNonBroker message.
//
// Once ConnectToReferredBroker is received by the broker on the new transport,
// the broker sends back a ConnectToReferredNonBroker to the sender of this
// message, as well as a NonBrokerReferralAccepted message to the original
// referrer.
IPCZ_MSG_BEGIN(ConnectToReferredBroker, IPCZ_MSG_ID(3))
  IPCZ_MSG_BEGIN_VERSION(0)
    // The highest protocol version known and desired by the sender.
    IPCZ_MSG_PARAM(uint32_t, protocol_version)

    // The number of initial portals assumed on the sender's end of the
    // transport. This is passed along by the broker to the referrer via
    // NonBrokerReferralAccepted.
    IPCZ_MSG_PARAM(uint32_t, num_initial_portals)
  IPCZ_MSG_END_VERSION(0)

  IPCZ_MSG_BEGIN_VERSION(1)
    // Features enabled on the sending node.
    IPCZ_MSG_PARAM_ARRAY(Features::Bitfield, features)
  IPCZ_MSG_END_VERSION(1)
IPCZ_MSG_END()

// Sent from a broker to a referred non-broker node over a transport that was
// provided to the broker by some other non-broker via a ReferNonBroker message.
//
// This is sent to the referred node if and only if the referral has been
// accepted by the broker, and only the broker has received a
// ConnectToReferredBroker message over the same transport that sends this
// message.
IPCZ_MSG_BEGIN(ConnectToReferredNonBroker, IPCZ_MSG_ID(4))
  IPCZ_MSG_BEGIN_VERSION(0)
    // The newly assigned name of the node receiving this message.
    IPCZ_MSG_PARAM(NodeName, name)

    // The name of the broker node which has accepted the referred recipient of
    // this message.
    IPCZ_MSG_PARAM(NodeName, broker_name)

    // The name of the node which referred the recipient to the broker sending
    // this message.
    IPCZ_MSG_PARAM(NodeName, referrer_name)

    // The highest protocol version known and desired by the sending broker.
    IPCZ_MSG_PARAM(uint32_t, broker_protocol_version)

    // The highest protocol version known and desired by the referrer.
    IPCZ_MSG_PARAM(uint32_t, referrer_protocol_version)

    // The number of initial portals assumed by the referred on its initial link
    // to the receipient of this message.
    IPCZ_MSG_PARAM(uint32_t, num_initial_portals)

    // A driver memory object to serve as the primary NodeLinkMemory buffer for
    // the NodeLink between the broker and the recipient of this message (i.e.
    // the NodeLink established from the transport which carries this message.)
    IPCZ_MSG_PARAM_DRIVER_OBJECT(broker_link_buffer)

    // A new transport and primary buffer the receipient can use to establish a
    // new NodeLink to the referrer. The other end of the transport (and another
    // handle to the same memory object) is given to the referrer via
    // NonBrokerReferralAccepted.
    IPCZ_MSG_PARAM_DRIVER_OBJECT(referrer_link_transport)
    IPCZ_MSG_PARAM_DRIVER_OBJECT(referrer_link_buffer)
  IPCZ_MSG_END_VERSION(0)

  IPCZ_MSG_BEGIN_VERSION(1)
    // Features enabled on the broker node.
    IPCZ_MSG_PARAM_ARRAY(Features::Bitfield, broker_features)

    // Features enabled on the referring node.
    IPCZ_MSG_PARAM_ARRAY(Features::Bitfield, referrer_features)
  IPCZ_MSG_END_VERSION(1)
IPCZ_MSG_END()

// Sent from a broker to a non-broker who previously referred another node via
// ReferNonBroker. This message indicates that the referral was accepted, and it
// provides objects and details necessary for the recipient (i.e. the referrer)
// to establish a direct NodeLink to the referred node.
IPCZ_MSG_BEGIN(NonBrokerReferralAccepted, IPCZ_MSG_ID(5))
  IPCZ_MSG_BEGIN_VERSION(0)
    // A unique identifier for the referral in question, as provided by the
    // original ReferNonBroker message sent by the receipient of this message.
    IPCZ_MSG_PARAM(uint64_t, referral_id)

    // The highest protocol version known and desired by the referred node.
    IPCZ_MSG_PARAM(uint32_t, protocol_version)

    // The number of initial portals assumed by the referred node on its end of
    // the link conveyed by `transport` in this message.
    IPCZ_MSG_PARAM(uint32_t, num_initial_portals)

    // The name of the referred node, as assigned by the broker.
    IPCZ_MSG_PARAM(NodeName, name)

    // A driver transport and primary buffer memory object the receipient can
    // use to establish a direct NodeLink to the referred node.
    IPCZ_MSG_PARAM_DRIVER_OBJECT(transport)
    IPCZ_MSG_PARAM_DRIVER_OBJECT(buffer)
  IPCZ_MSG_END_VERSION(0)

  IPCZ_MSG_BEGIN_VERSION(1)
    // Features enabled on the sending broker.
    IPCZ_MSG_PARAM_ARRAY(Features::Bitfield, features)
  IPCZ_MSG_END_VERSION(1)
IPCZ_MSG_END()

// Sent from a broker to a non-broker who previously referred another node via
// ReferNonBroker. This message indicates that the referral was rejected. No
// link to the referred node has been established by the broker, and none will
// be provided to the referrer. This can occur for example if the referred node
// disconnects from the broker before establishing a handshake.
IPCZ_MSG_BEGIN(NonBrokerReferralRejected, IPCZ_MSG_ID(6))
  IPCZ_MSG_BEGIN_VERSION(0)
    // A unique identifier for the referral in question, as provided by the
    // original ReferNonBroker message sent by the receipient of this message.
    IPCZ_MSG_PARAM(uint64_t, referral_id)
  IPCZ_MSG_END_VERSION(0)
IPCZ_MSG_END()

// Sent from one broker to another to establish an initial link between two
// distinct node networks. Once two brokers are connected their networks are
// effectively merged and nodes in either network can become interconnected by
// ipcz.
IPCZ_MSG_BEGIN(ConnectFromBrokerToBroker, IPCZ_MSG_ID(7))
  IPCZ_MSG_BEGIN_VERSION(0)
    // The name of the sending node.
    IPCZ_MSG_PARAM(NodeName, name)

    // The highest protocol version known and desired by the sending broker.
    IPCZ_MSG_PARAM(uint32_t, protocol_version)

    // The number of initial portals assumed by the sending broker on its end of
    // the link established by this handshake.
    IPCZ_MSG_PARAM(uint32_t, num_initial_portals)

    // A primary buffer memory object which may be used to establish a
    // NodeLinkMemory between the two brokers. Since each broker sends a memory
    // object during this handshake, the one to use for the primary buffer is
    // determined by whichever broker's name is the lesser of the two when
    // compared numerically.
    IPCZ_MSG_PARAM_DRIVER_OBJECT(buffer)

    // Explicit padding to preserve 8-byte size alignment.
    IPCZ_MSG_PARAM(uint32_t, padding)
  IPCZ_MSG_END_VERSION(0)

  IPCZ_MSG_BEGIN_VERSION(1)
    // Features enabled on the sending broker.
    IPCZ_MSG_PARAM_ARRAY(Features::Bitfield, features)
  IPCZ_MSG_END_VERSION(1)
IPCZ_MSG_END()

// Sent to a broker to ask for an introduction to one of the non-broker nodes in
// its own network. This may be sent either from a non-broker in the same
// network, or a broker from another network.
//
// The non-broker to be introduced to the sender is identified by `name`. If the
// broker is willing and able to comply with this request, it will send an
// AcceptIntroduction message (see below) to both the sender of this message and
// the node identified by `name`.
//
// If the broker does not know the node named `name`, it will send only a
// RejectIntroduction message back to the sender to indicate failure.
IPCZ_MSG_BEGIN(RequestIntroduction, IPCZ_MSG_ID(10))
  IPCZ_MSG_BEGIN_VERSION(0)
    IPCZ_MSG_PARAM(NodeName, name)
  IPCZ_MSG_END_VERSION(0)
IPCZ_MSG_END()

// Introduces one node to another. Sent only by broker nodes and must only be
// accepted from broker nodes.
IPCZ_MSG_BEGIN(AcceptIntroduction, IPCZ_MSG_ID(11))
  IPCZ_MSG_BEGIN_VERSION(0)
    // The name of the node being introduced to the recipient of this message.
    IPCZ_MSG_PARAM(NodeName, name)

    // Indicates which nominal side of the link (A or B) the recipient must
    // assume for the NodeLink it will establish over `transport`.
    IPCZ_MSG_PARAM(LinkSide, link_side)

    // Indicates the type of the remote node being introduced.
    IPCZ_MSG_PARAM(NodeType, remote_node_type)

    // Explicit padding to preserve 4-byte alignment of the following field.
    IPCZ_MSG_PARAM(uint16_t, padding)

    // Indicates the highest ipcz protocol version which the remote side of
    // `transport` able and willing to use according to the broker.
    IPCZ_MSG_PARAM(uint32_t, remote_protocol_version)

    // The DriverTransport which should be used by the recipient to establish a
    // new NodeLink to the named node. The transport's peer endpoint will be
    // given by the broker to the node identified by `name`.
    IPCZ_MSG_PARAM_DRIVER_OBJECT(transport)

    // A DriverMemory object which should adopted for the NodeLinkMemory
    // instance of the newly established NodeLink. This becomes the new
    // NodeLinkMemory's primary buffer.
    IPCZ_MSG_PARAM_DRIVER_OBJECT(memory)
  IPCZ_MSG_END_VERSION(0)

  IPCZ_MSG_BEGIN_VERSION(1)
    // Features enabled on the remote node being introduced.
    IPCZ_MSG_PARAM_ARRAY(Features::Bitfield, remote_features)
  IPCZ_MSG_END_VERSION(1)
IPCZ_MSG_END()

// Sent to a node in response to RequestIntroduction if the broker receiving
// that message did not recognize or otherwise could not introduce the requested
// node.
IPCZ_MSG_BEGIN(RejectIntroduction, IPCZ_MSG_ID(12))
  IPCZ_MSG_BEGIN_VERSION(0)
    // The name of the node whose introduction cannot be fulfilled.
    IPCZ_MSG_PARAM(NodeName, name)
  IPCZ_MSG_END_VERSION(0)
IPCZ_MSG_END()

// Sent from a broker to another broker to request that the receiving broker
// introduce a named node in its own network to a named node in the sender's
// network.
IPCZ_MSG_BEGIN(RequestIndirectIntroduction, IPCZ_MSG_ID(13))
  IPCZ_MSG_BEGIN_VERSION(0)
    // The name of the node to be introduced on the sender's own network.
    IPCZ_MSG_PARAM(NodeName, source_node)

    // The name of the node to be introduced on the recipient's own network.
    IPCZ_MSG_PARAM(NodeName, target_node)
  IPCZ_MSG_END_VERSION(0)
IPCZ_MSG_END()

// Shares a new buffer to support allocation of blocks of `block_size` bytes.
// The sender must initialize an appropriate BlockAllocator within the buffer's
// memory before sending this message.
IPCZ_MSG_BEGIN(AddBlockBuffer, IPCZ_MSG_ID(14))
  IPCZ_MSG_BEGIN_VERSION(0)
    // The ID of the new buffer as allocated by the NodeLinkMemory on the
    // NodeLink transmitting this message.
    IPCZ_MSG_PARAM(BufferId, id)

    // The size of blocks which can be allocated from within this buffer.
    IPCZ_MSG_PARAM(uint32_t, block_size)

    // A handle to the driver-managed, read-write-mappable buffer.
    IPCZ_MSG_PARAM_DRIVER_OBJECT(buffer)
  IPCZ_MSG_END_VERSION(0)
IPCZ_MSG_END()

// Conveys the contents of a parcel.
IPCZ_MSG_BEGIN(AcceptParcel, IPCZ_MSG_ID(20))
  IPCZ_MSG_BEGIN_VERSION(0)
    // The SublinkId linking the source and destination Routers along the
    // transmitting NodeLink.
    IPCZ_MSG_PARAM(SublinkId, sublink)

    // The SequenceNumber of this parcel within the transmitting portal's
    // outbound parcel sequence (and the receiving portal's inbound parcel
    // sequence.)
    IPCZ_MSG_PARAM(SequenceNumber, sequence_number)

    // For any given Parcel in a portal's sequence, there may be any number of
    // additional subparcels transmitted separately and indexed sequentially
    // starting from 1. This is the subparcel index for this transmitted Parcel.
    // If 0, then this is the main Parcel for the given SequenceNumber.
    IPCZ_MSG_PARAM(uint32_t, subparcel_index)

    // The total number of subparcels belonging to this Parcel or its main
    // containing Parcel. If no subparcels are associated with this Parcel, this
    // value is 1.
    //
    // Note that subparcels themselves never contain other subparcels, so for
    // subparcels this field always conveys the number of subparcels belonging
    // to its main containing Parcel.
    IPCZ_MSG_PARAM(uint32_t, num_subparcels)

    // An optional shared memory fragment containing this parcel's data. If this
    // is null, the parcel data is instead inlined via the `parcel_data` array
    // above.
    IPCZ_MSG_PARAM(FragmentDescriptor, parcel_fragment)

    // Free-form array of application-provided data bytes for this parcel. This
    // field is only meaningful if `parcel_fragment` is null.
    IPCZ_MSG_PARAM_ARRAY(uint8_t, parcel_data)

    // Array of handle types, with each corresponding to a single IpczHandle
    // attached to the parcel.
    IPCZ_MSG_PARAM_ARRAY(HandleType, handle_types)

    // For every portal handle attached, there is also a RouterDescriptor
    // encoding the details needed to construct a new Router at the parcel's
    // destination to extend the transmitted portal's route there.
    IPCZ_MSG_PARAM_ARRAY(RouterDescriptor, new_routers)

    // Explicit padding to preserve 8-byte alignment.
    IPCZ_MSG_PARAM(uint32_t, padding)

    // Every DriverObject boxed and attached to this parcel has an entry in this
    // array.
    IPCZ_MSG_PARAM_DRIVER_OBJECT_ARRAY(driver_objects)
  IPCZ_MSG_END_VERSION(0)
IPCZ_MSG_END()

// Conveys partial parcel contents, namely just its attached driver objects.
// When a parcel with driver objects cannot be transmitted directly to its
// destination, this message is split off and relayed through the broker while
// the rest of the parcel contents are sent directly, without the objects
// attached. The receiving node can reconstitute the full parcel once both
// messages are received.
IPCZ_MSG_BEGIN(AcceptParcelDriverObjects, IPCZ_MSG_ID(21))
  IPCZ_MSG_BEGIN_VERSION(0)
    // The SublinkId linking the source and destination Routers along the
    // transmitting NodeLink.
    IPCZ_MSG_PARAM(SublinkId, sublink)

    // The SequenceNumber of this parcel within the transmitting portal's
    // outbound parcel sequence (and the receiving portal's inbound parcel
    // sequence.)
    IPCZ_MSG_PARAM(SequenceNumber, sequence_number)

    // The driver objects to be accepted.
    IPCZ_MSG_PARAM_DRIVER_OBJECT_ARRAY(driver_objects)
  IPCZ_MSG_END_VERSION(0)
IPCZ_MSG_END()

// Notifies a node that the route has been closed on one side. This message
// always pertains to the side of the route opposite of the router receiving it,
// guaranteed by the fact that the closed side of the route only transmits this
// message outward once its terminal router is adjacent to the central link.
IPCZ_MSG_BEGIN(RouteClosed, IPCZ_MSG_ID(22))
  IPCZ_MSG_BEGIN_VERSION(0)
    // In the context of the receiving NodeLink, this identifies the specific
    // Router to receive this message.
    IPCZ_MSG_PARAM(SublinkId, sublink)

    // The total number of parcels sent from the side of the route which closed,
    // before closing. Because parcels may arrive out-of-order from each other
    // and from messages like this one under various conditions (broker relays,
    // different transport mechanisms, etc.), parcels are tagged with strictly
    // increasing SequenceNumbers by the sender. This field informs the
    // recipient that the closed endpoint has transmitted exactly
    // `sequence_length` parcels, from SequenceNumber 0 to `sequence_length-1`.
    // The recipient can use this to know, for example, that it must still
    // expect some additional parcels to arrive before completely forgetting
    // about the route's link(s).
    IPCZ_MSG_PARAM(SequenceNumber, sequence_length)
  IPCZ_MSG_END_VERSION(0)
IPCZ_MSG_END()

// Notifies a specific router that its route from the direction of this link has
// been unexpectedly disconnected (e.g. due to a node crashing). This is
// essentially the same as route closure but without respect for complete parcel
// sequence delivery.
IPCZ_MSG_BEGIN(RouteDisconnected, IPCZ_MSG_ID(23))
  IPCZ_MSG_BEGIN_VERSION(0)
    IPCZ_MSG_PARAM(SublinkId, sublink)
  IPCZ_MSG_END_VERSION(0)
IPCZ_MSG_END()

// Informs a router that its outward peer can be bypassed. Given routers X and Y
// on the central link, and a router Z as Y's inward peer:
//
//     X ==== (central) ==== Y ======== Z
//
// Once Y successfully locks the central link, Y may send this message to Z
// with sufficient information for Z to establish a direct link to X. Z
// accomplishes this via an AcceptBypassLink message to X's node.
//
// Note that this message is only used when X and Y belong to different nodes.
// If X and Y belong to the same node, then Y sends Z a BypassPeerWithLink
// message instead.
IPCZ_MSG_BEGIN(BypassPeer, IPCZ_MSG_ID(30))
  IPCZ_MSG_BEGIN_VERSION(0)
    // Identifies the router to receive this message.
    IPCZ_MSG_PARAM(SublinkId, sublink)

    // Padding for NodeName alignment. Reserved for future use and must be zero.
    IPCZ_MSG_PARAM(uint64_t, reserved0)

    // The name of the node where router X lives. That is the outward peer of
    // the recipient's outward peer.
    IPCZ_MSG_PARAM(NodeName, bypass_target_node)

    // The sublink used to route between the recipient's outward peer and that
    // router's own outward peer; i.e., the link between X and Y.
    IPCZ_MSG_PARAM(SublinkId, bypass_target_sublink)
  IPCZ_MSG_END_VERSION(0)
IPCZ_MSG_END()

// Provides a router with a new outward link to replace its existing outward
// link to some other node. Given routers X and Y on the central link, and a
// router Z as Y's inward peer:
//
//     X ==== (central) ==== Y ======== Z
//
// Z sends this message to X's node to establish a new direct link to X. Both
// X's and Z's existing links to Y are left intact in a decaying state:
//
//         - - - Y - - -
//       /               \
//     X === (central) === Z
//
// The recipient of this message must send a StopProxying message to Y, as well
// as a ProxyWillStop message to Z, in order for those decaying links to be
// phased out.
//
// Z must send this message to X only after receiving a BypassPeer request from
// Y. That request signifies that X's node has been adequately prepared by Y to
// authenticate this request from Z.
IPCZ_MSG_BEGIN(AcceptBypassLink, IPCZ_MSG_ID(31))
  IPCZ_MSG_BEGIN_VERSION(0)
    // Identifies the node of the targeted router's own outward peer, as well as
    // the sublink their nodes use to route between those routers. In the above
    // scenario these fields identify the link between X and Y to be replaced,
    // and as a consequence they uniquely identify X itself to the recipient.
    IPCZ_MSG_PARAM(NodeName, current_peer_node)
    IPCZ_MSG_PARAM(SublinkId, current_peer_sublink)

    // The length of the parcel sequence routed from Z to Y before Z adopted X
    // as its new outward peer, which is implicitly also the final length of the
    // parcel sequence to be routed from Y to X before that link is dropped.
    IPCZ_MSG_PARAM(SequenceNumber, inbound_sequence_length_from_bypassed_link)

    // A new sublink which can now be used to route messages between X and Z on
    // the NodeLink transmitting this AcceptBypassLink message.
    IPCZ_MSG_PARAM(SublinkId, new_sublink)

    // The shared memory location of the new link's RouterLinkState. This may be
    // null if one could not be allocated ahead of time, in which case one will
    // be allocated and shared later.
    IPCZ_MSG_PARAM(FragmentDescriptor, new_link_state_fragment)
  IPCZ_MSG_END_VERSION(0)
IPCZ_MSG_END()

// Informs a router about how many more parcels it can expect to receive from
// its inward and outward peers before it can stop proxying between them and
// cease to exist. Given routers X, Y, and Z in a configuration resulting from
// a BypassPeer from Y to Z, followed by an AcceptBypassLink from Z to X:
//
//         - - - Y - - -
//       /               \
//     X === (central) === Z
//
// This message is sent from X to Y to provide the final length of the sequence
// of parcels routed through Y in either direction, now that X and Z have
// established a direct connection.
IPCZ_MSG_BEGIN(StopProxying, IPCZ_MSG_ID(32))
  IPCZ_MSG_BEGIN_VERSION(0)
    // Identifies the router to receive this message.
    IPCZ_MSG_PARAM(SublinkId, sublink)

    // The final sequence length of inbound parcels the router can expect from
    // its outward peer. In the scenario above, this is the sequence of parcels
    // routed from X to Y.
    IPCZ_MSG_PARAM(SequenceNumber, inbound_sequence_length)

    // The final sequence length of outbound parcels the router can expect from
    // its inward peer. In the scenario above, this is the sequence of parcels
    // routed from Z to Y.
    IPCZ_MSG_PARAM(SequenceNumber, outbound_sequence_length)
  IPCZ_MSG_END_VERSION(0)
IPCZ_MSG_END()

// Informs a router about how many more parcels it can expect to receive from
// its outward peer. Given routers X, Y, and Z in a configuration resulting from
// a BypassPeer from Y to Z, followed by an AcceptBypassLink from Z to X:
//
//         - - - Y - - -
//       /               \
//     X === (central) === Z
//
// This message is sent from X to Z to provide the final length of the sequence
// of parcels routed from X to Y (and therefore from Y to Z), now that X and Z
// have established a direct connection.
IPCZ_MSG_BEGIN(ProxyWillStop, IPCZ_MSG_ID(33))
  IPCZ_MSG_BEGIN_VERSION(0)
    // Identifies the router to receive this message.
    IPCZ_MSG_PARAM(SublinkId, sublink)

    // The final sequence length of inbound parcels the router can expect from
    // its outward peer.
    IPCZ_MSG_PARAM(SequenceNumber, inbound_sequence_length)
  IPCZ_MSG_END_VERSION(0)
IPCZ_MSG_END()

// Informs a router that its outward peer can be bypassed, and provides a new
// link with which to execute that bypass. Given the following arrangement where
// X, Y, and Z are routers; AND X and Y live on the same node as each other:
//
//     X ==== (central) ==== Y ======== Z
//
// Y sends this to Z to establish a new link (over the same NodeLink) directly
// between Z and X. Both X's and Z's existing links to Y are left intact in a
// decaying state:
//
//         - - - Y - - -
//       /               \
//     X === (central) === Z
//
// Note that unlike with BypassPeer/AcceptBypassLink, there is no need to
// authenticate this request, as it's only swapping one sublink out for another
// along the same NodeLink.
IPCZ_MSG_BEGIN(BypassPeerWithLink, IPCZ_MSG_ID(34))
  IPCZ_MSG_BEGIN_VERSION(0)
    // Identifies the router to receive this message.
    IPCZ_MSG_PARAM(SublinkId, sublink)

    // A new sublink which can now be used to route messages between X and Z on
    // the NodeLink transmitting this BypassPeerWithLink message.
    IPCZ_MSG_PARAM(SublinkId, new_sublink)

    // The shared memory location of the new link's RouterLinkState. This may be
    // null if one could not be allocated ahead of time, in which case one will
    // be allocated and shared later.
    IPCZ_MSG_PARAM(FragmentDescriptor, new_link_state_fragment)

    // The final length of the sequence of inbound parcels the recipient Z can
    // expect to receive from Y. Parcels beyond this point come directly from X
    // over the newly established link.
    IPCZ_MSG_PARAM(SequenceNumber, inbound_sequence_length)
  IPCZ_MSG_END_VERSION(0)
IPCZ_MSG_END()

// Provides a router with the final length of the sequence of outbound parcels
// that will be routed to it via its decaying inward link. Given the following
// arrangement where X, Y, and Z are routers; X and Y live on the same node
// as each other: and Y has already sent a BypassPeerWithLink message to Z:
//
//         - - - Y - - -
//       /               \
//     X === (central) === Z
//
// This message is sent from Z back to Y, informing Y of the last parcel it can
// expect to receive from Z, now that X and Z are connected directly.
IPCZ_MSG_BEGIN(StopProxyingToLocalPeer, IPCZ_MSG_ID(35))
  IPCZ_MSG_BEGIN_VERSION(0)
    // Identifies the router to receive this message.
    IPCZ_MSG_PARAM(SublinkId, sublink)

    // The final length of the sequence of outbound parcels the recipient Y can
    // expect to receive from the sender Z. Beyond this point, parcels are no
    // longer routed through Y in that direction.
    IPCZ_MSG_PARAM(SequenceNumber, outbound_sequence_length)
  IPCZ_MSG_END_VERSION(0)
IPCZ_MSG_END()

// Hints to the target router that it should flush its state. Generally sent to
// catalyze route reduction or elicit some other state change which was blocked
// on some other work being done first by the sender of this message.
IPCZ_MSG_BEGIN(FlushRouter, IPCZ_MSG_ID(36))
  IPCZ_MSG_BEGIN_VERSION(0)
    IPCZ_MSG_PARAM(SublinkId, sublink)
  IPCZ_MSG_END_VERSION(0)
IPCZ_MSG_END()

// Requests allocation of a shared memory region of a given size. If the
// recipient can comply, they will send back a corresponding ProvideMemory
// message with a serialized memory region. This message is only sent to a
// node's allocation delegate (usually the broker), which is established by
// providing the IPCZ_CONNECT_NODE_TO_ALLOCATION_DELEGATE flag to ConnectNode().
IPCZ_MSG_BEGIN(RequestMemory, IPCZ_MSG_ID(64))
  IPCZ_MSG_BEGIN_VERSION(0)
    IPCZ_MSG_PARAM(uint32_t, size)
    IPCZ_MSG_PARAM(uint32_t, padding)
  IPCZ_MSG_END_VERSION(0)
IPCZ_MSG_END()

// Provides a new shared buffer to the receiver, owned exclusively by the
// receiver. The receiver is free to duplicate this buffer and share it with
// other nodes.
IPCZ_MSG_BEGIN(ProvideMemory, IPCZ_MSG_ID(65))
  IPCZ_MSG_BEGIN_VERSION(0)
    IPCZ_MSG_PARAM(uint32_t, size)
    IPCZ_MSG_PARAM_DRIVER_OBJECT(buffer)
  IPCZ_MSG_END_VERSION(0)
IPCZ_MSG_END()

// Sends a message payload to the broker to be relayed to another node. Used to
// relay messages which carry driver objects through the broker when they cannot
// be transmitted directly between their source and destination nodes.
IPCZ_MSG_BEGIN(RelayMessage, IPCZ_MSG_ID(66))
  IPCZ_MSG_BEGIN_VERSION(0)
    // The node to which this message's contents should ultimately be relayed.
    IPCZ_MSG_PARAM(NodeName, destination)

    // The actual serialized message to be relayed, including its own header.
    IPCZ_MSG_PARAM_ARRAY(uint8_t, data)

    // Padding to preserve 8-byte alignment of the `driver_objects` field below.
    IPCZ_MSG_PARAM(uint32_t, padding)

    // The set of driver objects to be relayed along with `data`.
    IPCZ_MSG_PARAM_DRIVER_OBJECT_ARRAY(driver_objects)
  IPCZ_MSG_END_VERSION(0)
IPCZ_MSG_END()

// Relays a message payload from an intermediate broker to its destination. This
// is the continuation of RelayMessage above. Must only be accepted on a broker.
IPCZ_MSG_BEGIN(AcceptRelayedMessage, IPCZ_MSG_ID(67))
  IPCZ_MSG_BEGIN_VERSION(0)
    // The node which originally requested that the broker relay this message.
    IPCZ_MSG_PARAM(NodeName, source)

    // The full serialized data of the relayed message.
    IPCZ_MSG_PARAM_ARRAY(uint8_t, data)

    // Padding to preserve 8-byte alignment of the `driver_objects` field below.
    IPCZ_MSG_PARAM(uint32_t, padding)

    // The set of driver objects relayed along with `data`.
    IPCZ_MSG_PARAM_DRIVER_OBJECT_ARRAY(driver_objects)
  IPCZ_MSG_END_VERSION(0)
IPCZ_MSG_END()

IPCZ_MSG_END_INTERFACE()
