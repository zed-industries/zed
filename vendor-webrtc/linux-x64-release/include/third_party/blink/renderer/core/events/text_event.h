/*
 * Copyright (C) 2007, 2008 Apple Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_TEXT_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_TEXT_EVENT_H_

#include "third_party/blink/renderer/core/event_interface_names.h"
#include "third_party/blink/renderer/core/events/text_event_input_type.h"
#include "third_party/blink/renderer/core/events/ui_event.h"

namespace blink {

class DocumentFragment;

class TextEvent final : public UIEvent {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static TextEvent* Create();
  static TextEvent* Create(AbstractView*,
                           const String& data,
                           TextEventInputType = kTextEventInputKeyboard);
  static TextEvent* CreateForPlainTextPaste(AbstractView*,
                                            const String& data,
                                            bool should_smart_replace);
  static TextEvent* CreateForFragmentPaste(AbstractView*,
                                           DocumentFragment* data,
                                           bool should_smart_replace,
                                           bool should_match_style);
  static TextEvent* CreateForDrop(AbstractView*, const String& data);

  TextEvent();
  TextEvent(AbstractView*,
            const String& data,
            TextEventInputType = kTextEventInputKeyboard);
  TextEvent(AbstractView*,
            const String& data,
            DocumentFragment*,
            bool should_smart_replace,
            bool should_match_style);
  ~TextEvent() override;

  void initTextEvent(const AtomicString& type,
                     bool bubbles,
                     bool cancelable,
                     AbstractView*,
                     const String& data);

  String data() const { return data_; }

  const AtomicString& InterfaceName() const override;

  bool IsLineBreak() const { return input_type_ == kTextEventInputLineBreak; }
  bool IsComposition() const {
    return input_type_ == kTextEventInputComposition;
  }
  bool IsPaste() const { return input_type_ == kTextEventInputPaste; }
  bool IsDrop() const { return input_type_ == kTextEventInputDrop; }
  bool IsIncrementalInsertion() const {
    return input_type_ == kTextEventInputIncrementalInsertion;
  }

  bool ShouldSmartReplace() const { return should_smart_replace_; }
  bool ShouldMatchStyle() const { return should_match_style_; }
  DocumentFragment* PastingFragment() const { return pasting_fragment_.Get(); }

  void Trace(Visitor*) const override;

 private:
  TextEventInputType input_type_;
  String data_;

  Member<DocumentFragment> pasting_fragment_;
  bool should_smart_replace_;
  bool should_match_style_;
};

inline bool IsTextEvent(const Event& event) {
  return event.type() == event_type_names::kTextInput &&
         event.HasInterface(event_interface_names::kTextEvent);
}

template <>
struct DowncastTraits<TextEvent> {
  static bool AllowFrom(const Event& event) { return IsTextEvent(event); }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_TEXT_EVENT_H_
