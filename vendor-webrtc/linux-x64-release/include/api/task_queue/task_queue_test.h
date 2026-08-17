/*
 *  Copyright 2019 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef API_TASK_QUEUE_TASK_QUEUE_TEST_H_
#define API_TASK_QUEUE_TASK_QUEUE_TEST_H_

#include <functional>
#include <memory>

#include "api/field_trials_view.h"
#include "api/task_queue/task_queue_factory.h"
#include "test/gtest.h"

namespace webrtc {

// Suite of tests to verify TaskQueue implementation with.
// Example usage:
//
// namespace {
//
// using ::testing::Values;
// using ::webrtc::TaskQueueTest;
//
// std::unique_ptr<webrtc::TaskQueueFactory> CreateMyFactory();
//
// INSTANTIATE_TEST_SUITE_P(My, TaskQueueTest, Values(CreateMyFactory));
//
// }  // namespace
class TaskQueueTest
    : public ::testing::TestWithParam<std::function<
          std::unique_ptr<TaskQueueFactory>(const FieldTrialsView*)>> {};

}  // namespace webrtc

#endif  // API_TASK_QUEUE_TASK_QUEUE_TEST_H_
