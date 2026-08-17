// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_ABORT_PAYMENT_RESPOND_WITH_OBSERVER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_ABORT_PAYMENT_RESPOND_WITH_OBSERVER_H_

#include "third_party/blink/public/mojom/service_worker/service_worker_error_type.mojom-blink-forward.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/service_worker/respond_with_observer.h"

namespace blink {

class ExecutionContext;
class WaitUntilObserver;

// Implementation for AbortPaymentEvent.respondWith(), which is used by the
// payment handler to indicate whether it was able to abort the payment.
class MODULES_EXPORT AbortPaymentRespondWithObserver final
    : public RespondWithObserver {
 public:
  AbortPaymentRespondWithObserver(ExecutionContext*,
                                  int event_id,
                                  WaitUntilObserver*);
  ~AbortPaymentRespondWithObserver() override = default;

  void OnResponseRejected(mojom::ServiceWorkerResponseError) override;
  void OnResponseFulfilled(ScriptState*, bool);
  void OnNoResponse(ScriptState*) override;

  void Trace(Visitor*) const override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_ABORT_PAYMENT_RESPOND_WITH_OBSERVER_H_
