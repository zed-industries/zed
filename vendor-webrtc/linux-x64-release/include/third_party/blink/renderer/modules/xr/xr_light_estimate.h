// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_LIGHT_ESTIMATE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_LIGHT_ESTIMATE_H_

#include "device/vr/public/mojom/vr_service.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/typed_arrays/array_buffer_view_helpers.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_typed_array.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class DOMPointReadOnly;

class XRLightEstimate : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit XRLightEstimate(const device::mojom::blink::XRLightProbe&);

  NotShared<DOMFloat32Array> sphericalHarmonicsCoefficients() const {
    return sh_coefficients_;
  }

  DOMPointReadOnly* primaryLightDirection() const {
    return primary_light_direction_.Get();
  }

  DOMPointReadOnly* primaryLightIntensity() const {
    return primary_light_intensity_.Get();
  }

  void Trace(Visitor* visitor) const override;

 private:
  NotShared<DOMFloat32Array> sh_coefficients_;
  Member<DOMPointReadOnly> primary_light_direction_;
  Member<DOMPointReadOnly> primary_light_intensity_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_LIGHT_ESTIMATE_H_
