// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_RAY_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_RAY_H_

#include <memory>

#include "third_party/blink/renderer/core/typed_arrays/array_buffer_view_helpers.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_typed_array.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "ui/gfx/geometry/point3_f.h"
#include "ui/gfx/geometry/transform.h"
#include "ui/gfx/geometry/vector3d_f.h"

namespace blink {

class DOMPointInit;
class DOMPointReadOnly;
class ExceptionState;
class XRRayDirectionInit;
class XRRigidTransform;

class XRRay final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  XRRay();
  explicit XRRay(XRRigidTransform* transform, ExceptionState& exception_state);
  XRRay(DOMPointInit* origin,
        XRRayDirectionInit* direction,
        ExceptionState& exception_state);
  ~XRRay() override;

  DOMPointReadOnly* origin() const { return origin_.Get(); }
  DOMPointReadOnly* direction() const { return direction_.Get(); }
  NotShared<DOMFloat32Array> matrix();

  // Calling |RawMatrix()| is equivalent to calling |matrix()| w.r.t. the data
  // that will be returned, the only difference is the returned type.
  gfx::Transform RawMatrix();

  static XRRay* Create(DOMPointInit* origin,
                       XRRayDirectionInit* direction,
                       ExceptionState& exception_state);
  static XRRay* Create(XRRigidTransform* transform,
                       ExceptionState& exception_state);

  void Trace(Visitor*) const override;

 private:
  void Set(const gfx::Transform& matrix, ExceptionState& exception_state);
  void Set(gfx::Point3F origin,
           gfx::Vector3dF direction,
           ExceptionState& exception_state);

  Member<DOMPointReadOnly> origin_;
  Member<DOMPointReadOnly> direction_;
  NotShared<DOMFloat32Array> matrix_;
  std::unique_ptr<gfx::Transform> raw_matrix_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_RAY_H_
