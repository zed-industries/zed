// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_ANCHOR_ELEMENT_OBSERVER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_ANCHOR_ELEMENT_OBSERVER_H_

#include "third_party/blink/renderer/core/dom/element_rare_data_field.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class Element;
class IdTargetObserver;

// Tracks the value of Element::anchorElement() to help other elements know
// whether they are used as implicit anchor elements.
// NOTE: this class is unrelated to the <a> element.
class AnchorElementObserver : public GarbageCollected<AnchorElementObserver>,
                              public ElementRareDataField {
 public:
  // This observer is placed on an element (the "source" element) that has
  // the anchor attribute. The observer maintains the
  // "ImplicitlyAnchoredElementCount" on the target element.
  explicit AnchorElementObserver(Element* source_element)
      : source_element_(source_element) {
    DCHECK(source_element_);
  }
  const Element& GetSourceElement() const { return *source_element_; }

  void Notify();

  void Trace(Visitor* visitor) const override;

 private:
  void ResetIdTargetObserverIfNeeded();

  Member<Element> source_element_;
  Member<Element> current_anchor_;
  Member<IdTargetObserver> id_target_observer_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_ANCHOR_ELEMENT_OBSERVER_H_
