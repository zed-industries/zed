// Copyright 2021 gRPC authors.
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

#ifndef GRPC_TEST_CORE_TEST_UTIL_EVALUATE_ARGS_TEST_UTIL_H
#define GRPC_TEST_CORE_TEST_UTIL_EVALUATE_ARGS_TEST_UTIL_H

#include <grpc/event_engine/memory_allocator.h>
#include <grpc/grpc_security.h>
#include <grpc/support/port_platform.h>
#include <stdlib.h>

#include <memory>

#include "absl/strings/string_view.h"
#include "src/core/call/metadata_batch.h"
#include "src/core/handshaker/endpoint_info/endpoint_info_handshaker.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/resource_quota/arena.h"
#include "src/core/lib/resource_quota/memory_quota.h"
#include "src/core/lib/resource_quota/resource_quota.h"
#include "src/core/lib/security/authorization/evaluate_args.h"
#include "src/core/lib/slice/slice.h"
#include "src/core/transport/auth_context.h"
#include "src/core/util/ref_counted_ptr.h"

namespace grpc_core {

class EvaluateArgsTestUtil final {
 public:
  void AddPairToMetadata(const char* key, const char* value) {
    metadata_.Append(key, Slice::FromStaticString(value),
                     [](absl::string_view, const Slice&) {
                       // We should never ever see an error here.
                       abort();
                     });
  }

  void SetLocalEndpoint(absl::string_view local_uri) {
    args_ = args_.Set(GRPC_ARG_ENDPOINT_LOCAL_ADDRESS, local_uri);
  }

  void SetPeerEndpoint(absl::string_view peer_uri) {
    args_ = args_.Set(GRPC_ARG_ENDPOINT_PEER_ADDRESS, peer_uri);
  }

  void AddPropertyToAuthContext(const char* name, const char* value) {
    auth_context_.add_cstring_property(name, value);
  }

  EvaluateArgs MakeEvaluateArgs() {
    channel_args_ =
        std::make_unique<EvaluateArgs::PerChannelArgs>(&auth_context_, args_);
    return EvaluateArgs(&metadata_, channel_args_.get());
  }

 private:
  MemoryAllocator allocator_ =
      ResourceQuota::Default()->memory_quota()->CreateMemoryAllocator(
          "EvaluateArgsTestUtil");
  grpc_metadata_batch metadata_;
  grpc_auth_context auth_context_{nullptr};
  ChannelArgs args_;
  std::unique_ptr<EvaluateArgs::PerChannelArgs> channel_args_;
};

}  // namespace grpc_core

#endif  // GRPC_TEST_CORE_TEST_UTIL_EVALUATE_ARGS_TEST_UTIL_H
