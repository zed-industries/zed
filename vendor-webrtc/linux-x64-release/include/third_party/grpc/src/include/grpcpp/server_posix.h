//
//
// Copyright 2016 gRPC authors.
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

#ifndef GRPCPP_SERVER_POSIX_H
#define GRPCPP_SERVER_POSIX_H

#include <grpc/support/port_platform.h>
#include <grpcpp/server.h>

#include <memory>

namespace grpc {

#ifdef GPR_SUPPORT_CHANNELS_FROM_FD

/// Add a new client to a \a Server communicating over the given
/// file descriptor.
///
/// \param server The server to add the client to.
/// \param fd The file descriptor representing a socket.
void AddInsecureChannelFromFd(Server* server, int fd);

#endif  // GPR_SUPPORT_CHANNELS_FROM_FD

}  // namespace grpc

#endif  // GRPCPP_SERVER_POSIX_H
