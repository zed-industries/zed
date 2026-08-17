// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_SUCCESSFUL_POSITION_FALLBACK_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_SUCCESSFUL_POSITION_FALLBACK_H_

#include "third_party/blink/renderer/core/style/position_try_fallbacks.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class CSSPropertyValueSet;

// When an anchored element is positioned using one of the position-try-
// fallbacks without overflowing, we need to keep track of it as the last
// successful fallback because if the same set of fallbacks in later layout
// cannot fit any of the fallbacks, we should fall back to the last successful
// one.
class SuccessfulPositionFallback {
  DISALLOW_NEW();

 public:
  bool operator==(const SuccessfulPositionFallback&) const;
  bool operator!=(const SuccessfulPositionFallback& fallback) const {
    return !operator==(fallback);
  }
  bool IsEmpty() const { return position_try_fallbacks_ == nullptr; }
  void Clear();

  void Trace(Visitor*) const;

  // The computed value of position-try-fallbacks the sets below are based on
  Member<const PositionTryFallbacks> position_try_fallbacks_;
  // The try set used for the successful fallback
  Member<const CSSPropertyValueSet> try_set_;
  // The try tactics used for the successful fallback
  TryTacticList try_tactics_ = kNoTryTactics;
  // The index of the successful option in the given position_try_options_.
  std::optional<size_t> index_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_SUCCESSFUL_POSITION_FALLBACK_H_
