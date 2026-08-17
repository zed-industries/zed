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

#ifndef SENTENCEPIECE_PROCESSOR_H_
#define SENTENCEPIECE_PROCESSOR_H_

#include <cstring>
#include <memory>
#include <string>
#include <string_view>
#include <utility>
#include <vector>

#include "absl/status/status.h"

#ifndef SWIG
namespace absl {
using std::string_view;
}  // namespace absl
#endif  // SWIG

namespace sentencepiece {
namespace util {

using StatusCode = absl::StatusCode;
using Status = absl::Status;
}  // namespace util

// SentencePieceProcessor:
// Simple and language independent tokenizer and de-tokenizer for
// Neural Network Machine Translation.
//
// SentencePieceProcessor provides Encode() and Decode() methods,
// which correspond to tokenization and de-tokenization respectively.
//
// - Encode:
//   Given a raw source sentence, encode it into a sequence
//   of pieces or vocabulary ids.
//
// - Decode:
//   Given a sequence of pieces or vocabulary ids, decode it
//   into a de-tokenized raw sentence.
//
// SentencePieceProcessor provides a lossless data conversion
// that allows the original raw sentence to be perfectly reconstructed
// from the encoded data, i.e., Decode(Encode(input)) == input.
// This characteristics is useful, as we can make the de-tokenization
// completely language independent.
//
// Usage:
//   SentencePieceProcessor sp;
//   sp.Load("//path/to/model");
//
//   vector<string> sps;
//   sp.Encode("hello world.", &sps).IgnoreError();
//
//   vector<int> ids;
//   sp.Encode("hello world.", &ids).IgnoreError();
//
//   string detok;
//   sp.Decode(sps, &detok);
//   CHECK_EQ("hello world.", detok).IgnoreError();
//
//   sp.Decode(ids, &detok);
//   CHECK_EQ("hello world.", detok).IgnoreError();
//
//  We can also use SentencePieceText which manages the byte-offsets
//  between user input (output) and internal sentence pieces.
//
//   SentencePieceText spt;
//   sp.Encode("hello world.", &spt);
//   // Emits the byte range of each piece.
//   for (const auto &piece : spt.pieces()) {
//      LOG(INFO) << piece.begin() << " " << piece.end();
//   }
//
//   sp.Decode({0, 1, 2, 3..}, &spt);
//   for (const auto &piece : spt.pieces()) {
//      LOG(INFO) << piece.begin() << " " << piece.end();
//   }
//

class NBestSentencePieceText;
class ModelInterface;
class SentencePieceText;
class ModelProto;

namespace normalizer {
class Normalizer;
}  // namespace normalizer

#ifndef SWIGGO
namespace util {
// Redefine std::string for serialized_proto interface as Python's string is
// a Unicode string. We can enforce the return value to be raw byte sequence
// with SWIG's typemap.
using bytes = std::string;
}  // namespace util
#endif  // SWIGGO

class NBestSentencePieceText;
class ModelInterface;
class SentencePieceText;
class SentencePieceText_SentencePiece;

// Wrapper class of SentencePieceText
// This wrapper only allows an immutable access to the proto and
// hides the actual implementation of protobuf.
// See sentencepiece.proto for the details of this class.
class ImmutableSentencePieceText_ImmutableSentencePiece {
 public:
  ImmutableSentencePieceText_ImmutableSentencePiece();
  ~ImmutableSentencePieceText_ImmutableSentencePiece() = default;

  const std::string& piece() const;
  const std::string& surface() const;
  uint32_t id() const;
  uint32_t begin() const;
  uint32_t end() const;

  friend class ImmutableSentencePieceText;

 private:
  explicit ImmutableSentencePieceText_ImmutableSentencePiece(
      const SentencePieceText_SentencePiece& sp);
  const SentencePieceText_SentencePiece* sp_ = nullptr;
};

class ImmutableSentencePieceText {
 public:
  ImmutableSentencePieceText();
  virtual ~ImmutableSentencePieceText();

  std::vector<ImmutableSentencePieceText_ImmutableSentencePiece> pieces() const;

  size_t pieces_size() const;
  ImmutableSentencePieceText_ImmutableSentencePiece pieces(int index) const;

  const std::string& text() const;
  float score() const;

  util::bytes SerializeAsString() const;

  // Returns the actual mutable proto.
  // Do not use this outside of SentencePieceProcessor, as
  // it returns the raw pointer managed by the shared_ptr.
  SentencePieceText* mutable_proto();

