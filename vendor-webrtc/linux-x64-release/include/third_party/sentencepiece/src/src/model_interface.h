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

#ifndef MODEL_INTERFACE_H_
#define MODEL_INTERFACE_H_

#include <memory>
#include <set>
#include <string>
#include <utility>
#include <vector>

#include "absl/container/flat_hash_map.h"
#include "absl/strings/string_view.h"
#include "common.h"
#include "normalizer.h"
#include "sentencepiece_model.pb.h"
#include "sentencepiece_processor.h"
#include "third_party/darts_clone/darts.h"
#include "util.h"

namespace sentencepiece {

// "_this_is_a_pen" => ["_this", "_is", "_a", "_pen"]
std::vector<absl::string_view> SplitIntoWords(
    absl::string_view text,
    bool treat_ws_as_suffix = false,
    bool allow_ws_only_pieces = false);

// Converts byte (0-255) to piece (e.g., 58 -> "<0x3A>").
std::string ByteToPiece(unsigned char c);

// Converts piece to byte (e.g., "<0x3A>" -> 58). Returns -1 if `piece` is not
// a valid byte piece.
int PieceToByte(absl::string_view piece);

using EncodeResult = std::vector<std::pair<absl::string_view, int>>;
using NBestEncodeResult = std::vector<std::pair<EncodeResult, float>>;

class ModelProto;

// Underlying model interface.
// Given a normalized string, returns a sequence of sentence pieces with ids.
class ModelInterface {
 public:
  using PieceToIdMap = absl::flat_hash_map<absl::string_view, int>;
  //                                           string_util::string_view_hash>;

  absl::string_view unk_piece() const;
  absl::string_view bos_piece() const;
  absl::string_view eos_piece() const;
  absl::string_view pad_piece() const;

  // `model_proto` should not be deleted until ModelInterface is destroyed.
  explicit ModelInterface(const ModelProto &model_proto);
  ModelInterface() {}

  virtual ~ModelInterface();

  // Returns Status.
  // Encode/Decode functions are valid only when status is OK.
  virtual util::Status status() const { return status_; }

  virtual const ModelProto &model_proto() const { return *model_proto_; }

  virtual const normalizer::PrefixMatcher *prefix_matcher() const {
    return matcher_.get();
  }

  // Given a normalized string, returns a sequence of sentence pieces with ids.
  // The concatenation of pieces must be the same as `normalized`.
  virtual EncodeResult Encode(absl::string_view normalized) const = 0;

  // The same as above, but returns nbest result with score.
  virtual NBestEncodeResult NBestEncode(absl::string_view normalized,
                                        int nbest_size) const {
    LOG(ERROR) << "Not implemented.";
    return NBestEncodeResult();
  }

  virtual EncodeResult SampleEncode(absl::string_view normalized,
                                    float alpha) const {
    LOG(ERROR) << "Not implemented.";
    return EncodeResult();
  }

  // Sample `samples` many tokenisations from the segmentation lattice
  // If `wor` is true, the samples are taken without replacement, and the scores
  // are the inclusion probabilities of the elements in the sample; otherwise
  // the samples are taken with replacement and the scores are the log-probs of
  // sample elements
  // If `include_best` is true, the best tokenisation is always included in the
  // sample, and the remaining elements are sampled excluding the best.
  virtual NBestEncodeResult SampleEncodeAndScore(absl::string_view normalized,
                                                 float alpha,
                                                 int samples,
                                                 bool wor,
                                                 bool include_best) const {
    LOG(ERROR) << "Not implemented.";
    return {{EncodeResult(), 0.0}};
  }

  // Calculates the entropy of the segmentation lattice with inverse temperature
  // `alpha`. Uses a novel dynamic program to calculate the entropy.
  virtual float CalculateEntropy(absl::string_view normalized,
                                 float alpha) const {
    LOG(ERROR) << "Not implemented.";
    return 0.0;
  }

  // Return true if SampleEncode returns a valid result.
  virtual bool IsSampleEncodeAvailable() const { return false; }

  // Return true if NBestEncode returns a valid result.
  virtual bool IsNBestEncodeAvailable() const { return false; }

