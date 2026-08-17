// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_IPCZ_ROUTER_H_
#define IPCZ_SRC_IPCZ_ROUTER_H_

#include <cstdint>
#include <utility>

#include "ipcz/fragment_ref.h"
#include "ipcz/ipcz.h"
#include "ipcz/parcel_queue.h"
#include "ipcz/pending_transaction_set.h"
#include "ipcz/route_edge.h"
#include "ipcz/router_descriptor.h"
#include "ipcz/router_link.h"
#include "ipcz/sequence_number.h"
#include "ipcz/sublink_id.h"
#include "ipcz/trap_set.h"
#include "third_party/abseil-cpp/absl/base/thread_annotations.h"
#include "third_party/abseil-cpp/absl/synchronization/mutex.h"
#include "util/ref_counted.h"

namespace ipcz {

class NodeLink;
class RemoteRouterLink;
struct RouterLinkState;
class TrapEventDispatcher;

// The Router is the main primitive responsible for routing parcels between ipcz
// portals. This class is thread-safe.
//
// Before a Router can participate in any actual routing, it must have an
// outward link to another Router (see SetOutwardLink()). To establish a locally
// connected pair of Routers, pass both to LocalRouterLink::Create() and pass
// each returned link to the coresponding router:
//
//     Router::Pair routers = {MakeRefCounted<Router>(),
//                             MakeRefCounted<Router>()};
//     RouterLink::Pair links =
//         LocalRouterLink::CreatePair(LinkType::kCentral, routers);
//     routers.first->SetOutwardLink(std::move(links.first));
//     routers.second->SetOutwardLink(std::move(links.second));
//
// Each ipcz portal handle directly controls a terminal Router along its route,
// and all routes stabilize to eventually consist of only two interconnected
// terminal Routers. When a portal moves, its side of the route is extended by
// creating a new terminal Router at the portal's new location. The previous
// terminal Router remains as a proxying hop to be phased out eventually.
class Router : public APIObjectImpl<Router, APIObject::kPortal> {
 public:
  using Pair = std::pair<Ref<Router>, Ref<Router>>;

  Router();

  // Creates a new pair of terminal routers which are directly connected to each
  // other by a LocalRouterLink.
  static Pair CreatePair();

  // APIObject:
  IpczResult Close() override;
  bool CanSendFrom(Router& sender) override;

  // *Put/*Get APIs exposed through the ipcz API via portal handles.
  IpczResult Put(absl::Span<const uint8_t> data,
                 absl::Span<const IpczHandle> handles);
  IpczResult BeginPut(IpczBeginPutFlags flags,
                      volatile void** data,
                      size_t* num_bytes,
                      IpczTransaction* transaction);
  IpczResult EndPut(IpczTransaction transaction,
                    size_t num_bytes_produced,
                    absl::Span<const IpczHandle> handles,
                    IpczEndPutFlags flags);
  IpczResult Get(IpczGetFlags flags,
                 void* data,
                 size_t* num_data_bytes,
                 IpczHandle* handles,
                 size_t* num_handles,
                 IpczHandle* parcel);
  IpczResult BeginGet(IpczBeginGetFlags flags,
                      const volatile void** data,
                      size_t* num_data_bytes,
                      IpczHandle* handles,
                      size_t* num_handles,
                      IpczTransaction* transaction);
  IpczResult EndGet(IpczTransaction transaction,
                    IpczEndGetFlags flags,
                    IpczHandle* parcel);

  // Indicates whether the terminal router on the other side of the central link
  // is known to be closed.
  bool IsPeerClosed();

  // Indicates whether the terminal router on the other side of the central link
  // is known to be closed AND there are no more inbound parcels to be
  // retrieved.
  bool IsRouteDead();

  // Indicates whether this Router is currently on a central link which is
  // connected to a router on another node. Used by tests to verify route
  // reduction behavior, and may only be called on terminal Routers.
  bool IsOnCentralRemoteLink();

  // Fills in an IpczPortalStatus corresponding to the current state of this
  // Router.
  void QueryStatus(IpczPortalStatus& status);

  // Returns true iff this Router's outward link is a LocalRouterLink between
  // `this` and `router`.
  bool HasLocalPeer(Router& router);

