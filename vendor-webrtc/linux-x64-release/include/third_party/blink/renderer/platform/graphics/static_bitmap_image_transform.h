// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_STATIC_BITMAP_IMAGE_TRANSFORM_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_STATIC_BITMAP_IMAGE_TRANSFORM_H_

#include "third_party/blink/renderer/platform/graphics/flush_reason.h"
#include "third_party/blink/renderer/platform/graphics/static_bitmap_image.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

// A helper class for transformations of a StaticBitmapImage that can
// potentially be accomplished by a blit. This includes the transforms caused
// by ImageBitmapOptions and cloning, but will be expanded to include
// changing the alpha type, color space, and pixel format.
class PLATFORM_EXPORT StaticBitmapImageTransform {
 public:
  // The parameters to the most generic Apply function that all other functions
  // are built upon.
  struct Params {
    // If true, then the source image must be copied (even if the transform
    // is a no-op, or can be accomplished without allocating a new backing).
    bool force_copy = false;

    // If true, then the final result should be flipped vertically. This happens
    // in the space after `source_orientation` has been applied.
    bool flip_y = false;

    // If true, then the final result must be premultiplied (or opaque).
    bool premultiply_alpha = true;

    // If true, then strip the color space from the input (and therefore
    // reinterpret the image as being sRGB).
    bool reinterpret_as_srgb = false;

    // If this is set to a non-nullptr value, then convert the source to this
    // color space. It's not clear what it means to set `dest_color_space` and
    // also `reinterpret_as_srgb`, so any call with both parameters will CHECK.
    sk_sp<SkColorSpace> dest_color_space;

    // If false, then strip the orientation from teh imgae (and therefore
    // reinterpret the image as having the origin be the top-left).
    bool orientation_from_image = true;

    // The sampling options to use. This will be set to nearest-neighbor if no
    // resampling is performed.
    SkSamplingOptions sampling;

    // The `source_size`, `source_rect`, and `dest_size` parameters are all in
    // the space after the `source_orientation` has been applied.
    gfx::Rect source_rect;
    gfx::Size dest_size;
  };

  // Apply the specified transform to the indcated image.
  static scoped_refptr<StaticBitmapImage> Apply(
      FlushReason,
      scoped_refptr<StaticBitmapImage> image,
      const Params& params);

  // Create a copy of the input image, with a newly created backing.
  static scoped_refptr<StaticBitmapImage> Clone(
      FlushReason,
      scoped_refptr<StaticBitmapImage> image);

  // Convert `image` to the specified color space.
  static scoped_refptr<StaticBitmapImage> ConvertToColorSpace(
      FlushReason,
      scoped_refptr<StaticBitmapImage> image,
      sk_sp<SkColorSpace> color_space);

 private:
  // Apply the specified transform by manipulating SkPixmaps in software. This
  // (SkPixmap::scalePixels) is the only path that allows resampling of images
  // while preserving unpremultiplied alpha values.
  static scoped_refptr<StaticBitmapImage> ApplyUsingPixmap(
      scoped_refptr<StaticBitmapImage> image,
      const Params& params);

  // Apply the specified transform by using a blit. The blit may be done on the
  // GPU or may be done in software. The result is always premultiplied.
  static scoped_refptr<StaticBitmapImage> ApplyWithBlit(
      FlushReason,
      scoped_refptr<StaticBitmapImage> image,
      const Params& params);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_STATIC_BITMAP_IMAGE_TRANSFORM_H_
