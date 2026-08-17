// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_CPU_DEPTH_INFORMATION_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_CPU_DEPTH_INFORMATION_H_

#include "device/vr/public/mojom/xr_session.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_array_buffer.h"
#include "third_party/blink/renderer/modules/xr/xr_depth_information.h"

namespace gfx {
class Size;
class Transform;
}  // namespace gfx

namespace blink {

class ExceptionState;
class XRFrame;

class XRCPUDepthInformation final : public XRDepthInformation {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit XRCPUDepthInformation(
      const XRFrame* xr_frame,
      const gfx::Size& size,
      const gfx::Transform& norm_texture_from_norm_view,
      float raw_value_to_meters,
      device::mojom::XRDepthDataFormat data_format,
      DOMArrayBuffer* data);

  DOMArrayBuffer* data(ExceptionState& exception_state) const;

  float getDepthInMeters(float x,
                         float y,
                         ExceptionState& exception_state) const;

  void Trace(Visitor* visitor) const override;

 private:
  const Member<DOMArrayBuffer> data_;
  const device::mojom::XRDepthDataFormat data_format_;
  const size_t bytes_per_element_;

  // Helper, returns value at `index` in `data_`. Depending on `data_format_`,
  // the `data_` will be viewed into via Uint16Array (for luminance-alpha),
  // or via Float32Array (for float32).
  float GetItem(size_t index) const;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_CPU_DEPTH_INFORMATION_H_
