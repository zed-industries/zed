// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_CANVAS_RESOURCE_PROVIDER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_CANVAS_RESOURCE_PROVIDER_H_

#include <algorithm>
#include <memory>
#include <optional>

#include "base/feature_list.h"
#include "base/memory/raw_ptr.h"
#include "base/notreached.h"
#include "base/time/time.h"
#include "base/timer/timer.h"
#include "cc/paint/skia_paint_canvas.h"
#include "cc/raster/playback_image_provider.h"
#include "gpu/command_buffer/common/shared_image_usage.h"
#include "third_party/blink/renderer/platform/graphics/canvas_resource.h"
#include "third_party/blink/renderer/platform/graphics/canvas_resource_host.h"
#include "third_party/blink/renderer/platform/graphics/image_orientation.h"
#include "third_party/blink/renderer/platform/graphics/memory_managed_paint_recorder.h"
#include "third_party/blink/renderer/platform/graphics/paint/paint_recorder.h"
#include "third_party/blink/renderer/platform/graphics/scoped_raster_timer.h"
#include "third_party/blink/renderer/platform/instrumentation/canvas_memory_dump_provider.h"
#include "third_party/blink/renderer/platform/wtf/thread_specific.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "third_party/skia/include/core/SkSurface.h"
#include "third_party/skia/include/gpu/ganesh/GrTypes.h"

class GrDirectContext;

namespace cc {
class ImageDecodeCache;
class PaintCanvas;
}  // namespace cc

namespace gpu {

struct Mailbox;
struct SyncToken;

namespace gles2 {

class GLES2Interface;

}  // namespace gles2

namespace raster {

class RasterInterface;

}  // namespace raster
}  // namespace gpu

