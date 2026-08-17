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

#ifndef GRPC_TEST_CPP_QPS_REPORT_H
#define GRPC_TEST_CPP_QPS_REPORT_H

#include <grpcpp/channel.h>
#include <grpcpp/support/config.h>

#include <memory>
#include <set>
#include <vector>

#include "src/proto/grpc/testing/report_qps_scenario_service.grpc.pb.h"
#include "test/cpp/qps/driver.h"

namespace grpc {
namespace testing {

/// Interface for all reporters.
class Reporter {
 public:
  /// Construct a reporter with the given \a name.
  explicit Reporter(const string& name) : name_(name) {}

  virtual ~Reporter() {}

  /// Returns this reporter's name.
  ///
  /// Names are constants, set at construction time.
  string name() const { return name_; }

  /// Reports QPS for the given \a result.
  virtual void ReportQPS(const ScenarioResult& result) = 0;

  /// Reports QPS per core as (YYY/server core).
  virtual void ReportQPSPerCore(const ScenarioResult& result) = 0;

  /// Reports latencies for the 50, 90, 95, 99 and 99.9 percentiles, in ms.
  virtual void ReportLatency(const ScenarioResult& result) = 0;

  /// Reports system and user time for client and server systems.
  virtual void ReportTimes(const ScenarioResult& result) = 0;

  /// Reports server cpu usage.
  virtual void ReportCpuUsage(const ScenarioResult& result) = 0;

  /// Reports client and server poll usage inside completion queue.
  virtual void ReportPollCount(const ScenarioResult& result) = 0;

  /// Reports queries per cpu-sec.
  virtual void ReportQueriesPerCpuSec(const ScenarioResult& result) = 0;

 private:
  const string name_;
};

/// A composite for all reporters to be considered.
class CompositeReporter : public Reporter {
 public:
  CompositeReporter() : Reporter("CompositeReporter") {}

  /// Adds a \a reporter to the composite.
  void add(std::unique_ptr<Reporter> reporter);

  void ReportQPS(const ScenarioResult& result) override;
  void ReportQPSPerCore(const ScenarioResult& result) override;
  void ReportLatency(const ScenarioResult& result) override;
  void ReportTimes(const ScenarioResult& result) override;
  void ReportCpuUsage(const ScenarioResult& result) override;
  void ReportPollCount(const ScenarioResult& result) override;
  void ReportQueriesPerCpuSec(const ScenarioResult& result) override;

 private:
  std::vector<std::unique_ptr<Reporter> > reporters_;
};

///  Reporter to LOG(INFO).
class GprLogReporter : public Reporter {
 public:
  explicit GprLogReporter(const string& name) : Reporter(name) {}

 private:
  void ReportQPS(const ScenarioResult& result) override;
  void ReportQPSPerCore(const ScenarioResult& result) override;
  void ReportLatency(const ScenarioResult& result) override;
  void ReportTimes(const ScenarioResult& result) override;
  void ReportCpuUsage(const ScenarioResult& result) override;
  void ReportPollCount(const ScenarioResult& result) override;
  void ReportQueriesPerCpuSec(const ScenarioResult& result) override;
};

/// Dumps the report to a JSON file.
class JsonReporter : public Reporter {
 public:
  JsonReporter(const string& name, const string& report_file)
      : Reporter(name), report_file_(report_file) {}

 private:
  void ReportQPS(const ScenarioResult& result) override;
  void ReportQPSPerCore(const ScenarioResult& result) override;
  void ReportLatency(const ScenarioResult& result) override;
  void ReportTimes(const ScenarioResult& result) override;
  void ReportCpuUsage(const ScenarioResult& result) override;
  void ReportPollCount(const ScenarioResult& result) override;
  void ReportQueriesPerCpuSec(const ScenarioResult& result) override;

  const string report_file_;
};

class RpcReporter : public Reporter {
 public:
  RpcReporter(const string& name, const std::shared_ptr<grpc::Channel>& channel)
      : Reporter(name), stub_(ReportQpsScenarioService::NewStub(channel)) {}

 private:
  void ReportQPS(const ScenarioResult& result) override;
  void ReportQPSPerCore(const ScenarioResult& result) override;
  void ReportLatency(const ScenarioResult& result) override;
  void ReportTimes(const ScenarioResult& result) override;
  void ReportCpuUsage(const ScenarioResult& result) override;
  void ReportPollCount(const ScenarioResult& result) override;
  void ReportQueriesPerCpuSec(const ScenarioResult& result) override;

  std::unique_ptr<ReportQpsScenarioService::Stub> stub_;
};

}  // namespace testing
}  // namespace grpc

#endif  // GRPC_TEST_CPP_QPS_REPORT_H
