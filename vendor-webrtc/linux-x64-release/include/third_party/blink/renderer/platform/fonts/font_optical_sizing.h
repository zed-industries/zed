// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_OPTICAL_SIZING_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_OPTICAL_SIZING_H_

#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {
enum OpticalSizing { kAutoOpticalSizing, kNoneOpticalSizing };

PLATFORM_EXPORT String ToString(OpticalSizing);
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_OPTICAL_SIZING_H_
