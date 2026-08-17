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

#ifndef BPE_MODEL_TRAINER_H_
#define BPE_MODEL_TRAINER_H_

#include <cstdint>
#include <limits>
#include <set>
#include <string>
#include <vector>

#include "absl/container/flat_hash_map.h"
#include "sentencepiece_model.pb.h"
#include "trainer_interface.h"

namespace sentencepiece {
namespace bpe {

// Trainer class for BPE model.
class Trainer : public TrainerInterface {
 public:
  Trainer(const TrainerSpec& trainer_spec,
          const NormalizerSpec& normalizer_spec,
          const NormalizerSpec& denormalizer_spec)
      : TrainerInterface::TrainerInterface(trainer_spec,
                                           normalizer_spec,
                                           denormalizer_spec) {}

  util::Status Train() override;

 private:
  // Symbol represents a character or symbol bigram.
  struct Symbol {
    const Symbol *left;              // left symbol in bigram
    const Symbol *right;             // right symbol in bigram
    string_util::UnicodeText chars;  // all flattend chracter sequence
    bool is_unk;                     // true if this symbol is unknown.
    uint64_t fp;                     // fingerprint of this symbol.
    uint64_t freq;                   // frequency of this symbol.

    // Position list. Use set so that we can keep the order of occurrence.
    // See EncodePos/DecodePos.
    std::set<uint64_t> positions;

    bool IsBigram() const { return left != nullptr && right != nullptr; }
    std::string ToString() const;
    Symbol() : left(nullptr), right(nullptr), is_unk(false), fp(0), freq(0) {}
  };

  struct Position {
    int sid;    // sentence id
    int left;   // left symbol index
    int right;  // right symbol index
  };

  // Encodes sid, left and right bigram index into uint64_t.
  // Encoded value keeps the order of sid, left and right.
  static uint64_t EncodePos(int sid, int l, int r) {
    CHECK_GE(l, 0);
    CHECK_GE(r, 0);
    CHECK_LE(l, std::numeric_limits<uint16_t>::max());
    CHECK_LE(r, std::numeric_limits<uint16_t>::max());
    const uint64_t n = (static_cast<uint64_t>(sid) << 32) |
                       (static_cast<uint64_t>(l) << 16) | r;
    return n;
  }

  // Decodes sid, left and right bigram index from uint64_t.
  static Position DecodePos(uint64_t n) {
    Position p;
    p.sid = n >> 32;
    p.left = (n >> 16) & 0xffff;
    p.right = n & 0xffff;
    return p;
  }

  // Gets unary (character) symbol from the char code |c|.
  // The return value is cached.
  Symbol *GetCharSymbol(char32 c);

  // Gets symbol pair from left/right symbols. The return value is cached.
  Symbol *GetPairSymbol(const Symbol *left, const Symbol *right);

  // Computes the frequency of |symbol| and update symbol->freq field.
  void ComputeFreq(Symbol *symbol) const;

  // Returns the valid index before symbols_[sid][index].
  int GetNextIndex(int sid, int index) const;

  // Returns the valid index after symbols_[sid][index].
  int GetPrevIndex(int sid, int index) const;

  // Makes a new bigram from [symbols_[sid][left], symbols_[sid][right]] and
  // Adds it to symbols_cache_ and active_symbols_.
  void AddNewPair(int sid, int left, int right);

  // Resets the fequency of bigram [symbols_[sid][left] symbols_[sid][right]],
  // if this bigram is not |best|.
  void ResetFreq(int sid, int left, int right, const Symbol *best);

  // Updates |active_symbols_| by copying the top 5% frequent symbols in
  // symbols_cache_.
  void UpdateActiveSymbols();

  // All unique symbols. Key is a fingerprint of Symbol.
  absl::flat_hash_map<uint64_t, Symbol*> symbols_cache_;

  // Set of symbols from which we find the best symbol in each iteration.
  std::set<Symbol *> active_symbols_;

  // Stores symbols allocated in heap so that we can delete them at onece.
  std::vector<Symbol *> allocated_;

  // Sentences. symbols_[sid][index] stores a symbol in sentence_[sid][index].
  std::vector<std::vector<Symbol *>> symbols_;
};
}  // namespace bpe
}  // namespace sentencepiece
#endif  // BPE_MODEL_TRAINER_H_
