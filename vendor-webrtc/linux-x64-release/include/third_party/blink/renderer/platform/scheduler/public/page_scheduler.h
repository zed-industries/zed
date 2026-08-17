// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_PAGE_SCHEDULER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_PAGE_SCHEDULER_H_

#include <memory>

#include "third_party/blink/public/platform/scheduler/web_scoped_virtual_time_pauser.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/scheduler/public/frame_scheduler.h"
#include "third_party/blink/renderer/platform/scheduler/public/scheduling_policy.h"
#include "third_party/blink/renderer/platform/scheduler/public/virtual_time_controller.h"
#include "third_party/blink/renderer/platform/scheduler/public/widget_scheduler.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class AgentGroupScheduler;

class PLATFORM_EXPORT PageScheduler {
 public:
  class PLATFORM_EXPORT Delegate {
   public:
    virtual ~Delegate() = default;

    // An "ordinary" page is a fully-featured page owned by a web view.
    virtual bool IsOrdinary() const = 0;
    // Returns true if the request has been succcessfully relayed to the
    // compositor.
    virtual bool RequestBeginMainFrameNotExpected(bool new_state) = 0;
    virtual void OnSetPageFrozen(bool is_frozen) = 0;
  };

  virtual ~PageScheduler() = default;

  // Signals that communications with the user took place via either a title
  // updates or a change to the favicon.
  virtual void OnTitleOrFaviconUpdated() = 0;
  // The scheduler may throttle tasks associated with background pages.
  virtual void SetPageVisible(bool) = 0;
  // Return whether the page is visible or not.  Note that learning
  // `!IsPageVisible()` does not tell you if the page should continue to paint
  // or not.  There are two hidden states, `kHidden` and `kHiddenButPainting`
  // which both correspond to a page that's not visible to the user.  The latter
  // indicates that the page's content is still meaningful in some other way,
  // such as if it's being captured.
  // TODO(https://crbug.com/1495854): add `IsPagePainting()` to make it easy to
  // tell the difference.
  virtual bool IsPageVisible() const = 0;
  // The scheduler transitions app to and from FROZEN state in background.
  virtual void SetPageFrozen(bool) = 0;
  // Handles operations required for storing the page in the back-forward cache.
  virtual void SetPageBackForwardCached(bool) = 0;
  // Whether the main frame of this page is local or not (remote).
  virtual bool IsMainFrameLocal() const = 0;
  virtual void SetIsMainFrameLocal(bool) = 0;
  // Whether the main frame of this page is in BackForwardCache or not.
  virtual bool IsInBackForwardCache() const = 0;

  // Creates a new FrameScheduler. The caller is responsible for deleting
  // it.
  virtual std::unique_ptr<FrameScheduler> CreateFrameScheduler(
      FrameScheduler::Delegate* delegate,
      bool is_in_embedded_frame_tree,
      FrameScheduler::FrameType) = 0;

  virtual void AudioStateChanged(bool is_audio_playing) = 0;

  virtual bool IsAudioPlaying() const = 0;

  // Returns true if the page should be exempted from aggressive throttling
  // (e.g. due to a page maintaining an active connection).
  virtual bool IsExemptFromBudgetBasedThrottling() const = 0;

  virtual bool OptedOutFromAggressiveThrottlingForTest() const = 0;

  // Returns true if the request has been succcessfully relayed to the
  // compositor.
  virtual bool RequestBeginMainFrameNotExpected(bool new_state) = 0;

  // Returns AgentGroupScheduler
  virtual AgentGroupScheduler& GetAgentGroupScheduler() = 0;

  // Guaranteed to be non-null for real PageScheduler implementation, but may
  // be null in unit tests.
  virtual VirtualTimeController* GetVirtualTimeController() = 0;

  // Creates a WidgetScheduler implementation. The delegate must remain alive
  // until `scheduler::WidgetScheduler::WillShutdown()` is called.
  virtual scoped_refptr<scheduler::WidgetScheduler> CreateWidgetScheduler(
      scheduler::WidgetScheduler::Delegate*) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_PAGE_SCHEDULER_H_
