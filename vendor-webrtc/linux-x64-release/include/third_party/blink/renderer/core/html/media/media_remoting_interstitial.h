// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_MEDIA_MEDIA_REMOTING_INTERSTITIAL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_MEDIA_MEDIA_REMOTING_INTERSTITIAL_H_

#include "third_party/blink/renderer/core/html/html_div_element.h"
#include "third_party/blink/renderer/platform/timer.h"

namespace blink {

class HTMLImageElement;
class HTMLVideoElement;
class WebString;

// Media Remoting UI. DOM structure looks like:
//
// MediaRemotingInterstitial
//     (-internal-media-remoting-interstitial)
// +-HTMLImageElement
// |    (-internal-media-interstitial-background-image)
// \-HTMLDivElement
// |    (-internal-media-remoting-cast-icon)
// \-HTMLDivElement
// |    (-internal-media-interstitial-message)
// |-HTMLDivElement
//      (-internal-media-remoting-toast-message)
class MediaRemotingInterstitial final : public HTMLDivElement {
 public:
  explicit MediaRemotingInterstitial(HTMLVideoElement&);

  // Show Media Remoting interstitial. |remote_device_friendly_name| will be
  // shown in the UI to indicate which device the content is rendered on. An
  // empty name indicates an unknown remote device. A default message will be
  // shown in this case.
  void Show(const WebString& remote_device_friendly_name);

  // Hide Media Remoting interstitial. A text message may be displayed for five
  // seconds according to the IDS string associated with the given |error_code|.
  void Hide(int error_code);

  void OnPosterImageChanged();

  // Query for whether the remoting interstitial is visible.
  bool IsVisible() const { return state_ == kVisible; }

  HTMLVideoElement& GetVideoElement() const { return *video_element_; }

  void Trace(Visitor*) const override;

 private:
  // Node override.
  bool IsMediaRemotingInterstitial() const override { return true; }
  void DidMoveToNewDocument(Document&) override;

  void ToggleInterstitialTimerFired(TimerBase*);

  // Indicates whether the interstitial should be visible. It is set/changed
  // when Show()/Hide() is called.
  enum State {
    kHidden,   // The interstitial is currently not showing.
    kVisible,  // The interstitial is currently visible except the toast.
    kToast,    // Only the toast is visible.
  };
  State state_ = kHidden;

  HeapTaskRunnerTimer<MediaRemotingInterstitial> toggle_interstitial_timer_;
  Member<HTMLVideoElement> video_element_;
  Member<HTMLImageElement> background_image_;
  Member<HTMLDivElement> cast_icon_;
  Member<HTMLDivElement> cast_text_message_;
  Member<HTMLDivElement> toast_message_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_MEDIA_MEDIA_REMOTING_INTERSTITIAL_H_
