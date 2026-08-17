// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CANVAS_CANVAS_RENDERING_CONTEXT_HOST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CANVAS_CANVAS_RENDERING_CONTEXT_HOST_H_

#include "services/metrics/public/cpp/ukm_recorder.h"
#include "third_party/blink/public/common/privacy_budget/identifiable_token.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/events/event_dispatcher.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/fileapi/blob.h"
#include "third_party/blink/renderer/core/html/canvas/canvas_image_source.h"
#include "third_party/blink/renderer/core/html/canvas/ukm_parameters.h"
#include "third_party/blink/renderer/platform/bindings/exception_state.h"
#include "third_party/blink/renderer/platform/graphics/canvas_resource_host.h"
#include "third_party/blink/renderer/platform/graphics/canvas_resource_provider.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/text/text_direction.h"
#include "ui/gfx/geometry/size.h"

namespace blink {

class CanvasRenderingContext;
class CanvasResource;
class CanvasResourceDispatcher;
class ComputedStyle;
class KURL;
class LayoutLocale;
class PlainTextPainter;
class StaticBitmapImage;
class UniqueFontSelector;

class CORE_EXPORT CanvasRenderingContextHost : public GarbageCollectedMixin,
                                               public CanvasResourceHost,
                                               public CanvasImageSource,
                                               public ImageBitmapSource {
 public:
  enum class HostType {
    kNone,
    kCanvasHost,
    kOffscreenCanvasHost,
  };
  CanvasRenderingContextHost(HostType host_type, const gfx::Size& size);
  void Trace(Visitor* visitor) const override;

  void RecordCanvasSizeToUMA();

  virtual void DetachContext() = 0;

  virtual void DidDraw(const SkIRect& rect) = 0;
  void DidDraw() { DidDraw(SkIRect::MakeWH(width(), height())); }

  virtual void PreFinalizeFrame() = 0;
  virtual void PostFinalizeFrame(FlushReason) = 0;
  virtual bool PushFrame(scoped_refptr<CanvasResource>&& frame,
                         const SkIRect& damage_rect) = 0;
  virtual bool OriginClean() const = 0;
  virtual void SetOriginTainted() = 0;
  virtual CanvasRenderingContext* RenderingContext() const = 0;
  virtual CanvasResourceDispatcher* GetOrCreateResourceDispatcher() = 0;
  virtual void DiscardResourceDispatcher() = 0;

  virtual ExecutionContext* GetTopExecutionContext() const = 0;
  virtual DispatchEventResult HostDispatchEvent(Event*) = 0;
  virtual const KURL& GetExecutionContextUrl() const = 0;

  // If WebGL1 is disabled by enterprise policy or command line switch.
  virtual bool IsWebGL1Enabled() const = 0;
  // If WebGL2 is disabled by enterprise policy or command line switch.
  virtual bool IsWebGL2Enabled() const = 0;
  // If WebGL is temporarily blocked because WebGL contexts were lost one or
  // more times, in particular, via the GL_ARB_robustness extension.
  virtual bool IsWebGLBlocked() const = 0;
  virtual void SetContextCreationWasBlocked() {}

  // The ComputedStyle argument is optional. Use it if you already have the
  // computed style for the host. If nullptr is passed, the style will be
  // computed within the method.
  virtual TextDirection GetTextDirection(const ComputedStyle*) = 0;
  virtual const LayoutLocale* GetLocale() const = 0;
  virtual UniqueFontSelector* GetFontSelector() = 0;

  virtual bool ShouldAccelerate2dContext() const = 0;

  virtual void Commit(scoped_refptr<CanvasResource>&& canvas_resource,
                      const SkIRect& damage_rect);

  virtual UkmParameters GetUkmParameters() = 0;

  bool IsPaintable() const;

  bool PrintedInCurrentTask() const final;

  // Required by template functions in WebGLRenderingContextBase
  int width() const { return Size().width(); }
  int height() const { return Size().height(); }

  // Partial CanvasResourceHost implementation
  void InitializeForRecording(cc::PaintCanvas*) const final;
  CanvasResourceProvider* GetOrCreateCanvasResourceProvider() override;
  void PageVisibilityChanged() override;

  bool IsWebGL() const;
  bool IsWebGPU() const;
  bool IsRenderingContext2D() const;
  bool IsImageBitmapRenderingContext() const;

  SkAlphaType GetRenderingContextAlphaType() const;
  viz::SharedImageFormat GetRenderingContextFormat() const;
  gfx::ColorSpace GetRenderingContextColorSpace() const;
  PlainTextPainter& GetPlainTextPainter();

  // blink::CanvasImageSource
  bool IsOffscreenCanvas() const override;

  // ImageBitmapSource implementation
  ImageBitmapSourceStatus CheckUsability() const override;

  // This method attempts to ensure that the canvas' resource exists on the GPU.
  // A HTMLCanvasElement can downgrade itself from GPU to CPU when readback
  // occurs too frequently, so a canvas may exist on the CPU even if the browser
  // is normally GPU-capable.
  // Returns true if the canvas resources live on the GPU. If the canvas needed
  // to be migrated off of the CPU, the canvas resource provider and canvas 2D
  // layer bridge will be destroyed and recreated; when this occurs, any
  // existing pointers to these objects will be invalidated. If the canvas
  // resource provider did not exist at all, it may be created.
  virtual bool EnableAcceleration() = 0;

  bool IsContextLost() const override;

 protected:
  ~CanvasRenderingContextHost() override = default;

  scoped_refptr<StaticBitmapImage> CreateTransparentImage(
      const gfx::Size&) const;

  CanvasResourceProvider* GetOrCreateCanvasResourceProviderImpl() final;
  void CreateCanvasResourceProvider2D();
  void CreateCanvasResourceProviderWebGL();
  void CreateCanvasResourceProviderWebGPU();

  bool ContextHasOpenLayers(const CanvasRenderingContext*) const;

  // Computes the digest that corresponds to the "input" of this canvas,
  // including the context type, and if applicable, canvas digest, and taint
  // bits.
  IdentifiableToken IdentifiabilityInputDigest(
      const CanvasRenderingContext* const context) const;

  Member<PlainTextPainter> plain_text_painter_;
  Member<UniqueFontSelector> unique_font_selector_;
  // `did_fail_to_create_resource_provider_` prevents repeated attempts in
  // allocating resources after the first attempt failed.
  bool did_fail_to_create_resource_provider_ = false;
  bool did_record_canvas_size_to_uma_ = false;
  HostType host_type_ = HostType::kNone;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CANVAS_CANVAS_RENDERING_CONTEXT_HOST_H_
