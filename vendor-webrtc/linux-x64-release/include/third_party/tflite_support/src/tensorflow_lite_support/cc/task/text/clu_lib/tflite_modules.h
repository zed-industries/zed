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

#ifndef TENSORFLOW_LITE_SUPPORT_CC_TASK_TEXT_CLU_LIB_TFLITE_MODULES_H_
#define TENSORFLOW_LITE_SUPPORT_CC_TASK_TEXT_CLU_LIB_TFLITE_MODULES_H_

#include "absl/status/statusor.h"  // from @com_google_absl
#include "absl/strings/string_view.h"  // from @com_google_absl
#include "tensorflow_lite_support/cc/task/core/tflite_engine.h"
#include "tensorflow_lite_support/cc/task/text/proto/bert_clu_annotator_options_proto_inc.h"
#include "tensorflow_lite_support/cc/task/text/proto/clu_proto_inc.h"
#include "tensorflow_lite_support/cc/text/tokenizers/bert_tokenizer.h"

namespace tflite::task::text::clu {

// Artifacts used by modules.
struct Artifacts {
  std::vector<std::pair<int, int>> token_alignments;
  std::vector<absl::string_view> reverse_utterance_list_to_encode;
  std::vector<int> token_turn_ids;
  std::vector<int> first_subword_indicators;

  void Clear() {
    token_alignments.clear();
    reverse_utterance_list_to_encode.clear();
    token_turn_ids.clear();
    first_subword_indicators.clear();
  }
};

// Tensor index used in different TFLite modules.
struct TensorIndexMap {
  int token_id_idx;
  int token_mask_idx;
  int token_type_id_idx;
  int domain_names_idx;
  int domain_scores_idx;
  int intent_names_idx;
  int intent_scores_idx;
  int slot_names_idx;
  int slot_scores_idx;
};

// A super class for `modules` which do feature extraction on the input proto
// and convert the output tensors into the output response proto
class AbstractModule {
 public:
  AbstractModule(const AbstractModule& other) = delete;
  AbstractModule& operator=(const AbstractModule& rhs) = delete;
  virtual ~AbstractModule() = default;

  // Populates the Interpreter input tensors from the Request proto.
  virtual absl::Status Preprocess(const CluRequest& request,
                                  Artifacts* artifacts) const {
    return absl::OkStatus();
  }

  // Reads the output tensors and populates the Response proto.
  virtual absl::Status Postprocess(Artifacts* artifacts,
                                   CluResponse* response) const {
    return absl::OkStatus();
  }

 protected:
  AbstractModule() = default;

  absl::Status Init(core::TfLiteEngine::Interpreter* interpreter,
                    const BertCluAnnotatorOptions* options);

  using NamesAndConfidences =
      std::tuple<std::vector<absl::string_view>, const float*>;
  // Reads a sequence of strings, confidence scores and their size from the
  // output tensors.
  // The tensors are assumed to be of shape [1, max_seq_len]
  absl::StatusOr<NamesAndConfidences> NamesAndConfidencesFromOutput(
      int names_tensor_idx, int scores_tensor_idx) const;

  // TFLite interpreter
  core::TfLiteEngine::Interpreter* interpreter_ = nullptr;

  const TensorIndexMap* tensor_index_map_ = nullptr;
};

// Below are modules used in the TFLite code path.

class UtteranceSeqModule : public AbstractModule {
 public:
  static absl::StatusOr<std::unique_ptr<AbstractModule>> Create(
      core::TfLiteEngine::Interpreter* interpreter,
      const TensorIndexMap* tensor_index_map,
      const BertCluAnnotatorOptions* options,
      const tflite::support::text::tokenizer::BertTokenizer* tokenizer);

  absl::Status Preprocess(const CluRequest& request,
                          Artifacts* artifacts) const override;

 private:
  // The length of the input sequence as required by the model.
  size_t max_seq_len_;
  // The maximum number of previous turns to consider. Used in BERT-DeepCLU.
  int max_history_turns_;
  const tflite::support::text::tokenizer::BertTokenizer* tokenizer_ = nullptr;
};

class DomainModule : public AbstractModule {
 public:
  static absl::StatusOr<std::unique_ptr<AbstractModule>> Create(
      core::TfLiteEngine::Interpreter* interpreter,
      const TensorIndexMap* tensor_index_map,
      const BertCluAnnotatorOptions* options);

  absl::Status Postprocess(Artifacts* artifacts,
                           CluResponse* response) const override;

 private:
  float domain_threshold_;
};

// Responsible for intents and categorical slots.
class IntentModule : public AbstractModule {
 public:
  static absl::StatusOr<std::unique_ptr<AbstractModule>> Create(
      core::TfLiteEngine::Interpreter* interpreter,
      const TensorIndexMap* tensor_index_map,
      const BertCluAnnotatorOptions* options);

  absl::Status Postprocess(Artifacts* artifacts,
                           CluResponse* response) const override;

 private:
  float intent_threshold_;
  float categorical_slot_threshold_;
};

// Responsible for mentioned slots.
class SlotModule : public AbstractModule {
 public:
  static absl::StatusOr<std::unique_ptr<AbstractModule>> Create(
      core::TfLiteEngine::Interpreter* interpreter,
      const TensorIndexMap* tensor_index_map,
      const BertCluAnnotatorOptions* options);

  absl::Status Postprocess(Artifacts* artifacts,
                           CluResponse* response) const override;

 private:
  float mentioned_slot_threshold_;
};

}  // namespace tflite::task::text::clu

#endif  // TENSORFLOW_LITE_SUPPORT_CC_TASK_TEXT_CLU_LIB_TFLITE_MODULES_H_
