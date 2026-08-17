/*
 *  Copyright 2004 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_NETWORK_H_
#define RTC_BASE_NETWORK_H_

#include <stdint.h>

#include <map>
#include <memory>
#include <string>
#include <utility>
#include <vector>

#include "absl/base/nullability.h"
#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "api/environment/environment.h"
#include "api/field_trials_view.h"
#include "api/scoped_refptr.h"
#include "api/sequence_checker.h"
#include "api/task_queue/pending_task_safety_flag.h"
#include "rtc_base/checks.h"
#include "rtc_base/ip_address.h"
#include "rtc_base/mdns_responder_interface.h"
#include "rtc_base/network_constants.h"
#include "rtc_base/network_monitor.h"
#include "rtc_base/network_monitor_factory.h"
#include "rtc_base/socket_factory.h"
#include "rtc_base/system/rtc_export.h"
#include "rtc_base/third_party/sigslot/sigslot.h"
#include "rtc_base/thread.h"
#include "rtc_base/thread_annotations.h"

#if defined(WEBRTC_POSIX)
#include "rtc_base/ifaddrs_converter.h"
struct ifaddrs;
#endif  // defined(WEBRTC_POSIX)

namespace webrtc {

extern const char kPublicIPv4Host[];
extern const char kPublicIPv6Host[];

class Network;

// By default, ignore loopback interfaces on the host.
const int kDefaultNetworkIgnoreMask = webrtc::ADAPTER_TYPE_LOOPBACK;

namespace webrtc_network_internal {
bool CompareNetworks(const std::unique_ptr<Network>& a,
                     const std::unique_ptr<Network>& b);
}  // namespace webrtc_network_internal

// Makes a string key for this network. Used in the network manager's maps.
// Network objects are keyed on interface name, network prefix and the
// length of that prefix.
std::string MakeNetworkKey(absl::string_view name,
                           const IPAddress& prefix,
                           int prefix_length);

// Utility function that attempts to determine an adapter type by an interface
// name (e.g., "wlan0"). Can be used by NetworkManager subclasses when other
// mechanisms fail to determine the type.
RTC_EXPORT AdapterType GetAdapterTypeFromName(absl::string_view network_name);
RTC_EXPORT AdapterType GetAdapterTypeFromName(absl::string_view network_name);

class DefaultLocalAddressProvider {
 public:
  virtual ~DefaultLocalAddressProvider() = default;

  // The default local address is the local address used in multi-homed endpoint
  // when the any address (0.0.0.0 or ::) is used as the local address. It's
  // important to check the return value as a IP family may not be enabled.
  virtual bool GetDefaultLocalAddress(int family, IPAddress* ipaddr) const = 0;
};

class MdnsResponderProvider {
 public:
  virtual ~MdnsResponderProvider() = default;

  // Returns the mDNS responder that can be used to obfuscate the local IP
  // addresses of ICE host candidates by mDNS hostnames.
  //
  // The provider MUST outlive the mDNS responder.
  virtual MdnsResponderInterface* GetMdnsResponder() const = 0;
};

// Network/mask in CIDR representation.
class NetworkMask {
 public:
  NetworkMask(const IPAddress& addr, int prefix_length)
      : address_(addr), prefix_length_(prefix_length) {}

  const IPAddress& address() const { return address_; }
  int prefix_length() const { return prefix_length_; }

  bool operator==(const NetworkMask& o) const {
    return address_ == o.address_ && prefix_length_ == o.prefix_length_;
  }

 private:
  IPAddress address_;
  // Length of valid bits in address_ (for ipv4 valid range is 0-32)
  int prefix_length_;
};

// Generic network manager interface. It provides list of local
// networks.
//
// Every method of NetworkManager (including the destructor) must be called on
// the same thread, except for the constructor which may be called on any
// thread.
//
// This allows constructing a NetworkManager subclass on one thread and
// passing it into an object that uses it on a different thread.
class RTC_EXPORT NetworkManager : public DefaultLocalAddressProvider,
                                  public MdnsResponderProvider {
 public:
  // This enum indicates whether adapter enumeration is allowed.
  enum EnumerationPermission {
    ENUMERATION_ALLOWED,  // Adapter enumeration is allowed. Getting 0 network
                          // from GetNetworks means that there is no network
                          // available.
    ENUMERATION_BLOCKED,  // Adapter enumeration is disabled.
                          // GetAnyAddressNetworks() should be used instead.
  };

  // Called when network list is updated.
  sigslot::signal0<> SignalNetworksChanged;

  // Indicates a failure when getting list of network interfaces.
  sigslot::signal0<> SignalError;

  // This should be called on the NetworkManager's thread before the
  // NetworkManager is used. Subclasses may override this if necessary.
  virtual void Initialize() {}

  // Start/Stop monitoring of network interfaces
  // list. SignalNetworksChanged or SignalError is emitted immediately
  // after StartUpdating() is called. After that SignalNetworksChanged
  // is emitted whenever list of networks changes.
  virtual void StartUpdating() = 0;
  virtual void StopUpdating() = 0;

  // Returns the current list of networks available on this machine.
  // StartUpdating() must be called before this method is called.
  // It makes sure that repeated calls return the same object for a
  // given network, so that quality is tracked appropriately. Does not
  // include ignored networks.
  // The returned vector of Network* is valid as long as the NetworkManager is
  // alive.
  virtual std::vector<const Network*> GetNetworks() const = 0;

  // Returns the current permission state of GetNetworks().
  virtual EnumerationPermission enumeration_permission() const;

  // "AnyAddressNetwork" is a network which only contains single "any address"
  // IP address.  (i.e. INADDR_ANY for IPv4 or in6addr_any for IPv6). This is
  // useful as binding to such interfaces allow default routing behavior like
  // http traffic.
  //
  // This method appends the "any address" networks to the list, such that this
  // can optionally be called after GetNetworks.
  virtual std::vector<const Network*> GetAnyAddressNetworks() = 0;

  // Dumps the current list of networks in the network manager.
  virtual void DumpNetworks() {}
  bool GetDefaultLocalAddress(int family, IPAddress* ipaddr) const override;

  struct Stats {
    int ipv4_network_count;
    int ipv6_network_count;
    Stats() {
      ipv4_network_count = 0;
      ipv6_network_count = 0;
    }
  };

  // MdnsResponderProvider interface.
  MdnsResponderInterface* GetMdnsResponder() const override;

  virtual void set_vpn_list(const std::vector<NetworkMask>& /* vpn */) {}
};

