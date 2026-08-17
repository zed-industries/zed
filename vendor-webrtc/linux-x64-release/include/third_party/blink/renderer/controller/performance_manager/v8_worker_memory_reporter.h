// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CONTROLLER_PERFORMANCE_MANAGER_V8_WORKER_MEMORY_REPORTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CONTROLLER_PERFORMANCE_MANAGER_V8_WORKER_MEMORY_REPORTER_H_

#include "base/functional/callback.h"
#include "base/gtest_prod_util.h"
#include "base/memory/weak_ptr.h"
#include "base/time/time.h"
#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/renderer/controller/controller_export.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "v8/include/v8.h"

namespace base {
class SingleThreadTaskRunner;
}

namespace blink {

class WorkerThread;

// This class measures memory usage of all workers in the renderer process.
// Memory measurement is performed by posting a task on each worker thread
// and then combining the results. Unresponsive workers (workers that never
// go back to the message loop to run tasks) are skipped after a timeout.
// The entry point is `GetMemoryUsage` that internally works as follows:
// 1. Create an instance of V8WorkerMemoryReporter.
// 2. Post a memory measurement task on each worker thread using
//    WorkerThread::CallOnAllWorkerThreads. Each task has a weak reference
//    to V8WorkerMemoryReporter.
// 3. Post a timeout task on the main thread and transfer the ownership of
//    V8WorkerMemoryReporter to the task.
// 4. Each worker task starts memory measurement using V8's MeasureMemory.
// 5. Once a measurement succeeds (or fails) a task is posted on the main
//    thread that records the result.
// 6. Once results from all workers are collected, the callback is invoked.
// 7. If there is an unresponsive worker, then the timeout task will eventually
//    run and invoke the callback with partial results.
class CONTROLLER_EXPORT V8WorkerMemoryReporter {
 public:
  struct WorkerMemoryUsage {
    WorkerToken token;
    size_t bytes;
    // TODO(906991): Remove this once PlzDedicatedWorker ships. Until then
    // the browser does not know URLs of dedicated workers, so we pass them
    // together with the measurement result.
    // URLs longer than kMaxReportedUrlLength are skipped. In such a case
    // url.IsNull() returns true.
    KURL url;
  };

  struct Result {
    Vector<WorkerMemoryUsage> workers;
  };

  using ResultCallback = base::OnceCallback<void(const Result&)>;

  // This function should be called on the main thread. Upon completion
  // the given callback is also invoked on the main thread.
  static void GetMemoryUsage(ResultCallback callback,
                             v8::MeasureMemoryExecution mode);

  // These functions are called by WorkerMeasurementDelegate on a worker thread.
  static void NotifyMeasurementSuccess(
      WorkerThread*,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner,
      base::WeakPtr<V8WorkerMemoryReporter>,
      std::unique_ptr<WorkerMemoryUsage> memory_usage);
  static void NotifyMeasurementFailure(
      WorkerThread*,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner,
      base::WeakPtr<V8WorkerMemoryReporter>);

 private:
  // The initial state is kWaiting.
  // Transition from kWaiting to kDone happens on two events:
  // A. Responses from all workers are collected. A response is either a
  //    successful measurement or a failure.
  // B. The timeout task runs.
  // Upon transition to kDone, the callback is invoked with the results.
  // All operations become no-ops in the kDone state.
  enum class State { kWaiting, kDone };
  // V8's MeasureMemory API may take 0-30 seconds depending on the GC schedule.
  static const base::TimeDelta kTimeout;

  explicit V8WorkerMemoryReporter(ResultCallback callback)
      : callback_(std::move(callback)) {}

  // This function runs on a worker thread.
  static void StartMeasurement(
      WorkerThread*,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner,
      base::WeakPtr<V8WorkerMemoryReporter>,
      v8::MeasureMemoryExecution);

  // Functions that run on the main thread.
  void OnTimeout();
  void OnMeasurementFailure();
  void OnMeasurementSuccess(std::unique_ptr<WorkerMemoryUsage> memory_usage);
  void InvokeCallback();
  base::WeakPtr<V8WorkerMemoryReporter> GetWeakPtr() {
    return weak_factory_.GetWeakPtr();
  }
  void SetWorkerCount(unsigned worker_count);

  Result result_;
  ResultCallback callback_;
  State state_ = State::kWaiting;
  unsigned worker_count_ = 0;
  unsigned success_count_ = 0;
  unsigned failure_count_ = 0;
  base::WeakPtrFactory<V8WorkerMemoryReporter> weak_factory_{this};

  FRIEND_TEST_ALL_PREFIXES(V8WorkerMemoryReporterTest, OnMeasurementSuccess);
  FRIEND_TEST_ALL_PREFIXES(V8WorkerMemoryReporterTest, OnMeasurementFailure);
  FRIEND_TEST_ALL_PREFIXES(V8WorkerMemoryReporterTest, OnTimeout);
  FRIEND_TEST_ALL_PREFIXES(V8WorkerMemoryReporterTest, OnTimeoutNoop);
  FRIEND_TEST_ALL_PREFIXES(V8WorkerMemoryReporterTest, GetMemoryUsage);
  FRIEND_TEST_ALL_PREFIXES(V8WorkerMemoryReporterTestWithMockPlatform,
                           GetMemoryUsageTimeout);
};

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_CONTROLLER_PERFORMANCE_MANAGER_V8_WORKER_MEMORY_REPORTER_H_
