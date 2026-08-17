// Copyright 2022 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
#ifndef GRPC_SRC_CORE_LIB_EVENT_ENGINE_TCP_SOCKET_UTILS_H
#define GRPC_SRC_CORE_LIB_EVENT_ENGINE_TCP_SOCKET_UTILS_H

#include <grpc/event_engine/event_engine.h>
#include <grpc/support/port_platform.h>

#include <optional>
#include <string>

#include "absl/status/statusor.h"

namespace grpc_event_engine::experimental {

// Returns true if resolved_addr is an IPv4-mapped IPv6 address within the
//  ::ffff:0.0.0.0/96 range, or false otherwise.
//  If resolved_addr4_out is non-NULL, the inner IPv4 address will be copied
//  here when returning true.
bool ResolvedAddressIsV4Mapped(
    const EventEngine::ResolvedAddress& resolved_addr,
    EventEngine::ResolvedAddress* resolved_addr4_out);

// If resolved_addr is an AF_INET address, writes the corresponding
// ::ffff:0.0.0.0/96 address to resolved_addr6_out and returns true.  Otherwise
// returns false.
bool ResolvedAddressToV4Mapped(
    const EventEngine::ResolvedAddress& resolved_addr,
    EventEngine::ResolvedAddress* resolved_addr6_out);

// Make wild card IPv6 address with specified port.
EventEngine::ResolvedAddress ResolvedAddressMakeWild6(int port);

// Make wild card IPv4 address with specified port.
EventEngine::ResolvedAddress ResolvedAddressMakeWild4(int port);

// Given a resolved address, return the port number in the address.
int ResolvedAddressGetPort(const EventEngine::ResolvedAddress& resolved_addr);

// Modifies the address, setting the specified port number.
// The operation would only succeed if the passed address is an IPv4 or Ipv6
// address. Otherwise the function call would abort fail.
void ResolvedAddressSetPort(EventEngine::ResolvedAddress& resolved_addr,
                            int port);

// Returns the port number associated with the address if the given address is
// a wildcard ipv4 or ipv6 address. Otherwise returns std::nullopt
std::optional<int> MaybeGetWildcardPortFromAddress(
    const EventEngine::ResolvedAddress& addr);

// Returns true if resolved_addr is an VSOCK address. Otherwise returns false.
bool ResolvedAddressIsVSock(const EventEngine::ResolvedAddress& resolved_addr);

// Converts a EventEngine::ResolvedAddress into a newly-allocated
// human-readable string.
// Currently, only the AF_INET, AF_INET6, and AF_UNIX families are
// recognized.
absl::StatusOr<std::string> ResolvedAddressToString(
    const EventEngine::ResolvedAddress& resolved_addr);

// Converts a EventEngine::ResolvedAddress into a newly-allocated
// human-readable string. See ResolvedAddressToString.
// This functional normalizes, so for example: ::ffff:0.0.0.0/96 IPv6
// addresses are displayed as plain IPv4.
absl::StatusOr<std::string> ResolvedAddressToNormalizedString(
    const EventEngine::ResolvedAddress& resolved_addr);

// Returns the URI string corresponding to the resolved_address
absl::StatusOr<std::string> ResolvedAddressToURI(
    const EventEngine::ResolvedAddress& resolved_address);

// Given a URI string, returns the corresponding resolved address if the URI
// is valid. Otherwise it returns an appropriate error.
absl::StatusOr<EventEngine::ResolvedAddress> URIToResolvedAddress(
    std::string address_str);

}  // namespace grpc_event_engine::experimental

#endif  // GRPC_SRC_CORE_LIB_EVENT_ENGINE_TCP_SOCKET_UTILS_H
