// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_UNACCELERATED_STATIC_BITMAP_IMAGE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_UNACCELERATED_STATIC_BITMAP_IMAGE_H_

#include "base/memory/weak_ptr.h"
#include "base/task/single_thread_task_runner.h"
#include "base/threading/thread_checker.h"
#include "third_party/blink/renderer/platform/graphics/skia/skia_utils.h"
#include "third_party/blink/renderer/platform/graphics/static_bitmap_image.h"
#include "third_party/blink/renderer/platform/scheduler/public/thread.h"

namespace blink {

class PLATFORM_EXPORT UnacceleratedStaticBitmapImage final
    : public StaticBitmapImage {
 public:
  ~UnacceleratedStaticBitmapImage() override;

  // The ImageOrientation should be derived from the source of the image data.
  static scoped_refptr<UnacceleratedStaticBitmapImage> Create(
      sk_sp<SkImage>,
      ImageOrientation orientation = ImageOrientationEnum::kDefault);
  static scoped_refptr<UnacceleratedStaticBitmapImage> Create(
      PaintImage,
      ImageOrientation orientation = ImageOrientationEnum::kDefault);

  bool CurrentFrameKnownToBeOpaque() override;

  void Draw(cc::PaintCanvas*,
            const cc::PaintFlags&,
            const gfx::RectF& dst_rect,
            const gfx::RectF& src_rect,
            const ImageDrawOptions&) override;

  PaintImage PaintImageForCurrentFrame() override;

  void Transfer() final;

  bool CopyToResourceProvider(CanvasResourceProvider* resource_provider,
                              const gfx::Rect& copy_rect) override;

  SkImageInfo GetSkImageInfo() const;
  gfx::Size GetSize() const override {
    return gfx::Size(GetSkImageInfo().width(), GetSkImageInfo().height());
  }
  SkAlphaType GetAlphaType() const override {
    return GetSkImageInfo().alphaType();
  }
  SkColorType GetSkColorType() const { return GetSkImageInfo().colorType(); }
  sk_sp<SkColorSpace> GetSkColorSpace() const override {
    return GetSkImageInfo().refColorSpace();
  }
  gfx::ColorSpace GetColorSpace() const override {
    return SkColorSpaceToGfxColorSpace(GetSkColorSpace());
  }
  viz::SharedImageFormat GetSharedImageFormat() const override {
    return viz::SkColorTypeToSinglePlaneSharedImageFormat(GetSkColorType());
  }

 private:
  UnacceleratedStaticBitmapImage(sk_sp<SkImage>, ImageOrientation);
  UnacceleratedStaticBitmapImage(PaintImage, ImageOrientation);

  PaintImage paint_image_;
  THREAD_CHECKER(thread_checker_);

  sk_sp<SkImage> original_skia_image_;
  scoped_refptr<base::SingleThreadTaskRunner> original_skia_image_task_runner_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_UNACCELERATED_STATIC_BITMAP_IMAGE_H_
