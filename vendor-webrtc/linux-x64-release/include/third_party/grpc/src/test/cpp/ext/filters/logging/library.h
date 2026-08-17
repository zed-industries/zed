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

#ifndef GRPC_TEST_CPP_EXT_FILTERS_LOGGING_LIBRARY_H
#define GRPC_TEST_CPP_EXT_FILTERS_LOGGING_LIBRARY_H

#include <grpc++/grpc++.h>
#include <grpcpp/opencensus.h>
#include <grpcpp/support/status.h>

#include <chrono>
#include <thread>  // NOLINT

#include "absl/log/log.h"
#include "absl/strings/escaping.h"
#include "absl/strings/str_cat.h"
#include "absl/time/time.h"
#include "gmock/gmock.h"
#include "google/protobuf/text_format.h"
#include "gtest/gtest.h"
#include "src/core/ext/filters/logging/logging_filter.h"
#include "src/core/util/sync.h"
#include "src/cpp/ext/gcp/observability_logging_sink.h"
#include "src/proto/grpc/testing/echo.grpc.pb.h"
#include "src/proto/grpc/testing/echo_messages.pb.h"
#include "test/core/test_util/port.h"
#include "test/core/test_util/test_config.h"
#include "test/cpp/end2end/test_service_impl.h"

namespace grpc {
namespace testing {

class TestLoggingSink : public grpc_core::LoggingSink {
 public:
  Config FindMatch(bool /* is_client */, absl::string_view /* service */,
                   absl::string_view /* method */) override {
    grpc_core::MutexLock lock(&mu_);
    return config_;
  }

  void LogEntry(Entry entry) override {
    ::google::protobuf::Struct json;
    grpc::internal::EntryToJsonStructProto(entry, &json);
    std::string output;
    ::google::protobuf::TextFormat::PrintToString(json, &output);
    LOG(INFO) << output;
    LOG(INFO) << "trace_id: " << entry.trace_id;
    LOG(INFO) << "span_id: " << entry.span_id;
    LOG(INFO) << "is_sampled: " << entry.is_sampled;
    grpc_core::MutexLock lock(&mu_);
    entries_.push_back(std::move(entry));
    cv_.SignalAll();
  }

  void SetConfig(Config config) {
    grpc_core::MutexLock lock(&mu_);
    config_ = config;
  }

  std::vector<LoggingSink::Entry> entries() {
    grpc_core::MutexLock lock(&mu_);
    return entries_;
  }

  void Clear() {
    grpc_core::MutexLock lock(&mu_);
    entries_.clear();
  }

  // Waits for \a duration till we have \a num_entries in the log.
  bool WaitForNumEntries(size_t num_entries, absl::Duration duration) {
    absl::Time deadline = absl::Now() + duration * grpc_test_slowdown_factor();
    grpc_core::MutexLock lock(&mu_);
    while (entries_.size() != num_entries) {
      if (cv_.WaitWithDeadline(&mu_, deadline)) {
        LOG(ERROR) << "\nDeadline expired while waiting on logging "
                      "entries\nExpected number: "
                   << num_entries << "\nActual number: " << entries_.size();
        return false;
      }
    }
    return true;
  }

 private:
  grpc_core::Mutex mu_;
  std::vector<LoggingSink::Entry> entries_ ABSL_GUARDED_BY(mu_);
  Config config_ ABSL_GUARDED_BY(mu_);
  grpc_core::CondVar cv_;  // Signalled each time an entry is logged
};

extern TestLoggingSink* g_test_logging_sink;

class LoggingTest : public ::testing::Test {
 protected:
  static void SetUpTestSuite() {
    g_test_logging_sink = new TestLoggingSink;
    grpc_core::RegisterLoggingFilter(g_test_logging_sink);
  }

  void SetUp() override {
    // Clean up previous entries
    g_test_logging_sink->Clear();
    // Set up a synchronous server on a different thread to avoid the asynch
    // interface.
    grpc::ServerBuilder builder;
    int port = grpc_pick_unused_port_or_die();
    server_address_ = absl::StrCat("localhost:", port);
    // Use IPv4 here because it's less flaky than IPv6 ("[::]:0") on Travis.
    builder.AddListeningPort(server_address_, grpc::InsecureServerCredentials(),
                             &port);
    builder.RegisterService(&service_);
    server_ = builder.BuildAndStart();
    ASSERT_NE(nullptr, server_);

    server_thread_ = std::thread(&LoggingTest::RunServerLoop, this);

    stub_ = EchoTestService::NewStub(grpc::CreateChannel(
        server_address_, grpc::InsecureChannelCredentials()));
  }

  void ResetStub(std::shared_ptr<Channel> channel) {
    stub_ = EchoTestService::NewStub(std::move(channel));
  }

  void TearDown() override {
    server_->Shutdown();
    server_thread_.join();
  }

  void RunServerLoop() { server_->Wait(); }

  std::string server_address_;
  CallbackTestServiceImpl service_;
  std::unique_ptr<grpc::Server> server_;
  std::thread server_thread_;

  std::unique_ptr<EchoTestService::Stub> stub_;
};

}  // namespace testing
}  // namespace grpc

#endif  // GRPC_TEST_CPP_EXT_FILTERS_LOGGING_LIBRARY_H
