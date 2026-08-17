//
//
// Copyright 2020 gRPC authors.
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

#ifndef GRPC_TEST_CORE_TEST_UTIL_RESOLVE_LOCALHOST_IP46_H
#define GRPC_TEST_CORE_TEST_UTIL_RESOLVE_LOCALHOST_IP46_H

#include <string>

#include "absl/strings/string_view.h"

namespace grpc_core {

// Test whether localhost resolves to ipv4 and/or ipv6
void LocalhostResolves(bool* ipv4, bool* ipv6);

// Returns true if running with IPv6 only, false otherwise.
bool RunningWithIPv6Only();

// Returns the IP address of localhost.
// If RunningWithIPv6Only() is true, returns the IPv6 address;
// otherwise, returns the IPv4 address.
absl::string_view LocalIp();

// Returns LocalIp() with a port.
std::string LocalIpAndPort(int port);

// Returns the URI of the IP address of localhost with the given port.
// If RunningWithIPv6Only() is true, returns the IPv6 address;
// otherwise, returns the IPv4 address.
std::string LocalIpUri(int port);

}  // namespace grpc_core

#endif  // GRPC_TEST_CORE_TEST_UTIL_RESOLVE_LOCALHOST_IP46_H
