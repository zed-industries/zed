// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_POPUP_MENU_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_POPUP_MENU_ELEMENT_H_

#include "third_party/blink/renderer/modules/media_controls/elements/media_control_div_element.h"

namespace blink {

class MediaControlsImpl;

class MediaControlPopupMenuElement : public MediaControlDivElement {
 public:
  ~MediaControlPopupMenuElement() override;

  void SetIsWanted(bool) override;

  // Callback run when an item is selected from the popup menu.
  virtual void OnItemSelected();

  // Node override.
  void DefaultEventHandler(Event&) override;
  bool KeepEventInNode(const Event&) const override;
  void RemovedFrom(ContainerNode&) override;

  void Trace(Visitor*) const override;

 protected:
  MediaControlPopupMenuElement(MediaControlsImpl&);

  void SetPosition();

 private:
  class EventListener;

  Element* PopupAnchor() const;

  void HideIfNotFocused();

  bool FocusListItemIfDisplayed(Node* node);
  void SelectFirstItem();

  // Actions called by the EventListener object when specific evenst are
  // received.
  void SelectNextItem();
  void SelectPreviousItem();
  void CloseFromKeyboard();
  void FocusPopupAnchorIfOverflowClosed();

  Member<EventListener> event_listener_;
  // |last_focused_element_| is used to return focus to the proper element
  // within the media controls popup menu, after the user finishes interacting
  // with the popup's scrollbar.
  Member<Element> last_focused_element_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_POPUP_MENU_ELEMENT_H_
