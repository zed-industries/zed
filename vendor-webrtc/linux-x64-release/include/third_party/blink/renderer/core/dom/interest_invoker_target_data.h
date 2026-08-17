// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_INTEREST_INVOKER_TARGET_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_INTEREST_INVOKER_TARGET_DATA_H_

#include "base/check_op.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/core/dom/element.h"
#include "third_party/blink/renderer/core/dom/element_rare_data_field.h"
#include "third_party/blink/renderer/core/html/closewatcher/close_watcher.h"
#include "third_party/blink/renderer/core/html/forms/html_form_control_element.h"
#include "third_party/blink/renderer/core/html_element_type_helpers.h"
#include "third_party/blink/renderer/core/inspector/console_message.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

// The InterestInvokerTargetData class stores information that is needed when
// the Element it is attached to becomes the target of an interest invoker.
class InterestInvokerTargetData final
    : public GarbageCollected<InterestInvokerTargetData>,
      public ElementRareDataField {
 public:
  InterestInvokerTargetData() = default;
  InterestInvokerTargetData(const InterestInvokerTargetData&) = delete;
  InterestInvokerTargetData& operator=(const InterestInvokerTargetData&) =
      delete;

  Element* interestInvoker() { return interest_invoker_; }

  void setInterestInvoker(Element* invoker) {
    DCHECK(!invoker ||
           RuntimeEnabledFeatures::HTMLInterestTargetAttributeEnabled(
               invoker->GetDocument().GetExecutionContext()));
    interest_invoker_ = invoker;
  }

  void Trace(Visitor* visitor) const override {
    visitor->Trace(interest_invoker_);
    ElementRareDataField::Trace(visitor);
  }

 private:
  Member<Element> interest_invoker_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_INTEREST_INVOKER_TARGET_DATA_H_
