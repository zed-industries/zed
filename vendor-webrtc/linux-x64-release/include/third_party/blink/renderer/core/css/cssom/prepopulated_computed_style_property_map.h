// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_PREPOPULATED_COMPUTED_STYLE_PROPERTY_MAP_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_PREPOPULATED_COMPUTED_STYLE_PROPERTY_MAP_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/cssom/style_property_map_read_only_main_thread.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string_hash.h"

namespace blink {

class ComputedStyle;

// This class has the same behaviour as the ComputedStylePropertyMap, except it
// only contains the properties given to the constructor.
//
// It is to be used with the Houdini APIs (css-paint-api, css-layout-api) which
// require style maps with a subset of properties.
//
// It will pre-populate internal maps (property->CSSValue), as the above APIs
// have a high probability of querying the values inside the map, however as a
// result when the ComputedStyle changes UpdateStyle needs to be called to
// re-populate the internal maps.
class CORE_EXPORT PrepopulatedComputedStylePropertyMap
    : public StylePropertyMapReadOnlyMainThread {
 public:
  // NOTE: styled_node may be null, in the case where this map is for an
  // anonymous box.
  PrepopulatedComputedStylePropertyMap(
      const Document&,
      const ComputedStyle&,
      const Vector<CSSPropertyID>& native_properties,
      const Vector<AtomicString>& custom_properties);
  PrepopulatedComputedStylePropertyMap(
      const PrepopulatedComputedStylePropertyMap&) = delete;
  PrepopulatedComputedStylePropertyMap& operator=(
      const PrepopulatedComputedStylePropertyMap&) = delete;

  // Updates the values of the properties based on the new computed style.
  void UpdateStyle(const Document&, const ComputedStyle&);

  unsigned size() const override;
  void Trace(Visitor*) const override;

 protected:
  const CSSValue* GetProperty(CSSPropertyID) const override;
  const CSSValue* GetCustomProperty(const AtomicString&) const override;
  void ForEachProperty(IterationFunction visitor) override;

  String SerializationForShorthand(const CSSProperty&) const override;

 private:
  void UpdateNativeProperty(const ComputedStyle&, CSSPropertyID);
  void UpdateCustomProperty(const Document&,
                            const ComputedStyle&,
                            const AtomicString& property_name);

  HeapHashMap<CSSPropertyID, Member<const CSSValue>> native_values_;
  HeapHashMap<AtomicString, Member<const CSSValue>> custom_values_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_PREPOPULATED_COMPUTED_STYLE_PROPERTY_MAP_H_
