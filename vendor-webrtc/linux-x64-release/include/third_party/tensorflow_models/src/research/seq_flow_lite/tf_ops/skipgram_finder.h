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
#ifndef TENSORFLOW_MODELS_SEQ_FLOW_LITE_TF_OPS_SKIPGRAM_FINDER_H_
#define TENSORFLOW_MODELS_SEQ_FLOW_LITE_TF_OPS_SKIPGRAM_FINDER_H_

#include <string>
#include <vector>

#include "absl/container/flat_hash_map.h"
#include "absl/container/flat_hash_set.h"
#include "absl/strings/string_view.h"

namespace seq_flow_lite {

// SkipgramFinder finds skipgrams in strings.
//
// To use: First, add skipgrams using AddSkipgram() - each skipgram is
// associated with some category.  Then, call FindSkipgrams() on a string,
// which will return the set of categories of the skipgrams in the string.
//
// Both the skipgrams and the input strings will be tokenzied by splitting
// on spaces.  Additionally, the tokens will be lowercased and have any
// trailing punctuation removed.
class SkipgramFinder {
 public:
  explicit SkipgramFinder(int max_skip_size) : max_skip_size_(max_skip_size) {}

  // Adds a skipgram that SkipgramFinder should look for in input strings.
  // Tokens may use the regex '.*' as a suffix.
  void AddSkipgram(absl::string_view skipgram, int category);

  // Find all of the skipgrams in `input`, and return their categories.
  absl::flat_hash_set<int> FindSkipgrams(absl::string_view input) const;

  // Find all of the skipgrams in `tokens`, and return their categories.
  absl::flat_hash_set<int> FindSkipgrams(
      const std::vector<absl::string_view>& tokens) const;

 private:
  struct TrieNode {
    absl::flat_hash_set<int> categories;
    // Maps tokens to the next node in the trie.
    absl::flat_hash_map<std::string, TrieNode> token_to_node;
    // Maps token prefixes (<prefix>.*) to the next node in the trie.
    absl::flat_hash_map<std::string, TrieNode> prefix_to_node;
  };

  TrieNode skipgram_trie_;
  int max_skip_size_;
};

}  // namespace seq_flow_lite

#endif  // TENSORFLOW_MODELS_SEQ_FLOW_LITE_TF_OPS_SKIPGRAM_FINDER_H_
