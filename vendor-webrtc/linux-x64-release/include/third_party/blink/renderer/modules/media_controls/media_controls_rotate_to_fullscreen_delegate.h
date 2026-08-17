// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_MEDIA_CONTROLS_ROTATE_TO_FULLSCREEN_DELEGATE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_MEDIA_CONTROLS_ROTATE_TO_FULLSCREEN_DELEGATE_H_

#include <optional>

#include "third_party/blink/renderer/core/dom/events/native_event_listener.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"

namespace blink {

class DeviceOrientationEvent;
class HTMLVideoElement;
class IntersectionObserver;
class IntersectionObserverEntry;

// MediaControlsRotateToFullscreenDelegate automatically enters and exits
// fullscreen when the device is rotated whilst watching a <video>. It is meant
// to be created by `MediaControlsImpl` when the feature applies. Once created,
// it will listen for events.
class MediaControlsRotateToFullscreenDelegate final
    : public NativeEventListener {
 public:
  explicit MediaControlsRotateToFullscreenDelegate(HTMLVideoElement&);

  // Called by MediaControlsImpl when the HTMLMediaElement is added to a
  // document. All event listeners should be added.
  void Attach();

  // Called by MediaControlsImpl when the HTMLMediaElement is no longer in the
  // document. All event listeners should be removed in order to prepare the
  // object to be garbage collected.
  void Detach();

  // EventListener implementation.
  void Invoke(ExecutionContext*, Event*) override;

  void Trace(Visitor*) const override;

 private:
  friend class MediaControlsRotateToFullscreenDelegateTest;

  // Represents either screen orientation or video aspect ratio.
  enum class SimpleOrientation { kPortrait, kLandscape, kUnknown };

  void OnStateChange();
  void OnIntersectionChange(
      const HeapVector<Member<IntersectionObserverEntry>>& entries);
  void OnDeviceOrientationAvailable(DeviceOrientationEvent*);
  void OnScreenOrientationChange();

  MODULES_EXPORT SimpleOrientation ComputeVideoOrientation() const;
  SimpleOrientation ComputeScreenOrientation() const;

  std::optional<bool> device_orientation_supported_;

  SimpleOrientation current_screen_orientation_ = SimpleOrientation::kUnknown;

  // Only valid when intersection_observer_ is active and the first
  // OnIntersectionChanged has been received; otherwise assume video is hidden.
  bool is_visible_ = false;

  // This is null whenever we're not listening.
  Member<IntersectionObserver> intersection_observer_ = nullptr;

  // `video_element_` owns MediaControlsImpl that owns |this|.
  Member<HTMLVideoElement> video_element_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_MEDIA_CONTROLS_ROTATE_TO_FULLSCREEN_DELEGATE_H_
