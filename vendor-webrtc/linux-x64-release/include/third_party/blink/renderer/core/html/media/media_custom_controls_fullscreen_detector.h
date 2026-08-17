// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_MEDIA_MEDIA_CUSTOM_CONTROLS_FULLSCREEN_DETECTOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_MEDIA_MEDIA_CUSTOM_CONTROLS_FULLSCREEN_DETECTOR_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/events/native_event_listener.h"
#include "third_party/blink/renderer/core/intersection_observer/intersection_observer.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/timer.h"

namespace blink {

class HTMLVideoElement;
class TimerBase;

// This class tracks the state and size of HTMLVideoElement and reports to it
// two signals
// 1. If a given video occupies a large part of the viewport (85%),
//    it is reported via HTMLVideoElement::SetIsDominantVisibleContent.
// 2. If a given video occupies a large part of the viewport (85%) and
//    any of the video's parent elements are in fullscreen mode, it is reported
//    via SetIsEffectivelyFullscreen.
class CORE_EXPORT MediaCustomControlsFullscreenDetector final
    : public NativeEventListener {
 public:
  explicit MediaCustomControlsFullscreenDetector(HTMLVideoElement& video);

  void Attach();
  void Detach();
  void ContextDestroyed();

  // EventListener implementation.
  void Invoke(ExecutionContext*, Event*) override;

  void Trace(Visitor*) const override;
  void TriggerObservation();

 private:
  friend class MediaCustomControlsFullscreenDetectorTest;
  friend class HTMLMediaElementEventListenersTest;

  HTMLVideoElement& VideoElement() { return *video_element_; }

  void OnCheckViewportIntersectionTimerFired(TimerBase*);
  void OnIntersectionChanged(
      const HeapVector<Member<IntersectionObserverEntry>>&);
  bool IsVideoOrParentFullscreen();
  void ReportEffectivelyFullscreen(bool);
  static bool IsFullscreenVideoOfDifferentRatioForTesting(
      const gfx::Size& video_size,
      const gfx::Size& viewport_size,
      const gfx::Size& intersection_size);

  void UpdateDominantAndFullscreenStatus(bool is_dominant_visible_content,
                                         bool is_effectively_fullscreen);

  // `video_element_` owns |this|.
  Member<HTMLVideoElement> video_element_;
  Member<IntersectionObserver> viewport_intersection_observer_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_MEDIA_MEDIA_CUSTOM_CONTROLS_FULLSCREEN_DETECTOR_H_