  // Converts the utf8 byte spans into Unicode char span.
  void ConvertToUnicodeSpans();

  friend class ImmutableNBestSentencePieceText;

 private:
  explicit ImmutableSentencePieceText(const SentencePieceText& spt);
  const SentencePieceText* spt_ = nullptr;
  std::shared_ptr<SentencePieceText> rep_;
};

// Wrapper class of SentencePieceText
// This wrapper only allows an immutable access to the proto and
// hides the actual implementation of protobuf.
// See sentencepiece.proto for the details of this class.
class ImmutableNBestSentencePieceText {
 public:
  ImmutableNBestSentencePieceText();
  virtual ~ImmutableNBestSentencePieceText();

  std::vector<ImmutableSentencePieceText> nbests() const;

  size_t nbests_size() const;
  ImmutableSentencePieceText nbests(int index) const;

  util::bytes SerializeAsString() const;

  // Returns the actual mutable proto.
  // Do not use this outside of SentencePieceProcessor, as
  // it returns the raw pointer managed by the shared_ptr.
  NBestSentencePieceText* mutable_proto();

  void ConvertToUnicodeSpans();

 private:
  std::shared_ptr<NBestSentencePieceText> rep_;
};

class SentencePieceProcessor {
 public:
  SentencePieceProcessor();
  virtual ~SentencePieceProcessor();

  // Loads model from `filename`.
  // Returns false if `filename` cannot be loaded.
  virtual util::Status Load(absl::string_view filename);

  // Loads model from `filename`.
  // Crash if `filename` cannot be loaded.
  virtual void LoadOrDie(absl::string_view filename);

  // Loads model from `model_proto`.
  // `model_proto` is copied.
  virtual util::Status Load(const ModelProto& model_proto);

  // Loads model from `model_proto`.
  // `model_proto` is moved.
  virtual util::Status Load(std::unique_ptr<ModelProto> model_proto);

  // Loads model from `serialized`, which is a string-serialized model proto.
  // Useful to load the model from a platform independent blob object.
  virtual util::Status LoadFromSerializedProto(absl::string_view serialized);

  // Returns the status. Encode/Decode methods are valid when status is OK.
  virtual util::Status status() const;

  // Sets encode extra_option sequence.
  virtual util::Status SetEncodeExtraOptions(absl::string_view extra_option);

  // Sets decode extra_option sequence.
  virtual util::Status SetDecodeExtraOptions(absl::string_view extra_option);

  //////////////////////////////////////////////////////////////
  // Vocabulary restriction.
  // Background:
  // https://github.com/rsennrich/subword-nmt#best-practice-advice-for-byte-pair-encoding-in-nmt

  // Restricts the vocabulary set.
  // The input sentences are encoded into the tokens in `valid_vocab`.
  virtual util::Status SetVocabulary(
      const std::vector<absl::string_view>& valid_vocab);

  // Reverts the vocabulary restriction.
  virtual util::Status ResetVocabulary();

  // Loads the valid vocabulary set from `filename` in TSV format.
  // Format:  <token> <tab> <freq>.
  // Any token with frequency < threshold will be treated as OOV.
  virtual util::Status LoadVocabulary(absl::string_view filename,
                                      int threshold);

  //////////////////////////////////////////////////////////////
  // Simple Encode and Decode API.
  //
  // Given a UTF8 input, encodes it into a sequence of sentence pieces.
  virtual util::Status Encode(absl::string_view input,
                              std::vector<std::string>* pieces) const;

  // Given a UTF8 input, encodes it into a sequence of ids.
  virtual util::Status Encode(absl::string_view input,
                              std::vector<int>* ids) const;

  // Given a sequence of pieces, decodes it into a detokenized output.
  virtual util::Status Decode(const std::vector<std::string>& pieces,
                              std::string* detokenized) const;

  // Given a sequence of pieces, decodes it into a detokenized output.
  virtual util::Status Decode(const std::vector<absl::string_view>& pieces,
                              std::string* detokenized) const;

  // Given a sequence of ids, decodes it into a detokenized output.
  virtual util::Status Decode(const std::vector<int>& ids,
                              std::string* detokenized) const;

  //////////////////////////////////////////////////////////////
  // NBest API.
  //
  // Same as Encode, but returns nbest results.
  virtual util::Status NBestEncode(
      absl::string_view input,
      int nbest_size,
      std::vector<std::vector<std::string>>* pieces) const;

  // Same as Encode, but returns nbest results.
  virtual util::Status NBestEncode(absl::string_view input,
                                   int nbest_size,
                                   std::vector<std::vector<int>>* ids) const;

