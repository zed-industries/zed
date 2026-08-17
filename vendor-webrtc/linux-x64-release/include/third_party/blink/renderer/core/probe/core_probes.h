/*
 * Copyright (C) 2010 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PROBE_CORE_PROBES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PROBE_CORE_PROBES_H_

#include "third_party/blink/renderer/bindings/core/v8/v8_binding_for_core.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/core/frame/ad_tracker.h"
#include "third_party/blink/renderer/core/frame/local_frame.h"
#include "third_party/blink/renderer/platform/bindings/callback_function_base.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource.h"

namespace network {
namespace mojom {
namespace blink {
class WebSocketHandshakeResponse;
class WebSocketHandshakeRequest;
}  // namespace blink
}  // namespace mojom
}  // namespace network

namespace blink {

class CoreProbeSink;
class OffscreenCanvas;
class ThreadDebugger;

namespace protocol {
namespace Network {
class DirectTCPSocketOptions;
}  // namespace Network
namespace Audits {
class InspectorIssue;
}  // namespace Audits
}  // namespace protocol

namespace probe {

class AsyncTaskContext;

class CORE_EXPORT ProbeBase {
  STACK_ALLOCATED();

 public:
  base::TimeTicks CaptureStartTime() const;
  base::TimeTicks CaptureEndTime() const;
  base::TimeDelta Duration() const;

 private:
  mutable base::TimeTicks start_time_;
  mutable base::TimeTicks end_time_;
};

// Tracks execution of a (previously scheduled) asynchronous task. An instance
// should exist for the full duration of the task's execution.
class CORE_EXPORT AsyncTask {
  STACK_ALLOCATED();

 public:
  // Represents how this AsyncTask should be reported to the AdTracker.
  enum class AdTrackingType {
    // Don't report this task to the ad tracker.
    kIgnore,
    // Causes all scripts and tasks executed within this task to be considered
    // executing as ads.
    kReport,
  };

  // Args:
  //   context: The ExecutionContext in which the task is executed.
  //   task: An identifier for the AsyncTask.
  //   step: A nullptr indicates a task that is not recurring. A non-null value
  //     indicates a recurring task with the value used for tracing events.
  //   enabled: Whether the task is asynchronous. If false, the task is not
  //     reported to the debugger and AdTracker.
  //   ad_tracking_type: Whether this is reported to the AdTracker.
  AsyncTask(ExecutionContext* execution_context,
            AsyncTaskContext* async_context,
            const char* step = nullptr,
            bool enabled = true,
            AdTrackingType ad_tracking_type = AdTrackingType::kReport);
  ~AsyncTask();

 private:
  ThreadDebugger* debugger_;
  AsyncTaskContext* task_context_;
  bool recurring_;

  // This persistent is safe since the class is STACK_ALLOCATED.
  Persistent<AdTracker> ad_tracker_;
};

// Called from generated instrumentation code.
inline CoreProbeSink* ToCoreProbeSink(LocalFrame* frame) {
  return frame ? frame->GetProbeSink() : nullptr;
}

inline CoreProbeSink* ToCoreProbeSink(ExecutionContext* context) {
  return context ? context->GetProbeSink() : nullptr;
}

inline CoreProbeSink* ToCoreProbeSink(v8::Isolate* isolate) {
  return isolate ? CurrentExecutionContext(isolate)->GetProbeSink() : nullptr;
}

inline CoreProbeSink* ToCoreProbeSink(const ScriptState& script_state) {
  return ToCoreProbeSink(ToExecutionContext(&script_state));
}

inline CoreProbeSink* ToCoreProbeSink(Document& document) {
  return ToCoreProbeSink(document.GetExecutionContext());
}

inline CoreProbeSink* ToCoreProbeSink(Document* document) {
  return document ? ToCoreProbeSink(document->GetExecutionContext()) : nullptr;
}

inline CoreProbeSink* ToCoreProbeSink(CoreProbeSink* sink) {
  return sink;
}

inline CoreProbeSink* ToCoreProbeSink(Node* node) {
  return node ? ToCoreProbeSink(node->GetDocument()) : nullptr;
}

inline CoreProbeSink* ToCoreProbeSink(EventTarget* event_target) {
  return event_target ? ToCoreProbeSink(event_target->GetExecutionContext())
                      : nullptr;
}

CoreProbeSink* ToCoreProbeSink(OffscreenCanvas* offscreen_canvas);

CORE_EXPORT void AllAsyncTasksCanceled(ExecutionContext*);

}  // namespace probe
}  // namespace blink

#include "base/time/time.h"
#include "third_party/blink/renderer/core/core_probes_inl.h"

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PROBE_CORE_PROBES_H_
