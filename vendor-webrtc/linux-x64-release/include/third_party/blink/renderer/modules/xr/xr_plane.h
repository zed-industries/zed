// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_PLANE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_PLANE_H_

#include <optional>

#include "device/vr/public/mojom/pose.h"
#include "device/vr/public/mojom/vr_service.mojom-blink-forward.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/core/geometry/dom_point_read_only.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "ui/gfx/geometry/transform.h"

namespace blink {

class V8XRPlaneOrientation;
class XRSession;
class XRSpace;

template <typename IDLType>
class FrozenArray;

class XRPlane : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  enum class Orientation { kHorizontal, kVertical };

  XRPlane(uint64_t id,
          XRSession* session,
          const device::mojom::blink::XRPlaneData& plane_data,
          double timestamp);

  uint64_t id() const;

  XRSpace* planeSpace() const;

  std::optional<gfx::Transform> MojoFromObject() const;

  device::mojom::blink::XRNativeOriginInformationPtr NativeOrigin() const;

  std::optional<V8XRPlaneOrientation> orientation() const;
  const FrozenArray<DOMPointReadOnly>& polygon() const;
  double lastChangedTime() const;

  // Updates plane data from passed in |plane_data|. The resulting instance
  // should be equivalent to the instance that would be create by calling
  // XRPlane(plane_data).
  void Update(const device::mojom::blink::XRPlaneData& plane_data,
              double timestamp);

  bool IsStationary() const {
    // Plane objects are not considered stationary since their pose may vary
    // dramatically from frame to frame (it depends on the plane boundary,
    // drastic changes in plane boundary will cause the pose to change a lot).
    return false;
  }

  void Trace(Visitor* visitor) const override;

 private:
  XRPlane(uint64_t id,
          XRSession* session,
          const std::optional<Orientation>& orientation,
          HeapVector<Member<DOMPointReadOnly>> polygon,
          const std::optional<device::Pose>& mojo_from_plane,
          double timestamp);

  const uint64_t id_;
  Member<FrozenArray<DOMPointReadOnly>> polygon_;
  std::optional<Orientation> orientation_;

  // Plane center's pose in device (mojo) space.  Nullptr if the pose of the
  // anchor is unknown in the current frame.
  std::optional<device::Pose> mojo_from_plane_;

  Member<XRSession> session_;

  double last_changed_time_;

  // Cached plane space - it will be created by `planeSpace()` if it's not set.
  mutable Member<XRSpace> plane_space_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_PLANE_H_
