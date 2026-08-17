// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_BUTTON_PANEL_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_BUTTON_PANEL_ELEMENT_H_

#include "third_party/blink/renderer/modules/media_controls/elements/media_control_div_element.h"

namespace blink {

class MediaControlsImpl;

// This element groups the buttons together in the media controls. It needs to
// be a subclass of MediaControlElementBase so it can be factored in when
// working out which elements to hide/show based on the size of the media
// element.
class MediaControlButtonPanelElement final : public MediaControlDivElement {
 public:
  explicit MediaControlButtonPanelElement(MediaControlsImpl&);

 private:
  bool KeepEventInNode(const Event&) const override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_BUTTON_PANEL_ELEMENT_H_
