/*
 * (C) 1999-2003 Lars Knoll (knoll@kde.org)
 * Copyright (C) 2004, 2005, 2006, 2008, 2012 Apple Inc. All rights reserved.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 */
#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_PROPERTY_VALUE_SET_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_PROPERTY_VALUE_SET_H_

#include "base/bits.h"
#include "base/types/pass_key.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_primitive_value.h"
#include "third_party/blink/renderer/core/css/css_property_name.h"
#include "third_party/blink/renderer/core/css/css_property_names.h"
#include "third_party/blink/renderer/core/css/css_property_value.h"
#include "third_party/blink/renderer/core/css/parser/css_parser_mode.h"
#include "third_party/blink/renderer/core/css/property_set_css_style_declaration.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class CSSStyleDeclaration;
class ExecutionContext;
class ImmutableCSSPropertyValueSet;
class MutableCSSPropertyValueSet;
class StyleSheetContents;
enum class CSSValueID;
enum class SecureContextMode;

class CORE_EXPORT CSSPropertyValueSet
    : public GarbageCollected<CSSPropertyValueSet> {
  friend class PropertyReference;

 public:
  CSSPropertyValueSet(const CSSPropertyValueSet&) = delete;
  CSSPropertyValueSet& operator=(const CSSPropertyValueSet&) = delete;

  void FinalizeGarbageCollectedObject();

  unsigned PropertyCount() const;
  bool IsEmpty() const;

  const CSSPropertyValue& PropertyAt(unsigned index) const;
  base::span<const CSSPropertyValue> Properties() const;

  template <typename T>  // CSSPropertyID or AtomicString
  int FindPropertyIndex(const T& property) const;

  bool HasProperty(CSSPropertyID property) const {
    return FindPropertyIndex(property) != -1;
  }

  template <typename T>  // CSSPropertyID or AtomicString
  const CSSValue* GetPropertyCSSValue(const T& property) const;

  template <typename T>  // CSSPropertyID or AtomicString
  String GetPropertyValue(const T& property) const;

  template <typename T>  // CSSPropertyID or AtomicString
  bool PropertyIsImportant(const T& property) const;

  const CSSValue* GetPropertyCSSValueWithHint(const AtomicString& property_name,
                                              unsigned index) const;
  String GetPropertyValueWithHint(const AtomicString& property_name,
                                  unsigned index) const;
  bool PropertyIsImportantWithHint(const AtomicString& property_name,
                                   unsigned index) const;

  bool ShorthandIsImportant(CSSPropertyID) const;
  bool ShorthandIsImportant(const AtomicString& custom_property_name) const {
    // Custom properties are never shorthands.
    return false;
  }

  CSSPropertyID GetPropertyShorthand(CSSPropertyID) const;
  bool IsPropertyImplicit(CSSPropertyID) const;

  CSSParserMode CssParserMode() const {
    return static_cast<CSSParserMode>(css_parser_mode_);
  }

  MutableCSSPropertyValueSet* MutableCopy() const;
  ImmutableCSSPropertyValueSet* ImmutableCopyIfNeeded() const;

  MutableCSSPropertyValueSet* CopyPropertiesInSet(
      const Vector<const CSSProperty*>&) const;

  String AsText() const;

  bool IsMutable() const { return is_mutable_; }
  bool ContainsCursorHand() const { return contains_cursor_hand_; }

  // Computes a hash of the contents of this property value set
  // (cached after first call). Note that hash equality may have
  // false negatives (there is no guarantee that a.AsText() == b.AsText()
  // implies a.GetHash() == b.GetHash()), because not all CSSValues are
  // easily hashed and may fall back to using the pointer value.
  //
  // The hash value has a second task, namely to mark property sets that have
  // been modified after we've stored them into the MPC
  // (MatchedPropertiesCache). If you call GetHash() and then later modify the
  // property set (in any way that would change the hash), all future calls to
  // GetHash() for this object will return HashTraits<unsigned>::DeletedValue(),
  // making them invalid for cache lookup uses. (This status can be reset if you
  // clone the object, such as calling ImmutableCopyIfNeeded().) This protects
  // the MPC from returning false positives when a mutable CSSPropertyValueSet
  // has changed, such as for SVG objects' “direct update” of the presentation
  // attribute style.
  //
  // Can never return HashTraits<unsigned>::EmptyValue() (it is used
  // internally).
  unsigned GetHash() const {
    if (hash_ == WTF::HashTraits<unsigned>::EmptyValue()) {
      hash_ = ComputeHash();
    }
    return hash_;
  }
  unsigned GetExistingHash() const {
    DCHECK_NE(hash_, WTF::HashTraits<unsigned>::EmptyValue());
    return hash_;
  }
  bool ModifiedSinceHashing() const {
    return hash_ == WTF::HashTraits<unsigned>::DeletedValue();
  }

  bool Equals(const CSSPropertyValueSet& other) {
    if (this == &other) {
      return true;
    }
    return Properties() == other.Properties();
  }

  bool HasFailedOrCanceledSubresources() const;

  static unsigned AverageSizeInBytes();

#ifndef NDEBUG
  void ShowStyle();
#endif

  bool PropertyMatches(CSSPropertyID, const CSSValue&) const;

  void Trace(Visitor*) const;
  void TraceAfterDispatch(blink::Visitor* visitor) const {}

 protected:
  static constexpr unsigned kMaxArraySize = (1 << 25) - 1;

  explicit CSSPropertyValueSet(CSSParserMode css_parser_mode)
      : array_size_(0),
        css_parser_mode_(css_parser_mode),
        is_mutable_(true),
        contains_cursor_hand_(false) {}

  CSSPropertyValueSet(CSSParserMode css_parser_mode,
                      unsigned immutable_array_size,
                      bool contains_cursor_hand)
      // Avoid min()/max() from std here in the header, because that would
      // require inclusion of <algorithm>, which is slow to compile.
      : array_size_((immutable_array_size < unsigned(kMaxArraySize))
                        ? immutable_array_size
                        : unsigned(kMaxArraySize)),
        css_parser_mode_(css_parser_mode),
        is_mutable_(false),
        contains_cursor_hand_(contains_cursor_hand) {}

  unsigned ComputeHash() const;

  const uint32_t array_size_ : 25;  // Only for immutable sets.
  const uint32_t css_parser_mode_ : 4;
  const uint32_t is_mutable_ : 1;
  const uint32_t contains_cursor_hand_ : 1;
  uint32_t may_have_logical_properties_ : 1 = false;  // Only for mutable sets.

  // EmptyValue() means “not computed yet”. DeletedValue() means “invalid”
  // (see GetHash()).
  mutable unsigned hash_ = WTF::HashTraits<unsigned>::EmptyValue();

  friend class PropertySetCSSStyleDeclaration;
};

