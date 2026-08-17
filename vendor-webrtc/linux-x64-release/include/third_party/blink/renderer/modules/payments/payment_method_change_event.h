// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_PAYMENT_METHOD_CHANGE_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_PAYMENT_METHOD_CHANGE_EVENT_H_

#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/bindings/core/v8/world_safe_v8_reference.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_payment_method_change_event_init.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/payments/payment_request_update_event.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class ScriptState;

class MODULES_EXPORT PaymentMethodChangeEvent final
    : public PaymentRequestUpdateEvent {
  DEFINE_WRAPPERTYPEINFO();

 public:
  ~PaymentMethodChangeEvent() override;

  static PaymentMethodChangeEvent* Create(ScriptState* script_state,
                                          const AtomicString& type) {
    return Create(
        script_state, type,
        PaymentMethodChangeEventInit::Create(script_state->GetIsolate()));
  }
  static PaymentMethodChangeEvent* Create(ScriptState*,
                                          const AtomicString& type,
                                          const PaymentMethodChangeEventInit*);

  const String& methodName() const;
  const ScriptObject& methodDetails() const;

  PaymentMethodChangeEvent(ScriptState*,
                           const AtomicString& type,
                           const PaymentMethodChangeEventInit*);

  void Trace(Visitor* visitor) const override;

 private:
  String method_name_;
  const ScriptObject method_details_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_PAYMENT_METHOD_CHANGE_EVENT_H_
