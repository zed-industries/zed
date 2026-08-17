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

#ifndef GRPCPP_CHANNEL_H
#define GRPCPP_CHANNEL_H

#include <grpc/grpc.h>
#include <grpcpp/completion_queue.h>
#include <grpcpp/impl/call.h>
#include <grpcpp/impl/channel_interface.h>
#include <grpcpp/impl/grpc_library.h>
#include <grpcpp/impl/sync.h>
#include <grpcpp/support/client_interceptor.h>
#include <grpcpp/support/config.h>

#include <memory>

struct grpc_channel;

namespace grpc {
namespace testing {
class ChannelTestPeer;
}  // namespace testing

std::shared_ptr<Channel> CreateChannelInternal(
    const std::string& host, grpc_channel* c_channel,
    std::vector<
        std::unique_ptr<experimental::ClientInterceptorFactoryInterface>>
        interceptor_creators);

namespace experimental {
/// Resets the channel's connection backoff.
/// TODO(roth): Once we see whether this proves useful, either create a gRFC
/// and change this to be a method of the Channel class, or remove it.
void ChannelResetConnectionBackoff(Channel* channel);
}  // namespace experimental

/// Channels represent a connection to an endpoint. Created by \a CreateChannel.
class Channel final : public grpc::ChannelInterface,
                      public grpc::internal::CallHook,
                      public std::enable_shared_from_this<Channel>,
                      private grpc::internal::GrpcLibrary {
 public:
  ~Channel() override;

  /// Get the current channel state. If the channel is in IDLE and
  /// \a try_to_connect is set to true, try to connect.
  grpc_connectivity_state GetState(bool try_to_connect) override;

  /// Returns the LB policy name, or the empty string if not yet available.
  std::string GetLoadBalancingPolicyName() const;

  /// Returns the service config in JSON form, or the empty string if
  /// not available.
  std::string GetServiceConfigJSON() const;

 private:
  template <class InputMessage, class OutputMessage>
  friend class grpc::internal::BlockingUnaryCallImpl;
  friend class grpc::testing::ChannelTestPeer;
  friend void experimental::ChannelResetConnectionBackoff(Channel* channel);
  friend std::shared_ptr<Channel> grpc::CreateChannelInternal(
      const std::string& host, grpc_channel* c_channel,
      std::vector<std::unique_ptr<
          grpc::experimental::ClientInterceptorFactoryInterface>>
          interceptor_creators);
  friend class grpc::internal::InterceptedChannel;
  Channel(const std::string& host, grpc_channel* c_channel,
          std::vector<std::unique_ptr<
              grpc::experimental::ClientInterceptorFactoryInterface>>
              interceptor_creators);

  grpc::internal::Call CreateCall(const grpc::internal::RpcMethod& method,
                                  grpc::ClientContext* context,
                                  grpc::CompletionQueue* cq) override;
  void PerformOpsOnCall(grpc::internal::CallOpSetInterface* ops,
                        grpc::internal::Call* call) override;
  void* RegisterMethod(const char* method) override;

  void NotifyOnStateChangeImpl(grpc_connectivity_state last_observed,
                               gpr_timespec deadline, grpc::CompletionQueue* cq,
                               void* tag) override;
  bool WaitForStateChangeImpl(grpc_connectivity_state last_observed,
                              gpr_timespec deadline) override;

  grpc::CompletionQueue* CallbackCQ() override;

  grpc::internal::Call CreateCallInternal(
      const grpc::internal::RpcMethod& method, grpc::ClientContext* context,
      grpc::CompletionQueue* cq, size_t interceptor_pos) override;

  const std::string host_;
  grpc_channel* const c_channel_;  // owned

  // mu_ protects callback_cq_ (the per-channel callbackable completion queue)
  grpc::internal::Mutex mu_;

  // callback_cq_ references the callbackable completion queue associated
  // with this channel (if any). It is set on the first call to CallbackCQ().
  // It is _not owned_ by the channel; ownership belongs with its internal
  // shutdown callback tag (invoked when the CQ is fully shutdown).
  std::atomic<CompletionQueue*> callback_cq_{nullptr};

  std::vector<
      std::unique_ptr<grpc::experimental::ClientInterceptorFactoryInterface>>
      interceptor_creators_;
};

}  // namespace grpc

#endif  // GRPCPP_CHANNEL_H
