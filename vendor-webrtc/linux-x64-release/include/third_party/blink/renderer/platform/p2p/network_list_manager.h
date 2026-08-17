// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// NetworkListManager interface is introduced to enable unit test on
// IpcNetworkManager such that it doesn't depend on implementation of
// P2PSocketDispatcher.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_P2P_NETWORK_LIST_MANAGER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_P2P_NETWORK_LIST_MANAGER_H_

#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

class NetworkListObserver;

// TODO(crbug.com/787254): Verify whether this abstract class is still
// needed now that its Clients have all switched to Blink.
class PLATFORM_EXPORT NetworkListManager : public GarbageCollectedMixin {
 public:
  // Add a new network list observer. Each observer is called
  // immidiately after it is registered and then later whenever
  // network configuration changes. Can be called on any thread. The
  // observer is always called on the thread it was added.
  virtual void AddNetworkListObserver(
      NetworkListObserver* network_list_observer) = 0;

  // Removes network list observer. Must be called on the thread on
  // which the observer was added.
  virtual void RemoveNetworkListObserver(
      NetworkListObserver* network_list_observer) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_P2P_NETWORK_LIST_MANAGER_H_
