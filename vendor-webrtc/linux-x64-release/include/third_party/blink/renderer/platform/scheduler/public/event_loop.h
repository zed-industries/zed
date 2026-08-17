// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_EVENT_LOOP_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_EVENT_LOOP_H_

#include <memory>

#include "base/functional/callback.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/deque.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"
#include "third_party/blink/renderer/platform/wtf/ref_counted.h"

namespace v8 {
class Isolate;
class MicrotaskQueue;
}  // namespace v8

namespace blink {

class Agent;
class FrameOrWorkerScheduler;

namespace scheduler {

// Represents an event loop. The instance is held by ExecutionContexts.
// https://html.spec.whatwg.org/C#event-loop
//
// Browsing contexts must share the same EventLoop if they have a chance to
// access each other synchronously.
// That is:
//  - Two Documents must share the same EventLoop if they are scriptable with
//    each other.
//  - Workers and Worklets can have its own EventLoop, as no other browsing
//    context can access it synchronously.
//
// The specification says an event loop has (non-micro) task queues. However,
// we process regular tasks in a different granularity; in our implementation,
// a frame has task queues. This is an intentional violation of the
// specification.
//
// Therefore, currently, EventLoop is a unit that just manages a microtask
// queue: <https://html.spec.whatwg.org/C#microtask-queue>
//
// Microtasks queued during a task are executed at the end of the task or
// after a user script is executed (for the exact timings, refer to the
// specification). Some web platform features require this functionality.
//
// Implementation notes: Originally, microtask queues were created in V8
// for JavaScript promises. V8 allocates a default microtask queue per isolate,
// and it still uses the default queue, not the one in this EventLoop class.
// This is not correct in terms of the standards conformance, and we'll
// eventually merge the queues so both Blink and V8 can use the microtask queue
// allocated in the correct granularity.
class PLATFORM_EXPORT EventLoop final : public WTF::RefCounted<EventLoop> {
  USING_FAST_MALLOC(EventLoop);

 public:
  // A pure virtual class implemented by the `environment settings object`.
  // Callbacks exist for steps completed in the microtask completion
  // algorithm.
  class Delegate : public GarbageCollectedMixin {
   public:
    virtual void NotifyRejectedPromises() = 0;
  };

  EventLoop(const EventLoop&) = delete;
  EventLoop& operator=(const EventLoop&) = delete;

  // Queues |cb| to the backing v8::MicrotaskQueue.
  void EnqueueMicrotask(base::OnceClosure cb);

  // Runs |cb| at the end of microtask checkpoint.
  // The tasks are run when control is returning to C++ from script, after
  // executing a script task (e.g. callback, event) or microtasks
  // (e.g. promise). This is explicitly needed for Indexed DB transactions
  // per spec, but should in general be avoided.
  void EnqueueEndOfMicrotaskCheckpointTask(base::OnceClosure cb);

  // Run any pending tasks.
  void RunEndOfMicrotaskCheckpointTasks();

  // Runs pending microtasks until the queue is empty.
  void PerformMicrotaskCheckpoint();

  // Runs pending microtasks on the isolate's default MicrotaskQueue until it's
  // empty.
  static void PerformIsolateGlobalMicrotasksCheckpoint(v8::Isolate* isolate);

  void AttachScheduler(FrameOrWorkerScheduler*);
  void DetachScheduler(FrameOrWorkerScheduler*);

  // Returns the MicrotaskQueue instance to be associated to v8::Context. Pass
  // it to v8::Context::New().
  v8::MicrotaskQueue* microtask_queue() const { return microtask_queue_.get(); }

  bool IsSchedulerAttachedForTest(FrameOrWorkerScheduler*);

 private:
  friend class WTF::RefCounted<EventLoop>;
  friend blink::Agent;

  EventLoop(Delegate* delegate,
            v8::Isolate* isolate,
            std::unique_ptr<v8::MicrotaskQueue> microtask_queue);
  ~EventLoop();

  static void RunPendingMicrotask(void* data);
  static void RunEndOfCheckpointTasks(v8::Isolate* isolat, void* data);

  WeakPersistent<Delegate> delegate_;
  raw_ptr<v8::Isolate> isolate_;
  bool loop_enabled_ = true;
  Deque<base::OnceClosure> pending_microtasks_;
  Vector<base::OnceClosure> end_of_checkpoint_tasks_;
  std::unique_ptr<v8::MicrotaskQueue> microtask_queue_;
  HashSet<FrameOrWorkerScheduler*> schedulers_;
};

}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_EVENT_LOOP_H_
