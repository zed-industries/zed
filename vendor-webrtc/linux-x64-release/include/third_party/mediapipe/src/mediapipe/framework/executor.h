// Copyright 2019 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Executor class for the MediaPipe scheduler.

#ifndef MEDIAPIPE_FRAMEWORK_EXECUTOR_H_
#define MEDIAPIPE_FRAMEWORK_EXECUTOR_H_

#include <functional>

// TODO: Move protos in another CL after the C++ code migration.
#include "mediapipe/framework/deps/registration.h"
#include "mediapipe/framework/mediapipe_options.pb.h"
#include "mediapipe/framework/port/statusor.h"

namespace mediapipe {

// Abstract base class for the task queue.
// NOTE: The task queue orders the ready tasks by their priorities. This
// enables the executor to run ready tasks in priority order.
class TaskQueue {
 public:
  virtual ~TaskQueue();

  // Runs the next ready task in the current thread. Should be invoked by the
  // executor. This method should be called exactly as many times as AddTask
  // was called on the executor.
  virtual void RunNextTask() = 0;
};

// Abstract base class for the Executor.
class Executor {
 public:
  virtual ~Executor();

  // A registered Executor subclass must implement the static factory method
  // Create.  The Executor subclass cannot be registered without it.
  //
  // static absl::StatusOr<Executor*> Create(
  //     const MediaPipeOptions& extendable_options);
  //
  // Create validates extendable_options, then calls the constructor, and
  // returns the newly allocated Executor object.

  // The scheduler queue calls this method to tell the executor that it has
  // a new task to run. The executor should use its execution mechanism to
  // invoke task_queue->RunNextTask.
  virtual void AddTask(TaskQueue* task_queue) {
    Schedule([task_queue] { task_queue->RunNextTask(); });
  }

  // Schedule the specified "task" for execution in this executor.
  virtual void Schedule(std::function<void()> task) = 0;
};

using ExecutorRegistry =
    GlobalFactoryRegistry<absl::StatusOr<Executor*>, const MediaPipeOptions&>;

// Macro for registering the executor.
#define REGISTER_EXECUTOR(name)        \
  REGISTER_FACTORY_FUNCTION_QUALIFIED( \
      mediapipe::ExecutorRegistry, executor_registration, name, name::Create)

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_EXECUTOR_H_
