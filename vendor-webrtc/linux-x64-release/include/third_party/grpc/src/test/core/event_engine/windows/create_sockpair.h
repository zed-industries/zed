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
#ifndef GRPC_TEST_CORE_EVENT_ENGINE_WINDOWS_CREATE_SOCKPAIR_H
#define GRPC_TEST_CORE_EVENT_ENGINE_WINDOWS_CREATE_SOCKPAIR_H

#include <grpc/support/port_platform.h>
#ifdef GPR_WINDOWS

#include <winsock2.h>

namespace grpc_event_engine {
namespace experimental {

sockaddr_in GetSomeIpv4LoopbackAddress();

// Creates a connected pair of sockets on the loopback address
// sockpair[0] is a connected client
// sockpair[1] is a listener running `accept`  on the socket
void CreateSockpair(SOCKET sockpair[2], DWORD flags);

}  // namespace experimental
}  // namespace grpc_event_engine

#endif  // GPR_WINDOWS
#endif  // GRPC_TEST_CORE_EVENT_ENGINE_WINDOWS_CREATE_SOCKPAIR_H
