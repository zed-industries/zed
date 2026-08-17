/* Copyright 2021 The TensorFlow Authors. All Rights Reserved.

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
#ifndef TENSORFLOW_LITE_SUPPORT_CC_TASK_AUDIO_AUDIO_CLASSIFIER_H_
#define TENSORFLOW_LITE_SUPPORT_CC_TASK_AUDIO_AUDIO_CLASSIFIER_H_

#include <memory>

#include "absl/status/status.h"  // from @com_google_absl
#include "tensorflow/lite/core/api/op_resolver.h"
#include "tensorflow/lite/kernels/register.h"
#include "tensorflow_lite_support/cc/port/statusor.h"
#include "tensorflow_lite_support/cc/task/audio/core/audio_buffer.h"
#include "tensorflow_lite_support/cc/task/audio/proto/audio_classifier_options.pb.h"
#include "tensorflow_lite_support/cc/task/audio/proto/classifications_proto_inc.h"
#include "tensorflow_lite_support/cc/task/core/base_task_api.h"
#include "tensorflow_lite_support/cc/task/core/classification_head.h"
#include "tensorflow_lite_support/cc/task/processor/audio_preprocessor.h"
#include "tensorflow_lite_support/cc/task/processor/classification_postprocessor.h"

namespace tflite {
namespace task {
namespace audio {

// Performs classification on audio clips.
//
// This API expects a TFLite model with metadata.
//
// Input tensor:
//   (kTfLiteFloat32)
//    - input audio buffer of size `[batch * samples]`.
//    - batch inference is not supported (`batch` is required to be 1).
//    - for multi-channel models, the channels need be interleaved.
// At least one output tensor with:
//   (kTfLiteFloat32)
//    - `[1 x N]` array with `N` represents the class number.
//    - optional (but recommended) label map(s) as AssociatedFile-s with type
//      TENSOR_AXIS_LABELS, containing one label per line. The first such
//      AssociatedFile (if any) is used to fill the `class_name` field of the
//      results. The `display_name` field is filled from the AssociatedFile (if
//      any) whose locale matches the `display_names_locale` field of the
//      `ImageClassifierOptions` used at creation time ("en" by default, i.e.
//      English). If none of these are available, only the `index` field of the
//      results will be filled.
//
// An example of such model can be found at:
// https://tfhub.dev/google/lite-model/yamnet/classification/tflite/1

// A CLI demo tool is available for easily trying out this API, and provides
// example usage. See:
// https://github.com/tensorflow/tflite-support/tree/master/tensorflow_lite_support/examples/task/audio/desktop
class AudioClassifier
    : public tflite::task::core::BaseTaskApi<ClassificationResult,
                                             const AudioBuffer&> {
 public:
  using BaseTaskApi::BaseTaskApi;

  // Creates an AudioClassifier from the provided options. A non-default
  // OpResolver can be specified in order to support custom Ops or specify a
  // subset of built-in Ops.
  static tflite::support::StatusOr<std::unique_ptr<AudioClassifier>>
  CreateFromOptions(
      const AudioClassifierOptions& options,
      std::unique_ptr<tflite::OpResolver> resolver =
          absl::make_unique<tflite::ops::builtin::BuiltinOpResolver>());

  // Performs classification on the provided audio buffer.
  //
  // The input `audio_buffer` are the raw buffer captured by the required format
  // which can retrieved by GetRequiredAudioFormat().
  tflite::support::StatusOr<ClassificationResult> Classify(
      const AudioBuffer& audio_buffer);

  // Returns the required input audio format if it is set. Otherwise, returns
  // kMetadataNotFoundError.
  // TODO(b/182625132): Add unit test after the format is populated from model
  // metadata.
  tflite::support::StatusOr<AudioBuffer::AudioFormat> GetRequiredAudioFormat() {
    return preprocessor_->GetRequiredAudioFormat();
  }

  // Returns the required input buffer size in number of float elements.
  int GetRequiredInputBufferSize() {
    return preprocessor_->GetRequiredInputBufferSize();
  }

 private:
  // Performs sanity checks on the provided AudioClassifierOptions.
  static absl::Status SanityCheckOptions(const AudioClassifierOptions& options);

  // Initializes the AudioClassifier from the provided AudioClassifierOptions,
  // whose ownership is transferred to this object.
  absl::Status Init(std::unique_ptr<AudioClassifierOptions> options);

  // Passes through the input audio buffer into model's input tensor.
  absl::Status Preprocess(const std::vector<TfLiteTensor*>& input_tensors,
                          const AudioBuffer& audio_buffer) override {
    return preprocessor_->Preprocess(audio_buffer);
  }

  // Post-processing to transform the raw model outputs into classification
  // results.
  tflite::support::StatusOr<ClassificationResult> Postprocess(
      const std::vector<const TfLiteTensor*>& output_tensors,
      const AudioBuffer& audio_buffer) override;

  // The options used to build this AudioClassifier.
  std::unique_ptr<AudioClassifierOptions> options_;

  std::unique_ptr<tflite::task::processor::AudioPreprocessor> preprocessor_ =
      nullptr;

  std::vector<
      std::unique_ptr<tflite::task::processor::ClassificationPostprocessor>>
      postprocessors_;
};

}  // namespace audio
}  // namespace task
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_CC_TASK_AUDIO_AUDIO_CLASSIFIER_H_
