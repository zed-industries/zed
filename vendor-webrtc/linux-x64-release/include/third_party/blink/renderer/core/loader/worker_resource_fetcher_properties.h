// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_WORKER_RESOURCE_FETCHER_PROPERTIES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_WORKER_RESOURCE_FETCHER_PROPERTIES_H_

#include "base/check_op.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_fetcher_properties.h"

namespace blink {

class WebWorkerFetchContext;
class WorkerOrWorkletGlobalScope;

// WorkerResourceFetcherProperties is a ResourceFetcherProperties implementation
// for workers and worklets.
class WorkerResourceFetcherProperties final : public ResourceFetcherProperties {
 public:
  WorkerResourceFetcherProperties(
      WorkerOrWorkletGlobalScope&,
      const FetchClientSettingsObject& fetch_client_settings_object,
      scoped_refptr<WebWorkerFetchContext> web_context);
  ~WorkerResourceFetcherProperties() override = default;

  void Trace(Visitor* visitor) const override;

  // ResourceFetcherProperties implementation
  const FetchClientSettingsObject& GetFetchClientSettingsObject()
      const override {
    return *fetch_client_settings_object_;
  }
  bool IsOutermostMainFrame() const override { return false; }
  ControllerServiceWorkerMode GetControllerServiceWorkerMode() const override;
  int64_t ServiceWorkerId() const override {
    DCHECK_NE(GetControllerServiceWorkerMode(),
              mojom::ControllerServiceWorkerMode::kNoController);
    // Currently ServiceWorkerId is used only with MemoryCache which is disabled
    // on a non-main thread. Hence this value doesn't matter.
    // TODO(nhiroki): Return the valid service worker ID and make this function
    // available also on a non-main thread.
    return -1;
  }
  bool IsPaused() const override;
  LoaderFreezeMode FreezeMode() const override;
  bool IsDetached() const override { return false; }
  bool IsLoadComplete() const override { return false; }
  bool ShouldBlockLoadingSubResource() const override { return false; }
  bool IsSubframeDeprioritizationEnabled() const override { return false; }
  scheduler::FrameStatus GetFrameStatus() const override {
    return scheduler::FrameStatus::kNone;
  }
  int GetOutstandingThrottledLimit() const override;

 private:
  const Member<WorkerOrWorkletGlobalScope> global_scope_;
  const Member<const FetchClientSettingsObject> fetch_client_settings_object_;
  const scoped_refptr<WebWorkerFetchContext> web_context_;
  const int outstanding_throttled_limit_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_WORKER_RESOURCE_FETCHER_PROPERTIES_H_
