// Copyright 2024 The gRPC Authors
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
#ifndef GRPC_TEST_CPP_UTIL_CREDENTIALS_H
#define GRPC_TEST_CPP_UTIL_CREDENTIALS_H

#include <grpcpp/security/credentials.h>

#include "src/core/credentials/transport/fake/fake_credentials.h"

namespace grpc {
namespace testing {

class FakeTransportSecurityChannelCredentials : public ChannelCredentials {
 public:
  FakeTransportSecurityChannelCredentials()
      : ChannelCredentials(grpc_fake_transport_security_credentials_create()) {}
};

class TestCompositeChannelCredentials : public ChannelCredentials {
 public:
  TestCompositeChannelCredentials(grpc_channel_credentials* channel_creds,
                                  grpc_call_credentials* call_creds)
      : ChannelCredentials(grpc_composite_channel_credentials_create(
            channel_creds, call_creds, nullptr)) {}
};

}  // namespace testing
}  // namespace grpc

#endif  // GRPC_TEST_CPP_UTIL_CREDENTIALS_H