// Represents a Unix-type network interface, with a name and single address.
class RTC_EXPORT Network {
 public:
  Network(absl::string_view name,
          absl::string_view description,
          const IPAddress& prefix,
          int prefix_length)
      : Network(name,
                description,
                prefix,
                prefix_length,
                webrtc::ADAPTER_TYPE_UNKNOWN) {}

  Network(absl::string_view name,
          absl::string_view description,
          const IPAddress& prefix,
          int prefix_length,
          AdapterType type);

  Network(const Network&);
  ~Network();

  // This signal is fired whenever type() or underlying_type_for_vpn() changes.
  // Mutable, to support connecting on the const Network passed to webrtc::Port
  // constructor.
  mutable sigslot::signal1<const Network*> SignalTypeChanged;

  // This signal is fired whenever network preference changes.
  sigslot::signal1<const Network*> SignalNetworkPreferenceChanged;

  const DefaultLocalAddressProvider* default_local_address_provider() const {
    return default_local_address_provider_;
  }
  void set_default_local_address_provider(
      const DefaultLocalAddressProvider* provider) {
    default_local_address_provider_ = provider;
  }

  void set_mdns_responder_provider(const MdnsResponderProvider* provider) {
    mdns_responder_provider_ = provider;
  }

  // Returns the name of the interface this network is associated with.
  const std::string& name() const { return name_; }

  // Returns the OS-assigned name for this network. This is useful for
  // debugging but should not be sent over the wire (for privacy reasons).
  const std::string& description() const { return description_; }

  // Returns the prefix for this network.
  const IPAddress& prefix() const { return prefix_; }
  // Returns the length, in bits, of this network's prefix.
  int prefix_length() const { return prefix_length_; }

  // Returns the family for the network prefix.
  int family() const { return prefix_.family(); }

  // `key_` has unique value per network interface. Used in sorting network
  // interfaces. Key is derived from interface name and it's prefix.
  std::string key() const { return key_; }

  // Returns the Network's current idea of the 'best' IP it has.
  // Or return an unset IP if this network has no active addresses.
  // Here is the rule on how we mark the IPv6 address as ignorable for WebRTC.
  // 1) return all global temporary dynamic and non-deprecated ones.
  // 2) if #1 not available, return global ones.
  // 3) if #2 not available, return local link ones.
  // 4) if #3 not available, use ULA ipv6 as last resort. (ULA stands for
  // unique local address, which is not route-able in open internet but might
  // be useful for a close WebRTC deployment.

