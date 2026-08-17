// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_PAYMENT_REQUEST_UPDATE_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_PAYMENT_REQUEST_UPDATE_EVENT_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_payment_request_update_event_init.h"
#include "third_party/blink/renderer/core/dom/events/event.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/timer.h"

namespace blink {

class ExceptionState;
class ExecutionContext;
class PaymentDetailsUpdate;
class PaymentRequestDelegate;
class ScriptState;

class MODULES_EXPORT PaymentRequestUpdateEvent : public Event,
                                                 public GarbageCollectedMixin {
  DEFINE_WRAPPERTYPEINFO();

 public:
  PaymentRequestUpdateEvent(ExecutionContext*,
                            const AtomicString& type,
                            const PaymentRequestUpdateEventInit*);
  ~PaymentRequestUpdateEvent() override;

  static PaymentRequestUpdateEvent* Create(
      ExecutionContext*,
      const AtomicString& type,
      const PaymentRequestUpdateEventInit* =
          PaymentRequestUpdateEventInit::Create());

  void SetPaymentRequest(PaymentRequestDelegate* request);

  void updateWith(ScriptState*,
                  ScriptPromise<PaymentDetailsUpdate>,
                  ExceptionState&);

  void start_waiting_for_update(bool value) { wait_for_update_ = value; }
  bool is_waiting_for_update() const { return wait_for_update_; }

  void Trace(Visitor*) const override;

 private:
  // True after event.updateWith() was called.
  bool wait_for_update_;

  Member<PaymentRequestDelegate> request_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_PAYMENT_REQUEST_UPDATE_EVENT_H_
