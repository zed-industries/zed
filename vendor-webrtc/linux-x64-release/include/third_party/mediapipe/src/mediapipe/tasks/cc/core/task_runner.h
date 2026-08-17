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

#ifndef MEDIAPIPE_TASKS_CC_CORE_TASK_RUNNER_H_
#define MEDIAPIPE_TASKS_CC_CORE_TASK_RUNNER_H_

#if defined(OS_POSIX) || defined(OS_FUCHSIA)
#include <unistd.h>
#endif

#include <atomic>
#include <functional>
#include <map>
#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "absl/base/thread_annotations.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/synchronization/mutex.h"
#include "mediapipe/framework/calculator.pb.h"
#include "mediapipe/framework/calculator_framework.h"
#include "mediapipe/framework/executor.h"
#include "tensorflow/lite/core/api/op_resolver.h"

namespace mediapipe {

#if !MEDIAPIPE_DISABLE_GPU
class GpuResources;
#endif  // !MEDIAPIPE_DISABLE_GPU

namespace tasks {
namespace core {

using ErrorFn = std::function<void(absl::Status)>;

// Mapping from the MediaPipe calculator graph stream/side packet names to the
// packets.
using PacketMap = std::map<std::string, Packet>;
// A callback method to get output packets from the task runner.
using PacketsCallback = std::function<void(absl::StatusOr<PacketMap>)>;

// The mediapipe task runner class.
// The runner has two processing modes: synchronous mode and asynchronous mode.
// In the synchronous mode, clients send input data using the blocking API,
// Process(), and wait until the results are returned from the same method.
// In the asynchronous mode, clients send input data using the non-blocking
// method, Send(), and receive the results in the user-defined PacketsCallback
// at a later point in time.
// As the two processing modes are incompatible, each TaskRunner instance can
// operate in only one processing mode, which is defined at construction time
// based on whether a PacketsCallback is provided (asynchronous mode) or not
// (synchronous mode).
class TaskRunner {
 public:
  // Creates the task runner with a CalculatorGraphConfig proto.
  // If a tflite op resolver object is provided, the task runner will take
  // it as the global op resolver for all models running within this task.
  // The op resolver's ownership will be transferred into the pipeleine runner.
  // When a user-defined PacketsCallback is provided, clients must use the
  // asynchronous method, Send(), to provide the input packets. If the packets
  // callback is absent, clients must use the synchronous method, Process(), to
  // provide the input packets and receive the output packets.
#if !MEDIAPIPE_DISABLE_GPU
  static absl::StatusOr<std::unique_ptr<TaskRunner>> Create(
      CalculatorGraphConfig config,
      std::unique_ptr<tflite::OpResolver> op_resolver = nullptr,
      PacketsCallback packets_callback = nullptr,
      std::shared_ptr<Executor> default_executor = nullptr,
      std::optional<PacketMap> input_side_packets = std::nullopt,
      std::shared_ptr<::mediapipe::GpuResources> resources = nullptr,
      std::optional<ErrorFn> error_fn = std::nullopt,
      bool disable_default_service = false);
#else
  static absl::StatusOr<std::unique_ptr<TaskRunner>> Create(
      CalculatorGraphConfig config,
      std::unique_ptr<tflite::OpResolver> op_resolver = nullptr,
      PacketsCallback packets_callback = nullptr,
      std::shared_ptr<Executor> default_executor = nullptr,
      std::optional<PacketMap> input_side_packets = std::nullopt,
      std::optional<ErrorFn> error_fn = std::nullopt,
      bool disable_default_service = false);
#endif  // !MEDIAPIPE_DISABLE_GPU

  // TaskRunner is neither copyable nor movable.
  TaskRunner(const TaskRunner&) = delete;
  TaskRunner& operator=(const TaskRunner&) = delete;

  // A synchronous method that is designed for processing either batch data such
  // as unrelated images and texts or offline streaming data such as the decoded
  // frames from a video file and an audio file. The call blocks the current
  // thread until a failure status or a successful result is returned.
  // If the input packets have no timestamp, an internal timestamp will be
  // assigned per invocation. Otherwise, when the timestamp is set in the
  // input packets, the caller must ensure that the input packet timestamps are
  // greater than the timestamps of the previous invocation. This method is
  // thread-unsafe and it is the caller's responsibility to synchronize access
  // to this method across multiple threads and to ensure that the input packet
  // timestamps are in order.
  absl::StatusOr<PacketMap> Process(PacketMap inputs);

  // An asynchronous method that is designed for handling live streaming data
  // such as live camera and microphone data. A user-defined PacketsCallback
  // function must be provided in the constructor to receive the output packets.
  // The caller must ensure that the input packet timestamps are monotonically
  // increasing. This method is thread-unsafe and it is the caller's
  // responsibility to synchronize access to this method across multiple
  // threads and to ensure that the input packet timestamps are in order.
  absl::Status Send(PacketMap inputs);

  // Shuts down the task runner. After the runner is closed, unless the
  // runner's Start method is called again, any calls that send input data
  // to the runner are illegal and will receive errors.
  absl::Status Close();

  // Resets and restarts the task runner. This can be useful for resetting
  // a stateful task graph to process new data.
  absl::Status Restart();

  // Returns the canonicalized CalculatorGraphConfig of the underlying graph.
  const CalculatorGraphConfig& GetGraphConfig() { return graph_.Config(); }

 private:
  // Constructor.
  // Creates a TaskRunner instance with an optional PacketsCallback method.
  explicit TaskRunner(PacketsCallback packets_callback = nullptr)
      : packets_callback_(packets_callback) {}

  // Initializes the task runner. Returns an ok status to indicate that the
  // runner is ready to start. Otherwise, returns an error status to indicate
  // that the runner isn't initialized successfully. A task runner should
  // be only initialized once.
  absl::Status Initialize(
      CalculatorGraphConfig config,
      std::unique_ptr<tflite::OpResolver> op_resolver = nullptr,
      std::shared_ptr<Executor> default_executor = nullptr,
      std::optional<PacketMap> input_side_packets = std::nullopt,
      std::optional<ErrorFn> error_fn = std::nullopt,
      bool disable_default_service = false);

  // Starts the task runner. Returns an ok status to indicate that the
  // runner is ready to accept input data. Otherwise, returns an error status to
  // indicate that the runner isn't started successfully.
  absl::Status Start();

  PacketsCallback packets_callback_;
  std::vector<std::string> output_stream_names_;
  CalculatorGraph graph_;
  bool initialized_ = false;
  std::atomic_bool is_running_ = false;

  absl::StatusOr<PacketMap> status_or_output_packets_;
  Timestamp last_seen_ ABSL_GUARDED_BY(mutex_);
  absl::Mutex mutex_;
};

}  // namespace core
}  // namespace tasks
}  // namespace mediapipe

#endif  // MEDIAPIPE_TASKS_CC_CORE_TASK_RUNNER_H_
