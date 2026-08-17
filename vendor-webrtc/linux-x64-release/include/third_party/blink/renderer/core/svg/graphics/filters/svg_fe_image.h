/*
 * Copyright (C) 2004, 2005, 2006, 2007 Nikolas Zimmermann <zimmermann@kde.org>
 * Copyright (C) 2004, 2005 Rob Buis <buis@kde.org>
 * Copyright (C) 2005 Eric Seidel <eric@webkit.org>
 * Copyright (C) 2010 Dirk Schulze <krit@webkit.org>
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_GRAPHICS_FILTERS_SVG_FE_IMAGE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_GRAPHICS_FILTERS_SVG_FE_IMAGE_H_

#include "third_party/blink/renderer/platform/graphics/filters/filter_effect.h"

namespace blink {

class AffineTransform;
class Image;
class LayoutObject;
class SVGElement;
class SVGPreserveAspectRatio;

class FEImage final : public FilterEffect {
 public:
  FEImage(Filter*, scoped_refptr<Image>, const SVGPreserveAspectRatio*);
  FEImage(Filter*, const SVGElement*, const SVGPreserveAspectRatio*);
  ~FEImage() override = default;

  // feImage does not perform color interpolation of any kind, so doesn't
  // depend on the value of color-interpolation-filters.
  void SetOperatingInterpolationSpace(InterpolationSpace) override {}

  StringBuilder& ExternalRepresentation(StringBuilder&,
                                        wtf_size_t indent) const override;

  void Trace(Visitor*) const override;

 private:
  const LayoutObject* ReferencedLayoutObject() const;
  scoped_refptr<Image> GetImage(const gfx::SizeF& container_size) const;

  FilterEffectType GetFilterEffectType() const override {
    return kFilterEffectTypeImage;
  }

  AffineTransform SourceToDestinationTransform(
      const LayoutObject& layout_object,
      const gfx::RectF& dest_rect) const;
  gfx::RectF MapInputs(const gfx::RectF&) const override;

  sk_sp<PaintFilter> CreateImageFilter() override;
  // The `dst_rect` and `crop_rect` arguments are in (potentially) zoomed user
  // space coordinates (essentially "zoomed CSS pixels").
  sk_sp<PaintFilter> CreateImageFilterForLayoutObject(
      const LayoutObject&,
      const gfx::RectF& dst_rect,
      const gfx::RectF& crop_rect);

  scoped_refptr<Image> image_;
  Member<const SVGElement> element_;
  Member<const SVGPreserveAspectRatio> preserve_aspect_ratio_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_GRAPHICS_FILTERS_SVG_FE_IMAGE_H_
