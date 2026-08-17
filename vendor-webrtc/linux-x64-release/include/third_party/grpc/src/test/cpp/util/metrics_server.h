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
#ifndef GRPC_TEST_CPP_UTIL_METRICS_SERVER_H
#define GRPC_TEST_CPP_UTIL_METRICS_SERVER_H

#include <grpcpp/server.h>

#include <map>
#include <mutex>

#include "src/proto/grpc/testing/metrics.grpc.pb.h"
#include "src/proto/grpc/testing/metrics.pb.h"

//
// This implements a Metrics server defined in
// src/proto/grpc/testing/metrics.proto. Any
// test service can use this to export Metrics (TODO (sreek): Only Gauges for
// now).
//
// Example:
//    MetricsServiceImpl metricsImpl;
//    ..
//    // Create QpsGauge(s). Note: QpsGauges can be created even after calling
//    // 'StartServer'.
//    QpsGauge qps_gauge1 = metricsImpl.CreateQpsGauge("foo", is_present);
//    // qps_gauge1 can now be used anywhere in the program by first making a
//    // one-time call qps_gauge1.Reset() and then calling qps_gauge1.Incr()
//    // every time to increment a query counter
//
//    ...
//    // Create the metrics server
//    std::unique_ptr<grpc::Server> server = metricsImpl.StartServer(port);
//    server->Wait(); // Note: This is blocking.
//
namespace grpc {
namespace testing {

class QpsGauge {
 public:
  QpsGauge();

  // Initialize the internal timer and reset the query count to 0
  void Reset();

  // Increment the query count by 1
  void Incr();

  // Return the current qps (i.e query count divided by the time since this
  // QpsGauge object created (or Reset() was called))
  long Get();

 private:
  gpr_timespec start_time_;
  long num_queries_;
  std::mutex num_queries_mu_;
};

class MetricsServiceImpl final : public MetricsService::Service {
 public:
  grpc::Status GetAllGauges(ServerContext* context, const EmptyMessage* request,
                            ServerWriter<GaugeResponse>* writer) override;

  grpc::Status GetGauge(ServerContext* context, const GaugeRequest* request,
                        GaugeResponse* response) override;

  // Create a QpsGauge with name 'name'. is_present is set to true if the Gauge
  // is already present in the map.
  // NOTE: CreateQpsGauge can be called anytime (i.e before or after calling
  // StartServer).
  std::shared_ptr<QpsGauge> CreateQpsGauge(const std::string& name,
                                           bool* already_present);

  std::unique_ptr<grpc::Server> StartServer(int port);

 private:
  std::map<string, std::shared_ptr<QpsGauge>> qps_gauges_;
  std::mutex mu_;
};

}  // namespace testing
}  // namespace grpc

#endif  // GRPC_TEST_CPP_UTIL_METRICS_SERVER_H
