/*
 *  Copyright 2009 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_FAKE_NETWORK_H_
#define RTC_BASE_FAKE_NETWORK_H_

#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "absl/strings/string_view.h"
#include "rtc_base/ip_address.h"
#include "rtc_base/mdns_responder_interface.h"
#include "rtc_base/net_helpers.h"
#include "rtc_base/network.h"
#include "rtc_base/network_constants.h"
#include "rtc_base/socket_address.h"
#include "rtc_base/string_encode.h"
#include "rtc_base/thread.h"

namespace webrtc {

const int kFakeIPv4NetworkPrefixLength = 24;
const int kFakeIPv6NetworkPrefixLength = 64;

// Fake network manager that allows us to manually specify the IPs to use.
class FakeNetworkManager : public NetworkManagerBase {
 public:
  FakeNetworkManager() {}

  struct Iface {
    SocketAddress socket_address;
    AdapterType adapter_type;
    std::optional<AdapterType> underlying_vpn_adapter_type;
  };
  typedef std::vector<Iface> IfaceList;

  void AddInterface(const SocketAddress& iface) {
    // Ensure a unique name for the interface if its name is not given.
    AddInterface(iface, "test" + absl::StrCat(next_index_++));
  }

  void AddInterface(const SocketAddress& iface, absl::string_view if_name) {
    AddInterface(iface, if_name, ADAPTER_TYPE_UNKNOWN);
  }

  void AddInterface(
      const SocketAddress& iface,
      absl::string_view if_name,
      AdapterType type,
      std::optional<AdapterType> underlying_vpn_adapter_type = std::nullopt) {
    SocketAddress address(if_name, 0);
    address.SetResolvedIP(iface.ipaddr());
    ifaces_.push_back({address, type, underlying_vpn_adapter_type});
    DoUpdateNetworks();
  }

  void RemoveInterface(const SocketAddress& iface) {
    for (IfaceList::iterator it = ifaces_.begin(); it != ifaces_.end(); ++it) {
      if (it->socket_address.EqualIPs(iface)) {
        ifaces_.erase(it);
        break;
      }
    }
    DoUpdateNetworks();
  }

  void StartUpdating() override {
    ++start_count_;
    if (start_count_ == 1) {
      sent_first_update_ = false;
      Thread::Current()->PostTask([this] { DoUpdateNetworks(); });
    } else if (sent_first_update_) {
      Thread::Current()->PostTask([this] { SignalNetworksChanged(); });
    }
  }

  void StopUpdating() override { --start_count_; }

  using NetworkManagerBase::set_default_local_addresses;
  using NetworkManagerBase::set_enumeration_permission;

  // webrtc::NetworkManager override.
  MdnsResponderInterface* GetMdnsResponder() const override {
    return mdns_responder_.get();
  }

  void set_mdns_responder(
      std::unique_ptr<MdnsResponderInterface> mdns_responder) {
    mdns_responder_ = std::move(mdns_responder);
  }

 private:
  void DoUpdateNetworks() {
    if (start_count_ == 0)
      return;
    std::vector<std::unique_ptr<Network>> networks;
    for (IfaceList::iterator it = ifaces_.begin(); it != ifaces_.end(); ++it) {
      int prefix_length = 0;
      if (it->socket_address.ipaddr().family() == AF_INET) {
        prefix_length = kFakeIPv4NetworkPrefixLength;
      } else if (it->socket_address.ipaddr().family() == AF_INET6) {
        prefix_length = kFakeIPv6NetworkPrefixLength;
      }
      IPAddress prefix =
          webrtc::TruncateIP(it->socket_address.ipaddr(), prefix_length);
      auto net = std::make_unique<Network>(
          it->socket_address.hostname(), it->socket_address.hostname(), prefix,
          prefix_length, it->adapter_type);
      if (it->underlying_vpn_adapter_type.has_value()) {
        net->set_underlying_type_for_vpn(*it->underlying_vpn_adapter_type);
      }
      net->set_default_local_address_provider(this);
      net->AddIP(it->socket_address.ipaddr());
      networks.push_back(std::move(net));
    }
    bool changed;
    MergeNetworkList(std::move(networks), &changed);
    if (changed || !sent_first_update_) {
      SignalNetworksChanged();
      sent_first_update_ = true;
    }
  }

  IfaceList ifaces_;
  int next_index_ = 0;
  int start_count_ = 0;
  bool sent_first_update_ = false;

  std::unique_ptr<MdnsResponderInterface> mdns_responder_;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::FakeNetworkManager;
using ::webrtc::kFakeIPv4NetworkPrefixLength;
using ::webrtc::kFakeIPv6NetworkPrefixLength;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // RTC_BASE_FAKE_NETWORK_H_
