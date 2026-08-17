// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_WORKLET_THREAD_HOLDER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_WORKLET_THREAD_HOLDER_H_

#include "base/synchronization/lock.h"
#include "base/synchronization/waitable_event.h"
#include "base/thread_annotations.h"
#include "third_party/blink/renderer/core/workers/worker_backing_thread.h"
#include "third_party/blink/renderer/core/workers/worker_backing_thread_startup_data.h"
#include "third_party/blink/renderer/platform/scheduler/public/post_cross_thread_task.h"
#include "third_party/blink/renderer/platform/wtf/wtf.h"

namespace blink {

// WorkletThreadHolder is a template class which is designed for per-process
// singleton instance of DerivedWorkletThread (i.e. AnimationWorkletThread,
// AudioWorkletThread).
template <class DerivedWorkletThread>
class WorkletThreadHolder {
 public:
  static WorkletThreadHolder<DerivedWorkletThread>* GetInstance() {
    base::AutoLock locker(HolderInstanceLock());
    return thread_holder_instance_;
  }

  static void EnsureInstance(const ThreadCreationParams& params) {
    DCHECK(IsMainThread());
    base::AutoLock locker(HolderInstanceLock());
    if (thread_holder_instance_)
      return;
    thread_holder_instance_ = new WorkletThreadHolder<DerivedWorkletThread>;
    thread_holder_instance_->Initialize(
        std::make_unique<WorkerBackingThread>(params));
  }

  static void ClearInstance() {
    DCHECK(IsMainThread());
    base::AutoLock locker(HolderInstanceLock());
    if (thread_holder_instance_) {
      thread_holder_instance_->ShutdownAndWait();
      delete thread_holder_instance_;
      thread_holder_instance_ = nullptr;
    }
  }

  WorkerBackingThread* GetThread() { return thread_.get(); }

 private:
  WorkletThreadHolder() = default;
  ~WorkletThreadHolder() = default;

  static base::Lock& HolderInstanceLock() {
    DEFINE_THREAD_SAFE_STATIC_LOCAL(base::Lock, holder_lock, ());
    return holder_lock;
  }

  void Initialize(std::unique_ptr<WorkerBackingThread> backing_thread) {
    thread_ = std::move(backing_thread);
    PostCrossThreadTask(
        *thread_->BackingThread().GetTaskRunner(), FROM_HERE,
        CrossThreadBindOnce(&WorkletThreadHolder::InitializeOnWorkletThread,
                            CrossThreadUnretained(this)));
  }

  void InitializeOnWorkletThread() {
    base::AutoLock locker(HolderInstanceLock());
    thread_->InitializeOnBackingThread(
        WorkerBackingThreadStartupData::CreateDefault());
  }

  void ShutdownAndWait() {
    DCHECK(IsMainThread());
    base::WaitableEvent waitable_event;
    PostCrossThreadTask(
        *thread_->BackingThread().GetTaskRunner(), FROM_HERE,
        CrossThreadBindOnce(&WorkletThreadHolder::ShutdownOnWorkletThread,
                            CrossThreadUnretained(this),
                            CrossThreadUnretained(&waitable_event)));
    waitable_event.Wait();
  }

  void ShutdownOnWorkletThread(base::WaitableEvent* waitable_event) {
    thread_->ShutdownOnBackingThread();
    waitable_event->Signal();
  }

  std::unique_ptr<WorkerBackingThread> thread_;

  static WorkletThreadHolder<DerivedWorkletThread>* thread_holder_instance_;
};

template <class DerivedWorkletThread>
WorkletThreadHolder<DerivedWorkletThread>*
    WorkletThreadHolder<DerivedWorkletThread>::thread_holder_instance_
        GUARDED_BY(
            WorkletThreadHolder<DerivedWorkletThread>::HolderInstanceLock()) =
            nullptr;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_WORKLET_THREAD_HOLDER_H_
