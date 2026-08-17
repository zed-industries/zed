// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_P2P_IPC_NETWORK_MANAGER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_P2P_IPC_NETWORK_MANAGER_H_

#include <memory>

#include "base/memory/weak_ptr.h"
#include "base/threading/thread_checker.h"
#include "third_party/blink/renderer/platform/heap/cross_thread_persistent.h"
#include "third_party/blink/renderer/platform/p2p/network_list_manager.h"
#include "third_party/blink/renderer/platform/p2p/network_list_observer.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/webrtc/rtc_base/mdns_responder_interface.h"
#include "third_party/webrtc/rtc_base/network.h"

namespace net {
class IPAddress;
}  // namespace net

namespace blink {

// IpcNetworkManager is a NetworkManager for libjingle that gets a
// list of network interfaces from the browser.
//
// Threading note:
// The IpcNetworkManager is constructed on the network thread, and after that
// may only be accessed from the signaling thread. The one exception to that is
// access to slots (e.g., `NetworkManager::SignalNetworksChanged`) that are safe
// to access from any thread.
class IpcNetworkManager : public webrtc::NetworkManagerBase,
                          public blink::NetworkListObserver {
 public:
  // Constructor doesn't take ownership of the |network_list_manager|.
  PLATFORM_EXPORT IpcNetworkManager(
      blink::NetworkListManager* network_list_manager,
      std::unique_ptr<webrtc::MdnsResponderInterface> mdns_responder);
  ~IpcNetworkManager() override;

  void PLATFORM_EXPORT ContextDestroyed();

  // Weak pointers may only be dereferenced on the signaling thread.
  base::WeakPtr<IpcNetworkManager> PLATFORM_EXPORT
  AsWeakPtrForSignalingThread();

  // webrtc:::NetworkManager:
  void StartUpdating() override;
  void StopUpdating() override;
  webrtc::MdnsResponderInterface* GetMdnsResponder() const override;

  // blink::NetworkListObserver interface.
  void OnNetworkListChanged(
      const net::NetworkInterfaceList& list,
      const net::IPAddress& default_ipv4_local_address,
      const net::IPAddress& default_ipv6_local_address) override;

 private:
  void SendNetworksChangedSignal();

  // 'this' is created on the network thread, whereas the `NetworkListManager`
  // is owned by the main thread, so it needs to be accessed in a thread-safe
  // manner. The `NetworkListManager` will be reset once the context is
  // destroyed, so the strong reference will not cause a leak.
  CrossThreadPersistent<NetworkListManager> network_list_manager_;
  std::unique_ptr<webrtc::MdnsResponderInterface> mdns_responder_;
  int start_count_ = 0;
  bool network_list_received_ = false;

  THREAD_CHECKER(thread_checker_);

  base::WeakPtrFactory<IpcNetworkManager> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_P2P_IPC_NETWORK_MANAGER_H_
