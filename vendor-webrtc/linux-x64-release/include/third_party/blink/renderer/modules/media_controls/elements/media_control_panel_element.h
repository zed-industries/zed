// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_PANEL_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_PANEL_ELEMENT_H_

#include "third_party/blink/renderer/modules/media_controls/elements/media_control_div_element.h"
#include "third_party/blink/renderer/modules/media_controls/media_controls_shared_helper.h"
#include "third_party/blink/renderer/modules/modules_export.h"

namespace blink {

class ContainerNode;
class Event;
class MediaControlsImpl;

class MODULES_EXPORT MediaControlPanelElement final
    : public MediaControlDivElement {
 public:
  explicit MediaControlPanelElement(MediaControlsImpl&);

 public:
  // Methods called by `MediaControlsImpl`.
  void SetIsDisplayed(bool);
  bool IsOpaque() const;
  void MakeOpaque();
  void MakeTransparent();

  bool KeepDisplayedForAccessibility();
  void SetKeepDisplayedForAccessibility(bool);

  // Node override;
  void RemovedFrom(ContainerNode&) override;

  void Trace(Visitor*) const override;

 protected:
  friend class MediaControlPanelElementTest;

  bool EventListenerIsAttachedForTest() const;

 private:
  void EnsureTransitionEventListener();
  void DetachTransitionEventListener();

  bool KeepEventInNode(const Event&) const override;

  void DidBecomeVisible();
  void HandleTransitionEndEvent();

  bool is_displayed_ = false;
  bool opaque_ = true;
  bool keep_displayed_for_accessibility_ = false;

  Member<MediaControlsSharedHelpers::TransitionEventListener> event_listener_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_PANEL_ELEMENT_H_
