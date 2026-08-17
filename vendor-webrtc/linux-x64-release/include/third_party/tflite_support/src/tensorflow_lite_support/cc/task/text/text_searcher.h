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

#ifndef TENSORFLOW_LITE_SUPPORT_CC_TASK_TEXT_TEXT_SEARCHER_H_
#define TENSORFLOW_LITE_SUPPORT_CC_TASK_TEXT_TEXT_SEARCHER_H_

#include <memory>
#include <vector>

#include "absl/memory/memory.h"  // from @com_google_absl
#include "absl/status/status.h"  // from @com_google_absl
#include "absl/strings/string_view.h"  // from @com_google_absl
#include "tensorflow/lite/c/common.h"
#include "tensorflow/lite/core/api/op_resolver.h"
#include "tensorflow/lite/kernels/register.h"
#include "tensorflow_lite_support/cc/port/statusor.h"
#include "tensorflow_lite_support/cc/task/core/base_task_api.h"
#include "tensorflow_lite_support/cc/task/processor/proto/search_result.pb.h"
#include "tensorflow_lite_support/cc/task/processor/search_postprocessor.h"
#include "tensorflow_lite_support/cc/task/processor/text_preprocessor.h"
#include "tensorflow_lite_support/cc/task/text/proto/text_searcher_options.pb.h"

namespace tflite {
namespace task {
namespace text {

// Performs embedding extraction on text, followed by nearest-neighbor search in
// an index of embeddings through ScaNN.
// TODO(b/223535177): add pointer to README in the scann folder once available.
//
// The API expects a TFLite model with metadata populated. The metadata should
// contain the following information:
// 1. For Bert based TFLite model:
//   - 3 input tensors of type kTfLiteString with names "ids", "mask" and
//   "segment_ids".
//   - input_process_units for Wordpiece/Sentencepiece Tokenizer
//   - exactly one output tensor of type kTfLiteFloat32
// 2. For Regex based TFLite model:
//   - 1 input tensor.
//   - input_process_units for RegexTokenizer Tokenizer
//   - exactly one output tensor of type kTfLiteFloat32
// 3. For Universal Sentence Encoder based TFLite model:
//   - 3 input tensors with names "inp_text", "res_context" and "res_text"
//   - 2 output tensors with names "query_encoding" and "response_encoding" of
//     type kTfLiteFloat32
//
// TODO(b/227746553): create this tool.
// A CLI demo tool is available for easily trying out this API, and provides
// example usage. See:
// examples/task/text/desktop/text_searcher_demo.cc
class TextSearcher
    : public core::BaseTaskApi<tflite::task::processor::SearchResult,
                               const std::string&> {
 public:
  using BaseTaskApi::BaseTaskApi;

  // Creates a TextSearcher from the provided options. A non-default OpResolver
  // can be specified in order to support custom Ops or specify a subset of
  // built-in Ops.
  static tflite::support::StatusOr<std::unique_ptr<TextSearcher>>
  CreateFromOptions(
      const TextSearcherOptions& options,
      std::unique_ptr<tflite::OpResolver> resolver =
          absl::make_unique<tflite::ops::builtin::BuiltinOpResolver>());

  // Performs embedding extraction on the provided text input, followed by
  // nearest-neighbor search in the index.
  tflite::support::StatusOr<tflite::task::processor::SearchResult> Search(
      const std::string& input);

  // Provides access to the opaque user info stored in the index file (if any),
  // in raw binary form. Returns an empty string if the index doesn't contain
  // user info.
  tflite::support::StatusOr<absl::string_view> GetUserInfo();

 protected:
  // The options used to build this TextSearcher.
  std::unique_ptr<TextSearcherOptions> options_;

  // Pre-processing to fill the input tensors.
  absl::Status Preprocess(const std::vector<TfLiteTensor*>& input_tensors,
                          const std::string& input) override;

  // Post-processing to transform the raw model outputs into embeddings, then
  // perform the nearest-neighbor search in the index.
  tflite::support::StatusOr<tflite::task::processor::SearchResult> Postprocess(
      const std::vector<const TfLiteTensor*>& output_tensors,
      const std::string& input) override;

  // Initializes the TextSearcher.
  absl::Status Init(std::unique_ptr<TextSearcherOptions> options);

 private:
  std::unique_ptr<tflite::task::processor::TextPreprocessor> preprocessor_;
  std::unique_ptr<tflite::task::processor::SearchPostprocessor> postprocessor_;
};

}  // namespace text
}  // namespace task
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_CC_TASK_TEXT_TEXT_SEARCHER_H_
