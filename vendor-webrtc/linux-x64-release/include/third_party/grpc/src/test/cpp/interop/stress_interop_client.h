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
// is % allowed in string
//

#ifndef GRPC_TEST_CPP_INTEROP_STRESS_INTEROP_CLIENT_H
#define GRPC_TEST_CPP_INTEROP_STRESS_INTEROP_CLIENT_H

#include <grpcpp/create_channel.h>

#include <memory>
#include <string>
#include <vector>

#include "test/cpp/interop/interop_client.h"
#include "test/cpp/util/metrics_server.h"

namespace grpc {
namespace testing {

using std::pair;
using std::vector;

enum TestCaseType {
  UNKNOWN_TEST = -1,
  EMPTY_UNARY,
  LARGE_UNARY,
  CLIENT_COMPRESSED_UNARY,
  CLIENT_COMPRESSED_STREAMING,
  CLIENT_STREAMING,
  SERVER_STREAMING,
  SERVER_COMPRESSED_UNARY,
  SERVER_COMPRESSED_STREAMING,
  SLOW_CONSUMER,
  HALF_DUPLEX,
  PING_PONG,
  CANCEL_AFTER_BEGIN,
  CANCEL_AFTER_FIRST_RESPONSE,
  TIMEOUT_ON_SLEEPING_SERVER,
  EMPTY_STREAM,
  STATUS_CODE_AND_MESSAGE,
  CUSTOM_METADATA
};

const vector<pair<TestCaseType, std::string>> kTestCaseList = {
    {EMPTY_UNARY, "empty_unary"},
    {LARGE_UNARY, "large_unary"},
    {CLIENT_COMPRESSED_UNARY, "client_compressed_unary"},
    {CLIENT_COMPRESSED_STREAMING, "client_compressed_streaming"},
    {CLIENT_STREAMING, "client_streaming"},
    {SERVER_STREAMING, "server_streaming"},
    {SERVER_COMPRESSED_UNARY, "server_compressed_unary"},
    {SERVER_COMPRESSED_STREAMING, "server_compressed_streaming"},
    {SLOW_CONSUMER, "slow_consumer"},
    {HALF_DUPLEX, "half_duplex"},
    {PING_PONG, "ping_pong"},
    {CANCEL_AFTER_BEGIN, "cancel_after_begin"},
    {CANCEL_AFTER_FIRST_RESPONSE, "cancel_after_first_response"},
    {TIMEOUT_ON_SLEEPING_SERVER, "timeout_on_sleeping_server"},
    {EMPTY_STREAM, "empty_stream"},
    {STATUS_CODE_AND_MESSAGE, "status_code_and_message"},
    {CUSTOM_METADATA, "custom_metadata"}};

class WeightedRandomTestSelector {
 public:
  // Takes a vector of <test_case, weight> pairs as the input
  explicit WeightedRandomTestSelector(
      const vector<pair<TestCaseType, int>>& tests);

  // Returns a weighted-randomly chosen test case based on the test cases and
  // weights passed in the constructor
  TestCaseType GetNextTest() const;

 private:
  const vector<pair<TestCaseType, int>> tests_;
  int total_weight_;
};

class StressTestInteropClient {
 public:
  StressTestInteropClient(int test_id, const std::string& server_address,
                          ChannelCreationFunc channel_creation_func,
                          const WeightedRandomTestSelector& test_selector,
                          long test_duration_secs, long sleep_duration_ms,
                          bool do_not_abort_on_transient_failures);

  // The main function. Use this as the thread entry point.
  // qps_gauge is the QpsGauge to record the requests per second metric
  void MainLoop(const std::shared_ptr<QpsGauge>& qps_gauge);

 private:
  bool RunTest(TestCaseType test_case);

  int test_id_;
  const std::string& server_address_;
  ChannelCreationFunc channel_creation_func_;
  std::unique_ptr<InteropClient> interop_client_;
  const WeightedRandomTestSelector& test_selector_;
  long test_duration_secs_;
  long sleep_duration_ms_;
};

}  // namespace testing
}  // namespace grpc

#endif  // GRPC_TEST_CPP_INTEROP_STRESS_INTEROP_CLIENT_H
