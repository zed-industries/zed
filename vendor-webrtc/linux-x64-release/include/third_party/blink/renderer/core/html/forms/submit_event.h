// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_SUBMIT_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_SUBMIT_EVENT_H_

#include "third_party/blink/renderer/core/dom/events/event.h"

namespace blink {

class HTMLElement;
class SubmitEventInit;

class SubmitEvent : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static SubmitEvent* Create(const AtomicString& type,
                             const SubmitEventInit* event_init);
  SubmitEvent(const AtomicString& type, const SubmitEventInit* event_init);

  void Trace(Visitor* visitor) const override;
  HTMLElement* submitter() const { return submitter_.Get(); }
  const AtomicString& InterfaceName() const override;

 private:
  Member<HTMLElement> submitter_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_SUBMIT_EVENT_H_
