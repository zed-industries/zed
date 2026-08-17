// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_IPCZ_REMOTE_ROUTER_LINK_H_
#define IPCZ_SRC_IPCZ_REMOTE_ROUTER_LINK_H_

#include <atomic>
#include <functional>
#include <vector>

#include "ipcz/fragment_ref.h"
#include "ipcz/link_side.h"
#include "ipcz/link_type.h"
#include "ipcz/router_link.h"
#include "ipcz/router_link_state.h"
#include "ipcz/sublink_id.h"
#include "third_party/abseil-cpp/absl/synchronization/mutex.h"
#include "util/ref_counted.h"

namespace ipcz {

class NodeLink;

// One side of a link between two Routers living on different nodes. A
// RemoteRouterLink uses a NodeLink plus a SublinkId as its transport between
// the routers. On the other end (on another node) is another RemoteRouterLink
// using a NodeLink back to this node, with the same SublinkId.
//
// As with other RouterLink instances, each RemoteRouterLink is assigned a
// LinkSide at construction time. This assignment is arbitrary but will always
// be the opposite of the LinkSide assigned to the RemoteRouteLink on the other
// end.
//
// NOTE: This implementation must take caution when calling into any Router. See
// note on RouterLink's own class documentation.
class RemoteRouterLink : public RouterLink {
 public:
  // Constructs a new RemoteRouterLink which sends messages over `node_link`
  // using `sublink` specifically. `side` is the side of this link on which
  // this RemoteRouterLink falls (side A or B), and `type` indicates what type
  // of link it is -- which for remote links must be either kCentral,
  // kPeripheralInward, or kPeripheralOutward. If the link is kCentral, a
  // non-null `link_state` must be provided for the link's RouterLinkState.
  static Ref<RemoteRouterLink> Create(Ref<NodeLink> node_link,
                                      SublinkId sublink,
                                      FragmentRef<RouterLinkState> link_state,
                                      LinkType type,
                                      LinkSide side);

  const Ref<NodeLink>& node_link() const { return node_link_; }
  SublinkId sublink() const { return sublink_; }

  // RouterLink:
  LinkType GetType() const override;
  RouterLinkState* GetLinkState() const override;
  void WaitForLinkStateAsync(std::function<void()> callback) override;
  Ref<Router> GetLocalPeer() override;
  RemoteRouterLink* AsRemoteRouterLink() override;
  void AllocateParcelData(size_t num_bytes,
                          bool allow_partial,
                          Parcel& parcel) override;
  void AcceptParcel(std::unique_ptr<Parcel> parcel) override;
  void AcceptRouteClosure(SequenceNumber sequence_length) override;
  void AcceptRouteDisconnected() override;
  void MarkSideStable() override;
  bool TryLockForBypass(const NodeName& bypass_request_source) override;
  bool TryLockForClosure() override;
  void Unlock() override;
  bool FlushOtherSideIfWaiting() override;
  bool CanNodeRequestBypass(const NodeName& bypass_request_source) override;
  void BypassPeer(const NodeName& bypass_target_node,
                  SublinkId bypass_request_sublink) override;
  void StopProxying(SequenceNumber inbound_sequence_length,
                    SequenceNumber outbound_sequence_length) override;
  void ProxyWillStop(SequenceNumber inbound_sequence_length) override;
  void BypassPeerWithLink(SublinkId new_sublink,
                          FragmentRef<RouterLinkState> new_link_state,
                          SequenceNumber inbound_sequence_length) override;
  void StopProxyingToLocalPeer(
      SequenceNumber outbound_sequence_length) override;
  void Deactivate() override;
  std::string Describe() const override;

 private:
  RemoteRouterLink(Ref<NodeLink> node_link,
                   SublinkId sublink,
                   FragmentRef<RouterLinkState> link_state,
                   LinkType type,
                   LinkSide side);

  ~RemoteRouterLink() override;

  // Sets this link's RouterLinkState. `state` must be pending or addressable
  // and this must be a central link.
  void SetLinkState(FragmentRef<RouterLinkState> state);

  // Flushes the link's local router.
  void FlushRouter();

  const Ref<NodeLink> node_link_;
  const SublinkId sublink_;
  const LinkType type_;
  const LinkSide side_;

  // Local atomic cache of whether this side of the link is marked stable. If
  // MarkSideStable() is called when no RouterLinkState is present, this will be
  // used to remember it once a RouterLinkState is finally established.
  std::atomic<bool> side_is_stable_{false};

  // A reference to the shared memory Fragment containing the RouterLinkState
  // shared by both ends of this RouterLink. Only used by central links. Once
  // this is set to a non-null fragment and that fragment is addressable by this
  // link's node, `link_state_` is also updated to cache a pointer to this
  // fragment's mapped memory.
  FragmentRef<RouterLinkState> link_state_fragment_;

  // Cached address of the shared RouterLinkState referenced by
  // `link_state_fragment_`. Once this is set to a non-null value it retains
  // that value indefinitely, so any non-null value loaded from this field is
  // safe to dereference for the duration of the RemoteRouterLink's lifetime.
  std::atomic<RouterLinkState*> link_state_{nullptr};

  // Set of callbacks to be invoked as soon as this link has a RouterLinkState.
  absl::Mutex mutex_;
  std::vector<std::function<void()>> link_state_callbacks_
      ABSL_GUARDED_BY(mutex_);
};

}  // namespace ipcz

#endif  // IPCZ_SRC_IPCZ_REMOTE_ROUTER_LINK_H_
