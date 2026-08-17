// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_SCOPED_ABORT_STATE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_SCOPED_ABORT_STATE_H_

#include "base/check.h"
#include "third_party/blink/renderer/core/dom/abort_signal.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// Helper class that removes an abort algorithm from its associated signal when
// the handle is destroyed. This is useful for passing abort state around to
// callbacks or when an abort algorithm should be immediately removed on
// completion, e.g. where subsequent operations might use a different signal.
//
// This keeps both the algorithm handle and signal alive, the latter because
// some consumers need access to signal state after the abort algorithm runs, at
// which point we can't guarantee the signal is alive.
class ScopedAbortState {
  USING_FAST_MALLOC(ScopedAbortState);

 public:
  ScopedAbortState(AbortSignal* signal, AbortSignal::AlgorithmHandle* handle)
      : signal_(signal), abort_handle_(handle) {
    DCHECK(signal_);
    DCHECK(abort_handle_);
  }

  ScopedAbortState(const ScopedAbortState&) = delete;
  ScopedAbortState& operator=(const ScopedAbortState&) = delete;

  ~ScopedAbortState() { signal_->RemoveAlgorithm(abort_handle_); }

  AbortSignal* Signal() { return signal_; }

 private:
  Persistent<AbortSignal> signal_;
  Persistent<AbortSignal::AlgorithmHandle> abort_handle_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_SCOPED_ABORT_STATE_H_
