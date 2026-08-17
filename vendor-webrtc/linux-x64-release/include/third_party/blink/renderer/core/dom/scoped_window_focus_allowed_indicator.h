// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_SCOPED_WINDOW_FOCUS_ALLOWED_INDICATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_SCOPED_WINDOW_FOCUS_ALLOWED_INDICATOR_H_

#include "third_party/blink/renderer/core/execution_context/execution_context.h"

namespace blink {

class ScopedWindowFocusAllowedIndicator final {
  USING_FAST_MALLOC(ScopedWindowFocusAllowedIndicator);

 public:
  explicit ScopedWindowFocusAllowedIndicator(
      ExecutionContext* execution_context)
      : execution_context_(execution_context) {
    execution_context->AllowWindowInteraction();
  }
  ScopedWindowFocusAllowedIndicator(const ScopedWindowFocusAllowedIndicator&) =
      delete;
  ScopedWindowFocusAllowedIndicator& operator=(
      const ScopedWindowFocusAllowedIndicator&) = delete;
  ~ScopedWindowFocusAllowedIndicator() {
    execution_context_->ConsumeWindowInteraction();
  }

 private:
  // This doesn't create a cycle because ScopedWindowFocusAllowedIndicator
  // is used only on a machine stack.
  Persistent<ExecutionContext> execution_context_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_SCOPED_WINDOW_FOCUS_ALLOWED_INDICATOR_H_
