//
//
// Copyright 2019 gRPC authors.
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

#ifndef GRPCPP_SUPPORT_MESSAGE_ALLOCATOR_H
#define GRPCPP_SUPPORT_MESSAGE_ALLOCATOR_H

namespace grpc {

// NOTE: This is an API for advanced users who need custom allocators.
// Per rpc struct for the allocator. This is the interface to return to user.
class RpcAllocatorState {
 public:
  virtual ~RpcAllocatorState() = default;
  // Optionally deallocate request early to reduce the size of working set.
  // A custom MessageAllocator needs to be registered to make use of this.
  // This is not abstract because implementing it is optional.
  virtual void FreeRequest() {}
};

// This is the interface returned by the allocator.
// grpc library will call the methods to get request/response pointers and to
// release the object when it is done.
template <typename RequestT, typename ResponseT>
class MessageHolder : public RpcAllocatorState {
 public:
  // Release this object. For example, if the custom allocator's
  // AllocateMessasge creates an instance of a subclass with new, the Release()
  // should do a "delete this;".
  virtual void Release() = 0;
  RequestT* request() { return request_; }
  ResponseT* response() { return response_; }

 protected:
  void set_request(RequestT* request) { request_ = request; }
  void set_response(ResponseT* response) { response_ = response; }

 private:
  // NOTE: subclasses should set these pointers.
  RequestT* request_;
  ResponseT* response_;
};

// A custom allocator can be set via the generated code to a callback unary
// method, such as SetMessageAllocatorFor_Echo(custom_allocator). The allocator
// needs to be alive for the lifetime of the server.
// Implementations need to be thread-safe.
template <typename RequestT, typename ResponseT>
class MessageAllocator {
 public:
  virtual ~MessageAllocator() = default;
  virtual MessageHolder<RequestT, ResponseT>* AllocateMessages() = 0;
};

}  // namespace grpc

#endif  // GRPCPP_SUPPORT_MESSAGE_ALLOCATOR_H
