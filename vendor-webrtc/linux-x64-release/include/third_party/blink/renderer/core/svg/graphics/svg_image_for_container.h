/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
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
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_GRAPHICS_SVG_IMAGE_FOR_CONTAINER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_GRAPHICS_SVG_IMAGE_FOR_CONTAINER_H_

#include "third_party/blink/public/mojom/css/preferred_color_scheme.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/svg/graphics/svg_image.h"
#include "third_party/blink/renderer/platform/graphics/image.h"
#include "ui/gfx/geometry/rect_f.h"
#include "ui/gfx/geometry/size_f.h"

namespace blink {

class Element;
class KURL;
class Node;

// SVGImageForContainer contains a reference to an SVGImage and includes context
// about how the image is being used (size, fragment identifier).
//
// The concrete size of an SVG image is calculated based on the image itself and
// the dimensions where the image is used (see: SVGImage::ConcreteObjectSize).
// This concrete size cannot be stored on the SVGImage itself because only a
// single SVGImage is created per SVG image resource, but this SVGImage can be
// referenced multiple times by containers of different sizes. Similarly, each
// use of an image can have a different fragment identifier as part of its URL
// (e.g., foo.svg#abc) which can influence rendering.
//
// For example, the following would create three SVGImageForContainers
// referencing a single SVGImage for 'foo.svg':
// <img src='foo.svg#a' width='20'>
// <img src='foo.svg#a' width='10'>
// <img src='foo.svg#b' width='10'>
//
// SVGImageForContainer stores this per-use information and delegates to the
// SVGImage for how to draw the image.
class CORE_EXPORT SVGImageForContainer final : public Image {
  USING_FAST_MALLOC(SVGImageForContainer);

 public:
  static scoped_refptr<SVGImageForContainer> Create(
      SVGImage& image,
      const gfx::SizeF& target_size,
      float zoom,
      const SVGImageViewInfo* viewinfo) {
    gfx::SizeF container_size_without_zoom =
        gfx::ScaleSize(target_size, 1 / zoom);
    return base::AdoptRef(new SVGImageForContainer(
        image, container_size_without_zoom, zoom, viewinfo));
  }

  static scoped_refptr<SVGImageForContainer> Create(
      SVGImage& image,
      const gfx::SizeF& target_size,
      float zoom,
      const SVGImageViewInfo* viewinfo,
      mojom::blink::PreferredColorScheme preferred_color_scheme) {
    gfx::SizeF container_size_without_zoom =
        gfx::ScaleSize(target_size, 1 / zoom);
    return base::AdoptRef(
        new SVGImageForContainer(image, container_size_without_zoom, zoom,
                                 viewinfo, preferred_color_scheme));
  }

  // Create view info for an SVGImage with a fragment identifier string.
  static const SVGImageViewInfo* CreateViewInfo(SVGImage&,
                                                const String& fragment);

  // Create view info for an SVGImage with a URL. The same as calling the above
  // with the URL's fragment identifier.
  static const SVGImageViewInfo* CreateViewInfo(SVGImage&, const KURL&);

  // Create view info for an SVGImage with an Element as the context. Will use
  // `Element::ImageSourceURL()` resolved against the Element's Document.
  static const SVGImageViewInfo* CreateViewInfo(SVGImage&, const Element&);

  // Create view info for an SVGImage with a Node as the context. Essentially a
  // convenience wrapper for `MakeWithElement()`.
  static const SVGImageViewInfo* CreateViewInfo(SVGImage&, const Node*);

  // Get the natural dimensions for the SVGImage with the view info
  // applied. Returns false if the SVGImage lacks a document element, otherwise
  // true.
  static std::optional<NaturalSizingInfo> GetNaturalDimensions(
      SVGImage& image,
      const SVGImageViewInfo* info);

  // Determine the concrete object size of this SVGImage with the view info
  // applied using the specified default object size (in CSS pixels).
  static gfx::SizeF ConcreteObjectSize(SVGImage& image,
                                       const SVGImageViewInfo* info,
                                       const gfx::SizeF& default_object_size);

  gfx::Size SizeWithConfig(SizeConfig) const override;
  gfx::SizeF SizeWithConfigAsFloat(SizeConfig) const override;

  bool HasIntrinsicSize() const override;

  bool ApplyShader(cc::PaintFlags&,
                   const SkMatrix& local_matrix,
                   const gfx::RectF& src_rect,
                   const ImageDrawOptions& draw_options) override;

  void Draw(cc::PaintCanvas*,
            const cc::PaintFlags&,
            const gfx::RectF& dest_rect,
            const gfx::RectF& src_rect,
            const ImageDrawOptions&) override;

  // FIXME: Implement this to be less conservative.
  bool CurrentFrameKnownToBeOpaque() override { return false; }

  PaintImage PaintImageForCurrentFrame() override;

 protected:
  void DrawPattern(GraphicsContext&,
                   const cc::PaintFlags&,
                   const gfx::RectF& dest_rect,
                   const ImageTilingInfo&,
                   const ImageDrawOptions& draw_options) override;

 private:
  SVGImageForContainer(
      SVGImage& image,
      const gfx::SizeF& container_size,
      float zoom,
      const SVGImageViewInfo* viewinfo,
      mojom::blink::PreferredColorScheme preferred_color_scheme);
  SVGImageForContainer(SVGImage& image,
                       const gfx::SizeF& container_size,
                       float zoom,
                       const SVGImageViewInfo* viewinfo);

  void DestroyDecodedData() override {}

  SVGImage& image_;
  Persistent<const SVGImageViewInfo> viewinfo_;
  const gfx::SizeF container_size_;
  const float zoom_;
};
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_GRAPHICS_SVG_IMAGE_FOR_CONTAINER_H_
