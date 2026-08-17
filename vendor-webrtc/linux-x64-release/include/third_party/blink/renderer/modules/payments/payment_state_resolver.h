// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_PAYMENT_STATE_RESOLVER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_PAYMENT_STATE_RESOLVER_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class PaymentValidationErrors;
class ScriptState;

class MODULES_EXPORT PaymentStateResolver : public GarbageCollectedMixin {
 public:
  enum PaymentComplete { kFail, kSuccess, kUnknown };

  virtual ScriptPromise<IDLUndefined> Complete(ScriptState*,
                                               PaymentComplete result,
                                               ExceptionState&) = 0;
  virtual ScriptPromise<IDLUndefined> Retry(ScriptState*,
                                            const PaymentValidationErrors*,
                                            ExceptionState&) = 0;

 protected:
  virtual ~PaymentStateResolver() = default;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_PAYMENT_STATE_RESOLVER_H_
