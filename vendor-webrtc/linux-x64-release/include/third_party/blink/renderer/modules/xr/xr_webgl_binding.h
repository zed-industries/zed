// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_WEBGL_BINDING_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_WEBGL_BINDING_H_

#include "third_party/blink/renderer/modules/webgl/webgl2_rendering_context.h"
#include "third_party/blink/renderer/modules/webgl/webgl_rendering_context.h"
#include "third_party/blink/renderer/modules/xr/xr_graphics_binding.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class ExceptionState;
class WebGLRenderingContextBase;
class WebGLTexture;
class XRCamera;
class XRLightProbe;
class XRSession;
class XRView;
class XRWebGLDepthInformation;
class XRProjectionLayer;
class XRProjectionLayerInit;
class XRWebGLSubImage;

class XRWebGLBinding final : public ScriptWrappable, public XRGraphicsBinding {
  DEFINE_WRAPPERTYPEINFO();

 public:
  XRWebGLBinding(XRSession*, WebGLRenderingContextBase*, bool webgl2);
  ~XRWebGLBinding() override = default;

  static XRWebGLBinding* Create(XRSession* session,
                                const V8XRWebGLRenderingContext* context,
                                ExceptionState& exception_state);

  bool usesDepthValues() const;

  XRProjectionLayer* createProjectionLayer(const XRProjectionLayerInit* init,
                                           ExceptionState& exception_state);

  XRWebGLSubImage* getViewSubImage(XRProjectionLayer* layer,
                                   XRView* view,
                                   ExceptionState& exception_state);

  WebGLTexture* getReflectionCubeMap(XRLightProbe*, ExceptionState&);

  WebGLTexture* getCameraImage(XRCamera* camera,
                               ExceptionState& exception_state);

  XRWebGLDepthInformation* getDepthInformation(XRView* view,
                                               ExceptionState& exception_state);

  gfx::Rect GetViewportForView(XRProjectionLayer* layer,
                               XRViewData* view) override;

  WebGLRenderingContextBase* context() const { return webgl_context_.Get(); }

  void Trace(Visitor*) const override;

 private:
  bool CanCreateLayer(ExceptionState& exception_state);
  bool ValidateLayerColorFormat(GLenum color_format,
                                ExceptionState& exception_state);
  bool ValidateLayerDepthStencilFormat(GLenum depth_stencil_format,
                                       ExceptionState& exception_state);
  GLenum FormatForLayerFormat(GLenum format);
  GLenum InternalFormatForLayerFormat(GLenum format);
  GLenum TypeForLayerFormat(GLenum format);

  Member<WebGLRenderingContextBase> webgl_context_;
  bool webgl2_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_WEBGL_BINDING_H_
