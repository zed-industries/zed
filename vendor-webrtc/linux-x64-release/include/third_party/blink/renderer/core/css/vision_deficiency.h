// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_VISION_DEFICIENCY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_VISION_DEFICIENCY_H_

#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

enum class VisionDeficiency {
  kNoVisionDeficiency,
  kBlurredVision,
  kReducedContrast,
  kAchromatopsia,
  kDeuteranopia,
  kProtanopia,
  kTritanopia,
};

AtomicString CreateVisionDeficiencyFilterUrl(
    VisionDeficiency vision_deficiency);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_VISION_DEFICIENCY_H_
