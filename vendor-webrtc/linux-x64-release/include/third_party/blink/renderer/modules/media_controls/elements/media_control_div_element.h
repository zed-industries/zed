// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_DIV_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_DIV_ELEMENT_H_

#include "third_party/blink/renderer/core/html/html_div_element.h"
#include "third_party/blink/renderer/modules/media_controls/elements/media_control_element_base.h"

namespace blink {

class MediaControlsImpl;

// MediaControlElementBase implementation based on a <div>. Used for panels, and
// floating UI.
class MODULES_EXPORT MediaControlDivElement : public HTMLDivElement,
                                              public MediaControlElementBase {
 public:
  // Implements MediaControlElementBase.
  void SetOverflowElementIsWanted(bool) final;
  void MaybeRecordDisplayed() final;

  // Get the size of the element in pixels or the default if we cannot get the
  // size because the element has not been laid out yet.
  gfx::Size GetSizeOrDefault() const override;

  bool IsDisabled() const override;

  void Trace(Visitor*) const override;

 protected:
  MediaControlDivElement(MediaControlsImpl&);

 private:
  bool IsMediaControlElement() const final;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_DIV_ELEMENT_H_
