// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_FONT_FEATURE_VALUES_MAP_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_FONT_FEATURE_VALUES_MAP_H_

#include "third_party/blink/renderer/bindings/core/v8/idl_types.h"
#include "third_party/blink/renderer/bindings/core/v8/iterable.h"
#include "third_party/blink/renderer/bindings/core/v8/maplike.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_sync_iterator_css_font_feature_values_map.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_union_unsignedlong_unsignedlongsequence.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_function_value.h"
#include "third_party/blink/renderer/core/css/style_rule_font_feature_values.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class CSSFontFeatureValuesRule;

using FeatureValuesMaplike = Maplike<CSSFontFeatureValuesMap>;

class CSSFontFeatureValuesMap : public ScriptWrappable,
                                public FeatureValuesMaplike {
  DEFINE_WRAPPERTYPEINFO();

 public:
  CSSFontFeatureValuesMap(
      CSSFontFeatureValuesRule* parent_rule,
      StyleRuleFontFeatureValues* style_rule_font_feature_values,
      FontFeatureAliases* aliases)
      : parent_rule_(parent_rule),
        backing_style_rule_(style_rule_font_feature_values),
        aliases_(aliases) {}

  CSSFontFeatureValuesMap(const CSSFontFeatureValuesMap&) = delete;
  CSSFontFeatureValuesMap& operator=(const CSSFontFeatureValuesMap&) = delete;

  // IDL attributes / methods
  uint32_t size() const;

  CSSFontFeatureValuesMap* set(
      const String& key,
      V8UnionUnsignedLongOrUnsignedLongSequence* value);
  void clearForBinding(ScriptState*, ExceptionState&);
  bool deleteForBinding(ScriptState*, const String&, ExceptionState&);

  void Trace(Visitor* visitor) const override {
    ScriptWrappable::Trace(visitor);
    visitor->Trace(parent_rule_);
    visitor->Trace(backing_style_rule_);
  }

 private:
  CSSFontFeatureValuesMap() = default;

  PairSyncIterable<CSSFontFeatureValuesMap>::IterationSource*
  CreateIterationSource(ScriptState*, ExceptionState&) override;
  bool GetMapEntry(ScriptState*,
                   const String& key,
                   Vector<uint32_t>& value,
                   ExceptionState&) override;

  Member<CSSFontFeatureValuesRule> parent_rule_;
  Member<StyleRuleFontFeatureValues> backing_style_rule_;
  FontFeatureAliases* aliases_ = nullptr;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_FONT_FEATURE_VALUES_MAP_H_
