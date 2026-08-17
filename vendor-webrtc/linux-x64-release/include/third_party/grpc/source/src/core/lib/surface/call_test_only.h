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

#ifndef GRPC_SRC_CORE_LIB_SURFACE_CALL_TEST_ONLY_H
#define GRPC_SRC_CORE_LIB_SURFACE_CALL_TEST_ONLY_H

#include <grpc/grpc.h>
#include <grpc/impl/compression_types.h>
#include <grpc/support/port_platform.h>
#include <stdint.h>

/// Return the message compression algorithm from \a call.
///
///\warning This function should \b only be used in test code.
grpc_compression_algorithm grpc_call_test_only_get_compression_algorithm(
    grpc_call* call);

/// Return the message flags from \a call.
///
///\warning This function should \b only be used in test code.
uint32_t grpc_call_test_only_get_message_flags(grpc_call* call);

/// Returns a bitset for the encodings (compression algorithms) supported by \a
/// call's peer.
///
/// To be indexed by grpc_compression_algorithm enum values.
uint32_t grpc_call_test_only_get_encodings_accepted_by_peer(grpc_call* call);

#endif  // GRPC_SRC_CORE_LIB_SURFACE_CALL_TEST_ONLY_H
