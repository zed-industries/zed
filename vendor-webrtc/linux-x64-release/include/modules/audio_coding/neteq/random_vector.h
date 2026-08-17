/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_NETEQ_RANDOM_VECTOR_H_
#define MODULES_AUDIO_CODING_NETEQ_RANDOM_VECTOR_H_

#include <stddef.h>
#include <stdint.h>

namespace webrtc {

// This class generates pseudo-random samples.
class RandomVector {
 public:
  static const size_t kRandomTableSize = 256;
  static const int16_t kRandomTable[kRandomTableSize];

  RandomVector() : seed_(777), seed_increment_(1) {}

  RandomVector(const RandomVector&) = delete;
  RandomVector& operator=(const RandomVector&) = delete;

  void Reset();

  void Generate(size_t length, int16_t* output);

  void IncreaseSeedIncrement(int16_t increase_by);

  // Accessors and mutators.
  int16_t seed_increment() { return seed_increment_; }
  void set_seed_increment(int16_t value) { seed_increment_ = value; }

 private:
  uint32_t seed_;
  int16_t seed_increment_;
};

}  // namespace webrtc
#endif  // MODULES_AUDIO_CODING_NETEQ_RANDOM_VECTOR_H_