  // Closes this side of the Router's own route. Only called terminal Routers.
  void CloseRoute();

  // Uses `link` as this Router's new outward link. This is the primary link on
  // which the router transmits parcels and control messages directed toward the
  // other side of its route. Must only be called on a Router which has no
  // outward link.
  //
  // NOTE: This is NOT safe to call when the other side of the link is already
  // in active use by another Router, as `this` Router may already be in a
  // transitional state and must be able to block decay around `link` from
  // within this call.
  void SetOutwardLink(const Ref<RouterLink> link);

  // Accepts an inbound parcel from the outward edge of this router, either to
  // queue it for retrieval or forward it further inward. `source` indicates
  // whether the parcel is arriving as a direct result of some local ipcz API
  // call, or if it came from a remote node.
  bool AcceptInboundParcel(std::unique_ptr<Parcel> parcel);

  // Accepts an outbound parcel here from some other Router. The parcel is
  // transmitted immediately or queued for later transmission over the Router's
  // outward link. Called only on proxying Routers.
  bool AcceptOutboundParcel(std::unique_ptr<Parcel> parcel);

  // Accepts notification that the other end of the route has been closed and
  // that the closed end transmitted a total of `sequence_length` parcels before
  // closing. `source` indicates whether the portal's peer was closed locally,
  // or if we were notified of its closure from a remote node.
  bool AcceptRouteClosureFrom(LinkType link_type,
                              SequenceNumber sequence_length);

  // Accepts notification from a link bound to this Router that some node along
  // the route (in the direction of that link) has been disconnected, e.g. due
  // to a crash, and that the route is no longer functional as a result. This is
  // similar to route closure, except no effort can realistically be made to
  // deliver the complete sequence of parcels transmitted from that end of the
  // route. `link_type` specifies the type of link which is propagating the
  // notification to this rouer.
  bool AcceptRouteDisconnectedFrom(LinkType link_type);

  // Attempts to install a new trap on this Router, to invoke `handler` as soon
  // as one or more conditions in `conditions` is met. This method effectively
  // implements the ipcz Trap() API. See its description in ipcz.h for details.
  IpczResult Trap(const IpczTrapConditions& conditions,
                  IpczTrapEventHandler handler,
                  uint64_t context,
                  IpczTrapConditionFlags* satisfied_condition_flags,
                  IpczPortalStatus* status);

  // Attempts to merge this Router's route with the route terminated by `other`.
  // Both `other` and this Router must be terminal routers on their own separate
  // routes, and neither Router must have transmitted or retreived any parcels
  // via Put or Get APIs.
  IpczResult MergeRoute(const Ref<Router>& other);

  // Deserializes a new Router from `descriptor` received over `from_node_link`
  // on `receiving_sublink`.
  static Ref<Router> Deserialize(const RouterDescriptor& descriptor,
                                 NodeLink& from_node_link,
                                 SublinkId receiving_sublink);

  // Serializes a description of a new Router which will be used to extend this
  // Router's route across `to_node_link` by introducing a new Router on the
  // remote node.
  void SerializeNewRouter(NodeLink& to_node_link, RouterDescriptor& descriptor);

  // Configures this Router to begin proxying incoming parcels toward (and
  // outgoing parcels from) the Router described by `descriptor`, living on the
  // remote node of `to_node_link`.
  void BeginProxyingToNewRouter(NodeLink& to_node_link,
                                const RouterDescriptor& descriptor);

  // Notifies this router that it should reach out to its outward peer's own
  // outward peer in order to establish a direct link. `requestor` is the link
  // over which this request arrived, and it must be this router's current
  // outward peer in order for the request to be valid.
  //
  // Note that the requestor and its own outward peer must exist on different
  // nodes in order for this method to be called. `bypass_target_node`
  // identifies the node where that router lives, and `bypass_target_sublink`
  // identifies the Sublink used to route between that router and the requestor;
  // i.e., it identifies the link to be bypassed.
  //
  // If the requestor's own outward peer lives on a different node from this
  // router, this router proceeds with the bypass by allocating a new link
  // between itself and the requestor's outward peer and sharing it with that
  // router's node via an AcceptBypassLink message, which will ultimately invoke
  // AcceptBypassLink() on the targeted router.
  //
  // If the requestor's outward peer lives on the same node as this router,
  // bypass is completed immediately by establishing a new LocalRotuerLink
  // between the two routers. In this case a StopProxying message is sent back
  // to the requestor in order to finalize the bypass.
  //
  // Returns true if the BypassPeer() request was valid, or false if it was
  // invalid. Note that a return value of true does not necessarily imply that
  // bypass was or will be successful (e.g. it may silently fail due to lost
  // node connections).
  bool BypassPeer(RemoteRouterLink& requestor,
                  const NodeName& bypass_target_node,
                  SublinkId bypass_target_sublink);

