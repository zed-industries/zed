/*
 * Copyright (C) 2000 Lars Knoll (knoll@kde.org)
 *           (C) 2000 Antti Koivisto (koivisto@kde.org)
 *           (C) 2000 Dirk Mueller (mueller@kde.org)
 * Copyright (C) 2003, 2005, 2006, 2007, 2008 Apple Inc. All rights reserved.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_FETCHED_IMAGE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_FETCHED_IMAGE_H_

#include "third_party/blink/renderer/core/loader/resource/image_resource_observer.h"
#include "third_party/blink/renderer/core/style/style_image.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class Document;

// This class represents an <image> that loads a single image resource (the
// url(...) function.)
class CORE_EXPORT StyleFetchedImage final : public StyleImage,
                                            public ImageResourceObserver {
  USING_PRE_FINALIZER(StyleFetchedImage, Prefinalize);

 public:
  StyleFetchedImage(ImageResourceContent* image,
                    const Document& document,
                    bool is_lazyload_possibly_deferred,
                    bool origin_clean,
                    bool is_ad_related,
                    const KURL& url,
                    const float override_image_resolution = 0.0f);
  ~StyleFetchedImage() override;

  WrappedImagePtr Data() const override;

  float ImageScaleFactor() const override;

  CSSValue* CssValue() const override;
  CSSValue* ComputedCSSValue(const ComputedStyle&,
                             bool allow_visited_style,
                             CSSValuePhase value_phase) const override;

  bool CanRender() const override;
  bool IsLoaded() const override;
  bool IsLoading() const override;
  bool ErrorOccurred() const override;
  bool IsAccessAllowed(String&) const override;

  NaturalSizingInfo GetNaturalSizingInfo(
      float multiplier,
      RespectImageOrientationEnum) const override;
  gfx::SizeF ImageSize(float multiplier,
                       const gfx::SizeF& default_object_size,
                       RespectImageOrientationEnum) const override;
  bool HasIntrinsicSize() const override;
  void AddClient(ImageResourceObserver*) override;
  void RemoveClient(ImageResourceObserver*) override;
  String DebugName() const override { return "StyleFetchedImage"; }
  scoped_refptr<Image> GetImage(const ImageResourceObserver&,
                                const Document&,
                                const ComputedStyle&,
                                const gfx::SizeF& target_size) const override;
  bool KnownToBeOpaque(const Document&, const ComputedStyle&) const override;
  ImageResourceContent* CachedImage() const override;

  void LoadDeferredImage(const Document& document);

  RespectImageOrientationEnum ForceOrientationIfNecessary(
      RespectImageOrientationEnum default_orientation) const override;

  void Trace(Visitor*) const override;

  bool IsOriginClean() const override { return origin_clean_; }

 private:
  bool IsEqual(const StyleImage&) const override;
  void Prefinalize();

  // Apply the image's natural/override resolution to `multiplier`, producing a
  // scale factor that will yield "zoomed CSS pixels".
  float ApplyImageResolution(float multiplier) const;

  // ImageResourceObserver overrides
  void ImageNotifyFinished(ImageResourceContent*) override;
  bool GetImageAnimationPolicy(mojom::blink::ImageAnimationPolicy&) override;
  bool CanBeSpeculativelyDecoded() const override;

  Member<ImageResourceContent> image_;
  Member<const Document> document_;

  const KURL url_;

  // This overrides an images natural resolution.
  // A value of zero indicates no override.
  const float override_image_resolution_;

  const bool origin_clean_;

  // Whether this was created by an ad-related CSSParserContext.
  const bool is_ad_related_;
};

template <>
struct DowncastTraits<StyleFetchedImage> {
  static bool AllowFrom(const StyleImage& styleImage) {
    return styleImage.IsImageResource();
  }
};

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_FETCHED_IMAGE_H_
