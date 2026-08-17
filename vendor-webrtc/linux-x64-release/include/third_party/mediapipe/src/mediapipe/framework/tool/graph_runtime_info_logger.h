// Copyright 2024 The MediaPipe Authors.
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

#ifndef MEDIAPIPE_FRAMEWORK_TOOL_GRAPH_RUNTIME_INFO_LOGGER_H_
#define MEDIAPIPE_FRAMEWORK_TOOL_GRAPH_RUNTIME_INFO_LOGGER_H_

#include "absl/functional/any_invocable.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/synchronization/notification.h"
#include "mediapipe/framework/calculator.pb.h"
#include "mediapipe/framework/graph_runtime_info.pb.h"
#include "mediapipe/framework/port/threadpool.h"

namespace mediapipe::tool {

// Periodically collects the graph runtime info and output it to LOG(INFO).
class GraphRuntimeInfoLogger {
 public:
  GraphRuntimeInfoLogger();
  ~GraphRuntimeInfoLogger();

  // Starts the collector in the background. Can be called only once.
  absl::Status StartInBackground(
      const mediapipe::GraphRuntimeInfoConfig& config,
      absl::AnyInvocable<absl::StatusOr<GraphRuntimeInfo>()>
          get_runtime_info_fn);

 private:
  void Stop();

  absl::Notification shutdown_signal_;
  absl::Notification is_running_;
  absl::AnyInvocable<absl::StatusOr<GraphRuntimeInfo>()> get_runtime_info_fn_;
  ThreadPool thread_pool_;
};

}  // namespace mediapipe::tool

#endif  // MEDIAPIPE_FRAMEWORK_TOOL_GRAPH_RUNTIME_INFO_LOGGER_H_
