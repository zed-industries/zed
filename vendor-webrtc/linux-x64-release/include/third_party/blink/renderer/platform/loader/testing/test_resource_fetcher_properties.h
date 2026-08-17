// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_TESTING_TEST_RESOURCE_FETCHER_PROPERTIES_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_TESTING_TEST_RESOURCE_FETCHER_PROPERTIES_H_

#include "base/check_op.h"
#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/platform/heap/forward.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/loader/fetch/loader_freeze_mode.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_fetcher_properties.h"

namespace blink {

class FetchClientSettingsObject;
class SecurityOrigin;

// TestResourceFetcherProperties is a ResourceFetcherProperties implementation
// for tests.
class TestResourceFetcherProperties final : public ResourceFetcherProperties {
 public:
  TestResourceFetcherProperties();
  explicit TestResourceFetcherProperties(scoped_refptr<const SecurityOrigin>);
  explicit TestResourceFetcherProperties(const FetchClientSettingsObject&);
  ~TestResourceFetcherProperties() override = default;

  void Trace(Visitor* visitor) const override;

  DetachableResourceFetcherProperties& MakeDetachable() const {
    return *MakeGarbageCollected<DetachableResourceFetcherProperties>(*this);
  }

  // ResourceFetcherProperties implementation
  const FetchClientSettingsObject& GetFetchClientSettingsObject()
      const override {
    return *fetch_client_settings_object_;
  }
  bool IsOutermostMainFrame() const override {
    return is_outermost_main_frame_;
  }
  ControllerServiceWorkerMode GetControllerServiceWorkerMode() const override {
    return service_worker_mode_;
  }
  int64_t ServiceWorkerId() const override {
    DCHECK_NE(GetControllerServiceWorkerMode(),
              ControllerServiceWorkerMode::kNoController);
    return service_worker_id_;
  }
  bool IsPaused() const override { return paused_; }
  LoaderFreezeMode FreezeMode() const override { return freeze_mode_; }
  bool IsDetached() const override { return false; }
  bool IsLoadComplete() const override { return load_complete_; }
  bool ShouldBlockLoadingSubResource() const override {
    return should_block_loading_sub_resource_;
  }
  bool IsSubframeDeprioritizationEnabled() const override {
    return is_subframe_deprioritization_enabled_;
  }
  scheduler::FrameStatus GetFrameStatus() const override {
    return frame_status_;
  }
  int GetOutstandingThrottledLimit() const override {
    return IsOutermostMainFrame() ? 3 : 2;
  }

  void SetIsOutermostMainFrame(bool value) { is_outermost_main_frame_ = value; }
  void SetControllerServiceWorkerMode(ControllerServiceWorkerMode mode) {
    service_worker_mode_ = mode;
  }
  void SetServiceWorkerId(int64_t id) { service_worker_id_ = id; }
  void SetIsPaused(bool value) { paused_ = value; }
  void SetIsLoadComplete(bool value) { load_complete_ = value; }
  void SetShouldBlockLoadingSubResource(bool value) {
    should_block_loading_sub_resource_ = value;
  }
  void SetIsSubframeDeprioritizationEnabled(bool value) {
    is_subframe_deprioritization_enabled_ = value;
  }
  void SetFrameStatus(scheduler::FrameStatus status) { frame_status_ = status; }

 private:
  const Member<const FetchClientSettingsObject> fetch_client_settings_object_;
  bool is_outermost_main_frame_ = false;
  ControllerServiceWorkerMode service_worker_mode_ =
      ControllerServiceWorkerMode::kNoController;
  int64_t service_worker_id_ = 0;
  bool paused_ = false;
  LoaderFreezeMode freeze_mode_ = LoaderFreezeMode::kNone;
  bool load_complete_ = false;
  bool should_block_loading_sub_resource_ = false;
  bool is_subframe_deprioritization_enabled_ = false;
  scheduler::FrameStatus frame_status_ = scheduler::FrameStatus::kNone;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_TESTING_TEST_RESOURCE_FETCHER_PROPERTIES_H_
