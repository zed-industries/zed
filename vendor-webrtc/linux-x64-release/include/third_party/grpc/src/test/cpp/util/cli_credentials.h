//
//
// Copyright 2016 gRPC authors.
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

#ifndef GRPC_TEST_CPP_UTIL_CLI_CREDENTIALS_H
#define GRPC_TEST_CPP_UTIL_CLI_CREDENTIALS_H

#include <grpcpp/security/credentials.h>
#include <grpcpp/support/config.h>

namespace grpc {
namespace testing {

class CliCredentials {
 public:
  virtual ~CliCredentials() {}
  std::shared_ptr<grpc::ChannelCredentials> GetCredentials() const;
  virtual std::string GetCredentialUsage() const;
  virtual std::string GetSslTargetNameOverride() const;

 protected:
  // Returns the appropriate channel_creds_type value for the set of legacy
  // flag arguments.
  virtual std::string GetDefaultChannelCredsType() const;
  // Returns the appropriate call_creds value for the set of legacy flag
  // arguments.
  virtual std::string GetDefaultCallCreds() const;
  // Returns the base transport channel credentials. Child classes can override
  // to support additional channel_creds_types unknown to this base class.
  virtual std::shared_ptr<grpc::ChannelCredentials> GetChannelCredentials()
      const;
  // Returns call credentials to composite onto the base transport channel
  // credentials. Child classes can override to support additional
  // authentication flags unknown to this base class.
  virtual std::shared_ptr<grpc::CallCredentials> GetCallCredentials() const;
};

}  // namespace testing
}  // namespace grpc

#endif  // GRPC_TEST_CPP_UTIL_CLI_CREDENTIALS_H
