// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_LAYER_SHARED_IMAGE_MANAGER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_LAYER_SHARED_IMAGE_MANAGER_H_

#include <optional>

#include "gpu/command_buffer/client/client_shared_image.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"

namespace blink {

class XRLayer;

struct XRSharedImageData {
  scoped_refptr<gpu::ClientSharedImage> shared_image;
  gpu::SyncToken sync_token;
};

struct XRLayerSharedImages {
  XRSharedImageData content_image_data;
  XRSharedImageData camera_image_data;
};

class XRLayerSharedImageManager {
 public:
  XRLayerSharedImageManager() = default;
  ~XRLayerSharedImageManager() = default;

  void Reset();
  void SetLayerSharedImages(
      XRLayer*,
      const scoped_refptr<gpu::ClientSharedImage>& color_shared_image,
      const gpu::SyncToken& color_sync_token,
      const scoped_refptr<gpu::ClientSharedImage>& camera_image_shared_image,
      const gpu::SyncToken& camera_image_sync_token);

  const XRLayerSharedImages& GetLayerSharedImages(const XRLayer*) const;

 private:
  XRLayerSharedImages empty_shared_images_;
  WTF::HashMap<uint32_t, XRLayerSharedImages> layer_shared_images_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_LAYER_SHARED_IMAGE_MANAGER_H_
