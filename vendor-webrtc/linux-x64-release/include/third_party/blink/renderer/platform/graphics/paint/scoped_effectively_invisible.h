// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_SCOPED_EFFECTIVELY_INVISIBLE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_SCOPED_EFFECTIVELY_INVISIBLE_H_

#include "third_party/blink/renderer/platform/graphics/paint/paint_controller.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class ScopedEffectivelyInvisible final {
  STACK_ALLOCATED();

 public:
  explicit ScopedEffectivelyInvisible(PaintController& paint_controller)
      : paint_controller_(paint_controller),
        previous_effectively_invisible_(
            paint_controller.CurrentEffectivelyInvisible()) {
    paint_controller.SetCurrentEffectivelyInvisible(true);
  }
  ScopedEffectivelyInvisible(const ScopedEffectivelyInvisible&) = delete;
  ScopedEffectivelyInvisible& operator=(const ScopedEffectivelyInvisible&) =
      delete;
  ~ScopedEffectivelyInvisible() {
    paint_controller_.SetCurrentEffectivelyInvisible(
        previous_effectively_invisible_);
  }

 private:
  PaintController& paint_controller_;
  bool previous_effectively_invisible_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_SCOPED_EFFECTIVELY_INVISIBLE_H_
