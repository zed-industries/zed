// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_OPENTYPE_VARIABLE_AXES_NAMES_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_OPENTYPE_VARIABLE_AXES_NAMES_H_

#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/skia/include/core/SkRefCnt.h"
#include "third_party/skia/include/core/SkTypeface.h"

class SkTypeface;

namespace blink {

struct VariationAxis {
  String tag;
  String name;
  double minValue;
  double maxValue;
  double defaultValue;
};

class PLATFORM_EXPORT VariableAxesNames {
 public:
  static Vector<VariationAxis> GetVariationAxes(sk_sp<SkTypeface> typeface);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_OPENTYPE_VARIABLE_AXES_NAMES_H_
