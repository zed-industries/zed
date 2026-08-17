// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_COUNTER_STYLE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_COUNTER_STYLE_H_

#include "base/check_op.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

class StyleRuleCounterStyle;
class CSSValue;

enum class CounterStyleSystem {
  kCyclic,
  kFixed,
  kSymbolic,
  kAlphabetic,
  kNumeric,
  kAdditive,
  kHebrew,
  kSimpChineseInformal,
  kSimpChineseFormal,
  kTradChineseInformal,
  kTradChineseFormal,
  kKoreanHangulFormal,
  kKoreanHanjaInformal,
  kKoreanHanjaFormal,
  kLowerArmenian,
  kUpperArmenian,
  kEthiopicNumeric,
  kUnresolvedExtends,
};

enum class CounterStyleSpeakAs {
  kAuto,
  kBullets,
  kNumbers,
  kWords,
  kReference,
};

// Represents a valid counter style defined in a tree scope.
class CORE_EXPORT CounterStyle final : public GarbageCollected<CounterStyle> {
 public:
  static CounterStyle& GetDecimal();

  static CounterStyleSystem ToCounterStyleSystemEnum(const CSSValue* value);

  // Returns nullptr if the @counter-style rule is invalid.
  static CounterStyle* Create(const StyleRuleCounterStyle&);

  const StyleRuleCounterStyle& GetStyleRule() const { return *style_rule_; }

  AtomicString GetName() const;
  CounterStyleSystem GetSystem() const { return system_; }

  bool IsPredefined() const { return is_predefined_; }
  void SetIsPredefined() { is_predefined_ = true; }

  // Returns true for the predefined symbolic counter styles 'disc', 'circle',
  // 'square', 'disclosure-open' and 'disclosure-closed'.
  bool IsPredefinedSymbolMarker() const { return is_predefined_symbol_marker_; }
  void SetIsPredefinedSymbolMarker() { is_predefined_symbol_marker_ = true; }

  // A CounterStyle object is dirtied when the information it holds becomes
  // stale, e.g., when the style rule mutated or the 'extends' or 'fallback'
  // counter styles mutated, etc. Once dirtied, it will never be reused, and
  // will be removed or replaced by a newly created clean CounterStyle.
  // Elements using dirty CounterStyles should update style and layout.
  bool IsDirty() const { return is_dirty_; }
  void SetIsDirty() { is_dirty_ = true; }

  void TraverseAndMarkDirtyIfNeeded(HeapHashSet<Member<CounterStyle>>& visited);

  // Set to true when there's no counter style matching 'extends', 'fallback' or
  // 'speak-as', so this style must be dirtied when new styles are added.
  void SetHasInexistentReferences() { has_inexistent_references_ = true; }

  // https://drafts.csswg.org/css-counter-styles/#generate-a-counter
  String GenerateRepresentation(int value) const;

  String GetPrefix() const { return prefix_; }
  String GetSuffix() const { return suffix_; }

  String GenerateRepresentationWithPrefixAndSuffix(int value) const {
    return prefix_ + GenerateRepresentation(value) + suffix_;
  }

  AtomicString GetExtendsName() const { return extends_name_; }
  const CounterStyle& GetExtendedStyle() const { return *extended_style_; }
  bool HasUnresolvedExtends() const {
    return system_ == CounterStyleSystem::kUnresolvedExtends;
  }
  void ResolveExtends(CounterStyle& extended);

  AtomicString GetFallbackName() const { return fallback_name_; }
  const CounterStyle& GetFallbackStyle() const { return *fallback_style_; }
  bool HasUnresolvedFallback() const { return !fallback_style_; }
  void ResolveFallback(CounterStyle& fallback) { fallback_style_ = &fallback; }

