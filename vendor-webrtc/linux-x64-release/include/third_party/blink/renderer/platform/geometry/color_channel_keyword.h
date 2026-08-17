// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_COLOR_CHANNEL_KEYWORD_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_COLOR_CHANNEL_KEYWORD_H_

namespace blink {

// Set of color channel keywords that can appear in a calc() expression.
enum class ColorChannelKeyword {
  kA,
  kB,
  kC,
  kG,
  kH,
  kL,
  kR,
  kS,
  kW,
  kX,
  kY,
  kZ,
  kAlpha
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_COLOR_CHANNEL_KEYWORD_H_