  // TODO(guoweis): rule #3 actually won't happen at current
  // implementation. The reason being that ULA address starting with
  // 0xfc 0r 0xfd will be grouped into its own Network. The result of
  // that is WebRTC will have one extra Network to generate candidates
  // but the lack of rule #3 shouldn't prevent turning on IPv6 since
  // ULA should only be tried in a close deployment anyway.

  // Note that when not specifying any flag, it's treated as case global
  // IPv6 address
  IPAddress GetBestIP() const;

  // Adds an active IP address to this network. Does not check for duplicates.
  void AddIP(const InterfaceAddress& ip) { ips_.push_back(ip); }
  void AddIP(const IPAddress& ip) { ips_.push_back(InterfaceAddress(ip)); }

  // Sets the network's IP address list. Returns true if new IP addresses were
  // detected. Passing true to already_changed skips this check.
  bool SetIPs(const std::vector<InterfaceAddress>& ips, bool already_changed);
  // Get the list of IP Addresses associated with this network.
  const std::vector<InterfaceAddress>& GetIPs() const { return ips_; }
  // Clear the network's list of addresses.
  void ClearIPs() { ips_.clear(); }
  // Returns the mDNS responder that can be used to obfuscate the local IP
  // addresses of host candidates by mDNS names in ICE gathering. After a
  // name-address mapping is created by the mDNS responder, queries for the
  // created name will be resolved by the responder.
  MdnsResponderInterface* GetMdnsResponder() const;

  // Returns the scope-id of the network's address.
  // Should only be relevant for link-local IPv6 addresses.
  int scope_id() const { return scope_id_; }
  void set_scope_id(int id) { scope_id_ = id; }

  // Indicates whether this network should be ignored, perhaps because
  // the IP is 0, or the interface is one we know is invalid.
  bool ignored() const { return ignored_; }
  void set_ignored(bool ignored) { ignored_ = ignored; }

  AdapterType type() const { return type_; }
  // When type() is ADAPTER_TYPE_VPN, this returns the type of the underlying
  // network interface used by the VPN, typically the preferred network type
  // (see for example, the method setUnderlyingNetworks(android.net.Network[])
  // on https://developer.android.com/reference/android/net/VpnService.html).
  // When this information is unavailable from the OS, ADAPTER_TYPE_UNKNOWN is
  // returned.
  AdapterType underlying_type_for_vpn() const {
    return underlying_type_for_vpn_;
  }
  void set_type(AdapterType type) {
    if (type_ == type) {
      return;
    }
    type_ = type;
    if (type != webrtc::ADAPTER_TYPE_VPN) {
      underlying_type_for_vpn_ = webrtc::ADAPTER_TYPE_UNKNOWN;
    }
    SignalTypeChanged(this);
  }

  void set_underlying_type_for_vpn(AdapterType type) {
    if (underlying_type_for_vpn_ == type) {
      return;
    }
    underlying_type_for_vpn_ = type;
    SignalTypeChanged(this);
  }

  bool IsVpn() const { return type_ == webrtc::ADAPTER_TYPE_VPN; }

  bool IsCellular() const { return IsCellular(type_); }

  static bool IsCellular(AdapterType type) {
    switch (type) {
      case webrtc::ADAPTER_TYPE_CELLULAR:
      case webrtc::ADAPTER_TYPE_CELLULAR_2G:
      case webrtc::ADAPTER_TYPE_CELLULAR_3G:
      case webrtc::ADAPTER_TYPE_CELLULAR_4G:
      case webrtc::ADAPTER_TYPE_CELLULAR_5G:
        return true;
      default:
        return false;
    }
  }

  // Note: This function is called "rarely".
  // Twice per Network in BasicPortAllocator if
  // PORTALLOCATOR_DISABLE_COSTLY_NETWORKS. Once in Port::Construct() (and when
  // Port::OnNetworkTypeChanged is called).
  uint16_t GetCost(const FieldTrialsView& field_trials) const;

  // A unique id assigned by the network manager, which may be signaled
  // to the remote side in the candidate.
  uint16_t id() const { return id_; }
  void set_id(uint16_t id) { id_ = id; }

  int preference() const { return preference_; }
  void set_preference(int preference) { preference_ = preference; }

