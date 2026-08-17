// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_WIDGET_SCHEDULER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_WIDGET_SCHEDULER_H_

#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/public/common/input/web_input_event.h"
#include "third_party/blink/public/platform/web_input_event_result.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/thread_safe_ref_counted.h"

namespace viz {
struct BeginFrameArgs;
}  // namespace viz

namespace blink {

class WebInputEventAttribution;

namespace scheduler {

// A new WidgetScheduler should be allocated for each Widget created. This
// class is ThreadSafeRefCounted because some of its methods can be invoked
// from the compositor thread. Each method is marked in what context it
// will be called. Destruction of the class can happen on any thread
// and thread specific shutdown code should be performed in the `Shutdown`
// method. The `Shutdown` method should be invoked on the main thread before
// the reference is released on that thread.
class PLATFORM_EXPORT WidgetScheduler
    : public ThreadSafeRefCounted<WidgetScheduler> {
 public:
  class Delegate {
   public:
    virtual ~Delegate() = default;

    // Controls whether or not BeginMainFrameNotExpected messages are sent from
    // the compositor to the `WidgetScheduler`.
    virtual void RequestBeginMainFrameNotExpected(bool new_state) = 0;
  };

  virtual ~WidgetScheduler() = default;

  // Must be called prior to `Shutdown` on the main thread. The associated
  // `Delegate` must be kept alive until this is called.
  virtual void WillShutdown() = 0;

  // Shutdown the WidgetScheduler on the main thread.
  virtual void Shutdown() = 0;

  // Returns the input task runner.
  virtual scoped_refptr<base::SingleThreadTaskRunner> InputTaskRunner() = 0;

  // Called to notify about the start of an extended period where no frames
  // need to be drawn. Must be called from the main thread.
  virtual void BeginFrameNotExpectedSoon() = 0;

  // Called to notify about the start of a period where main frames are not
  // scheduled and so short idle work can be scheduled. This will precede
  // BeginFrameNotExpectedSoon and is also called when the compositor may be
  // busy but the main thread is not.
  virtual void BeginMainFrameNotExpectedUntil(base::TimeTicks time) = 0;

  // Called to notify about the start of a new frame.  Must be called from the
  // main thread.
  virtual void WillBeginFrame(const viz::BeginFrameArgs& args) = 0;

  // Called to notify that a previously begun frame was committed. Must be
  // called from the main thread.
  virtual void DidCommitFrameToCompositor() = 0;

  // Keep InputEventStateToString() in sync with this enum.
  enum class InputEventState {
    EVENT_CONSUMED_BY_COMPOSITOR,
    EVENT_FORWARDED_TO_MAIN_THREAD,
  };

  // Tells the scheduler that the system processed an input event. Called by the
  // compositor (impl) thread.  Note it's expected that every call to
  // DidHandleInputEventOnCompositorThread where |event_state| is
  // EVENT_FORWARDED_TO_MAIN_THREAD will be followed by a corresponding call
  // to DidHandleInputEventOnMainThread.
  virtual void DidHandleInputEventOnCompositorThread(
      const WebInputEvent& web_input_event,
      InputEventState event_state) = 0;

  // Tells the scheduler that an input event of the given type is about to be
  // posted to the main thread. Must be followed later by a call to
  // WillHandleInputEventOnMainThread. Called by the compositor thread.
  virtual void WillPostInputEventToMainThread(
      WebInputEvent::Type web_input_event_type,
      const WebInputEventAttribution& web_input_event_attribution) = 0;

  // Tells the scheduler the input event of the given type is about to be
  // handled. Called on the main thread.
  virtual void WillHandleInputEventOnMainThread(
      WebInputEvent::Type web_input_event_type,
      const WebInputEventAttribution& web_input_event_attribution) = 0;

  // Tells the scheduler that the system processed an input event. Must be
  // called from the main thread.
  virtual void DidHandleInputEventOnMainThread(
      const WebInputEvent& web_input_event,
      WebInputEventResult result,
      bool frame_requested) = 0;

  // Tells the scheduler that the main thread processed a BeginMainFrame task
  // from its queue. Note that DidRunBeginMainFrame will be called
  // unconditionally, even if BeginMainFrame early-returns without committing
  // a frame.
  virtual void DidRunBeginMainFrame() = 0;

  // The Widget changed hidden state. Called from the main thread.
  virtual void SetHidden(bool hidden) = 0;
};

}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_WIDGET_SCHEDULER_H_
