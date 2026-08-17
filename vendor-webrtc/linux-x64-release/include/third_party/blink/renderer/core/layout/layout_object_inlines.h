// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_OBJECT_INLINES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_OBJECT_INLINES_H_

#include "third_party/blink/renderer/core/css/style_engine.h"
#include "third_party/blink/renderer/core/layout/layout_object.h"

namespace blink {

// The following methods are inlined for performance but not put in
// layout_object.h because that would unnecessarily tie layout_object.h
// to style_engine.h for all users of layout_object.h that don't use
// these methods.

inline const ComputedStyle* LayoutObject::FirstLineStyle() const {
  NOT_DESTROYED();
  if (GetDocument().GetStyleEngine().UsesFirstLineRules()) {
    if (const ComputedStyle* first_line_style = FirstLineStyleWithoutFallback())
      return first_line_style;
  }
  return Style();
}

inline const ComputedStyle& LayoutObject::FirstLineStyleRef() const {
  NOT_DESTROYED();
  const ComputedStyle* style = FirstLineStyle();
  CHECK(style);
  return *style;
}

inline const ComputedStyle* LayoutObject::Style(bool first_line) const {
  NOT_DESTROYED();
  return first_line ? FirstLineStyle() : Style();
}

inline const ComputedStyle& LayoutObject::StyleRef(bool first_line) const {
  NOT_DESTROYED();
  const ComputedStyle* style = Style(first_line);
  CHECK(style);
  return *style;
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_OBJECT_INLINES_H_
