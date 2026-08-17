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

#ifndef GRPCPP_GENERIC_GENERIC_STUB_H
#define GRPCPP_GENERIC_GENERIC_STUB_H

#include <grpcpp/client_context.h>
#include <grpcpp/impl/generic_stub_internal.h>
#include <grpcpp/impl/rpc_method.h>
#include <grpcpp/support/async_stream.h>
#include <grpcpp/support/async_unary_call.h>
#include <grpcpp/support/byte_buffer.h>
#include <grpcpp/support/stub_options.h>

namespace grpc {

class CompletionQueue;

typedef ClientAsyncReaderWriter<ByteBuffer, ByteBuffer>
    GenericClientAsyncReaderWriter;
typedef ClientAsyncResponseReader<ByteBuffer> GenericClientAsyncResponseReader;

/// Generic stubs provide a type-unaware interface to call gRPC methods
/// by name. In practice, the Request and Response types should be basic
/// types like grpc::ByteBuffer or proto::MessageLite (the base protobuf).
template <class RequestType, class ResponseType>
class TemplatedGenericStub final
    : public internal::TemplatedGenericStubCallbackInternal<RequestType,
                                                            ResponseType> {
 public:
  using internal::TemplatedGenericStubCallbackInternal<
      RequestType, ResponseType>::TemplatedGenericStubCallbackInternal;

  /// Setup a call to a named method \a method using \a context, but don't
  /// start it. Let it be started explicitly with StartCall and a tag.
  /// The return value only indicates whether or not registration of the call
  /// succeeded (i.e. the call won't proceed if the return value is nullptr).
  std::unique_ptr<ClientAsyncReaderWriter<RequestType, ResponseType>>
  PrepareCall(ClientContext* context, const std::string& method,
              grpc::CompletionQueue* cq) {
    return CallInternal(channel_.get(), context, method, /*options=*/{}, cq,
                        false, nullptr);
  }

  /// Setup a unary call to a named method \a method using \a context, and don't
  /// start it. Let it be started explicitly with StartCall.
  /// The return value only indicates whether or not registration of the call
  /// succeeded (i.e. the call won't proceed if the return value is nullptr).
  std::unique_ptr<ClientAsyncResponseReader<ResponseType>> PrepareUnaryCall(
      ClientContext* context, const std::string& method,
      const RequestType& request, grpc::CompletionQueue* cq) {
    return std::unique_ptr<ClientAsyncResponseReader<ResponseType>>(
        internal::ClientAsyncResponseReaderHelper::Create<ResponseType>(
            channel_.get(), cq,
            grpc::internal::RpcMethod(method.c_str(),
                                      /*suffix_for_stats=*/nullptr,
                                      grpc::internal::RpcMethod::NORMAL_RPC),
            context, request));
  }

  using internal::TemplatedGenericStubCallbackInternal<
      RequestType, ResponseType>::PrepareUnaryCall;

  /// DEPRECATED for multi-threaded use
  /// Begin a call to a named method \a method using \a context.
  /// A tag \a tag will be delivered to \a cq when the call has been started
  /// (i.e, initial metadata has been sent).
  /// The return value only indicates whether or not registration of the call
  /// succeeded (i.e. the call won't proceed if the return value is nullptr).
  std::unique_ptr<ClientAsyncReaderWriter<RequestType, ResponseType>> Call(
      ClientContext* context, const std::string& method,
      grpc::CompletionQueue* cq, void* tag) {
    return CallInternal(channel_.get(), context, method, /*options=*/{}, cq,
                        true, tag);
  }

 private:
  using internal::TemplatedGenericStubCallbackInternal<RequestType,
                                                       ResponseType>::channel_;

  std::unique_ptr<ClientAsyncReaderWriter<RequestType, ResponseType>>
  CallInternal(grpc::ChannelInterface* channel, ClientContext* context,
               const std::string& method, StubOptions options,
               grpc::CompletionQueue* cq, bool start, void* tag) {
    return std::unique_ptr<ClientAsyncReaderWriter<RequestType, ResponseType>>(
        internal::ClientAsyncReaderWriterFactory<RequestType, ResponseType>::
            Create(channel, cq,
                   grpc::internal::RpcMethod(
                       method.c_str(), options.suffix_for_stats(),
                       grpc::internal::RpcMethod::BIDI_STREAMING),
                   context, start, tag));
  }
};

typedef TemplatedGenericStub<grpc::ByteBuffer, grpc::ByteBuffer> GenericStub;

}  // namespace grpc

#endif  // GRPCPP_GENERIC_GENERIC_STUB_H
