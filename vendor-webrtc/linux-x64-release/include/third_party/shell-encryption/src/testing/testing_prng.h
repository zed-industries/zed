/*
 * Copyright 2017 Google LLC.
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef RLWE_TESTING_TESTING_PRNG_H_
#define RLWE_TESTING_TESTING_PRNG_H_

#include <random>

#include "integral_types.h"
#include "prng/prng.h"

namespace rlwe {
namespace testing {

// An insecure pseudo-random number generator for testing.
class TestingPrng : public rlwe::SecurePrng {
 public:
  explicit TestingPrng(int seed) : generator_(seed) {}

  rlwe::StatusOr<Uint8> Rand8() override { return distr_(generator_); }

  rlwe::StatusOr<Uint64> Rand64() override { return distr_(generator_); }

 private:
  std::mt19937_64 generator_;
  std::uniform_int_distribution<Uint64> distr_;
};

}  // namespace testing
}  // namespace rlwe

#endif  // RLWE_TESTING_TESTING_PRNG_H_
