/*
 * Copyright (C) 2011 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_FILTER_EFFECT_BUILDER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_FILTER_EFFECT_BUILDER_H_

#include "cc/paint/paint_flags.h"
#include "third_party/blink/public/mojom/frame/color_scheme.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/graphics/color.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "ui/gfx/geometry/rect_f.h"

namespace blink {

class CompositorFilterOperations;
class Filter;
class FilterEffect;
class FilterOperations;
class ReferenceFilterOperation;
class SVGFilterGraphNodeMap;

class CORE_EXPORT FilterEffectBuilder final {
  STACK_ALLOCATED();

 public:
  FilterEffectBuilder(const gfx::RectF& reference_box,
                      std::optional<gfx::SizeF> viewport,
                      float zoom,
                      Color current_color,
                      mojom::blink::ColorScheme color_scheme,
                      const cc::PaintFlags* fill_flags = nullptr,
                      const cc::PaintFlags* stroke_flags = nullptr);

  Filter* BuildReferenceFilter(const ReferenceFilterOperation&,
                               FilterEffect* previous_effect,
                               SVGFilterGraphNodeMap* = nullptr) const;

  FilterEffect* BuildFilterEffect(const FilterOperations&,
                                  bool input_tainted = false) const;
  CompositorFilterOperations BuildFilterOperations(
      const FilterOperations&) const;

  void SetShorthandScale(float shorthand_scale) {
    shorthand_scale_ = shorthand_scale;
  }

 private:
  const gfx::RectF reference_box_;
  const std::optional<gfx::SizeF> viewport_;
  const float zoom_;
  float shorthand_scale_;  // Scale factor for shorthand filter functions.
  const Color current_color_;
  const mojom::blink::ColorScheme color_scheme_;
  const cc::PaintFlags* fill_flags_;
  const cc::PaintFlags* stroke_flags_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_FILTER_EFFECT_BUILDER_H_
