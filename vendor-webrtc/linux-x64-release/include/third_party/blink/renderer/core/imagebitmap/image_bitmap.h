// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_IMAGEBITMAP_IMAGE_BITMAP_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_IMAGEBITMAP_IMAGE_BITMAP_H_

#include <memory>

#include "base/memory/scoped_refptr.h"
#include "base/task/sequenced_task_runner.h"
#include "third_party/blink/public/mojom/css/preferred_color_scheme.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_image_bitmap_options.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/html/canvas/canvas_image_source.h"
#include "third_party/blink/renderer/core/imagebitmap/image_bitmap_source.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/bindings/v8_external_memory_accounter.h"
#include "third_party/blink/renderer/platform/graphics/image.h"
#include "third_party/blink/renderer/platform/graphics/image_orientation.h"
#include "third_party/blink/renderer/platform/graphics/paint/paint_record.h"
#include "third_party/blink/renderer/platform/graphics/static_bitmap_image.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"
#include "third_party/skia/include/core/SkRefCnt.h"
#include "ui/gfx/geometry/rect.h"

namespace blink {
class HTMLCanvasElement;
class HTMLVideoElement;
class ImageData;
class ImageElementBase;
class ImageDecoder;
class OffscreenCanvas;

class CORE_EXPORT ImageBitmap final : public ScriptWrappable,
                                      public CanvasImageSource,
                                      public ImageBitmapSource {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // Expects the ImageElementBase to return/have an SVGImage.
  static ScriptPromise<ImageBitmap> CreateAsync(
      ImageElementBase*,
      std::optional<gfx::Rect>,
      ScriptState*,
      scoped_refptr<base::SequencedTaskRunner> task_runner,
      mojom::blink::PreferredColorScheme,
      ExceptionState&,
      const ImageBitmapOptions* = ImageBitmapOptions::Create());
  static sk_sp<SkImage> GetSkImageFromDecoder(std::unique_ptr<ImageDecoder>);

  ImageBitmap(ImageElementBase*,
              std::optional<gfx::Rect>,
              const ImageBitmapOptions* = ImageBitmapOptions::Create());
  ImageBitmap(HTMLVideoElement*,
              std::optional<gfx::Rect>,
              const ImageBitmapOptions* = ImageBitmapOptions::Create());
  ImageBitmap(HTMLCanvasElement*,
              std::optional<gfx::Rect>,
              const ImageBitmapOptions* = ImageBitmapOptions::Create());
  ImageBitmap(OffscreenCanvas*,
              std::optional<gfx::Rect>,
              const ImageBitmapOptions* = ImageBitmapOptions::Create());
  ImageBitmap(ImageData*,
              std::optional<gfx::Rect>,
              const ImageBitmapOptions* = ImageBitmapOptions::Create());
  ImageBitmap(ImageBitmap*,
              std::optional<gfx::Rect>,
              const ImageBitmapOptions* = ImageBitmapOptions::Create());
  ImageBitmap(scoped_refptr<StaticBitmapImage>);
  ImageBitmap(scoped_refptr<StaticBitmapImage>,
              std::optional<gfx::Rect>,
              const ImageBitmapOptions* = ImageBitmapOptions::Create());
  // This constructor may called by structured-cloning an ImageBitmap.
  // isImageBitmapOriginClean indicates whether the original ImageBitmap is
  // origin clean or not.
  ImageBitmap(const SkPixmap& pixmap,
              bool is_image_bitmap_origin_clean,
              ImageOrientationEnum);

  scoped_refptr<StaticBitmapImage> BitmapImage() const { return image_; }

  // Retrieve the SkImageInfo that best represents BitmapImage().
  SkImageInfo GetBitmapSkImageInfo() const;

  // When apply_orientation is true this method will orient the data according
  // to the source's EXIF information.
  Vector<uint8_t> CopyBitmapData(const SkImageInfo& info,
                                 bool apply_orientation);
  unsigned width() const;
  unsigned height() const;
  gfx::Size Size() const;

  bool IsNeutered() const override { return is_neutered_; }
  bool OriginClean() const { return image_->OriginClean(); }
  bool IsPremultiplied() const { return image_->IsPremultiplied(); }
  ImageOrientationEnum ImageOrientation() const {
    return image_->CurrentFrameOrientation().Orientation();
  }
  scoped_refptr<StaticBitmapImage> Transfer();
  void close();

  ~ImageBitmap() override;

  // CanvasImageSource implementation
  scoped_refptr<Image> GetSourceImageForCanvas(FlushReason,
                                               SourceImageStatus*,
                                               const gfx::SizeF&) override;
  bool WouldTaintOrigin() const override {
    return image_ && !image_->OriginClean();
  }
  gfx::SizeF ElementSize(const gfx::SizeF&,
                         const RespectImageOrientationEnum) const override;
  bool IsImageBitmap() const override { return true; }
  bool IsAccelerated() const override;

  // ImageBitmapSource implementation
  ImageBitmapSourceStatus CheckUsability() const override;
  ScriptPromise<ImageBitmap> CreateImageBitmap(ScriptState*,
                                               std::optional<gfx::Rect>,
                                               const ImageBitmapOptions*,
                                               ExceptionState&) override;

  struct ParsedOptions {
    // If true, then the final result should be flipped vertically. This happens
    // in the space after `source_orientation` has been applied.
    bool flip_y = false;
    bool premultiply_alpha = true;
    // TODO(crbug.com/40773069): This is based on the incorrect values and needs
    // to be removed.
    bool should_scale_input = false;
    bool has_color_space_conversion = false;
    bool source_is_unpremul = false;
    bool orientation_from_image = true;
    // TODO(crbug.com/40773069): The value of `resize_width`, `resize_height`,
    // and `crop_rect` are computed incorrectly. Remove this when all code that
    // uses it is removed.
    unsigned resize_width = 0;
    unsigned resize_height = 0;
    gfx::Rect crop_rect;
    cc::PaintFlags::FilterQuality resize_quality =
        cc::PaintFlags::FilterQuality::kLow;

    // The sampling options to use. This will be set to nearest-neighbor if no
    // resampling is performed.
    SkSamplingOptions sampling;

    // The orientation of the source.
    class ImageOrientation source_orientation;

    // The `source_size`, `source_rect`, and `dest_size` parameters are all in
    // the space after the `source_orientation` has been applied.
    gfx::Size source_size;
    gfx::Rect source_rect;
    gfx::Size dest_size;

    // Compute the parameters for creating and then resizing a subset of the
    // source image. In the underlying PaintImage, `source_skrect` corresponds
    // to `source_rect`, `source_skrect_valid` corresponds to the intersection
    // of that with the PaintImage size, and `dest_sksize` corresponds to the
    // output size.
    void ComputeSubsetParameters(SkIRect& source_skrect,
                                 SkIRect& source_skrect_valid,
                                 SkISize& dest_sksize) const;
  };

 private:
  void UpdateImageBitmapMemoryUsage();
  static void ResolvePromiseOnOriginalThread(
      ScriptPromiseResolver<ImageBitmap>*,
      bool origin_clean,
      std::unique_ptr<ParsedOptions>,
      sk_sp<SkImage>,
      const ImageOrientationEnum);
  static void RasterizeImageOnBackgroundThread(
      PaintRecord,
      const gfx::Rect&,
      scoped_refptr<base::SequencedTaskRunner>,
      WTF::CrossThreadOnceFunction<void(sk_sp<SkImage>,
                                        const ImageOrientationEnum)> callback);
  scoped_refptr<StaticBitmapImage> image_;
  bool is_neutered_ = false;
  int32_t memory_usage_ = 0;
  NO_UNIQUE_ADDRESS V8ExternalMemoryAccounterBase external_memory_accounter_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_IMAGEBITMAP_IMAGE_BITMAP_H_
