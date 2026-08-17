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

#ifndef GRPCPP_IMPL_RPC_METHOD_H
#define GRPCPP_IMPL_RPC_METHOD_H

#include <grpcpp/impl/codegen/channel_interface.h>

#include <memory>

namespace grpc {
namespace internal {
/// Descriptor of an RPC method
class RpcMethod {
 public:
  enum RpcType {
    NORMAL_RPC = 0,
    CLIENT_STREAMING,  // request streaming
    SERVER_STREAMING,  // response streaming
    BIDI_STREAMING
  };

  RpcMethod(const char* name, RpcType type)
      : name_(name),
        suffix_for_stats_(nullptr),
        method_type_(type),
        channel_tag_(nullptr) {}

  RpcMethod(const char* name, const char* suffix_for_stats, RpcType type)
      : name_(name),
        suffix_for_stats_(suffix_for_stats),
        method_type_(type),
        channel_tag_(nullptr) {}

  RpcMethod(const char* name, RpcType type,
            const std::shared_ptr<ChannelInterface>& channel)
      : name_(name),
        suffix_for_stats_(nullptr),
        method_type_(type),
        channel_tag_(channel->RegisterMethod(name)) {}

  RpcMethod(const char* name, const char* suffix_for_stats, RpcType type,
            const std::shared_ptr<ChannelInterface>& channel)
      : name_(name),
        suffix_for_stats_(suffix_for_stats),
        method_type_(type),
        channel_tag_(channel->RegisterMethod(name)) {}

  const char* name() const { return name_; }
  const char* suffix_for_stats() const { return suffix_for_stats_; }
  RpcType method_type() const { return method_type_; }
  void SetMethodType(RpcType type) { method_type_ = type; }
  void* channel_tag() const { return channel_tag_; }

 private:
  const char* const name_;
  const char* const suffix_for_stats_;
  RpcType method_type_;
  void* const channel_tag_;
};

}  // namespace internal
}  // namespace grpc

#endif  // GRPCPP_IMPL_RPC_METHOD_H
