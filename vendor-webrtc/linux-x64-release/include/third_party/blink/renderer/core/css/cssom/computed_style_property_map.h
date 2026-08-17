// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_COMPUTED_STYLE_PROPERTY_MAP_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_COMPUTED_STYLE_PROPERTY_MAP_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_computed_style_declaration.h"
#include "third_party/blink/renderer/core/css/css_selector.h"
#include "third_party/blink/renderer/core/css/cssom/style_property_map_read_only_main_thread.h"
#include "third_party/blink/renderer/core/css/parser/css_selector_parser.h"
#include "third_party/blink/renderer/core/dom/element.h"

namespace blink {

// This class implements computed StylePropertMapReadOnly in the Typed CSSOM
// API. The specification is here:
// https://drafts.css-houdini.org/css-typed-om/#computed-StylePropertyMapReadOnly-objects
//
// The computed StylePropertyMapReadOnlyMainThread retrieves computed styles and
// returns them as CSSStyleValues. The IDL for this class is in
// StylePropertyMap.idl. The computed StylePropertyMapReadOnlyMainThread for an
// element is accessed via element.computedStyleMap() (see
// ElementComputedStyleMap.idl/h)
class CORE_EXPORT ComputedStylePropertyMap
    : public StylePropertyMapReadOnlyMainThread {
 public:
  explicit ComputedStylePropertyMap(Element* element,
                                    const String& pseudo_element = String())
      : pseudo_id_(CSSSelectorParser::ParsePseudoElement(pseudo_element,
                                                         element,
                                                         pseudo_argument_)),
        element_(element) {}
  ComputedStylePropertyMap(const ComputedStylePropertyMap&) = delete;
  ComputedStylePropertyMap& operator=(const ComputedStylePropertyMap&) = delete;

  void Trace(Visitor* visitor) const override {
    visitor->Trace(element_);
    StylePropertyMapReadOnlyMainThread::Trace(visitor);
  }

  unsigned int size() const override;

  // ComputedStylePropertyMap needs to be sorted. This puts CSS properties
  // first, then prefixed properties, then custom properties. Everything is
  // sorted by code point within each category.
  static bool ComparePropertyNames(const CSSPropertyName&,
                                   const CSSPropertyName&);

 protected:
  const CSSValue* GetProperty(CSSPropertyID) const override;
  const CSSValue* GetCustomProperty(const AtomicString&) const override;
  void ForEachProperty(IterationFunction visitor) override;

  String SerializationForShorthand(const CSSProperty&) const final;

 private:
  // TODO: Pseudo-element support requires reintroducing Element.pseudo(...).
  // See
  // https://github.com/w3c/css-houdini-drafts/issues/350#issuecomment-294690156
  PseudoId pseudo_id_;
  AtomicString pseudo_argument_;
  Member<Element> element_;

  Element* StyledElement() const;
  const ComputedStyle* UpdateStyle() const;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_COMPUTED_STYLE_PROPERTY_MAP_H_
