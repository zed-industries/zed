// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CANVAS_IMAGE_ELEMENT_BASE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CANVAS_IMAGE_ELEMENT_BASE_H_

#include "third_party/blink/public/mojom/css/preferred_color_scheme.mojom-blink.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/html/canvas/canvas_image_source.h"
#include "third_party/blink/renderer/core/imagebitmap/image_bitmap_source.h"
#include "third_party/blink/renderer/platform/graphics/image.h"

namespace blink {

class Element;
class ImageLoader;
class ImageResourceContent;

class CORE_EXPORT ImageElementBase : public CanvasImageSource,
                                     public ImageBitmapSource {
 public:
  virtual ImageLoader& GetImageLoader() const = 0;

  // Parses the given async parameter value into an ImageDecodingMode. This is
  // used by SVGImageElement and HTMLImageElement since this class is a common
  // base for both elements.
  static Image::ImageDecodingMode ParseImageDecodingMode(const AtomicString&);

  ImageBitmapSourceStatus CheckUsability() const override;
  ScriptPromise<ImageBitmap> CreateImageBitmap(ScriptState*,
                                               std::optional<gfx::Rect>,
                                               const ImageBitmapOptions*,
                                               ExceptionState&) override;

  scoped_refptr<Image> GetSourceImageForCanvas(FlushReason,
                                               SourceImageStatus*,
                                               const gfx::SizeF&) override;

  bool WouldTaintOrigin() const override;

  gfx::SizeF ElementSize(const gfx::SizeF& default_object_size,
                         const RespectImageOrientationEnum) const override;
  gfx::SizeF DefaultDestinationSize(
      const gfx::SizeF& default_object_size,
      const RespectImageOrientationEnum) const override;

  bool IsAccelerated() const override;

  bool IsSVGSource() const override;

  bool IsImageElement() const override;

  bool IsOpaque() const override;

  const KURL& SourceURL() const override;

  ImageResourceContent* CachedImage() const;

  // Returns the decoding mode that should be used when painting this element,
  // given the PaintImage::Id that will be used to paint it.
  // Used with HTMLImageElement and SVGImageElement types.
  Image::ImageDecodingMode GetDecodingModeForPainting(PaintImage::Id);

 protected:
  Image::ImageDecodingMode decoding_mode_ =
      Image::ImageDecodingMode::kUnspecifiedDecode;

 private:
  const Element& GetElement() const;

  mojom::blink::PreferredColorScheme PreferredColorScheme() const;

  // The id for the PaintImage used the last time this element was painted.
  PaintImage::Id last_painted_image_id_ = PaintImage::kInvalidId;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CANVAS_IMAGE_ELEMENT_BASE_H_
