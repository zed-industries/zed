// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_WORKLET_PENDING_TASKS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_WORKLET_PENDING_TASKS_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class SerializedScriptValue;
class Worklet;

// Implementation of the "pending tasks struct":
// https://drafts.css-houdini.org/worklets/#pending-tasks-struct
//
// This also implements a part of the "fetch and invoke a worklet script"
// algorithm:
// https://drafts.css-houdini.org/worklets/#fetch-and-invoke-a-worklet-script
//
// All functions must be accessed on the main thread.
class CORE_EXPORT WorkletPendingTasks final
    : public GarbageCollected<WorkletPendingTasks> {
 public:
  WorkletPendingTasks(Worklet*, ScriptPromiseResolver<IDLUndefined>*);

  // This must be called after the construction and before decrementing the
  // counter.
  void InitializeCounter(int counter);

  // Sets |counter_| to -1 and rejects the promise.
  void Abort(scoped_refptr<SerializedScriptValue> error_to_rethrow);

  // Decrements |counter_| and resolves the promise if the counter becomes 0.
  void DecrementCounter();

  void Trace(Visitor*) const;

 private:
  // The number of pending tasks. -1 indicates these tasks are aborted and
  // |resolver_| already rejected the promise.
  int counter_;

  Member<ScriptPromiseResolver<IDLUndefined>> resolver_;

  Member<Worklet> worklet_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_WORKLET_PENDING_TASKS_H_
