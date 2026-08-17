// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_THREAD_POOL_WORKER_THREAD_OBSERVER_H_
#define BASE_TASK_THREAD_POOL_WORKER_THREAD_OBSERVER_H_

namespace base {

// Interface to observe entry and exit of the main function of a ThreadPool
// worker.
class WorkerThreadObserver {
 public:
  virtual ~WorkerThreadObserver() = default;

  // Invoked at the beginning of the main function of a ThreadPool worker,
  // before any task runs.
  virtual void OnWorkerThreadMainEntry() = 0;

  // Invoked at the end of the main function of a ThreadPool worker, when it
  // can no longer run tasks.
  virtual void OnWorkerThreadMainExit() = 0;
};

}  // namespace base

#endif  // BASE_TASK_THREAD_POOL_WORKER_THREAD_OBSERVER_H_
