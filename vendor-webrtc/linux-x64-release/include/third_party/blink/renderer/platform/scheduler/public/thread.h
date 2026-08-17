/*
 * Copyright (C) 2011 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE AND ITS CONTRIBUTORS "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE OR ITS CONTRIBUTORS BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_THREAD_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_THREAD_H_

#include <stdint.h>
#include "base/functional/callback_forward.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/scoped_refptr.h"
#include "base/task/task_observer.h"
#include "base/threading/thread.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/scheduler/public/thread_type.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace base {
class TimeTicks;
namespace sequence_manager {
class TaskTimeObserver;
}
}  // namespace base

namespace blink {

class FrameOrWorkerScheduler;
class MainThread;
class NonMainThread;
class ThreadScheduler;

// Always an integer value.
typedef uintptr_t PlatformThreadId;

struct PLATFORM_EXPORT ThreadCreationParams {
  explicit ThreadCreationParams(ThreadType);

  ThreadCreationParams& SetThreadNameForTest(const char* name);

  // Sets a scheduler for the context which was responsible for the creation
  // of this thread.
  ThreadCreationParams& SetFrameOrWorkerScheduler(FrameOrWorkerScheduler*);

  ThreadCreationParams& SetSupportsGC(bool supports_gc);

  ThreadType thread_type;
  const char* name;
  raw_ptr<FrameOrWorkerScheduler> frame_or_worker_scheduler;  // NOT OWNED

  // Do NOT set the thread priority for non-WebAudio usages. Please consult
  // scheduler-dev@ first in order to use an elevated thread priority.
  base::ThreadType base_thread_type = base::ThreadType::kDefault;

  // The interval at which the thread expects to have work to do. Zero if
  // unknown. Used when configuring a thread with `base_thread_type`
  // base::ThreadType::kRealtimeAudio.
  base::TimeDelta realtime_period;

  bool supports_gc = false;
};

// The interface of a thread recognized by Blink.
//
// Deleting the thread blocks until all pending, non-delayed tasks have been
// run.
class PLATFORM_EXPORT Thread {
  USING_FAST_MALLOC(Thread);

 public:
  // An IdleTask is expected to complete before the deadline it is passed.
  using IdleTask = base::OnceCallback<void(base::TimeTicks deadline)>;

  // TaskObserver is an observer fired before and after a task is executed.
  using TaskObserver = base::TaskObserver;

  // Create and save (as a global variable) the compositor thread. The thread
  // will be accessible through CompositorThread().
  static void CreateAndSetCompositorThread();

  // Return an interface to the current thread.
  static Thread* Current();

  // Return an interface to the main thread.
  static blink::MainThread* MainThread();

  // Return an interface to the compositor thread (if initialized). This can be
  // null if the renderer was created with threaded rendering disabled.
  static NonMainThread* CompositorThread();

  Thread();
  Thread(const Thread&) = delete;
  Thread& operator=(const Thread&) = delete;
  virtual ~Thread();

  // Must be called immediately after the construction.
  virtual void Init() {}

  bool IsCurrentThread() const;

  // TaskObserver is an object that receives task notifications from the
  // MessageLoop.
  // NOTE: TaskObserver implementation should be extremely fast!
  // This API is performance sensitive. Use only if you have a compelling
  // reason.
  void AddTaskObserver(TaskObserver*);
  void RemoveTaskObserver(TaskObserver*);

  // TaskTimeObserver is an object that receives notifications for
  // CPU time spent in each top-level MessageLoop task.
  // NOTE: TaskTimeObserver implementation should be extremely fast!
  // This API is performance sensitive. Use only if you have a compelling
  // reason.
  virtual void AddTaskTimeObserver(base::sequence_manager::TaskTimeObserver*) {}
  virtual void RemoveTaskTimeObserver(
      base::sequence_manager::TaskTimeObserver*) {}

  // Returns the start time for the current task. Can be used instead of
  // TaskTimeObserver if only task start time is needed.
  virtual base::TimeTicks CurrentTaskStartTime() const {
    NOTIMPLEMENTED();
    return base::TimeTicks();
  }

  // Returns the scheduler associated with the thread.
  virtual ThreadScheduler* Scheduler() = 0;

  // See WorkerThread::ShutdownOnThread().
  virtual void ShutdownOnThread() {}

 protected:
  static void UpdateThreadTLS(Thread* thread);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_THREAD_H_