  // When we enumerate networks and find a previously-seen network is missing,
  // we do not remove it (because it may be used elsewhere). Instead, we mark
  // it inactive, so that we can detect network changes properly.
  bool active() const { return active_; }
  void set_active(bool active) {
    if (active_ != active) {
      active_ = active;
    }
  }

  // Property set by operating system/firmware that has information
  // about connection strength to e.g WIFI router or CELL base towers.
  NetworkPreference network_preference() const { return network_preference_; }
  void set_network_preference(NetworkPreference val) {
    if (network_preference_ == val) {
      return;
    }
    network_preference_ = val;
    SignalNetworkPreferenceChanged(this);
  }

  static std::pair<AdapterType, bool /* vpn */> GuessAdapterFromNetworkCost(
      int network_cost);

  // Debugging description of this network
  std::string ToString() const;

 private:
  const DefaultLocalAddressProvider* default_local_address_provider_ = nullptr;
  const MdnsResponderProvider* mdns_responder_provider_ = nullptr;
  std::string name_;
  std::string description_;
  IPAddress prefix_;
  int prefix_length_;
  std::string key_;
  std::vector<InterfaceAddress> ips_;
  int scope_id_;
  bool ignored_;
  AdapterType type_;
  AdapterType underlying_type_for_vpn_ = webrtc::ADAPTER_TYPE_UNKNOWN;
  int preference_;
  bool active_ = true;
  uint16_t id_ = 0;
  NetworkPreference network_preference_ = NetworkPreference::NEUTRAL;

  friend class NetworkManager;
};

// Base class for NetworkManager implementations.
class RTC_EXPORT NetworkManagerBase : public NetworkManager {
 public:
  NetworkManagerBase();

  std::vector<const Network*> GetNetworks() const override;
  std::vector<const Network*> GetAnyAddressNetworks() override;

  EnumerationPermission enumeration_permission() const override;

  bool GetDefaultLocalAddress(int family, IPAddress* ipaddr) const override;

  // Check if MAC address in |bytes| is one of the pre-defined
  // MAC addresses for know VPNs.
  static bool IsVpnMacAddress(ArrayView<const uint8_t> address);

 protected:
  // Updates `networks_` with the networks listed in `list`. If
  // `networks_map_` already has a Network object for a network listed
  // in the `list` then it is reused. Accept ownership of the Network
  // objects in the `list`. `changed` will be set to true if there is
  // any change in the network list.
  void MergeNetworkList(std::vector<std::unique_ptr<Network>> list,
                        bool* changed);

  // `stats` will be populated even if |*changed| is false.
  void MergeNetworkList(std::vector<std::unique_ptr<Network>> list,
                        bool* changed,
                        NetworkManager::Stats* stats);

  void set_enumeration_permission(EnumerationPermission state) {
    enumeration_permission_ = state;
  }

  void set_default_local_addresses(const IPAddress& ipv4,
                                   const IPAddress& ipv6);

  Network* GetNetworkFromAddress(const IPAddress& ip) const;

  // To enable subclasses to get the networks list, without interfering with
  // refactoring of the interface GetNetworks method.
  const std::vector<Network*>& GetNetworksInternal() const { return networks_; }

  std::unique_ptr<Network> CreateNetwork(absl::string_view name,
                                         absl::string_view description,
                                         const IPAddress& prefix,
                                         int prefix_length,
                                         AdapterType type) const;

 private:
  friend class NetworkTest;
  EnumerationPermission enumeration_permission_;

  std::vector<Network*> networks_;

  std::map<std::string, std::unique_ptr<Network>> networks_map_;

  std::unique_ptr<Network> ipv4_any_address_network_;
  std::unique_ptr<Network> ipv6_any_address_network_;

  IPAddress default_local_ipv4_address_;
  IPAddress default_local_ipv6_address_;
  // We use 16 bits to save the bandwidth consumption when sending the network
  // id over the Internet. It is OK that the 16-bit integer overflows to get a
  // network id 0 because we only compare the network ids in the old and the new
  // best connections in the transport channel.
  uint16_t next_available_network_id_ = 1;
};

