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

#ifndef GRPCPP_IMPL_CLIENT_UNARY_CALL_H
#define GRPCPP_IMPL_CLIENT_UNARY_CALL_H

#include <grpcpp/impl/call.h>
#include <grpcpp/impl/call_op_set.h>
#include <grpcpp/impl/channel_interface.h>
#include <grpcpp/support/config.h>
#include <grpcpp/support/status.h>

namespace grpc {

class ClientContext;
namespace internal {
class RpcMethod;

/// Wrapper that performs a blocking unary call. May optionally specify the base
/// class of the Request and Response so that the internal calls and structures
/// below this may be based on those base classes and thus achieve code reuse
/// across different RPCs (e.g., for protobuf, MessageLite would be a base
/// class).
template <class InputMessage, class OutputMessage,
          class BaseInputMessage = InputMessage,
          class BaseOutputMessage = OutputMessage>
Status BlockingUnaryCall(ChannelInterface* channel, const RpcMethod& method,
                         grpc::ClientContext* context,
                         const InputMessage& request, OutputMessage* result) {
  static_assert(std::is_base_of<BaseInputMessage, InputMessage>::value,
                "Invalid input message specification");
  static_assert(std::is_base_of<BaseOutputMessage, OutputMessage>::value,
                "Invalid output message specification");
  return BlockingUnaryCallImpl<BaseInputMessage, BaseOutputMessage>(
             channel, method, context, request, result)
      .status();
}

template <class InputMessage, class OutputMessage>
class BlockingUnaryCallImpl {
 public:
  BlockingUnaryCallImpl(ChannelInterface* channel, const RpcMethod& method,
                        grpc::ClientContext* context,
                        const InputMessage& request, OutputMessage* result) {
    grpc::CompletionQueue cq(grpc_completion_queue_attributes{
        GRPC_CQ_CURRENT_VERSION, GRPC_CQ_PLUCK, GRPC_CQ_DEFAULT_POLLING,
        nullptr});  // Pluckable completion queue
    grpc::internal::Call call(channel->CreateCall(method, context, &cq));
    CallOpSet<CallOpSendInitialMetadata, CallOpSendMessage,
              CallOpRecvInitialMetadata, CallOpRecvMessage<OutputMessage>,
              CallOpClientSendClose, CallOpClientRecvStatus>
        ops;
    status_ = ops.SendMessagePtr(&request);
    if (!status_.ok()) {
      return;
    }
    ops.SendInitialMetadata(&context->send_initial_metadata_,
                            context->initial_metadata_flags());
    ops.RecvInitialMetadata(context);
    ops.RecvMessage(result);
    ops.AllowNoMessage();
    ops.ClientSendClose();
    ops.ClientRecvStatus(context, &status_);
    call.PerformOps(&ops);
    cq.Pluck(&ops);
    // Some of the ops might fail. If the ops fail in the core layer, status
    // would reflect the error. But, if the ops fail in the C++ layer, the
    // status would still be the same as the one returned by gRPC Core. This can
    // happen if deserialization of the message fails.
    // TODO(yashykt): If deserialization fails, but the status received is OK,
    // then it might be a good idea to change the status to something better
    // than StatusCode::UNIMPLEMENTED to reflect this.
    if (!ops.got_message && status_.ok()) {
      status_ = Status(StatusCode::UNIMPLEMENTED,
                       "No message returned for unary request");
    }
  }
  Status status() { return status_; }

 private:
  Status status_;
};

}  // namespace internal
}  // namespace grpc

#endif  // GRPCPP_IMPL_CLIENT_UNARY_CALL_H
