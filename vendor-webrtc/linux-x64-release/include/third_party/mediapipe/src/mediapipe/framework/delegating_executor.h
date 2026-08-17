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

#ifndef MEDIAPIPE_FRAMEWORK_DELEGATING_EXECUTOR_H_
#define MEDIAPIPE_FRAMEWORK_DELEGATING_EXECUTOR_H_

#include "mediapipe/framework/executor.h"

namespace mediapipe {
namespace internal {

// An executor that delegates the running of tasks using a callback.
class DelegatingExecutor : public Executor {
 public:
  explicit DelegatingExecutor(
      std::function<void(std::function<void()>)> callback)
      : callback_(std::move(callback)) {}
  void Schedule(std::function<void()> task) override;

 private:
  std::function<void(std::function<void()>)> callback_;
};

}  // namespace internal
}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_DELEGATING_EXECUTOR_H_
