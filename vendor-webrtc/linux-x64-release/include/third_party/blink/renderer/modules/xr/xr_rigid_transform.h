// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_RIGID_TRANSFORM_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_RIGID_TRANSFORM_H_

#include <memory>

#include "third_party/blink/renderer/core/typed_arrays/array_buffer_view_helpers.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_typed_array.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace gfx {
class Transform;
}

namespace blink {

class DOMPointInit;
class DOMPointReadOnly;
class ExceptionState;

// MODULES_EXPORT is required for unit tests using XRRigidTransform (currently
// just xr_rigid_transform_test.cc) to build without linker errors.
class MODULES_EXPORT XRRigidTransform : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit XRRigidTransform(const gfx::Transform&);
  XRRigidTransform(DOMPointInit*, DOMPointInit*);
  static XRRigidTransform* Create(DOMPointInit*,
                                  DOMPointInit*,
                                  ExceptionState&);

  XRRigidTransform(const XRRigidTransform&) = delete;
  XRRigidTransform& operator=(const XRRigidTransform&) = delete;

  ~XRRigidTransform() override = default;

  DOMPointReadOnly* position() const { return position_.Get(); }
  DOMPointReadOnly* orientation() const { return orientation_.Get(); }
  NotShared<DOMFloat32Array> matrix();
  XRRigidTransform* inverse();

  gfx::Transform InverseTransformMatrix();
  gfx::Transform TransformMatrix();  // copies matrix_

  void Trace(Visitor*) const override;

 private:
  void DecomposeMatrix();
  void EnsureMatrix();
  void EnsureInverse();

  Member<DOMPointReadOnly> position_;
  Member<DOMPointReadOnly> orientation_;
  Member<XRRigidTransform> inverse_;
  NotShared<DOMFloat32Array> matrix_array_;
  std::unique_ptr<gfx::Transform> matrix_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_RIGID_TRANSFORM_H_
