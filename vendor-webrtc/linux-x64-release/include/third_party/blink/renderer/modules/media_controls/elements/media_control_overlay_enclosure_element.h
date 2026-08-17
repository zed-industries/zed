// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_OVERLAY_ENCLOSURE_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_OVERLAY_ENCLOSURE_ELEMENT_H_

#include "third_party/blink/renderer/modules/media_controls/elements/media_control_div_element.h"

namespace blink {

class Event;
class MediaControlsImpl;

class MediaControlOverlayEnclosureElement final
    : public MediaControlDivElement {
 public:
  explicit MediaControlOverlayEnclosureElement(MediaControlsImpl&);

  void DefaultEventHandler(Event&) override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_OVERLAY_ENCLOSURE_ELEMENT_H_
