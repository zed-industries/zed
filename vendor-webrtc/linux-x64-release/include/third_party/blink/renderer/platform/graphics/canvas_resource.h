// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#include <string>

#include "base/check_op.h"
#include "base/dcheck_is_on.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/shared_memory_mapping.h"
#include "base/memory/weak_ptr.h"
#include "base/notreached.h"
#include "base/task/single_thread_task_runner.h"
#include "components/viz/common/resources/release_callback.h"
#include "components/viz/common/resources/shared_image_format.h"
#include "components/viz/common/resources/transferable_resource.h"
#include "gpu/command_buffer/client/client_shared_image.h"
#include "gpu/command_buffer/client/gles2_interface.h"
#include "gpu/command_buffer/client/shared_image_interface.h"
#include "gpu/command_buffer/common/mailbox.h"
#include "gpu/command_buffer/common/shared_image_usage.h"
#include "gpu/command_buffer/common/sync_token.h"
#include "skia/buildflags.h"
#include "third_party/blink/public/platform/web_graphics_shared_image_interface_provider.h"
#include "third_party/blink/renderer/platform/graphics/web_graphics_context_3d_provider_wrapper.h"
#include "third_party/blink/renderer/platform/scheduler/public/thread.h"
#include "third_party/blink/renderer/platform/wtf/thread_safe_ref_counted.h"
#include "third_party/skia/include/core/SkImageInfo.h"
#include "third_party/skia/include/core/SkRefCnt.h"
#include "ui/gfx/buffer_types.h"
#include "ui/gfx/geometry/size.h"

class GrBackendTexture;
class SkSurface;

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_CANVAS_RESOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_CANVAS_RESOURCE_H_

namespace gfx {

class ColorSpace;

}  // namespace gfx

namespace gpu::raster {

class RasterInterface;

}  // namespace gpu::raster

