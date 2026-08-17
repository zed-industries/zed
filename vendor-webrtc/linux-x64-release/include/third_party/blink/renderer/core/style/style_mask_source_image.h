// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_MASK_SOURCE_IMAGE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_MASK_SOURCE_IMAGE_H_

#include "third_party/blink/renderer/core/style/style_image.h"

#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

class CSSImageValue;
class StyleFetchedImage;
class SVGResource;
class SVGResourceClient;

// A pseudo-<image> representing the <mask-source> production described in
// [1]. It's a url() <image> that can either be a regular image, or an SVG
// <mask> reference.
//
// If the reference is non-local this wraps an StyleFetchedImage, and a
// corresponding SVGResource that wraps the same ImageResourceContent as the
// StyleFetchedImage. If the url() has fragment that in turn references an SVG
// <mask> element this can be used to paint/generate a mask from that source.
//
// If the reference is local an SVGResource is wrapped.
//
// [1] https://drafts.fxtf.org/css-masking/#the-mask-image
class StyleMaskSourceImage : public StyleImage {
 public:
  StyleMaskSourceImage(StyleFetchedImage*, SVGResource*, CSSImageValue*);
  StyleMaskSourceImage(SVGResource*, CSSImageValue*);
  ~StyleMaskSourceImage() override;

  CSSValue* CssValue() const override;
  CSSValue* ComputedCSSValue(const ComputedStyle&,
                             bool allow_visited_style,
                             CSSValuePhase value_phase) const override;

  bool CanRender() const override;
  bool IsLoaded() const override;
  bool IsLoading() const override;
  bool ErrorOccurred() const override;
  bool IsAccessAllowed(WTF::String& failing_url) const override;

  NaturalSizingInfo GetNaturalSizingInfo(
      float multiplier,
      RespectImageOrientationEnum) const override;

  gfx::SizeF ImageSize(float multiplier,
                       const gfx::SizeF& default_object_size,
                       RespectImageOrientationEnum) const override;

  bool HasIntrinsicSize() const override;

  void AddClient(ImageResourceObserver*) override;
  void RemoveClient(ImageResourceObserver*) override;

  scoped_refptr<Image> GetImage(const ImageResourceObserver&,
                                const Document&,
                                const ComputedStyle&,
                                const gfx::SizeF& target_size) const override;
  float ImageScaleFactor() const override;

  WrappedImagePtr Data() const override;

  bool KnownToBeOpaque(const Document&, const ComputedStyle&) const override;
  ImageResourceContent* CachedImage() const override;

  bool HasSVGMask() const;
  SVGResource* GetSVGResource() const;
  SVGResourceClient* GetSVGResourceClient(const ImageResourceObserver&) const;

  void Trace(Visitor* visitor) const override;

 private:
  bool IsEqual(const StyleImage&) const override;

  // url() <image> being wrapped. Will be null if keeping a document-local
  // resource.
  Member<StyleFetchedImage> image_;

  // SVG resource. Can be null if keeping a document-local resource for an
  // empty fragment.
  Member<SVGResource> resource_;

  // Original CSS value.
  Member<CSSImageValue> resource_css_value_;
};

template <>
struct DowncastTraits<StyleMaskSourceImage> {
  static bool AllowFrom(const StyleImage& style_image) {
    return style_image.IsMaskSource();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_MASK_SOURCE_IMAGE_H_
