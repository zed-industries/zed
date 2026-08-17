// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_THREADED_MESSAGING_PROXY_BASE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_THREADED_MESSAGING_PROXY_BASE_H_

#include <optional>

#include "base/sequence_checker.h"
#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/public/mojom/devtools/console_message.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/frame/web_feature_forward.h"
#include "third_party/blink/renderer/core/inspector/worker_devtools_params.h"
#include "third_party/blink/renderer/core/workers/parent_execution_context_task_runners.h"
#include "third_party/blink/renderer/core/workers/worker_backing_thread_startup_data.h"
#include "third_party/blink/renderer/core/workers/worker_thread.h"
#include "third_party/blink/renderer/platform/heap/self_keep_alive.h"
#include "third_party/blink/renderer/platform/scheduler/public/frame_or_worker_scheduler.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace base {

class WaitableEvent;

}  // namespace base

namespace blink {

class ExecutionContext;
class SourceLocation;
struct GlobalScopeCreationParams;

// The base proxy class to talk to Worker/WorkletGlobalScope on a worker thread
// from the parent context thread (Note that this is always the main thread for
// now because nested worker is not supported yet). This must be created,
// accessed and destroyed on the parent context thread.
//
// This has a unique lifetime: this is co-owned by the parent object (e.g.,
// DedicatedWorker, AnimationWorklet) and by itself via SelfKeepAlive. The
// parent object releases the reference on its destructor and SelfKeepAlive is
// cleared when the worker thread is terminated.
//
// This co-ownership is necessary because the proxy needs to outlive components
// living on the worker thread (e.g., WorkerGlobalScope) but the parent object
// can be destroyed before the completion of worker thread termination.
class CORE_EXPORT ThreadedMessagingProxyBase
    : public GarbageCollected<ThreadedMessagingProxyBase> {
 public:
  virtual ~ThreadedMessagingProxyBase();

  void TerminateGlobalScope();

  // This method should be called in the destructor of the object which
  // initially created it. This object could either be a Worker or a Worklet.
  // This may cause deletion of this via |keep_alive_|.
  void ParentObjectDestroyed();

  void CountFeature(WebFeature);
  void CountWebDXFeature(mojom::blink::WebDXFeature);

  void ReportConsoleMessage(mojom::ConsoleMessageSource,
                            mojom::ConsoleMessageLevel,
                            const String& message,
                            std::unique_ptr<SourceLocation>);

  virtual void WorkerThreadTerminated();

  // Number of live messaging proxies, used by leak detection.
  static int ProxyCount();

  virtual void Trace(Visitor*) const;

 protected:
  explicit ThreadedMessagingProxyBase(
      ExecutionContext*,
      scoped_refptr<base::SingleThreadTaskRunner>
          parent_agent_group_task_runner = nullptr);

  // Normally, for dedicated worker and for worklets created in-process, the
  // devtools params will be derived within this function, based on the parent
  // (Window) context and/or based on the dedicated worker token. When worklet
  // creation is proxied via the browser process (e.g. shared storage worklet),
  // where the original Window context isn't directly accessible,
  // `client_provided_devtools_params` will be pre-calculated and passed to this
  // function, and this param will be used directly to start the worklet thread.
  void InitializeWorkerThread(
      std::unique_ptr<GlobalScopeCreationParams>,
      const std::optional<WorkerBackingThreadStartupData>&,
      const std::optional<const blink::DedicatedWorkerToken>&,
      std::unique_ptr<WorkerDevToolsParams> client_provided_devtools_params =
          nullptr);

  ExecutionContext* GetExecutionContext() const;
  ParentExecutionContextTaskRunners* GetParentExecutionContextTaskRunners()
      const;

  scoped_refptr<base::SingleThreadTaskRunner> GetParentAgentGroupTaskRunner()
      const;

  // May return nullptr after termination is requested.
  WorkerThread* GetWorkerThread() const;

  bool AskedToTerminate() const { return asked_to_terminate_; }

  // Returns true if this is called on the parent context thread.
  bool IsParentContextThread() const;

 private:
  virtual std::unique_ptr<WorkerThread> CreateWorkerThread() = 0;

  Member<ExecutionContext> execution_context_;

  // Accessed cross-thread when worker thread posts tasks to the parent. Only
  // one of `parent_execution_context_task_runners_` and
  // `parent_agent_group_task_runner_` will be set.
  CrossThreadPersistent<ParentExecutionContextTaskRunners>
      parent_execution_context_task_runners_;
  scoped_refptr<base::SingleThreadTaskRunner> parent_agent_group_task_runner_;

  SEQUENCE_CHECKER(sequence_checker_);
  std::unique_ptr<WorkerThread> worker_thread_;

  bool asked_to_terminate_ = false;

  // Used to terminate the synchronous resource loading (XMLHttpRequest) on the
  // worker thread from the main thread.
  base::WaitableEvent terminate_sync_load_event_;

  // Used to keep this alive until the worker thread gets terminated. This is
  // necessary because the co-owner (i.e., Worker or Worklet object) can be
  // destroyed before thread termination.
  SelfKeepAlive<ThreadedMessagingProxyBase> keep_alive_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_THREADED_MESSAGING_PROXY_BASE_H_
