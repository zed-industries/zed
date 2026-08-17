/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef TEST_RUN_LOOP_H_
#define TEST_RUN_LOOP_H_

#include <utility>

#include "absl/functional/any_invocable.h"
#include "api/task_queue/task_queue_base.h"
#include "rtc_base/thread.h"

namespace webrtc {
namespace test {

// This utility class allows you to run a TaskQueue supported interface on the
// main test thread, call Run() while doing things asynchonously and break
// the loop (from the same thread) from a callback by calling Quit().
class RunLoop {
 public:
  RunLoop();
  ~RunLoop();

  TaskQueueBase* task_queue();

  void Run();
  void Quit();

  void Flush();

  void PostTask(absl::AnyInvocable<void() &&> task) {
    task_queue()->PostTask(std::move(task));
  }

 private:
  class FakeSocketServer : public SocketServer {
   public:
    FakeSocketServer();
    ~FakeSocketServer();

    void FailNextWait();

   private:
    bool Wait(webrtc::TimeDelta max_wait_duration, bool process_io) override;
    void WakeUp() override;

    Socket* CreateSocket(int family, int type) override;

   private:
    bool fail_next_wait_ = false;
  };

  class WorkerThread : public Thread {
   public:
    explicit WorkerThread(SocketServer* ss);

   private:
    CurrentTaskQueueSetter tq_setter_;
  };

  FakeSocketServer socket_server_;
  WorkerThread worker_thread_{&socket_server_};
};

}  // namespace test
}  // namespace webrtc

#endif  // TEST_RUN_LOOP_H_
