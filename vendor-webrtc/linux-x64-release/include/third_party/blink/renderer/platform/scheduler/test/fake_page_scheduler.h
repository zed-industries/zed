// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_TEST_FAKE_PAGE_SCHEDULER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_TEST_FAKE_PAGE_SCHEDULER_H_

#include "third_party/blink/public/platform/scheduler/web_agent_group_scheduler.h"
#include "third_party/blink/renderer/platform/scheduler/public/dummy_schedulers.h"
#include "third_party/blink/renderer/platform/scheduler/public/page_scheduler.h"
#include "third_party/blink/renderer/platform/scheduler/public/widget_scheduler.h"

namespace blink {
namespace scheduler {

class FakePageScheduler : public PageScheduler {
 public:
  FakePageScheduler(bool is_audio_playing, bool is_throttling_exempt)
      : is_audio_playing_(is_audio_playing),
        is_throttling_exempt_(is_throttling_exempt),
        agent_group_scheduler_(CreateDummyAgentGroupScheduler()) {}
  FakePageScheduler(const FakePageScheduler&) = delete;
  FakePageScheduler& operator=(const FakePageScheduler&) = delete;

  class Builder {
   public:
    Builder() = default;
    Builder(const Builder&) = delete;
    Builder& operator=(const Builder&) = delete;

    Builder& SetIsAudioPlaying(bool is_audio_playing) {
      is_audio_playing_ = is_audio_playing;
      return *this;
    }

    Builder& SetIsThrottlingExempt(bool is_throttling_exempt) {
      is_throttling_exempt_ = is_throttling_exempt;
      return *this;
    }

    std::unique_ptr<FakePageScheduler> Build() {
      return std::make_unique<FakePageScheduler>(is_audio_playing_,
                                                 is_throttling_exempt_);
    }

   private:
    bool is_audio_playing_ = false;
    bool is_throttling_exempt_ = false;
  };

  bool IsAudioPlaying() const override { return is_audio_playing_; }

  bool IsExemptFromBudgetBasedThrottling() const override {
    return is_throttling_exempt_;
  }

  // PageScheduler implementation:
  void OnTitleOrFaviconUpdated() override {}
  void SetPageVisible(bool is_page_visible) override {}
  bool IsPageVisible() const override { return true; }
  void SetPageFrozen(bool is_page_frozen) override {}
  void SetPageBackForwardCached(bool) override {}
  bool IsMainFrameLocal() const override { return true; }
  void SetIsMainFrameLocal(bool is_local) override {}

  std::unique_ptr<FrameScheduler> CreateFrameScheduler(
      FrameScheduler::Delegate* delegate,
      bool is_in_embedded_frame_tree,
      FrameScheduler::FrameType frame_type) override {
    return nullptr;
  }
  void AudioStateChanged(bool is_audio_playing) override {}
  bool OptedOutFromAggressiveThrottlingForTest() const override {
    return false;
  }
  bool RequestBeginMainFrameNotExpected(bool new_state) override {
    return false;
  }
  AgentGroupScheduler& GetAgentGroupScheduler() override {
    return *agent_group_scheduler_;
  }
  VirtualTimeController* GetVirtualTimeController() override { return nullptr; }
  bool IsInBackForwardCache() const override { return false; }

  scoped_refptr<WidgetScheduler> CreateWidgetScheduler(
      WidgetScheduler::Delegate*) override {
    return nullptr;
  }

 private:
  bool is_audio_playing_;
  bool is_throttling_exempt_;
  Persistent<AgentGroupScheduler> agent_group_scheduler_;
};

}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_TEST_FAKE_PAGE_SCHEDULER_H_
