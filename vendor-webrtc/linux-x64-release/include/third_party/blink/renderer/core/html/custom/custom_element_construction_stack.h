// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CUSTOM_CUSTOM_ELEMENT_CONSTRUCTION_STACK_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CUSTOM_CUSTOM_ELEMENT_CONSTRUCTION_STACK_H_

#include "third_party/blink/renderer/bindings/core/v8/v8_custom_element_constructor_hash.h"
#include "third_party/blink/renderer/core/dom/element.h"
#include "third_party/blink/renderer/core/html/custom/custom_element_definition.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"

namespace blink {

class LocalDOMWindow;

// https://html.spec.whatwg.org/multipage/custom-elements.html#concept-custom-element-definition-construction-stack
// To support scoped custom element registries, we modify it such that there's a
// stack corresponding to each (window, custom element constructor) pair. Since
// this is 1:1 to definitions, there's no behavioral change. The benefit is that
// when scoped registries are enabled, we can check the construction stack to
// find out which definition to use, instead of always looking up the global
// registry.

struct CustomElementConstructionStackEntry {
  DISALLOW_NEW();

  CustomElementConstructionStackEntry() = default;
  CustomElementConstructionStackEntry(Element& element,
                                      CustomElementDefinition& definition)
      : element(element), definition(definition) {}

  Member<Element> element;
  Member<CustomElementDefinition> definition;

  void Trace(Visitor* visitor) const {
    visitor->Trace(element);
    visitor->Trace(definition);
  }
};

}  // namespace blink

WTF_ALLOW_CLEAR_UNUSED_SLOTS_WITH_MEM_FUNCTIONS(
    blink::CustomElementConstructionStackEntry)

namespace blink {

using CustomElementConstructionStack =
    GCedHeapVector<CustomElementConstructionStackEntry, 1>;

// Returns the construction stack associated with the construction in the
// window. Nullptr return value means the stack is empty (when the memory
// backing isn't created yet).
CORE_EXPORT CustomElementConstructionStack* GetCustomElementConstructionStack(
    const LocalDOMWindow* window,
    v8::Local<v8::Object> constructor);

// Pushes the construction stack of the constructor of a definition when
// entering the scope, and pops it when exiting. Helper class for manipulating
// the construction stacks.
class CORE_EXPORT CustomElementConstructionStackScope final {
  STACK_ALLOCATED();

 public:
  CustomElementConstructionStackScope(CustomElementDefinition&, Element&);
  CustomElementConstructionStackScope(
      const CustomElementConstructionStackScope&) = delete;
  CustomElementConstructionStackScope& operator=(
      const CustomElementConstructionStackScope&) = delete;
  ~CustomElementConstructionStackScope();

 private:
  CustomElementConstructionStack& construction_stack_;
#if DCHECK_IS_ON()
  Element* element_;
  wtf_size_t depth_;
#endif

  static wtf_size_t nesting_level_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CUSTOM_CUSTOM_ELEMENT_CONSTRUCTION_STACK_H_
