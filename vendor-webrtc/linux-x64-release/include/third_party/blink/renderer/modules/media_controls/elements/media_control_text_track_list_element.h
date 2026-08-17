// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_TEXT_TRACK_LIST_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_TEXT_TRACK_LIST_ELEMENT_H_

#include "third_party/blink/renderer/modules/media_controls/elements/media_control_popup_menu_element.h"

namespace blink {

class Event;
class MediaControlsImpl;
class TextTrack;

class MediaControlTextTrackListElement final
    : public MediaControlPopupMenuElement {
 public:
  explicit MediaControlTextTrackListElement(MediaControlsImpl&);

  // Node interface.
  bool WillRespondToMouseClickEvents() override;

  void SetIsWanted(bool) final;

 private:
  void DefaultEventHandler(Event&) override;

  void RefreshTextTrackListMenu();

  // Creates the track element in the list when a valid track is passed in and
  // the "Off" item when the parameter is null.
  Element* CreateTextTrackListItem(TextTrack*);

  // Creates the header element of the text track list.
  Element* CreateTextTrackHeaderItem();
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_TEXT_TRACK_LIST_ELEMENT_H_