namespace blink {

PLATFORM_EXPORT BASE_DECLARE_FEATURE(kCanvas2DAutoFlushParams);
PLATFORM_EXPORT BASE_DECLARE_FEATURE(kCanvas2DReclaimUnusedResources);

class MemoryManagedPaintCanvas;
class WebGraphicsContext3DProviderWrapper;
class WebGraphicsSharedImageInterfaceProvider;

// CanvasResourceProvider
//==============================================================================
//
// This is an abstract base class that encapsulates a drawable graphics
// resource.  Subclasses manage specific resource types (Gpu Textures,
// GpuMemoryBuffer, Bitmap in RAM). CanvasResourceProvider serves as an
// abstraction layer for these resource types. It is designed to serve
// the needs of Canvas2DLayerBridge, but can also be used as a general purpose
// provider of drawable surfaces for 2D rendering with skia.
//
// General usage:
//   1) Use the Create() static method to create an instance
//   2) use Canvas() to get a drawing interface
//   3) Call Snapshot() to acquire a bitmap with the rendered image in it.

class PLATFORM_EXPORT CanvasResourceProvider
    : public WebGraphicsContext3DProviderWrapper::DestructionObserver,
      public base::CheckedObserver,
      public CanvasMemoryDumpClient,
      public MemoryManagedPaintRecorder::Client,
      public ScopedRasterTimer::Host {
 public:
  // These values are persisted to logs. Entries should not be renumbered and
  // numeric values should never be reused.
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wdeprecated-declarations"
  enum ResourceProviderType {
    kTexture [[deprecated]] = 0,
    kBitmap = 1,
    kSharedBitmap [[deprecated]] = 2,
    kTextureGpuMemoryBuffer [[deprecated]] = 3,
    kBitmapGpuMemoryBuffer [[deprecated]] = 4,
    kSharedImage = 5,
    kDirectGpuMemoryBuffer [[deprecated]] = 6,
    kPassThrough = 7,
    kSwapChain = 8,
    kSkiaDawnSharedImage [[deprecated]] = 9,
    kMaxValue = kSkiaDawnSharedImage,
  };
#pragma GCC diagnostic pop

  // Used to determine if the provider is going to be initialized or not,
  // ignored by PassThrough
  enum class ShouldInitialize { kNo, kCallClear };

  static std::unique_ptr<CanvasResourceProvider> CreateBitmapProvider(
      gfx::Size size,
      viz::SharedImageFormat format,
      SkAlphaType alpha_type,
      const gfx::ColorSpace& color_space,
      ShouldInitialize initialize_provider,
      CanvasResourceHost* resource_host = nullptr);

  static std::unique_ptr<CanvasResourceProvider>
  CreateSharedImageProviderForSoftwareCompositor(
      gfx::Size size,
      viz::SharedImageFormat format,
      SkAlphaType alpha_type,
      const gfx::ColorSpace& color_space,
      ShouldInitialize initialize_provider,
      WebGraphicsSharedImageInterfaceProvider* shared_image_interface_provider,
      CanvasResourceHost* resource_host = nullptr);

  static std::unique_ptr<CanvasResourceProvider> CreateSharedImageProvider(
      gfx::Size size,
      viz::SharedImageFormat format,
      SkAlphaType alpha_type,
      const gfx::ColorSpace& color_space,
      ShouldInitialize initialize_provider,
      base::WeakPtr<WebGraphicsContext3DProviderWrapper>,
      RasterMode raster_mode,
      gpu::SharedImageUsageSet shared_image_usage_flags,
      CanvasResourceHost* resource_host = nullptr);

  static std::unique_ptr<CanvasResourceProvider> CreateWebGPUImageProvider(
      gfx::Size size,
      viz::SharedImageFormat format,
      SkAlphaType alpha_type,
      const gfx::ColorSpace& color_space,
      gpu::SharedImageUsageSet shared_image_usage_flags = {},
      CanvasResourceHost* resource_host = nullptr);

  static std::unique_ptr<CanvasResourceProvider> CreatePassThroughProvider(
      gfx::Size size,
      viz::SharedImageFormat format,
      SkAlphaType alpha_type,
      const gfx::ColorSpace& color_space,
      base::WeakPtr<WebGraphicsContext3DProviderWrapper>,
      CanvasResourceHost* resource_host = nullptr);

  static std::unique_ptr<CanvasResourceProvider> CreateSwapChainProvider(
      gfx::Size size,
      viz::SharedImageFormat format,
      SkAlphaType alpha_type,
      const gfx::ColorSpace& color_space,
      ShouldInitialize initialize_provider,
      base::WeakPtr<WebGraphicsContext3DProviderWrapper>,
      CanvasResourceHost* resource_host = nullptr);

  // Use Snapshot() for capturing a frame that is intended to be displayed via
  // the compositor. Cases that are destined to be transferred via a
  // TransferableResource should call ProduceCanvasResource() instead.
  // The ImageOrientationEnum conveys the desired orientation of the image, and
  // should be derived from the source of the bitmap data.
  virtual scoped_refptr<CanvasResource> ProduceCanvasResource(FlushReason) = 0;
  virtual scoped_refptr<StaticBitmapImage> Snapshot(
      FlushReason,
      ImageOrientation = ImageOrientationEnum::kDefault) = 0;

  void SetCanvasResourceHost(CanvasResourceHost* resource_host) {
    resource_host_ = resource_host;
  }

  // WebGraphicsContext3DProvider::DestructionObserver implementation.
  void OnContextDestroyed() override;

  MemoryManagedPaintCanvas& Canvas(bool needs_will_draw = false);
  // FlushCanvas and preserve recording only if IsPrinting or
  // FlushReason indicates printing in progress.
  std::optional<cc::PaintRecord> FlushCanvas(FlushReason);

  // TODO(crbug.com/371227617): Trim callsites of this method to those that
  // actually need to pass this info to Skia APIs and then eliminate the
  // method/this class holding `info_` by inlining creation of SkImageInfo at
  // those callsites.
  const SkImageInfo& GetSkImageInfo() const { return info_; }
  SkSurfaceProps GetSkSurfaceProps() const;
  viz::SharedImageFormat GetSharedImageFormat() const { return format_; }
  gfx::ColorSpace GetColorSpace() const { return color_space_; }
  SkAlphaType GetAlphaType() const { return alpha_type_; }
  gfx::Size Size() const { return size_; }
  virtual bool IsValid() const = 0;
  virtual bool IsAccelerated() const = 0;
  // Returns true if the resource can be used by the display compositor.
  virtual bool SupportsDirectCompositing() const = 0;
  uint32_t ContentUniqueID() const;

  // Indicates that the compositing path is single buffered, meaning that
  // ProduceCanvasResource() return a reference to the same resource each time,
  // which implies that Producing an animation frame may overwrite the resource
  // used by the previous frame. This results in graphics updates skipping the
  // queue, thus reducing latency, but with the possible side effects of tearing
  // (in cases where the resource is scanned out directly) and irregular frame
  // rate.
  virtual bool IsSingleBuffered() const = 0;

  // Subclasses implementing import of external canvas resources must override
  // this method.
  virtual void ImportResource(scoped_refptr<ExternalCanvasResource>&&) {
    NOTREACHED();
  }

  // CanvasResourceProviderSharedImage overrides these methods as part of
  // implementing resource recycling.
  virtual void OnResourceReturnedFromCompositor(
      scoped_refptr<CanvasResourceSharedImage>&&) {}
  virtual void SetResourceRecyclingEnabled(bool) {}
  virtual void ClearUnusedResources() {}

  SkSurface* GetSkSurface() const;
  bool IsGpuContextLost() const;

  // Returns true iff the resource provider is (a) using a GPU channel for
  // software SharedImages and (b) that channel has been lost.
  virtual bool IsSoftwareSharedImageGpuChannelLost() const;
  static void NotifyGpuContextLostTask(base::WeakPtr<CanvasResourceProvider>);

  virtual bool WritePixels(const SkImageInfo& orig_info,
                           const void* pixels,
                           size_t row_bytes,
                           int x,
                           int y);

  // Returns the ClientSharedImage backing this CanvasResourceProvider, if one
  // exists, after flushing the resource and signaling that an external write
  // will occur on it. The caller should wait on `internal_access_sync_token`
  // before writing the contents unless the caller's usage model makes such a
  // wait unnecessary (in which case the client should pass `nullptr` for the
  // token together with an explanation at the callsite).
  // `required_shared_image_usages` is a set of usages that the passed-back
  // ClientSharedImage must support. A copy will be performed if either (a) the
  // display compositor is reading the current resource or (b) the current
  // resource does not support `required_shared_image_usages.` In these cases,
  // `was_copy_performed` will be set to true if it is non-null.
  virtual scoped_refptr<gpu::ClientSharedImage>
  GetBackingClientSharedImageForExternalWrite(
      gpu::SyncToken* internal_access_sync_token,
      gpu::SharedImageUsageSet required_shared_image_usages,
      bool* was_copy_performed = nullptr) {
    return nullptr;
  }

  // Signals that an external write has completed, passing the token that should
  // be waited on to ensure that the service-side operations of the external
  // write have completed. Ensures that the next read of this resource (whether
  // via raster or the compositor) waits on this token.
  virtual void EndExternalWrite(
      const gpu::SyncToken& external_write_sync_token) {
    NOTREACHED();
  }

  // Returns the ClientSharedImage backing this CanvasResourceProvider, if one
  // exists, for the purpose of allowing the caller to overwrite its contents.
  // First flushes the resource and signals that an external write will occur on
  // it.
  // TODO(crbug.com/340922308): Eliminate this method in favor of all callers
  // calling the above method with explanations at callsites for why they don't
  // need to wait for any internal writes to finish.
  virtual scoped_refptr<gpu::ClientSharedImage>
  GetBackingClientSharedImageForOverwrite() {
    return GetBackingClientSharedImageForExternalWrite(
        /*internal_access_sync_token=*/nullptr, gpu::SharedImageUsageSet());
  }
  virtual gpu::SharedImageUsageSet GetSharedImageUsageFlags() const {
    NOTREACHED();
  }

  CanvasResourceProvider(const CanvasResourceProvider&) = delete;
  CanvasResourceProvider& operator=(const CanvasResourceProvider&) = delete;
  ~CanvasResourceProvider() override;

  base::WeakPtr<CanvasResourceProvider> CreateWeakPtr() {
    return weak_ptr_factory_.GetWeakPtr();
  }

  // Notifies the provider when the texture params associated with |resource|
  // are modified externally from the provider's SkSurface.
  virtual void NotifyTexParamsModified(const CanvasResource* resource) {}

  FlushReason printing_fallback_reason() { return printing_fallback_reason_; }

  void RestoreBackBuffer(const cc::PaintImage&);

  ResourceProviderType GetType() const { return type_; }

  virtual void OnDestroyResource() {}

  virtual void OnAcquireRecyclableCanvasResource() {}
  virtual void OnDestroyRecyclableCanvasResource(
      const gpu::SyncToken& sync_token) {}

  void FlushIfRecordingLimitExceeded();

  const MemoryManagedPaintRecorder& Recorder() const { return *recorder_; }
  MemoryManagedPaintRecorder& Recorder() { return *recorder_; }
  std::unique_ptr<MemoryManagedPaintRecorder> ReleaseRecorder();
  void SetRecorder(std::unique_ptr<MemoryManagedPaintRecorder> recorder);

  void InitializeForRecording(cc::PaintCanvas* canvas) const override;

  bool IsPrinting() { return resource_host_ && resource_host_->IsPrinting(); }

  static void NotifyWillTransfer(cc::PaintImage::ContentId content_id);

  void AlwaysEnableRasterTimersForTesting(bool value) {
    always_enable_raster_timers_for_testing_ = value;
  }

  const std::optional<cc::PaintRecord>& LastRecording() {
    return last_recording_;
  }

  // Overwrites the current image (either completely or partially) with the
  // passed-in SharedImage. Waits on `ready_sync_token` before copying; pass
  // SyncToken() if no sync is required. Synthesizes a new sync token in
  // `completion_sync_token` which will satisfy after the image copy completes.
  // In practice, this API can be used to replace a resource with the contents
  // of an AcceleratedStaticBitmapImage or with a WebGPUMailboxTexture.
  bool OverwriteImage(const scoped_refptr<gpu::ClientSharedImage>& shared_image,
                      const gfx::Rect& copy_rect,
                      const gpu::SyncToken& ready_sync_token,
                      gpu::SyncToken& completion_sync_token);

  virtual bool HasUnusedResourcesForTesting() const { return false; }
  virtual bool unused_resources_reclaim_timer_is_running_for_testing() const {
    return false;
  }
  virtual int NumInflightResourcesForTesting() const { return 0; }

  constexpr static base::TimeDelta kUnusedResourceExpirationTime =
      base::Seconds(5);

 protected:
  class CanvasImageProvider;

  gpu::gles2::GLES2Interface* ContextGL() const;
  gpu::raster::RasterInterface* RasterInterface() const;
  GrDirectContext* GetGrContext() const;
  base::WeakPtr<WebGraphicsContext3DProviderWrapper> ContextProviderWrapper()
      const {
    return context_provider_wrapper_;
  }

  scoped_refptr<StaticBitmapImage> SnapshotInternal(ImageOrientation,
                                                    FlushReason);

  CanvasResourceProvider(const ResourceProviderType&,
                         gfx::Size size,
                         viz::SharedImageFormat format,
                         SkAlphaType alpha_type,
                         const gfx::ColorSpace& color_space,
                         base::WeakPtr<WebGraphicsContext3DProviderWrapper>
                             context_provider_wrapper,
                         CanvasResourceHost* resource_host);

  // Its important to use this method for generating PaintImage wrapped canvas
  // snapshots to get a cache hit from cc's ImageDecodeCache. This method
  // ensures that the PaintImage ID for the snapshot, used for keying
  // decodes/uploads in the cache is invalidated only when the canvas contents
  // change.
  cc::PaintImage MakeImageSnapshot(FlushReason);
  virtual void RasterRecord(cc::PaintRecord);
  void RasterRecordOOP(cc::PaintRecord last_recording,
                       bool needs_clear,
                       gpu::Mailbox mailbox);

  CanvasImageProvider* GetOrCreateCanvasImageProvider();
  void TearDownSkSurface();

  // Will only notify a will draw if its needed. This is initially done for the
  // CanvasResourceProviderSharedImage use case.
  virtual void WillDrawIfNeeded() {}

  ResourceProviderType type_;
  mutable sk_sp<SkSurface> surface_;  // mutable for lazy init
  SkSurface::ContentChangeMode mode_ = SkSurface::kRetain_ContentChangeMode;

  virtual void OnFlushForImage(cc::PaintImage::ContentId content_id);
  void OnMemoryDump(base::trace_event::ProcessMemoryDump*) override;

  CanvasResourceHost* resource_host() { return resource_host_; }

 private:
  friend class FlushForImageListener;

  virtual sk_sp<SkSurface> CreateSkSurface() const = 0;
  virtual bool UseOopRasterization() { return false; }
  bool UseHardwareDecodeCache() const {
    return IsAccelerated() && context_provider_wrapper_;
  }
  // Notifies before any drawing will be done on the resource used by this
  // provider.
  virtual void WillDraw() {}

  size_t ComputeSurfaceSize() const;
  size_t GetSize() const override;

  cc::ImageDecodeCache* ImageDecodeCacheRGBA8();
  cc::ImageDecodeCache* ImageDecodeCacheF16();
  void EnsureSkiaCanvas();

  void Clear();

  // Called after the recording was cleared from any draw ops it might have had.
  void RecordingCleared() override;

  // Disables lines drawing as paths if necessary. Drawing lines as paths is
  // only needed for ganesh.
  void DisableLineDrawingAsPathsIfNecessary();

  void ReleaseLockedImages();

  base::WeakPtr<WebGraphicsContext3DProviderWrapper> context_provider_wrapper_;
  // Note that `info_` should be const, but the relevant SkImageInfo
  // constructors do not exist.
  SkImageInfo info_;
  gfx::Size size_;
  viz::SharedImageFormat format_;
  SkAlphaType alpha_type_;
  gfx::ColorSpace color_space_;
  std::unique_ptr<CanvasImageProvider> canvas_image_provider_;
  std::unique_ptr<cc::SkiaPaintCanvas> skia_canvas_;
  raw_ptr<CanvasResourceHost> resource_host_ = nullptr;
  // Recording accumulating draw ops. This pointer is always valid and safe to
  // dereference.
  std::unique_ptr<MemoryManagedPaintRecorder> recorder_;

  const cc::PaintImage::Id snapshot_paint_image_id_;
  cc::PaintImage::ContentId snapshot_paint_image_content_id_ =
      cc::PaintImage::kInvalidContentId;
  uint32_t snapshot_sk_image_id_ = 0u;

  bool oopr_uses_dmsaa_ = false;
  bool always_enable_raster_timers_for_testing_ = false;

  // The maximum number of draw ops executed on the canvas, after which the
  // underlying GrContext is flushed.
  // Note: This parameter does not affect the flushing of recorded PaintOps.
  // See kMaxRecordedOpBytes above.
  static constexpr int kMaxDrawsBeforeContextFlush = 50;

  // Parameters for the auto-flushing heuristic.
  size_t max_recorded_op_bytes_;
  size_t max_pinned_image_bytes_;

  bool clear_frame_ = true;
  FlushReason last_flush_reason_ = FlushReason::kNone;
  FlushReason printing_fallback_reason_ = FlushReason::kNone;
  std::optional<cc::PaintRecord> last_recording_;

  base::WeakPtrFactory<CanvasResourceProvider> weak_ptr_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_CANVAS_RESOURCE_PROVIDER_H_
