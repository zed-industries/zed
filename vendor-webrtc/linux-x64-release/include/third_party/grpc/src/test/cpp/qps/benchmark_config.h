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

#ifndef GRPC_TEST_CPP_QPS_BENCHMARK_CONFIG_H
#define GRPC_TEST_CPP_QPS_BENCHMARK_CONFIG_H

#include <memory>

#include "test/cpp/qps/report.h"

namespace grpc {
namespace testing {

/// Returns the benchmark Reporter instance.
///
/// The returned instance will take care of generating reports for all the
/// actual reporters configured via the "enable_*_reporter" command line flags
/// (see benchmark_config.cc).
std::shared_ptr<Reporter> GetReporter();

}  // namespace testing
}  // namespace grpc

#endif  // GRPC_TEST_CPP_QPS_BENCHMARK_CONFIG_H
