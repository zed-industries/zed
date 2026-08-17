// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_IME_TEXT_UPDATE_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_IME_TEXT_UPDATE_EVENT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/events/event.h"

namespace blink {

class TextUpdateEventInit;
class TextUpdateEvent;

// The textupdate event will be fired on the EditContext when user input has
// resulted in characters being applied to the editable region. The event
// signals the fact that the software keyboard or IME updated the text (and as
// such that state is reflected in the shared buffer at the time the event is
// fired). This can be a single character update, in the case of typical typing
// scenarios, or multiple-character insertion based on the user changing
// composition candidates. Even though text updates are the results of the
// software keyboard modifying the buffer, the creator of the EditContext is
// ultimately responsible for keeping its underlying model up-to-date with the
// content that is being edited as well as telling the EditContext about such
// changes. These could get out of sync, for example, when updates to the
// editable content come in through other means (the backspace key is a
// canonical example — no textupdate is fired in this case, and the consumer of
// the EditContext should detect the keydown event and remove characters as
// appropriate).
class CORE_EXPORT TextUpdateEvent final : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  TextUpdateEvent(const AtomicString& type,
                  const TextUpdateEventInit* initializer);
  TextUpdateEvent(const AtomicString& type,
                  const String& update_text,
                  uint32_t update_range_start,
                  uint32_t update_range_end,
                  uint32_t selection_start,
                  uint32_t selection_end);
  static TextUpdateEvent* Create(const AtomicString& type,
                                 const TextUpdateEventInit* initializer);
  ~TextUpdateEvent() override;

  String text() const;
  uint32_t updateRangeStart() const;
  uint32_t updateRangeEnd() const;
  uint32_t selectionStart() const;
  uint32_t selectionEnd() const;

  const AtomicString& InterfaceName() const override;
  // member variables to keep track of the event parameters
 private:
  String text_;
  uint32_t update_range_start_ = 0;
  uint32_t update_range_end_ = 0;
  uint32_t selection_start_ = 0;
  uint32_t selection_end_ = 0;
};

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_IME_TEXT_UPDATE_EVENT_H_
