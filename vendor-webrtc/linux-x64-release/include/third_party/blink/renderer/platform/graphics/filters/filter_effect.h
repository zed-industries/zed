/*
 * Copyright (C) 2008 Alex Mathews <possessedpenguinbob@gmail.com>
 * Copyright (C) 2009 Dirk Schulze <krit@webkit.org>
 * Copyright (C) Research In Motion Limited 2010. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_FILTERS_FILTER_EFFECT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_FILTERS_FILTER_EFFECT_H_

#include <array>
#include <optional>

#include "third_party/blink/renderer/platform/graphics/color.h"
#include "third_party/blink/renderer/platform/graphics/interpolation_space.h"
#include "third_party/blink/renderer/platform/graphics/paint/paint_filter.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "ui/gfx/geometry/rect_f.h"

namespace blink {

class Filter;
class FilterEffect;

typedef HeapVector<Member<FilterEffect>> FilterEffectVector;

enum FilterEffectType {
  kFilterEffectTypeUnknown,
  kFilterEffectTypeImage,
  kFilterEffectTypeTile,
  kFilterEffectTypeSourceInput
};

class PLATFORM_EXPORT FilterEffect : public GarbageCollected<FilterEffect> {
 public:
  FilterEffect(const FilterEffect&) = delete;
  FilterEffect& operator=(const FilterEffect&) = delete;
  virtual ~FilterEffect();
  virtual void Trace(Visitor*) const;

  void DisposeImageFilters();
  void DisposeImageFiltersRecursive();

  FilterEffectVector& InputEffects() { return input_effects_; }
  FilterEffect* InputEffect(unsigned) const;
  unsigned NumberOfEffectInputs() const { return input_effects_.size(); }

  inline bool HasImageFilter() const {
    return image_filters_[0] || image_filters_[1] || image_filters_[2] ||
           image_filters_[3];
  }

  // Clipped primitive subregion in the coordinate space of the target.
  gfx::RectF AbsoluteBounds() const;

  // Mapping a rect forwards to determine which which destination pixels a
  // given source rect would affect.
  gfx::RectF MapRect(const gfx::RectF&) const;

  virtual sk_sp<PaintFilter> CreateImageFilter();
  virtual sk_sp<PaintFilter> CreateImageFilterWithoutValidation();

  virtual FilterEffectType GetFilterEffectType() const {
    return kFilterEffectTypeUnknown;
  }

  virtual StringBuilder& ExternalRepresentation(StringBuilder&,
                                                wtf_size_t indention = 0) const;

  gfx::RectF FilterPrimitiveSubregion() const {
    return filter_primitive_subregion_;
  }
  void SetFilterPrimitiveSubregion(
      const gfx::RectF& filter_primitive_subregion) {
    filter_primitive_subregion_ = filter_primitive_subregion;
  }

  Filter* GetFilter() { return filter_.Get(); }
  const Filter* GetFilter() const { return filter_.Get(); }

  bool ClipsToBounds() const { return clips_to_bounds_; }
  void SetClipsToBounds(bool value) { clips_to_bounds_ = value; }

  InterpolationSpace OperatingInterpolationSpace() const {
    return operating_interpolation_space_;
  }
  virtual void SetOperatingInterpolationSpace(
      InterpolationSpace interpolation_space) {
    operating_interpolation_space_ = interpolation_space;
  }

  virtual bool AffectsTransparentPixels() const { return false; }

  // Return false if the filter will only operate correctly on valid RGBA
  // values, with alpha in [0,255] and each color component in [0, alpha].
  virtual bool MayProduceInvalidPreMultipliedPixels() { return false; }

  PaintFilter* GetImageFilter(InterpolationSpace,
                              bool requires_pm_color_validation) const;
  void SetImageFilter(InterpolationSpace,
                      bool requires_pm_color_validation,
                      sk_sp<PaintFilter>);

  bool OriginTainted() const { return origin_tainted_; }
  void SetOriginTainted() { origin_tainted_ = true; }

  bool InputsTaintOrigin() const;

 protected:
  FilterEffect(Filter*);

  // Determine the contribution from the filter effect's inputs.
  virtual gfx::RectF MapInputs(const gfx::RectF&) const;

  // Apply the contribution from the filter effect's itself. (Like
  // expanding with the blur radius etc.)
  virtual gfx::RectF MapEffect(const gfx::RectF&) const;

  // Apply the clip bounds and factor in the effect of
  // affectsTransparentPixels().
  gfx::RectF ApplyBounds(const gfx::RectF&) const;

  sk_sp<PaintFilter> CreateTransparentBlack() const;

  Color AdaptColorToOperatingInterpolationSpace(const Color& device_color);

  std::optional<PaintFilter::CropRect> GetCropRect() const;

 private:
  FilterEffectVector input_effects_;

  Member<Filter> filter_;

  // The following member variables are SVG specific and will move to
  // LayoutSVGResourceFilterPrimitive.
  // See bug https://bugs.webkit.org/show_bug.cgi?id=45614.

  // The subregion of a filter primitive according to the SVG Filter
  // specification in local coordinates.
  gfx::RectF filter_primitive_subregion_;

  // Whether the effect should clip to its primitive region, or expand to use
  // the combined region of its inputs.
  bool clips_to_bounds_;

  bool origin_tainted_;

  InterpolationSpace operating_interpolation_space_;

  std::array<sk_sp<PaintFilter>, 4> image_filters_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_FILTERS_FILTER_EFFECT_H_
