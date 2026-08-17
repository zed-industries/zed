// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_POSITION_TRY_FALLBACKS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_POSITION_TRY_FALLBACKS_H_

#include <array>

#include "third_party/blink/renderer/core/style/computed_style_constants.h"
#include "third_party/blink/renderer/core/style/position_area.h"
#include "third_party/blink/renderer/core/style/scoped_css_name.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"

namespace blink {

// https://drafts.csswg.org/css-anchor-position-1/#typedef-position-try-fallbacks-try-tactic
//
// Note that we have to preserve the individual "flips"
// in the order they were specified.
using TryTacticList = std::array<TryTactic, 3>;

static inline constexpr TryTacticList kNoTryTactics = {
    TryTactic::kNone, TryTactic::kNone, TryTactic::kNone};

class CORE_EXPORT PositionTryFallback {
  DISALLOW_NEW();

 public:
  PositionTryFallback(const ScopedCSSName* name, TryTacticList tactic_list)
      : position_try_name_(name), tactic_list_(tactic_list) {}
  explicit PositionTryFallback(PositionArea position_area)
      : tactic_list_(kNoTryTactics), position_area_(position_area) {}

  const TryTacticList& GetTryTactic() const { return tactic_list_; }
  const ScopedCSSName* GetPositionTryName() const { return position_try_name_; }
  const PositionArea& GetPositionArea() const { return position_area_; }

  bool operator==(const PositionTryFallback& other) const;

  void Trace(Visitor* visitor) const;

 private:
  Member<const ScopedCSSName> position_try_name_;
  TryTacticList tactic_list_;
  PositionArea position_area_;
};

class CORE_EXPORT PositionTryFallbacks
    : public GarbageCollected<PositionTryFallbacks> {
 public:
  PositionTryFallbacks(HeapVector<PositionTryFallback> fallbacks)
      : fallbacks_(std::move(fallbacks)) {}

  const HeapVector<PositionTryFallback>& GetFallbacks() const {
    return fallbacks_;
  }
  bool operator==(const PositionTryFallbacks& other) const;
  // Returns true if at least one option refers to at least one name in the
  // passed in 'names' set.
  bool HasPositionTryName(const HashSet<AtomicString>& names) const;
  void Trace(Visitor* visitor) const;

 private:
  HeapVector<PositionTryFallback> fallbacks_;
};

}  // namespace blink

namespace WTF {

template <>
struct VectorTraits<blink::PositionTryFallback>
    : VectorTraitsBase<blink::PositionTryFallback> {
  static const bool kCanClearUnusedSlotsWithMemset = true;
  static const bool kCanInitializeWithMemset = true;
  static const bool kCanMoveWithMemcpy = true;
  static const bool kCanTraceConcurrently = true;
};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_POSITION_TRY_FALLBACKS_H_
