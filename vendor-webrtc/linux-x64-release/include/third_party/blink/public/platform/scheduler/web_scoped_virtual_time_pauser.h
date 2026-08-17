// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_SCHEDULER_WEB_SCOPED_VIRTUAL_TIME_PAUSER_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_SCHEDULER_WEB_SCOPED_VIRTUAL_TIME_PAUSER_H_

#include "base/memory/raw_ptr.h"
#include "base/time/time.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_string.h"

namespace blink {
namespace scheduler {
class ThreadSchedulerBase;
}  // namespace scheduler

// VirtualTime is a headless feature which is intended to make renders (more)
// deterministic by pausing task execution in the Blink main thread (and pausing
// the clock) while certain asynchronous operations are pending, e.g. fetching
// resources. Generally new instances of WebScopedVirtualTimePauser should only
// be added if there are determinism problems with renders on certain pages.
// The WebScopedVirtualTimePauser itself is a move only RAII style helper which
// makes it easier for subsystems to robustly pause and unpause virtual time.
class BLINK_PLATFORM_EXPORT WebScopedVirtualTimePauser {
 public:
  enum class VirtualTaskDuration {
    kInstant,    // Virtual time will not be advanced when it's unpaused.
    kNonInstant  // Virtual time may be advanced when it's unpaused.
  };

  // Note simply creating a WebScopedVirtualTimePauser doesn't cause VirtualTime
  // to pause, instead you need to call PauseVirtualTime.
  // By default VirtualTaskDuration::kInstant should be used unless there is a
  // risk of virtual time getting stalled (e.g. if a page requests a
  // non-existent resource and it has an error handler which always fetches
  // another non-existent resource, then there is a risk that virtual time will
  // be blocked forever unless we use VirtualTaskDuration::kNonInstant).
  WebScopedVirtualTimePauser(scheduler::ThreadSchedulerBase*,
                             VirtualTaskDuration,
                             const WebString& debug_name);

  WebScopedVirtualTimePauser();
  ~WebScopedVirtualTimePauser();

  WebScopedVirtualTimePauser(WebScopedVirtualTimePauser&& other);
  WebScopedVirtualTimePauser& operator=(WebScopedVirtualTimePauser&& other);

  WebScopedVirtualTimePauser(const WebScopedVirtualTimePauser&) = delete;
  WebScopedVirtualTimePauser& operator=(const WebScopedVirtualTimePauser&) =
      delete;

  // Virtual time will be paused if any WebScopedVirtualTimePauser votes to
  // pause it, and only unpaused only if all WebScopedVirtualTimePauser are
  // either destroyed or vote to unpause.
  void PauseVirtualTime();
  void UnpauseVirtualTime();

 private:
  void DecrementVirtualTimePauseCount();

  base::TimeTicks virtual_time_when_paused_;
  bool paused_ = false;
  bool virtual_time_enabled_when_paused_ = false;
  VirtualTaskDuration duration_ = VirtualTaskDuration::kInstant;
  raw_ptr<scheduler::ThreadSchedulerBase, DanglingUntriaged>
      scheduler_;  // NOT OWNED
  WebString debug_name_;
  intptr_t trace_id_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_SCHEDULER_WEB_SCOPED_VIRTUAL_TIME_PAUSER_H_
