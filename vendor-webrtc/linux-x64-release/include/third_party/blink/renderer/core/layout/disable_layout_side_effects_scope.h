// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_DISABLE_LAYOUT_SIDE_EFFECTS_SCOPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_DISABLE_LAYOUT_SIDE_EFFECTS_SCOPE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// Setup a scope where this layout is for computing purposes, and that it should
// just return fragments without updating the |LayoutObject| tree, paint
// properties, and other global objects. This is used e.g., when computing
// MinMax after layout.
class CORE_EXPORT DisableLayoutSideEffectsScope {
  STACK_ALLOCATED();

 public:
  DisableLayoutSideEffectsScope() { ++count_; }
  ~DisableLayoutSideEffectsScope() {
    DCHECK(count_);
    --count_;
  }

  static bool IsDisabled() { return count_; }

 private:
  static unsigned count_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_DISABLE_LAYOUT_SIDE_EFFECTS_SCOPE_H_
