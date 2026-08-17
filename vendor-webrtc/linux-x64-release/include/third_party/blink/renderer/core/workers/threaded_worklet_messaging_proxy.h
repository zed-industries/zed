// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_THREADED_WORKLET_MESSAGING_PROXY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_THREADED_WORKLET_MESSAGING_PROXY_H_

#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/public/mojom/worker/worklet_global_scope_creation_params.mojom-blink.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/workers/threaded_messaging_proxy_base.h"
#include "third_party/blink/renderer/core/workers/threaded_worklet_object_proxy.h"
#include "third_party/blink/renderer/core/workers/worklet_global_scope_proxy.h"

namespace blink {

class WorkerClients;
class WorkletModuleResponsesMap;

class CORE_EXPORT ThreadedWorkletMessagingProxy
    : public ThreadedMessagingProxyBase,
      public WorkletGlobalScopeProxy {
 public:
  // WorkletGlobalScopeProxy implementation.
  void FetchAndInvokeScript(
      const KURL& module_url_record,
      network::mojom::CredentialsMode,
      const FetchClientSettingsObjectSnapshot& outside_settings_object,
      WorkerResourceTimingNotifier& outside_resource_timing_notifier,
      scoped_refptr<base::SingleThreadTaskRunner> outside_settings_task_runner,
      WorkletPendingTasks*) final;
  void WorkletObjectDestroyed() final;
  void TerminateWorkletGlobalScope() final;

  // Normally, for worklets created in-process, the worklet thread will be
  // initialized with state taking from the original Window context. When
  // worklet creation is proxied via the browser process (e.g. shared storage
  // worklet), where the original Window context isn't directly accessible, the
  // worklet thread will be initialized with state taking from
  // `client_provided_global_scope_creation_params`.
  void Initialize(
      WorkerClients*,
      WorkletModuleResponsesMap*,
      const std::optional<WorkerBackingThreadStartupData>& = std::nullopt,
      mojom::blink::WorkletGlobalScopeCreationParamsPtr
          client_provided_global_scope_creation_params = {});

  void Trace(Visitor*) const override;

 protected:
  explicit ThreadedWorkletMessagingProxy(
      ExecutionContext*,
      scoped_refptr<base::SingleThreadTaskRunner>
          parent_agent_group_task_runner = nullptr);

  ThreadedWorkletObjectProxy& WorkletObjectProxy();

 private:
  friend class ThreadedWorkletMessagingProxyForTest;

  virtual std::unique_ptr<ThreadedWorkletObjectProxy> CreateObjectProxy(
      ThreadedWorkletMessagingProxy*,
      ParentExecutionContextTaskRunners*,
      scoped_refptr<base::SingleThreadTaskRunner>
          parent_agent_group_task_runner);

  std::unique_ptr<ThreadedWorkletObjectProxy> worklet_object_proxy_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_THREADED_WORKLET_MESSAGING_PROXY_H_
