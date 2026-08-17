/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef AUDIO_VOIP_TEST_MOCK_TASK_QUEUE_H_
#define AUDIO_VOIP_TEST_MOCK_TASK_QUEUE_H_

#include <memory>

#include "api/task_queue/task_queue_factory.h"
#include "api/task_queue/test/mock_task_queue_base.h"
#include "test/gmock.h"

namespace webrtc {

// MockTaskQueue enables immediate task run from global TaskQueueBase.
// It's necessary for some tests depending on TaskQueueBase internally.
class MockTaskQueue : public MockTaskQueueBase {
 public:
  MockTaskQueue() : current_(this) {}

  // Delete is deliberately defined as no-op as MockTaskQueue is expected to
  // hold onto current global TaskQueueBase throughout the testing.
  void Delete() override {}

 private:
  CurrentTaskQueueSetter current_;
};

class MockTaskQueueFactory : public TaskQueueFactory {
 public:
  explicit MockTaskQueueFactory(MockTaskQueue* task_queue)
      : task_queue_(task_queue) {}

  std::unique_ptr<TaskQueueBase, TaskQueueDeleter> CreateTaskQueue(
      absl::string_view /* name */,
      Priority /* priority */) const override {
    // Default MockTaskQueue::Delete is no-op, therefore it's safe to pass the
    // raw pointer.
    return std::unique_ptr<TaskQueueBase, TaskQueueDeleter>(task_queue_);
  }

 private:
  MockTaskQueue* task_queue_;
};

}  // namespace webrtc

#endif  // AUDIO_VOIP_TEST_MOCK_TASK_QUEUE_H_