  // Return true if SampleEncodeAndScore returns a valid result.
  virtual bool IsSampleEncodeAndScoreAvailable() const { return false; }

  // Return true if CalculateEntropy returns a valid result.
  virtual bool IsCalculateEntropyAvailable() const { return false; }

  // Returns the vocab id of `piece`.
  // Returns UNK(0) if `piece` is unknown
  virtual int PieceToId(absl::string_view piece) const;

  // Returns the string representation of vocab with `id`.
  // id must be 0 <= id < GetPieceSize().
  virtual const std::string &IdToPiece(int id) const {
    return model_proto_->pieces(id).piece();
  }

  // Returns the size of sentence pieces, which is the same
  // as the size of vocabulary for NMT.
  virtual int GetPieceSize() const {
    if (!model_proto_) {
      return 0;
    }
    return model_proto_->pieces_size();
  }

  // Returns the score of `id`.
  // Score represents a log probability of the piece.
  // We can roughly estimate the unigram frequency of the piece.
  virtual float GetScore(int id) const {
    return model_proto_->pieces(id).score();
  }

  // Returns true if `id` is unknown symbol.
  virtual bool IsUnknown(int id) const {
    return (model_proto_->pieces(id).type() ==
            ModelProto::SentencePiece::UNKNOWN);
  }

  // Returns true if `id` is control symbol.
  virtual bool IsControl(int id) const {
    return (model_proto_->pieces(id).type() ==
            ModelProto::SentencePiece::CONTROL);
  }

  // Returns true if `id` is unused symbol.
  virtual bool IsUnused(int id) const {
    return (model_proto_->pieces(id).type() ==
            ModelProto::SentencePiece::UNUSED);
  }

  // Returns true if `id` is user defined symbol.
  virtual bool IsUserDefined(int id) const {
    return (model_proto_->pieces(id).type() ==
            ModelProto::SentencePiece::USER_DEFINED);
  }

  // Returns true if `id` is byte symbol.
  virtual bool IsByte(int id) const {
    return (model_proto_->pieces(id).type() == ModelProto::SentencePiece::BYTE);
  }

  virtual bool ByteFallbackEnabled() const {
    return model_proto_ && model_proto_->trainer_spec().byte_fallback();
  }

  // Verifies if the `expected` and `actual` outputs are equivalent. `expected`
  // and `actual` are sentence pieces joined by space (` `). Normally it means
  // that the two strings are identical. In some model, due to float rounding
  // errors, the strings may not be identical, but they may be still equivalent
  // provided their scores are close enough (by some espilon).
  virtual bool VerifyOutputsEquivalent(absl::string_view expected,
                                       absl::string_view actual) const {
    return expected == actual;
  }

 protected:
  void InitializePieces();

  // Non-virtual (inlined) implementation for faster execution.
  inline float GetScoreInlined(int id) const {
    return model_proto_->pieces(id).score();
  }

  inline bool IsUnknownInlined(int id) const {
    return (model_proto_->pieces(id).type() ==
            ModelProto::SentencePiece::UNKNOWN);
  }

  inline bool IsControlInlined(int id) const {
    return (model_proto_->pieces(id).type() ==
            ModelProto::SentencePiece::CONTROL);
  }

  inline bool IsUnusedInlined(int id) const {
    return (model_proto_->pieces(id).type() ==
            ModelProto::SentencePiece::UNUSED);
  }

  inline bool IsUserDefinedInlined(int id) const {
    return (model_proto_->pieces(id).type() ==
            ModelProto::SentencePiece::USER_DEFINED);
  }

  inline bool IsByteInlined(int id) const {
    return (model_proto_->pieces(id).type() == ModelProto::SentencePiece::BYTE);
  }

  const ModelProto *model_proto_ = nullptr;

  // PrefixMatcher for user defined symbols.
  std::unique_ptr<normalizer::PrefixMatcher> matcher_;

  // piece -> id map for normal pieces
  PieceToIdMap pieces_;

  // piece -> id map for control, unknown, and byte pieces
  PieceToIdMap reserved_id_map_;

  // unknown id.
  int unk_id_ = 0;

  // status.
  util::Status status_;
};
}  // namespace sentencepiece
#endif  // MODEL_INTERFACE_H_
