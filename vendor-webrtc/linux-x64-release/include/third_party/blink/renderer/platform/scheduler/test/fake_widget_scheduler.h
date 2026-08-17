// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_TEST_FAKE_WIDGET_SCHEDULER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_TEST_FAKE_WIDGET_SCHEDULER_H_

#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/renderer/platform/scheduler/public/widget_scheduler.h"
#include "third_party/blink/renderer/platform/scheduler/test/fake_task_runner.h"

namespace blink::scheduler {

class FakeWidgetScheduler : public WidgetScheduler {
 public:
  FakeWidgetScheduler() {
    input_task_runner_ = base::MakeRefCounted<FakeTaskRunner>();
  }
  FakeWidgetScheduler(const FakeWidgetScheduler&) = delete;
  FakeWidgetScheduler& operator=(const FakeWidgetScheduler&) = delete;
  ~FakeWidgetScheduler() override;

  void Shutdown() override;

  // Returns the input task runner.
  scoped_refptr<base::SingleThreadTaskRunner> InputTaskRunner() override {
    return input_task_runner_;
  }
  void WillBeginFrame(const viz::BeginFrameArgs& args) override {}
  void BeginFrameNotExpectedSoon() override {}
  void BeginMainFrameNotExpectedUntil(base::TimeTicks time) override {}
  void DidCommitFrameToCompositor() override {}
  void DidHandleInputEventOnCompositorThread(
      const WebInputEvent& web_input_event,
      InputEventState event_state) override {}
  void WillPostInputEventToMainThread(
      WebInputEvent::Type web_input_event_type,
      const WebInputEventAttribution& web_input_event_attribution) override {}
  void WillHandleInputEventOnMainThread(
      WebInputEvent::Type web_input_event_type,
      const WebInputEventAttribution& web_input_event_attribution) override {}
  void DidHandleInputEventOnMainThread(const WebInputEvent& web_input_event,
                                       WebInputEventResult result,
                                       bool frame_requested) override {}
  void DidRunBeginMainFrame() override {}
  void SetHidden(bool hidden) override {}
  void WillShutdown() override {}

 private:
  scoped_refptr<FakeTaskRunner> input_task_runner_;
};

}  // namespace blink::scheduler

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_TEST_FAKE_WIDGET_SCHEDULER_H_
