/*
 *  Copyright 2015 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_NETWORK_MONITOR_H_
#define RTC_BASE_NETWORK_MONITOR_H_

#include <functional>
#include <utility>

#include "absl/strings/string_view.h"
#include "rtc_base/ip_address.h"
#include "rtc_base/network_constants.h"

namespace webrtc {

enum class NetworkBindingResult {
  SUCCESS = 0,   // No error
  FAILURE = -1,  // Generic error
  NOT_IMPLEMENTED = -2,
  ADDRESS_NOT_FOUND = -3,
  NETWORK_CHANGED = -4
};

// NetworkPreference property set by operating system/firmware that has
// information about connection strength to e.g WIFI router or CELL base towers.
// GENERATED_JAVA_ENUM_PACKAGE: org.webrtc
enum class NetworkPreference {
  NEUTRAL = 0,
  NOT_PREFERRED = -1,
};

const char* NetworkPreferenceToString(NetworkPreference preference);

// This interface is set onto a socket server,
// where only the ip address is known at the time of binding.
class NetworkBinderInterface {
 public:
  // Binds a socket to the network that is attached to `address` so that all
  // packets on the socket `socket_fd` will be sent via that network.
  // This is needed because some operating systems (like Android) require a
  // special bind call to put packets on a non-default network interface.
  virtual NetworkBindingResult BindSocketToNetwork(
      int socket_fd,
      const IPAddress& address) = 0;
  virtual ~NetworkBinderInterface() {}
};

/*
 * Receives network-change events via `OnNetworksChanged` and signals the
 * networks changed event.
 *
 * Threading consideration:
 * It is expected that all upstream operations (from native to Java) are
 * performed from the worker thread. This includes creating, starting and
 * stopping the monitor. This avoids the potential race condition when creating
 * the singleton Java NetworkMonitor class. Downstream operations can be from
 * any thread, but this class will forward all the downstream operations onto
 * the worker thread.
 *
 * Memory consideration:
 * NetworkMonitor is owned by the caller (NetworkManager). The global network
 * monitor factory is owned by the PeerConnectionFactory.
 */
// Generic network monitor interface. It starts and stops monitoring network
// changes, and fires the SignalNetworksChanged event when networks change.
class NetworkMonitorInterface {
 public:
  struct InterfaceInfo {
    // The type of adapter if known.
    AdapterType adapter_type;

    // Is ADAPTER_TYPE_UNKNOWN unless adapter_type == ADAPTER_TYPE_VPN.
    AdapterType underlying_type_for_vpn = webrtc::ADAPTER_TYPE_UNKNOWN;

    // The OS/firmware specific preference of this interface.
    NetworkPreference network_preference = NetworkPreference::NEUTRAL;

    // Is this interface available to use? WebRTC shouldn't attempt to use it if
    // this returns false.
    //
    // It's possible for this status to change, in which case
    // SignalNetworksChanged will be fired.
    //
    // The specific use case this was added for was a phone with two SIM
    // cards, where attempting to use all interfaces returned from getifaddrs
    // caused the connection to be dropped.
    bool available = true;
  };

  NetworkMonitorInterface();
  virtual ~NetworkMonitorInterface();

  virtual void Start() = 0;
  virtual void Stop() = 0;

  // Get information about an interface.
  // If the interface is not known, the return struct will have set
  // `adapter_type` to ADAPTER_TYPE_UNKNOWN and `available` to false.
  virtual InterfaceInfo GetInterfaceInfo(absl::string_view interface_name) = 0;

  // Does `this` NetworkMonitorInterface implement BindSocketToNetwork?
  // Only Android returns true.
  virtual bool SupportsBindSocketToNetwork() const { return false; }

  // Bind a socket to an interface specified by ip address and/or interface
  // name. Only implemented on Android.
  virtual NetworkBindingResult BindSocketToNetwork(
      int /* socket_fd */,
      const IPAddress& /* address */,
      absl::string_view /* interface_name */) {
    return NetworkBindingResult::NOT_IMPLEMENTED;
  }

  void SetNetworksChangedCallback(std::function<void()> callback) {
    networks_changed_callback_ = std::move(callback);
  }

 protected:
  void InvokeNetworksChangedCallback() {
    if (networks_changed_callback_) {
      networks_changed_callback_();
    }
  }

 private:
  std::function<void()> networks_changed_callback_;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::NetworkBinderInterface;
using ::webrtc::NetworkBindingResult;
using ::webrtc::NetworkMonitorInterface;
using ::webrtc::NetworkPreference;
using ::webrtc::NetworkPreferenceToString;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // RTC_BASE_NETWORK_MONITOR_H_
