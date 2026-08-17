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

#ifndef GRPC_TEST_CPP_QPS_DRIVER_H
#define GRPC_TEST_CPP_QPS_DRIVER_H

#include <memory>

#include "src/proto/grpc/testing/control.pb.h"
#include "test/cpp/qps/histogram.h"

namespace grpc {
namespace testing {

std::unique_ptr<ScenarioResult> RunScenario(
    const grpc::testing::ClientConfig& client_config, size_t num_clients,
    const grpc::testing::ServerConfig& server_config, size_t num_servers,
    int warmup_seconds, int benchmark_seconds, int spawn_local_worker_count,
    const std::string& qps_server_target_override,
    const std::string& credential_type,
    const std::map<std::string, std::string>& per_worker_credential_types,
    bool run_inproc, int32_t median_latency_collection_interval_millis);

bool RunQuit(
    const std::string& credential_type,
    const std::map<std::string, std::string>& per_worker_credential_types);
}  // namespace testing
}  // namespace grpc

#endif  // GRPC_TEST_CPP_QPS_DRIVER_H
