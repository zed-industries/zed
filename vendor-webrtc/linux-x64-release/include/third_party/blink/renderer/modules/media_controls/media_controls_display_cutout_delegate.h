// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_MEDIA_CONTROLS_DISPLAY_CUTOUT_DELEGATE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_MEDIA_CONTROLS_DISPLAY_CUTOUT_DELEGATE_H_

#include <optional>

#include "third_party/blink/renderer/core/dom/events/native_event_listener.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class Document;
class HTMLVideoElement;
class TouchEvent;

// MediaControlsDisplayCutoutDelegate implements a two-fingered pinch gesture
// to expand the video element into the display cutout when the video is
// fullscreen.
class MODULES_EXPORT MediaControlsDisplayCutoutDelegate final
    : public NativeEventListener {
 public:
  static bool IsEnabled();

  explicit MediaControlsDisplayCutoutDelegate(HTMLVideoElement&);

  // Called by MediaControlsImpl when the HTMLVideoElement is added to a
  // document. All event listeners should be added.
  void Attach();

  // Called by MediaControls when the HTMLVideoElement is no longer in the
  // document. All event listeners should be removed in order to prepare the
  // object to be garbage collected.
  void Detach();

  // EventListener implementation.
  void Invoke(ExecutionContext*, Event*) override;

  void Trace(Visitor*) const override;

 private:
  friend class MediaControlsDisplayCutoutDelegateTest;

  // Stores the direction of the pinch gesture.
  enum class Direction {
    // We do not have enough data to work out the direction.
    kUnknown,

    // The distance between the two points is expanding.
    kExpanding,

    // The distance between the two points is contracting.
    kContracting,
  };

  void DidEnterFullscreen();
  void DidExitFullscreen();

  void HandleTouchEvent(TouchEvent*);

  Document& GetDocument();

  // Pair storing the previous result. |first| is the distance between the two
  // points stored as a double. |second| is the Direction.
  using ResultPair = std::pair<double, Direction>;
  std::optional<ResultPair> previous_;

  Member<HTMLVideoElement> video_element_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_MEDIA_CONTROLS_DISPLAY_CUTOUT_DELEGATE_H_
