// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_VOLUME_CONTROL_CONTAINER_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_VOLUME_CONTROL_CONTAINER_ELEMENT_H_

#include "third_party/blink/renderer/modules/media_controls/elements/media_control_div_element.h"

namespace blink {

class MediaControlsImpl;

class MediaControlVolumeControlContainerElement final
    : public MediaControlDivElement {
 public:
  explicit MediaControlVolumeControlContainerElement(MediaControlsImpl&);

  void OpenContainer();
  void CloseContainer();

 private:
  void DefaultEventHandler(Event&) override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_VOLUME_CONTROL_CONTAINER_ELEMENT_H_
