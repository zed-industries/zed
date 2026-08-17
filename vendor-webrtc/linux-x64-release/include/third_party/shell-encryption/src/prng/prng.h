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

#ifndef RLWE_PRNG_H_
#define RLWE_PRNG_H_

#include <cstdint>

#include "absl/strings/string_view.h"
#include "integral_types.h"
#include "statusor.h"

namespace rlwe {

// An interface for a secure pseudo-random number generator.
class SecurePrng {
 public:
  virtual rlwe::StatusOr<Uint8> Rand8() = 0;
  virtual rlwe::StatusOr<Uint64> Rand64() = 0;
  virtual ~SecurePrng() = default;
  static rlwe::StatusOr<std::unique_ptr<SecurePrng>> Create(
      absl::string_view seed);
  static rlwe::StatusOr<std::string> GenerateSeed();
  static int SeedLength();
};

}  // namespace rlwe

#endif  // RLWE_PRNG_H_
