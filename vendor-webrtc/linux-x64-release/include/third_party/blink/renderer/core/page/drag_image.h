/*
 * Copyright (C) 2007 Apple Inc.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_DRAG_IMAGE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_DRAG_IMAGE_H_

#include <memory>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/graphics/graphics_context_types.h"
#include "third_party/blink/renderer/platform/graphics/image_orientation.h"
#include "third_party/blink/renderer/platform/graphics/paint/display_item_client.h"
#include "third_party/blink/renderer/platform/graphics/paint/paint_image.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/skia/include/core/SkBitmap.h"
#include "ui/gfx/geometry/size.h"
#include "ui/gfx/geometry/size_f.h"

namespace blink {

class Image;
class KURL;

class CORE_EXPORT DragImage {
  USING_FAST_MALLOC(DragImage);

 public:
  static std::unique_ptr<DragImage> Create(
      Image*,
      RespectImageOrientationEnum = kRespectImageOrientation,
      InterpolationQuality = GetDefaultInterpolationQuality(),
      float opacity = 1,
      gfx::Vector2dF image_scale = gfx::Vector2dF(1, 1));

  static std::unique_ptr<DragImage> Create(const KURL&,
                                           const String& label,
                                           float device_scale_factor);

  DragImage(const DragImage&) = delete;
  DragImage& operator=(const DragImage&) = delete;
  ~DragImage();

  static gfx::Vector2dF ClampedImageScale(const gfx::Size&,
                                          const gfx::Size&,
                                          const gfx::Size& max_size);

  const SkBitmap& Bitmap() { return bitmap_; }
  gfx::Size Size() const {
    return gfx::Size(bitmap_.width(), bitmap_.height());
  }

  void Scale(float scale_x, float scale_y);

 private:
  DragImage(const SkBitmap&, InterpolationQuality);

  SkBitmap bitmap_;
  InterpolationQuality interpolation_quality_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_DRAG_IMAGE_H_
