// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_VR_SERVICE_TYPE_CONVERTERS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_VR_SERVICE_TYPE_CONVERTERS_H_

#include <optional>

#include "device/vr/public/mojom/vr_service.mojom-blink.h"
#include "third_party/blink/renderer/core/geometry/dom_point_read_only.h"
#include "third_party/blink/renderer/modules/xr/xr_plane.h"
#include "ui/gfx/geometry/transform.h"

namespace mojo {

template <>
struct TypeConverter<std::optional<blink::XRPlane::Orientation>,
                     device::mojom::blink::XRPlaneOrientation> {
  static std::optional<blink::XRPlane::Orientation> Convert(
      const device::mojom::blink::XRPlaneOrientation& orientation);
};

template <>
struct TypeConverter<blink::HeapVector<blink::Member<blink::DOMPointReadOnly>>,
                     Vector<device::mojom::blink::XRPlanePointDataPtr>> {
  static blink::HeapVector<blink::Member<blink::DOMPointReadOnly>> Convert(
      const Vector<device::mojom::blink::XRPlanePointDataPtr>& vertices);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_VR_SERVICE_TYPE_CONVERTERS_H_
