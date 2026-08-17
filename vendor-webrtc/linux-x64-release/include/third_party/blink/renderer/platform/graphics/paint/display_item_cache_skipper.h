// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_DISPLAY_ITEM_CACHE_SKIPPER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_DISPLAY_ITEM_CACHE_SKIPPER_H_

#include "third_party/blink/renderer/platform/graphics/graphics_context.h"
#include "third_party/blink/renderer/platform/graphics/paint/paint_controller.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class DisplayItemCacheSkipper final {
  STACK_ALLOCATED();

 public:
  explicit DisplayItemCacheSkipper(GraphicsContext& context)
      : context_(context) {
    context.GetPaintController().BeginSkippingCache();
  }
  DisplayItemCacheSkipper(const DisplayItemCacheSkipper&) = delete;
  DisplayItemCacheSkipper& operator=(const DisplayItemCacheSkipper&) = delete;
  ~DisplayItemCacheSkipper() {
    context_.GetPaintController().EndSkippingCache();
  }

 private:
  GraphicsContext& context_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_DISPLAY_ITEM_CACHE_SKIPPER_H_
