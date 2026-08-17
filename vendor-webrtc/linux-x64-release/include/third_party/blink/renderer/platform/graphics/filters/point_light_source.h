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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_FILTERS_POINT_LIGHT_SOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_FILTERS_POINT_LIGHT_SOURCE_H_

#include "third_party/blink/renderer/platform/graphics/filters/light_source.h"
#include "ui/gfx/geometry/point3_f.h"

namespace blink {

class PLATFORM_EXPORT PointLightSource final : public LightSource {
 public:
  static scoped_refptr<PointLightSource> Create(const gfx::Point3F& position) {
    return base::AdoptRef(new PointLightSource(position));
  }

  const gfx::Point3F& GetPosition() const { return position_; }
  bool SetPosition(const gfx::Point3F&) override;

  StringBuilder& ExternalRepresentation(StringBuilder&) const override;

 private:
  explicit PointLightSource(const gfx::Point3F& position)
      : LightSource(kLsPoint), position_(position) {}

  gfx::Point3F position_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_FILTERS_POINT_LIGHT_SOURCE_H_
