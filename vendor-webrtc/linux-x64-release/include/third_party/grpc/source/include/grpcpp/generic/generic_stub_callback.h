//
//
// Copyright 2024 gRPC authors.
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

#ifndef GRPCPP_GENERIC_GENERIC_STUB_CALLBACK_H
#define GRPCPP_GENERIC_GENERIC_STUB_CALLBACK_H

#include <grpcpp/impl/generic_stub_internal.h>
#include <grpcpp/support/byte_buffer.h>

namespace grpc {

/// Generic stubs provide a type-unaware interface to call gRPC methods
/// by name. In practice, the Request and Response types should be basic
/// types like grpc::ByteBuffer or proto::MessageLite (the base protobuf).
template <class RequestType, class ResponseType>
class TemplatedGenericStubCallback final
    : public internal::TemplatedGenericStubCallbackInternal<RequestType,
                                                            ResponseType> {
 public:
  using internal::TemplatedGenericStubCallbackInternal<
      RequestType, ResponseType>::TemplatedGenericStubCallbackInternal;
};

typedef TemplatedGenericStubCallback<grpc::ByteBuffer, grpc::ByteBuffer>
    GenericStubCallback;

}  // namespace grpc

#endif  // GRPCPP_GENERIC_GENERIC_STUB_CALLBACK_H
