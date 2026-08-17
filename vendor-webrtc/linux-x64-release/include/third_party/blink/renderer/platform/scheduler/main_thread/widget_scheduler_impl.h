// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_WIDGET_SCHEDULER_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_WIDGET_SCHEDULER_IMPL_H_

#include <memory>

#include "base/memory/raw_ptr.h"
#include "base/task/sequence_manager/task_queue.h"
#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/scheduler/public/widget_scheduler.h"

namespace blink::scheduler {

class MainThreadSchedulerImpl;
class MainThreadTaskQueue;
class RenderWidgetSignals;

class PLATFORM_EXPORT WidgetSchedulerImpl : public WidgetScheduler {
 public:
  WidgetSchedulerImpl(MainThreadSchedulerImpl*,
                      RenderWidgetSignals*,
                      Delegate*);
  WidgetSchedulerImpl(const WidgetSchedulerImpl&) = delete;
  WidgetSchedulerImpl& operator=(const WidgetSchedulerImpl&) = delete;
  ~WidgetSchedulerImpl() override;

  // WidgetScheduler overrides:
  void WillShutdown() override;
  void Shutdown() override;
  scoped_refptr<base::SingleThreadTaskRunner> InputTaskRunner() override;
  void WillBeginFrame(const viz::BeginFrameArgs& args) override;
  void BeginFrameNotExpectedSoon() override;
  void BeginMainFrameNotExpectedUntil(base::TimeTicks time) override;
  void DidCommitFrameToCompositor() override;
  void DidHandleInputEventOnCompositorThread(
      const WebInputEvent& web_input_event,
      InputEventState event_state) override;
  void WillPostInputEventToMainThread(
      WebInputEvent::Type web_input_event_type,
      const WebInputEventAttribution& web_input_event_attribution) override;
  void WillHandleInputEventOnMainThread(
      WebInputEvent::Type web_input_event_type,
      const WebInputEventAttribution& web_input_event_attribution) override;
  void DidHandleInputEventOnMainThread(const WebInputEvent& web_input_event,
                                       WebInputEventResult result,
                                       bool frame_requested) override;
  void DidRunBeginMainFrame() override;
  void SetHidden(bool hidden) override;

  void RequestBeginMainFrameNotExpected(bool);

  // Returns true if we know begin frame is not expected soon, and false if we
  // don't know (signals haven't been requested) or we are expecting a frame
  // soon.
  bool IsBeginFrameNotExpectedSoon() const {
    return begin_frame_not_expected_soon_;
  }

 private:
  scoped_refptr<MainThreadTaskQueue> input_task_queue_;
  scoped_refptr<base::SingleThreadTaskRunner> input_task_runner_;
  std::unique_ptr<base::sequence_manager::TaskQueue::QueueEnabledVoter>
      input_task_queue_enabled_voter_;

  const raw_ptr<MainThreadSchedulerImpl, DanglingUntriaged>
      main_thread_scheduler_;
  const raw_ptr<RenderWidgetSignals, DanglingUntriaged> render_widget_signals_;
  raw_ptr<Delegate> delegate_;
  bool hidden_ = false;
  bool begin_frame_not_expected_soon_ = false;
};

}  // namespace blink::scheduler

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_WIDGET_SCHEDULER_IMPL_H_