namespace blink {

class CanvasResourceProvider;
class StaticBitmapImage;

// Generic resource interface, used for locking (RAII) and recycling pixel
// buffers of any type.
// Note that this object may be accessed across multiple threads but not
// concurrently. The caller is responsible to call Transfer on the object before
// using it on a different thread.
class PLATFORM_EXPORT CanvasResource
    : public WTF::ThreadSafeRefCounted<CanvasResource> {
 public:
  using ReleaseCallback = base::OnceCallback<void(
      scoped_refptr<blink::CanvasResource>&& canvas_resource,
      const gpu::SyncToken& sync_token,
      bool is_lost)>;

  using LastUnrefCallback = base::OnceCallback<void(
      scoped_refptr<blink::CanvasResource> canvas_resource)>;

  virtual ~CanvasResource();

  // Non-virtual override of ThreadSafeRefCounted::Release
  void Release();

  // Set a callback that will be invoked as the last outstanding reference to
  // this CanvasResource goes out of scope.  This provides a last chance hook
  // to intercept a canvas before it get destroyed. For resources that need to
  // be destroyed on their thread of origin, this hook can be used to return
  // resources to their creators.
  void SetLastUnrefCallback(LastUnrefCallback callback) {
    last_unref_callback_ = std::move(callback);
  }

  bool HasLastUnrefCallback() { return !!last_unref_callback_; }

  // Returns true if this instance creates TransferableResources for usage with
  // GPU compositing.
  virtual bool CreatesAcceleratedTransferableResources() const = 0;

  // Transfers ownership of the resource's vix::ReleaseCallback.  This is useful
  // prior to transferring a resource to another thread, to retain the release
  // callback on the current thread since the callback may not be thread safe.
  // Even if the callback is never executed on another thread, simply transiting
  // through another thread is dangerous because garbage collection races may
  // make it impossible to return the resource to its thread of origin for
  // destruction; in which case the callback (and its bound arguments) may be
  // destroyed on the wrong thread.
  virtual viz::ReleaseCallback TakeVizReleaseCallback() {
    return viz::ReleaseCallback();
  }

  virtual void OnReturnedFromCompositor(
      scoped_refptr<CanvasResource>&& resource) {}

  virtual void SetVizReleaseCallback(viz::ReleaseCallback cb) {
    CHECK(cb.is_null());
  }

  // Returns true if the resource is still usable. It maybe not be valid in the
  // case of a context loss or if we fail to initialize the memory backing for
  // the resource.
  virtual bool IsValid() const = 0;

  // The bounds for this resource.
  gfx::Size Size() const { return size_; }

  viz::SharedImageFormat GetFormat() const { return format_; }

  const gfx::ColorSpace& GetColorSpace() const { return color_space_; }

  SkAlphaType GetAlphaType() const { return alpha_type_; }

  // The ClientSharedImage containing information on the SharedImage
  // attached to the resource.
  virtual scoped_refptr<gpu::ClientSharedImage> GetClientSharedImage() = 0;

  // A CanvasResource is not thread-safe and does not allow concurrent usage
  // from multiple threads. But it maybe used from any thread. It remains bound
  // to the current thread until Transfer is called. Note that while the
  // resource maybe used for reads on any thread, it can be written to only on
  // the thread where it was created.
  virtual void Transfer() {}

  // Returns the sync token to indicate when all writes to the current resource
  // are finished on the GPU thread. Note that the token is not guaranteed to be
  // verified at the time of calling this method.
  const gpu::SyncToken GetSyncToken() {
    return GetSyncTokenWithOptionalVerification(false);
  }

  // Provides a TransferableResource representation of this resource to share it
  // with the compositor.
  bool PrepareTransferableResource(viz::TransferableResource*,
                                   ReleaseCallback*,
                                   bool needs_verified_synctoken);

  // Issues a wait for this sync token on the context used by this resource for
  // rendering.
  void WaitSyncToken(const gpu::SyncToken&);

  bool OriginClean() const { return is_origin_clean_; }
  void SetOriginClean(bool flag) { is_origin_clean_ = flag; }

  // Provides a StaticBitmapImage wrapping this resource. Commonly used for
  // snapshots not used in compositing (for instance to draw to another canvas).
  virtual scoped_refptr<StaticBitmapImage> Bitmap() = 0;

  // Called when the resource is marked lost. Losing a resource does not mean
  // that the backing memory has been destroyed, since the resource itself keeps
  // a ref on that memory.
  // It means that the consumer (commonly the compositor) can not provide a sync
  // token for the resource to be safely recycled and its the GL state may be
  // inconsistent with when the resource was given to the compositor. So it
  // should not be recycled for writing again but can be safely read from.
  virtual void NotifyResourceLost() = 0;

  SkImageInfo CreateSkImageInfo() const;

  bool is_cross_thread() const {
    return base::PlatformThread::CurrentRef() != owning_thread_ref_;
  }

 protected:
  CanvasResource(base::WeakPtr<CanvasResourceProvider>,
                 gfx::Size size,
                 viz::SharedImageFormat format,
                 SkAlphaType alpha_type,
                 const gfx::ColorSpace& color_space);

  virtual gfx::HDRMetadata GetHDRMetadata() const { return gfx::HDRMetadata(); }
  virtual viz::TransferableResource::ResourceSource
  GetTransferableResourceSource() const {
    return viz::TransferableResource::ResourceSource::kCanvas;
  }

  gpu::InterfaceBase* InterfaceBase() const;
  gpu::gles2::GLES2Interface* ContextGL() const;
  gpu::raster::RasterInterface* RasterInterface() const;
  gpu::webgpu::WebGPUInterface* WebGPUInterface() const;
  virtual base::WeakPtr<WebGraphicsContext3DProviderWrapper>
  ContextProviderWrapper() const = 0;

  CanvasResourceProvider* Provider() { return provider_.get(); }
  base::WeakPtr<CanvasResourceProvider> WeakProvider() { return provider_; }

  const base::PlatformThreadRef owning_thread_ref_;
  const scoped_refptr<base::SingleThreadTaskRunner> owning_thread_task_runner_;

 private:
  // Returns true if the resource is rastered via the GPU.
  virtual bool UsesAcceleratedRaster() const = 0;

  // Returns the sync token to indicate when all writes to the current resource
  // are finished on the GPU thread. Note that in some subclasses the token is
  // not guaranteed to be verified at the time of calling this method. Passing
  // true for `needs_verified_token` ensures that the returned token will be
  // verified.
  virtual const gpu::SyncToken GetSyncTokenWithOptionalVerification(
      bool needs_verified_token) {
    NOTREACHED();
  }

  base::WeakPtr<CanvasResourceProvider> provider_;
  gfx::Size size_;
  viz::SharedImageFormat format_;
  SkAlphaType alpha_type_;
  gfx::ColorSpace color_space_;
  LastUnrefCallback last_unref_callback_;
  bool is_origin_clean_ = true;
};

// Resource type for SharedImage
class PLATFORM_EXPORT CanvasResourceSharedImage final : public CanvasResource {
 public:
  static scoped_refptr<CanvasResourceSharedImage> CreateSoftware(
      gfx::Size size,
      viz::SharedImageFormat format,
      SkAlphaType alpha_type,
      const gfx::ColorSpace& color_space,
      base::WeakPtr<CanvasResourceProvider>,
      base::WeakPtr<WebGraphicsSharedImageInterfaceProvider>);

