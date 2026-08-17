// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_PRE_PAINT_DISABLE_SIDE_EFFECTS_SCOPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_PRE_PAINT_DISABLE_SIDE_EFFECTS_SCOPE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// A scope to prevent pre-paint from writing to LayoutObject, PaintLayer,
// FragmentData, etc. This is used when setting up the internal temporary
// pre-paint contexts based on a LayoutObject without actually walking the
// LayoutObject. This is needed before walking missed OOF descendants, so that
// the missed descendants get their FragmentData object(s) set up with the
// correct paint properties from their ancestors.
class CORE_EXPORT PrePaintDisableSideEffectsScope {
  STACK_ALLOCATED();

 public:
  PrePaintDisableSideEffectsScope() { ++count_; }
  ~PrePaintDisableSideEffectsScope() {
    DCHECK(count_);
    --count_;
  }

  static bool IsDisabled() { return count_; }

 private:
  static unsigned count_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_PRE_PAINT_DISABLE_SIDE_EFFECTS_SCOPE_H_
