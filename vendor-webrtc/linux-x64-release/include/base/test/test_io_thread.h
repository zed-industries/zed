// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_TEST_IO_THREAD_H_
#define BASE_TEST_TEST_IO_THREAD_H_

#include "base/compiler_specific.h"
#include "base/functional/callback_forward.h"
#include "base/memory/ref_counted.h"
#include "base/task/task_runner.h"
#include "base/threading/thread.h"

namespace base {

// Create and run an IO thread with a MessageLoop, and
// making the MessageLoop accessible from its client.
// It also provides some ideomatic API like PostTaskAndWait().
//
// This API is not thread-safe:
//   - Start()/Stop() should only be called from the main (creation) thread.
//   - PostTask()/message_loop()/task_runner() are also safe to call from the
//     underlying thread itself (to post tasks from other threads: get the
//     task_runner() from the main thread first, it is then safe to pass _it_
//     around).
class TestIOThread {
 public:
  enum Mode { kAutoStart, kManualStart };
  explicit TestIOThread(Mode mode);

  TestIOThread(const TestIOThread&) = delete;
  TestIOThread& operator=(const TestIOThread&) = delete;

  // Stops the I/O thread if necessary.
  ~TestIOThread();

  // After Stop(), Start() may be called again to start a new I/O thread.
  // Stop() may be called even when the I/O thread is not started.
  void Start();
  void Stop();

  // Post |task| to the IO thread.
  void PostTask(const Location& from_here, base::OnceClosure task);

  scoped_refptr<SingleThreadTaskRunner> task_runner() {
    return io_thread_.task_runner();
  }

 private:
  base::Thread io_thread_;
  bool io_thread_started_;
};

}  // namespace base

#endif  // BASE_TEST_TEST_IO_THREAD_H_
