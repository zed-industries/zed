// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_GEOMETRY_DOM_POINT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_GEOMETRY_DOM_POINT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/geometry/dom_point_read_only.h"

namespace blink {

class DOMPointInit;

class CORE_EXPORT DOMPoint : public DOMPointReadOnly {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static DOMPoint* Create(double x, double y, double z = 0, double w = 1);
  static DOMPoint* fromPoint(const DOMPointInit*);

  DOMPoint(double x, double y, double z, double w);

  virtual void setX(double x) { x_ = x; }
  virtual void setY(double y) { y_ = y; }
  void setZ(double z) { z_ = z; }
  void setW(double w) { w_ = w; }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_GEOMETRY_DOM_POINT_H_
