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

#ifndef TENSORFLOW_LITE_SUPPORT_CC_TASK_TEXT_CLU_BERT_CLU_ANNOTATOR_H_
#define TENSORFLOW_LITE_SUPPORT_CC_TASK_TEXT_CLU_BERT_CLU_ANNOTATOR_H_

#include <memory>

#include "tensorflow_lite_support/cc/task/text/clu_annotator.h"
#include "tensorflow_lite_support/cc/task/text/clu_lib/tflite_modules.h"
#include "tensorflow_lite_support/cc/task/text/proto/bert_clu_annotator_options_proto_inc.h"
#include "tensorflow_lite_support/cc/text/tokenizers/bert_tokenizer.h"

namespace tflite {
namespace task {
namespace text {
namespace clu {

// BertCluAnnotator task API, performs tokenization for models (BERT) in
// preprocess and returns CLU annotations.
//
// The API expects a Bert based TFLite model with metadata populated.
// The metadata should contain the following information:
//   - input_process_units for Wordpiece Tokenizer.
//   - 3 input tensors with names "ids", "mask" and "segment_ids".
//   - 6 output tensors with names "domain_task/names", "domain_task/scores",
//   "intent_task/names", "intent_task/scores", "slot_task/names", and
//   "slot_task/scores".
class BertCluAnnotator : public CluAnnotator {
 public:
  static constexpr int kNumLiteThreads = 4;

  // Factory function to create a `BertCluAnnotator` from
  // `BertCluAnnotatorOptions`.
  static tflite::support::StatusOr<std::unique_ptr<CluAnnotator>>
  CreateFromOptions(
      const BertCluAnnotatorOptions& options,
      std::unique_ptr<tflite::OpResolver> resolver =
          std::make_unique<tflite::ops::builtin::BuiltinOpResolver>());

  explicit BertCluAnnotator(std::unique_ptr<core::TfLiteEngine> engine)
      : CluAnnotator(std::move(engine)) {}

  absl::StatusOr<CluResponse> Annotate(const CluRequest& request) override;

 private:
  absl::Status Init(std::unique_ptr<BertCluAnnotatorOptions> options);

  absl::Status Preprocess(const std::vector<TfLiteTensor*>& input_tensors,
                          const CluRequest& request) override;

  tflite::support::StatusOr<CluResponse> Postprocess(
      const std::vector<const TfLiteTensor*>& output_tensors,
      const CluRequest& request) override;

  std::unique_ptr<tflite::support::text::tokenizer::Tokenizer> tokenizer_;

  // A list of modules in topological ordering.
  std::vector<std::unique_ptr<const AbstractModule>> modules_;

  // The artifacts for the current CLU request.
  Artifacts artifacts_;

  std::unique_ptr<BertCluAnnotatorOptions> options_;

  std::unique_ptr<TensorIndexMap> tensor_index_map_;
};

}  // namespace clu
}  // namespace text
}  // namespace task
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_CC_TASK_TEXT_CLU_BERT_CLU_ANNOTATOR_H_
