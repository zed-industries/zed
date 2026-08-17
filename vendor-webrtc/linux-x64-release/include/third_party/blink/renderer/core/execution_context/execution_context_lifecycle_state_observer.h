/*
 * Copyright (C) 2008 Apple Inc. All Rights Reserved.
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
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EXECUTION_CONTEXT_EXECUTION_CONTEXT_LIFECYCLE_STATE_OBSERVER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EXECUTION_CONTEXT_EXECUTION_CONTEXT_LIFECYCLE_STATE_OBSERVER_H_

#include "base/dcheck_is_on.h"
#include "third_party/blink/public/mojom/frame/lifecycle.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"

namespace blink {

// A ExecutionContextLifecycleStateObserver responds to situations where Blink
// is pausing a context or freezing a frame.
//
// Context lifecycle state changes can happen in cases such as:
// - modal dialogs during script execution (e.g. window.alert, window.print)
// - script execution stopped at a debugger breakpoint
// - frozen contexts (resource coordinator background tasks, iframe freezing)
//
// The scheduler will automatically suspend certain task queues for the duration
// that the page is paused.
//
// Objects with asynchronous activity, especially activity that may have an
// observable effect on web-visible state, on should suspend that activity while
// the page is paused by overriding ContextLifecycleStateChanged().
//
// https://html.spec.whatwg.org/C/#pause
// https://wicg.github.io/page-lifecycle/spec.html
class CORE_EXPORT ExecutionContextLifecycleStateObserver
    : public ExecutionContextLifecycleObserver {
 public:
  explicit ExecutionContextLifecycleStateObserver(ExecutionContext*);

  // UpdateStateIfNeeded() should be called exactly once after object
  // construction to synchronize the suspend state with that in
  // ExecutionContext.
  void UpdateStateIfNeeded();
#if DCHECK_IS_ON()
  bool UpdateStateIfNeededCalled() const {
    return update_state_if_needed_called_;
  }
#endif

  virtual void ContextLifecycleStateChanged(
      mojom::blink::FrameLifecycleState state) {}

  void SetExecutionContext(ExecutionContext*) override;

 protected:
  ~ExecutionContextLifecycleStateObserver() override;

 private:
#if DCHECK_IS_ON()
  bool update_state_if_needed_called_ = false;
#endif
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EXECUTION_CONTEXT_EXECUTION_CONTEXT_LIFECYCLE_STATE_OBSERVER_H_
