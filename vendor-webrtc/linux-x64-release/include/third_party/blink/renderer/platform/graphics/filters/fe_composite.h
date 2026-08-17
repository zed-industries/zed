/*
 * Copyright (C) 2004, 2005, 2006, 2007 Nikolas Zimmermann <zimmermann@kde.org>
 * Copyright (C) 2004, 2005 Rob Buis <buis@kde.org>
 * Copyright (C) 2005 Eric Seidel <eric@webkit.org>
 * Copyright (C) 2013 Google Inc. All rights reserved.
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
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_FILTERS_FE_COMPOSITE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_FILTERS_FE_COMPOSITE_H_

#include "third_party/blink/renderer/platform/graphics/filters/filter_effect.h"
#include "third_party/skia/include/core/SkBlendMode.h"

namespace blink {

enum CompositeOperationType {
  FECOMPOSITE_OPERATOR_UNKNOWN = 0,
  FECOMPOSITE_OPERATOR_OVER = 1,
  FECOMPOSITE_OPERATOR_IN = 2,
  FECOMPOSITE_OPERATOR_OUT = 3,
  FECOMPOSITE_OPERATOR_ATOP = 4,
  FECOMPOSITE_OPERATOR_XOR = 5,
  FECOMPOSITE_OPERATOR_ARITHMETIC = 6,
  FECOMPOSITE_OPERATOR_LIGHTER = 7
};

class PLATFORM_EXPORT FEComposite final : public FilterEffect {
 public:
  FEComposite(Filter*,
              const CompositeOperationType&,
              float,
              float,
              float,
              float);

  CompositeOperationType Operation() const;
  bool SetOperation(CompositeOperationType);

  float K1() const;
  bool SetK1(float);

  float K2() const;
  bool SetK2(float);

  float K3() const;
  bool SetK3(float);

  float K4() const;
  bool SetK4(float);

  StringBuilder& ExternalRepresentation(StringBuilder&,
                                        wtf_size_t indent) const override;

 protected:
  bool MayProduceInvalidPreMultipliedPixels() override {
    return type_ == FECOMPOSITE_OPERATOR_ARITHMETIC;
  }

 private:
  gfx::RectF MapInputs(const gfx::RectF&) const override;

  bool AffectsTransparentPixels() const override;

  sk_sp<PaintFilter> CreateImageFilter() override;
  sk_sp<PaintFilter> CreateImageFilterWithoutValidation() override;
  sk_sp<PaintFilter> CreateImageFilterInternal(
      bool requires_pm_color_validation);

  CompositeOperationType type_;
  float k1_;
  float k2_;
  float k3_;
  float k4_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_FILTERS_FE_COMPOSITE_H_
