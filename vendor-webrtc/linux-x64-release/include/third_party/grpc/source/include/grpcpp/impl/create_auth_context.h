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

#ifndef GRPCPP_IMPL_CREATE_AUTH_CONTEXT_H
#define GRPCPP_IMPL_CREATE_AUTH_CONTEXT_H

#include <grpc/impl/grpc_types.h>
#include <grpcpp/security/auth_context.h>

#include <memory>

namespace grpc {

/// TODO(ctiller): not sure we want to make this a permanent thing
std::shared_ptr<const AuthContext> CreateAuthContext(grpc_call* call);

}  // namespace grpc

#endif  // GRPCPP_IMPL_CREATE_AUTH_CONTEXT_H
