// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CANVAS_INTERVENTIONS_NOISE_HASH_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CANVAS_INTERVENTIONS_NOISE_HASH_H_

#include <cstdint>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

// Uses the FNV1a hash function as a pseudo-random number generator. The caller
// should make sure that |Update| is called before the 64 bits of the current
// token hash have been consumed by |GetValueBelow|.
class CORE_EXPORT NoiseHash {
 public:
  // This hashes the provided token with the partition string.
  NoiseHash(const uint64_t token, const String& partition);

  // Computes a new pseudo-random value by hashing with the provided value.
  void Update(const uint64_t value);

  // Returns a (pseudo-)random value that is less than the provided |max_value|.
  // This consumes log2(max_value) bits of the current token hash. NEVER call
  // this function when insufficient bits of the hash are remaining. To reset
  // the available bits, use Update().
  int GetValueBelow(const int max_value);

  // Used for testing the correctness of the generated token hash.
  uint64_t GetTokenHashForTesting() const;

 private:
  uint64_t token_hash_ = 0;
  int remaining_bits_ = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CANVAS_INTERVENTIONS_NOISE_HASH_H_