  // Begins decaying this router's outward link and replaces it with a new link
  // over `new_node_link` via `new_sublink`, and using `new_link_state` for its
  // shared state.
  //
  // `inbound_sequence_length_from_bypassed_link` conveys the final length of
  // sequence of inbound parcels to expect over the decaying link from the peer.
  // See comments on the BypassPeer definition in node_messages_generator.h.
  //
  // Returns true if the request was valid, or false if it was invalid. An
  // invalid request implies that a remote node tried to do something bad and
  // should be disconnected ASAP. Note that a return value of true does not
  // necessarily imply that the bypass link was accepted, as it may be
  // silently discarded if other links have been disconnected already.
  //
  // If `new_node_link` links to a remote node which differs from that of this
  // router's current outward link, the current outward link must have already
  // been configured to accept replacement by the new remote node via its
  // RouterLinkState's `allowed_bypass_request_source` field. This method
  // authenticates the request accordingly.
  bool AcceptBypassLink(
      NodeLink& new_node_link,
      SublinkId new_sublink,
      FragmentRef<RouterLinkState> new_link_state,
      SequenceNumber inbound_sequence_length_from_bypassed_link);

  // Configures the final inbound and outbound sequence lengths of this router's
  // decaying links. Once these lengths are set and sequences have progressed
  // to the specified length in each direction, those decaying links -- and
  // eventually the router itself -- are dropped.
  //
  // Returns true if and only if this router is a proxy with decaying inward and
  // outward links. Otherwise returns false, indicating an invalid request.
  bool StopProxying(SequenceNumber inbound_sequence_length,
                    SequenceNumber outbound_sequence_length);

  // Configures the final length of the inbound parcel sequence coming from the
  // this router's decaying outward link. Once this length is set and the
  // decaying link has forwarded the full sequence of parcels up to this limit,
  // the decaying link can be dropped.
  //
  // Returns true if this router has a decaying outward link -- implying that
  // its outward peer is a proxy -- or the router has been disconnected.
  // Otherwise the request is invalid and this returns false.
  bool NotifyProxyWillStop(SequenceNumber inbound_sequence_length);

  // Configures the final sequence length of outbound parcels to expect on this
  // proxying Router's decaying inward link. Once this is set and the decaying
  // link has received the full sequence of parcels, the link can be dropped.
  //
  // Returns true if the request is valid, meaning that this Router is a proxy
  // whose outward peer is local to the same node. Otherwise this returns false.
  bool StopProxyingToLocalPeer(SequenceNumber outbound_sequence_length);

  // Notifies this Router that one of its links has been disconnected from a
  // remote node. The link is identified by a combination of a specific NodeLink
  // and SublinkId.
  //
  // Note that this is invoked if ANY RemoteRouterLink bound to this router is
  // disconnected at its underlying NodeLink, and the result is aggressive
  // teardown of the route in both directions across any remaining (i.e. primary
  // and/or decaying) links.
  //
  // For a proxying router which is generally only kept alive by the links
  // which are bound to it, this call will typically be followed by imminent
  // destruction of this Router once the caller releases its own reference.
  void NotifyLinkDisconnected(RemoteRouterLink& link);

