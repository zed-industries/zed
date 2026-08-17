// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_SCOPED_IMAGE_RENDERING_SETTINGS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_SCOPED_IMAGE_RENDERING_SETTINGS_H_

#include "third_party/blink/renderer/platform/graphics/graphics_context.h"
#include "third_party/blink/renderer/platform/graphics/graphics_context_types.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// Helper to update the image rendering settings of a GraphicsContext within the
// current scope. Be careful when mixing with other GraphicsContext mechanisms
// that save/restore state (like GraphicsContextStateSaver or the Save/Restore
// methods) to ensure the restoration behavior is the expected one.
class ScopedImageRenderingSettings {
  STACK_ALLOCATED();

 public:
  ScopedImageRenderingSettings(GraphicsContext& context,
                               InterpolationQuality interpolation_quality,
                               DynamicRangeLimit dynamic_range_limit)
      : context_(context),
        previous_interpolation_quality_(context.ImageInterpolationQuality()),
        previous_dynamic_range_limit_(context.DynamicRangeLimit()) {
    if (previous_interpolation_quality_ != interpolation_quality) {
      context_.SetImageInterpolationQuality(interpolation_quality);
    }
    if (previous_dynamic_range_limit_ != dynamic_range_limit) {
      context_.SetDynamicRangeLimit(dynamic_range_limit);
    }
  }

  ~ScopedImageRenderingSettings() {
    if (previous_interpolation_quality_ !=
        context_.ImageInterpolationQuality()) {
      context_.SetImageInterpolationQuality(previous_interpolation_quality_);
    }
    if (previous_dynamic_range_limit_ != context_.DynamicRangeLimit()) {
      context_.SetDynamicRangeLimit(previous_dynamic_range_limit_);
    }
  }

 private:
  GraphicsContext& context_;
  const InterpolationQuality previous_interpolation_quality_;
  const DynamicRangeLimit previous_dynamic_range_limit_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_SCOPED_IMAGE_RENDERING_SETTINGS_H_
