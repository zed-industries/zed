// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_FONT_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_FONT_H_

#include "third_party/blink/public/platform/web_common.h"
#include "third_party/skia/include/core/SkColor.h"
#include "v8/include/cppgc/persistent.h"

// To avoid conflicts with the DrawText macro from the Windows SDK...
#undef DrawText

namespace cc {
class PaintCanvas;
}

namespace gfx {
class PointF;
class RectF;
}

namespace blink {
struct WebFontDescription;
struct WebTextRun;

class BLINK_PLATFORM_EXPORT WebFont {
 public:
  static WebFont* Create(const WebFontDescription&);
  ~WebFont();

  WebFontDescription GetFontDescription() const;
  int Ascent() const;
  int Descent() const;
  int Height() const;
  int LineSpacing() const;
  float XHeight() const;
  void DrawText(cc::PaintCanvas*,
                const WebTextRun&,
                const gfx::PointF& left_baseline,
                SkColor) const;
  int CalculateWidth(const WebTextRun&) const;
  int OffsetForPosition(const WebTextRun&, float position) const;
  gfx::RectF SelectionRectForText(const WebTextRun&,
                                  const gfx::PointF& left_baseline,
                                  int height,
                                  int from = 0,
                                  int to = -1) const;

 private:
  explicit WebFont(const WebFontDescription&);

  class Impl;
  cppgc::Persistent<Impl> private_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_FONT_H_
