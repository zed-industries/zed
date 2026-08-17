// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_PAYMENT_REQUEST_DELEGATE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_PAYMENT_REQUEST_DELEGATE_H_

#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {
class PaymentDetailsUpdate;

// The interface for updating the payment details (shopping cart, shipping
// options, total) in response to shipping address or option change, or through
// the promise passed into the PaymentRequest.show() method.
class MODULES_EXPORT PaymentRequestDelegate : public GarbageCollectedMixin {
 public:
  // Updates the payment details in response to a change in, e.g., shipping
  // address. This stops the spinner in the UI.
  virtual void OnUpdatePaymentDetails(PaymentDetailsUpdate*) = 0;

  // Called when the merchant failed to update the payment details in response
  // to a change in, e.g., shipping address. This will abort the payment.
  virtual void OnUpdatePaymentDetailsFailure(const String& error) = 0;

  // Whether the PaymentRequest.show() or PaymentResponse.retry() is ongoing.
  virtual bool IsInteractive() const = 0;

 protected:
  virtual ~PaymentRequestDelegate() = default;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_PAYMENT_REQUEST_DELEGATE_H_
