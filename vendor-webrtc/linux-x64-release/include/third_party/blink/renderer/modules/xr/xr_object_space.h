// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_OBJECT_SPACE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_OBJECT_SPACE_H_

#include <string>

#include "device/vr/public/mojom/vr_service.mojom-blink-forward.h"
#include "third_party/blink/renderer/modules/xr/xr_space.h"
#include "ui/gfx/geometry/transform.h"

namespace blink {

class XRSession;

// Helper class that returns an XRSpace that tracks the position of object of
// type T (for example XRPlane, XRAnchor). The type T has to have a
// NativeOrigin() method, returning a
// device::mojom::blink::XRNativeOriginInformationPtr, a MojoFromObject()
// method, returning a std::optional<gfx::Transform>, and IsStationary()
// method returning true if the object is supposed to be treated as stationary
// for the purposes of anchor creation.
//
// If the object's MojoFromObject() method returns a std::nullopt, it means
// that the object is not localizable in the current frame (i.e. its pose is
// unknown) - the `frame.getPose(objectSpace, otherSpace)` will return null.
// That does not necessarily mean that object tracking is lost - it may be that
// the object's location will become known in subsequent frames.
template <typename T>
class XRObjectSpace final : public XRSpace {
 public:
  explicit XRObjectSpace(XRSession* session, const T* object)
      : XRSpace(session),
        object_(object),
        is_stationary_(object->IsStationary()) {}

  std::optional<gfx::Transform> MojoFromNative() const override {
    return object_->MojoFromObject();
  }

  device::mojom::blink::XRNativeOriginInformationPtr NativeOrigin()
      const override {
    return object_->NativeOrigin();
  }

  bool IsStationary() const override { return is_stationary_; }

  std::string ToString() const override { return "XRObjectSpace"; }

  void Trace(Visitor* visitor) const override {
    visitor->Trace(object_);
    XRSpace::Trace(visitor);
  }

 private:
  Member<const T> object_;
  const bool is_stationary_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_OBJECT_SPACE_H_