  // Flushes any inbound or outbound parcels, as well as any route closure
  // notifications. RouterLinks which are no longer needed for the operation of
  // this Router may be deactivated by this call.
  //
  // Since this may be called by many other Router methods, RouterLink
  // implementations must exercise caution when calling into a Router to ensure
  // that their own potentially reentrant deactivation by Flush() won't end up
  // dropping the last reference and deleting `this` before Flush() returns.
  //
  // A safe way to ensure that is for RouterLink implementations to only call
  // into Router using a reference held on the calling stack.
  //
  // The specified FlushBehavior determines whether the Flush() operation will
  // unconditionally attempt to initiate bypass of this Router or its outward
  // peer after performing all other flushing operations. By default, bypass
  // progress is only attempted if the flush iteslf resulted in an unstable
  // central link becoming potentially stable. But various operations which
  // invoke Flush() may also elicit state changes that can unblock a bypass
  // operation. These operatoins may specify kForceProxyBypassAttempt in such
  // cases.
  //
  // `source` indicates why the flush is occurring.
  enum FlushBehavior { kDefault, kForceProxyBypassAttempt };
  void Flush(FlushBehavior behavior = kDefault);

 private:
  friend class RefCounted<Router>;

  ~Router();

  // Allocates an outbound parcel with the intention of eventually sending it
  // from this Router via SendOutboundParcel(). This will always try to allocate
  // exactly `num_bytes` capacity unless `allow_partial` is true; in which case
  // the allocated size may be less than requested. If available, this will also
  // attempt to allocate the parcel data as a fragment of the router's outward
  // link memory.
  std::unique_ptr<Parcel> AllocateOutboundParcel(size_t num_bytes,
                                                 bool allow_partial);

  // Attempts to send an outbound parcel originating from this Router. Called
  // only as a direct result of a Put() or EndPut() call on the router's owning
  // portal.
  IpczResult SendOutboundParcel(std::unique_ptr<Parcel> parcel);

  // Attempts to initiate bypass of this router by its peers, and ultimately to
  // remove this router from its route.
  //
  // Called during a Flush() if this is a proxying router which just dropped its
  // last decaying link, or if Flush() was called with kForceProxyBypassAttempt,
  // indicating that some significant state has changed on the route which might
  // unblock our bypass.
  bool MaybeStartSelfBypass();

  // Starts bypass of this Router when its outward peer lives on the same node.
  // This must only be called once the central link is already locked. If
  // `new_link_state` is non-null, it will be used for the RouterLinkState of
  // the new RemoteRouterLink between this Routers inward and outward peers.
  // Otherwise one will be allocated asynchronously before proceeding.
  //
  // Returns true if and only if self-bypass has been initiated by reaching out
  // to this router's inward peer with with a BypassPeer() or
  // BypassPeerWithLink() request. Otherwise returns false.
  bool StartSelfBypassToLocalPeer(Router& local_outward_peer,
                                  RemoteRouterLink& inward_link,
                                  FragmentRef<RouterLinkState> new_link_state);

  // Attempts to start bypass of this Router, which must be on a bridge link, as
  // well bypassing the bridge link itself and the bridge peer router on its
  // other side. This method will attempt to lock this Router's outward link as
  // well as the outward link of this Router's bridge peer. If either fails,
  // both are left unlocked and this operation cannot yet proceed.
  void MaybeStartBridgeBypass();

  // Starts bypass of this Router, which must be on a bridge link and must have
  // a local outward peer link. The router on the other side of the bridge must
  // have a remote outward peer, and `link_state` if non-null will be used to
  // establish a new remote link to that peer to bypass the entire bridge. If
  // `link_state` is null, the operation will be deferred until a fragment can
  // be allocated.
  void StartBridgeBypassFromLocalPeer(FragmentRef<RouterLinkState> link_state);

  // Attempts to bypass the link identified by `requestor` in favor of a new
  // link that runs over `node_link`. If `new_link_state` is non-null, it will
  // be used for the RouterLinkState of the new RemoteRouterLink; otherwise one
  // will be allocated asynchronously before proceeding.
  //
  // Returns true if and only if this request was valid.
  bool BypassPeerWithNewRemoteLink(RemoteRouterLink& requestor,
                                   NodeLink& node_link,
                                   SublinkId bypass_target_sublink,
                                   FragmentRef<RouterLinkState> new_link_state);

  // Attempts to bypass the link identified by `requestor` in favor of a new
  // LocalRouterLink to a Router bound to `bypass_target_sublink` on the same
  // NodeLink as `requestor`.
  //
  // Returns true if and only if this request was valid.
  bool BypassPeerWithNewLocalLink(RemoteRouterLink& requestor,
                                  SublinkId bypass_target_sublink);

