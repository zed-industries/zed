/*
 * Copyright (C) 2008 Apple Inc. All Rights Reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SCHEDULER_DOM_TIMER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SCHEDULER_DOM_TIMER_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/probe/async_task_context.h"
#include "third_party/blink/renderer/platform/bindings/name_client.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"
#include "third_party/blink/renderer/platform/timer.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

class ExecutionContext;
class ScheduledAction;
class ScriptState;
class ScriptValue;
class V8Function;

class CORE_EXPORT DOMTimer final : public GarbageCollected<DOMTimer>,
                                   public ExecutionContextLifecycleObserver,
                                   public TimerBase,
                                   public NameClient {
  USING_PRE_FINALIZER(DOMTimer, Dispose);

 public:
  static int setTimeout(ScriptState*,
                        ExecutionContext&,
                        V8Function* handler,
                        int timeout,
                        const HeapVector<ScriptValue>& arguments);
  static int setTimeout(ScriptState*,
                        ExecutionContext&,
                        const WTF::String& handler,
                        int timeout,
                        const HeapVector<ScriptValue>&);
  static int setInterval(ScriptState*,
                         ExecutionContext&,
                         V8Function* handler,
                         int timeout,
                         const HeapVector<ScriptValue>&);
  static int setInterval(ScriptState*,
                         ExecutionContext&,
                         const WTF::String& handler,
                         int timeout,
                         const HeapVector<ScriptValue>&);
  static void clearTimeout(ExecutionContext&, int timeout_id);
  static void clearInterval(ExecutionContext&, int timeout_id);

  DOMTimer(ExecutionContext&,
           ScheduledAction*,
           base::TimeDelta timeout,
           bool single_shot);
  ~DOMTimer() override;

  // ExecutionContextLifecycleObserver
  void ContextDestroyed() override;

  // Pre finalizer is needed to promptly stop this Timer object.
  // Otherwise timer events might fire at an object that's slated for
  // destruction (when lazily swept), but some of its members (m_action) may
  // already have been finalized & must not be accessed.
  void Dispose();

  void Trace(Visitor*) const override;
  const char* NameInHeapSnapshot() const override { return "DOMTimer"; }

  void Stop() override;

 private:
  static void RemoveByID(ExecutionContext&, int timeout_id);
  void Fired() override;

  // Increments the nesting level, clamping at the maximum value that can be
  // represented by |int|. Since the value is only used to compare with
  // |kMaxTimerNestingLevel|, the clamping doesn't affect behavior.
  void IncrementNestingLevel();

  int timeout_id_;
  int nesting_level_;
  probe::AsyncTaskContext async_task_context_;
  Member<ScheduledAction> action_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SCHEDULER_DOM_TIMER_H_
