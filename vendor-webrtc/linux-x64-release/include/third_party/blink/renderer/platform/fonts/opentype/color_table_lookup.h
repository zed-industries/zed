// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_OPENTYPE_COLOR_TABLE_LOOKUP_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_OPENTYPE_COLOR_TABLE_LOOKUP_H_

#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/skia/include/core/SkTypeface.h"

namespace blink {

class PLATFORM_EXPORT ColorTableLookup {
 public:
  static bool TypefaceHasAnySupportedColorTable(const SkTypeface* typeface);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_OPENTYPE_COLOR_TABLE_LOOKUP_H_
