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

#ifndef GRPCPP_GENERIC_ASYNC_GENERIC_SERVICE_H
#define GRPCPP_GENERIC_ASYNC_GENERIC_SERVICE_H

#include <grpc/support/port_platform.h>
#include <grpcpp/generic/callback_generic_service.h>
#include <grpcpp/support/async_stream.h>
#include <grpcpp/support/byte_buffer.h>

struct grpc_server;

namespace grpc {

typedef ServerAsyncReaderWriter<ByteBuffer, ByteBuffer>
    GenericServerAsyncReaderWriter;
typedef ServerAsyncResponseWriter<ByteBuffer> GenericServerAsyncResponseWriter;
typedef ServerAsyncReader<ByteBuffer, ByteBuffer> GenericServerAsyncReader;
typedef ServerAsyncWriter<ByteBuffer> GenericServerAsyncWriter;

class GenericServerContext final : public ServerContext {
 public:
  const std::string& method() const { return method_; }
  const std::string& host() const { return host_; }

 private:
  friend class ServerInterface;

  std::string method_;
  std::string host_;
};

// A generic service at the server side accepts all RPC methods and hosts. It is
// typically used in proxies. The generic service can be registered to a server
// which also has other services.
// Sample usage:
//   ServerBuilder builder;
//   auto cq = builder.AddCompletionQueue();
//   AsyncGenericService generic_service;
//   builder.RegisterAsyncGenericService(&generic_service);
//   auto server = builder.BuildAndStart();
//
//   // request a new call
//   GenericServerContext context;
//   GenericServerAsyncReaderWriter stream;
//   generic_service.RequestCall(&context, &stream, cq.get(), cq.get(), tag);
//
// When tag is retrieved from cq->Next(), context.method() can be used to look
// at the method and the RPC can be handled accordingly.
class AsyncGenericService final {
 public:
  AsyncGenericService() : server_(nullptr) {}

  void RequestCall(GenericServerContext* ctx,
                   GenericServerAsyncReaderWriter* reader_writer,
                   grpc::CompletionQueue* call_cq,
                   grpc::ServerCompletionQueue* notification_cq, void* tag);

 private:
  friend class grpc::Server;
  grpc::Server* server_;
};

}  // namespace grpc

#endif  // GRPCPP_GENERIC_ASYNC_GENERIC_SERVICE_H
