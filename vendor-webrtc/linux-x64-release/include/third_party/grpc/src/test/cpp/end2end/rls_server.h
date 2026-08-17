//
// Copyright 2020 gRPC authors.
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

#ifndef GRPC_TEST_CPP_END2END_RLS_SERVER_H
#define GRPC_TEST_CPP_END2END_RLS_SERVER_H

#include <optional>

#include "src/core/util/time.h"
#include "src/proto/grpc/lookup/v1/rls.grpc.pb.h"
#include "src/proto/grpc/lookup/v1/rls.pb.h"
#include "test/cpp/end2end/counted_service.h"

namespace grpc {
namespace testing {

using RlsService =
    CountedService<grpc::lookup::v1::RouteLookupService::Service>;

class RlsServiceImpl : public RlsService {
 public:
  using ContextProcessingFunc = std::function<void(grpc::ServerContext*)>;

  explicit RlsServiceImpl(ContextProcessingFunc context_proc = nullptr)
      : context_proc_(std::move(context_proc)) {}

  grpc::Status RouteLookup(
      grpc::ServerContext* context,
      const grpc::lookup::v1::RouteLookupRequest* request,
      grpc::lookup::v1::RouteLookupResponse* response) override;

  void Start() {}

  void Shutdown() {}

  void SetResponse(grpc::lookup::v1::RouteLookupRequest request,
                   grpc::lookup::v1::RouteLookupResponse response,
                   grpc_core::Duration response_delay = grpc_core::Duration());

  void RemoveResponse(const grpc::lookup::v1::RouteLookupRequest& request);

  std::vector<grpc::lookup::v1::RouteLookupRequest> GetUnmatchedRequests();

 private:
  // Sorting thunk for RouteLookupRequest.
  struct RlsRequestLessThan {
    bool operator()(const grpc::lookup::v1::RouteLookupRequest& req1,
                    const grpc::lookup::v1::RouteLookupRequest& req2) const {
      std::map<absl::string_view, absl::string_view> key_map1(
          req1.key_map().begin(), req1.key_map().end());
      std::map<absl::string_view, absl::string_view> key_map2(
          req2.key_map().begin(), req2.key_map().end());
      if (key_map1 < key_map2) return true;
      if (req1.reason() < req2.reason()) return true;
      if (req1.stale_header_data() < req2.stale_header_data()) return true;
      return false;
    }
  };

  struct ResponseData {
    grpc::lookup::v1::RouteLookupResponse response;
    grpc_core::Duration response_delay;
  };

  ContextProcessingFunc context_proc_;
  grpc::internal::Mutex mu_;
  std::map<grpc::lookup::v1::RouteLookupRequest, ResponseData,
           RlsRequestLessThan>
      responses_ ABSL_GUARDED_BY(&mu_);
  std::vector<grpc::lookup::v1::RouteLookupRequest> unmatched_requests_
      ABSL_GUARDED_BY(&mu_);
};

grpc::lookup::v1::RouteLookupRequest BuildRlsRequest(
    std::map<std::string, std::string> key,
    grpc::lookup::v1::RouteLookupRequest::Reason reason =
        grpc::lookup::v1::RouteLookupRequest::REASON_MISS,
    const char* stale_header_data = "");

grpc::lookup::v1::RouteLookupResponse BuildRlsResponse(
    std::vector<std::string> targets, const char* header_data = "");

}  // namespace testing
}  // namespace grpc

#endif  // GRPC_TEST_CPP_END2END_RLS_SERVER_H
