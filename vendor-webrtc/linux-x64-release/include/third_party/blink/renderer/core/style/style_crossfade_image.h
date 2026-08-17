// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_CROSSFADE_IMAGE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_CROSSFADE_IMAGE_H_

#include "third_party/blink/renderer/core/style/style_image.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

class CSSLengthResolver;

namespace cssvalue {
class CSSCrossfadeValue;
}

// This class represents a <cross-fade()> <image> function.
class StyleCrossfadeImage final : public StyleImage {
 public:
  StyleCrossfadeImage(cssvalue::CSSCrossfadeValue&,
                      HeapVector<Member<StyleImage>> images,
                      const CSSLengthResolver&);
  ~StyleCrossfadeImage() override;

  CSSValue* CssValue() const override;
  CSSValue* ComputedCSSValue(const ComputedStyle&,
                             bool allow_visited_style,
                             CSSValuePhase value_phase) const override;

  bool CanRender() const override;
  bool IsLoading() const override;
  bool IsLoaded() const override;
  bool ErrorOccurred() const override;
  bool IsAccessAllowed(WTF::String&) const override;

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

  WrappedImagePtr Data() const override;
  bool KnownToBeOpaque(const Document&, const ComputedStyle&) const override;

  void Trace(Visitor*) const override;

 private:
  bool IsEqual(const StyleImage&) const override;

  // Can only return true for -webkit-cross-fade, not cross-fade.
  bool AnyImageIsNone() const;

  // Converts from 0..100 to 0..1, and fills in missing percentages.
  // If for_sizing is true, skips all <color> StyleImages, since they
  // do not participate in the sizing algorithms.
  HeapVector<float> ComputeWeights(const CSSLengthResolver& length_resolver,
                                   bool for_sizing) const;

  Member<cssvalue::CSSCrossfadeValue> original_value_;
  HeapVector<Member<StyleImage>> images_;

  // Weights for the actual cross-fade operation.
  HeapVector<float> weights_;

  // Weights for non-<color> images (see comment on ComputeWeights()).
  HeapVector<float> weights_for_sizing_;
};

template <>
struct DowncastTraits<StyleCrossfadeImage> {
  static bool AllowFrom(const StyleImage& style_image) {
    return style_image.IsCrossfadeImage();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_CROSSFADE_IMAGE_H_
