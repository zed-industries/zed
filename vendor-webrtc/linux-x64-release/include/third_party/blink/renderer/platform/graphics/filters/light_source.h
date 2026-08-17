/*
 * Copyright (C) 2008 Alex Mathews <possessedpenguinbob@gmail.com>
 * Copyright (C) 2004, 2005, 2006, 2007 Nikolas Zimmermann <zimmermann@kde.org>
 * Copyright (C) 2004, 2005 Rob Buis <buis@kde.org>
 * Copyright (C) 2005 Eric Seidel <eric@webkit.org>
 * Copyright (C) 2010 Zoltan Herczeg <zherczeg@webkit.org>
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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_FILTERS_LIGHT_SOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_FILTERS_LIGHT_SOURCE_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/ref_counted.h"

namespace gfx {
class Point3F;
}

namespace blink {

enum LightType { kLsDistant, kLsPoint, kLsSpot };

class PLATFORM_EXPORT LightSource : public RefCounted<LightSource> {

 public:
  LightSource(LightType type) : type_(type) {}
  LightSource(const LightSource&) = delete;
  LightSource& operator=(const LightSource&) = delete;

  virtual ~LightSource();

  LightType GetType() const { return type_; }
  virtual StringBuilder& ExternalRepresentation(StringBuilder&) const = 0;

  virtual bool SetAzimuth(float) { return false; }
  virtual bool SetElevation(float) { return false; }
  virtual bool SetPosition(const gfx::Point3F&) { return false; }
  virtual bool SetPointsAt(const gfx::Point3F&) { return false; }
  virtual bool SetSpecularExponent(float) { return false; }
  virtual bool SetLimitingConeAngle(float) { return false; }

 private:
  LightType type_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_FILTERS_LIGHT_SOURCE_H_
