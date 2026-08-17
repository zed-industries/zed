// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_IME_TEXT_FORMAT_UPDATE_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_IME_TEXT_FORMAT_UPDATE_EVENT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/events/event.h"

namespace blink {

class TextFormatUpdateEventInit;
class TextFormat;

// The textformatupdate event is fired when the input method desires a specific
// region to be styled in a certain fashion, limited to the style properties
// that correspond with the properties that are exposed on TextFormatUpdateEvent
// (e.g. backgroundColor, textDecoration, etc.). The consumer of the EditContext
// should update their view accordingly to provide the user with visual feedback
// as prescribed by the software keyboard. Note that this may have accessibility
// implications, as the IME may not be aware of the color scheme of the editable
// contents (i.e. may be requesting blue highlight on text that was already
// blue).
class CORE_EXPORT TextFormatUpdateEvent final : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  TextFormatUpdateEvent(const AtomicString& type,
                        const TextFormatUpdateEventInit* initializer);
  TextFormatUpdateEvent(const AtomicString& type,
                        HeapVector<Member<TextFormat>>& textFormats);
  static TextFormatUpdateEvent* Create(
      const AtomicString& type,
      const TextFormatUpdateEventInit* initializer);
  ~TextFormatUpdateEvent() override;

  HeapVector<Member<TextFormat>> getTextFormats() const;

  const AtomicString& InterfaceName() const override;

  void Trace(Visitor*) const override;

 private:
  HeapVector<Member<TextFormat>> text_formats_;
};

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_IME_TEXT_FORMAT_UPDATE_EVENT_H_
