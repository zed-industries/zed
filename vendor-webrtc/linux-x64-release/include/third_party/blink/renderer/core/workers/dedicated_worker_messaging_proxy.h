// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_DEDICATED_WORKER_MESSAGING_PROXY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_DEDICATED_WORKER_MESSAGING_PROXY_H_

#include <memory>

#include "base/functional/function_ref.h"
#include "base/memory/scoped_refptr.h"
#include "services/network/public/mojom/referrer_policy.mojom-blink-forward.h"
#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/public/mojom/frame/back_forward_cache_controller.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/messaging/transferable_message.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/worker/dedicated_worker_host.mojom-blink-forward.h"
#include "third_party/blink/public/platform/scheduler/web_scoped_virtual_time_pauser.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/node.h"
#include "third_party/blink/renderer/core/messaging/blink_transferable_message.h"
#include "third_party/blink/renderer/core/messaging/message_port.h"
#include "third_party/blink/renderer/core/workers/custom_event_message.h"
#include "third_party/blink/renderer/core/workers/parent_execution_context_task_runners.h"
#include "third_party/blink/renderer/core/workers/threaded_messaging_proxy_base.h"
#include "third_party/blink/renderer/core/workers/worker_backing_thread_startup_data.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class DedicatedWorker;
class DedicatedWorkerObjectProxy;
class FetchClientSettingsObjectSnapshot;
class WorkerOptions;
struct WorkerMainScriptLoadParameters;

// A proxy class to talk to the DedicatedWorkerGlobalScope on a worker thread
// via the DedicatedWorkerMessagingProxy from the main thread. See class
// comments on ThreadedMessagingProxyBase for the lifetime and thread affinity.
class CORE_EXPORT DedicatedWorkerMessagingProxy
    : public ThreadedMessagingProxyBase {
 public:
  DedicatedWorkerMessagingProxy(ExecutionContext*, DedicatedWorker*);
  // Exposed for testing.
  DedicatedWorkerMessagingProxy(
      ExecutionContext*,
      DedicatedWorker*,
      base::FunctionRef<std::unique_ptr<DedicatedWorkerObjectProxy>(
          DedicatedWorkerMessagingProxy*,
          DedicatedWorker*,
          ParentExecutionContextTaskRunners*)> worker_object_proxy_factory);
  DedicatedWorkerMessagingProxy(const DedicatedWorkerMessagingProxy&) = delete;
  DedicatedWorkerMessagingProxy& operator=(
      const DedicatedWorkerMessagingProxy&) = delete;
  ~DedicatedWorkerMessagingProxy() override;

  // These methods should only be used on the parent context thread.
  void StartWorkerGlobalScope(
      std::unique_ptr<GlobalScopeCreationParams>,
      std::unique_ptr<WorkerMainScriptLoadParameters>
          worker_main_script_load_params,
      const WorkerOptions*,
      const KURL& script_url,
      const FetchClientSettingsObjectSnapshot& outside_settings_object,
      const v8_inspector::V8StackTraceId&,
      const blink::DedicatedWorkerToken& token,
      mojo::PendingRemote<mojom::blink::DedicatedWorkerHost>
          dedicated_worker_host,
      mojo::PendingRemote<mojom::blink::BackForwardCacheControllerHost>
          back_forward_cache_controller_host);
  void PostMessageToWorkerGlobalScope(BlinkTransferableMessage);
  void PostCustomEventToWorkerGlobalScope(
      TaskType task_type,
      CrossThreadFunction<Event*(ScriptState*, CustomEventMessage)>
          event_factory_callback,
      CrossThreadFunction<Event*(ScriptState*)> event_factory_error_callback,
      CustomEventMessage);

  bool HasPendingActivity() const;

  // This is called from DedicatedWorkerObjectProxy when off-the-main-thread
  // worker script fetch is enabled. Otherwise, this is called from
  // DedicatedWorker.
  void DidFailToFetchScript();

  // These methods come from worker context thread via
  // DedicatedWorkerObjectProxy and are called on the parent context thread.
  void DidEvaluateScript(bool success);
  void PostMessageToWorkerObject(BlinkTransferableMessage);
  void DispatchErrorEvent(const String& error_message,
                          std::unique_ptr<SourceLocation>,
                          int exception_id);

  // Freezes the WorkerThread. `is_in_back_forward_cache` is true only when the
  // page goes to back/forward cache.
  void Freeze(bool is_in_back_forward_cache);
  void Resume();

  DedicatedWorkerObjectProxy& WorkerObjectProxy() {
    return *worker_object_proxy_.get();
  }

  void Trace(Visitor*) const override;

 private:
  friend class DedicatedWorkerMessagingProxyForTest;
  struct CustomEventInfo {
    TaskType task_type;
    CustomEventMessage message;
    CrossThreadFunction<Event*(ScriptState*, CustomEventMessage)>
        event_factory_callback;
    CrossThreadFunction<Event*(ScriptState*)> event_factory_error_callback;
  };
  // This struct abstracts MessageEvent (worker.postMessage()) and other events
  // to be dispatched on the worker context. These parameters are mutually
  // exclusive. If the task info is for MessageEvent, only the message should be
  // set. Otherwise, only the custome_event_info should be set.
  struct TaskInfo {
    std::optional<CustomEventInfo> custom_event_info;
    std::optional<BlinkTransferableMessage> message;
  };

  std::optional<WorkerBackingThreadStartupData> CreateBackingThreadStartupData(
      v8::Isolate*);

  std::unique_ptr<WorkerThread> CreateWorkerThread() override;

  std::unique_ptr<DedicatedWorkerObjectProxy> worker_object_proxy_;

  // This must be weak. The base class (i.e., ThreadedMessagingProxyBase) has a
  // strong persistent reference to itself via SelfKeepAlive (see class-level
  // comments on ThreadedMessagingProxyBase.h for details). To cut the
  // persistent reference, this worker object needs to call a cleanup function
  // in its dtor. If this is a strong reference, the dtor is never called
  // because the worker object is reachable from the persistent reference.
  WeakMember<DedicatedWorker> worker_object_;

  // Set once the initial script evaluation has been completed and it's ready
  // to dispatch events (e.g., Message events) on the worker global scope.
  bool was_script_evaluated_ = false;

  // Tasks are queued here until worker scripts are evaluated on the worker
  // global scope.
  Vector<TaskInfo> queued_early_tasks_;

  // Passed to DedicatedWorkerThread on worker thread creation.
  mojo::PendingRemote<mojom::blink::DedicatedWorkerHost>
      pending_dedicated_worker_host_;
  mojo::PendingRemote<mojom::blink::BackForwardCacheControllerHost>
      pending_back_forward_cache_controller_host_;

  // Pauses virtual time in parent context until the worker is initialized.
  WebScopedVirtualTimePauser virtual_time_pauser_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_DEDICATED_WORKER_MESSAGING_PROXY_H_
