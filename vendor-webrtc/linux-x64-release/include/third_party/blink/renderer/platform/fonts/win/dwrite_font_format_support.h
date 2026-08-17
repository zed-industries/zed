// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_WIN_DWRITE_FONT_FORMAT_SUPPORT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_WIN_DWRITE_FONT_FORMAT_SUPPORT_H_

#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

// Return whether DirectWrite on this system supports variable fonts for
// retrieving metrics and performing rasterization.
bool PLATFORM_EXPORT DWriteVersionSupportsVariations();
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_WIN_DWRITE_FONT_FORMAT_SUPPORT_H_
