/*
 * Copyright (C) 2004, 2005, 2006, 2007 Nikolas Zimmermann <zimmermann@kde.org>
 * Copyright (C) 2004, 2005 Rob Buis <buis@kde.org>
 * Copyright (C) 2005 Eric Seidel <eric@webkit.org>
 * Copyright (C) 2009 Dirk Schulze <krit@webkit.org>
 * Copyright (C) 2010 Renata Hodovan <reni@inf.u-szeged.hu>
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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_FILTERS_FE_TURBULENCE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_FILTERS_FE_TURBULENCE_H_

#include "third_party/blink/renderer/platform/graphics/filters/filter_effect.h"

namespace blink {

enum TurbulenceType {
  FETURBULENCE_TYPE_UNKNOWN = 0,
  FETURBULENCE_TYPE_FRACTALNOISE = 1,
  FETURBULENCE_TYPE_TURBULENCE = 2
};

class PLATFORM_EXPORT FETurbulence final : public FilterEffect {
 public:
  FETurbulence(Filter*, TurbulenceType, float, float, int, float, bool);

  TurbulenceType GetType() const;
  bool SetType(TurbulenceType);

  float BaseFrequencyY() const;
  bool SetBaseFrequencyY(float);

  float BaseFrequencyX() const;
  bool SetBaseFrequencyX(float);

  float Seed() const;
  bool SetSeed(float);

  int NumOctaves() const;
  bool SetNumOctaves(int);

  bool StitchTiles() const;
  bool SetStitchTiles(bool);

  StringBuilder& ExternalRepresentation(StringBuilder&,
                                        wtf_size_t indent) const override;

 private:
  sk_sp<PaintFilter> CreateImageFilter() override;

  TurbulenceType type_;
  float base_frequency_x_;
  float base_frequency_y_;
  int num_octaves_;
  float seed_;
  bool stitch_tiles_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_FILTERS_FE_TURBULENCE_H_
