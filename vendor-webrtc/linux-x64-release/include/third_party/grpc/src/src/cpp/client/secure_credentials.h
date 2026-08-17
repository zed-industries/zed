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

#ifndef GRPC_SRC_CPP_CLIENT_SECURE_CREDENTIALS_H
#define GRPC_SRC_CPP_CLIENT_SECURE_CREDENTIALS_H

#include <grpc/grpc.h>
#include <grpc/grpc_security.h>
#include <grpc/status.h>
#include <grpcpp/channel.h>
#include <grpcpp/impl/grpc_library.h>
#include <grpcpp/security/credentials.h>
#include <grpcpp/support/channel_arguments.h>
#include <grpcpp/support/client_interceptor.h>

namespace grpc {

namespace experimental {

// Transforms C++ STS Credentials options to core options. The pointers of the
// resulting core options point to the memory held by the C++ options so C++
// options need to be kept alive until after the core credentials creation.
grpc_sts_credentials_options StsCredentialsCppToCoreOptions(
    const StsCredentialsOptions& options);

}  // namespace experimental

/// ---- DEPRECATED ----
/// This type is going away. Prefer creating a subclass of
/// grpc::ChannelCredentials.
class SecureChannelCredentials final : public grpc::ChannelCredentials {
 public:
  explicit SecureChannelCredentials(grpc_channel_credentials* c_creds)
      : ChannelCredentials(c_creds) {}
};

/// ---- DEPRECATED ----
/// This type is going away. Prefer creating a subclass of
/// grpc::CallCredentials.
class SecureCallCredentials final : public grpc::CallCredentials {
 public:
  explicit SecureCallCredentials(grpc_call_credentials* c_creds)
      : CallCredentials(c_creds) {}
};

}  // namespace grpc

#endif  // GRPC_SRC_CPP_CLIENT_SECURE_CREDENTIALS_H
