// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RESOURCE_FETCHER_PROPERTIES_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RESOURCE_FETCHER_PROPERTIES_H_

#include "third_party/blink/public/mojom/service_worker/controller_service_worker_mode.mojom-blink.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/loader/fetch/loader_freeze_mode.h"
#include "third_party/blink/renderer/platform/loader/fetch/url_loader/url_loader.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/scheduler/public/frame_status.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/weborigin/security_origin.h"

namespace blink {

class FetchClientSettingsObject;

// ResourceFetcherProperties consists of properties of the global context (e.g.,
// Frame, Worker) necessary to fetch resources. FetchClientSettingsObject
// implementing https://html.spec.whatwg.org/C/webappapis.html#settings-object
// is one such example.
//
// This class consists of pure virtual getters. Do not put operations. Do not
// put getters for a specific request such as
// GetCachePolicy(const ResourceRequest&, ResourceType). Do not put a function
// with default implementation.
//
// Storing a non-null ResourceFetcherProperties in an object that can be valid
// after the associated ResourceFetcher is detached is dangerous. Use
// DetachedResourceFetcherProperties below.
//
// The distinction between FetchClientSettingsObject and
// ResourceFetcherProperties is sometimes ambiguous. Put a property in
// FetchClientSettingsObject when the property is clearly defined in the spec.
// Otherwise, put it to this class.
class PLATFORM_EXPORT ResourceFetcherProperties
    : public GarbageCollected<ResourceFetcherProperties> {
 public:
  using ControllerServiceWorkerMode = mojom::ControllerServiceWorkerMode;

  ResourceFetcherProperties() = default;
  virtual ~ResourceFetcherProperties() = default;
  virtual void Trace(Visitor*) const {}

  // Returns the client settings object bound to this global context.
  virtual const FetchClientSettingsObject& GetFetchClientSettingsObject()
      const = 0;

  // Returns whether this global context is the outermost main frame.
  virtual bool IsOutermostMainFrame() const = 0;

  // Returns whether a controller service worker exists and if it has a fetch
  // handler.
  virtual ControllerServiceWorkerMode GetControllerServiceWorkerMode()
      const = 0;

  // Returns an identifier for the service worker controlling this global
  // context. This function cannot be called when
  // GetControllerServiceWorkerMode returns kNoController.
  virtual int64_t ServiceWorkerId() const = 0;

  // Returns whether this global context is suspended, which means we should
  // defer making a new request.
  // https://html.spec.whatwg.org/C/webappapis.html#pause
  virtual bool IsPaused() const = 0;

  // Returns the freezing mode set to this context.
  virtual LoaderFreezeMode FreezeMode() const = 0;

  // Returns whether this global context is detached. Note that in some cases
  // the loading pipeline continues working after detached (e.g., for fetch()
  // operations with "keepalive" specified).
  virtual bool IsDetached() const = 0;

  // Returns whether the main resource for this global context is loaded.
  virtual bool IsLoadComplete() const = 0;

  // Returns whether we should disallow a sub resource loading.
  virtual bool ShouldBlockLoadingSubResource() const = 0;

  // Returns whether we should de-prioritize requests in sub frames.
  // TODO(yhirano): Make this ShouldDepriotizeRequest once the related
  // histograms get deprecated. See https://crbug.com/800035.
  virtual bool IsSubframeDeprioritizationEnabled() const = 0;

  // Returns the scheduling status of the associated frame. Returns |kNone|
  // if there is no such a frame.
  virtual scheduler::FrameStatus GetFrameStatus() const = 0;

  virtual int GetOutstandingThrottledLimit() const = 0;
};

// A delegating ResourceFetcherProperties subclass which can be retained
// even when the associated ResourceFetcher is detached.
class PLATFORM_EXPORT DetachableResourceFetcherProperties final
    : public ResourceFetcherProperties {
 public:
  explicit DetachableResourceFetcherProperties(
      const ResourceFetcherProperties& properties)
      : properties_(properties) {}
  ~DetachableResourceFetcherProperties() override = default;

  void Detach();

  void Trace(Visitor* visitor) const override;

  // ResourceFetcherProperties implementation
  // Add a test in resource_fetcher_test.cc when you change behaviors.
  const FetchClientSettingsObject& GetFetchClientSettingsObject()
      const override {
    return properties_ ? properties_->GetFetchClientSettingsObject()
                       : *fetch_client_settings_object_;
  }
  bool IsOutermostMainFrame() const override {
    return properties_ ? properties_->IsOutermostMainFrame()
                       : is_outermost_main_frame_;
  }
  ControllerServiceWorkerMode GetControllerServiceWorkerMode() const override {
    return properties_ ? properties_->GetControllerServiceWorkerMode()
                       : ControllerServiceWorkerMode::kNoController;
  }
  int64_t ServiceWorkerId() const override {
    // When detached, GetControllerServiceWorkerMode returns kNoController, so
    // this function must not be called.
    DCHECK(properties_);
    return properties_->ServiceWorkerId();
  }
  bool IsPaused() const override {
    return properties_ ? properties_->IsPaused() : paused_;
  }
  LoaderFreezeMode FreezeMode() const override {
    return properties_ ? properties_->FreezeMode() : freeze_mode_;
  }
  bool IsDetached() const override {
    return properties_ ? properties_->IsDetached() : true;
  }
  bool IsLoadComplete() const override {
    return properties_ ? properties_->IsLoadComplete() : load_complete_;
  }
  bool ShouldBlockLoadingSubResource() const override {
    // Returns true when detached in order to preserve the existing behavior.
    return properties_ ? properties_->ShouldBlockLoadingSubResource() : true;
  }
  bool IsSubframeDeprioritizationEnabled() const override {
    return properties_ ? properties_->IsSubframeDeprioritizationEnabled()
                       : is_subframe_deprioritization_enabled_;
  }

  scheduler::FrameStatus GetFrameStatus() const override {
    return properties_ ? properties_->GetFrameStatus()
                       : scheduler::FrameStatus::kNone;
  }

  int GetOutstandingThrottledLimit() const override {
    return properties_ ? properties_->GetOutstandingThrottledLimit()
                       : outstanding_throttled_limit_;
  }

 private:
  // |properties_| is null if and only if detached.
  Member<const ResourceFetcherProperties> properties_;

  // The following members are used when detached.
  Member<const FetchClientSettingsObject> fetch_client_settings_object_;
  bool is_outermost_main_frame_ = false;
  bool paused_ = false;
  LoaderFreezeMode freeze_mode_;
  bool load_complete_ = false;
  bool is_subframe_deprioritization_enabled_ = false;
  int outstanding_throttled_limit_ = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RESOURCE_FETCHER_PROPERTIES_H_
