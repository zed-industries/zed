// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_SHARED_WORKER_REPORTING_PROXY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_SHARED_WORKER_REPORTING_PROXY_H_

#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/renderer/core/workers/worker_reporting_proxy.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class WebSharedWorkerImpl;

// An implementation of WorkerReportingProxy for SharedWorker. This is created
// and owned by WebSharedWorkerImpl on the main thread, accessed from a worker
// thread, and destroyed on the main thread.
class SharedWorkerReportingProxy final
    : public GarbageCollected<SharedWorkerReportingProxy>,
      public WorkerReportingProxy {
 public:
  explicit SharedWorkerReportingProxy(WebSharedWorkerImpl*);
  SharedWorkerReportingProxy(const SharedWorkerReportingProxy&) = delete;
  SharedWorkerReportingProxy& operator=(const SharedWorkerReportingProxy&) =
      delete;
  ~SharedWorkerReportingProxy() override;

  // WorkerReportingProxy methods:
  void CountFeature(WebFeature) override;
  void ReportException(const WTF::String&,
                       std::unique_ptr<SourceLocation>,
                       int exception_id) override;
  void ReportConsoleMessage(mojom::ConsoleMessageSource,
                            mojom::ConsoleMessageLevel,
                            const String& message,
                            SourceLocation*) override;
  void DidFailToFetchClassicScript() override;
  void DidFailToFetchModuleScript() override;
  void DidEvaluateTopLevelScript(bool success) override;
  void DidCloseWorkerGlobalScope() override;
  void WillDestroyWorkerGlobalScope() override {}
  void DidTerminateWorkerThread() override;

  void Trace(Visitor*) const;

 private:
  // Not owned because this outlives the reporting proxy.
  WebSharedWorkerImpl* worker_;

  scoped_refptr<base::SingleThreadTaskRunner> main_thread_task_runner_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_SHARED_WORKER_REPORTING_PROXY_H_
