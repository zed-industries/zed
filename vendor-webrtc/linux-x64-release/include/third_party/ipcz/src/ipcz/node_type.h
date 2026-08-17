// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_IPCZ_NODE_TYPE_H_
#define IPCZ_SRC_IPCZ_NODE_TYPE_H_

#include <cstdint>

namespace ipcz {

// Enumeration indicating the role a Node plays in its network of nodes. Note
// that this is used by internal wire messages, so values must not be changed or
// removed.
enum class NodeType : uint8_t {
  // A broker node assigns its own name and is able to assign names to other
  // nodes upon connection. Brokers are trusted to introduce nodes to each
  // other upon request, and brokers may connect to other brokers in order to
  // share information and effectively bridge two node networks together.
  kBroker,

  // A "normal" (i.e. non-broker) node is assigned a permanent name by the
  // first broker node it connects to, and it can only make contact with other
  // nodes by requesting an introduction from that broker.
  kNormal,
};

}  // namespace ipcz

#endif  // IPCZ_SRC_IPCZ_NODE_TYPE_H_
