// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_TEXT_FRAGMENT_PAINT_INFO_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_TEXT_FRAGMENT_PAINT_INFO_H_

#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/text/string_view.h"

namespace blink {

class ShapeResultView;

// Bridge struct for painting text. Encapsulates info needed by the paint code.
struct PLATFORM_EXPORT TextFragmentPaintInfo {
  STACK_ALLOCATED();

 public:
  TextFragmentPaintInfo Slice(unsigned slice_from, unsigned slice_to) const;
  TextFragmentPaintInfo WithStartOffset(unsigned start_from) const;
  TextFragmentPaintInfo WithEndOffset(unsigned end_to) const;
  unsigned Length() const { return to - from; }

  // The string to paint. May include surrounding context.
  const StringView text;

  // The range of the |text| to paint.
  unsigned from;
  unsigned to;

  // The |shape_result| may not contain all characters of the |text|, but is
  // guaranteed to contain |from| to |to|.
  const ShapeResultView* shape_result;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_TEXT_FRAGMENT_PAINT_INFO_H_
