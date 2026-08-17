// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_SCRUBBING_MESSAGE_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_SCRUBBING_MESSAGE_ELEMENT_H_

#include "third_party/blink/renderer/modules/media_controls/elements/media_control_div_element.h"
#include "third_party/blink/renderer/modules/modules_export.h"

namespace blink {

class MediaControlsImpl;

class MODULES_EXPORT MediaControlScrubbingMessageElement final
    : public MediaControlDivElement {
 public:
  explicit MediaControlScrubbingMessageElement(MediaControlsImpl&);

  void SetIsWanted(bool) final;

 private:
  void PopulateChildren();
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_SCRUBBING_MESSAGE_ELEMENT_H_
