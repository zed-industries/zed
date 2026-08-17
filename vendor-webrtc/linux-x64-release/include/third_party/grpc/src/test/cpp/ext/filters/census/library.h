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

#ifndef GRPC_TEST_CPP_EXT_FILTERS_CENSUS_LIBRARY_H
#define GRPC_TEST_CPP_EXT_FILTERS_CENSUS_LIBRARY_H

#include <grpc++/grpc++.h>
#include <grpcpp/opencensus.h>

#include <string>
#include <thread>  // NOLINT
#include <vector>

#include "absl/strings/str_cat.h"
#include "gtest/gtest.h"
#include "opencensus/stats/stats.h"
#include "opencensus/trace/exporter/span_exporter.h"
#include "src/core/config/core_configuration.h"
#include "src/cpp/client/client_stats_interceptor.h"
#include "src/cpp/ext/filters/census/client_filter.h"
#include "src/cpp/ext/filters/census/context.h"
#include "src/proto/grpc/testing/echo.grpc.pb.h"
#include "test/core/test_util/test_lb_policies.h"
#include "test/cpp/end2end/test_service_impl.h"

namespace opencensus {
namespace trace {
namespace exporter {
class SpanExporterTestPeer {
 public:
  static constexpr auto& ExportForTesting = SpanExporter::ExportForTesting;
};
}  // namespace exporter
}  // namespace trace
}  // namespace opencensus

namespace grpc {
namespace testing {

extern const ::opencensus::tags::TagKey TEST_TAG_KEY;
extern const char* TEST_TAG_VALUE;
extern const char* kExpectedTraceIdKey;

class EchoServer final : public TestServiceImpl {
  Status Echo(ServerContext* context, const EchoRequest* request,
              EchoResponse* response) override {
    CheckMetadata(context);
    // Enabled for compression trace annotation tests.
    context->set_compression_algorithm(GRPC_COMPRESS_GZIP);
    return TestServiceImpl::Echo(context, request, response);
  }

  Status BidiStream(
      ServerContext* context,
      ServerReaderWriter<EchoResponse, EchoRequest>* stream) override {
    CheckMetadata(context);
    return TestServiceImpl::BidiStream(context, stream);
  }

 private:
  void CheckMetadata(ServerContext* context) {
    for (const auto& metadata : context->client_metadata()) {
      if (metadata.first == kExpectedTraceIdKey) {
        EXPECT_EQ(metadata.second, reinterpret_cast<const CensusContext*>(
                                       context->census_context())
                                       ->Span()
                                       .context()
                                       .trace_id()
                                       .ToHex());
        break;
      }
    }
  }
};

// A handler that records exported traces. Traces can later be retrieved and
// inspected.
class ExportedTracesRecorder
    : public ::opencensus::trace::exporter::SpanExporter::Handler {
 public:
  ExportedTracesRecorder() : is_recording_(false) {}
  void Export(const std::vector<::opencensus::trace::exporter::SpanData>& spans)
      override {
    grpc_core::MutexLock lock(&mutex_);
    if (is_recording_) {
      for (auto const& span : spans) {
        recorded_spans_.push_back(span);
      }
    }
  }

  void StartRecording() {
    grpc_core::MutexLock lock(&mutex_);
    ASSERT_FALSE(is_recording_);
    is_recording_ = true;
  }

  void StopRecording() {
    grpc_core::MutexLock lock(&mutex_);
    ASSERT_TRUE(is_recording_);
    is_recording_ = false;
  }

  std::vector<::opencensus::trace::exporter::SpanData> GetAndClearSpans() {
    grpc_core::MutexLock lock(&mutex_);
    return std::move(recorded_spans_);
  }

 private:
  // This mutex is necessary as the SpanExporter runs a loop on a separate
  // thread which periodically exports spans.
  grpc_core::Mutex mutex_;
  bool is_recording_ ABSL_GUARDED_BY(mutex_);
  std::vector<::opencensus::trace::exporter::SpanData> recorded_spans_
      ABSL_GUARDED_BY(mutex_);
};

extern ExportedTracesRecorder* traces_recorder_;

class StatsPluginEnd2EndTest : public ::testing::Test {
 protected:
  static void SetUpTestSuite() {
    grpc_core::CoreConfiguration::Reset();
    grpc_core::CoreConfiguration::RegisterEphemeralBuilder(
        [](grpc_core::CoreConfiguration::Builder* builder) {
          grpc_core::RegisterQueueOnceLoadBalancingPolicy(builder);
        });
    grpc::internal::RegisterGlobalClientStatsInterceptorFactory(
        new grpc::internal::OpenCensusClientInterceptorFactory);
    RegisterOpenCensusPlugin();
    // OpenCensus C++ has no API to unregister a previously-registered handler,
    // therefore we register this handler once, and enable/disable recording in
    // the individual tests.
    ::opencensus::trace::exporter::SpanExporter::RegisterHandler(
        absl::WrapUnique(traces_recorder_));
    grpc_init();
  }

  static void TearDownTestSuite() {
    grpc_shutdown();
    grpc_core::CoreConfiguration::Reset();
  }

  void SetUp() override {
    // Set up a synchronous server on a different thread to avoid the asynch
    // interface.
    grpc::ServerBuilder builder;
    int port;
    // Use IPv4 here because it's less flaky than IPv6 ("[::]:0") on Travis.
    builder.AddListeningPort("0.0.0.0:0", grpc::InsecureServerCredentials(),
                             &port);
    builder.RegisterService(&service_);
    server_ = builder.BuildAndStart();
    ASSERT_NE(nullptr, server_);
    ASSERT_NE(0, port);
    server_address_ = absl::StrCat("localhost:", port);
    server_thread_ = std::thread(&StatsPluginEnd2EndTest::RunServerLoop, this);

    stub_ = EchoTestService::NewStub(grpc::CreateChannel(
        server_address_, grpc::InsecureChannelCredentials()));

    // Clear out any previous spans
    ::opencensus::trace::exporter::SpanExporterTestPeer::ExportForTesting();
  }

  void ResetStub(std::shared_ptr<Channel> channel) {
    stub_ = EchoTestService::NewStub(channel);
  }

  void TearDown() override {
    server_->Shutdown();
    server_thread_.join();
  }

  void RunServerLoop() { server_->Wait(); }

  const std::string client_method_name_ = "grpc.testing.EchoTestService/Echo";
  const std::string server_method_name_ = "grpc.testing.EchoTestService/Echo";

  std::string server_address_;
  EchoServer service_;
  std::unique_ptr<grpc::Server> server_;
  std::thread server_thread_;

  std::unique_ptr<EchoTestService::Stub> stub_;
};

}  // namespace testing
}  // namespace grpc

#endif  // GRPC_TEST_CPP_EXT_FILTERS_CENSUS_LIBRARY_H