  static scoped_refptr<CanvasResourceSharedImage> Create(
      gfx::Size size,
      viz::SharedImageFormat format,
      SkAlphaType alpha_type,
      const gfx::ColorSpace& color_space,
      base::WeakPtr<WebGraphicsContext3DProviderWrapper>,
      base::WeakPtr<CanvasResourceProvider>,
      bool is_accelerated,
      gpu::SharedImageUsageSet shared_image_usage_flags);
  ~CanvasResourceSharedImage() override;

  bool CreatesAcceleratedTransferableResources() const override {
    return !GetClientSharedImage()->is_software();
  }
  void OnReturnedFromCompositor(scoped_refptr<CanvasResource>&& resource) final;
  bool IsValid() const final;
  scoped_refptr<StaticBitmapImage> Bitmap() final;
  void Transfer() final;

  void NotifyResourceLost() final;
  void BeginWriteAccess();
  void EndWriteAccess();
  GrBackendTexture CreateGrTexture() const;

  GLuint GetTextureIdForReadAccess() const {
    return owning_thread_data().texture_id_for_read_access;
  }
  GLuint GetTextureIdForWriteAccess() const {
    return owning_thread_data().texture_id_for_write_access;
  }

  void WillDraw();
  bool IsLost() const { return owning_thread_data().is_lost; }

  scoped_refptr<gpu::ClientSharedImage> GetClientSharedImage() override;
  const scoped_refptr<gpu::ClientSharedImage>& GetClientSharedImage() const;
  void OnMemoryDump(base::trace_event::ProcessMemoryDump* pmd,
                    const std::string& parent_path) const;

  // Signals that an external write has completed, passing the token that should
  // be waited on to ensure that the service-side operations of the external
  // write have completed. Ensures that the next read of this resource (whether
  // via raster or the compositor) waits on this token.
  void EndExternalWrite(const gpu::SyncToken& external_write_sync_token);

  // Uploads the contents of |sk_surface| to the resource's backing memory.
  // Should be called only if the resource is using software raster.
  void UploadSoftwareRenderingResults(SkSurface* sk_surface);

 private:
  // These members are either only accessed on the owning thread, or are only
  // updated on the owning thread and then are read on a different thread.
  // We ensure to correctly update their state in Transfer, which is called
  // before a resource is used on a different thread.
  struct OwningThreadData {
    bool mailbox_needs_new_sync_token = true;
    scoped_refptr<gpu::ClientSharedImage> client_shared_image;
    gpu::SyncToken sync_token;
    size_t bitmap_image_read_refs = 0u;
    bool is_lost = false;

