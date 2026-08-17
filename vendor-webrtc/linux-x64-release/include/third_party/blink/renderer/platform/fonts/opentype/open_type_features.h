// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_OPENTYPE_OPEN_TYPE_FEATURES_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_OPENTYPE_OPEN_TYPE_FEATURES_H_

#include <hb.h>

#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class SimpleFontData;

//
// Represents OpenType features in a font.
//
class PLATFORM_EXPORT OpenTypeFeatures {
  STACK_ALLOCATED();

 public:
  explicit OpenTypeFeatures(const SimpleFontData& font);

  bool Contains(hb_tag_t feature_tag) const {
    return std::find(features_.begin(), features_.end(), feature_tag) !=
           features_.end();
  }

 private:
  // This value is heuristic, 64 is enough to load all features of "Yu Gothic".
  constexpr static unsigned kInitialSize = 64;

  Vector<hb_tag_t, kInitialSize> features_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_OPENTYPE_OPEN_TYPE_FEATURES_H_
