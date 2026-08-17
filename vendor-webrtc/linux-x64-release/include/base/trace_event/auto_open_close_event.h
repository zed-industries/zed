// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TRACE_EVENT_AUTO_OPEN_CLOSE_EVENT_H_
#define BASE_TRACE_EVENT_AUTO_OPEN_CLOSE_EVENT_H_

#include "base/check.h"
#include "base/memory/weak_ptr.h"
#include "base/threading/thread_checker.h"
#include "base/time/time.h"
#include "base/trace_event/trace_event.h"

namespace base {
namespace trace_event {

// Class for tracing events that support "auto-opening" and "auto-closing".
// "auto-opening" = if the trace event is started (call Begin() before
// tracing is started,the trace event will be opened, with the start time
// being the time that the trace event was actually started.
// "auto-closing" = if the trace event is started but not ended by the time
// tracing ends, then the trace event will be automatically closed at the
// end of tracing.
// |category| must be known at compile-time in order to be used in trace macros.
// Hence, it's passed as a class templace argument.
template <const char* category>
class AutoOpenCloseEvent : public TraceLog::AsyncEnabledStateObserver {
 public:
  enum Type { kAsync };

  // As in the rest of the tracing macros, the const char* arguments here
  // must be pointers to indefinitely lived strings (e.g. hard-coded string
  // literals are okay, but not strings created by c_str())
  AutoOpenCloseEvent(Type type, const char* event_name)
      : event_name_(event_name) {
    base::trace_event::TraceLog::GetInstance()->AddAsyncEnabledStateObserver(
        weak_factory_.GetWeakPtr());
  }
  AutoOpenCloseEvent(const AutoOpenCloseEvent&) = delete;
  AutoOpenCloseEvent& operator=(const AutoOpenCloseEvent&) = delete;
  ~AutoOpenCloseEvent() override {
    DCHECK(thread_checker_.CalledOnValidThread());
    base::trace_event::TraceLog::GetInstance()->RemoveAsyncEnabledStateObserver(
        this);
  }

  void Begin() {
    DCHECK(thread_checker_.CalledOnValidThread());
    start_time_ = TRACE_TIME_TICKS_NOW();
    TRACE_EVENT_ASYNC_BEGIN_WITH_TIMESTAMP0(
        category, event_name_, static_cast<void*>(this), start_time_);
  }
  void End() {
    DCHECK(thread_checker_.CalledOnValidThread());
    TRACE_EVENT_ASYNC_END0(category, event_name_, static_cast<void*>(this));
    start_time_ = base::TimeTicks();
  }

  // AsyncEnabledStateObserver implementation
  void OnTraceLogEnabled() override {
    DCHECK(thread_checker_.CalledOnValidThread());
    if (!start_time_.is_null()) {
      TRACE_EVENT_ASYNC_BEGIN_WITH_TIMESTAMP0(
          category, event_name_, static_cast<void*>(this), start_time_);
    }
  }
  void OnTraceLogDisabled() override {}

 private:
  const char* const event_name_;
  base::TimeTicks start_time_;
  base::ThreadChecker thread_checker_;
  WeakPtrFactory<AutoOpenCloseEvent> weak_factory_{this};
};

}  // namespace trace_event
}  // namespace base

#endif  // BASE_TRACE_EVENT_AUTO_OPEN_CLOSE_EVENT_H_
