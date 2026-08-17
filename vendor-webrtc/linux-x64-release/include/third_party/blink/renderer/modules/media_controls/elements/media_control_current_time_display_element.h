// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_CURRENT_TIME_DISPLAY_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_CURRENT_TIME_DISPLAY_ELEMENT_H_

#include "third_party/blink/renderer/modules/media_controls/elements/media_control_time_display_element.h"

namespace blink {

class MediaControlsImpl;

class MediaControlCurrentTimeDisplayElement final
    : public MediaControlTimeDisplayElement {
 public:
  explicit MediaControlCurrentTimeDisplayElement(MediaControlsImpl&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_CURRENT_TIME_DISPLAY_ELEMENT_H_