    // We need to create 2 representations if canvas is operating in single
    // buffered mode to allow concurrent scopes for read and write access,
    // because the Begin/EndSharedImageAccessDirectCHROMIUM APIs allow only one
    // active access mode for a representation.
    // In non single buffered mode, the 2 texture ids are the same.
    GLuint texture_id_for_read_access = 0u;
    GLuint texture_id_for_write_access = 0u;
  };

  static void OnBitmapImageDestroyed(
      scoped_refptr<CanvasResourceSharedImage> resource,
      bool has_read_ref_on_texture,
      const gpu::SyncToken& sync_token,
      bool is_lost);

  base::WeakPtr<WebGraphicsContext3DProviderWrapper> ContextProviderWrapper()
      const override;
  const gpu::SyncToken GetSyncTokenWithOptionalVerification(
      bool needs_verified_token) override;
  bool UsesAcceleratedRaster() const final { return is_accelerated_; }

  CanvasResourceSharedImage(
      gfx::Size size,
      viz::SharedImageFormat format,
      SkAlphaType alpha_type,
      const gfx::ColorSpace& color_space,
      base::WeakPtr<CanvasResourceProvider>,
      base::WeakPtr<WebGraphicsSharedImageInterfaceProvider>);

  CanvasResourceSharedImage(gfx::Size size,
                            viz::SharedImageFormat format,
                            SkAlphaType alpha_type,
                            const gfx::ColorSpace& color_space,
                            base::WeakPtr<WebGraphicsContext3DProviderWrapper>,
                            base::WeakPtr<CanvasResourceProvider>,
                            bool is_accelerated,
                            gpu::SharedImageUsageSet shared_image_usage_flags);

  OwningThreadData& owning_thread_data() {
    DCHECK(!is_cross_thread());
    return owning_thread_data_;
  }
  const OwningThreadData& owning_thread_data() const {
    DCHECK(!is_cross_thread());
    return owning_thread_data_;
  }

  // Can be read on any thread.

  bool mailbox_needs_new_sync_token() const {
    return owning_thread_data_.mailbox_needs_new_sync_token;
  }
  const gpu::SyncToken& sync_token() const {
    return owning_thread_data_.sync_token;
  }

  // This should only be de-referenced on the owning thread but may be copied
  // on a different thread.
  base::WeakPtr<WebGraphicsContext3DProviderWrapper> context_provider_wrapper_;

  // Accessed on any thread.
  const bool is_accelerated_;
  const bool use_oop_rasterization_;
  OwningThreadData owning_thread_data_;
};

// Resource type for a given opaque external resource described on construction
// via a TransferableResource. This CanvasResource should only be used with
// context that support GL.
class PLATFORM_EXPORT ExternalCanvasResource final : public CanvasResource {
 public:
  static scoped_refptr<ExternalCanvasResource> Create(
      scoped_refptr<gpu::ClientSharedImage> client_si,
      const gpu::SyncToken& sync_token,
      viz::TransferableResource::ResourceSource resource_source,
      gfx::HDRMetadata hdr_metadata,
      viz::ReleaseCallback release_callback,
      base::WeakPtr<WebGraphicsContext3DProviderWrapper>,
      base::WeakPtr<CanvasResourceProvider>);

  ~ExternalCanvasResource() override;
  bool IsValid() const override;
  bool CreatesAcceleratedTransferableResources() const override { return true; }
  void NotifyResourceLost() override { resource_is_lost_ = true; }
  scoped_refptr<gpu::ClientSharedImage> GetClientSharedImage() final {
    return client_si_;
  }

