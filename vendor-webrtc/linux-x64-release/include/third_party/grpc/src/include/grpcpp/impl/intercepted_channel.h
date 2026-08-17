//
//
// Copyright 2018 gRPC authors.
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

#ifndef GRPCPP_IMPL_INTERCEPTED_CHANNEL_H
#define GRPCPP_IMPL_INTERCEPTED_CHANNEL_H

#include <grpcpp/impl/channel_interface.h>

namespace grpc {
class CompletionQueue;

namespace internal {

class InterceptorBatchMethodsImpl;

/// An InterceptedChannel is available to client Interceptors. An
/// InterceptedChannel is unique to an interceptor, and when an RPC is started
/// on this channel, only those interceptors that come after this interceptor
/// see the RPC.
class InterceptedChannel : public ChannelInterface {
 public:
  ~InterceptedChannel() override { channel_ = nullptr; }

  /// Get the current channel state. If the channel is in IDLE and
  /// \a try_to_connect is set to true, try to connect.
  grpc_connectivity_state GetState(bool try_to_connect) override {
    return channel_->GetState(try_to_connect);
  }

 private:
  InterceptedChannel(ChannelInterface* channel, size_t pos)
      : channel_(channel), interceptor_pos_(pos) {}

  Call CreateCall(const RpcMethod& method, grpc::ClientContext* context,
                  grpc::CompletionQueue* cq) override {
    return channel_->CreateCallInternal(method, context, cq, interceptor_pos_);
  }

  void PerformOpsOnCall(CallOpSetInterface* ops, Call* call) override {
    return channel_->PerformOpsOnCall(ops, call);
  }
  void* RegisterMethod(const char* method) override {
    return channel_->RegisterMethod(method);
  }

  void NotifyOnStateChangeImpl(grpc_connectivity_state last_observed,
                               gpr_timespec deadline, grpc::CompletionQueue* cq,
                               void* tag) override {
    return channel_->NotifyOnStateChangeImpl(last_observed, deadline, cq, tag);
  }
  bool WaitForStateChangeImpl(grpc_connectivity_state last_observed,
                              gpr_timespec deadline) override {
    return channel_->WaitForStateChangeImpl(last_observed, deadline);
  }

  grpc::CompletionQueue* CallbackCQ() override {
    return channel_->CallbackCQ();
  }

  ChannelInterface* channel_;
  size_t interceptor_pos_;

  friend class InterceptorBatchMethodsImpl;
};
}  // namespace internal
}  // namespace grpc

#endif  // GRPCPP_IMPL_INTERCEPTED_CHANNEL_H
