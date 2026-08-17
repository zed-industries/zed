// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_PAINT_WORKLET_GLOBAL_SCOPE_PROXY_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_PAINT_WORKLET_GLOBAL_SCOPE_PROXY_H_

#include "third_party/blink/renderer/core/workers/worklet_global_scope_proxy.h"

#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/renderer/core/workers/main_thread_worklet_reporting_proxy.h"
#include "third_party/blink/renderer/modules/csspaint/paint_worklet_global_scope.h"
#include "third_party/blink/renderer/modules/modules_export.h"

namespace blink {

class CSSPaintDefinition;
class LocalFrame;
class WorkletModuleResponsesMap;

// A proxy for PaintWorklet to talk to PaintWorkletGlobalScope.
class MODULES_EXPORT PaintWorkletGlobalScopeProxy
    : public GarbageCollected<PaintWorkletGlobalScopeProxy>,
      public WorkletGlobalScopeProxy {
 public:
  static PaintWorkletGlobalScopeProxy* From(WorkletGlobalScopeProxy*);

  PaintWorkletGlobalScopeProxy(LocalFrame*,
                               WorkletModuleResponsesMap*,
                               size_t global_scope_number);
  ~PaintWorkletGlobalScopeProxy() override = default;

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

  CSSPaintDefinition* FindDefinition(const String& name);

  PaintWorkletGlobalScope* global_scope() const { return global_scope_.Get(); }

  void Trace(Visitor*) const override;

 private:
  std::unique_ptr<MainThreadWorkletReportingProxy> reporting_proxy_;
  Member<PaintWorkletGlobalScope> global_scope_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_PAINT_WORKLET_GLOBAL_SCOPE_PROXY_H_
