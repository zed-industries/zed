// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_TEXTURE_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_TEXTURE_UTILS_H_

#include "third_party/blink/renderer/modules/webgpu/dawn_object.h"

namespace blink {

size_t EstimateWriteTextureBytesUpperBound(wgpu::TexelCopyBufferLayout layout,
                                           wgpu::Extent3D extent,
                                           wgpu::TextureFormat format,
                                           wgpu::TextureAspect aspect);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_TEXTURE_UTILS_H_
