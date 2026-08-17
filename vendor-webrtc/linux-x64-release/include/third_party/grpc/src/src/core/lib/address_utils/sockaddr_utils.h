//
//
// Copyright 2015 gRPC authors.
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
//
//

#ifndef GRPC_SRC_CORE_LIB_ADDRESS_UTILS_SOCKADDR_UTILS_H
#define GRPC_SRC_CORE_LIB_ADDRESS_UTILS_SOCKADDR_UTILS_H

#include <grpc/support/port_platform.h>
#include <stdint.h>

#include <string>

#include "absl/status/statusor.h"
#include "src/core/lib/iomgr/resolved_address.h"

// Returns true if addr is an IPv4-mapped IPv6 address within the
// ::ffff:0.0.0.0/96 range, or false otherwise.

// If addr4_out is non-NULL, the inner IPv4 address will be copied here when
// returning true.
int grpc_sockaddr_is_v4mapped(const grpc_resolved_address* addr,
                              grpc_resolved_address* addr4_out);

// If addr is an AF_INET address, writes the corresponding ::ffff:0.0.0.0/96
// address to addr6_out and returns true.  Otherwise returns false.
int grpc_sockaddr_to_v4mapped(const grpc_resolved_address* addr,
                              grpc_resolved_address* addr6_out);

// If addr is ::, 0.0.0.0, or ::ffff:0.0.0.0, writes the port number to
// port_out (if not NULL) and returns true, otherwise returns false.
int grpc_sockaddr_is_wildcard(const grpc_resolved_address* addr, int* port_out);

// Writes 0.0.0.0:port and [::]:port to separate sockaddrs.
void grpc_sockaddr_make_wildcards(int port, grpc_resolved_address* wild4_out,
                                  grpc_resolved_address* wild6_out);

// Writes 0.0.0.0:port.
void grpc_sockaddr_make_wildcard4(int port, grpc_resolved_address* wild_out);

// Writes [::]:port.
void grpc_sockaddr_make_wildcard6(int port, grpc_resolved_address* wild_out);

// Return the IP port number of a sockaddr
int grpc_sockaddr_get_port(const grpc_resolved_address* addr);

// Set IP port number of a sockaddr
int grpc_sockaddr_set_port(grpc_resolved_address* addr, int port);

// Converts a sockaddr into a newly-allocated human-readable string.
//
// Currently, only the AF_INET, AF_INET6, and AF_UNIX families are recognized.
// If the normalize flag is enabled, ::ffff:0.0.0.0/96 IPv6 addresses are
// displayed as plain IPv4.
GRPC_MUST_USE_RESULT absl::StatusOr<std::string> grpc_sockaddr_to_string(
    const grpc_resolved_address* addr, bool normalize);

// Returns the URI string corresponding to \a addr
absl::StatusOr<std::string> grpc_sockaddr_to_uri(
    const grpc_resolved_address* addr);

// Returns the URI scheme corresponding to \a addr
const char* grpc_sockaddr_get_uri_scheme(const grpc_resolved_address* addr);

int grpc_sockaddr_get_family(const grpc_resolved_address* resolved_addr);

std::string grpc_sockaddr_get_packed_host(
    const grpc_resolved_address* resolved_addr);

// Applies a mask of \a mask_bits to IPv4/IPv6 addresses. Has no effect if the
// address type is not IPv4/IPv6.
void grpc_sockaddr_mask_bits(grpc_resolved_address* address,
                             uint32_t mask_bits);

// If \a address is IPv4/IPv6, checks if the IP address falls in the CIDR
// specified by \a subnet_address and \a mask_bits.
// Returns false if \a address is not an IPv4/IPv6 address. The ports (if set)
// are ignored for matching purposes. Note that, \a subnet_address should be
// normalized, i.e., `grpc_sockaddr_mask_bits` should have been called on it if
// necessary.
bool grpc_sockaddr_match_subnet(const grpc_resolved_address* address,
                                const grpc_resolved_address* subnet_address,
                                uint32_t mask_bits);

#endif  // GRPC_SRC_CORE_LIB_ADDRESS_UTILS_SOCKADDR_UTILS_H
