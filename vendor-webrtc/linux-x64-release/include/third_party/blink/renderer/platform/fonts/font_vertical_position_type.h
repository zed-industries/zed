// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_VERTICAL_POSITION_TYPE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_VERTICAL_POSITION_TYPE_H_

namespace blink {

enum class FontVerticalPositionType {
  // TextTop and TextBottom are the top/bottom of the content area.
  // This is where 'vertical-align: text-top/text-bottom' aligns to.
  // This is explicitly undefined in CSS2.
  // https://drafts.csswg.org/css2/visudet.html#inline-non-replaced
  TextTop,
  TextBottom,
  // Em height as being discussed in Font Metrics API.
  // https://drafts.css-houdini.org/font-metrics-api-1/#fontmetrics
  TopOfEmHeight,
  BottomOfEmHeight
};

// Returns whether the position type is CSS "line-over"; i.e., ascender side
// or "top" side of a line box.
// https://drafts.csswg.org/css-writing-modes-3/#line-over
inline bool IsLineOverSide(FontVerticalPositionType type) {
  return type == FontVerticalPositionType::TextTop ||
         type == FontVerticalPositionType::TopOfEmHeight;
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_VERTICAL_POSITION_TYPE_H_
