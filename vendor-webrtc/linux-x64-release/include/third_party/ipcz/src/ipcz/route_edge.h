// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_IPCZ_ROUTE_EDGE_H_
#define IPCZ_SRC_IPCZ_ROUTE_EDGE_H_

#include <optional>

#include "ipcz/router_link.h"
#include "ipcz/sequence_number.h"
#include "ipcz/sublink_id.h"
#include "third_party/abseil-cpp/absl/base/macros.h"
#include "util/ref_counted.h"

namespace ipcz {

class NodeLink;
class Router;
class RouterLink;

// A RouteEdge is responsible for message ingress and egress on one
// (inward-facing or outward-facing) side of a router. Every functioning router
// has one outward edge and (if proxying) one inward edge.
//
// Over the course of its lifetime a RouteEdge may utilize many different
// RouterLinks, but at any moment it has at most two: one "primary" link and one
// "decaying" link.
//
// The decaying link's usage is restricted to transmission and receipt of a
// limited range of parcels based on SequenceNumber, and once all expected
// parcels are sent and received, the link is dropped from the edge.
//
// When a RouteEdge has no decaying link, it may be able to transition its
// primary link to a decaying link, while adopting a new primary link to take
// its place. This process of incremental link replacement is the basis for ipcz
// route reduction.
//
// This object is not thread-safe.
class RouteEdge {
 public:
  RouteEdge();
  RouteEdge(const RouteEdge&) = delete;
  RouteEdge& operator=(const RouteEdge&) = delete;
  ~RouteEdge();

  const Ref<RouterLink>& primary_link() const { return primary_link_; }

  RouterLink* decaying_link() const {
    return decaying_link_ ? decaying_link_->link.get() : nullptr;
  }

  // Indicates whether this edge is stable, meaning it has no decaying link and
  // it is not set to decay the next primary link it adopts.
  bool is_stable() const { return !decaying_link_; }

  // Indicates whether this edge has been marked for decay before being assigned
  // a primary link. When this is true, any assigned primary link will
  // immediately begin to decay.
  bool is_decay_deferred() const {
    return decaying_link_ && !decaying_link_->link;
  }

  // These accessors set the limits on the current (or deferred) decaying link.
  // Once both of these values are set, the decaying link will be dropped from
  // this edge as soon as the link transmits all messages up to (but not
  // including) the SequenceNumber in `length_to_decaying_link_` AND the link
  // receives all messages up to (but not including) the SequenceNumber in
  // `length_from_decaying_link_`.
  void set_length_to_decaying_link(SequenceNumber length) {
    ABSL_ASSERT(!is_stable());
    ABSL_ASSERT(!decaying_link_->outgoing_length);
    decaying_link_->outgoing_length = length;
  }

  void set_length_from_decaying_link(SequenceNumber length) {
    ABSL_ASSERT(!is_stable());
    ABSL_ASSERT(!decaying_link_->incoming_length);
    decaying_link_->incoming_length = length;
  }

  std::optional<SequenceNumber> length_to_decaying_link() const {
    return decaying_link_ ? decaying_link_->outgoing_length : std::nullopt;
  }

  std::optional<SequenceNumber> length_from_decaying_link() const {
    return decaying_link_ ? decaying_link_->incoming_length : std::nullopt;
  }

  // Sets the primary link for this edge. Only valid to call if the edge does
  // not currently have a primary link.
  void SetPrimaryLink(Ref<RouterLink> link);

  // Releases this edge's primary link and returns a reference to it.
  Ref<RouterLink> ReleasePrimaryLink();

  // Releases this edge's decaying link and returns a reference to it.
  Ref<RouterLink> ReleaseDecayingLink();

  // If the primary link is present and is a LocalRouterLink, this returns the
  // Router on the other side of the link. Otherwise it returns null.
  Ref<Router> GetLocalPeer();

  // If the decaying link is present and is a LocalRouterLink, this returns the
  // Router on the other side of the link. Otherwise it returns null.
  Ref<Router> GetDecayingLocalPeer();

  // Sets the current primary link to begin decay; or if there is no primary
  // link yet, marks this edge for deferred decay. In the latter case, the next
  // primary link adopted by this edge will immediately begin to decay. This may
  // only be called while the edge has no decaying link.
  bool BeginPrimaryLinkDecay();

  // Indicates whether a parcel with the given SequenceNumber should be
  // transmitted over this edge's decaying link. If not, the parcel should be
  // transmitted over this edge's primary link.
  bool ShouldTransmitOnDecayingLink(SequenceNumber sequence_number) const;

  // Attempts to drop this edge's decaying link, given that it has already
  // transmitted a parcel sequence up to `length_sent` and received a parcel
  // sequence up to `length_received`. Returns true if the decaying link was
  // dropped, and false otherwise.
  bool MaybeFinishDecay(SequenceNumber length_sent,
                        SequenceNumber length_received);

 private:
  struct DecayingLink {
    // A link which used to be the edge's primary link but which is being
    // phased out. The decaying link may continue to receive parcels, but once
    // `length_from_decaying_link` is set, it will only expect to receive
    // parcels with a SequenceNumber up to (but not including) that value.
    // Similarly, the decaying link will be preferred for message transmission
    // as long as `length_to_decaying_link_` remains unknown, but as soon as
    // that value is set, only parcels with a SequenceNumber up to(but not
    // including) that value will be transmitted over this link. Once both
    // sequence lengths are known and surpassed, the edge will drop this link.
    //
    // This field may be null, indicating that the edge was marked for link
    // decay but hasn't yet been assigned a primary link. When a primary link
    // is assigned in this case, it will immediately begin to decay.
    Ref<RouterLink> link;

    // If present, the length of the parcel sequence after which this edge must
    // stop using `decaying_link_` to transmit parcels. If this is 5, then the
    // decaying link must be used to transmit any new parcels with a
    // SequenceNumber in the range [0, 4] inclusive. Beyond that point the
    // primary link must be used.
    std::optional<SequenceNumber> outgoing_length;

    // If present, the length of the parcel sequence after which this edge can
    // stop expecting to receive parcels over `decaying_link_`. If this is 7,
    // then the Router using this edge should still expect to receive parcels
    // from the decaying link as long as it is missing any parcel in the range
    // [0, 6] inclusive. Beyond that point parcels should only be expected from
    // the primary link.
    std::optional<SequenceNumber> incoming_length;
  };

  // The primary link over which this edge transmits and accepts parcels and
  // other messages. If a decaying link is also present, then the decaying link
  // is preferred for transmission of all parcels with a SequenceNumber up to
  // (but not including) `decaying_link_->outgoing_length`. If that value is not
  // set, the decaying link is always preferred when not null.
  Ref<RouterLink> primary_link_;

  // State regarding the edge's previous primary link which is currently
  // decaying. Only non-null when the edge starts to decay (or will decay ASAP)
  // its primary link.
  std::unique_ptr<DecayingLink> decaying_link_;
};

}  // namespace ipcz

#endif  // IPCZ_SRC_IPCZ_ROUTE_EDGE_H_
