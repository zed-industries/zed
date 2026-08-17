// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_VOLUME_SLIDER_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_VOLUME_SLIDER_ELEMENT_H_

#include "third_party/blink/renderer/modules/media_controls/elements/media_control_slider_element.h"

namespace blink {

class Event;
class MediaControlsImpl;
class MediaControlVolumeControlContainerElement;
class WheelEvent;

class MediaControlVolumeSliderElement final : public MediaControlSliderElement {
 public:
  MediaControlVolumeSliderElement(
      MediaControlsImpl&,
      MediaControlVolumeControlContainerElement* container);

  // TODO: who calls this?
  void SetVolume(double);

  void OpenSlider();
  void CloseSlider();

  // MediaControlInputElement overrides.
  bool WillRespondToMouseMoveEvents() const override;
  bool WillRespondToMouseClickEvents() override;

  void OnMediaKeyboardEvent(Event* event) { DefaultEventHandler(*event); }

  void Trace(Visitor*) const override;

 protected:
  const char* GetNameForHistograms() const override;

 private:
  class WheelEventListener;

  void DefaultEventHandler(Event&) override;
  bool KeepEventInNode(const Event&) const override;
  void SetVolumeInternal(double);
  void OnWheelEvent(WheelEvent*);
  void UnmuteAndSetVolume(double);

  Member<WheelEventListener> wheel_event_listener_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_VOLUME_SLIDER_ELEMENT_H_
