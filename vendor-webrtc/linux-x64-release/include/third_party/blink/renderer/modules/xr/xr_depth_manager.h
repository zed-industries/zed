// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_DEPTH_MANAGER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_DEPTH_MANAGER_H_

#include "base/types/pass_key.h"
#include "device/vr/public/mojom/vr_service.mojom-blink.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_array_buffer.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class ExceptionState;
class XRCPUDepthInformation;
class XRWebGLDepthInformation;
class XRFrame;
class XRViewData;

// Helper class, used to separate the code related to depth buffer processing
// out of XRViewData.
class XRDepthManager : public GarbageCollected<XRDepthManager> {
 public:
  explicit XRDepthManager(
      base::PassKey<XRViewData> pass_key,
      const device::mojom::blink::XRDepthConfig& device_configuration);
  virtual ~XRDepthManager();

  void ProcessDepthInformation(device::mojom::blink::XRDepthDataPtr depth_data);

  XRCPUDepthInformation* GetCpuDepthInformation(
      const XRFrame* xr_frame,
      ExceptionState& exception_state);

  XRWebGLDepthInformation* GetWebGLDepthInformation(
      const XRFrame* xr_frame,
      ExceptionState& exception_state);

  void Trace(Visitor* visitor) const;

 private:
  const device::mojom::XRDepthUsage usage_;
  const device::mojom::XRDepthDataFormat data_format_;

  // Current depth data buffer.
  device::mojom::blink::XRDepthDataUpdatedPtr depth_data_;

  // Cached version of the depth buffer data. If not null, contains the same
  // information as |depth_data_.pixel_data| buffer.
  Member<DOMArrayBuffer> data_;

  void EnsureData();
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_DEPTH_MANAGER_H_
