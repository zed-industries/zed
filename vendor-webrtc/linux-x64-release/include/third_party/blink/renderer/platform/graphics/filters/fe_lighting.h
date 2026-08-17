/*
 * Copyright (C) 2010 University of Szeged
 * Copyright (C) 2010 Zoltan Herczeg
 * Copyright (C) 2013 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY UNIVERSITY OF SZEGED ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL UNIVERSITY OF SZEGED OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_FILTERS_FE_LIGHTING_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_FILTERS_FE_LIGHTING_H_

#include "third_party/blink/renderer/platform/graphics/color.h"
#include "third_party/blink/renderer/platform/graphics/filters/filter_effect.h"

// Common base class for FEDiffuseLighting and FESpecularLighting

namespace blink {

class LightSource;

class PLATFORM_EXPORT FELighting : public FilterEffect {
 public:
  Color LightingColor() const;
  bool SetLightingColor(const Color&);

  float SurfaceScale() const;
  bool SetSurfaceScale(float);

  LightSource* GetLightSource() { return light_source_.get(); }
  const LightSource* GetLightSource() const { return light_source_.get(); }

 protected:
  enum LightingType { kDiffuseLighting, kSpecularLighting };

  sk_sp<PaintFilter> CreateImageFilter() override;

  bool AffectsTransparentPixels() const override { return true; }

  FELighting(Filter*,
             LightingType,
             const Color&,
             float,
             float,
             float,
             float,
             scoped_refptr<LightSource>);

  PaintFilter::LightingType GetLightingType();
  float GetFilterConstant();

  LightingType lighting_type_;
  scoped_refptr<LightSource> light_source_;

  Color lighting_color_;
  float surface_scale_;
  float diffuse_constant_;
  float specular_constant_;
  float specular_exponent_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_FILTERS_FE_LIGHTING_H_
