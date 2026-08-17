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

/// \mainpage gRPC C++ API
///
/// The gRPC C++ API mainly consists of the following classes:
/// <br>
/// - grpc::Channel, which represents the connection to an endpoint. See [the
/// gRPC Concepts page](https://grpc.io/docs/what-is-grpc/core-concepts) for
/// more details. Channels are created by the factory function
/// grpc::CreateChannel.
///
/// - grpc::CompletionQueue, the producer-consumer queue used for all
/// asynchronous communication with the gRPC runtime.
///
/// - grpc::ClientContext and grpc::ServerContext, where optional configuration
/// for an RPC can be set, such as setting custom metadata to be conveyed to the
/// peer, compression settings, authentication, etc.
///
/// - grpc::Server, representing a gRPC server, created by grpc::ServerBuilder.
///
/// Streaming calls are handled with the streaming classes in
/// \ref sync_stream.h and
/// \ref async_stream.h.
///
/// Refer to the
/// [examples](https://github.com/grpc/grpc/blob/master/examples/cpp)
/// for code putting these pieces into play.

#ifndef GRPCPP_GRPCPP_H
#define GRPCPP_GRPCPP_H

// Pragma for http://include-what-you-use.org/ tool, tells that following
// headers are not private for grpcpp.h and are part of its interface.
// IWYU pragma: begin_exports
#include <grpc/grpc.h>
#include <grpcpp/channel.h>
#include <grpcpp/client_context.h>
#include <grpcpp/completion_queue.h>
#include <grpcpp/create_channel.h>
#include <grpcpp/create_channel_posix.h>
#include <grpcpp/server.h>
#include <grpcpp/server_builder.h>
#include <grpcpp/server_context.h>
#include <grpcpp/server_posix.h>
#include <grpcpp/version_info.h>
// IWYU pragma: end_exports

namespace grpc {
/// Return gRPC library version.
std::string Version();
}  // namespace grpc

#endif  // GRPCPP_GRPCPP_H
