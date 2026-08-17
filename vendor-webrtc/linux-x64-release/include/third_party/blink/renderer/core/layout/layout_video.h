/*
 * Copyright (C) 2007, 2009, 2010 Apple Inc. All rights reserved.
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
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_VIDEO_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_VIDEO_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/layout_media.h"
#include "third_party/blink/renderer/core/layout/natural_sizing_info.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class HTMLVideoElement;

class CORE_EXPORT LayoutVideo final : public LayoutMedia {
 public:
  explicit LayoutVideo(HTMLVideoElement*);
  ~LayoutVideo() override;

  static PhysicalSize DefaultSize();

  PhysicalRect ReplacedContentRectFrom(
      const PhysicalRect& base_content_rect) const final;

  bool SupportsAcceleratedRendering() const;

  enum DisplayMode { kPoster, kVideo };
  DisplayMode GetDisplayMode() const;

  HTMLVideoElement* VideoElement() const;

  void StyleDidChange(StyleDifference, const ComputedStyle* old_style) override;

  const char* GetName() const override {
    NOT_DESTROYED();
    return "LayoutVideo";
  }

  void NaturalSizeChanged() override;

  OverflowClipAxes ComputeOverflowClipAxes() const final {
    NOT_DESTROYED();
    return RespectsCSSOverflow() ? LayoutMedia::ComputeOverflowClipAxes()
                                 : kOverflowClipBothAxis;
  }

 private:
  void UpdateAfterLayout() final;
  void UpdateFromElement() final;
  void InvalidateCompositing();

  PhysicalNaturalSizingInfo GetNaturalDimensions() const override;
  void UpdateNaturalSize();

  void ImageChanged(WrappedImagePtr, CanDeferInvalidation) override;

  bool IsVideo() const final {
    NOT_DESTROYED();
    return true;
  }

  void PaintReplaced(const PaintInfo&,
                     const PhysicalOffset& paint_offset) const override;

  bool CanHaveAdditionalCompositingReasons() const override {
    NOT_DESTROYED();
    return true;
  }
  CompositingReasons AdditionalCompositingReasons() const override;

  PhysicalNaturalSizingInfo natural_dimensions_;
};

template <>
struct DowncastTraits<LayoutVideo> {
  static bool AllowFrom(const LayoutObject& object) { return object.IsVideo(); }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_VIDEO_H_
