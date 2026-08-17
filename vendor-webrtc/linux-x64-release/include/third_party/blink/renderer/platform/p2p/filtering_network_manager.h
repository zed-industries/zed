// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_P2P_FILTERING_NETWORK_MANAGER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_P2P_FILTERING_NETWORK_MANAGER_H_

#include "base/memory/raw_ptr.h"
#include "base/memory/weak_ptr.h"
#include "base/threading/thread_checker.h"
#include "base/time/time.h"
#include "third_party/blink/renderer/platform/p2p/network_manager_uma.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/webrtc/rtc_base/network.h"
#include "third_party/webrtc/rtc_base/third_party/sigslot/sigslot.h"

namespace media {
class MediaPermission;
}  // namespace media

namespace blink {

class FilteringNetworkManagerTest;
class IpcNetworkManager;

// FilteringNetworkManager exposes webrtc::NetworkManager to
// PeerConnectionDependencyFactory and wraps the IpcNetworkManager. It only
// handles the case where multiple_routes is requested. It checks at least one
// of mic/camera permissions is granted before allowing WebRTC to use the local
// IP addresses as ICE candidates. The class handles asynchronous signals like
// SignalNetworksChanged from IpcNetworkManager and permission status from
// MediaPermission before it signals WebRTC that the network information is
// ready. It is designed to fire the network change event at the earliest time
// to reduce any extra call setup delay. This class is not thread safe and
// should only be used by WebRTC's network thread. It inherits from
// webrtc::NetworkManagerBase to have the same implementation of
// GetAnyAddressNetworks(). We can't mark the whole class PLATFORM_EXPORT
// as it requires all super classes to be PLATFORM_EXPORT as well.
class FilteringNetworkManager : public webrtc::NetworkManagerBase,
                                public sigslot::has_slots<> {
 public:
  // This class is created by WebRTC's main thread but used by WebRTC's
  // worker thread |task_runner|.
  PLATFORM_EXPORT FilteringNetworkManager(
      IpcNetworkManager* network_manager,
      media::MediaPermission* media_permission,
      bool allow_mdns_obfuscation);
  FilteringNetworkManager(const FilteringNetworkManager&) = delete;
  FilteringNetworkManager& operator=(const FilteringNetworkManager&) = delete;

  PLATFORM_EXPORT ~FilteringNetworkManager() override;

  // webrtc::NetworkManager:
  void Initialize() override;
  void StartUpdating() override;
  void StopUpdating() override;
  std::vector<const webrtc::Network*> GetNetworks() const override;

  webrtc::MdnsResponderInterface* GetMdnsResponder() const override;

 private:
  friend class FilteringNetworkManagerTest;

  PLATFORM_EXPORT FilteringNetworkManager(
      base::WeakPtr<webrtc::NetworkManager>
          network_manager_for_signaling_thread,
      media::MediaPermission* media_permission,
      bool allow_mdns_obfuscation);

  // Check mic/camera permission.
  void CheckPermission();

  // Receive callback from MediaPermission when the permission status is
  // available.
  void OnPermissionStatus(bool granted);

  base::WeakPtr<FilteringNetworkManager> GetWeakPtr();

  // Receive callback from the wrapped NetworkManager when the underneath
  // network list is changed.
  void OnNetworksChanged();

  // A tri-state permission checking status. It starts with UNKNOWN and will
  // change to GRANTED if one of permissions is granted. Otherwise, DENIED will
  // be returned.
  blink::IPPermissionStatus GetIPPermissionStatus() const;

  void FireEventIfStarted();

  void SendNetworksChangedSignal();

  // `network_manager_for_signaling_thread_` is owned by the
  // `PeerConnectionDependencyFactory`, that may be destroyed when the frame is
  // detached.
  // TODO(crbug.com/1191914): Clarify the lifetime of
  // `network_manager_for_signaling_thread_` and `this`.
  base::WeakPtr<webrtc::NetworkManager> network_manager_for_signaling_thread_;

  // The class is created by the main thread but used by the worker thread.
  THREAD_CHECKER(thread_checker_);

  raw_ptr<media::MediaPermission, DanglingUntriaged> media_permission_;

  int pending_permission_checks_ = 0;

  // Whether we're waiting for a network change signal from |network_manager_|.
  bool pending_network_update_ = false;

  // Whether we have fired the first SignalNetworksChanged.
  // Used to ensure we only report metrics once.
  bool sent_first_update_ = false;

  // SignalNetworksChanged will only be fired if there is any outstanding
  // StartUpdating.
  int start_count_ = 0;

  // Track whether CheckPermission has been called before StartUpdating.
  bool started_permission_check_ = false;

  // Track whether StartUpdating has been called.
  bool start_updating_called_ = false;

  // When the mDNS obfuscation is allowed, access to the mDNS responder provided
  // by the base network manager is provided to conceal IPs with mDNS hostnames.
  bool allow_mdns_obfuscation_ = true;

  base::WeakPtrFactory<FilteringNetworkManager> weak_ptr_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_P2P_FILTERING_NETWORK_MANAGER_H_
