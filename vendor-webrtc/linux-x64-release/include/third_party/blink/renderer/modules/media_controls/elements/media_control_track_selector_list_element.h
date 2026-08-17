// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_TRACK_SELECTOR_LIST_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_TRACK_SELECTOR_LIST_ELEMENT_H_

#include "third_party/blink/public/strings/grit/blink_strings.h"
#include "third_party/blink/renderer/core/html/forms/html_input_element.h"
#include "third_party/blink/renderer/core/html/forms/html_label_element.h"
#include "third_party/blink/renderer/core/html/html_element.h"
#include "third_party/blink/renderer/core/html/html_span_element.h"
#include "third_party/blink/renderer/core/html/track/audio_track_list.h"
#include "third_party/blink/renderer/core/html/track/track_list_base.h"
#include "third_party/blink/renderer/core/html/track/video_track_list.h"
#include "third_party/blink/renderer/core/input_type_names.h"
#include "third_party/blink/renderer/core/keywords.h"
#include "third_party/blink/renderer/modules/media_controls/elements/media_control_popup_menu_element.h"
#include "third_party/blink/renderer/platform/text/platform_locale.h"
#include "ui/strings/grit/ax_strings.h"

namespace blink {

class Event;
class MediaControlsImpl;

class MediaControlTrackSelectorListElement final
    : public MediaControlPopupMenuElement {
 public:
  MediaControlTrackSelectorListElement(MediaControlsImpl& media_controls,
                                       bool is_video);

  // Node interface.
  bool WillRespondToMouseClickEvents() final { return true; }

  // MediaControlElementBase interface.
  void SetIsWanted(bool) final;

  // GarbageCollectedMixin interface.
  void Trace(Visitor* visitor) const final;

 private:
  bool is_video_;

  void DefaultEventHandler(Event&) override;

  void RepopulateTrackList();
  Element* CreateListItem(const String& label, bool selected, int index);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_TRACK_SELECTOR_LIST_ELEMENT_H_
