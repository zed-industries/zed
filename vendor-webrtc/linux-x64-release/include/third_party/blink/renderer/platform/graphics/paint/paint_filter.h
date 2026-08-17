// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_PAINT_FILTER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_PAINT_FILTER_H_

#include "cc/paint/paint_filter.h"  // IWYU pragma: export

namespace blink {
using cc::AlphaThresholdPaintFilter;
using cc::ArithmeticPaintFilter;
using cc::BlurPaintFilter;
using cc::ColorFilterPaintFilter;
using cc::ComposePaintFilter;
using cc::DisplacementMapEffectPaintFilter;
using cc::DropShadowPaintFilter;
using cc::ImagePaintFilter;
using cc::LightingDistantPaintFilter;
using cc::LightingPointPaintFilter;
using cc::LightingSpotPaintFilter;
using cc::MagnifierPaintFilter;
using cc::MatrixConvolutionPaintFilter;
using cc::MatrixPaintFilter;
using cc::MergePaintFilter;
using cc::MorphologyPaintFilter;
using cc::OffsetPaintFilter;
using cc::PaintFilter;
using cc::RecordPaintFilter;
using cc::ShaderPaintFilter;
using cc::TilePaintFilter;
using cc::TurbulencePaintFilter;
using cc::XfermodePaintFilter;
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_PAINT_FILTER_H_
