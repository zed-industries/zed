/*
 * Copyright (C) 2011 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_WORKER_INSPECTOR_CONTROLLER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_WORKER_INSPECTOR_CONTROLLER_H_

#include "base/functional/callback.h"
#include "base/memory/scoped_refptr.h"
#include "base/unguessable_token.h"
#include "third_party/blink/renderer/core/inspector/devtools_agent.h"
#include "third_party/blink/renderer/core/inspector/inspector_task_runner.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/instrumentation/tracing/trace_event.h"
#include "third_party/blink/renderer/platform/scheduler/public/thread.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class CoreProbeSink;
class InspectedFrames;
class WorkerThread;
class WorkerThreadDebugger;
struct WorkerDevToolsParams;

class WorkerInspectorController final
    : public GarbageCollected<WorkerInspectorController>,
      public trace_event::EnabledStateObserver,
      public DevToolsAgent::Client,
      private Thread::TaskObserver {
 public:
  static WorkerInspectorController* Create(
      WorkerThread*,
      const KURL&,
      scoped_refptr<InspectorTaskRunner>,
      std::unique_ptr<WorkerDevToolsParams>);

  WorkerInspectorController(WorkerThread*,
                            const KURL&,
                            WorkerThreadDebugger*,
                            scoped_refptr<InspectorTaskRunner>,
                            std::unique_ptr<WorkerDevToolsParams>);
  WorkerInspectorController(const WorkerInspectorController&) = delete;
  WorkerInspectorController& operator=(const WorkerInspectorController&) =
      delete;
  ~WorkerInspectorController() override;
  void Trace(Visitor*) const override;

  CoreProbeSink* GetProbeSink() const { return probe_sink_.Get(); }
  DevToolsAgent* GetDevToolsAgent() const { return agent_.Get(); }
  void Dispose();
  void FlushProtocolNotifications();
  void WaitForDebuggerIfNeeded();

 private:
  // Thread::TaskObserver implementation.
  void WillProcessTask(const base::PendingTask&, bool) override;
  void DidProcessTask(const base::PendingTask&) override;

  // blink::trace_event::EnabledStateObserver implementation:
  void OnTraceLogEnabled() override;
  void OnTraceLogDisabled() override;

  void EmitTraceEvent();

  // DevToolsAgent::Client implementation.
  void AttachSession(DevToolsSession*, bool restore) override;
  void DetachSession(DevToolsSession*) override;
  void InspectElement(const gfx::Point&) override;
  void DebuggerTaskStarted() override;
  void DebuggerTaskFinished() override;

  Member<DevToolsAgent> agent_;
  WorkerThreadDebugger* debugger_;
  WorkerThread* thread_;
  Member<InspectedFrames> inspected_frames_;
  Member<CoreProbeSink> probe_sink_;
  int session_count_ = 0;
  bool wait_for_debugger_ = false;

  // These fields are set up in the constructor and then read
  // on a random thread from EmitTraceEvent().
  base::UnguessableToken worker_devtools_token_;
  base::UnguessableToken parent_devtools_token_;
  KURL url_;
  const base::PlatformThreadId worker_thread_id_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_WORKER_INSPECTOR_CONTROLLER_H_
