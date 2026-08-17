// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file contains the Windows-specific exporting to ETW.
#ifndef BASE_TRACE_EVENT_ETW_INTERCEPTOR_WIN_H_
#define BASE_TRACE_EVENT_ETW_INTERCEPTOR_WIN_H_

#include <stdint.h>

#include "base/base_export.h"
#include "base/trace_event/trace_logging_minimal_win.h"
#include "third_party/perfetto/include/perfetto/tracing/interceptor.h"
#include "third_party/perfetto/include/perfetto/tracing/track_event_state_tracker.h"
#include "third_party/perfetto/protos/perfetto/config/track_event/track_event_config.gen.h"  // nogncheck

namespace base::trace_event {

// Receives perfetto tracing packets and emits ETW events through TlmProvider.
class BASE_EXPORT ETWInterceptor
    : public perfetto::Interceptor<ETWInterceptor> {
 public:
  static void OnTracePacket(InterceptorContext context);

  explicit ETWInterceptor(TlmProvider* provider);
  ~ETWInterceptor() override;

  static void Register(TlmProvider* provider);

  void OnSetup(const SetupArgs&) override;
  void OnStart(const StartArgs&) override;
  void OnStop(const StopArgs&) override;

  struct ThreadLocalState : public InterceptorBase::ThreadLocalState {
    explicit ThreadLocalState(ThreadLocalStateArgs&);
    ~ThreadLocalState() override;

    // We only support a single trace writer sequence per thread, so the
    // sequence state is stored in TLS.
    perfetto::TrackEventStateTracker::SequenceState sequence_state;
  };

 private:
  class Delegate;

  perfetto::TrackEventStateTracker::SessionState session_state_;
  raw_ptr<TlmProvider> provider_;
};

BASE_EXPORT perfetto::protos::gen::TrackEventConfig
ETWKeywordToTrackEventConfig(uint64_t keyword);

}  // namespace base::trace_event

#endif  // BASE_TRACE_EVENT_ETW_INTERCEPTOR_WIN_H_
