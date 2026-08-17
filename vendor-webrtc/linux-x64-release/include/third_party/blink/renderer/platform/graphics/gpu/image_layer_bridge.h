// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GPU_IMAGE_LAYER_BRIDGE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GPU_IMAGE_LAYER_BRIDGE_H_

#include "cc/layers/texture_layer_client.h"
#include "components/viz/common/resources/shared_image_format.h"
#include "third_party/blink/renderer/platform/graphics/opacity_mode.h"
#include "third_party/blink/renderer/platform/graphics/static_bitmap_image.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "ui/gfx/geometry/point_f.h"

namespace cc {
class Layer;
class TextureLayer;
}

namespace gfx {
class Size;
}

namespace blink {
class WebGraphicsSharedImageInterfaceProvider;

class PLATFORM_EXPORT ImageLayerBridge
    : public GarbageCollected<ImageLayerBridge>,
      public cc::TextureLayerClient {
 public:
  ImageLayerBridge(OpacityMode);
  ImageLayerBridge(const ImageLayerBridge&) = delete;
  ImageLayerBridge& operator=(const ImageLayerBridge&) = delete;
  ~ImageLayerBridge() override;

  void SetImage(scoped_refptr<StaticBitmapImage>);
  void Dispose();

  // cc::TextureLayerClient implementation.
  bool PrepareTransferableResource(
      viz::TransferableResource* out_resource,
      viz::ReleaseCallback* out_release_callback) override;

  scoped_refptr<StaticBitmapImage> GetImage() { return image_; }

  cc::Layer* CcLayer() const;

  void SetUV(const gfx::PointF& left_top, const gfx::PointF& right_bottom);

  bool IsAccelerated() { return image_ && image_->IsTextureBacked(); }

  void Trace(Visitor* visitor) const {}

 private:
  // Resource holding a software SharedImage. Used only with software
  // compositing.
  struct SoftwareResource {
    SoftwareResource();
    SoftwareResource(SoftwareResource&& other);
    SoftwareResource& operator=(SoftwareResource&& other);

    scoped_refptr<gpu::ClientSharedImage> shared_image;
    gpu::SyncToken sync_token;
    base::WeakPtr<blink::WebGraphicsSharedImageInterfaceProvider> sii_provider;
  };

  // Returns a software resource of |size|. Tries to recycle returned resources
  // first and allocates a new resource if necessary. Note this will delete
  // recycled resources that are the wrong size.
  SoftwareResource CreateOrRecycleSoftwareResource(
      const gfx::Size& size,
      viz::SharedImageFormat format);

  void ResourceReleasedGpu(scoped_refptr<StaticBitmapImage>,
                           const gpu::SyncToken&,
                           bool lost_resource);

  void ResourceReleasedSoftware(SoftwareResource resource,
                                const gpu::SyncToken&,
                                bool lost_resource);

  scoped_refptr<StaticBitmapImage> image_;
  scoped_refptr<cc::TextureLayer> layer_;

  // SharedMemory resources that can be recycled.
  Vector<SoftwareResource> recycled_software_resources_;

  bool disposed_ = false;
  bool has_presented_since_last_set_image_ = false;
  bool is_opaque_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GPU_IMAGE_LAYER_BRIDGE_H_
