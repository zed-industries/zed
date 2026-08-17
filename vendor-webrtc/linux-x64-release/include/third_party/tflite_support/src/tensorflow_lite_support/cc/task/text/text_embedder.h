/* Copyright 2022 The TensorFlow Authors. All Rights Reserved.

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

#ifndef TENSORFLOW_LITE_SUPPORT_CC_TASK_TEXT_TEXT_EMBEDDER_H_
#define TENSORFLOW_LITE_SUPPORT_CC_TASK_TEXT_TEXT_EMBEDDER_H_

#include <string>
#include <vector>

#include "absl/status/status.h"  // from @com_google_absl
#include "tensorflow/lite/core/api/op_resolver.h"
#include "tensorflow/lite/kernels/register.h"
#include "tensorflow_lite_support/cc/port/statusor.h"
#include "tensorflow_lite_support/cc/task/core/base_task_api.h"
#include "tensorflow_lite_support/cc/task/processor/embedding_postprocessor.h"
#include "tensorflow_lite_support/cc/task/processor/proto/embedding.pb.h"
#include "tensorflow_lite_support/cc/task/processor/proto/embedding_options.pb.h"
#include "tensorflow_lite_support/cc/task/processor/text_preprocessor.h"
#include "tensorflow_lite_support/cc/task/text/proto/text_embedder_options.pb.h"

namespace tflite {
namespace task {
namespace text {

// Performs dense feature vector extraction on text.
//
// The API expects a TFLite model with metadata populated. The metadata should
// contain the following information:
// 1. For Bert based TFLite model:
//   - 3 input tensors of type kTfLiteString with names "ids", "mask" and
//   "segment_ids".
//   - input_process_units for Wordpiece/Sentencepiece Tokenizer
//   - one or more output tensors of type kTfLiteFloat32
// 2. For Regex based TFLite model:
//   - 1 input tensor.
//   - input_process_units for RegexTokenizer Tokenizer
//   - one or more output tensors of type kTfLiteFloat32
// 3. For Universal Sentence Encoder based TFLite model:
//   - 3 input tensors with names "inp_text", "res_context" and "res_text"
//   - 2 output tensors with names "query_encoding" and "response_encoding" of
//     type kTfLiteFloat32
class TextEmbedder
    : public core::BaseTaskApi<processor::EmbeddingResult, const std::string&> {
 public:
  // Use base class constructor.
  using BaseTaskApi::BaseTaskApi;

  // Creates a TextEmbedder from the provided options. A non-default
  // OpResolver can be specified in order to support custom Ops or specify a
  // subset of built-in Ops.
  static tflite::support::StatusOr<std::unique_ptr<TextEmbedder>>
  CreateFromOptions(
      const TextEmbedderOptions& options,
      std::unique_ptr<tflite::OpResolver> resolver =
          absl::make_unique<tflite::ops::builtin::BuiltinOpResolver>());

  // Performs actual feature vector extraction on the provided raw text.
  tflite::support::StatusOr<processor::EmbeddingResult> Embed(
      const std::string& text);

  // Returns the dimensionality of the embedding output by the output_index'th
  // output layer. Returns -1 if `output_index` is out of bounds.
  int GetEmbeddingDimension(int output_index) const;

  // Returns the number of output layers of the model.
  int GetNumberOfOutputLayers() const;

  // Utility function to compute cosine similarity [1] between two feature
  // vectors. May return an InvalidArgumentError if e.g. the feature vectors are
  // of different types (quantized vs. float), have different sizes, or have a
  // an L2-norm of 0.
  //
  // [1]: https://en.wikipedia.org/wiki/Cosine_similarity
  static tflite::support::StatusOr<double> CosineSimilarity(
      const processor::FeatureVector& u, const processor::FeatureVector& v);

 protected:
  // The options used to build this TextEmbedder.
  std::unique_ptr<TextEmbedderOptions> options_;

  // Passes through the input raw text into model's input tensor.
  absl::Status Preprocess(const std::vector<TfLiteTensor*>& input_tensors,
                          const std::string& input) override;

  // Post-processing to transform the raw model outputs into embedding results.
  tflite::support::StatusOr<processor::EmbeddingResult> Postprocess(
      const std::vector<const TfLiteTensor*>& output_tensors,
      const std::string& input) override;

  // Initializes the TextEmbedder.
  absl::Status Init(std::unique_ptr<TextEmbedderOptions> options);

 private:
  std::unique_ptr<tflite::task::processor::TextPreprocessor> preprocessor_ =
      nullptr;
  std::vector<std::unique_ptr<processor::EmbeddingPostprocessor>>
      postprocessors_;
};

}  // namespace text
}  // namespace task
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_CC_TASK_TEXT_TEXT_EMBEDDER_H_
