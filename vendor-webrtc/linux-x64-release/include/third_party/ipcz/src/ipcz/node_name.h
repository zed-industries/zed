// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_IPCZ_NODE_NAME_H_
#define IPCZ_SRC_IPCZ_NODE_NAME_H_

#include <cstdint>
#include <string>
#include <tuple>
#include <utility>

#include "ipcz/ipcz.h"

namespace ipcz {

// A NodeName is a 128-bit UUID used to uniquely identify a node within a
// connected graph of ipcz nodes.
//
// Names are assigned by a broker node to any connecting client, and clients are
// introduced to each other by name, exclusively through a broker. This means
// that the remote node identity on every NodeLink is authoritative and node
// identities cannot be spoofed without the help of a broker node.
//
// NodeNames are randomly generated and hard to guess, but they are not kept
// secret and their knowledge alone is not enough to exfiltrate messages from or
// otherwise interfere with any existing routes running through the named node.
class IPCZ_ALIGN(8) NodeName {
 public:
  constexpr NodeName() = default;
  constexpr NodeName(uint64_t high, uint64_t low) : high_(high), low_(low) {}

  bool is_valid() const { return high_ != 0 || low_ != 0; }

  uint64_t high() const { return high_; }
  uint64_t low() const { return low_; }

  bool operator==(const NodeName& rhs) const {
    return std::tie(high_, low_) == std::tie(rhs.high_, rhs.low_);
  }

  bool operator!=(const NodeName& rhs) const {
    return std::tie(high_, low_) != std::tie(rhs.high_, rhs.low_);
  }

  bool operator<(const NodeName& rhs) const {
    return std::tie(high_, low_) < std::tie(rhs.high_, rhs.low_);
  }

  // Convenient store-release and load-acquire operations for dealing with
  // NodeNames in shared memory.
  void StoreRelease(const NodeName& name);
  NodeName LoadAcquire();

  // Support for absl::Hash.
  template <typename H>
  friend H AbslHashValue(H h, const NodeName& name) {
    return H::combine(std::move(h), name.high_, name.low_);
  }

  std::string ToString() const;

 private:
  uint64_t high_ = 0;
  uint64_t low_ = 0;
};

}  // namespace ipcz

#endif  // IPCZ_SRC_IPCZ_NODE_NAME_H_