  //////////////////////////////////////////////////////////////
  // Sampling API.
  //
  // Unigram and BPE support sampling mode.
  // - Unigram (--model_type=unigram):
  // `nbest_size`: When `nbest_size` is positive value, approximately samples
  // one segmentation from nbest candidates. When `nbest_size` is negative
  // value, samples one segmentation from the hypotheses (Lattice) according to
  // the generation probabilities using forward-filtering and backward-sampling
  // algorithm.
  // `alpha`: Smoothing parameter (inverse temperature). The best segmentation
  // (Viterbi segmentation) is more likely sampled when setting larger alpha.
  // When alpha is 0.0, one segmentation is uniformly sampled from the nbest or
  // lattice. `nbest_size` and `alpha` correspond to parameters `l` and `alpha`
  // in https://arxiv.org/abs/1804.10959  (nbest_size < 0 means l = infinity)
  //
  // - BPE (--model_type=bpe):
  // `alpha`: The dropout probability `p` of bpe merge operations in
  // https://arxiv.org/abs/1910.13267 Nbest-based sampling is not supported so
  // nbest_size parameter is ignored in BPE.
  virtual util::Status SampleEncode(absl::string_view input,
                                    int nbest_size,
                                    float alpha,
                                    std::vector<std::string>* pieces) const;

  // Same as above, but returns a sequence of ids.
  virtual util::Status SampleEncode(absl::string_view input,
                                    int nbest_size,
                                    float alpha,
                                    std::vector<int>* ids) const;

  //////////////////////////////////////////////////////////////
  // SampleEncodeAndScore API.
  //
  // Sample `samples` many tokenisations from the segmentation lattice.
  // These methods are only available in model_type=unigram.
  //
  // `alpha`: smoothing parameter (inverse temperature). The same as `alpha` in
  // `Sample` method.
  // 'wor`: If `wor` is true, the samples are taken without replacement, and the
  // scores are the inclusion probabilities of the elements in the sample;
  // otherwise the samples are taken with replacement and the scores are the
  // log-probs of sample elements
  // `include_best`: If `include_best` is true, the best tokenisation is always
  // included in the sample, and the remaining elements are sampled excluding
  // the best.
  virtual util::Status SampleEncodeAndScore(
      absl::string_view input,
      int num_samples,
      float alpha,
      bool wor,
      bool include_best,
      std::vector<std::pair<std::vector<std::string>, float>>* pieces) const;

  // Same as above, but returns a sequence of ids.
  virtual util::Status SampleEncodeAndScore(
      absl::string_view input,
      int num_samples,
      float alpha,
      bool wor,
      bool include_best,
      std::vector<std::pair<std::vector<int>, float>>* ids) const;

  //////////////////////////////////////////////////////////////
  // Entropy API.
  //
  // This only available in model_type=unigram.
  // Calculate entropy of possible tokenisations
  virtual util::Status CalculateEntropy(absl::string_view input,
                                        float alpha,
                                        float* entropy) const;

  //////////////////////////////////////////////////////////////
  // Advanced API returning SentencePieceText, which manages
  // utf8-byte alignments between user-input/detokenized text
  // and internal sentencepiece sequence.
  //
  // Given a UTF8 input, encodes it into SentencePieceText.
  //
  // When using these APIs, sentencepiece.pb.h header files must be included.
  // We can also use ImutableSentencePieceText as follows.
  //
  // ImmutableSentencePieceText spt;
  // Encode("hello", spt.mutable_proto()).IgnoreError();
  // std::cout << spt.pieces_size() << std::endl;
  virtual util::Status Encode(absl::string_view input,
                              SentencePieceText* spt) const;

  virtual util::Status NBestEncode(absl::string_view input,
                                   int nbest_size,
                                   NBestSentencePieceText* nbest_spt) const;

  virtual util::Status SampleEncode(absl::string_view input,
                                    int nbest_size,
                                    float alpha,
                                    SentencePieceText* spt) const;

  virtual util::Status SampleEncodeAndScore(
      absl::string_view input,
      int num_samples,
      float alpha,
      bool wor,
      bool include_best,
      NBestSentencePieceText* samples_spt) const;

  // DEPRECATED: Remove this API and use std::vector<std::string_view>
  virtual util::Status Decode(const std::vector<std::string>& pieces,
                              SentencePieceText* spt) const;

  virtual util::Status Decode(const std::vector<absl::string_view>& pieces,
                              SentencePieceText* spt) const;

