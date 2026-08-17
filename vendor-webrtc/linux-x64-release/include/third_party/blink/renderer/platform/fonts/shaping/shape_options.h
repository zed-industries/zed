// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_SHAPE_OPTIONS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_SHAPE_OPTIONS_H_

#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

//
// Options when shaping by `HarfBuzzShaper`.
//
struct PLATFORM_EXPORT ShapeOptions {
  bool is_line_start = false;
  bool han_kerning_start = false;
  bool han_kerning_end = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_SHAPE_OPTIONS_H_
