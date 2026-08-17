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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_FILTERS_FE_MORPHOLOGY_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_FILTERS_FE_MORPHOLOGY_H_

#include "third_party/blink/renderer/platform/graphics/filters/filter_effect.h"

namespace blink {

enum MorphologyOperatorType {
  FEMORPHOLOGY_OPERATOR_UNKNOWN = 0,
  FEMORPHOLOGY_OPERATOR_ERODE = 1,
  FEMORPHOLOGY_OPERATOR_DILATE = 2
};

class PLATFORM_EXPORT FEMorphology final : public FilterEffect {
 public:
  FEMorphology(Filter*, MorphologyOperatorType, float radius_x, float radius_y);

  MorphologyOperatorType MorphologyOperator() const;
  bool SetMorphologyOperator(MorphologyOperatorType);

  float RadiusX() const;
  bool SetRadiusX(float);

  float RadiusY() const;
  bool SetRadiusY(float);

  StringBuilder& ExternalRepresentation(StringBuilder&,
                                        wtf_size_t indent) const override;

 private:
  gfx::RectF MapEffect(const gfx::RectF&) const override;

  sk_sp<PaintFilter> CreateImageFilter() override;

  MorphologyOperatorType type_;
  float radius_x_;
  float radius_y_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_FILTERS_FE_MORPHOLOGY_H_
