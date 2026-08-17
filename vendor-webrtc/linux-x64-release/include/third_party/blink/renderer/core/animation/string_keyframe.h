// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_STRING_KEYFRAME_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_STRING_KEYFRAME_H_

#include "base/containers/span.h"
#include "third_party/blink/renderer/core/animation/keyframe.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_property_value_set.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/text/writing_direction_mode.h"

namespace blink {

class CSSPropertyName;
class StyleSheetContents;

// An implementation of Keyframe used for CSS Animations.
//
// A StringKeyframe instance supports an arbitrary number of (CSS property,
// value) pairs. CSS properties added to a StringKeyframe are expanded to
// shorthand and de-duplicated, with newer properties replacing older ones.
//
class CORE_EXPORT StringKeyframe : public Keyframe {
 public:
  class PropertyIterator : public VirtualPropertyIterator {
   public:
    explicit PropertyIterator(const StringKeyframe* keyframe);
    ~PropertyIterator() override = default;
    void Advance(const Keyframe* keyframe) override;
    PropertyHandle Deref(const Keyframe* keyframe) const override;
    bool AtEnd(const Keyframe* keyframe) const override;

   private:
    base::span<const CSSPropertyValue> css_properties_;
  };

  class CORE_EXPORT IterableStringKeyframeProperties
      : public Keyframe::IterableProperties {
   public:
    explicit IterableStringKeyframeProperties(const StringKeyframe* keyframe)
        : keyframe_(keyframe) {}
    ~IterableStringKeyframeProperties() override = default;
    PropertyIteratorWrapper begin() const override;
    size_t size() const override;

    void Trace(Visitor* visitor) const override {
      Keyframe::IterableProperties::Trace(visitor);
      visitor->Trace(keyframe_);
    }

   private:
    Member<const StringKeyframe> keyframe_;
  };

  explicit StringKeyframe(const TreeScope* tree_scope = nullptr)
      : Keyframe(MakeGarbageCollected<IterableStringKeyframeProperties>(this)),
        tree_scope_(tree_scope) {}
  StringKeyframe(const StringKeyframe& copy_from);

  MutableCSSPropertyValueSet::SetResult SetCSSPropertyValue(
      const AtomicString& custom_property_name,
      const String& value,
      SecureContextMode,
      StyleSheetContents*);
  MutableCSSPropertyValueSet::SetResult SetCSSPropertyValue(
      CSSPropertyID,
      const String& value,
      SecureContextMode,
      StyleSheetContents*);
  void SetCSSPropertyValue(const CSSPropertyName&, const CSSValue&);
  void RemoveCustomCSSProperty(const PropertyHandle& property);

  const CSSValue& CssPropertyValue(const PropertyHandle& property) const {
    EnsureCssPropertyMap();
    int index = -1;
    if (property.IsCSSCustomProperty()) {
      index =
          css_property_map_->FindPropertyIndex(property.CustomPropertyName());
    } else {
      DCHECK(!property.GetCSSProperty().IsShorthand());
      index = css_property_map_->FindPropertyIndex(
          property.GetCSSProperty().PropertyID());
    }
    CHECK_GE(index, 0);
    return css_property_map_->PropertyAt(static_cast<unsigned>(index))
        .Value()
        .EnsureScopedValue(tree_scope_.Get());
  }

  void AddKeyframePropertiesToV8Object(V8ObjectBuilder&,
                                       Element*) const override;

  Keyframe* Clone() const override;

  bool HasLogicalProperty() { return has_logical_property_; }

  bool SetLogicalPropertyResolutionContext(
      WritingDirectionMode writing_direction);

  void Trace(Visitor*) const override;

  class CSSPropertySpecificKeyframe
      : public Keyframe::PropertySpecificKeyframe {
   public:
    CSSPropertySpecificKeyframe(double offset,
                                scoped_refptr<TimingFunction> easing,
                                const CSSValue* value,
                                const TreeScope* tree_scope,
                                EffectModel::CompositeOperation composite)
        : Keyframe::PropertySpecificKeyframe(offset,
                                             std::move(easing),
                                             composite),
          value_(value),
          tree_scope_(tree_scope) {}

    const CSSValue* Value() const { return value_.Get(); }

    // The originating TreeScope for this keyframe. Note that certain
    // values also bake the TreeScope into their value (see CSSValue::
    // EnsureScopedValue); this is needed when need to represent a mix
    // of two interpolable values that originate from two different tree
    // scopes.
    //
    // CSSUnparsedDeclarationValue does *not* bake the TreeScope into
    // its value, however, since it's somewhat expensive, and we never
    // need to represent a mix of such values.
    const TreeScope* GetTreeScope() const { return tree_scope_.Get(); }

    bool PopulateCompositorKeyframeValue(
        const PropertyHandle&,
        Element&,
        const ComputedStyle& base_style,
        const ComputedStyle* parent_style) const final;
    const CompositorKeyframeValue* GetCompositorKeyframeValue() const final {
      return compositor_keyframe_value_cache_.Get();
    }

    bool IsNeutral() const final { return !value_; }
    bool IsRevert() const final;
    bool IsRevertLayer() const final;
    Keyframe::PropertySpecificKeyframe* NeutralKeyframe(
        double offset,
        scoped_refptr<TimingFunction> easing) const final;

    void Trace(Visitor*) const override;

   private:
    bool IsCSSPropertySpecificKeyframe() const override { return true; }

    Member<const CSSValue> value_;
    Member<const TreeScope> tree_scope_;
    mutable Member<CompositorKeyframeValue> compositor_keyframe_value_cache_;
  };

