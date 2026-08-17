// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_NULL_RESOURCE_FETCHER_PROPERTIES_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_NULL_RESOURCE_FETCHER_PROPERTIES_H_

#include "base/notreached.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_fetcher_properties.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

// NullResourceFetcherProperties is a ResourceFetcherProperties implementation
// which returns default values.
// This is used for ResourceFetchers with a detached document, as well as tests.
class PLATFORM_EXPORT NullResourceFetcherProperties final
    : public ResourceFetcherProperties {
 public:
  NullResourceFetcherProperties();
  ~NullResourceFetcherProperties() override = default;

  void Trace(Visitor*) const override;

  // ResourceFetcherProperties implementation
  const FetchClientSettingsObject& GetFetchClientSettingsObject()
      const override {
    return *fetch_client_settings_object_;
  }
  bool IsOutermostMainFrame() const override { return false; }
  ControllerServiceWorkerMode GetControllerServiceWorkerMode() const override {
    return ControllerServiceWorkerMode::kNoController;
  }
  int64_t ServiceWorkerId() const override { NOTREACHED(); }
  bool IsPaused() const override { return false; }
  LoaderFreezeMode FreezeMode() const override {
    return LoaderFreezeMode::kNone;
  }
  bool IsDetached() const override { return true; }
  bool IsLoadComplete() const override { return true; }
  bool ShouldBlockLoadingSubResource() const override { return true; }
  bool IsSubframeDeprioritizationEnabled() const override { return false; }
  scheduler::FrameStatus GetFrameStatus() const override {
    return scheduler::FrameStatus::kNone;
  }
  int GetOutstandingThrottledLimit() const override { return 0; }

 private:
  const Member<const FetchClientSettingsObject> fetch_client_settings_object_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_NULL_RESOURCE_FETCHER_PROPERTIES_H_
