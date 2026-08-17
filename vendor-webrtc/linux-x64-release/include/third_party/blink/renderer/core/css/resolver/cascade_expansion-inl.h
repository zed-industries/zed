// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_CASCADE_EXPANSION_INL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_CASCADE_EXPANSION_INL_H_

// Implementation of cascade_expansion.h.

#include "third_party/blink/renderer/core/css/resolver/cascade_expansion.h"

#include "third_party/blink/renderer/core/css/properties/css_property.h"
#include "third_party/blink/renderer/core/css/property_bitsets.h"
#include "third_party/blink/renderer/core/css/resolver/match_result.h"

namespace blink {

template <class CustomPropertyCallback, class RegularPropertyCallback>
void ExpandCascade(const MatchedProperties& matched_properties,
                   const Document& document,
                   wtf_size_t matched_properties_index,
                   CustomPropertyCallback&& custom_property_callback,
                   RegularPropertyCallback&& regular_property_callback) {
  CascadeFilter filter = CreateExpansionFilter(matched_properties);

  // We can't handle a MatchResult with more than 0xFFFF MatchedProperties,
  // or a MatchedProperties object with more than 0xFFFF declarations. If this
  // happens, we skip right to the end, and emit nothing.
  base::span<const CSSPropertyValue> properties =
      matched_properties.properties->Properties();
  if (properties.size() > kMaxDeclarationIndex + 1 ||
      matched_properties_index > kMaxMatchedPropertiesIndex) {
    return;
  }

  const bool expand_visited = !filter.Rejects(CSSProperty::kVisited, true);

  unsigned property_idx = 0;
  for (const CSSPropertyValue& reference : properties) {
    CSSPropertyID id = reference.PropertyID();
    CascadePriority priority(
        matched_properties.data_.origin, reference.IsImportant(),
        matched_properties.data_.tree_order,
        matched_properties.data_.is_inline_style,
        matched_properties.data_.is_try_style,
        matched_properties.data_.is_try_tactics_style,
        matched_properties.data_.layer_order,
        EncodeMatchResultPosition(matched_properties_index, property_idx++));

    if (id == CSSPropertyID::kVariable) {
      CustomProperty custom(reference.CustomPropertyName(), document);
      if (!filter.Rejects(custom)) {
        custom_property_callback(priority, reference.CustomPropertyName());
      }
      // Custom properties never have visited counterparts,
      // so no need to check for expand_visited here.
    } else if (id == CSSPropertyID::kAll) {
      for (int i = kIntFirstCSSProperty; i <= kIntLastCSSProperty; ++i) {
        CSSPropertyID expanded_id = ConvertToCSSPropertyID(i);
        if (!IsInAllExpansion(expanded_id)) {
          continue;
        }
        const CSSProperty& property = CSSProperty::Get(expanded_id);
        if (!filter.Rejects(property)) {
          regular_property_callback(priority, expanded_id);
        }
      }
    } else {
      const CSSProperty& property = CSSProperty::Get(id);
      if (!filter.Rejects(property)) {
        regular_property_callback(priority, id);
      }
      if (expand_visited) {
        const CSSProperty* visited_property = property.GetVisitedProperty();
        if (visited_property && !filter.Rejects(*visited_property)) {
          regular_property_callback(priority, visited_property->PropertyID());
        }
      }
    }
  }
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_CASCADE_EXPANSION_INL_H_
