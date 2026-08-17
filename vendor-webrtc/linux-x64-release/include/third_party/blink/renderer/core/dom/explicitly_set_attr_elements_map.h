// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_EXPLICITLY_SET_ATTR_ELEMENTS_MAP_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_EXPLICITLY_SET_ATTR_ELEMENTS_MAP_H_

#include "third_party/blink/renderer/core/dom/element.h"
#include "third_party/blink/renderer/core/dom/element_rare_data_field.h"
#include "third_party/blink/renderer/core/dom/qualified_name.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"

namespace blink {

// A map of IDL attribute name to Element list value, for one particular
// element. For example,
//   el1.ariaActiveDescendant = el2
// would add the following pair to the ExplicitlySetAttrElementMap for el1:
//   ("ariaActiveDescendant", el2)
// This represents 'explicitly set attr-element' in the HTML specification.
//   https://html.spec.whatwg.org/multipage/common-dom-interfaces.html#explicitly-set-attr-element
// Note that in the interest of simplicity, attributes that reflect a single
// element reference are implemented using the same ExplicitlySetAttrElementsMap
// storage, but only store a single element vector which is DCHECKED at the
// calling site.
class ExplicitlySetAttrElementsMap
    : public GarbageCollected<ExplicitlySetAttrElementsMap>,
      public ElementRareDataField {
 public:
  HeapHashMap<QualifiedName, Member<GCedHeapLinkedHashSet<WeakMember<Element>>>>
      map;

  void Trace(Visitor* visitor) const override {
    ElementRareDataField::Trace(visitor);
    visitor->Trace(map);
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_EXPLICITLY_SET_ATTR_ELEMENTS_MAP_H_
