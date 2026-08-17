// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_CAST_BUTTON_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_CAST_BUTTON_ELEMENT_H_

#include "third_party/blink/renderer/modules/media_controls/elements/media_control_input_element.h"

namespace blink {

class Event;
class MediaControlsImpl;

class MediaControlCastButtonElement final : public MediaControlInputElement {
 public:
  MediaControlCastButtonElement(MediaControlsImpl&, bool is_overlay_button);

  // This will show a cast button if it is not covered by another element.
  // This MUST be called for cast button elements that are overlay elements.
  void TryShowOverlay();

  // TODO(avayvod): replace with the button listening to the state change
  // events.
  void UpdateDisplayType() override;

  // MediaControlInputElement overrides.
  bool WillRespondToMouseClickEvents() final;
  int GetOverflowStringId() const final;
  bool HasOverflowButton() const final;

 protected:
  const char* GetNameForHistograms() const final;

 private:
  void DefaultEventHandler(Event&) final;
  bool KeepEventInNode(const Event&) const final;

  bool IsPlayingRemotely() const;

  bool is_overlay_button_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_CAST_BUTTON_ELEMENT_H_
