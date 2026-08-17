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

#ifndef GRPCPP_IMPL_GRPC_LIBRARY_H
#define GRPCPP_IMPL_GRPC_LIBRARY_H

#include <grpc/grpc.h>
#include <grpcpp/impl/codegen/config.h>

#include <iostream>

namespace grpc {

namespace internal {

/// Classes that require gRPC to be initialized should inherit from this class.
class GrpcLibrary {
 public:
  explicit GrpcLibrary(bool call_grpc_init = true) : grpc_init_called_(false) {
    if (call_grpc_init) {
      grpc_init();
      grpc_init_called_ = true;
    }
  }
  virtual ~GrpcLibrary() {
    if (grpc_init_called_) {
      grpc_shutdown();
    }
  }

 private:
  bool grpc_init_called_;
};

}  // namespace internal
}  // namespace grpc

#endif  // GRPCPP_IMPL_GRPC_LIBRARY_H