  virtual util::Status Decode(const std::vector<int>& ids,
                              SentencePieceText* spt) const;
#ifdef SWIG
#define SPP_SWIG_CHECK_AND_THROW \
  if (!status.ok())              \
    throw status;
#else
#define SPP_SWIG_CHECK_AND_THROW \
  if (!status.ok()) {            \
  }
#endif  // SWIG

#define DEFINE_SPP_DIRECT_FUNC_IMPL(FuncName, OutType, ...) \
  OutType output;                                           \
  const auto status = FuncName(__VA_ARGS__, &output);       \
  SPP_SWIG_CHECK_AND_THROW;                                 \
  return output;

#define DEFINE_SPP_SERIALIZED_PROTO_IMPL(FuncName, OutType, ...)     \
  OutType output;                                                    \
  const auto status = FuncName(__VA_ARGS__, output.mutable_proto()); \
  SPP_SWIG_CHECK_AND_THROW;                                          \
  return output.SerializeAsString();

#define DEFINE_SPP_IMMUTABLE_PROTO_IMPL(FuncName, OutType, ...)      \
  OutType output;                                                    \
  const auto status = FuncName(__VA_ARGS__, output.mutable_proto()); \
  SPP_SWIG_CHECK_AND_THROW;                                          \
  return output;

  //////////////////////////////////////////////////////////////
  // Handy methods that return the result directly.
  // These functions ignore internal errors.
  virtual std::vector<std::string> EncodeAsPieces(
      absl::string_view input) const {
    DEFINE_SPP_DIRECT_FUNC_IMPL(Encode, std::vector<std::string>, input);
  }

  virtual std::vector<int> EncodeAsIds(absl::string_view input) const {
    DEFINE_SPP_DIRECT_FUNC_IMPL(Encode, std::vector<int>, input);
  }

  virtual std::vector<std::vector<std::string>> NBestEncodeAsPieces(
      absl::string_view input, int nbest_size) const {
    DEFINE_SPP_DIRECT_FUNC_IMPL(
        NBestEncode, std::vector<std::vector<std::string>>, input, nbest_size);
  }

  virtual std::vector<std::vector<int>> NBestEncodeAsIds(
      absl::string_view input, int nbest_size) const {
    DEFINE_SPP_DIRECT_FUNC_IMPL(NBestEncode, std::vector<std::vector<int>>,
                                input, nbest_size);
  }

  virtual std::vector<std::string> SampleEncodeAsPieces(absl::string_view input,
                                                        int nbest_size,
                                                        float alpha) const {
    DEFINE_SPP_DIRECT_FUNC_IMPL(SampleEncode, std::vector<std::string>, input,
                                nbest_size, alpha);
  }

  virtual std::vector<int> SampleEncodeAsIds(absl::string_view input,
                                             int nbest_size,
                                             float alpha) const {
    DEFINE_SPP_DIRECT_FUNC_IMPL(SampleEncode, std::vector<int>, input,
                                nbest_size, alpha);
  }

  virtual std::vector<std::pair<std::vector<std::string>, float>>
  SampleEncodeAndScoreAsPieces(absl::string_view input,
                               int num_samples,
                               float alpha,
                               bool wor,
                               bool include_best) const {
    using _T = std::vector<std::pair<std::vector<std::string>, float>>;
    DEFINE_SPP_DIRECT_FUNC_IMPL(SampleEncodeAndScore, _T, input, num_samples,
                                alpha, wor, include_best);
  }

  virtual std::vector<std::pair<std::vector<int>, float>>
  SampleEncodeAndScoreAsIds(absl::string_view input,
                            int num_samples,
                            float alpha,
                            bool wor,
                            bool include_best) const {
    using _T = std::vector<std::pair<std::vector<int>, float>>;
    DEFINE_SPP_DIRECT_FUNC_IMPL(SampleEncodeAndScore, _T, input, num_samples,
                                alpha, wor, include_best);
  }

  // DEPRECATED: Remove this API and use std::vector<std::string_view>
  virtual std::string DecodePieces(
      const std::vector<std::string> &pieces) const {
    DEFINE_SPP_DIRECT_FUNC_IMPL(Decode, std::string, pieces);
  }

  virtual std::string DecodePieces(
      const std::vector<absl::string_view>& pieces) const {
    DEFINE_SPP_DIRECT_FUNC_IMPL(Decode, std::string, pieces);
  }

  virtual std::string DecodeIds(const std::vector<int> &ids) const {
    DEFINE_SPP_DIRECT_FUNC_IMPL(Decode, std::string, ids);
  }

