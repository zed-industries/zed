// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_PATH_TYPES_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_PATH_TYPES_H_

#include "base/notreached.h"
#include "third_party/skia/include/core/SkPaint.h"
#include "third_party/skia/include/core/SkPathTypes.h"

namespace blink {

enum LineCap {
  kButtCap = SkPaint::kButt_Cap,
  kRoundCap = SkPaint::kRound_Cap,
  kSquareCap = SkPaint::kSquare_Cap
};

enum LineJoin {
  kMiterJoin = SkPaint::kMiter_Join,
  kRoundJoin = SkPaint::kRound_Join,
  kBevelJoin = SkPaint::kBevel_Join
};

enum WindRule {
  RULE_NONZERO = static_cast<int>(SkPathFillType::kWinding),
  RULE_EVENODD = static_cast<int>(SkPathFillType::kEvenOdd)
};

inline SkPathFillType WebCoreWindRuleToSkFillType(WindRule rule) {
  return static_cast<SkPathFillType>(rule);
}

inline WindRule SkFillTypeToWindRule(SkPathFillType fill_type) {
  switch (fill_type) {
    case SkPathFillType::kWinding:
    case SkPathFillType::kEvenOdd:
      return static_cast<WindRule>(fill_type);
    default:
      NOTREACHED();
  }
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_PATH_TYPES_H_
