/* Copyright 2020 The TensorFlow Authors. All Rights Reserved.

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

#ifndef TENSORFLOW_LITE_SUPPORT_CC_TASK_QA_BERT_QUESTION_ANSWERER_H_
#define TENSORFLOW_LITE_SUPPORT_CC_TASK_QA_BERT_QUESTION_ANSWERER_H_

#include "absl/base/macros.h"  // from @com_google_absl
#include "absl/container/flat_hash_map.h"  // from @com_google_absl
#include "absl/status/status.h"  // from @com_google_absl
#include "tensorflow_lite_support/cc/port/statusor.h"
#include "tensorflow_lite_support/cc/task/core/base_task_api.h"
#include "tensorflow_lite_support/cc/task/core/task_api_factory.h"
#include "tensorflow_lite_support/cc/task/core/tflite_engine.h"
#include "tensorflow_lite_support/cc/task/text/proto/bert_question_answerer_options_proto_inc.h"
#include "tensorflow_lite_support/cc/task/text/question_answerer.h"
#include "tensorflow_lite_support/cc/text/tokenizers/bert_tokenizer.h"
#include "tensorflow_lite_support/cc/text/tokenizers/sentencepiece_tokenizer.h"

namespace tflite {
namespace task {
namespace text {

// BertQA task API, performs tokenization for models (BERT, Albert, etc.) in
// preprocess and returns most possible answers.
//
// In particular, the branch of BERT models use WordPiece tokenizer, and the
// branch of Albert models use SentencePiece tokenizer, respectively.
//
// The API expects a Bert based TFLite model with metadata populated.
// The metadata should contain the following information:
//   - input_process_units for Wordpiece/Sentencepiece Tokenizer. Wordpiece
//         Tokenizer can be used for a MobileBert[0] model, Sentencepiece
//         Tokenizer Tokenizer can be used for an Albert[1] model.
//   - 3 input tensors with names "ids", "mask" and "segment_ids".
//   - 2 output tensors with names "end_logits" and "start_logits".
//   [0]: https://tfhub.dev/tensorflow/lite-model/mobilebert/1/default/1
//   [1]: https://tfhub.dev/tensorflow/lite-model/albert_lite_base/squadv1/1
//
// See the public documentation for more information:
// https://www.tensorflow.org/lite/inference_with_metadata/task_library/bert_question_answerer

class BertQuestionAnswerer : public QuestionAnswerer {
 public:
  // TODO(b/150904655): add support to parameterize.
  static constexpr int kMaxQueryLen = 64;
  static constexpr int kMaxSeqLen = 384;
  static constexpr int kPredictAnsNum = 5;
  static constexpr int kMaxAnsLen = 32;
  // TODO(b/151954803): clarify the offset usage
  static constexpr int kOutputOffset = 1;
  static constexpr int kNumLiteThreads = 4;
  static constexpr bool kUseLowerCase = true;

  // Factory function to create a `BertQuestionAnswerer` from
  // `BertQuestionAnswererOptions`.
  static tflite::support::StatusOr<std::unique_ptr<QuestionAnswerer>>
  CreateFromOptions(
      const BertQuestionAnswererOptions& options,
      std::unique_ptr<tflite::OpResolver> resolver =
          absl::make_unique<tflite::ops::builtin::BuiltinOpResolver>());

  ABSL_DEPRECATED("Prefer using `CreateFromOptions`")
  static tflite::support::StatusOr<std::unique_ptr<QuestionAnswerer>>
  CreateFromFile(const std::string& path_to_model_with_metadata);

  ABSL_DEPRECATED("Prefer using `CreateFromOptions`")
  static tflite::support::StatusOr<std::unique_ptr<QuestionAnswerer>>
  CreateFromBuffer(const char* model_with_metadata_buffer_data,
                   size_t model_with_metadata_buffer_size);

  ABSL_DEPRECATED("Prefer using `CreateFromOptions`")
  static tflite::support::StatusOr<std::unique_ptr<QuestionAnswerer>>
  CreateFromFd(int fd);

  ABSL_DEPRECATED("Prefer using `CreateFromOptions`")
  static tflite::support::StatusOr<std::unique_ptr<QuestionAnswerer>>
  CreateBertQuestionAnswererFromFile(const std::string& path_to_model,
                                     const std::string& path_to_vocab);

  ABSL_DEPRECATED("Prefer using `CreateFromOptions`")
  static tflite::support::StatusOr<std::unique_ptr<QuestionAnswerer>>
  CreateBertQuestionAnswererFromBuffer(const char* model_buffer_data,
                                       size_t model_buffer_size,
                                       const char* vocab_buffer_data,
                                       size_t vocab_buffer_size);

  ABSL_DEPRECATED("Prefer using `CreateFromOptions`")
  static tflite::support::StatusOr<std::unique_ptr<QuestionAnswerer>>
  CreateAlbertQuestionAnswererFromFile(const std::string& path_to_model,
                                       const std::string& path_to_spmodel);

  ABSL_DEPRECATED("Prefer using `CreateFromOptions`")
  static tflite::support::StatusOr<std::unique_ptr<QuestionAnswerer>>
  CreateAlbertQuestionAnswererFromBuffer(const char* model_buffer_data,
                                         size_t model_buffer_size,
                                         const char* spmodel_buffer_data,
                                         size_t spmodel_buffer_size);

  explicit BertQuestionAnswerer(std::unique_ptr<core::TfLiteEngine> engine)
      : QuestionAnswerer(std::move(engine)) {}

  // Answers question based on the context. Could be empty if no answer was
  // found from the given context.
  std::vector<QaAnswer> Answer(const std::string& context,
                               const std::string& question) override;

 private:
  absl::Status Preprocess(const std::vector<TfLiteTensor*>& input_tensors,
                          const std::string& lowercased_context,
                          const std::string& lowercased_query) override;

  tflite::support::StatusOr<std::vector<QaAnswer>> Postprocess(
      const std::vector<const TfLiteTensor*>& output_tensors,
      const std::string& lowercased_context,
      const std::string& lowercased_query) override;

  // Initialize API with a BertTokenizer from the vocabulary file.
  void InitializeBertTokenizer(const std::string& path_to_vocab);
  // Initialize API with a BertTokenizer from the vocabulary buffer.
  void InitializeBertTokenizerFromBinary(const char* vocab_buffer_data,
                                         size_t vocab_buffer_size);

  // Initialize API with a SentencepieceTokenizer from the model file.
  void InitializeSentencepieceTokenizer(const std::string& path_to_spmodel);
  // Initialize API with a SentencepieceTokenizer from the model buffer.
  void InitializeSentencepieceTokenizerFromBinary(
      const char* spmodel_buffer_data, size_t spmodel_buffer_size);

  // Initialize the API with the tokenizer set in the metadata.
  absl::Status InitializeFromMetadata(
      std::unique_ptr<BertQuestionAnswererOptions> options);

  std::string ConvertIndexToString(int start, int end);

  std::unique_ptr<tflite::support::text::tokenizer::Tokenizer> tokenizer_;
  // Maps index of input token to index of untokenized word from original input.
  absl::flat_hash_map<size_t, size_t> token_to_orig_map_;
  // Original tokens of context.
  std::vector<std::string> orig_tokens_;
  std::unique_ptr<BertQuestionAnswererOptions> options_;
};

}  // namespace text
}  // namespace task
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_CC_TASK_QA_BERT_QUESTION_ANSWERER_H_
