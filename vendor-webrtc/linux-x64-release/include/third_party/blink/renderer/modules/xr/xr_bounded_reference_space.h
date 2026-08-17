// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_BOUNDED_REFERENCE_SPACE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_BOUNDED_REFERENCE_SPACE_H_

#include <memory>
#include <optional>

#include "third_party/blink/renderer/core/geometry/dom_point_read_only.h"
#include "third_party/blink/renderer/modules/xr/xr_reference_space.h"
#include "ui/gfx/geometry/transform.h"

namespace blink {

template <typename IDLType>
class FrozenArray;

class XRBoundedReferenceSpace final : public XRReferenceSpace {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit XRBoundedReferenceSpace(XRSession*);
  XRBoundedReferenceSpace(XRSession*, XRRigidTransform*);
  ~XRBoundedReferenceSpace() override;

  std::optional<gfx::Transform> MojoFromNative() const override;

  const FrozenArray<DOMPointReadOnly>& boundsGeometry() const;

  void Trace(Visitor*) const override;

  void OnReset() override;

 private:
  XRBoundedReferenceSpace* cloneWithOriginOffset(
      XRRigidTransform* origin_offset) const override;

  void EnsureUpdated() const;

  mutable Member<FrozenArray<DOMPointReadOnly>> offset_bounds_geometry_;
  mutable std::unique_ptr<gfx::Transform> mojo_from_bounded_native_;
  mutable uint32_t stage_parameters_id_ = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_BOUNDED_REFERENCE_SPACE_H_
