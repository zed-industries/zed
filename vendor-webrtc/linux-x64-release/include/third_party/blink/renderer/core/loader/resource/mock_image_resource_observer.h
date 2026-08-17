// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RESOURCE_MOCK_IMAGE_RESOURCE_OBSERVER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RESOURCE_MOCK_IMAGE_RESOURCE_OBSERVER_H_

#include "base/memory/ptr_util.h"
#include "third_party/blink/renderer/core/loader/resource/image_resource.h"
#include "third_party/blink/renderer/core/loader/resource/image_resource_content.h"
#include "third_party/blink/renderer/core/loader/resource/image_resource_observer.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_status.h"

namespace blink {

class MockImageResourceObserver final
    : public GarbageCollected<MockImageResourceObserver>,
      public ImageResourceObserver {
 public:
  explicit MockImageResourceObserver(ImageResourceContent*);
  ~MockImageResourceObserver() override = default;

  void RemoveAsObserver();

  int ImageChangedCount() const { return image_changed_count_; }
  bool ImageNotifyFinishedCalled() const;

  int ImageWidthOnLastImageChanged() const {
    return image_width_on_last_image_changed_;
  }
  int ImageWidthOnImageNotifyFinished() const {
    return image_width_on_image_notify_finished_;
  }
  ResourceStatus StatusOnImageNotifyFinished() const {
    return status_on_image_notify_finished_;
  }

  CanDeferInvalidation Defer() const { return defer_; }

  void Trace(Visitor*) const override;

 private:
  // ImageResourceObserver overrides.
  void ImageNotifyFinished(ImageResourceContent*) override;
  void ImageChanged(ImageResourceContent*, CanDeferInvalidation) override;
  String DebugName() const override { return "MockImageResourceObserver"; }

  Member<ImageResourceContent> content_;
  int image_changed_count_;
  CanDeferInvalidation defer_;
  int image_width_on_last_image_changed_;
  int image_notify_finished_count_;
  int image_width_on_image_notify_finished_;
  ResourceStatus status_on_image_notify_finished_ = ResourceStatus::kNotStarted;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RESOURCE_MOCK_IMAGE_RESOURCE_OBSERVER_H_