  class PropertyResolver : public GarbageCollected<PropertyResolver> {
   public:
    // Custom properties must use this version of the constructor.
    PropertyResolver(CSSPropertyID property_id, const CSSValue& css_value);

    // Shorthand and logical properties must use this version of the
    // constructor.
    PropertyResolver(const CSSProperty& property,
                     const MutableCSSPropertyValueSet* property_value_set,
                     bool is_logical);

    bool IsValid() const;

    const CSSValue* CssValue();

    void AppendTo(MutableCSSPropertyValueSet* property_value_set,
                  WritingDirectionMode writing_direction);

    void SetProperty(MutableCSSPropertyValueSet* property_value_set,
                     CSSPropertyID property_id,
                     const CSSValue& value,
                     WritingDirectionMode writing_direction);

    static bool HasLowerPriority(PropertyResolver* first,
                                 PropertyResolver* second);

    // Helper methods for resolving longhand name collisions.
    // Longhands take priority over shorthands.
    // Physical properties take priority over logical.
    // Two shorthands with overlapping longhand properties are sorted based
    // on the number of longhand properties in their expansions.
    bool IsLogical() { return is_logical_; }
    bool IsShorthand() { return css_property_value_set_ != nullptr; }
    unsigned ExpansionCount() {
      return css_property_value_set_ ? css_property_value_set_->PropertyCount()
                                     : 1;
    }

    void Trace(Visitor* visitor) const;

   private:
    CSSPropertyID property_id_ = CSSPropertyID::kInvalid;
    Member<const CSSValue> css_value_ = nullptr;
    Member<ImmutableCSSPropertyValueSet> css_property_value_set_ = nullptr;
    bool is_logical_ = false;
  };

 private:
  friend class PropertyIterator;

  Keyframe::PropertySpecificKeyframe* CreatePropertySpecificKeyframe(
      const PropertyHandle&,
      EffectModel::CompositeOperation effect_composite,
      double offset) const override;

  void InvalidateCssPropertyMap() { css_property_map_ = nullptr; }
  void EnsureCssPropertyMap() const;

  bool IsStringKeyframe() const override { return true; }

  // The tree scope for all the tree-scoped names and references in the
  // keyframe. Nullptr if there's no such tree scope (e.g., the keyframe is
  // created via JavaScript or defined by UA style sheet).
  WeakMember<const TreeScope> tree_scope_;

  // Mapping of unresolved properties to a their resolvers. A resolver knows
  // how to expand shorthands to their corresponding longhand property names,
  // convert logical to physical property names and compare precedence for
  // resolving longhand name collisions.  The resolver also knows how to
  // create serialized text for a shorthand, which is required for getKeyframes
  // calls.
  // See: https://w3.org/TR/web-animations-1/#keyframes-section
  HeapHashMap<PropertyHandle, Member<PropertyResolver>> input_properties_;

  // The resolved properties are computed from unresolved ones applying these
  // steps:
  //  1. Resolve conflicts when multiple properties map to same underlying
  //      one (e.g., margin, margin-top)
  //  2. Expand shorthands to longhands
  //  3. Expand logical properties to physical ones
  mutable Member<MutableCSSPropertyValueSet> css_property_map_;

  // If the keyframes contain one or more logical properties, these need to be
  // remapped to physical properties when the writing mode or text direction
  // changes.
  bool has_logical_property_ = false;

  // The following member is required for mapping logical to physical
  // property names. Though the same for all keyframes within the same model,
  // we store the value here to facilitate lazy evaluation of the CSS
  // properties.
  WritingDirectionMode writing_direction_{WritingMode::kHorizontalTb,
                                          TextDirection::kLtr};
};

using CSSPropertySpecificKeyframe = StringKeyframe::CSSPropertySpecificKeyframe;

template <>
struct DowncastTraits<StringKeyframe> {
  static bool AllowFrom(const Keyframe& value) {
    return value.IsStringKeyframe();
  }
};
template <>
struct DowncastTraits<CSSPropertySpecificKeyframe> {
  static bool AllowFrom(const Keyframe::PropertySpecificKeyframe& value) {
    return value.IsCSSPropertySpecificKeyframe();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_STRING_KEYFRAME_H_
