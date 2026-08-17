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

#ifndef GRPCPP_TEST_SERVER_CONTEXT_TEST_SPOUSE_H
#define GRPCPP_TEST_SERVER_CONTEXT_TEST_SPOUSE_H

#include <grpcpp/server_context.h>

#include <map>

namespace grpc {
namespace testing {

/// A test-only class to access private members and methods of ServerContext.
class ServerContextTestSpouse {
 public:
  explicit ServerContextTestSpouse(ServerContext* ctx) : ctx_(ctx) {}

  /// Inject client metadata to the ServerContext for the test. The test spouse
  /// must be alive when \a ServerContext::client_metadata is called.
  void AddClientMetadata(const std::string& key, const std::string& value) {
    client_metadata_storage_.insert(
        std::pair<std::string, std::string>(key, value));
    ctx_->client_metadata_.map()->clear();
    for (const auto& item : client_metadata_storage_) {
      ctx_->client_metadata_.map()->insert(
          std::pair<grpc::string_ref, grpc::string_ref>(
              item.first.c_str(),
              grpc::string_ref(item.second.data(), item.second.size())));
    }
  }

  std::multimap<std::string, std::string> GetInitialMetadata() const {
    return ctx_->initial_metadata_;
  }

  std::multimap<std::string, std::string> GetTrailingMetadata() const {
    return ctx_->trailing_metadata_;
  }

 private:
  ServerContext* ctx_;  // not owned
  std::multimap<std::string, std::string> client_metadata_storage_;
};

}  // namespace testing
}  // namespace grpc

#endif  // GRPCPP_TEST_SERVER_CONTEXT_TEST_SPOUSE_H