class CORE_EXPORT alignas(CSSPropertyName) ImmutableCSSPropertyValueSet
    : public CSSPropertyValueSet {
 public:
  // The value and metadata arrays are allocated in-line with the containing
  // ImmutableCSSPropertyValueSet. In order to guarantee safety when accessing
  // those arrays, we must ensure that ImmutableCSSPropertyValueSet can only
  // be constructed via the Create() method, which allocates the correct amount
  // of space.
  using PassKey = base::PassKey<ImmutableCSSPropertyValueSet>;
  ImmutableCSSPropertyValueSet(PassKey,
                               base::span<const CSSPropertyValue>,
                               CSSParserMode,
                               bool contains_cursor_hand = false);

  static ImmutableCSSPropertyValueSet* Create(
      base::span<const CSSPropertyValue>,
      CSSParserMode,
      bool contains_cursor_hand = false);

  unsigned PropertyCount() const { return array_size_; }

  base::span<const CSSPropertyValue> Properties() const;

  template <typename T>  // CSSPropertyID or AtomicString
  int FindPropertyIndex(const T& property) const;

  void TraceAfterDispatch(blink::Visitor*) const;

 private:
  const CSSPropertyValue* ArrayBase() const;
};

inline const CSSPropertyValue* ImmutableCSSPropertyValueSet::ArrayBase() const {
  static_assert(
      sizeof(ImmutableCSSPropertyValueSet) % alignof(CSSPropertyName) == 0,
      "ValueArray may be improperly aligned");
  // SAFETY: By funneling all allocation of ImmutableCSSPropertyValueSet through
  // Create(), we guarantee that the array will exist where we expect it.
  CHECK_GT(array_size_, 0u);
  return UNSAFE_BUFFERS(reinterpret_cast<const CSSPropertyValue*>(this + 1));
}

