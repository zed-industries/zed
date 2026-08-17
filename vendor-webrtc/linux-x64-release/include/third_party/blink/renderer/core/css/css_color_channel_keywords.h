// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_COLOR_CHANNEL_KEYWORDS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_COLOR_CHANNEL_KEYWORDS_H_

#include "third_party/blink/renderer/core/core_export.h"

namespace blink {

enum class ColorChannelKeyword;
enum class CSSValueID;

ColorChannelKeyword CSSValueIDToColorChannelKeyword(CSSValueID value);
CSSValueID ColorChannelKeywordToCSSValueID(ColorChannelKeyword keyword);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_COLOR_CHANNEL_KEYWORDS_H_
