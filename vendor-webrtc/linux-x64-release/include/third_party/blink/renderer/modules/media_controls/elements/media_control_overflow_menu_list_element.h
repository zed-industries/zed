// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_OVERFLOW_MENU_LIST_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_OVERFLOW_MENU_LIST_ELEMENT_H_

#include "third_party/blink/renderer/modules/media_controls/elements/media_control_popup_menu_element.h"

namespace blink {

class Event;
class MediaControlsImpl;

// Holds a list of elements within the overflow menu.
class MediaControlOverflowMenuListElement final
    : public MediaControlPopupMenuElement {
 public:
  explicit MediaControlOverflowMenuListElement(MediaControlsImpl&);

  void OpenOverflowMenu();
  void CloseOverflowMenu();

  // Override MediaControlPopupMenuElement
  void SetIsWanted(bool) final;

 private:
  void DefaultEventHandler(Event&) override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_OVERFLOW_MENU_LIST_ELEMENT_H_