  virtual float CalculateEntropy(absl::string_view text, float alpha) const {
    DEFINE_SPP_DIRECT_FUNC_IMPL(CalculateEntropy, float, text, alpha);
  }

  //////////////////////////////////////////////////////////////
  // SerializedProto API. (DEPRECATED). Use ImmutableProto API.
  // They are used in Python interface. Returns serialized proto.
  // In python module, we can get access to the full Proto after
  // deserialzing the returned byte sequence.
  virtual util::bytes EncodeAsSerializedProto(absl::string_view input) const {
    DEFINE_SPP_SERIALIZED_PROTO_IMPL(Encode, ImmutableSentencePieceText, input);
  }

  virtual util::bytes SampleEncodeAsSerializedProto(absl::string_view input,
                                                    int nbest_size,
                                                    float alpha) const {
    DEFINE_SPP_SERIALIZED_PROTO_IMPL(SampleEncode, ImmutableSentencePieceText,
                                     input, nbest_size, alpha);
  }

  virtual util::bytes NBestEncodeAsSerializedProto(absl::string_view input,
                                                   int nbest_size) const {
    DEFINE_SPP_SERIALIZED_PROTO_IMPL(
        NBestEncode, ImmutableNBestSentencePieceText, input, nbest_size);
  }

  virtual util::bytes SampleEncodeAndScoreAsSerializedProto(
      absl::string_view input,
      int num_samples,
      float alpha,
      bool wor,
      bool include_best) const {
    DEFINE_SPP_SERIALIZED_PROTO_IMPL(SampleEncodeAndScore,
                                     ImmutableNBestSentencePieceText, input,
                                     num_samples, alpha, wor, include_best);
  }

  // TODO(taku): Remove this API and use std::vector<std::string_view>
  virtual util::bytes DecodePiecesAsSerializedProto(
      const std::vector<std::string>& pieces) const {
    DEFINE_SPP_SERIALIZED_PROTO_IMPL(Decode, ImmutableSentencePieceText,
                                     pieces);
  }

  virtual util::bytes DecodePiecesAsSerializedProto(
      const std::vector<absl::string_view>& pieces) const {
    DEFINE_SPP_SERIALIZED_PROTO_IMPL(Decode, ImmutableSentencePieceText,
                                     pieces);
  }

  virtual util::bytes DecodeIdsAsSerializedProto(
      const std::vector<int>& ids) const {
    DEFINE_SPP_SERIALIZED_PROTO_IMPL(Decode, ImmutableSentencePieceText, ids);
  }

  //////////////////////////////////////////////////////////////
  // ImmutableProto API.
  virtual ImmutableSentencePieceText EncodeAsImmutableProto(
      absl::string_view input) const {
    DEFINE_SPP_IMMUTABLE_PROTO_IMPL(Encode, ImmutableSentencePieceText, input);
  }

  virtual ImmutableSentencePieceText SampleEncodeAsImmutableProto(
      absl::string_view input,
      int nbest_size,
      float alpha) const {
    DEFINE_SPP_IMMUTABLE_PROTO_IMPL(SampleEncode, ImmutableSentencePieceText,
                                    input, nbest_size, alpha);
  }

  virtual ImmutableNBestSentencePieceText NBestEncodeAsImmutableProto(
      absl::string_view input,
      int nbest_size) const {
    DEFINE_SPP_IMMUTABLE_PROTO_IMPL(
        NBestEncode, ImmutableNBestSentencePieceText, input, nbest_size);
  }

  virtual ImmutableNBestSentencePieceText SampleEncodeAndScoreAsImmutableProto(
      absl::string_view input,
      int num_samples,
      float alpha,
      bool wor,
      bool include_best) const {
    DEFINE_SPP_IMMUTABLE_PROTO_IMPL(SampleEncodeAndScore,
                                    ImmutableNBestSentencePieceText, input,
                                    num_samples, alpha, wor, include_best);
  }

  // TODO(taku): Remove this API and use std::vector<std::string_view>
  virtual ImmutableSentencePieceText DecodePiecesAsImmutableProto(
      const std::vector<std::string>& pieces) const {
    DEFINE_SPP_IMMUTABLE_PROTO_IMPL(Decode, ImmutableSentencePieceText, pieces);
  }

  virtual ImmutableSentencePieceText DecodePiecesAsImmutableProto(
      const std::vector<absl::string_view>& pieces) const {
    DEFINE_SPP_IMMUTABLE_PROTO_IMPL(Decode, ImmutableSentencePieceText, pieces);
  }

