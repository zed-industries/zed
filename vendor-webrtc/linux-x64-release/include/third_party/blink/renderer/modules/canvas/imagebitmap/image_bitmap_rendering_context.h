// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_IMAGEBITMAP_IMAGE_BITMAP_RENDERING_CONTEXT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_IMAGEBITMAP_IMAGE_BITMAP_RENDERING_CONTEXT_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_typedefs.h"
#include "third_party/blink/renderer/core/html/canvas/canvas_rendering_context.h"
#include "third_party/blink/renderer/core/html/canvas/canvas_rendering_context_factory.h"
#include "third_party/blink/renderer/modules/canvas/imagebitmap/image_bitmap_rendering_context_base.h"
#include "third_party/blink/renderer/modules/modules_export.h"

namespace blink {

class ExceptionState;
class ImageBitmap;

class MODULES_EXPORT ImageBitmapRenderingContext final
    : public ImageBitmapRenderingContextBase {
  DEFINE_WRAPPERTYPEINFO();

 public:
  class Factory : public CanvasRenderingContextFactory {
   public:
    Factory() = default;

    Factory(const Factory&) = delete;
    Factory& operator=(const Factory&) = delete;

    ~Factory() override = default;

    CanvasRenderingContext* Create(
        CanvasRenderingContextHost*,
        const CanvasContextCreationAttributesCore&) override;
    CanvasRenderingContext::CanvasRenderingAPI GetRenderingAPI()
        const override {
      return CanvasRenderingContext::CanvasRenderingAPI::kBitmaprenderer;
    }
  };

  ImageBitmapRenderingContext(CanvasRenderingContextHost*,
                              const CanvasContextCreationAttributesCore&);

  // Script API
  void transferFromImageBitmap(ImageBitmap*, ExceptionState&);

  // CanvasRenderingContext implementation
  ImageBitmap* TransferToImageBitmap(ScriptState*, ExceptionState&) override;

  V8RenderingContext* AsV8RenderingContext() final;
  V8OffscreenRenderingContext* AsV8OffscreenRenderingContext() final;

  ~ImageBitmapRenderingContext() override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_IMAGEBITMAP_IMAGE_BITMAP_RENDERING_CONTEXT_H_
