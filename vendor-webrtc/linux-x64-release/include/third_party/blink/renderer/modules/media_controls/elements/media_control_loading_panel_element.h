// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_LOADING_PANEL_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_LOADING_PANEL_ELEMENT_H_

#include "third_party/blink/renderer/modules/media_controls/elements/media_control_animation_event_listener.h"
#include "third_party/blink/renderer/modules/media_controls/elements/media_control_div_element.h"
#include "third_party/blink/renderer/modules/modules_export.h"

namespace blink {

class ContainerNode;
class Element;
class HTMLDivElement;
class MediaControlsImpl;

// The loading panel shows the semi-transparent white mask and the transparent
// loading spinner.
class MODULES_EXPORT MediaControlLoadingPanelElement final
    : public MediaControlDivElement,
      public MediaControlAnimationEventListener::Observer {
 public:
  explicit MediaControlLoadingPanelElement(MediaControlsImpl&);

  // Update the state based on the Media Controls state.
  void UpdateDisplayState();

  // Inform the loading panel that the Media Controls have been hidden.
  void OnControlsHidden();
  // Inform the loading panel that the Media Controls have been shown.
  void OnControlsShown();

  void Trace(Visitor*) const override;

 private:
  friend class MediaControlLoadingPanelElementTest;

  enum State {
    // The loading panel is hidden.
    kHidden,

    // The loading panel is shown and is playing the "spinner" animation.
    kPlaying,

    // The loading panel is playing the "cooldown" animation and will hide
    // automatically once it is complete.
    kCoolingDown,
  };

  // These are used by AnimationEventListener to notify of animation events.
  void OnAnimationEnd() override;
  void OnAnimationIteration() override;
  Element& WatchedAnimationElement() const override;

  // Hide the animation and clean up the shadow DOM.
  void HideAnimation();

  // This sets the "animation-iteration-count" CSS property on the mask
  // background elements.
  void SetAnimationIterationCount(const String&);

  // The loading panel is only used once and has a lot of DOM elements so these
  // two functions will populate the shadow DOM or clean it if the panel is
  // hidden.
  void CleanupShadowDOM();
  void PopulateShadowDOM();

  // Cleans up the event listener when this element is removed from the DOM.
  void RemovedFrom(ContainerNode&) override;

  // This counts how many animation iterations the background elements have
  // played.
  int animation_count_ = 0;
  State state_ = State::kHidden;

  // Whether the Media Controls are hidden.
  bool controls_hidden_ = false;

  Member<MediaControlAnimationEventListener> event_listener_;
  Member<HTMLDivElement> mask1_background_;
  Member<HTMLDivElement> mask2_background_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_LOADING_PANEL_ELEMENT_H_
