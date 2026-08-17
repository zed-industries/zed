// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_THREAD_POOL_SERVICE_THREAD_H_
#define BASE_TASK_THREAD_POOL_SERVICE_THREAD_H_

#include "base/base_export.h"
#include "base/threading/thread.h"

namespace base {
namespace internal {

// The ThreadPool's ServiceThread is a mostly idle thread that is responsible
// for handling async events (e.g. delayed tasks and async I/O). Its role is to
// merely forward such events to their destination (hence staying mostly idle
// and highly responsive).
// It aliases Thread::Run() to enforce that ServiceThread::Run() be on the stack
// and make it easier to identify the service thread in stack traces.
class BASE_EXPORT ServiceThread : public Thread {
 public:
  ServiceThread();

  ServiceThread(const ServiceThread&) = delete;
  ServiceThread& operator=(const ServiceThread&) = delete;
  ~ServiceThread() override = default;

 private:
  // Thread:
  void Run(RunLoop* run_loop) override;
};

}  // namespace internal
}  // namespace base

#endif  // BASE_TASK_THREAD_POOL_SERVICE_THREAD_H_
