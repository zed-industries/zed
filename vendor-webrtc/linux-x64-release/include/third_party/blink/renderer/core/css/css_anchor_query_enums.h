// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_ANCHOR_QUERY_ENUMS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_ANCHOR_QUERY_ENUMS_H_

#include <cstdint>

namespace blink {

enum class CSSAnchorQueryType : uint8_t {
  kAnchor = 1 << 0,
  kAnchorSize = 1 << 1
};

using CSSAnchorQueryTypes = uint8_t;
constexpr CSSAnchorQueryTypes kCSSAnchorQueryTypesNone = 0u;
constexpr CSSAnchorQueryTypes kCSSAnchorQueryTypesAll =
    ~kCSSAnchorQueryTypesNone;

enum class CSSAnchorValue {
  kInside,
  kOutside,
  kTop,
  kLeft,
  kRight,
  kBottom,
  kStart,
  kEnd,
  kSelfStart,
  kSelfEnd,
  kCenter,
  kPercentage,
};

enum class CSSAnchorSizeValue {
  kImplicit,
  kWidth,
  kHeight,
  kBlock,
  kInline,
  kSelfBlock,
  kSelfInline,
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_ANCHOR_QUERY_ENUMS_H_
