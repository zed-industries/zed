// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_THREADED_OBJECT_PROXY_BASE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_THREADED_OBJECT_PROXY_BASE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/messaging/message_port.h"
#include "third_party/blink/renderer/core/workers/worker_reporting_proxy.h"
#include "third_party/blink/renderer/platform/bindings/source_location.h"
#include "third_party/blink/renderer/platform/heap/cross_thread_persistent.h"

namespace blink {

class ParentExecutionContextTaskRunners;
class ThreadedMessagingProxyBase;

// The base proxy class to talk to a DedicatedWorker or *Worklet object on the
// main thread via the ThreadedMessagingProxyBase from a worker thread. This is
// created and destroyed on the main thread, and used on the worker thread.
// ThreadedMessagingProxyBase always outlives this proxy.
class CORE_EXPORT ThreadedObjectProxyBase : public WorkerReportingProxy {
  USING_FAST_MALLOC(ThreadedObjectProxyBase);
 public:
  ThreadedObjectProxyBase(const ThreadedObjectProxyBase&) = delete;
  ThreadedObjectProxyBase& operator=(const ThreadedObjectProxyBase&) = delete;
  ~ThreadedObjectProxyBase() override = default;

  void ReportPendingActivity(bool has_pending_activity);

  // WorkerReportingProxy overrides.
  void CountFeature(WebFeature) override;
  void CountWebDXFeature(WebDXFeature) override;
  void ReportConsoleMessage(mojom::ConsoleMessageSource,
                            mojom::ConsoleMessageLevel,
                            const String& message,
                            SourceLocation*) override;
  void DidCloseWorkerGlobalScope() override;
  void DidTerminateWorkerThread() override;

 protected:
  explicit ThreadedObjectProxyBase(ParentExecutionContextTaskRunners*,
                                   scoped_refptr<base::SingleThreadTaskRunner>
                                       parent_agent_group_task_runner);
  virtual CrossThreadWeakPersistent<ThreadedMessagingProxyBase>
  MessagingProxyWeakPtr() = 0;
  ParentExecutionContextTaskRunners* GetParentExecutionContextTaskRunners();
  scoped_refptr<base::SingleThreadTaskRunner> GetParentAgentGroupTaskRunner();

 private:
  // Note: Only one of `parent_execution_context_task_runners_` and
  // `parent_agent_group_task_runner_` will be set.

  // Used to post a task to ThreadedMessagingProxyBase on the parent context
  // thread.
  CrossThreadPersistent<ParentExecutionContextTaskRunners>
      parent_execution_context_task_runners_;

  // Used to post a task to ThreadedMessagingProxyBase on the parent agent group
  // thread.
  scoped_refptr<base::SingleThreadTaskRunner> parent_agent_group_task_runner_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_THREADED_OBJECT_PROXY_BASE_H_
