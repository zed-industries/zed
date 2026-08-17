/*
 * Copyright (C) 2012 Apple Inc. All rights reserved.
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
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. AND ITS CONTRIBUTORS ``AS IS''
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO,
 * THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL APPLE INC. OR ITS CONTRIBUTORS
 * BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF
 * THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_IMAGE_SET_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_IMAGE_SET_H_

#include "third_party/blink/renderer/core/css/css_image_set_value.h"
#include "third_party/blink/renderer/core/style/style_image.h"
#include "third_party/blink/renderer/platform/graphics/image.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class CSSImageSetValue;

// This class represents an <image> that loads one image resource out of a set
// of alternatives (the image-set(...) function.)
//
// This class keeps one cached image from the set, and has access to a set of
// alternatives via the referenced CSSImageSetValue.
class StyleImageSet final : public StyleImage {
 public:
  StyleImageSet(StyleImage* best_fit_image,
                CSSImageSetValue* image_set_val,
                bool is_origin_clean);
  ~StyleImageSet() override;

  CSSValue* CssValue() const override;
  CSSValue* ComputedCSSValue(const ComputedStyle&,
                             bool allow_visited_style,
                             CSSValuePhase value_phase) const override;

  WrappedImagePtr Data() const override;

  bool CanRender() const override;
  bool IsLoading() const override;
  bool IsLoaded() const override;
  bool ErrorOccurred() const override;
  bool IsAccessAllowed(String& failing_url) const override;
  bool IsOriginClean() const override { return is_origin_clean_; }

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

  bool KnownToBeOpaque(const Document&, const ComputedStyle&) const override;

  ImageResourceContent* CachedImage() const override;

  RespectImageOrientationEnum ForceOrientationIfNecessary(
      RespectImageOrientationEnum default_orientation) const override;

  void Trace(Visitor*) const override;

 private:
  bool IsEqual(const StyleImage& other) const override;

  Member<StyleImage> best_fit_image_;

  Member<CSSImageSetValue> image_set_value_;  // Not retained; it owns us.

  bool is_origin_clean_;
};

template <>
struct DowncastTraits<StyleImageSet> {
  static bool AllowFrom(const StyleImage& styleImage) {
    return styleImage.IsImageResourceSet();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_IMAGE_SET_H_
