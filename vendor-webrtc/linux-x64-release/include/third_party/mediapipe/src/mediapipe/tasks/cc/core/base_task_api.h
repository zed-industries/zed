/* Copyright 2022 The MediaPipe Authors.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
==============================================================================*/

#ifndef MEDIAPIPE_TASKS_CC_CORE_BASE_TASK_API_H_
#define MEDIAPIPE_TASKS_CC_CORE_BASE_TASK_API_H_

#include <memory>
#include <utility>
#include <vector>

#include "mediapipe/tasks/cc/core/task_runner.h"

namespace mediapipe {
namespace tasks {
namespace core {

// The base class of the user-facing mediapipe tasks api classes.
class BaseTaskApi {
 public:
  // Constructor.
  explicit BaseTaskApi(std::unique_ptr<TaskRunner> runner)
      : runner_(std::move(runner)) {}
  // BaseTaskApi is neither copyable nor movable.
  BaseTaskApi(const BaseTaskApi&) = delete;
  BaseTaskApi& operator=(const BaseTaskApi&) = delete;

 protected:
  // The task runner of the task api.
  std::unique_ptr<TaskRunner> runner_;
};

}  // namespace core
}  // namespace tasks
}  // namespace mediapipe

#endif  // MEDIAPIPE_TASKS_CC_CORE_BASE_TASK_API_H_
