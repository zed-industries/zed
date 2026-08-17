// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SHADOW_REALM_SHADOW_REALM_GLOBAL_SCOPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SHADOW_REALM_SHADOW_REALM_GLOBAL_SCOPE_H_

#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/core/frame/universal_global_scope.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class CORE_EXPORT ShadowRealmGlobalScope final : public EventTarget,
                                                 public UniversalGlobalScope,
                                                 public ExecutionContext {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit ShadowRealmGlobalScope(
      ExecutionContext* initiator_execution_context);

  // Get the root execution context where the outermost shadow realm was
  // initialized.
  ExecutionContext* GetRootInitiatorExecutionContext() const;

  void Trace(Visitor* visitor) const override;

  // EventTarget:
  const AtomicString& InterfaceName() const override;
  ExecutionContext* GetExecutionContext() const override;

  // MojoBindingContext:
  const BrowserInterfaceBrokerProxy& GetBrowserInterfaceBroker() const override;
  scoped_refptr<base::SingleThreadTaskRunner> GetTaskRunner(
      TaskType task_type) override;

  // UseCounter:
  void CountUse(mojom::blink::WebFeature feature) override;
  void CountDeprecation(mojom::blink::WebFeature feature) override;
  void CountWebDXFeature(mojom::blink::WebDXFeature feature) override;

  // ExecutionContext:
  bool IsShadowRealmGlobalScope() const override;
  const KURL& Url() const override;
  const KURL& BaseURL() const override;
  KURL CompleteURL(const String& url) const override;
  void DisableEval(const String& error_message) override;
  void SetWasmEvalErrorMessage(const String& error_message) override;
  String UserAgent() const override;
  HttpsState GetHttpsState() const override;
  ResourceFetcher* Fetcher() override;
  void ExceptionThrown(ErrorEvent* error_event) override;
  void AddInspectorIssue(AuditsIssue issue) override;
  EventTarget* ErrorEventTarget() override;
  FrameOrWorkerScheduler* GetScheduler() override;
  bool CrossOriginIsolatedCapability() const override;
  bool IsIsolatedContext() const override;
  ukm::UkmRecorder* UkmRecorder() override;
  ukm::SourceId UkmSourceID() const override;
  ExecutionContextToken GetExecutionContextToken() const override;
  bool IsSecureContext() const override;

 private:
  void AddConsoleMessageImpl(ConsoleMessage* message,
                             bool discard_duplicates) override;

  const Member<ExecutionContext> initiator_execution_context_;
  KURL url_;
  ShadowRealmToken token_;
};

template <>
struct DowncastTraits<ShadowRealmGlobalScope> {
  static bool AllowFrom(const ExecutionContext& context) {
    return context.IsShadowRealmGlobalScope();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SHADOW_REALM_SHADOW_REALM_GLOBAL_SCOPE_H_
