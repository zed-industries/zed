/*
 *  Copyright 2011 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_IP_ADDRESS_H_
#define RTC_BASE_IP_ADDRESS_H_

#include <cstdint>
#if defined(WEBRTC_POSIX)
#include <arpa/inet.h>
#include <netdb.h>
#include <netinet/in.h>  // IWYU pragma: export

#include "absl/strings/string_view.h"
#endif
#if defined(WEBRTC_WIN)
#include <ws2tcpip.h>
#endif
#include <string.h>

#include <string>

#include "rtc_base/byte_order.h"
#if defined(WEBRTC_WIN)
#include "rtc_base/win32.h"
#endif
#include "absl/strings/string_view.h"
#include "rtc_base/net_helpers.h"
#include "rtc_base/system/rtc_export.h"
namespace webrtc {

enum IPv6AddressFlag {
  IPV6_ADDRESS_FLAG_NONE = 0x00,

  // Temporary address is dynamic by nature and will not carry MAC
  // address.
  IPV6_ADDRESS_FLAG_TEMPORARY = 1 << 0,

  // Temporary address could become deprecated once the preferred
  // lifetime is reached. It is still valid but just shouldn't be used
  // to create new connection.
  IPV6_ADDRESS_FLAG_DEPRECATED = 1 << 1,
};

// Version-agnostic IP address class, wraps a union of in_addr and in6_addr.
class RTC_EXPORT IPAddress {
 public:
  IPAddress() : family_(AF_UNSPEC) { ::memset(&u_, 0, sizeof(u_)); }

  explicit IPAddress(const in_addr& ip4) : family_(AF_INET) {
    memset(&u_, 0, sizeof(u_));
    u_.ip4 = ip4;
  }

  explicit IPAddress(const in6_addr& ip6) : family_(AF_INET6) { u_.ip6 = ip6; }

  explicit IPAddress(uint32_t ip_in_host_byte_order) : family_(AF_INET) {
    memset(&u_, 0, sizeof(u_));
    u_.ip4.s_addr = webrtc::HostToNetwork32(ip_in_host_byte_order);
  }

  IPAddress(const IPAddress& other) : family_(other.family_) {
    ::memcpy(&u_, &other.u_, sizeof(u_));
  }

  virtual ~IPAddress() {}

  const IPAddress& operator=(const IPAddress& other) {
    family_ = other.family_;
    ::memcpy(&u_, &other.u_, sizeof(u_));
    return *this;
  }

  bool operator==(const IPAddress& other) const;
  bool operator!=(const IPAddress& other) const;
  bool operator<(const IPAddress& other) const;
  bool operator>(const IPAddress& other) const;

  int family() const { return family_; }
  in_addr ipv4_address() const;
  in6_addr ipv6_address() const;

  // Returns the number of bytes needed to store the raw address.
  size_t Size() const;

  // Wraps inet_ntop.
  std::string ToString() const;

  // Same as ToString but anonymizes it by hiding the last part.
  std::string ToSensitiveString() const;

  // Returns an unmapped address from a possibly-mapped address.
  // Returns the same address if this isn't a mapped address.
  IPAddress Normalized() const;

  // Returns this address as an IPv6 address.
  // Maps v4 addresses (as ::ffff:a.b.c.d), returns v6 addresses unchanged.
  IPAddress AsIPv6Address() const;

  // For socketaddress' benefit. Returns the IP in host byte order.
  uint32_t v4AddressAsHostOrderInteger() const;

  // Get the network layer overhead per packet based on the IP address family.
  int overhead() const;

  // Whether this is an unspecified IP address.
  bool IsNil() const;

 private:
  int family_;
  union {
    in_addr ip4;
    in6_addr ip6;
  } u_;
};

// IP class which could represent IPv6 address flags which is only
// meaningful in IPv6 case.
class RTC_EXPORT InterfaceAddress : public IPAddress {
 public:
  InterfaceAddress() : ipv6_flags_(IPV6_ADDRESS_FLAG_NONE) {}

  explicit InterfaceAddress(IPAddress ip)
      : IPAddress(ip), ipv6_flags_(IPV6_ADDRESS_FLAG_NONE) {}

  InterfaceAddress(IPAddress addr, int ipv6_flags)
      : IPAddress(addr), ipv6_flags_(ipv6_flags) {}

  InterfaceAddress(const in6_addr& ip6, int ipv6_flags)
      : IPAddress(ip6), ipv6_flags_(ipv6_flags) {}

  InterfaceAddress(const InterfaceAddress& other) = default;
  const InterfaceAddress& operator=(const InterfaceAddress& other);

  bool operator==(const InterfaceAddress& other) const;
  bool operator!=(const InterfaceAddress& other) const;

  int ipv6_flags() const { return ipv6_flags_; }

  std::string ToString() const;

 private:
  int ipv6_flags_;
};

bool IPFromAddrInfo(struct addrinfo* info, IPAddress* out);
RTC_EXPORT bool IPFromString(absl::string_view str, IPAddress* out);
RTC_EXPORT bool IPFromString(absl::string_view str,
                             int flags,
                             InterfaceAddress* out);
bool IPIsAny(const IPAddress& ip);
RTC_EXPORT bool IPIsLoopback(const IPAddress& ip);
RTC_EXPORT bool IPIsLinkLocal(const IPAddress& ip);
// Identify a private network address like "192.168.111.222"
// (see https://en.wikipedia.org/wiki/Private_network )
bool IPIsPrivateNetwork(const IPAddress& ip);
// Identify a shared network address like "100.72.16.122"
// (see RFC6598)
bool IPIsSharedNetwork(const IPAddress& ip);
// Identify if an IP is "private", that is a loopback
// or an address belonging to a link-local, a private network or a shared
// network.
RTC_EXPORT bool IPIsPrivate(const IPAddress& ip);
RTC_EXPORT bool IPIsUnspec(const IPAddress& ip);
size_t HashIP(const IPAddress& ip);

// These are only really applicable for IPv6 addresses.
bool IPIs6Bone(const IPAddress& ip);
bool IPIs6To4(const IPAddress& ip);
RTC_EXPORT bool IPIsMacBased(const IPAddress& ip);
bool IPIsSiteLocal(const IPAddress& ip);
bool IPIsTeredo(const IPAddress& ip);
bool IPIsULA(const IPAddress& ip);
bool IPIsV4Compatibility(const IPAddress& ip);
bool IPIsV4Mapped(const IPAddress& ip);

// Returns the precedence value for this IP as given in RFC3484.
int IPAddressPrecedence(const IPAddress& ip);

// Returns 'ip' truncated to be 'length' bits long.
RTC_EXPORT IPAddress TruncateIP(const IPAddress& ip, int length);

IPAddress GetLoopbackIP(int family);
IPAddress GetAnyIP(int family);

// Returns the number of contiguously set bits, counting from the MSB in network
// byte order, in this IPAddress. Bits after the first 0 encountered are not
// counted.
int CountIPMaskBits(const IPAddress& mask);

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::CountIPMaskBits;
using ::webrtc::GetAnyIP;
using ::webrtc::GetLoopbackIP;
using ::webrtc::HashIP;
using ::webrtc::InterfaceAddress;
using ::webrtc::IPAddress;
using ::webrtc::IPAddressPrecedence;
using ::webrtc::IPFromAddrInfo;
using ::webrtc::IPFromString;
using ::webrtc::IPIs6Bone;
using ::webrtc::IPIs6To4;
using ::webrtc::IPIsAny;
using ::webrtc::IPIsLinkLocal;
using ::webrtc::IPIsLoopback;
using ::webrtc::IPIsMacBased;
using ::webrtc::IPIsPrivate;
using ::webrtc::IPIsPrivateNetwork;
using ::webrtc::IPIsSharedNetwork;
using ::webrtc::IPIsSiteLocal;
using ::webrtc::IPIsTeredo;
using ::webrtc::IPIsULA;
using ::webrtc::IPIsUnspec;
using ::webrtc::IPIsV4Compatibility;
using ::webrtc::IPIsV4Mapped;
using ::webrtc::IPV6_ADDRESS_FLAG_DEPRECATED;
using ::webrtc::IPV6_ADDRESS_FLAG_NONE;
using ::webrtc::IPV6_ADDRESS_FLAG_TEMPORARY;
using ::webrtc::IPv6AddressFlag;
using ::webrtc::TruncateIP;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // RTC_BASE_IP_ADDRESS_H_