  // Optimized Router serialization case when the Router's peer is local to the
  // same node and the existing (local) central link can be replaced with a new
  // remote link, without establishing an intermediate proxy. Returns true on
  // success, or false indicating that the caller must fall back onto the slower
  // Router serialization path defined below.
  bool SerializeNewRouterWithLocalPeer(NodeLink& to_node_link,
                                       RouterDescriptor& descriptor,
                                       Ref<Router> local_peer);

  // Default Router serialization case when the serializing Router must stay
  // behind as an intermediate proxy between its (remote) peer and the newly
  // established Router that will result from this serialization. As an
  // optimization, `initiate_proxy_bypass` may be true if the serializing router
  // is on the central link and was able to lock that link for bypass prior to
  // serialization.
  void SerializeNewRouterAndConfigureProxy(NodeLink& to_node_link,
                                           RouterDescriptor& descriptor,
                                           bool initiate_proxy_bypass);

  std::unique_ptr<Parcel> TakeNextInboundParcel(TrapEventDispatcher& dispatcher)
      ABSL_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  absl::Mutex mutex_;

  // Indicates whether the opposite end of the route has been closed. This is
  // the source of truth for peer closure status. The status bit
  // (IPCZ_PORTAL_STATUS_PEER_CLOSED) within `status_flags_`, and the
  // corresponding trap condition (IPCZ_TRAP_PEER_CLOSED) are only raised when
  // this is true AND we are not expecting any more in-flight parcels.
  bool is_peer_closed_ ABSL_GUARDED_BY(mutex_) = false;

  // Tracks whether this router has been unexpectedly disconnected from its
  // links. This may be used to prevent additional links from being established.
  bool is_disconnected_ ABSL_GUARDED_BY(mutex_) = false;

  // If `pending_gets_` has only one transaction, this indicates whether it's
  // exclusive. An exclusive transaction must return its Parcel to the head
  // element of `inbound_parcels_` if aborted.
  bool is_pending_get_exclusive_ ABSL_GUARDED_BY(mutex_) = false;

  // The current computed portal status flags state, to be reflected by a portal
  // controlling this router iff this is a terminal router.
  IpczPortalStatusFlags status_flags_ ABSL_GUARDED_BY(mutex_) = IPCZ_NO_FLAGS;

  // A set of traps installed via a controlling portal where applicable. These
  // traps are notified about any interesting state changes within the router.
  TrapSet traps_ ABSL_GUARDED_BY(mutex_);

  // The edge connecting this router outward to another, toward the portal on
  // the other side of the route.
  RouteEdge outward_edge_ ABSL_GUARDED_BY(mutex_);

  // The edge connecting this router inward to another, closer to the portal on
  // our own side of the route. Only present for proxying routers: terminal
  // routers by definition can have no inward edge.
  std::unique_ptr<RouteEdge> inward_edge_ ABSL_GUARDED_BY(mutex_);

  // A special inward edge which when present bridges this route with another
  // route. This is used only to implement route merging.
  std::unique_ptr<RouteEdge> bridge_ ABSL_GUARDED_BY(mutex_);

  // Parcels received from the other end of the route. If this is a terminal
  // router, these may be retrieved by the application via a controlling portal;
  // otherwise they will be forwarded along `inward_edge_` as soon as possible.
  ParcelQueue inbound_parcels_ ABSL_GUARDED_BY(mutex_);

  // Parcels transmitted directly from this router (if sent by a controlling
  // portal) or received from an inward peer which sent them outward toward this
  // Router. These parcels generally only accumulate if there is no outward link
  // present when attempting to transmit them, and they are forwarded along
  // `outward_edge_` as soon as possible.
  ParcelQueue outbound_parcels_ ABSL_GUARDED_BY(mutex_);

  // The set of pending get transactions in progress on this router.
  std::unique_ptr<PendingTransactionSet> pending_gets_ ABSL_GUARDED_BY(mutex_);

  // The set of pending get transactions in progress on this router.
  std::unique_ptr<PendingTransactionSet> pending_puts_;
};

}  // namespace ipcz

#endif  // IPCZ_SRC_IPCZ_ROUTER_H_
