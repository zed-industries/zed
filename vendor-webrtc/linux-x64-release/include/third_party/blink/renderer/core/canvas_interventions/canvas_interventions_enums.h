// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CANVAS_INTERVENTIONS_CANVAS_INTERVENTIONS_ENUMS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CANVAS_INTERVENTIONS_CANVAS_INTERVENTIONS_ENUMS_H_

#include <concepts>
#include <cstdint>

namespace blink {

enum class CanvasNoiseReason {
  kAllConditionsMet = 0,
  kNoRenderContext = 1,
  kNoTrigger = 2,
  kNo2d = 4,
  kNoGpu = 8,
  kNotEnabledInMode = 16,
  kNoExecutionContext = 32,
  kMaxValue = kNoExecutionContext
};

inline constexpr CanvasNoiseReason operator|(CanvasNoiseReason a,
                                             CanvasNoiseReason b) {
  return static_cast<CanvasNoiseReason>(static_cast<int>(a) |
                                        static_cast<int>(b));
}
inline constexpr CanvasNoiseReason& operator|=(CanvasNoiseReason& a,
                                               CanvasNoiseReason b) {
  a = a | b;
  return a;
}

enum class CanvasOperationType {
  kNone = 0,
  kArc = 1,
  kEllipse = 2,
  kSetShadowBlur = 4,
  kSetShadowColor = 8,
  kGlobalCompositionOperation = 16,
  kFillText = 32,
  kStrokeText = 64,
  kMaxValue = kStrokeText
};

inline constexpr CanvasOperationType operator|(CanvasOperationType a,
                                               CanvasOperationType b) {
  return static_cast<CanvasOperationType>(static_cast<int>(a) |
                                          static_cast<int>(b));
}
inline constexpr CanvasOperationType& operator|=(CanvasOperationType& a,
                                                 CanvasOperationType b) {
  a = a | b;
  return a;
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CANVAS_INTERVENTIONS_CANVAS_INTERVENTIONS_ENUMS_H_
