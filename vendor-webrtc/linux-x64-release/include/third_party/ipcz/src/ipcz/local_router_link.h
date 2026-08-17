// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_IPCZ_LOCAL_ROUTER_LINK_H_
#define IPCZ_SRC_IPCZ_LOCAL_ROUTER_LINK_H_

#include "ipcz/link_side.h"
#include "ipcz/router.h"
#include "ipcz/router_link.h"
#include "util/ref_counted.h"

namespace ipcz {

struct RouterLinkState;

// Local link between two Routers on the same node. This class is thread-safe.
//
// NOTE: This implementation must take caution when calling into any Router. See
// note on RouterLink's own class documentation.
class LocalRouterLink : public RouterLink {
 public:
  // Creates a new pair of LocalRouterLinks linking the given pair of Routers
  // together. `type` must be either kCentral or kBridge, as local links may
  // never be peripheral. `initial_state` determines whether the new link starts
  // in a stable state.
  //
  // It is the caller's responsibilty to give the returned links to their
  // respective Routers.
  enum InitialState { kUnstable, kStable };
  static RouterLink::Pair CreatePair(LinkType type,
                                     const Router::Pair& routers,
                                     InitialState initial_state = kUnstable);

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
                  SublinkId bypass_target_sublink) override;
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
  class SharedState;

  LocalRouterLink(LinkSide side, Ref<SharedState> state);
  ~LocalRouterLink() override;

  const LinkSide side_;
  const Ref<SharedState> state_;
};

}  // namespace ipcz

#endif  // IPCZ_SRC_IPCZ_LOCAL_ROUTER_LINK_H_