// Basic implementation of the NetworkManager interface that gets list
// of networks using OS APIs.
class RTC_EXPORT BasicNetworkManager : public NetworkManagerBase,
                                       public NetworkBinderInterface,
                                       public sigslot::has_slots<> {
 public:
  BasicNetworkManager(
      const Environment& env,
      SocketFactory* absl_nonnull socket_factory,
      NetworkMonitorFactory* absl_nullable network_monitor_factory = nullptr);

  ~BasicNetworkManager() override;

  void StartUpdating() override;
  void StopUpdating() override;

  void DumpNetworks() override;

  bool started() { return start_count_ > 0; }

  // Sets the network ignore list, which is empty by default. Any network on the
  // ignore list will be filtered from network enumeration results.
  // Should be called only before initialization.
  void set_network_ignore_list(const std::vector<std::string>& list) {
    RTC_DCHECK(thread_ == nullptr);
    network_ignore_list_ = list;
  }

  // Set a list of manually configured VPN's.
  void set_vpn_list(const std::vector<NetworkMask>& vpn) override;

  // Check if |prefix| is configured as VPN.
  bool IsConfiguredVpn(IPAddress prefix, int prefix_length) const;

  // Bind a socket to interface that ip address belong to.
  // Implementation look up interface name and calls
  // BindSocketToNetwork on NetworkMonitor.
  // The interface name is needed as e.g ipv4 over ipv6 addresses
  // are not exposed using Android functions, but it is possible
  // bind an ipv4 address to the interface.
  NetworkBindingResult BindSocketToNetwork(int socket_fd,
                                           const IPAddress& address) override;

 protected:
#if defined(WEBRTC_POSIX)
  // Separated from CreateNetworks for tests.
  void ConvertIfAddrs(ifaddrs* interfaces,
                      IfAddrsConverter* converter,
                      bool include_ignored,
                      std::vector<std::unique_ptr<Network>>* networks) const
      RTC_RUN_ON(thread_);
  NetworkMonitorInterface::InterfaceInfo GetInterfaceInfo(
      struct ifaddrs* cursor) const RTC_RUN_ON(thread_);
#endif  // defined(WEBRTC_POSIX)

  // Creates a network object for each network available on the machine.
  bool CreateNetworks(bool include_ignored,
                      std::vector<std::unique_ptr<Network>>* networks) const
      RTC_RUN_ON(thread_);

  // Determines if a network should be ignored. This should only be determined
  // based on the network's property instead of any individual IP.
  bool IsIgnoredNetwork(const Network& network) const RTC_RUN_ON(thread_);

  // This function connects a UDP socket to a public address and returns the
  // local address associated it. Since it binds to the "any" address
  // internally, it returns the default local address on a multi-homed endpoint.
  IPAddress QueryDefaultLocalAddress(int family) const RTC_RUN_ON(thread_);

 private:
  friend class NetworkTest;

  // Creates a network monitor and listens for network updates.
  void StartNetworkMonitor() RTC_RUN_ON(thread_);
  // Stops and removes the network monitor.
  void StopNetworkMonitor() RTC_RUN_ON(thread_);
  // Called when it receives updates from the network monitor.
  void OnNetworksChanged();

  // Updates the networks and reschedules the next update.
  void UpdateNetworksContinually() RTC_RUN_ON(thread_);
  // Only updates the networks; does not reschedule the next update.
  void UpdateNetworksOnce() RTC_RUN_ON(thread_);

  const Environment env_;
  Thread* thread_ = nullptr;
  bool sent_first_update_ = true;
  int start_count_ = 0;
  std::vector<std::string> network_ignore_list_;
  NetworkMonitorFactory* absl_nullable const network_monitor_factory_;
  SocketFactory* absl_nonnull const socket_factory_;
  std::unique_ptr<NetworkMonitorInterface> network_monitor_
      RTC_GUARDED_BY(thread_);
  bool allow_mac_based_ipv6_ RTC_GUARDED_BY(thread_) = false;
  bool bind_using_ifname_ RTC_GUARDED_BY(thread_) = false;

  std::vector<NetworkMask> vpn_;
  scoped_refptr<PendingTaskSafetyFlag> task_safety_flag_;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::BasicNetworkManager;
using ::webrtc::DefaultLocalAddressProvider;
using ::webrtc::GetAdapterTypeFromName;
using ::webrtc::kDefaultNetworkIgnoreMask;
using ::webrtc::kPublicIPv4Host;
using ::webrtc::kPublicIPv6Host;
using ::webrtc::MakeNetworkKey;
using ::webrtc::MdnsResponderProvider;
using ::webrtc::Network;
using ::webrtc::NetworkManager;
using ::webrtc::NetworkManagerBase;
using ::webrtc::NetworkMask;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // RTC_BASE_NETWORK_H_
