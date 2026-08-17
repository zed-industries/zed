// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_BITMAP_GLYPHS_BLOCK_LIST_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_BITMAP_GLYPHS_BLOCK_LIST_H_

#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

class SkTypeface;

namespace blink {

class PLATFORM_EXPORT BitmapGlyphsBlockList {
  STATIC_ONLY(BitmapGlyphsBlockList);

 public:
  static bool ShouldAvoidEmbeddedBitmapsForTypeface(const SkTypeface&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_BITMAP_GLYPHS_BLOCK_LIST_H_
