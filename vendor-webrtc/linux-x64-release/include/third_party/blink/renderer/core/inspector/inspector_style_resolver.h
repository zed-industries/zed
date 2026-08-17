// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_STYLE_RESOLVER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_STYLE_RESOLVER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_rule.h"
#include "third_party/blink/renderer/core/css/css_rule_list.h"
#include "third_party/blink/renderer/core/css/css_style_declaration.h"
#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/core/css/properties/css_property.h"
#include "third_party/blink/renderer/core/style/computed_style_constants.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

class Element;

// Contains matched rules for an element.
struct CORE_EXPORT InspectorCSSMatchedRules
    : public GarbageCollected<InspectorCSSMatchedRules> {
 public:
  Member<Element> element;
  Member<RuleIndexList> matched_rules;
  PseudoId pseudo_id;
  AtomicString view_transition_name = g_null_atom;

  void Trace(Visitor* visitor) const {
    visitor->Trace(element);
    visitor->Trace(matched_rules);
  }
};

// Contains matched pseudos for an element.
struct CORE_EXPORT InspectorCSSMatchedPseudoElements
    : public GarbageCollected<InspectorCSSMatchedPseudoElements> {
 public:
  Member<Element> element;
  HeapVector<Member<InspectorCSSMatchedRules>> pseudo_element_rules;

  void Trace(Visitor* visitor) const {
    visitor->Trace(element);
    visitor->Trace(pseudo_element_rules);
  }
};

// Resolves style rules for an element.
class CORE_EXPORT InspectorStyleResolver {
  STACK_ALLOCATED();

 public:
  InspectorStyleResolver(Element*,
                         PseudoId,
                         const AtomicString& view_transition_name);
  RuleIndexList* MatchedRules() const;
  HeapVector<Member<InspectorCSSMatchedRules>> PseudoElementRules();
  HeapVector<Member<InspectorCSSMatchedRules>> ParentRules();
  HeapVector<Member<InspectorCSSMatchedPseudoElements>>
  ParentPseudoElementRules();

 private:
  void AddPseudoElementRules(PseudoId pseudo_id,
                             const AtomicString& view_transition_name);

  Element* element_;
  RuleIndexList* matched_rules_;
  HeapVector<Member<InspectorCSSMatchedRules>> parent_rules_;
  HeapVector<Member<InspectorCSSMatchedRules>> pseudo_element_rules_;
  HeapVector<Member<InspectorCSSMatchedPseudoElements>>
      parent_pseudo_element_rules_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_STYLE_RESOLVER_H_
