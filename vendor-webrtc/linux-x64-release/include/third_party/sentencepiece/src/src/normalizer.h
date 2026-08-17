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

#ifndef NORMALIZER_NORMALIZER_H_
#define NORMALIZER_NORMALIZER_H_

#include <memory>
#include <set>
#include <string>
#include <utility>
#include <vector>

#include "absl/strings/string_view.h"
#include "common.h"
#include "sentencepiece_model.pb.h"
#include "sentencepiece_processor.h"
#include "third_party/darts_clone/darts.h"

namespace sentencepiece {
namespace normalizer {

// Given a list of strings, finds the longest string which is a
// prefix of a query.
class PrefixMatcher {
 public:
  // Initializes the PrefixMatcher with `dic`.
  explicit PrefixMatcher(const std::set<absl::string_view> &dic);

  // Finds the longest string in dic, which is a prefix of `w`.
  // Returns the UTF8 byte length of matched string.
  // `found` is set if a prefix match exists.
  // If no entry is found, consumes one Unicode character.
  int PrefixMatch(absl::string_view w, bool *found = nullptr) const;

  // Replaces entries in `w` with `out`.
  std::string GlobalReplace(absl::string_view w, absl::string_view out) const;

 private:
  std::unique_ptr<Darts::DoubleArray> trie_;
};

// Normalizer implements a simple text normalizer with
// user-defined string-to-string rules and leftmost longest
// matching. The rules of Normalizer are built with
// Builder::CompileCharsMap() method. Pre-compiled rules are
// also available via Builder::GetPrecompiledCharsMap(<name>) method.
//
// The motivation of Normalizer is to make flexible, user-customizable
// and self-contained normalizer.  All the logic of normalization is
// encoded in the model proto which allows us to define language/task
// dependent normalization rules without breaking the default rule.
class Normalizer {
 public:
  // Instantiates Normalizer with |spec|.
  // |spec| should not be deleted until Normalizer is destroyed.
  explicit Normalizer(const NormalizerSpec &spec);
  Normalizer(const NormalizerSpec &spec, const TrainerSpec &trainer_Spec);
  virtual ~Normalizer();

  virtual void SetPrefixMatcher(const PrefixMatcher *matcher) {
    matcher_ = matcher;
  }

  // Returns Status.
  // Normalizes function is valid only when status is OK.
  virtual util::Status status() const { return status_; }

  // Normalizes a plain utf8 string into an internal representation for
  // Sentencepiece model. |norm_to_orig| stores the byte-alignment from
  // normalized string to the original input.
  // This function can do the following normalizations:
  // - Character normalization.
  //   (NFKC / full-width to half-width conversion etc).
  // - Adds a prefix space.
  // - Replaces a space with a meta symbol.
  // - Removing heading, tailing and other redundant spaces.
  virtual util::Status Normalize(absl::string_view input,
                                 std::string* normalized,
                                 std::vector<size_t>* norm_to_orig) const;

  // Returns a normalized string without alignments.
  // This function is used in sentencepiece training.
  virtual std::string Normalize(absl::string_view input) const;

  friend class Builder;

 private:
  FRIEND_TEST(NormalizerTest, EncodeDecodePrecompiledCharsMapTest);

  void Init();

  // Normalizes the prefix of |input| and returns the pair of
  // normalized prefix and length we must consume after
  // normalization.
  // Here's the sample code for the full text normalization.
  //
  // string output;
  // absl::string_view input = "...";
  // while (!input.empty()) {
  //   const auto p = normalizer.NormalizePrefix(input);
  //   output.append(p.first.data(), p.first.size());
  //   input.remove_prefix(p.second);
  // }
  std::pair<absl::string_view, int> NormalizePrefix(
      absl::string_view input) const;

  // Encodes trie_blob and normalized string and return compiled blob.
  static std::string EncodePrecompiledCharsMap(absl::string_view trie_blob,
                                               absl::string_view normalized);

  // Decodes blob into trie_blob and normalized string.
  static util::Status DecodePrecompiledCharsMap(absl::string_view blob,
                                                absl::string_view* trie_blob,
                                                absl::string_view* normalized,
                                                std::string* buffer = nullptr);

  // Maximum size of the return value of Trie, which corresponds
  // to the maximum size of shared common prefix in the chars map.
  static constexpr int kMaxTrieResultsSize = 32;

  // Internal trie for efficient longest matching.
  std::unique_ptr<Darts::DoubleArray> trie_;

  // "\0" delimitered output string.
  // the value of |trie_| stores pointers to this string.
  const char *normalized_ = nullptr;

  // Spec for normalization.
  const NormalizerSpec *spec_;

  // Prefix matcher;
  const PrefixMatcher *matcher_ = nullptr;

  // Split hello world into "hello_" and "world_" instead of
  // "_hello" and "_world".
  const bool treat_whitespace_as_suffix_ = false;

#ifdef IS_BIG_ENDIAN
  // Stores the blob for TRIE encoded in big-endian.
  std::string precompiled_charsmap_buffer_;
#endif

  // Normalizer's status.
  util::Status status_;
};
}  // namespace normalizer
}  // namespace sentencepiece
#endif  // NORMALIZER_NORMALIZER_H_
