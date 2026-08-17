// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_SCOPED_DISPLAY_ITEM_FRAGMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_SCOPED_DISPLAY_ITEM_FRAGMENT_H_

#include "third_party/blink/renderer/platform/graphics/graphics_context.h"
#include "third_party/blink/renderer/platform/graphics/paint/paint_controller.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class ScopedDisplayItemFragment final {
  STACK_ALLOCATED();

 public:
  ScopedDisplayItemFragment(GraphicsContext& context, wtf_size_t fragment)
      : context_(context),
        original_fragment_(context.GetPaintController().CurrentFragment()) {
    context.GetPaintController().SetCurrentFragment(fragment);
  }
  ScopedDisplayItemFragment(const ScopedDisplayItemFragment&) = delete;
  ScopedDisplayItemFragment& operator=(const ScopedDisplayItemFragment&) =
      delete;
  ~ScopedDisplayItemFragment() {
    context_.GetPaintController().SetCurrentFragment(original_fragment_);
  }

 private:
  GraphicsContext& context_;
  wtf_size_t original_fragment_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_SCOPED_DISPLAY_ITEM_FRAGMENT_H_
