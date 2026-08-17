// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TEXT_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TEXT_UTILS_H_

#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class ComputedStyle;

float ComputeTextWidth(const StringView& text, const ComputedStyle& style);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TEXT_UTILS_H_
