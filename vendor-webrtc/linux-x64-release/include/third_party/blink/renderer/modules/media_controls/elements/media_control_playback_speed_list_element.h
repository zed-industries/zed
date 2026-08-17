// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_PLAYBACK_SPEED_LIST_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_PLAYBACK_SPEED_LIST_ELEMENT_H_

#include "third_party/blink/renderer/modules/media_controls/elements/media_control_popup_menu_element.h"

namespace blink {

class Event;
class MediaControlsImpl;

class MediaControlPlaybackSpeedListElement final
    : public MediaControlPopupMenuElement {
 public:
  explicit MediaControlPlaybackSpeedListElement(MediaControlsImpl&);

  // Node interface.
  bool WillRespondToMouseClickEvents() override;

  void SetIsWanted(bool) final;

  void Trace(Visitor*) const override;

 private:
  class RequestAnimationFrameCallback;

  void DefaultEventHandler(Event&) override;

  void RefreshPlaybackSpeedListMenu();

  // Creates the playback speed element in the list.
  Element* CreatePlaybackSpeedListItem(const int display_name,
                                       const double playback_rate);

  // Creates the header element of the playback speed list.
  Element* CreatePlaybackSpeedHeaderItem();

  // Centers vertically the checked item in the playback speed list.
  void CenterCheckedItem();

  Member<Element> checked_item_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_PLAYBACK_SPEED_LIST_ELEMENT_H_
