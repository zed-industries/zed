/*
 * Copyright (C) 2010 Apple Inc. All rights reserved.
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
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_PENDING_IMAGE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_PENDING_IMAGE_H_

#include "base/memory/values_equivalent.h"
#include "base/notreached.h"
#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/core/style/style_image.h"
#include "third_party/blink/renderer/platform/graphics/image.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class ImageResourceObserver;

// StylePendingImage is a placeholder StyleImage that is entered into the
// ComputedStyle during style resolution, in order to avoid loading images that
// are not referenced by the final style.  They should only exist in a
// ComputedStyle for non-rendered elements created with EnsureComputedStyle or
// display:contents.
class CORE_EXPORT StylePendingImage final : public StyleImage {
 public:
  explicit StylePendingImage(const CSSValue& value)
      : value_(const_cast<CSSValue*>(&value)) {
    is_pending_image_ = true;
  }

  WrappedImagePtr Data() const override { return value_.Get(); }

  CSSValue* CssValue() const override { return value_.Get(); }

  CSSValue* ComputedCSSValue(const ComputedStyle& style,
                             bool allow_visited_style,
                             CSSValuePhase value_phase) const override;

  bool IsAccessAllowed(String&) const override { return true; }
  NaturalSizingInfo GetNaturalSizingInfo(
      float multiplier,
      RespectImageOrientationEnum) const override {
    return NaturalSizingInfo();
  }
  gfx::SizeF ImageSize(float,
                       const gfx::SizeF&,
                       RespectImageOrientationEnum) const override {
    return gfx::SizeF();
  }
  bool HasIntrinsicSize() const override { return true; }
  void AddClient(ImageResourceObserver*) override {}
  void RemoveClient(ImageResourceObserver*) override {}
  scoped_refptr<Image> GetImage(const ImageResourceObserver&,
                                const Document&,
                                const ComputedStyle&,
                                const gfx::SizeF& target_size) const override {
    DUMP_WILL_BE_NOTREACHED();
    return nullptr;
  }
  bool KnownToBeOpaque(const Document&, const ComputedStyle&) const override {
    return false;
  }

  void Trace(Visitor* visitor) const override {
    visitor->Trace(value_);
    StyleImage::Trace(visitor);
  }

 private:
  bool IsEqual(const StyleImage& other) const override;

  Member<CSSValue> value_;
};

template <>
struct DowncastTraits<StylePendingImage> {
  static bool AllowFrom(const StyleImage& styleImage) {
    return styleImage.IsPendingImage();
  }
};

inline bool StylePendingImage::IsEqual(const StyleImage& other) const {
  // Ignore pending status when comparing; as long as the values are
  // the same, the images should be considered equal, too.
  return base::ValuesEquivalent(value_.Get(), other.CssValue());
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_PENDING_IMAGE_H_
