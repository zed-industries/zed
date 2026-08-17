// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_GEOMETRY_DOM_RECT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_GEOMETRY_DOM_RECT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/geometry/dom_rect_read_only.h"

namespace gfx {
class Rect;
class RectF;
}  // namespace gfx

namespace blink {

class DOMRect;
class DOMRectInit;

class CORE_EXPORT DOMRect final : public DOMRectReadOnly {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static DOMRect* Create(double x = 0,
                         double y = 0,
                         double width = 0,
                         double height = 0);
  static DOMRect* FromRectF(const gfx::RectF&);
  static DOMRect* fromRect(const DOMRectInit*);

  DOMRect(double x, double y, double z, double w);

  gfx::Rect ToEnclosingRect() const;

  void setX(double x) { x_ = x; }
  void setY(double y) { y_ = y; }
  void setWidth(double width) { width_ = width; }
  void setHeight(double height) { height_ = height; }
};

inline bool operator==(const DOMRect& lhs, const DOMRect& rhs) {
  return lhs.x() == rhs.x() && lhs.y() == rhs.y() &&
         lhs.width() == rhs.width() && lhs.height() == rhs.height();
}
inline bool operator!=(const DOMRect& lhs, const DOMRect& rhs) {
  return !(lhs == rhs);
}
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_GEOMETRY_DOM_RECT_H_
