// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_FULLSCREEN_BUTTON_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_FULLSCREEN_BUTTON_ELEMENT_H_

#include "third_party/blink/renderer/modules/media_controls/elements/media_control_input_element.h"

namespace blink {

class Event;
class MediaControlsImpl;

class MediaControlFullscreenButtonElement final
    : public MediaControlInputElement {
 public:
  explicit MediaControlFullscreenButtonElement(MediaControlsImpl&);

  // TODO(mlamouri): this should be changed to UpdateDisplayType().
  void SetIsFullscreen(bool);

  // MediaControlInputElement overrides.
  bool WillRespondToMouseClickEvents() override;
  int GetOverflowStringId() const override;
  bool HasOverflowButton() const override;
  bool IsControlPanelButton() const override;

 protected:
  const char* GetNameForHistograms() const override;

 private:
  void DefaultEventHandler(Event&) override;
  void RecordClickMetrics();
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_FULLSCREEN_BUTTON_ELEMENT_H_
