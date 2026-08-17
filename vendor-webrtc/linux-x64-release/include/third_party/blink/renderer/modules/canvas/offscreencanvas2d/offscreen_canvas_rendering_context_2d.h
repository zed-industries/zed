// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_OFFSCREENCANVAS2D_OFFSCREEN_CANVAS_RENDERING_CONTEXT_2D_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_OFFSCREENCANVAS2D_OFFSCREEN_CANVAS_RENDERING_CONTEXT_2D_H_

#include "base/notreached.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_typedefs.h"
#include "third_party/blink/renderer/core/canvas_interventions/canvas_interventions_enums.h"
#include "third_party/blink/renderer/core/html/canvas/canvas_2d_color_params.h"
#include "third_party/blink/renderer/core/html/canvas/canvas_context_creation_attributes_core.h"
#include "third_party/blink/renderer/core/html/canvas/canvas_rendering_context.h"
#include "third_party/blink/renderer/core/html/canvas/canvas_rendering_context_factory.h"
#include "third_party/blink/renderer/core/offscreencanvas/offscreen_canvas.h"
#include "third_party/blink/renderer/modules/canvas/canvas2d/base_rendering_context_2d.h"
#include "third_party/blink/renderer/modules/canvas/canvas2d/identifiability_study_helper.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/graphics/canvas_resource_provider.h"
#include "third_party/blink/renderer/platform/graphics/skia/skia_utils.h"
#include "third_party/blink/renderer/platform/privacy_budget/identifiability_digest_helpers.h"

namespace blink {

class CanvasResourceProvider;
class ExceptionState;

class MODULES_EXPORT OffscreenCanvasRenderingContext2D final
    : public ScriptWrappable,
      public BaseRenderingContext2D {
  DEFINE_WRAPPERTYPEINFO();

 public:
  class Factory : public CanvasRenderingContextFactory {
   public:
    Factory() = default;
    ~Factory() override = default;

    CanvasRenderingContext* Create(
        CanvasRenderingContextHost* host,
        const CanvasContextCreationAttributesCore& attrs) override;

    CanvasRenderingContext::CanvasRenderingAPI GetRenderingAPI()
        const override {
      return CanvasRenderingContext::CanvasRenderingAPI::k2D;
    }
  };

  OffscreenCanvasRenderingContext2D(
      OffscreenCanvas*,
      const CanvasContextCreationAttributesCore& attrs);

  OffscreenCanvas* offscreenCanvasForBinding() const {
    DCHECK(!Host() || Host()->IsOffscreenCanvas());
    return static_cast<OffscreenCanvas*>(Host());
  }

  // CanvasRenderingContext implementation
  ~OffscreenCanvasRenderingContext2D() override;
  bool IsComposited() const override { return false; }
  V8RenderingContext* AsV8RenderingContext() final;
  V8OffscreenRenderingContext* AsV8OffscreenRenderingContext() final;
  void Stop() final { NOTREACHED(); }
  scoped_refptr<StaticBitmapImage> GetImage(FlushReason) final;
  void Reset() override;
  // CanvasRenderingContext - ActiveScriptWrappable
  // This method will avoid this class to be garbage collected, as soon as
  // HasPendingActivity returns true.
  bool HasPendingActivity() const final {
    if (!Host())
      return false;
    DCHECK(Host()->IsOffscreenCanvas());
    return static_cast<OffscreenCanvas*>(Host())->HasPlaceholderCanvas() &&
           !dirty_rect_for_commit_.isEmpty();
  }

  // BaseRenderingContext2D implementation
  bool OriginClean() const final;
  void SetOriginTainted() final;

  int Width() const final;
  int Height() const final;

  bool CanCreateCanvas2dResourceProvider() const final;
  CanvasResourceProvider* GetOrCreateCanvasResourceProvider() const;
  CanvasResourceProvider* GetCanvasResourceProvider() const;

  // Offscreen canvas doesn't have any notion of image orientation.
  RespectImageOrientationEnum RespectImageOrientation() const final {
    return kRespectImageOrientation;
  }

  Color GetCurrentColor() const final;

  cc::PaintCanvas* GetOrCreatePaintCanvas() final;
  using BaseRenderingContext2D::GetPaintCanvas;  // Pull the non-const overload.
  const cc::PaintCanvas* GetPaintCanvas() const final;
  const MemoryManagedPaintRecorder* Recorder() const final;

  void WillDraw(const SkIRect& dirty_rect,
                CanvasPerformanceMonitor::DrawType) final;

  sk_sp<PaintFilter> StateGetFilter() final;

  bool HasAlpha() const final { return CreationAttributes().alpha; }
  bool IsDesynchronized() const final {
    return CreationAttributes().desynchronized;
  }
  void LoseContext(LostContextMode) override;

  ImageBitmap* TransferToImageBitmap(ScriptState* script_state,
                                     ExceptionState& exception_state) final;

  void Trace(Visitor*) const override;

  bool PushFrame() override;

  CanvasRenderingContextHost* GetCanvasRenderingContextHost() const override;
  ExecutionContext* GetTopExecutionContext() const override;

  IdentifiableToken IdentifiableTextToken() const override {
    return identifiability_study_helper_.GetToken();
  }

  bool IdentifiabilityEncounteredSkippedOps() const override {
    return identifiability_study_helper_.encountered_skipped_ops();
  }

  bool IdentifiabilityEncounteredSensitiveOps() const override {
    return identifiability_study_helper_.encountered_sensitive_ops();
  }

  bool IdentifiabilityEncounteredPartiallyDigestedImage() const override {
    return identifiability_study_helper_.encountered_partially_digested_image();
  }

  bool ShouldTriggerIntervention() const override {
    return HasTriggerForIntervention();
  }

  CanvasOperationType GetCanvasTriggerOperations() const override {
    return GetTriggersForIntervention();
  }

  std::optional<cc::PaintRecord> FlushCanvas(FlushReason) override;

 protected:
  OffscreenCanvas* HostAsOffscreenCanvas() const final;
  UniqueFontSelector* GetFontSelector() const final;

  bool WritePixels(const SkImageInfo& orig_info,
                   const void* pixels,
                   size_t row_bytes,
                   int x,
                   int y) override;

  bool ResolveFont(const String& new_font) override;

 private:
  void FinalizeFrame(FlushReason) final;

  bool IsPaintable() const final;
  bool IsCanvas2DBufferValid() const override;

  scoped_refptr<CanvasResource> ProduceCanvasResource(FlushReason);

  CanvasResourceProvider* GetOrCreateCanvas2DResourceProvider() override;

  SkIRect dirty_rect_for_commit_;

  bool is_valid_size_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_OFFSCREENCANVAS2D_OFFSCREEN_CANVAS_RENDERING_CONTEXT_2D_H_