  scoped_refptr<StaticBitmapImage> Bitmap() override;
  viz::ReleaseCallback TakeVizReleaseCallback() override {
    return std::move(release_callback_);
  }
  void SetVizReleaseCallback(viz::ReleaseCallback cb) override {
    release_callback_ = std::move(cb);
  }

 private:
  gfx::HDRMetadata GetHDRMetadata() const final { return hdr_metadata_; }
  viz::TransferableResource::ResourceSource GetTransferableResourceSource()
      const final {
    return resource_source_;
  }
  bool UsesAcceleratedRaster() const final { return true; }
  const gpu::SyncToken GetSyncTokenWithOptionalVerification(
      bool needs_verified_token) override;
  base::WeakPtr<WebGraphicsContext3DProviderWrapper> ContextProviderWrapper()
      const override;

  ExternalCanvasResource(
      scoped_refptr<gpu::ClientSharedImage> client_si,
      const gpu::SyncToken& sync_token,
      viz::TransferableResource::ResourceSource resource_source,
      gfx::HDRMetadata hdr_metadata,
      viz::ReleaseCallback out_callback,
      base::WeakPtr<WebGraphicsContext3DProviderWrapper>,
      base::WeakPtr<CanvasResourceProvider>);

  scoped_refptr<gpu::ClientSharedImage> client_si_;
  const base::WeakPtr<WebGraphicsContext3DProviderWrapper>
      context_provider_wrapper_;
  gpu::SyncToken sync_token_;
  viz::TransferableResource::ResourceSource resource_source_;
  gfx::HDRMetadata hdr_metadata_;
  viz::ReleaseCallback release_callback_;
  bool resource_is_lost_ = false;
};

class PLATFORM_EXPORT CanvasResourceSwapChain final : public CanvasResource {
 public:
  // The passed-in WeakPtrs must be non-null.
  static scoped_refptr<CanvasResourceSwapChain> Create(
      gfx::Size size,
      viz::SharedImageFormat format,
      SkAlphaType alpha_type,
      const gfx::ColorSpace& color_space,
      base::WeakPtr<WebGraphicsContext3DProviderWrapper>,
      base::WeakPtr<CanvasResourceProvider>);
  ~CanvasResourceSwapChain() override;
  bool IsValid() const override;
  bool CreatesAcceleratedTransferableResources() const override { return true; }
  void NotifyResourceLost() override {
    // Used for single buffering mode which doesn't need to care about sync
    // token synchronization.
  }

  scoped_refptr<StaticBitmapImage> Bitmap() override;

  GLuint GetBackBufferTextureId() const { return back_buffer_texture_id_; }
  scoped_refptr<gpu::ClientSharedImage> GetBackBufferClientSharedImage() {
    CHECK(back_buffer_shared_image_);
    return back_buffer_shared_image_;
  }
  void PresentSwapChain();
  scoped_refptr<gpu::ClientSharedImage> GetClientSharedImage() override;

 private:
  bool UsesAcceleratedRaster() const final { return true; }
  const gpu::SyncToken GetSyncTokenWithOptionalVerification(
      bool needs_verified_token) override;
  base::WeakPtr<WebGraphicsContext3DProviderWrapper> ContextProviderWrapper()
      const override;

  CanvasResourceSwapChain(gfx::Size size,
                          viz::SharedImageFormat format,
                          SkAlphaType alpha_type,
                          const gfx::ColorSpace& color_space,
                          base::WeakPtr<WebGraphicsContext3DProviderWrapper>,
                          base::WeakPtr<CanvasResourceProvider>);

  const base::WeakPtr<WebGraphicsContext3DProviderWrapper>
      context_provider_wrapper_;
  scoped_refptr<gpu::ClientSharedImage> front_buffer_shared_image_;
  scoped_refptr<gpu::ClientSharedImage> back_buffer_shared_image_;
  GLuint back_buffer_texture_id_ = 0u;
  gpu::SyncToken sync_token_;
  const bool use_oop_rasterization_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_CANVAS_RESOURCE_H_
