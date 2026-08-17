// Copyright 2025 gRPC authors.
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

#ifndef GRPC_TEST_CORE_HANDSHAKE_TEST_HANDSHAKE_H
#define GRPC_TEST_CORE_HANDSHAKE_TEST_HANDSHAKE_H

#include "src/core/lib/channel/channel_args.h"
#include "test/core/event_engine/fuzzing_event_engine/fuzzing_event_engine.pb.h"

namespace grpc_core {

// Create client, server connections, perform a handshake, return the result.
// Runs under a fuzzing event engine, and fuzzing parameters can be supplied as
// the last argument.
absl::StatusOr<std::tuple<ChannelArgs, ChannelArgs>> TestHandshake(
    ChannelArgs client_args, ChannelArgs server_args,
    const fuzzing_event_engine::Actions& actions =
        fuzzing_event_engine::Actions());

}  // namespace grpc_core

#endif  // GRPC_TEST_CORE_HANDSHAKE_TEST_HANDSHAKE_H
