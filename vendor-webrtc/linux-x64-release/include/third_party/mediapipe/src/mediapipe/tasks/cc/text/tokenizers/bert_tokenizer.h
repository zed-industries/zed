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

#ifndef MEDIAPIPE_TASKS_CC_TEXT_TOKENIZERS_BERT_TOKENIZER_H_
#define MEDIAPIPE_TASKS_CC_TEXT_TOKENIZERS_BERT_TOKENIZER_H_

#include <cstddef>
#include <fstream>
#include <string>
#include <vector>

#include "absl/container/flat_hash_map.h"
#include "absl/strings/string_view.h"
#include "mediapipe/tasks/cc/text/tokenizers/tokenizer.h"
#include "mediapipe/tasks/cc/text/utils/vocab_utils.h"
#include "re2/re2.h"
#include "tensorflow_text/core/kernels/wordpiece_tokenizer.h"

namespace mediapipe {
namespace tasks {
namespace text {
namespace tokenizers {

inline constexpr char kDefaultDelimRe[] =
    R"((\s+|[!-/]|[:-@]|[\[-`]|[{-~]|[\p{P}]|[\x{4E00}-\x{9FFF}]|[\x{3400}-\x{4DBF}]|[\x{20000}-\x{2A6DF}]|[\x{2A700}-\x{2B73F}]|[\x{2B740}-\x{2B81F}]|[\x{2B820}-\x{2CEAF}]|[\x{F900}-\x{FAFF}]|[\x{2F800}-\x{2FA1F}]))";
inline constexpr char kDefaultIncludeDelimRe[] =
    R"(([!-/]|[:-@]|[\[-`]|[{-~]|[\p{P}]|[\x{4E00}-\x{9FFF}]|[\x{3400}-\x{4DBF}]|[\x{20000}-\x{2A6DF}]|[\x{2A700}-\x{2B73F}]|[\x{2B740}-\x{2B81F}]|[\x{2B820}-\x{2CEAF}]|[\x{F900}-\x{FAFF}]|[\x{2F800}-\x{2FA1F}]))";
inline constexpr int kDefaultMaxBytesPerToken = 100;
inline constexpr int kDefaultMaxCharsPerSubToken = 100;
inline constexpr char kDefaultSuffixIndicator[] = "##";
inline constexpr bool kDefaultUseUnknownToken = true;
inline constexpr char kDefaultUnknownToken[] = "[UNK]";
inline constexpr bool kDefaultSplitUnknownChars = false;

// Result of wordpiece tokenization including subwords and offsets.
// Example:
// input:                tokenize     me  please
// subwords:             token ##ize  me  plea ##se
// wp_begin_offset:     [0,      5,   9,  12,    16]
// wp_end_offset:       [     5,    8,  11,   16,  18]
// row_lengths:         [2,          1,  1]
struct WordpieceTokenizerResult : TokenizerResult {
  std::vector<int> wp_begin_offset;
  std::vector<int> wp_end_offset;
  std::vector<int> row_lengths;
};
// Options to create a BertTokenizer.
struct BertTokenizerOptions {
  int max_bytes_per_token = kDefaultMaxBytesPerToken;
  int max_chars_per_subtoken = kDefaultMaxCharsPerSubToken;
  std::string suffix_indicator = kDefaultSuffixIndicator;
  bool use_unknown_token = kDefaultUseUnknownToken;
  std::string unknown_token = kDefaultUnknownToken;
  bool split_unknown_chars = kDefaultSplitUnknownChars;
  std::string delim_str = kDefaultDelimRe;
  std::string include_delim_str = kDefaultIncludeDelimRe;
};

// A flat-hash-map based implementation of WordpieceVocab, used in
// BertTokenizer to invoke tensorflow::text::WordpieceTokenize within.
class FlatHashMapBackedWordpiece : public tensorflow::text::WordpieceVocab {
 public:
  explicit FlatHashMapBackedWordpiece(const std::vector<std::string>& vocab);

  tensorflow::text::LookupStatus Contains(absl::string_view key,
                                          bool* value) const override;
  bool LookupId(absl::string_view key, int* result) const;
  bool LookupWord(int vocab_id, absl::string_view* result) const;
  int VocabularySize() const { return vocab_.size(); }

 private:
  // All words indexed position in vocabulary file.
  std::vector<std::string> vocab_;
  absl::flat_hash_map<absl::string_view, int> index_map_;
};

// Wordpiece tokenizer for bert models. Initialized with a vocab file or vector.
class BertTokenizer : public mediapipe::tasks::text::tokenizers::Tokenizer {
 public:
  // Initialize the tokenizer from vocab vector and tokenizer configs.
  explicit BertTokenizer(const std::vector<std::string>& vocab,
                         const BertTokenizerOptions& options = {})
      : vocab_{FlatHashMapBackedWordpiece(vocab)},
        options_{options},
        delim_re_{options.delim_str},
        include_delim_re_{options.include_delim_str} {}

  // Initialize the tokenizer from file path to vocab and tokenizer configs.
  explicit BertTokenizer(const std::string& path_to_vocab,
                         const BertTokenizerOptions& options = {})
      : BertTokenizer(mediapipe::tasks::text::LoadVocabFromFile(path_to_vocab),
                      options) {}

  // Initialize the tokenizer from buffer and size of vocab and tokenizer
  // configs.
  BertTokenizer(const char* vocab_buffer_data, size_t vocab_buffer_size,
                const BertTokenizerOptions& options = {})
      : BertTokenizer(mediapipe::tasks::text::LoadVocabFromBuffer(
                          vocab_buffer_data, vocab_buffer_size),
                      options) {}

  // Perform tokenization, return tokenized results containing the subwords.
  TokenizerResult Tokenize(const std::string& input) override;

  // Perform tokenization, return wordpiece-specific tokenized result including
  // subwords and offsets
  WordpieceTokenizerResult TokenizeWordpiece(const std::string& input) const;

  // Check if a certain key is included in the vocab.
  tensorflow::text::LookupStatus Contains(const absl::string_view key,
                                          bool* value) const {
    return vocab_.Contains(key, value);
  }

  // Find the id of a wordpiece.
  bool LookupId(absl::string_view key, int* result) const override {
    return vocab_.LookupId(key, result);
  }

  // Find the wordpiece from an id.
  bool LookupWord(int vocab_id, absl::string_view* result) const override {
    return vocab_.LookupWord(vocab_id, result);
  }

  int VocabularySize() const { return vocab_.VocabularySize(); }

 private:
  mediapipe::tasks::text::tokenizers::FlatHashMapBackedWordpiece vocab_;
  BertTokenizerOptions options_;
  RE2 delim_re_;
  RE2 include_delim_re_;
};

}  // namespace tokenizers
}  // namespace text
}  // namespace tasks
}  // namespace mediapipe

#endif  // MEDIAPIPE_TASKS_CC_TEXT_TOKENIZERS_BERT_TOKENIZER_H_
