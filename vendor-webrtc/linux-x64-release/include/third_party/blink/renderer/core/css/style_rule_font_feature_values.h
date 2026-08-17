// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_RULE_FONT_FEATURE_VALUES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_RULE_FONT_FEATURE_VALUES_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/style_rule.h"

namespace blink {

struct FeatureIndicesWithPriority {
  Vector<uint32_t> indices;
  uint16_t layer_order = std::numeric_limits<uint16_t>::max();
};

using FontFeatureAliases = HashMap<AtomicString, FeatureIndicesWithPriority>;

class CORE_EXPORT StyleRuleFontFeature : public StyleRuleBase {
 public:
  enum class FeatureType {
    kStylistic,
    kStyleset,
    kCharacterVariant,
    kSwash,
    kOrnaments,
    kAnnotation
  };

  explicit StyleRuleFontFeature(FeatureType);
  StyleRuleFontFeature(const StyleRuleFontFeature&);
  ~StyleRuleFontFeature();

  void UpdateAlias(AtomicString alias, Vector<uint32_t> features);
  void OverrideAliasesIn(FontFeatureAliases& destination);

  FeatureType GetFeatureType() { return type_; }

  void TraceAfterDispatch(blink::Visitor*) const;

 private:
  FeatureType type_;
  FontFeatureAliases feature_aliases_;
};

template <>
struct DowncastTraits<StyleRuleFontFeature> {
  static bool AllowFrom(const StyleRuleBase& rule) {
    return rule.IsFontFeatureRule();
  }
};

class CORE_EXPORT FontFeatureValuesStorage {
 public:
  FontFeatureValuesStorage(FontFeatureAliases stylistic,
                           FontFeatureAliases styleset,
                           FontFeatureAliases character_variant,
                           FontFeatureAliases swash,
                           FontFeatureAliases ornaments,
                           FontFeatureAliases annotation);

  FontFeatureValuesStorage() = default;
  FontFeatureValuesStorage(const FontFeatureValuesStorage& other) = default;

  FontFeatureValuesStorage& operator=(const FontFeatureValuesStorage& other) =
      default;

  Vector<uint32_t> ResolveStylistic(const AtomicString&) const;
  Vector<uint32_t> ResolveStyleset(const AtomicString&) const;
  Vector<uint32_t> ResolveCharacterVariant(const AtomicString&) const;
  Vector<uint32_t> ResolveSwash(const AtomicString&) const;
  Vector<uint32_t> ResolveOrnaments(const AtomicString&) const;
  Vector<uint32_t> ResolveAnnotation(const AtomicString&) const;

  void SetLayerOrder(uint16_t layer_order);

  // Update and extend this FontFeatureValuesStorage with information from
  // `other`. Intended to be used for fusing multiple at-rules in a document and
  // across cascade layers so that their maps became unified, compare
  // https://drafts.csswg.org/css-fonts-4/#font-feature-values-syntax: If
  // multiple @font-feature-values rules are defined for a given family, the
  // resulting values definitions are the union of the definitions contained
  // within these rules. If `other` is passed in with a higher `layer_order`,
  // existing alias keys are overridden with the values from `other`.
  void FuseUpdate(const FontFeatureValuesStorage& other, unsigned layer_order);

 private:
  // TODO(https://crbug.com/716567): Only styleset and character variant take
  // two values for each alias, the others take 1 value. Consider reducing
  // storage here.
  FontFeatureAliases stylistic_;
  FontFeatureAliases styleset_;
  FontFeatureAliases character_variant_;
  FontFeatureAliases swash_;
  FontFeatureAliases ornaments_;
  FontFeatureAliases annotation_;
  static Vector<uint32_t> ResolveInternal(const FontFeatureAliases&,
                                          const AtomicString&);

  friend class StyleRuleFontFeatureValues;
};

class CORE_EXPORT StyleRuleFontFeatureValues : public StyleRuleBase {
 public:
  StyleRuleFontFeatureValues(Vector<AtomicString> families,
                             FontFeatureAliases stylistic,
                             FontFeatureAliases styleset,
                             FontFeatureAliases character_variant,
                             FontFeatureAliases swash,
                             FontFeatureAliases ornaments,
                             FontFeatureAliases annotation);
  StyleRuleFontFeatureValues(const StyleRuleFontFeatureValues&);
  ~StyleRuleFontFeatureValues();

  const Vector<AtomicString>& GetFamilies() const { return families_; }
  String FamilyAsString() const;

  void SetFamilies(Vector<AtomicString>);

  StyleRuleFontFeatureValues* Copy() const {
    return MakeGarbageCollected<StyleRuleFontFeatureValues>(*this);
  }

  const FontFeatureValuesStorage& Storage() { return feature_values_storage_; }

  // Accessors needed for cssom implementation.
  FontFeatureAliases* GetStylistic() {
    return &feature_values_storage_.stylistic_;
  }
  FontFeatureAliases* GetStyleset() {
    return &feature_values_storage_.styleset_;
  }
  FontFeatureAliases* GetCharacterVariant() {
    return &feature_values_storage_.character_variant_;
  }
  FontFeatureAliases* GetSwash() { return &feature_values_storage_.swash_; }
  FontFeatureAliases* GetOrnaments() {
    return &feature_values_storage_.ornaments_;
  }
  FontFeatureAliases* GetAnnotation() {
    return &feature_values_storage_.annotation_;
  }

  void SetCascadeLayer(const CascadeLayer* layer) { layer_ = layer; }
  const CascadeLayer* GetCascadeLayer() const { return layer_.Get(); }

  void TraceAfterDispatch(blink::Visitor*) const;

 private:
  Vector<AtomicString> families_;
  FontFeatureValuesStorage feature_values_storage_;
  Member<const CascadeLayer> layer_;
};

template <>
struct DowncastTraits<StyleRuleFontFeatureValues> {
  static bool AllowFrom(const StyleRuleBase& rule) {
    return rule.IsFontFeatureValuesRule();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_RULE_FONT_FEATURE_VALUES_H_
