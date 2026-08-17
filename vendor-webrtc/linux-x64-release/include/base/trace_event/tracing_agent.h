// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TRACE_EVENT_TRACING_AGENT_H_
#define BASE_TRACE_EVENT_TRACING_AGENT_H_

#include "base/base_export.h"
#include "base/functional/callback.h"
#include "base/memory/ref_counted_memory.h"

namespace base {

class TimeTicks;

namespace trace_event {

class TraceConfig;

// A tracing agent is an entity that records its own sort of trace. Each
// tracing method that produces its own trace log should implement this
// interface. All tracing agents must only be controlled by TracingController.
// Some existing examples include TracingControllerImpl for Chrome trace events,
// DebugDaemonClient for CrOs system trace, and EtwTracingAgent for Windows
// system.
class BASE_EXPORT TracingAgent {
 public:
  using StartAgentTracingCallback =
      base::OnceCallback<void(const std::string& agent_name, bool success)>;
  // Passing a null or empty events_str_ptr indicates that no trace data is
  // available for the specified agent.
  using StopAgentTracingCallback = base::OnceCallback<void(
      const std::string& agent_name,
      const std::string& events_label,
      const scoped_refptr<base::RefCountedString>& events_str_ptr)>;
  using RecordClockSyncMarkerCallback =
      base::OnceCallback<void(const std::string& sync_id,
                              const TimeTicks& issue_ts,
                              const TimeTicks& issue_end_ts)>;

  virtual ~TracingAgent();

  // Gets the name of the tracing agent. Each tracing agent's name should be
  // unique.
  virtual std::string GetTracingAgentName() = 0;

  // Gets the trace event label of this tracing agent. The label will be used to
  // label this agent's trace when all traces from different tracing agents are
  // combined. Multiple tracing agents could have the same label. The tracing
  // agents using the same label should not be able to run at the same time. For
  // example, ETW on Windows and CrOS system tracing both use
  // "systemTraceEvents" as the label. Those two agents never run at the same
  // time because they are for different platforms.
  virtual std::string GetTraceEventLabel() = 0;

  // Starts tracing on the tracing agent with the trace configuration.
  virtual void StartAgentTracing(const TraceConfig& trace_config,
                                 StartAgentTracingCallback callback) = 0;

  // Stops tracing on the tracing agent. The trace data will be passed back to
  // the TracingController via the callback.
  virtual void StopAgentTracing(StopAgentTracingCallback callback) = 0;

  // Checks if the tracing agent supports explicit clock synchronization.
  virtual bool SupportsExplicitClockSync();

  // Records a clock sync marker issued by another tracing agent. This is only
  // used if the tracing agent supports explicit clock synchronization.
  //
  // Two things need to be done:
  // 1. The issuer asks the receiver to record the clock sync marker.
  // 2. The issuer records how long the receiver takes to do the recording.
  //
  // In Chrome, the receiver thread also runs in Chrome and it will talk to the
  // real receiver entity, e.g., power monitor or Android device system, via
  // different communication methods, e.g., through USB or file reading/writing.
  // The 2nd task measures that communication latency.
  //
  // Having a reliable timing measurement for the 2nd task requires synchronous
  // function call without any cross-thread or cross-process activity. However,
  // tracing agents in Chrome run in their own threads. Therefore, the issuer
  // needs to dedicate the 2nd task to the receiver to take time measurements
  // in the receiver thread, and the receiver thread needs to pass them back to
  // the issuer in the callback.
  //
  // The assumption is that the receiver thread knows the issuer's clock, which
  // is true in Chrome because all agent threads' clocks are Chrome clock.
  virtual void RecordClockSyncMarker(const std::string& sync_id,
                                     RecordClockSyncMarkerCallback callback);
};

}  // namespace trace_event
}  // namespace base

#endif  // BASE_TRACE_EVENT_TRACING_AGENT_H_
