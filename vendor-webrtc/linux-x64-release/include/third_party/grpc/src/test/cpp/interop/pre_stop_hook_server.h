//
//
// Copyright 2023 gRPC authors.
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

#ifndef GRPC_TEST_CPP_INTEROP_PRE_STOP_HOOK_SERVER_H
#define GRPC_TEST_CPP_INTEROP_PRE_STOP_HOOK_SERVER_H

#include <grpc/support/port_platform.h>
#include <grpcpp/server.h>

#include "src/core/util/sync.h"
#include "src/proto/grpc/testing/messages.pb.h"
#include "src/proto/grpc/testing/test.grpc.pb.h"

namespace grpc {
namespace testing {

class HookServiceImpl final : public HookService::CallbackService {
 public:
  ServerUnaryReactor* Hook(CallbackServerContext* context,
                           const Empty* /* request */,
                           Empty* /* reply */) override;

  ServerUnaryReactor* SetReturnStatus(CallbackServerContext* context,
                                      const SetReturnStatusRequest* request,
                                      Empty* /* reply */) override;

  ServerUnaryReactor* ClearReturnStatus(CallbackServerContext* context,
                                        const Empty* request,
                                        Empty* /* reply */) override;

  void AddReturnStatus(const Status& status);

  bool TestOnlyExpectRequests(size_t expected_requests_count,
                              const absl::Duration& timeout);

  void Stop();

 private:
  void MatchRequestsAndStatuses() ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_);

  grpc_core::Mutex mu_;
  grpc_core::CondVar request_var_ ABSL_GUARDED_BY(&mu_);
  std::vector<ServerUnaryReactor*> pending_requests_ ABSL_GUARDED_BY(&mu_);
  std::vector<Status> pending_statuses_ ABSL_GUARDED_BY(&mu_);
  std::optional<Status> respond_all_status_ ABSL_GUARDED_BY(&mu_);
};

// Implementation of the pre-stop hook server. An instance is created to start
// a server and destroyed to stop one.
class PreStopHookServer;

// Interface for interacting with PreStopHookServer. Provides operations
// required by the protocol, such as start, stop and return from the call.
class PreStopHookServerManager {
 public:
  Status Start(int port, size_t timeout_s);
  Status Stop();
  void Return(StatusCode code, absl::string_view description);
  // Suspends the thread until there are pending requests. Returns false
  // if the necessary number of requests have not been received before the
  // timeout.
  bool TestOnlyExpectRequests(
      size_t expected_requests_count,
      const absl::Duration& timeout = absl::Seconds(15));

 private:
  // Custom deleter so we don't have to include PreStopHookServer in this header
  struct PreStopHookServerDeleter {
    void operator()(PreStopHookServer* server);
  };

  std::unique_ptr<PreStopHookServer, PreStopHookServerDeleter> server_;
};

}  // namespace testing
}  // namespace grpc
#endif  // GRPC_TEST_CPP_INTEROP_PRE_STOP_HOOK_SERVER_H
