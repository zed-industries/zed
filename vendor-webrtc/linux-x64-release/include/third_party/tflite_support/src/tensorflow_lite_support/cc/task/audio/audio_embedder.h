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
#ifndef TENSORFLOW_LITE_SUPPORT_CC_TASK_AUDIO_AUDIO_EMBEDDER_H_
#define TENSORFLOW_LITE_SUPPORT_CC_TASK_AUDIO_AUDIO_EMBEDDER_H_

#include <memory>

#include "tensorflow/lite/c/common.h"
#include "tensorflow_lite_support/cc/port/statusor.h"
#include "tensorflow_lite_support/cc/task/audio/proto/audio_embedder_options.pb.h"
#include "tensorflow_lite_support/cc/task/core/base_task_api.h"
#include "tensorflow_lite_support/cc/task/processor/audio_preprocessor.h"
#include "tensorflow_lite_support/cc/task/processor/embedding_postprocessor.h"
#include "tensorflow_lite_support/cc/task/processor/proto/embedding.pb.h"
namespace tflite {
namespace task {
namespace audio {
class AudioEmbedder
    : public tflite::task::core::BaseTaskApi<
          tflite::task::processor::EmbeddingResult, const AudioBuffer&> {
 public:
  // Use base class constructor.
  using BaseTaskApi::BaseTaskApi;

  // Utility function to compute cosine similarity [1] between two feature
  // vectors. May return an InvalidArgumentError if e.g. the feature vectors are
  // of different types (quantized vs. float), have different sizes, or have a
  // an L2-norm of 0.
  //
  // [1]: https://en.wikipedia.org/wiki/Cosine_similarity
  static tflite::support::StatusOr<double> CosineSimilarity(
      const processor::FeatureVector& u, const processor::FeatureVector& v);

  // Creates an AudioEmbedder from the provided options. A non-default
  // OpResolver can be specified in order to support custom Ops or specify a
  // subset of built-in Ops.
  static tflite::support::StatusOr<std::unique_ptr<AudioEmbedder>>
  CreateFromOptions(
      const AudioEmbedderOptions& options,
      std::unique_ptr<tflite::OpResolver> resolver =
          absl::make_unique<tflite::ops::builtin::BuiltinOpResolver>());

  // Performs actual feature vector extraction on the provided AudioBuffer.
  tflite::support::StatusOr<tflite::task::processor::EmbeddingResult> Embed(
      const AudioBuffer& audio_buffer);

  // Returns the dimensionality of the embedding output by the output_index'th
  // output layer. Returns -1 if `output_index` is out of bounds.
  int GetEmbeddingDimension(int output_index) const;

  // Returns the number of output layers of the model.
  int GetNumberOfOutputLayers() const;

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
  static absl::Status SanityCheckOptions(const AudioEmbedderOptions& options);

  absl::Status Init(std::unique_ptr<AudioEmbedderOptions> options);

  // Passes through the input audio buffer into model's input tensor.
  absl::Status Preprocess(const std::vector<TfLiteTensor*>& input_tensors,
                          const AudioBuffer& audio_buffer) override {
    return preprocessor_->Preprocess(audio_buffer);
  }

  // Transforms the raw model outputs into embedding results.
  tflite::support::StatusOr<tflite::task::processor::EmbeddingResult>
  Postprocess(const std::vector<const TfLiteTensor*>& output_tensors,
              const AudioBuffer& audio_buffer) override;

  std::unique_ptr<AudioEmbedderOptions> options_ = nullptr;

  // Processors
  std::unique_ptr<tflite::task::processor::AudioPreprocessor> preprocessor_ =
      nullptr;
  std::vector<std::unique_ptr<tflite::task::processor::EmbeddingPostprocessor>>
      postprocessors_;
};

}  // namespace audio
}  // namespace task
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_CC_TASK_AUDIO_AUDIO_EMBEDDER_H_
