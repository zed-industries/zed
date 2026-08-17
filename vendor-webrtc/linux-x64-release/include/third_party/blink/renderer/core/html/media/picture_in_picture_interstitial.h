// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_MEDIA_PICTURE_IN_PICTURE_INTERSTITIAL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_MEDIA_PICTURE_IN_PICTURE_INTERSTITIAL_H_

#include "third_party/blink/renderer/core/html/html_div_element.h"
#include "third_party/blink/renderer/platform/timer.h"

namespace blink {

class DOMRectReadOnly;
class HTMLImageElement;
class HTMLVideoElement;
class ResizeObserver;

// Picture in Picture UI. DOM structure looks like:
//
// PictureInPictureInterstitial
//     (-internal-picture-in-picture-interstitial)
// +-HTMLImageElement
// |    (-internal-media-interstitial-background-image)
// \-HTMLDivElement
// |    (-internal-picture-in-picture-interstitial-message)
class PictureInPictureInterstitial final : public HTMLDivElement {
 public:
  explicit PictureInPictureInterstitial(HTMLVideoElement&);

  void Show();
  void Hide();

  void OnPosterImageChanged();
  bool IsVisible() const { return should_be_visible_; }

  HTMLVideoElement& GetVideoElement() const { return *video_element_; }

  // Node override.
  Node::InsertionNotificationRequest InsertedInto(ContainerNode&) override;
  void RemovedFrom(ContainerNode&) override;

  // Element:
  void Trace(Visitor*) const override;

 private:
  class VideoElementResizeObserverDelegate;

  // Node override.
  bool IsPictureInPictureInterstitial() const override { return true; }

  // Notify us that our controls enclosure has changed size.
  void NotifyElementSizeChanged(const DOMRectReadOnly& new_size);

  void ToggleInterstitialTimerFired(TimerBase*);

  // Indicates whether the interstitial should be visible. It is updated
  // when Show()/Hide() is called.
  bool should_be_visible_ = false;

  // Watches the video element for resize and updates the intersitial as
  // necessary.
  Member<ResizeObserver> resize_observer_;

  HeapTaskRunnerTimer<PictureInPictureInterstitial> interstitial_timer_;
  Member<HTMLVideoElement> video_element_;
  Member<HTMLImageElement> background_image_;
  Member<HTMLDivElement> message_element_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_MEDIA_PICTURE_IN_PICTURE_INTERSTITIAL_H_
