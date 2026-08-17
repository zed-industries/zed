// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_NULL_EXECUTION_CONTEXT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_NULL_EXECUTION_CONTEXT_H_

#include <memory>
#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/public/mojom/devtools/inspector_issue.mojom-blink.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/core/inspector/console_message.h"
#include "third_party/blink/renderer/core/inspector/inspector_audits_issue.h"
#include "third_party/blink/renderer/platform/bindings/source_location.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/scheduler/public/frame_scheduler.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"

namespace blink {

class NullExecutionContext : public GarbageCollected<NullExecutionContext>,
                             public ExecutionContext {
 public:
  // Deprecated: Use version that takes an Isolate.
  // TODO(crbug.com/1315595): Remove this constructor.
  NullExecutionContext();

  explicit NullExecutionContext(v8::Isolate* isolate);
  explicit NullExecutionContext(std::unique_ptr<FrameScheduler> scheduler);
  ~NullExecutionContext() override;

  void SetURL(const KURL& url) { url_ = url; }

  const KURL& Url() const override { return url_; }
  const KURL& BaseURL() const override { return url_; }
  KURL CompleteURL(const String&) const override { return url_; }

  void DisableEval(const String&) override {}
  void SetWasmEvalErrorMessage(const String&) override {}
  String UserAgent() const override { return String(); }

  HttpsState GetHttpsState() const override {
    return CalculateHttpsState(GetSecurityOrigin());
  }

  EventTarget* ErrorEventTarget() override { return nullptr; }

  void AddConsoleMessageImpl(ConsoleMessage*,
                             bool discard_duplicates) override {}
  void AddInspectorIssue(AuditsIssue) override {}
  void ExceptionThrown(ErrorEvent*) override {}

  void SetUpSecurityContextForTesting();

  ResourceFetcher* Fetcher() override { return nullptr; }
  bool CrossOriginIsolatedCapability() const override { return false; }
  bool IsIsolatedContext() const override { return false; }
  ukm::UkmRecorder* UkmRecorder() override { return nullptr; }
  ukm::SourceId UkmSourceID() const override { return ukm::kInvalidSourceId; }
  FrameOrWorkerScheduler* GetScheduler() override;
  scoped_refptr<base::SingleThreadTaskRunner> GetTaskRunner(TaskType) override;

  void CountUse(mojom::WebFeature) override {}
  void CountDeprecation(mojom::WebFeature) override {}
  void CountWebDXFeature(mojom::blink::WebDXFeature) override {}

  const BrowserInterfaceBrokerProxy& GetBrowserInterfaceBroker() const override;

  ExecutionContextToken GetExecutionContextToken() const final {
    return token_;
  }

 private:
  KURL url_;

  // A dummy scheduler to ensure that the callers of
  // ExecutionContext::GetScheduler don't have to check for whether it's null or
  // not.
  std::unique_ptr<FrameScheduler> scheduler_;

  // A fake token identifying this execution context.
  const LocalFrameToken token_;
};

class ScopedNullExecutionContext {
 public:
  ScopedNullExecutionContext()
      : execution_context_(MakeGarbageCollected<NullExecutionContext>()) {}

  explicit ScopedNullExecutionContext(std::unique_ptr<FrameScheduler> scheduler)
      : execution_context_(
            MakeGarbageCollected<NullExecutionContext>(std::move(scheduler))) {}

  ~ScopedNullExecutionContext() {
    execution_context_->NotifyContextDestroyed();
  }

  NullExecutionContext& GetExecutionContext() const {
    return *execution_context_;
  }

 private:
  Persistent<NullExecutionContext> execution_context_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_NULL_EXECUTION_CONTEXT_H_
