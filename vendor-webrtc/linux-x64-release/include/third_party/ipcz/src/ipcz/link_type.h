// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_IPCZ_LINK_TYPE_H_
#define IPCZ_SRC_IPCZ_LINK_TYPE_H_

#include <string>

namespace ipcz {

// Enumeration indicating what role a specific RouterLink plays along its route.
// Every end-to-end route has exactly one "central" link -- the link that
// connects one side of the route to the other -- along with any number of
// "peripheral" links extending the route away from the central link on one side
// or the other.
//
// If a peripheral link connects a router R1 to another router R2 which is
// closer to the central link, the link is considered to be "outward" from R1
// and "inward" from R2. Conversely if a peripheral link connects a router R1 to
// another router R2 which is *further* from the central link, the link is
// considered to be inward R1 and outward from R2.
//
// Finally, when two routes are joined via the MergePortals API, the merged
// portals' routers are linked together via a bridge link.
//
// The stable state of any given route is to have exactly two routers, both
// terminal, with a single central link between them. Routes which are extended
// by portal relocation or bridged via portal merges may grow into arbitrarily
// long chains of linked routers, but over time all interior routers are
// incrementally bypassed and discarded through a process of link decay and
// replacement.
//
// The different link classifications defined here help with the orchestration
// of this incremental reduction process.
struct LinkType {
  enum class Value {
    // The link along a route which connects one side of the route to the other.
    // This is the only link which is treated by both sides as an outward link,
    // and it's the only link along a route at which decay can be initiated by a
    // router.
    kCentral,

    // Any link along a route which is established to extend the route on one
    // side is a peripheral link. Peripheral links forward parcels and other
    // messages along the same direction in which they were received (e.g.
    // messages from an inward peer via a peripheral link are forwarded
    // outward).
    //
    // Peripheral links can only decay as part of a decaying process initiated
    // on a central link by a mutually adjacent router.
    //
    // Every peripheral link has a side facing inward and a side facing outward.
    // An inward peripheral link goes further toward the terminal endpoint of
    // the router's own side of the route, while an outward peripheral link goes
    // toward the terminal endpoint of the side opposite the router.
    kPeripheralInward,
    kPeripheralOutward,

    // Bridge links are special links formed only when merging two routes
    // together. A bridge link links two terminal routers from two different
    // routes, and it can only decay once both routers are adjacent to decayable
    // central links along their own respective routes; at which point both
    // routes atomically initiate decay of those links to replace them (and the
    // bridge link itself) with a single new central link.
    kBridge,
  };

  static constexpr Value kCentral = Value::kCentral;
  static constexpr Value kPeripheralInward = Value::kPeripheralInward;
  static constexpr Value kPeripheralOutward = Value::kPeripheralOutward;
  static constexpr Value kBridge = Value::kBridge;

  constexpr LinkType() = default;
  constexpr LinkType(Value value) : value_(value) {}

  bool operator==(const LinkType& rhs) const { return value_ == rhs.value_; }
  bool operator!=(const LinkType& rhs) const { return value_ != rhs.value_; }

  bool is_outward() const { return is_central() || is_peripheral_outward(); }
  bool is_central() const { return value_ == Value::kCentral; }
  bool is_peripheral_inward() const {
    return value_ == Value::kPeripheralInward;
  }
  bool is_peripheral_outward() const {
    return value_ == Value::kPeripheralOutward;
  }
  bool is_bridge() const { return value_ == Value::kBridge; }

  Value value() const { return value_; }

  explicit operator Value() const { return value_; }

  std::string ToString() const;

 private:
  Value value_ = Value::kCentral;
};

}  // namespace ipcz

#endif  // IPCZ_SRC_IPCZ_LINK_TYPE_H_
