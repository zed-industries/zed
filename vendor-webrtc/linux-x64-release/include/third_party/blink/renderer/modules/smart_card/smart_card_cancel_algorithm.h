// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SMART_CARD_SMART_CARD_CANCEL_ALGORITHM_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SMART_CARD_SMART_CARD_CANCEL_ALGORITHM_H_

#include "third_party/blink/renderer/core/dom/abort_signal.h"

namespace blink {

class SmartCardContext;

class SmartCardCancelAlgorithm final : public AbortSignal::Algorithm {
 public:
  explicit SmartCardCancelAlgorithm(SmartCardContext* blink_scard_context);
  ~SmartCardCancelAlgorithm() override;

  void Run() override;

  void Trace(Visitor* visitor) const override;

 private:
  Member<SmartCardContext> blink_scard_context_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SMART_CARD_SMART_CARD_CANCEL_ALGORITHM_H_
