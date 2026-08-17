//
//
// Copyright 2018 gRPC authors.
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

#ifndef GRPC_SRC_CPP_SERVER_LOAD_REPORTER_LOAD_REPORTER_ASYNC_SERVICE_IMPL_H
#define GRPC_SRC_CPP_SERVER_LOAD_REPORTER_LOAD_REPORTER_ASYNC_SERVICE_IMPL_H

#include <grpc/support/port_platform.h>
#include <grpcpp/alarm.h>
#include <grpcpp/grpcpp.h>
#include <grpcpp/support/async_stream.h>
#include <grpcpp/support/interceptor.h>
#include <stdint.h>

#include <atomic>
#include <functional>
#include <memory>
#include <string>
#include <utility>

#include "absl/log/check.h"
#include "src/core/util/sync.h"
#include "src/core/util/thd.h"
#include "src/cpp/server/load_reporter/load_reporter.h"
#include "src/proto/grpc/lb/v1/load_reporter.grpc.pb.h"

namespace grpc {
namespace load_reporter {

// Async load reporting service. It's mainly responsible for controlling the
// procedure of incoming requests. The real business logic is handed off to the
// LoadReporter. There should be at most one instance of this service on a
// server to avoid spreading the load data into multiple places.
class LoadReporterAsyncServiceImpl
    : public grpc::lb::v1::LoadReporter::AsyncService {
 public:
  explicit LoadReporterAsyncServiceImpl(
      std::unique_ptr<ServerCompletionQueue> cq);
  ~LoadReporterAsyncServiceImpl() override;

  // Starts the working thread.
  void StartThread();

  // Not copyable nor movable.
  LoadReporterAsyncServiceImpl(const LoadReporterAsyncServiceImpl&) = delete;
  LoadReporterAsyncServiceImpl& operator=(const LoadReporterAsyncServiceImpl&) =
      delete;

 private:
  class ReportLoadHandler;

  // A tag that can be called with a bool argument. It's tailored for
  // ReportLoadHandler's use. Before being used, it should be constructed with a
  // method of ReportLoadHandler and a shared pointer to the handler. The
  // shared pointer will be moved to the invoked function and the function can
  // only be invoked once. That makes ref counting of the handler easier,
  // because the shared pointer is not bound to the function and can be gone
  // once the invoked function returns (if not used any more).
  class CallableTag {
   public:
    using HandlerFunction =
        std::function<void(std::shared_ptr<ReportLoadHandler>, bool)>;

    CallableTag() {}

    CallableTag(HandlerFunction func,
                std::shared_ptr<ReportLoadHandler> handler)
        : handler_function_(std::move(func)), handler_(std::move(handler)) {
      CHECK(handler_function_ != nullptr);
      CHECK_NE(handler_, nullptr);
    }

    // Runs the tag. This should be called only once. The handler is no longer
    // owned by this tag after this method is invoked.
    void Run(bool ok);

    // Releases and returns the shared pointer to the handler.
    std::shared_ptr<ReportLoadHandler> ReleaseHandler() {
      return std::move(handler_);
    }

   private:
    HandlerFunction handler_function_ = nullptr;
    std::shared_ptr<ReportLoadHandler> handler_;
  };

  // Each handler takes care of one load reporting stream. It contains
  // per-stream data and it will access the members of the parent class (i.e.,
  // LoadReporterAsyncServiceImpl) for service-wide data (e.g., the load data).
  class ReportLoadHandler {
   public:
    // Instantiates a ReportLoadHandler and requests the next load reporting
    // call. The handler object will manage its own lifetime, so no action is
    // needed from the caller any more regarding that object.
    static void CreateAndStart(ServerCompletionQueue* cq,
                               LoadReporterAsyncServiceImpl* service,
                               LoadReporter* load_reporter);

    // This ctor is public because we want to use std::make_shared<> in
    // CreateAndStart(). This ctor shouldn't be used elsewhere.
    ReportLoadHandler(ServerCompletionQueue* cq,
                      LoadReporterAsyncServiceImpl* service,
                      LoadReporter* load_reporter);

   private:
    // After the handler has a call request delivered, it starts reading the
    // initial request. Also, a new handler is spawned so that we can keep
    // servicing future calls.
    void OnRequestDelivered(std::shared_ptr<ReportLoadHandler> self, bool ok);

    // The first Read() is expected to succeed, after which the handler starts
    // sending load reports back to the balancer. The second Read() is
    // expected to fail, which happens when the balancer half-closes the
    // stream to signal that it's no longer interested in the load reports. For
    // the latter case, the handler will then close the stream.
    void OnReadDone(std::shared_ptr<ReportLoadHandler> self, bool ok);

    // The report sending operations are sequential as: send report -> send
    // done, schedule the next send -> waiting for the alarm to fire -> alarm
    // fires, send report -> ...
    void SendReport(std::shared_ptr<ReportLoadHandler> self, bool ok);
    void ScheduleNextReport(std::shared_ptr<ReportLoadHandler> self, bool ok);

    // Called when Finish() is done.
    void OnFinishDone(std::shared_ptr<ReportLoadHandler> self, bool ok);

    // Called when AsyncNotifyWhenDone() notifies us.
    void OnDoneNotified(std::shared_ptr<ReportLoadHandler> self, bool ok);

    void Shutdown(std::shared_ptr<ReportLoadHandler> self, const char* reason);

    // The key fields of the stream.
    std::string lb_id_;
    std::string load_balanced_hostname_;
    std::string load_key_;
    uint64_t load_report_interval_ms_;

    // The data for RPC communication with the load reportee.
    ServerContext ctx_;
    grpc::lb::v1::LoadReportRequest request_;

    // The members passed down from LoadReporterAsyncServiceImpl.
    ServerCompletionQueue* cq_;
    LoadReporterAsyncServiceImpl* service_;
    LoadReporter* load_reporter_;
    ServerAsyncReaderWriter<grpc::lb::v1::LoadReportResponse,
                            grpc::lb::v1::LoadReportRequest>
        stream_;

    // The status of the RPC progress.
    enum CallStatus {
      WAITING_FOR_DELIVERY,
      DELIVERED,
      INITIAL_REQUEST_RECEIVED,
      INITIAL_RESPONSE_SENT,
      FINISH_CALLED
    } call_status_;
    bool shutdown_{false};
    bool done_notified_{false};
    bool is_cancelled_{false};
    CallableTag on_done_notified_;
    CallableTag on_finish_done_;
    CallableTag next_inbound_;
    CallableTag next_outbound_;
    std::unique_ptr<Alarm> next_report_alarm_;
  };

  // Handles the incoming requests and drives the completion queue in a loop.
  static void Work(void* arg);

  // Schedules the next data fetching from Census and LB feedback sampling.
  void ScheduleNextFetchAndSample();

  // Fetches data from Census and samples LB feedback.
  void FetchAndSample(bool ok);

  std::unique_ptr<ServerCompletionQueue> cq_;
  // To synchronize the operations related to shutdown state of cq_, so that we
  // don't enqueue new tags into cq_ after it is already shut down.
  grpc_core::Mutex cq_shutdown_mu_;
  std::atomic_bool shutdown_{false};
  std::unique_ptr<grpc_core::Thread> thread_;
  std::unique_ptr<LoadReporter> load_reporter_;
  std::unique_ptr<Alarm> next_fetch_and_sample_alarm_;
};

}  // namespace load_reporter
}  // namespace grpc

#endif  // GRPC_SRC_CPP_SERVER_LOAD_REPORTER_LOAD_REPORTER_ASYNC_SERVICE_IMPL_H
