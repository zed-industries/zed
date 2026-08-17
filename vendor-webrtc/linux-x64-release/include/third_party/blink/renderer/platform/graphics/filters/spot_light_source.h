/*
 * Copyright (C) 2008 Alex Mathews <possessedpenguinbob@gmail.com>
 * Copyright (C) 2004, 2005, 2006, 2007 Nikolas Zimmermann <zimmermann@kde.org>
 * Copyright (C) 2004, 2005 Rob Buis <buis@kde.org>
 * Copyright (C) 2005 Eric Seidel <eric@webkit.org>
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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_FILTERS_SPOT_LIGHT_SOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_FILTERS_SPOT_LIGHT_SOURCE_H_

#include "third_party/blink/renderer/platform/graphics/filters/light_source.h"
#include "third_party/blink/renderer/platform/wtf/math_extras.h"
#include "ui/gfx/geometry/point3_f.h"

namespace blink {

class PLATFORM_EXPORT SpotLightSource final : public LightSource {
 public:
  static scoped_refptr<SpotLightSource> Create(const gfx::Point3F& position,
                                               const gfx::Point3F& points_at,
                                               float specular_exponent,
                                               float limiting_cone_angle) {
    return base::AdoptRef(new SpotLightSource(
        position, points_at, specular_exponent, limiting_cone_angle));
  }

  const gfx::Point3F& GetPosition() const { return position_; }
  const gfx::Point3F& PointsAt() const { return points_at_; }
  float SpecularExponent() const { return specular_exponent_; }
  float LimitingConeAngle() const { return limiting_cone_angle_; }

  bool SetPosition(const gfx::Point3F&) override;
  bool SetPointsAt(const gfx::Point3F&) override;

  bool SetSpecularExponent(float) override;
  bool SetLimitingConeAngle(float) override;

  StringBuilder& ExternalRepresentation(StringBuilder&) const override;

 private:
  SpotLightSource(const gfx::Point3F& position,
                  const gfx::Point3F& points_at,
                  float specular_exponent,
                  float limiting_cone_angle)
      : LightSource(kLsSpot),
        position_(position),
        points_at_(points_at),
        specular_exponent_(ClampTo(specular_exponent, 1.0f, 128.0f)),
        limiting_cone_angle_(limiting_cone_angle) {}

  gfx::Point3F position_;
  gfx::Point3F points_at_;

  float specular_exponent_;
  float limiting_cone_angle_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_FILTERS_SPOT_LIGHT_SOURCE_H_
