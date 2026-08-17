// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_ABORT_PAYMENT_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_ABORT_PAYMENT_EVENT_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/modules/event_modules.h"
#include "third_party/blink/renderer/modules/service_worker/extendable_event.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace WTF {
class AtomicString;
}

namespace blink {

class AbortPaymentRespondWithObserver;
class ExtendableEventInit;
class ScriptState;

class MODULES_EXPORT AbortPaymentEvent final : public ExtendableEvent {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static AbortPaymentEvent* Create(const AtomicString& type,
                                   const ExtendableEventInit*);
  static AbortPaymentEvent* Create(const AtomicString& type,
                                   const ExtendableEventInit*,
                                   AbortPaymentRespondWithObserver*,
                                   WaitUntilObserver*);

  AbortPaymentEvent(const AtomicString& type,
                    const ExtendableEventInit*,
                    AbortPaymentRespondWithObserver*,
                    WaitUntilObserver*);

  AbortPaymentEvent(const AbortPaymentEvent&) = delete;
  AbortPaymentEvent& operator=(const AbortPaymentEvent&) = delete;

  ~AbortPaymentEvent() override;

  const AtomicString& InterfaceName() const override;

  void respondWith(ScriptState*, ScriptPromise<IDLBoolean>, ExceptionState&);

  void Trace(Visitor*) const override;

 private:
  Member<AbortPaymentRespondWithObserver> observer_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_ABORT_PAYMENT_EVENT_H_
