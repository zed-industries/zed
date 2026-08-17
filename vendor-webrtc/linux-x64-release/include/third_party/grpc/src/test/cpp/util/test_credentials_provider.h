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

#ifndef GRPC_TEST_CPP_UTIL_TEST_CREDENTIALS_PROVIDER_H
#define GRPC_TEST_CPP_UTIL_TEST_CREDENTIALS_PROVIDER_H

#include <grpcpp/security/credentials.h>
#include <grpcpp/security/server_credentials.h>
#include <grpcpp/support/channel_arguments.h>

#include <memory>

namespace grpc {
namespace testing {

const char kInsecureCredentialsType[] = "INSECURE_CREDENTIALS";
// For real credentials, like tls/ssl, this name should match the AuthContext
// property "transport_security_type".
const char kTlsCredentialsType[] = "ssl";
const char kAltsCredentialsType[] = "alts";
const char kGoogleDefaultCredentialsType[] = "google_default_credentials";

// Provide test credentials of a particular type.
class CredentialTypeProvider {
 public:
  virtual ~CredentialTypeProvider() {}

  virtual std::shared_ptr<ChannelCredentials> GetChannelCredentials(
      ChannelArguments* args) = 0;
  virtual std::shared_ptr<ServerCredentials> GetServerCredentials() = 0;
};

// Provide test credentials. Thread-safe.
class CredentialsProvider {
 public:
  virtual ~CredentialsProvider() {}

  // Add a secure type in addition to the defaults. The default provider has
  // (kInsecureCredentialsType, kTlsCredentialsType).
  virtual void AddSecureType(
      const std::string& type,
      std::unique_ptr<CredentialTypeProvider> type_provider) = 0;

  // Provide channel credentials according to the given type. Alter the channel
  // arguments if needed. Return nullptr if type is not registered.
  virtual std::shared_ptr<ChannelCredentials> GetChannelCredentials(
      const std::string& type, ChannelArguments* args) = 0;

  // Provide server credentials according to the given type.
  // Return nullptr if type is not registered.
  virtual std::shared_ptr<ServerCredentials> GetServerCredentials(
      const std::string& type) = 0;

  // Provide a list of secure credentials type.
  virtual std::vector<std::string> GetSecureCredentialsTypeList() = 0;
};

// Get the current provider. Create a default one if not set.
// Not thread-safe.
CredentialsProvider* GetCredentialsProvider();

// Set the global provider. Takes ownership. The previous set provider will be
// destroyed.
// Not thread-safe.
void SetCredentialsProvider(CredentialsProvider* provider);

}  // namespace testing
}  // namespace grpc

#endif  // GRPC_TEST_CPP_UTIL_TEST_CREDENTIALS_PROVIDER_H
