// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_CUSTOM_LAYOUT_WORKLET_GLOBAL_SCOPE_PROXY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_CUSTOM_LAYOUT_WORKLET_GLOBAL_SCOPE_PROXY_H_

#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/custom/layout_worklet_global_scope.h"
#include "third_party/blink/renderer/core/workers/main_thread_worklet_reporting_proxy.h"
#include "third_party/blink/renderer/core/workers/worklet_global_scope_proxy.h"

namespace blink {

class LocalFrame;
class WorkletModuleResponsesMap;

// A proxy for LayoutWorklet to talk to LayoutWorkletGlobalScope.
class CORE_EXPORT LayoutWorkletGlobalScopeProxy
    : public GarbageCollected<LayoutWorkletGlobalScopeProxy>,
      public WorkletGlobalScopeProxy {

 public:
  static LayoutWorkletGlobalScopeProxy* From(WorkletGlobalScopeProxy*);

  LayoutWorkletGlobalScopeProxy(LocalFrame*,
                                WorkletModuleResponsesMap*,
                                PendingLayoutRegistry*,
                                size_t global_scope_number);
  ~LayoutWorkletGlobalScopeProxy() override = default;

  // Implements WorkletGlobalScopeProxy.
  void FetchAndInvokeScript(
      const KURL& module_url_record,
      network::mojom::CredentialsMode,
      const FetchClientSettingsObjectSnapshot& outside_settings_object,
      WorkerResourceTimingNotifier& outside_resource_timing_notifier,
      scoped_refptr<base::SingleThreadTaskRunner> outside_settings_task_runner,
      WorkletPendingTasks*) override;
  void WorkletObjectDestroyed() override;
  void TerminateWorkletGlobalScope() override;

  CSSLayoutDefinition* FindDefinition(const AtomicString& name);

  LayoutWorkletGlobalScope* global_scope() const { return global_scope_.Get(); }

  void Trace(Visitor*) const override;

 private:
  std::unique_ptr<MainThreadWorkletReportingProxy> reporting_proxy_;
  Member<LayoutWorkletGlobalScope> global_scope_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_CUSTOM_LAYOUT_WORKLET_GLOBAL_SCOPE_PROXY_H_
