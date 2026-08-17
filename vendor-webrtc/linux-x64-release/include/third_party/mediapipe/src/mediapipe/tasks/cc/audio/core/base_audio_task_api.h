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

#ifndef MEDIAPIPE_TASKS_CC_AUDIO_CORE_BASE_AUDIO_TASK_API_H_
#define MEDIAPIPE_TASKS_CC_AUDIO_CORE_BASE_AUDIO_TASK_API_H_

#include <memory>
#include <string>
#include <utility>

#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/str_cat.h"
#include "mediapipe/tasks/cc/audio/core/running_mode.h"
#include "mediapipe/tasks/cc/common.h"
#include "mediapipe/tasks/cc/core/base_task_api.h"
#include "mediapipe/tasks/cc/core/task_runner.h"

namespace mediapipe {
namespace tasks {
namespace audio {
namespace core {

// The base class of the user-facing mediapipe audio task api classes.
class BaseAudioTaskApi : public tasks::core::BaseTaskApi {
 public:
  // Constructor.
  explicit BaseAudioTaskApi(std::unique_ptr<tasks::core::TaskRunner> runner,
                            RunningMode running_mode)
      : BaseTaskApi(std::move(runner)), running_mode_(running_mode) {}

 protected:
  // A synchronous method to process independent audio clips.
  // The call blocks the current thread until a failure status or a successful
  // result is returned.
  absl::StatusOr<tasks::core::PacketMap> ProcessAudioClip(
      tasks::core::PacketMap inputs) {
    if (running_mode_ != RunningMode::AUDIO_CLIPS) {
      return CreateStatusWithPayload(
          absl::StatusCode::kInvalidArgument,
          absl::StrCat(
              "Task is not initialized with the audio clips mode. Current "
              "running mode:",
              GetRunningModeName(running_mode_)),
          MediaPipeTasksStatus::kRunnerApiCalledInWrongModeError);
    }
    return runner_->Process(std::move(inputs));
  }

  // An asynchronous method to send audio stream data to the runner. The results
  // will be available in the user-defined results callback.
  absl::Status SendAudioStreamData(tasks::core::PacketMap inputs) {
    if (running_mode_ != RunningMode::AUDIO_STREAM) {
      return CreateStatusWithPayload(
          absl::StatusCode::kInvalidArgument,
          absl::StrCat("Task is not initialized with the audio stream mode. "
                       "Current running mode:",
                       GetRunningModeName(running_mode_)),
          MediaPipeTasksStatus::kRunnerApiCalledInWrongModeError);
    }
    return runner_->Send(std::move(inputs));
  }

  // Checks or sets the sample rate in the audio stream mode.
  absl::Status CheckOrSetSampleRate(std::string sample_rate_stream_name,
                                    double sample_rate) {
    if (running_mode_ != RunningMode::AUDIO_STREAM) {
      return CreateStatusWithPayload(
          absl::StatusCode::kInvalidArgument,
          absl::StrCat("Task is not initialized with the audio stream mode. "
                       "Current running mode:",
                       GetRunningModeName(running_mode_)),
          MediaPipeTasksStatus::kRunnerApiCalledInWrongModeError);
    }
    if (default_sample_rate_ > 0) {
      if (std::fabs(sample_rate - default_sample_rate_) >
          std::numeric_limits<double>::epsilon()) {
        return CreateStatusWithPayload(
            absl::StatusCode::kInvalidArgument,
            absl::StrCat("The input audio sample rate: ", sample_rate,
                         " is inconsistent with the previously provided: ",
                         default_sample_rate_),
            MediaPipeTasksStatus::kInvalidArgumentError);
      }
    } else {
      default_sample_rate_ = sample_rate;
      MP_RETURN_IF_ERROR(runner_->Send(
          {{sample_rate_stream_name, MakePacket<double>(default_sample_rate_)
                                         .At(Timestamp::PreStream())}}));
    }
    return absl::OkStatus();
  }

 private:
  RunningMode running_mode_;
  double default_sample_rate_ = -1.0;
};

}  // namespace core
}  // namespace audio
}  // namespace tasks
}  // namespace mediapipe

#endif  // MEDIAPIPE_TASKS_CC_AUDIO_CORE_BASE_AUDIO_TASK_API_H_
