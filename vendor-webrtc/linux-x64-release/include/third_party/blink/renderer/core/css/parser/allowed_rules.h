// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_ALLOWED_RULES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_ALLOWED_RULES_H_

#include <cstdint>
#include <initializer_list>

#include "css_at_rule_id.h"

namespace blink {

// https://drafts.csswg.org/css-syntax/#qualified-rule
enum class QualifiedRuleType {
  // A regular style rule, e.g. .foo:hover { ... }.
  kStyle,
  // A keyframe rule found within @keyframes, e.g. 50% {  ... }.
  kKeyframe,

  kCount  // Must go last.
};

// This class represents which kinds of rules are valid in a certain context.
// For example, @namespace rules are only valid top-level, and @ornaments is
// only valid within @font-feature-values.
class AllowedRules {
  template <typename T>
  static constexpr uint64_t EnumBits(std::initializer_list<T> list) {
    uint64_t bits = 0;
    for (T item : list) {
      bits |= uint64_t{1} << static_cast<uint64_t>(item);
    }
    return bits;
  }

 public:
  constexpr AllowedRules() : bits_(0) {}
  // https://drafts.csswg.org/css-syntax/#at-rule
  constexpr AllowedRules(std::initializer_list<CSSAtRuleID> list)
      : bits_(EnumBits(list)) {}
  // https://drafts.csswg.org/css-syntax/#qualified-rule
  constexpr AllowedRules(std::initializer_list<QualifiedRuleType> list)
      : bits_(EnumBits(list) << static_cast<uint64_t>(CSSAtRuleID::kCount)) {}
  constexpr AllowedRules operator|(const AllowedRules& other) const {
    return AllowedRules(bits_ | other.bits_);
  }

  bool operator==(const AllowedRules& other) const {
    return bits_ == other.bits_;
  }
  bool operator!=(const AllowedRules& other) const {
    return bits_ != other.bits_;
  }

  void Remove(CSSAtRuleID id) { bits_ &= ~(uint64_t{1} << AtRuleBit(id)); }
  bool Has(CSSAtRuleID id) const { return (bits_ >> AtRuleBit(id)) & 1; }

  void Remove(QualifiedRuleType id) { bits_ &= ~(uint64_t{1} << QRuleBit(id)); }
  bool Has(QualifiedRuleType id) const { return (bits_ >> QRuleBit(id)) & 1; }

 private:
  explicit constexpr AllowedRules(uint64_t bits) : bits_(bits) {}

  static constexpr uint64_t AtRuleBit(CSSAtRuleID id) {
    return static_cast<uint64_t>(id);
  }
  static constexpr uint64_t QRuleBit(QualifiedRuleType type) {
    return static_cast<uint64_t>(CSSAtRuleID::kCount) +
           static_cast<uint64_t>(type);
  }

  // TODO(andruud): We could use std::bitset to lift this restriction
  // when it becomes sufficiently constexpr (C++23).
  static_assert((static_cast<uint64_t>(CSSAtRuleID::kCount) +
                 static_cast<uint64_t>(QualifiedRuleType::kCount)) <= 64);

  // Bits for CSSAtRuleID values are stored first (lower bits), then for
  // QualifiedRuleType values (higher bits).
  uint64_t bits_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_ALLOWED_RULES_H_
