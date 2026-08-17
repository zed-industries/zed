// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_COLOR_CHANNEL_MAP_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_COLOR_CHANNEL_MAP_H_

#include <optional>

#include "third_party/blink/renderer/core/css_value_keywords.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"

namespace blink {

// Used in channel keyword substitutions for relative color syntax.
// https://www.w3.org/TR/css-color-5/#relative-colors
// Channel values may be unset if the base color is unknown at parse time.
using CSSColorChannelMap = HashMap<CSSValueID, std::optional<double>>;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_COLOR_CHANNEL_MAP_H_
