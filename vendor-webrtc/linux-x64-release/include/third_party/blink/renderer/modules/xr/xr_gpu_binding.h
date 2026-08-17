// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_GPU_BINDING_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_GPU_BINDING_H_

#include "device/vr/public/mojom/vr_service.mojom-blink.h"
#include "third_party/blink/renderer/modules/xr/xr_graphics_binding.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace gfx {

class Rect;

}  // namespace gfx

namespace blink {

class ExceptionState;
class GPUDevice;
class V8GPUTextureFormat;
class XRSession;
class XRView;
class XRViewData;
class XRProjectionLayer;
class XRGPUProjectionLayerInit;
class XRGPUSubImage;

class XRGPUBinding final : public ScriptWrappable, public XRGraphicsBinding {
  DEFINE_WRAPPERTYPEINFO();

 public:
  XRGPUBinding(XRSession*, GPUDevice*);
  ~XRGPUBinding() override = default;

  static XRGPUBinding* Create(XRSession* session,
                              GPUDevice* device,
                              ExceptionState& exception_state);

  XRProjectionLayer* createProjectionLayer(const XRGPUProjectionLayerInit* init,
                                           ExceptionState& exception_state);

  XRGPUSubImage* getViewSubImage(XRProjectionLayer* layer,
                                 XRView* view,
                                 ExceptionState& exception_state);

  V8GPUTextureFormat getPreferredColorFormat();

  GPUDevice* device() const { return device_.Get(); }

  gfx::Rect GetViewportForView(XRProjectionLayer* layer,
                               XRViewData* view) override;

  void Trace(Visitor*) const override;

 private:
  bool CanCreateLayer(ExceptionState& exception_state);
  bool ValidateFormats(const XRGPUProjectionLayerInit* init,
                       ExceptionState& exception_state);

  Member<GPUDevice> device_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_GPU_BINDING_H_
