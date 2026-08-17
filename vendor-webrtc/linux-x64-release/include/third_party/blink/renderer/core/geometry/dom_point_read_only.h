// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_GEOMETRY_DOM_POINT_READ_ONLY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_GEOMETRY_DOM_POINT_READ_ONLY_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class DOMMatrixInit;
class DOMPoint;
class DOMPointInit;
class ExceptionState;
class ScriptObject;
class ScriptState;

class CORE_EXPORT DOMPointReadOnly : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static DOMPointReadOnly* Create(double x, double y, double z, double w);
  static DOMPointReadOnly* fromPoint(const DOMPointInit*);

  DOMPointReadOnly(double x, double y, double z, double w);

  double x() const { return x_; }
  double y() const { return y_; }
  double z() const { return z_; }
  double w() const { return w_; }

  ScriptObject toJSONForBinding(ScriptState*) const;
  DOMPoint* matrixTransform(DOMMatrixInit*, ExceptionState&);

 protected:
  double x_;
  double y_;
  double z_;
  double w_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_GEOMETRY_DOM_POINT_READ_ONLY_H_
