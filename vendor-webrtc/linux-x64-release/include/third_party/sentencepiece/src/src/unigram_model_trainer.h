// Copyright 2016 Google Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.!

#ifndef UNIGRAM_MODEL_TRAINER_H_
#define UNIGRAM_MODEL_TRAINER_H_

#include <memory>
#include <string>
#include <utility>
#include <vector>

#include "absl/strings/string_view.h"
#include "sentencepiece_model.pb.h"
#include "trainer_interface.h"
#include "unigram_model.h"
#include "util.h"

namespace sentencepiece {
namespace unigram {

using string_util::UnicodeText;

class TrainerModel : public Model {
 public:
  using SentencePieces = std::vector<std::pair<std::string, float>>;

  TrainerModel() {}
  TrainerModel(const ModelProto &model_proto) = delete;
  TrainerModel(const TrainerSpec &trainer_spec,
               const NormalizerSpec &normalizaiton_spec);
  ~TrainerModel() override;

  // Returns the sentencepieces.
  // The meta symbols, e.g., </s> are NOT included.
  const SentencePieces &GetSentencePieces() const;

  // Sets sentencepieces. The sentencepieces are moved.
  // The meta symbols, e.g., </s> are NOT included.
  void SetSentencePieces(SentencePieces &&sentencepieces);

  EncodeResult Encode(absl::string_view normalized) const override {
    return {};
  }

 private:
  SentencePieces sentencepieces_;
  TrainerSpec trainer_spec_;
  NormalizerSpec normalizer_spec_;
  ModelProto model_proto_data_;
};

class Trainer : public TrainerInterface {
 public:
  Trainer(const TrainerSpec& trainer_spec,
          const NormalizerSpec& normalizer_spec,
          const NormalizerSpec& denormalizer_spec)
      : TrainerInterface::TrainerInterface(trainer_spec,
                                           normalizer_spec,
                                           denormalizer_spec) {}

  TrainerModel::SentencePieces MakeSeedSentencePieces();

  util::Status Train() override;

 private:
  FRIEND_TEST(TrainerTest, IsValidSentencePieceTest);

  // Makes seed pieces from the training corpus.
  // The size of seed pieces is determined by seed_sentencepiece_size.
  // node_int_type should be of integer type (int32 or int64),
  // determined by train_extremely_large_corpus.
  template <typename node_int_type>
  TrainerModel::SentencePieces MakeSeedSentencePiecesInternal();

  // Executes the E step of EM and returns expected count.
  // The index of return array is the vocab id.
  // |objective| is a negative likelihood of the current model.
  // |num_token| is the number of total tokens to tokenize
  // training corpus.
  std::vector<float> RunEStep(const TrainerModel &model, float *objective,
                              int64 *num_tokens) const;

  // Executes the M step of EM with the expected frequency and
  // returns new pieces.
  TrainerModel::SentencePieces RunMStep(
      const TrainerModel &model, const std::vector<float> &expected) const;

  // Heuristically prunes the current pieces.
  // This is called after each EM sub-iteration.
  TrainerModel::SentencePieces PruneSentencePieces(
      const TrainerModel &model) const;

  // Makes the final sentence pieces by incorporating the required characters
  // and control/user defined symbols.
  TrainerModel::SentencePieces FinalizeSentencePieces(
      const TrainerModel &model) const;

  // When the size of SentencePieces becomes less than desired_vocab_size_,
  // break the main training loop. desired_vocab_size_ = 1.1 * vocab_size_
  // for now.
  int desired_vocab_size_;
};
}  // namespace unigram
}  // namespace sentencepiece
#endif  // UNIGRAM_MODEL_TRAINER_H_
