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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_FILTERS_FE_FLOOD_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_FILTERS_FE_FLOOD_H_

#include "third_party/blink/renderer/platform/graphics/filters/filter_effect.h"
#include "third_party/skia/include/core/SkColor.h"

namespace blink {

class PLATFORM_EXPORT FEFlood final : public FilterEffect {
 public:
  FEFlood(Filter*, const SkColor4f&, float);

  SkColor4f FloodColor() const;
  bool SetFloodColor(const SkColor4f&);

  float FloodOpacity() const;
  bool SetFloodOpacity(float);

  // feFlood does not perform color interpolation of any kind, so the result is
  // always in the current color space regardless of the value of
  // color-interpolation-filters.
  void SetOperatingInterpolationSpace(InterpolationSpace) override {}

  StringBuilder& ExternalRepresentation(StringBuilder&,
                                        wtf_size_t indent) const override;

 private:
  sk_sp<PaintFilter> CreateImageFilter() override;

  SkColor4f flood_color_;
  float flood_opacity_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_FILTERS_FE_FLOOD_H_
