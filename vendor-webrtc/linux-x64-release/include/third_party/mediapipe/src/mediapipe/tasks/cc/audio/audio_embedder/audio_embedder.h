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

#ifndef MEDIAPIPE_TASKS_CC_AUDIO_AUDIO_EMBEDDER_AUDIO_EMBEDDER_H_
#define MEDIAPIPE_TASKS_CC_AUDIO_AUDIO_EMBEDDER_AUDIO_EMBEDDER_H_

#include <cstdint>
#include <memory>
#include <utility>
#include <vector>

#include "absl/status/statusor.h"
#include "mediapipe/framework/formats/matrix.h"
#include "mediapipe/tasks/cc/audio/core/base_audio_task_api.h"
#include "mediapipe/tasks/cc/audio/core/running_mode.h"
#include "mediapipe/tasks/cc/components/containers/embedding_result.h"
#include "mediapipe/tasks/cc/components/processors/embedder_options.h"
#include "mediapipe/tasks/cc/core/base_options.h"

namespace mediapipe::tasks::audio::audio_embedder {

// Alias the shared EmbeddingResult struct as result type.
using AudioEmbedderResult =
    ::mediapipe::tasks::components::containers::EmbeddingResult;

struct AudioEmbedderOptions {
  // Base options for configuring Task library, such as specifying the TfLite
  // model file with metadata, accelerator options, op resolver, etc.
  tasks::core::BaseOptions base_options;

  // Options for configuring the embedder behavior, such as score threshold,
  // number of results, etc.
  components::processors::EmbedderOptions embedder_options;

  // The running mode of the audio embedder. Default to the audio clips mode.
  // Audio embedder has two running modes:
  // 1) The audio clips mode for running embedding on independent audio clips.
  // 2) The audio stream mode for running embedding on the audio stream,
  //    such as from microphone. In this mode, the "result_callback" below must
  //    be specified to receive the embedding results asynchronously.
  core::RunningMode running_mode = core::RunningMode::AUDIO_CLIPS;

  // The user-defined result callback for processing audio stream data.
  // The result callback should only be specified when the running mode is set
  // to RunningMode::AUDIO_STREAM.
  std::function<void(absl::StatusOr<AudioEmbedderResult>)> result_callback =
      nullptr;
};

// Performs audio embedding extraction on audio clips or audio stream.
//
// This API expects a TFLite model with mandatory TFLite Model Metadata that
// contains the mandatory AudioProperties of the solo input audio tensor and the
// optional (but recommended) label items as AssociatedFiles with type
// TENSOR_AXIS_LABELS per output embedding tensor.
//
// Input tensor:
//   (kTfLiteFloat32)
//    - input audio buffer of size `[batch * samples]`.
//    - batch inference is not supported (`batch` is required to be 1).
//    - for multi-channel models, the channels need be interleaved.
// At least one output tensor with:
//   (kTfLiteUInt8/kTfLiteFloat32)
//    - `N` components corresponding to the `N` dimensions of the returned
//      feature vector for this output layer.
//    - Either 2 or 4 dimensions, i.e. `[1 x N]` or `[1 x 1 x 1 x N]`.
class AudioEmbedder : core::BaseAudioTaskApi {
 public:
  using BaseAudioTaskApi::BaseAudioTaskApi;

  // Creates an AudioEmbedder from the provided options. A non-default
  // OpResolver can be specified in the BaseOptions in order to support custom
  // Ops or specify a subset of built-in Ops.
  static absl::StatusOr<std::unique_ptr<AudioEmbedder>> Create(
      std::unique_ptr<AudioEmbedderOptions> options);

  // Performs embedding extraction on the provided audio clips. Only use this
  // method when the AudioEmbedder is created with the audio clips running mode.
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
  // function returns a vector of EmbeddingResult objects, each associated
  // with a timestamp corresponding to the start (in milliseconds) of the chunk
  // data that was extracted.
  absl::StatusOr<std::vector<AudioEmbedderResult>> Embed(
      Matrix audio_clip, double audio_sample_rate);

  // Sends audio stream data to embedder, and the results will be available via
  // the "result_callback" provided in the AudioEmbedderOptions. Only use this
  // method when the AudioEmbedder is created with the audio stream running
  // mode.
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
  absl::Status EmbedAsync(Matrix audio_block, double audio_sample_rate,
                          int64_t timestamp_ms);

  // Shuts down the AudioEmbedder when all works are done.
  absl::Status Close() { return runner_->Close(); }
};

}  // namespace mediapipe::tasks::audio::audio_embedder

#endif  // MEDIAPIPE_TASKS_CC_AUDIO_AUDIO_EMBEDDER_AUDIO_EMBEDDER_H_