  virtual ImmutableSentencePieceText DecodeIdsAsImmutableProto(
      const std::vector<int>& ids) const {
    DEFINE_SPP_IMMUTABLE_PROTO_IMPL(Decode, ImmutableSentencePieceText, ids);
  }

#undef DEFINE_SPP_DIRECT_FUNC_IMPL
#undef DEFINE_SPP_SERIALIZED_PROTO_IMPL
#undef DEFINE_SPP_IMMUTABLE_PROTO_IMPL

  //////////////////////////////////////////////////////////////
  // Vocabulary management methods.
  //
  // Returns the size of sentence pieces, which is the same as
  // the size of vocabulary for NMT.
  virtual int GetPieceSize() const;

  // Returns the vocab id of `piece`.
  // Returns UNK(0) if `piece` is unknown.
  virtual int PieceToId(absl::string_view piece) const;

  // Returns the string representation of vocab with `id`.
  virtual const std::string &IdToPiece(int id) const;

  // Returns the score of `id`.
  // Usually score is an emission log probability of unigram language
  // model.
  virtual float GetScore(int id) const;

  // Returns true if `id` is unknown symbol.
  virtual bool IsUnknown(int id) const;

  // Returns true if `id` is control symbol.
  virtual bool IsControl(int id) const;

  // Returns true if `id` is unused symbol.
  virtual bool IsUnused(int id) const;

  // Returns true if `id` is byte symbol.
  virtual bool IsByte(int id) const;

  // Returns the reserved id.
  // Returns -1 if not defined.

  // Returns unknown (<unk>) id.
  virtual int unk_id() const;

  // Returns BOS (<s>) id.
  virtual int bos_id() const;

  // Returns EOS (</s>) id.
  virtual int eos_id() const;

  // Returns PAD (<pad>) id.
  virtual int pad_id() const;

  //////////////////////////////////////////////////////////////
  // Model management.
  //
  // Allows injection of a mock model instance. `model` is moved.
  void SetModel(std::unique_ptr<ModelInterface> &&model);

  // Allows injection of a normalizer instance. `normalizer` is moved.
  void SetNormalizer(std::unique_ptr<normalizer::Normalizer> &&normalizer);

  // Returns immutable model proto. Useful to obtain extended
  // or experimental parameters encoded in model_proto.
  const ModelProto &model_proto() const;

  // returns immutable model proto as std::string.
  // Useful to save the state of this instance via Python's pickle object.
  util::bytes serialized_model_proto() const;

 private:
  enum ExtraOption { REVERSE, BOS, EOS, UNK_PIECE };

  util::Status ParseExtraOptions(absl::string_view extra_option,
                                 std::vector<ExtraOption>* extra_options) const;

  util::Status ApplyExtraOptions(const std::vector<ExtraOption>& extra_options,
                                 SentencePieceText* spt) const;

  util::Status PopulateSentencePieceText(
      absl::string_view input,
      absl::string_view normalized,
      const std::vector<size_t>& norm_to_orig,
      const std::vector<std::pair<absl::string_view, int>>& result,
      SentencePieceText* spt) const;

  std::unique_ptr<ModelInterface> model_;
  std::unique_ptr<normalizer::Normalizer> normalizer_;
  std::unique_ptr<normalizer::Normalizer> denormalizer_;

  // Underlying model protocol buffer. The same lifetime as model_.
  std::unique_ptr<ModelProto> model_proto_;

  std::vector<ExtraOption> encode_extra_options_;
  std::vector<ExtraOption> decode_extra_options_;
};

// Set seed value of random generator.
// Do not set static_cast<unique_int>(-1),
// as this seed is reserved for initializing from
// std::random_device.
void SetRandomGeneratorSeed(unsigned int seed);

// IO related functions to absorb model formats.
namespace io {
// Loads `model_proto` from `filename`.
// We can instantiate SentencePieceProcessor as follows:
//
//  auto model_proto = absl::make_unique<ModelProto>();
//  io::LoadModelProto("//path/spm.model", model_proto.get());
//  SentencePieceProcessor sp;
//  CHECK_OK(sp.Load(std::move(model_proto)));
util::Status LoadModelProto(absl::string_view, ModelProto* model_proto);

// Saves `model_proto` as `filename`.
util::Status SaveModelProto(absl::string_view, const ModelProto& model_proto);
}  // namespace io
}  // namespace sentencepiece
#endif  // SENTENCEPIECE_PROCESSOR_H_
