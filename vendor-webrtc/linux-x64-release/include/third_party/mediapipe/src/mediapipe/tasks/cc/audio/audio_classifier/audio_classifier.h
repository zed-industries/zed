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

#ifndef MEDIAPIPE_TASKS_CC_AUDIO_AUDIO_CLASSIFIER_AUDIO_CLASSIFIER_H_
#define MEDIAPIPE_TASKS_CC_AUDIO_AUDIO_CLASSIFIER_AUDIO_CLASSIFIER_H_

#include <cstdint>
#include <memory>
#include <utility>
#include <vector>

#include "absl/status/statusor.h"
#include "mediapipe/framework/formats/matrix.h"
#include "mediapipe/tasks/cc/audio/core/base_audio_task_api.h"
#include "mediapipe/tasks/cc/audio/core/running_mode.h"
#include "mediapipe/tasks/cc/components/containers/classification_result.h"
#include "mediapipe/tasks/cc/components/processors/classifier_options.h"
#include "mediapipe/tasks/cc/core/base_options.h"

namespace mediapipe {
namespace tasks {
namespace audio {
namespace audio_classifier {

// Alias the shared ClassificationResult struct as result type.
using AudioClassifierResult =
    ::mediapipe::tasks::components::containers::ClassificationResult;

// The options for configuring a mediapipe audio classifier task.
struct AudioClassifierOptions {
  // Base options for configuring Task library, such as specifying the TfLite
  // model file with metadata, accelerator options, op resolver, etc.
  tasks::core::BaseOptions base_options;

  // Options for configuring the classifier behavior, such as score threshold,
  // number of results, etc.
  components::processors::ClassifierOptions classifier_options;

  // The running mode of the audio classifier. Default to the audio clips mode.
  // Audio classifier has two running modes:
  // 1) The audio clips mode for running classification on independent audio
  //    clips.
  // 2) The audio stream mode for running classification on the audio stream,
  //    such as from microphone. In this mode, the "result_callback" below must
  //    be specified to receive the classification results asynchronously.
  core::RunningMode running_mode = core::RunningMode::AUDIO_CLIPS;

  // The user-defined result callback for processing audio stream data.
  // The result callback should only be specified when the running mode is set
  // to RunningMode::AUDIO_STREAM.
  std::function<void(absl::StatusOr<AudioClassifierResult>)> result_callback =
      nullptr;
};

// Performs audio classification on audio clips or audio stream.
//
// This API expects a TFLite model with mandatory TFLite Model Metadata that
// contains the mandatory AudioProperties of the solo input audio tensor and the
// optional (but recommended) label items as AssociatedFiles with type
// TENSOR_AXIS_LABELS per output classification tensor.
//
// Input tensor:
//   (kTfLiteFloat32)
//    - input audio buffer of size `[batch * samples]`.
//    - batch inference is not supported (`batch` is required to be 1).
//    - for multi-channel models, the channels need be interleaved.
// At least one output tensor with:
//   (kTfLiteFloat32)
//    - `[1 x N]` array with `N` represents the number of categories.
//    - optional (but recommended) label items as AssociatedFiles with type
//      TENSOR_AXIS_LABELS, containing one label per line. The first such
//      AssociatedFile (if any) is used to fill the `category_name` field of the
//      results. The `display_name` field is filled from the AssociatedFile (if
//      any) whose locale matches the `display_names_locale` field of the
//      `AudioClassifierOptions` used at creation time ("en" by default, i.e.
//      English). If none of these are available, only the `index` field of the
//      results will be filled.
// TODO: Create an audio container to replace the matrix, the
// sample rate, and the timestamp.
class AudioClassifier : tasks::audio::core::BaseAudioTaskApi {
 public:
  using BaseAudioTaskApi::BaseAudioTaskApi;

  // Creates an AudioClassifier to process either audio clips (e.g., audio
  // files) or audio stream data (e.g., microphone live input). Audio classifier
  // can be created with one of following two running modes:
  // 1) Audio clips mode for running audio classification on audio clips.
  //    Users feed audio clips to the `Classify` method, and will
  //    receive the classification results as the return value.
  // 2) Audio stream mode for running audio classification on the audio stream,
  //    such as from microphone. Users call `ClassifyAsync` to push the audio
  //    data into the AudioClassifier, the classification results will be
  //    available in the result callback when the audio classifier finishes the
  //    work.
  static absl::StatusOr<std::unique_ptr<AudioClassifier>> Create(
      std::unique_ptr<AudioClassifierOptions> options);

  // Performs audio classification on the provided audio clip. Only use this
  // method when the AudioClassifier is created with the audio clips running
  // mode.
  //
  // The audio clip is represented as a MediaPipe Matrix that has the number of
  // channels rows and the number of samples per channel columns. The method
  // accepts audio clips with various length and audio sample rate. It's
  // required to provide the corresponding audio sample rate along with the
  // input audio clips.
  //
  // The input audio clip may be longer than what the model is able to process
  // in a single inference. When this occurs, the input audio clip is split into
  // multiple chunks starting at different timestamps. For this reason, this
  // function returns a vector of ClassificationResult objects, each associated
  // with a timestamp corresponding to the start (in milliseconds) of the chunk
  // data that was classified, e.g:
  //
  // ClassificationResult #0 (first chunk of data):
  //  timestamp_ms: 0 (starts at 0ms)
  //  classifications #0 (single head model):
  //   category #0:
  //    category_name: "Speech"
  //    score: 0.6
  //   category #1:
  //    category_name: "Music"
  //    score: 0.2
  // ClassificationResult #1 (second chunk of data):
  //  timestamp_ms: 800 (starts at 800ms)
  //  classifications #0 (single head model):
  //   category #0:
  //    category_name: "Speech"
  //    score: 0.5
  //   category #1:
  //    category_name: "Silence"
  //    score: 0.1
  //  ...
  //
  // TODO: Use `sample_rate` in AudioClassifierOptions by default
  // and makes `audio_sample_rate` optional.
  absl::StatusOr<std::vector<AudioClassifierResult>> Classify(
      mediapipe::Matrix audio_clip, double audio_sample_rate);

  // Sends audio data (a block in a continuous audio stream) to perform audio
  // classification. Only use this method when the AudioClassifier is created
  // with the audio stream running mode.
  //
  // The audio block is represented as a MediaPipe Matrix that has the number
  // of channels rows and the number of samples per channel columns. The audio
  // data will be resampled, accumulated, and framed to the proper size for the
  // underlying model to consume. It's required to provide the corresponding
  // audio sample rate along with the input audio block as well as a timestamp
  // (in milliseconds) to indicate the start time of the input audio block. The
  // timestamps must be monotonically increasing.
  //
  // The input audio block may be longer than what the model is able to process
  // in a single inference. When this occurs, the input audio block is split
  // into multiple chunks. For this reason, the callback may be called multiple
  // times (once per chunk) for each call to this function.
  absl::Status ClassifyAsync(mediapipe::Matrix audio_block,
                             double audio_sample_rate, int64_t timestamp_ms);

  // Shuts down the AudioClassifier when all works are done.
  absl::Status Close() { return runner_->Close(); }
};

}  // namespace audio_classifier
}  // namespace audio
}  // namespace tasks
}  // namespace mediapipe

#endif  // MEDIAPIPE_TASKS_CC_AUDIO_AUDIO_CLASSIFIER_AUDIO_CLASSIFIER_H_