  CounterStyleSpeakAs GetSpeakAs() const { return speak_as_; }
  AtomicString GetSpeakAsName() const { return speak_as_name_; }
  bool HasUnresolvedSpeakAsReference() const {
    return speak_as_ == CounterStyleSpeakAs::kReference && !speak_as_style_;
  }
  void ResolveInvalidSpeakAsReference() {
    speak_as_ = CounterStyleSpeakAs::kAuto;
    speak_as_style_ = nullptr;
  }
  void ResolveSpeakAsReference(CounterStyle& speak_as) {
    DCHECK_NE(CounterStyleSpeakAs::kReference, speak_as.speak_as_);
    speak_as_style_ = speak_as;
  }
  const CounterStyle& GetSpeakAsStyle() const {
    DCHECK_EQ(CounterStyleSpeakAs::kReference, speak_as_);
    return *speak_as_style_;
  }

  // Converts kReference and kAuto to one of the remaining values.
  CounterStyleSpeakAs EffectiveSpeakAs() const;

  // Generates the alternative text for the given counter value according to the
  // 'speak-as' descriptor. Consumed by accessibility.
  String GenerateTextAlternative(int value) const;

  void Trace(Visitor*) const;

  explicit CounterStyle(const StyleRuleCounterStyle& rule);
  ~CounterStyle();

 private:
  // https://drafts.csswg.org/css-counter-styles/#counter-style-range
  bool RangeContains(int value) const;

  // Returns true if a negative sign is needed for the value.
  // https://drafts.csswg.org/css-counter-styles/#counter-style-negative
  bool NeedsNegativeSign(int value) const;

  // https://drafts.csswg.org/css-counter-styles/#initial-representation-for-the-counter-value
  // Returns nullptr if the counter value cannot be represented with the given
  // 'system', 'range' and 'symbols'/'additive-symbols' descriptor values.
  String GenerateInitialRepresentation(int value) const;

  // Uses the fallback counter style to generate a representation for the value.
  // It may recurse, and if it enters a loop, it uses 'decimal' instead.
  String GenerateFallbackRepresentation(int value) const;

  String IndexesToString(const Vector<wtf_size_t>& symbol_indexes) const;

  String GenerateTextAlternativeWithoutPrefixSuffix(int value) const;

  // The corresponding style rule in CSS.
  Member<const StyleRuleCounterStyle> style_rule_;

  // Tracks mutations of |style_rule_|.
  int style_rule_version_;

  // The actual system of the counter style with 'extends' resolved. The value
  // is kUnresolvedExtends temporarily before the resolution.
  CounterStyleSystem system_ = CounterStyleSystem::kSymbolic;

  AtomicString extends_name_;
  Member<CounterStyle> extended_style_;

  AtomicString fallback_name_{"decimal"};
  Member<CounterStyle> fallback_style_;

  CounterStyleSpeakAs speak_as_ = CounterStyleSpeakAs::kAuto;

  // These two members are set if 'speak-as' references another counter style.
  AtomicString speak_as_name_;
  Member<CounterStyle> speak_as_style_;

  // True if we are looking for a fallback counter style to generate a counter
  // value. Supports cycle detection in fallback.
  mutable bool is_in_fallback_ = false;

  // Value of 'symbols' for non-additive systems; Or symbol values in
  // 'additive-symbols' for the 'additive' system.
  Vector<String> symbols_;

  // Additive weights, for the 'additive' system only.
  Vector<unsigned> additive_weights_;

  // Value of 'range' descriptor. Empty vector means 'auto'.
  Vector<std::pair<int, int>> range_;

  String prefix_;
  String suffix_ = ". ";

  String negative_prefix_ = "-";
  String negative_suffix_;

  String pad_symbol_;
  wtf_size_t pad_length_ = 0;

  // First symbol value, for 'fixed' system only.
  wtf_size_t first_symbol_value_ = 1;

  bool is_predefined_ = false;
  bool is_predefined_symbol_marker_ = false;
  bool has_inexistent_references_ = false;
  bool is_dirty_ = false;

  friend class CounterStyleMapTest;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_COUNTER_STYLE_H_
