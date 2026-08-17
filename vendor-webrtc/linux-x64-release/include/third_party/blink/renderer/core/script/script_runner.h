/*
 * Copyright (C) 2010 Google, Inc. All Rights Reserved.
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
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_SCRIPT_RUNNER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_SCRIPT_RUNNER_H_

#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/script/pending_script.h"
#include "third_party/blink/renderer/platform/bindings/name_client.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_deque.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/deque.h"

namespace blink {

class Document;

class CORE_EXPORT ScriptRunner final : public GarbageCollected<ScriptRunner>,
                                       public PendingScriptClient,
                                       public NameClient {
 public:
  explicit ScriptRunner(Document*);
  ~ScriptRunner() override = default;
  ScriptRunner(const ScriptRunner&) = delete;
  ScriptRunner& operator=(const ScriptRunner&) = delete;

  // Delays script evaluation after `ScriptRunnerDelayer::Activate()` until
  // `ScriptRunnerDelayer::Deactivate()`.
  //
  // Each `DelayReason` value represents one reason to delay, and there should
  // be at most one active `ScriptRunnerDelayer` for each `ScriptRunnerDelayer`
  // for each `ScriptRunner`.
  //
  // Each script can choose to wait or not to wait for each `DelayReason`, and
  // are evaluated after all of its relevant `ScriptRunnerDelayer`s are
  // deactivated.
  //
  // This can be spec-conformant (pretending that the loading of async scripts
  // are not completed until `ScriptRunnerDelayer`s are deactivated), but be
  // careful to avoid deadlocks and infinite delays.
  //
  // Currently this only affects async scripts and not in-order scripts.
  enum class DelayReason : uint8_t {
    // Script is loaded. Should be enabled for all scripts.
    kLoad = 1 << 0,
    // Milestone is reached as defined by https://crbug.com/1340837.
    kMilestone = 1 << 1,

    kTest1 = 1 << 6,
    kTest2 = 1 << 7,
  };
  using DelayReasons = std::underlying_type<DelayReason>::type;

  void QueueScriptForExecution(PendingScript*, DelayReasons);
  bool IsActive(DelayReason delay_reason) const {
    return active_delay_reasons_ & static_cast<DelayReasons>(delay_reason);
  }

  void SetTaskRunnerForTesting(base::SingleThreadTaskRunner* task_runner) {
    task_runner_ = task_runner;
  }

  // Returns true until all force in-order scripts are evaluated.
  // pending_force_in_order_scripts_ can be empty a little earlier than that.
  bool HasForceInOrderScripts() const {
    return pending_force_in_order_scripts_count_ > 0;
  }

  void Trace(Visitor*) const override;
  const char* NameInHeapSnapshot() const override { return "ScriptRunner"; }

  // PendingScriptClient
  void PendingScriptFinished(PendingScript*) override;

  void ExecuteAsyncPendingScript(PendingScript* pending_script,
                                 base::TimeTicks ready_to_evaluate_time);
  void ExecuteForceInOrderPendingScript(PendingScript*);
  void ExecuteParserBlockingScriptsBlockedByForceInOrder();

 private:
  // Execute the given pending script.
  void ExecutePendingScript(PendingScript*);

  friend class ScriptRunnerDelayer;
  void AddDelayReason(DelayReason);
  void RemoveDelayReason(DelayReason);
  void RemoveDelayReasonFromScript(PendingScript*, DelayReason);

  // https://html.spec.whatwg.org/C/#list-of-scripts-that-will-execute-in-order-as-soon-as-possible
  HeapDeque<Member<PendingScript>> pending_in_order_scripts_;
  // https://html.spec.whatwg.org/C/#set-of-scripts-that-will-execute-as-soon-as-possible
  // The value represents the `DelayReason`s that the script is waiting for
  // before its evaluation.
  HeapHashMap<Member<PendingScript>, DelayReasons> pending_async_scripts_;

  Member<Document> document_;

  HeapDeque<Member<PendingScript>> pending_force_in_order_scripts_;
  // The number of force in-order scripts that aren't yet evaluated. This is
  // different from pending_force_in_order_scripts_.size() == the number of
  // force in-order scripts that aren't yet scheduled to evaluate.
  wtf_size_t pending_force_in_order_scripts_count_ = 0;

  scoped_refptr<base::SingleThreadTaskRunner> task_runner_;
  scoped_refptr<base::SingleThreadTaskRunner> low_priority_task_runner_;

  DelayReasons active_delay_reasons_ = 0;
};

class CORE_EXPORT ScriptRunnerDelayer final
    : public GarbageCollected<ScriptRunnerDelayer> {
 public:
  ScriptRunnerDelayer(ScriptRunner*, ScriptRunner::DelayReason);
  ~ScriptRunnerDelayer() = default;
  void Activate();
  void Deactivate();

  ScriptRunnerDelayer(const ScriptRunnerDelayer&) = delete;
  ScriptRunnerDelayer& operator=(const ScriptRunnerDelayer&) = delete;

  void Trace(Visitor*) const;

 private:
  WeakMember<ScriptRunner> script_runner_;
  const ScriptRunner::DelayReason delay_reason_;
  bool activated_ = false;
};

// This function is exported for testing purposes only.
void CORE_EXPORT PostTaskWithLowPriorityUntilTimeoutForTesting(
    const base::Location& from_here,
    base::OnceClosure task,
    base::TimeDelta timeout,
    scoped_refptr<base::SingleThreadTaskRunner> lower_priority_task_runner,
    scoped_refptr<base::SingleThreadTaskRunner> normal_priority_task_runner);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_SCRIPT_RUNNER_H_