inline base::span<const CSSPropertyValue>
ImmutableCSSPropertyValueSet::Properties() const {
  if (array_size_ == 0) {
    return base::span<CSSPropertyValue>();
  }
  // SAFETY: By funneling all allocation of ImmutableCSSPropertyValueSet through
  // Create(), we guarantee that the array will have the size we expect.
  return UNSAFE_BUFFERS(base::span(ArrayBase(), array_size_));
}

template <>
struct DowncastTraits<ImmutableCSSPropertyValueSet> {
  static bool AllowFrom(const CSSPropertyValueSet& set) {
    return !set.IsMutable();
  }
};

class CORE_EXPORT MutableCSSPropertyValueSet : public CSSPropertyValueSet {
 public:
  explicit MutableCSSPropertyValueSet(CSSParserMode);
  explicit MutableCSSPropertyValueSet(const CSSPropertyValueSet&);
  explicit MutableCSSPropertyValueSet(
      base::span<const CSSPropertyValue> properties);
  ~MutableCSSPropertyValueSet() = default;

  unsigned PropertyCount() const { return property_vector_.size(); }
  const HeapVector<CSSPropertyValue, 4>& Properties() const {
    return property_vector_;
  }

  enum SetResult {
    // The value failed to parse correctly, and thus, there was no change.
    kParseError = 0,

    // The value parsed correctly, but there was no change,
    // as it matched the value already in place.
    kUnchanged = 1,

    // The value parsed correctly, and there was a change to a property that
    // already existed.
    kModifiedExisting = 2,

    // The value parsed correctly, and caused a property to be added or
    // modified. (If you do not care whether it did, you can compare the
    // enum using result >= kModifiedExisting.)
    kChangedPropertySet = 3,
  };

  // Wrapper around SetLonghandProperty() for setting multiple properties
  // at a time.
  SetResult AddParsedProperties(base::span<CSSPropertyValue>);

  // Wrapper around SetLonghandProperty() that does nothing if the same property
  // already exists with an !important declaration.
  //
  // Returns whether this style set was changed.
  bool AddRespectingCascade(const CSSPropertyValue&);

  // Expands shorthand properties into multiple properties.
  void SetProperty(CSSPropertyID, const CSSValue&, bool important = false);

  // Convenience wrapper around the above that also supports custom properties.
  void SetProperty(const CSSPropertyName&,
                   const CSSValue&,
                   bool important = false);

  // Also a convenience wrapper around SetProperty(), parsing the value from a
  // string before setting it. If the value is empty, the property is removed.
  // Only for non-custom properties.
  SetResult ParseAndSetProperty(
      CSSPropertyID unresolved_property,
      StringView value,
      bool important,
      SecureContextMode,
      StyleSheetContents* context_style_sheet = nullptr);

  // Similar to ParseAndSetProperty(), but for custom properties instead.
  // (By implementation quirk, it attempts shorthand expansion, even though
  // custom properties can never be shorthands.) If the value is empty,
  // the property is removed.
  SetResult ParseAndSetCustomProperty(const AtomicString& custom_property_name,
                                      StringView value,
                                      bool important,
                                      SecureContextMode,
                                      StyleSheetContents* context_style_sheet,
                                      bool is_animation_tainted);

  // This one does not expand longhands, but is the second-most efficient form
  // save for the CSSPropertyID variant below.
  // All the other property setters eventually call down into this.
  SetResult SetLonghandProperty(CSSPropertyValue);

  // A streamlined version of the above, which can be used if you don't need
  // custom properties and don't need the return value (which requires an extra
  // comparison with the old property). This is the fastest form.
  void SetLonghandProperty(CSSPropertyID, const CSSValue&);

