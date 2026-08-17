// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_P2P_EMPTY_NETWORK_MANAGER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_P2P_EMPTY_NETWORK_MANAGER_H_

#include "base/memory/weak_ptr.h"
#include "base/threading/thread_checker.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/webrtc/rtc_base/network.h"
#include "third_party/webrtc/rtc_base/ip_address.h"
#include "third_party/webrtc/rtc_base/third_party/sigslot/sigslot.h"

namespace blink {

class FilteringNetworkManagerTest;
class IpcNetworkManager;

// A NetworkManager implementation which handles the case where local address
// enumeration is not requested and just returns empty network lists. This class
// is not thread safe and should only be used by WebRTC's network thread.
class EmptyNetworkManager : public webrtc::NetworkManagerBase,
                            public sigslot::has_slots<> {
 public:
  // This class is created on the main thread but used by WebRTC's worker thread
  // |task_runner|.
  PLATFORM_EXPORT explicit EmptyNetworkManager(
      IpcNetworkManager* network_manager);
  EmptyNetworkManager(const EmptyNetworkManager&) = delete;
  EmptyNetworkManager& operator=(const EmptyNetworkManager&) = delete;
  PLATFORM_EXPORT ~EmptyNetworkManager() override;

  // webrtc::NetworkManager:
  void StartUpdating() override;
  void StopUpdating() override;
  std::vector<const webrtc::Network*> GetNetworks() const override;
  bool GetDefaultLocalAddress(int family,
                              webrtc::IPAddress* ipaddress) const override;

 private:
  friend class FilteringNetworkManagerTest;
  // We can't dereference the wrapped network manager from the construction
  // thread, as that would cause it to bind to the wrong sequence. We also can't
  // obtain a `WeakPtr` from an arbitrary `webrtc::NetworkManager`, so we take 2
  // pointers pointing to the same instance, one is a raw pointer for use on the
  // constructing thread and the other is a weak pointer for use on the worker
  // thread.
  // TODO(crbug.com/1191914): Simplify this, to avoid the subtleties of having
  // to pass two pointers to the same object.
  PLATFORM_EXPORT EmptyNetworkManager(webrtc::NetworkManager* network_manager,
                                      base::WeakPtr<webrtc::NetworkManager>
                                          network_manager_for_signaling_thread);

  // Receive callback from the wrapped NetworkManager when the underneath
  // network list is changed.
  //
  // We wait for this so that we don't signal networks changed before we have
  // default IP addresses.
  void OnNetworksChanged();

  THREAD_CHECKER(thread_checker_);

  // SignalNetworksChanged will only be fired if there is any outstanding
  // StartUpdating.
  int start_count_ = 0;

  // `network_manager_for_signaling_thread_` is owned by the
  // PeerConnectionDependencyFactory, that may be destroyed when the frame is
  // detached.
  base::WeakPtr<webrtc::NetworkManager> network_manager_for_signaling_thread_;

  base::WeakPtrFactory<EmptyNetworkManager> weak_ptr_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_P2P_EMPTY_NETWORK_MANAGER_H_
