// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_SCROLLBAR_COLOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_SCROLLBAR_COLOR_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/style_color.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class CORE_EXPORT StyleScrollbarColor
    : public GarbageCollected<StyleScrollbarColor> {
 public:
  StyleScrollbarColor(StyleColor thumb_color, StyleColor track_color);

  void Trace(Visitor* visitor) const {
    visitor->Trace(thumb_color_);
    visitor->Trace(track_color_);
  }

  StyleColor GetThumbColor() const { return thumb_color_; }
  StyleColor GetTrackColor() const { return track_color_; }

  bool operator==(const StyleScrollbarColor& o) const {
    return thumb_color_ == o.thumb_color_ && track_color_ == o.track_color_;
  }

  bool operator!=(const StyleScrollbarColor& o) const { return !(*this == o); }

 private:
  StyleColor thumb_color_;
  StyleColor track_color_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_SCROLLBAR_COLOR_H_