  // Convenience form of the CSSPropertyValue overload above.
  SetResult SetLonghandProperty(CSSPropertyID,
                                CSSValueID identifier,
                                bool important = false);

  template <typename T>  // CSSPropertyID or AtomicString
  bool RemoveProperty(const T& property, String* return_text = nullptr);
  bool RemovePropertiesInSet(base::span<const CSSProperty* const> set);
  bool RemovePropertiesAffectedByAll();
  void RemoveEquivalentProperties(const CSSPropertyValueSet*);
  void RemoveEquivalentProperties(const CSSStyleDeclaration*);

  void MergeAndOverrideOnConflict(const CSSPropertyValueSet*);

  void Clear();
  void ParseDeclarationList(const String& style_declaration,
                            SecureContextMode,
                            StyleSheetContents* context_style_sheet);

  CSSStyleDeclaration* EnsureCSSStyleDeclaration(
      ExecutionContext* execution_context);

  template <typename T>  // CSSPropertyID or AtomicString
  int FindPropertyIndex(const T& property) const;

  void TraceAfterDispatch(blink::Visitor*) const;

 private:
  template <typename T>  // CSSPropertyID or AtomicString
  const CSSPropertyValue* FindPropertyPointer(const T& property) const;

  // Returns nullptr if there is no property to be overwritten.
  //
  // If property_id is a logical property we've already seen a different
  // property matching, this will remove the existing property (and return
  // nullptr).
  ALWAYS_INLINE CSSPropertyValue* FindInsertionPointForID(
      CSSPropertyID property_id);

  bool RemovePropertyAtIndex(int, String* return_text);

  bool RemoveShorthandProperty(CSSPropertyID);
  bool RemoveShorthandProperty(const AtomicString& custom_property_name) {
    return false;
  }
  CSSPropertyValue* FindCSSPropertyWithName(const CSSPropertyName&);

  void InvalidateHashIfComputed() {
    if (hash_ != WTF::HashTraits<unsigned>::EmptyValue()) {
      hash_ = WTF::HashTraits<unsigned>::DeletedValue();
    }
  }

  Member<PropertySetCSSStyleDeclaration> cssom_wrapper_;

  friend class CSSPropertyValueSet;

  HeapVector<CSSPropertyValue, 4> property_vector_;
};

template <>
struct DowncastTraits<MutableCSSPropertyValueSet> {
  static bool AllowFrom(const CSSPropertyValueSet& set) {
    return set.IsMutable();
  }
};

inline const CSSPropertyValue& CSSPropertyValueSet::PropertyAt(
    unsigned index) const {
  if (auto* mutable_property_set =
          DynamicTo<MutableCSSPropertyValueSet>(this)) {
    return mutable_property_set->property_vector_.at(index);
  }
  return To<ImmutableCSSPropertyValueSet>(*this).Properties()[index];
}

inline base::span<const CSSPropertyValue> CSSPropertyValueSet::Properties()
    const {
  if (auto* mutable_property_set =
          DynamicTo<MutableCSSPropertyValueSet>(this)) {
    return mutable_property_set->Properties();
  }
  return To<ImmutableCSSPropertyValueSet>(*this).Properties();
}

inline unsigned CSSPropertyValueSet::PropertyCount() const {
  if (auto* mutable_property_set =
          DynamicTo<MutableCSSPropertyValueSet>(this)) {
    return mutable_property_set->property_vector_.size();
  }
  return array_size_;
}

inline bool CSSPropertyValueSet::IsEmpty() const {
  return !PropertyCount();
}

template <typename T>
inline int CSSPropertyValueSet::FindPropertyIndex(const T& property) const {
  if (auto* mutable_property_set =
          DynamicTo<MutableCSSPropertyValueSet>(this)) {
    return mutable_property_set->FindPropertyIndex(property);
  }
  return To<ImmutableCSSPropertyValueSet>(this)->FindPropertyIndex(property);
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_PROPERTY_VALUE_SET_H_
