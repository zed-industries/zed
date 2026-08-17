// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_SCHEDULER_WEB_THREAD_SCHEDULER_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_SCHEDULER_WEB_THREAD_SCHEDULER_H_

#include <memory>

#include "base/message_loop/message_pump.h"
#include "base/task/single_thread_task_runner.h"
#include "base/time/time.h"
#include "build/build_config.h"
#include "ipc/urgent_message_observer.h"
#include "third_party/blink/public/platform/scheduler/web_agent_group_scheduler.h"
#include "third_party/blink/public/platform/web_common.h"

namespace blink {
class MainThread;
}  // namespace blink

namespace blink {
namespace scheduler {

enum class WebRendererProcessType;

class BLINK_PLATFORM_EXPORT WebThreadScheduler
    : public IPC::UrgentMessageObserver {
 public:
  WebThreadScheduler(const WebThreadScheduler&) = delete;
  WebThreadScheduler& operator=(const WebThreadScheduler&) = delete;
  ~WebThreadScheduler() override;

  // ==== Functions for the main thread scheduler  ============================
  //
  // Virtual functions below should only be called against the scheduler on
  // the main thread. They have default implementation that only does
  // NOTREACHED(), and are overridden only by the main thread scheduler.

  // Shuts down the scheduler by dropping any remaining pending work in the work
  // queues. After this call any work posted to the task runners will be
  // silently dropped.
  virtual void Shutdown() = 0;

  // If |message_pump| is null caller must have registered one using
  // base::MessageLoop.
  static std::unique_ptr<WebThreadScheduler> CreateMainThreadScheduler(
      std::unique_ptr<base::MessagePump> message_pump = nullptr);

  // Returns main thread scheduler for the main thread of the current process.
  static WebThreadScheduler& MainThreadScheduler();

  // Returns a default task runner. This is basically same as the default task
  // runner, but is explicitly allowed to run JavaScript. For the detail, see
  // the comment at blink::ThreadScheduler::DeprecatedDefaultTaskRunner.
  virtual scoped_refptr<base::SingleThreadTaskRunner>
  DeprecatedDefaultTaskRunner();

  // Creates a WebThread implementation for the renderer main thread.
  virtual std::unique_ptr<MainThread> CreateMainThread();

  // Creates a WebAgentGroupScheduler implementation. Must be called from the
  // main thread.
  virtual std::unique_ptr<WebAgentGroupScheduler>
  CreateWebAgentGroupScheduler() = 0;

  // Tells the scheduler about the change of renderer visibility status (e.g.
  // "all widgets are hidden" condition). Used mostly for metric purposes.
  // Must be called on the main thread.
  virtual void SetRendererHidden(bool hidden);

  // Tells the scheduler about the change of renderer background status, i.e.,
  // there are no critical, user facing activities (visual, audio, etc...)
  // driven by this process. A stricter condition than |SetRendererHidden()|,
  // the process is assumed to be foregrounded when the scheduler is
  // constructed. Must be called on the main thread.
  virtual void SetRendererBackgrounded(bool backgrounded);

#if BUILDFLAG(IS_ANDROID)
  // Android WebView has very strange WebView.pauseTimers/resumeTimers API.
  // It's very old and very inconsistent. The API promises that this
  // "pauses all layout, parsing, and JavaScript timers for all WebViews".
  // Also CTS tests expect that loading tasks continue to run.
  // We should change it to something consistent (e.g. stop all javascript)
  // but changing WebView and CTS is a slow and painful process, so for
  // the time being we're doing our best.
  // DO NOT USE FOR ANYTHING EXCEPT ANDROID WEBVIEW API IMPLEMENTATION.
  virtual void PauseTimersForAndroidWebView();
  virtual void ResumeTimersForAndroidWebView();
#endif  // BUILDFLAG(IS_ANDROID)

  // IPC::Channel::UrgentMessageDelegate implementation:
  void OnUrgentMessageReceived() override;
  void OnUrgentMessageProcessed() override;

 protected:
  WebThreadScheduler() = default;
};

}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_SCHEDULER_WEB_THREAD_SCHEDULER_H_
