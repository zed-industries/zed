// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_FILTERS_PAINT_FILTER_EFFECT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_FILTERS_PAINT_FILTER_EFFECT_H_

#include "cc/paint/paint_flags.h"
#include "third_party/blink/renderer/platform/graphics/filters/filter_effect.h"

namespace blink {

class PLATFORM_EXPORT PaintFilterEffect : public FilterEffect {
 public:
  PaintFilterEffect(Filter*, const cc::PaintFlags&);
  ~PaintFilterEffect() override;

  FilterEffectType GetFilterEffectType() const override {
    return kFilterEffectTypeSourceInput;
  }

  StringBuilder& ExternalRepresentation(StringBuilder&,
                                        wtf_size_t indent) const override;
  sk_sp<PaintFilter> CreateImageFilter() override;

 private:
  cc::PaintFlags flags_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_FILTERS_PAINT_FILTER_EFFECT_H_
