// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_TEXT_RUN_PAINT_INFO_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_TEXT_RUN_PAINT_INFO_H_

#include "third_party/blink/renderer/platform/text/text_run.h"

namespace blink {

// Container for parameters needed to paint TextRun.
struct TextRunPaintInfo {
  STACK_ALLOCATED();

 public:
  explicit TextRunPaintInfo(const TextRun& r)
      : run(r), from(0), to(r.length()) {}

  const TextRun& run;
  unsigned from;
  unsigned to;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_TEXT_RUN_PAINT_INFO_H_
