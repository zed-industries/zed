// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_CANVAS_CONTEXT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_CANVAS_CONTEXT_H_

#include "base/containers/heap_array.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_gpu_canvas_alpha_mode.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_gpu_canvas_tone_mapping_mode.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_typedefs.h"
#include "third_party/blink/renderer/core/html/canvas/canvas_rendering_context.h"
#include "third_party/blink/renderer/core/html/canvas/canvas_rendering_context_factory.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/graphics/gpu/webgpu_cpp.h"
#include "third_party/blink/renderer/platform/graphics/gpu/webgpu_swap_buffer_provider.h"
#include "third_party/blink/renderer/platform/graphics/predefined_color_space.h"
#include "third_party/blink/renderer/platform/graphics/static_bitmap_image.h"

namespace blink {

class ExceptionState;
class GPUDevice;
class GPUCanvasConfiguration;
class GPUSwapChain;
class GPUTexture;
class WebGPUTextureAlphaClearer;
class V8UnionHTMLCanvasElementOrOffscreenCanvas;

// A GPUCanvasContext does little by itself and basically just binds a canvas
// and a GPUSwapChain together and forwards calls from one to the other.
class GPUCanvasContext : public ScriptWrappable,
                         public CanvasRenderingContext,
                         public WebGPUSwapBufferProvider::Client {
  DEFINE_WRAPPERTYPEINFO();

 public:
  class Factory : public CanvasRenderingContextFactory {
   public:
    Factory() = default;

    Factory(const Factory&) = delete;
    Factory& operator=(const Factory&) = delete;

    ~Factory() override;

    CanvasRenderingContext* Create(
        CanvasRenderingContextHost*,
        const CanvasContextCreationAttributesCore&) override;
    CanvasRenderingContext::CanvasRenderingAPI GetRenderingAPI() const override;
  };

  GPUCanvasContext(CanvasRenderingContextHost*,
                   const CanvasContextCreationAttributesCore&);

  GPUCanvasContext(const GPUCanvasContext&) = delete;
  GPUCanvasContext& operator=(const GPUCanvasContext&) = delete;

  ~GPUCanvasContext() override;

  void Trace(Visitor*) const override;

  // CanvasRenderingContext implementation
  V8RenderingContext* AsV8RenderingContext() final;
  V8OffscreenRenderingContext* AsV8OffscreenRenderingContext() final;
  SkAlphaType GetAlphaType() const override;
  viz::SharedImageFormat GetSharedImageFormat() const override;
  gfx::ColorSpace GetColorSpace() const override;
  // Produces a snapshot of the current contents of the swap chain if possible
  // or else a snapshot of the most-recently presented contents.
  scoped_refptr<StaticBitmapImage> GetImage(FlushReason) final;
  bool PaintRenderingResultsToCanvas(SourceDrawingBuffer) final;
  bool CopyRenderingResultsToVideoFrame(
      WebGraphicsContext3DVideoFramePool* frame_pool,
      SourceDrawingBuffer src_buffer,
      const gfx::ColorSpace& dst_color_space,
      VideoFrameCopyCompletedCallback callback) override;
  void PageVisibilityChanged() override {}
  bool isContextLost() const override { return false; }
  bool IsComposited() const final { return true; }
  bool IsPaintable() const final { return true; }
  int ExternallyAllocatedBufferCountPerPixel() final { return 1; }
  void Stop() final;
  cc::Layer* CcLayer() const final;
  void Reshape(int width, int height) override;

  // OffscreenCanvas-specific methods
  bool PushFrame() final;
  // Returns a StaticBitmapImage backed by a texture containing the current
  // contents of the front buffer. This is done without any pixel copies. The
  // texture in the ImageBitmap is from the active ContextProvider on the
  // WebGPUSwapBufferProvider.
  ImageBitmap* TransferToImageBitmap(ScriptState*, ExceptionState&) final;

  bool IsOffscreenCanvas() const {
    if (Host())
      return Host()->IsOffscreenCanvas();
    return false;
  }

  // gpu_canvas_context.idl {{{
  V8UnionHTMLCanvasElementOrOffscreenCanvas* getHTMLOrOffscreenCanvas() const;
  void configure(const GPUCanvasConfiguration* descriptor, ExceptionState&);
  void unconfigure();
  GPUCanvasConfiguration* getConfiguration();
  GPUTexture* getCurrentTexture(ScriptState*, ExceptionState&);
  // }}} End of WebIDL binding implementation.

  // WebGPUSwapBufferProvider::Client implementation
  void OnTextureTransferred() override;
  void InitializeLayer(cc::Layer* layer) override;
  void SetNeedsCompositingUpdate() override;
  bool IsGPUDeviceDestroyed() override;

 private:
  scoped_refptr<WebGPUMailboxTexture> GetFrontBufferMailboxTexture();
  void DetachSwapBuffers();
  void ReplaceDrawingBuffer(bool destroy_swap_buffers);
  void InitializeAlphaModePipeline(wgpu::TextureFormat format);

  void FinalizeFrame(FlushReason) override;

  scoped_refptr<StaticBitmapImage> SnapshotInternal(
      const wgpu::Texture& texture) const;

  bool CopyTextureToResourceProvider(
      const wgpu::Texture& texture,
      CanvasResourceProvider* resource_provider) const;

  void CopyToSwapTexture();

  base::WeakPtr<WebGraphicsContext3DProviderWrapper> GetContextProviderWeakPtr()
      const;

  scoped_refptr<StaticBitmapImage> MakeFallbackStaticBitmapImage(
      V8GPUCanvasAlphaMode::Enum alpha_mode);

  Member<GPUDevice> device_;

  // If the system doesn't support the requested format but it's one that WebGPU
  // is required to offer, a texture_ will be allocated separately with the
  // desired format and the will be copied to swap_texture_, allocated by the
  // swap buffer provider with the system-supported format, when we're ready to
  // present. Otherwise texture_ and swap_texture_ will point to the same
  // texture, allocated by the swap buffer provider.
  Member<GPUTexture> texture_;
  Member<GPUTexture> swap_texture_;

  PredefinedColorSpace color_space_ = PredefinedColorSpace::kSRGB;
  V8GPUCanvasAlphaMode::Enum alpha_mode_;
  V8GPUCanvasToneMappingMode::Enum tone_mapping_mode_;
  scoped_refptr<WebGPUTextureAlphaClearer> alpha_clearer_;
  scoped_refptr<WebGPUSwapBufferProvider> swap_buffers_;

  bool new_texture_required_ = true;
  bool copy_to_swap_texture_required_ = false;
  bool suppress_preferred_format_warning_ = false;
  bool stopped_ = false;

  // Matches [[configuration]] != null in the WebGPU specification.
  bool configured_ = false;
  // Matches [[texture_descriptor]] in the WebGPU specification except that it
  // never becomes null.
  wgpu::TextureDescriptor texture_descriptor_;
  // The texture descriptor for the swap_texture is tracked separately, since
  // it may have different usage in the case that a copy is required.
  wgpu::TextureDescriptor swap_texture_descriptor_;
  wgpu::DawnTextureInternalUsageDescriptor texture_internal_usage_;
  base::HeapArray<wgpu::TextureFormat> view_formats_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_CANVAS_CONTEXT_H_
