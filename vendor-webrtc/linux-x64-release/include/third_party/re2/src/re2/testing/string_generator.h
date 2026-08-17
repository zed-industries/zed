// Copyright 2008 The RE2 Authors.  All Rights Reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

#ifndef RE2_TESTING_STRING_GENERATOR_H_
#define RE2_TESTING_STRING_GENERATOR_H_

// String generator: generates all possible strings of up to
// maxlen letters using the set of letters in alpha.
// Fetch strings using a Java-like Next()/HasNext() interface.

#include <stdint.h>

#include <random>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"

namespace re2 {

class StringGenerator {
 public:
  StringGenerator(int maxlen, const std::vector<std::string>& alphabet);
  ~StringGenerator() {}

  absl::string_view Next();
  bool HasNext() { return hasnext_; }

  // Resets generator to start sequence over.
  void Reset();

  // Causes generator to emit random strings for next n calls to Next().
  void Random(int32_t seed, int n);

  // Causes generator to emit a NULL as the next call.
  void GenerateNULL();

 private:
  bool IncrementDigits();
  bool RandomDigits();

  // Global state.
  int maxlen_;                         // Maximum length string to generate.
  std::vector<std::string> alphabet_;  // Alphabet, one string per letter.

  // Iteration state.
  absl::string_view sp_;     // Last string_view returned by Next().
  std::string s_;            // String data in last string_view returned by Next().
  bool hasnext_;             // Whether Next() can be called again.
  std::vector<int> digits_;  // Alphabet indices for next string.
  bool generate_null_;       // Whether to generate a NULL string_view next.
  bool random_;              // Whether generated strings are random.
  int nrandom_;              // Number of random strings left to generate.
  std::minstd_rand0 rng_;    // Random number generator.

  StringGenerator(const StringGenerator&) = delete;
  StringGenerator& operator=(const StringGenerator&) = delete;
};

// Generates and returns a string over binary alphabet {0,1} that contains
// all possible binary sequences of length n as subsequences.  The obvious
// brute force method would generate a string of length n * 2^n, but this
// generates a string of length n-1 + 2^n called a De Bruijn cycle.
// See Knuth, The Art of Computer Programming, Vol 2, Exercise 3.2.2 #17.
//
// Such a string is useful for testing a DFA.  If you have a DFA
// where distinct last n bytes implies distinct states, then running on a
// DeBruijn string causes the DFA to need to create a new state at every
// position in the input, never reusing any states until it gets to the
// end of the string.  This is the worst possible case for DFA execution.
std::string DeBruijnString(int n);

}  // namespace re2

#endif  // RE2_TESTING_STRING_GENERATOR_H_
