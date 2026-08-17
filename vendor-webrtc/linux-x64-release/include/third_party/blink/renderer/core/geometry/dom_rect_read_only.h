// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_GEOMETRY_DOM_RECT_READ_ONLY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_GEOMETRY_DOM_RECT_READ_ONLY_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/geometry/geometry_util.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "ui/gfx/geometry/rect.h"
#include "ui/gfx/geometry/rect_f.h"

namespace blink {

class DOMRectInit;
class ScriptObject;
class ScriptState;

class CORE_EXPORT DOMRectReadOnly : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static DOMRectReadOnly* Create(double x,
                                 double y,
                                 double width,
                                 double height);
  static DOMRectReadOnly* FromRect(const gfx::Rect&);
  static DOMRectReadOnly* FromRectF(const gfx::RectF&);
  static DOMRectReadOnly* fromRect(const DOMRectInit*);

  DOMRectReadOnly(double x, double y, double width, double height);

  double x() const { return x_; }
  double y() const { return y_; }
  double width() const { return width_; }
  double height() const { return height_; }

  double top() const { return geometry_util::NanSafeMin(y_, y_ + height_); }
  double right() const { return geometry_util::NanSafeMax(x_, x_ + width_); }
  double bottom() const { return geometry_util::NanSafeMax(y_, y_ + height_); }
  double left() const { return geometry_util::NanSafeMin(x_, x_ + width_); }

  ScriptObject toJSONForBinding(ScriptState*) const;

  bool IsPointInside(double x, double y) const {
    return x >= left() && x < right() && y >= top() && y < bottom();
  }

 protected:
  double x_;
  double y_;
  double width_;
  double height_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_GEOMETRY_DOM_RECT_READ_ONLY_H_
