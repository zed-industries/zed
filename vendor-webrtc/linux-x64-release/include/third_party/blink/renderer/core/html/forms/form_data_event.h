// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_FORM_DATA_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_FORM_DATA_EVENT_H_

#include "third_party/blink/renderer/core/dom/events/event.h"

namespace blink {

class FormData;
class FormDataEventInit;

class FormDataEvent : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static FormDataEvent* Create(FormData& form_data);
  static FormDataEvent* Create(const AtomicString& type,
                               const FormDataEventInit* event_init);
  FormDataEvent(FormData& form_data);
  FormDataEvent(const AtomicString& type, const FormDataEventInit* event_init);

  void Trace(Visitor* visitor) const override;

  FormData* formData() const { return form_data_.Get(); }

  const AtomicString& InterfaceName() const override;

 private:
  Member<FormData> form_data_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_FORM_DATA_EVENT_H_
