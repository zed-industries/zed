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

#ifndef RLWE_CONSTANTS_H_
#define RLWE_CONSTANTS_H_

#include <cstdint>

#include "absl/numeric/int128.h"
#include "integral_types.h"

namespace rlwe {

// To generate parameters for this Ring-LWE system, we need to choose a prime
// modulus q. In order for us to perform NTT transforms on dimension 2^N
// polynomials, we require 2^{N+1} to divide q-1. Thus, we set moduli to be
// primes of the form q = p*2^{N+1} + 1, where N becomes our LogDegreeBound.

// Parameters from the New Hope key exchange protocol. Note that we are not
// using these parameters for key exchange in this library.
constexpr Uint64 kNewhopeModulus = 12289;
constexpr Uint64 kNewhopeLogDegreeBound = 10;
constexpr Uint64 kNewhopeDegreeBound = 1 << kNewhopeLogDegreeBound;

// Montgomery parameters for a 59-bit modulus.
constexpr Uint64 kModulus59 = 332366567264636929;
constexpr Uint64 kInvModulus59 = 7124357790306815999;
constexpr Uint64 kLogDegreeBound59 = 10;
constexpr Uint64 kDegreeBound59 = 1L << kLogDegreeBound59;

// RLWE parameters for a 44-bit modulus.
constexpr Uint64 kModulus44 = 17592169240577;
constexpr Uint64 kLogDegreeBound44 = 10;
constexpr Uint64 kDegreeBound44 = 1L << kLogDegreeBound44;

// RLWE parameters for 128-element database in PIR.
constexpr Uint64 kModulus1PIR128 = 17592173449217;  // 44-bit modulus
constexpr Uint64 kModulus2PIR128 = 8380417;         // 23-bit modulus
constexpr Uint64 kLogTPIR128 = 10;
constexpr Uint64 kLogDegreeBoundPIR128 = 10;
constexpr Uint64 kDegreeBoundPIR128 = 1L << kLogDegreeBoundPIR128;

// RLWE parameters for 256-element database in PIR.
constexpr Uint64 kModulus1PIR256 = 8796087789569;  // 43-bit modulus
constexpr Uint64 kModulus2PIR256 = 2056193;        // 21-bit modulus
constexpr Uint64 kLogTPIR256 = 9;
constexpr Uint64 kLogDegreeBoundPIR256 = 10;
constexpr Uint64 kDegreeBoundPIR256 = 1L << kLogDegreeBoundPIR256;

// RLWE parameters for a 25-bit and a 29-bit moduli, both congruent to 4 modulo
// 5 = (1 << 2) + 1. These moduli will be useful for testing of modulus
// switching.
constexpr Uint64 kModulus29 = 463187969;
constexpr Uint64 kLogDegreeBound29 = 10;
constexpr Uint64 kDegreeBound29 = 1L << kLogDegreeBound29;
constexpr Uint64 kModulus25 = 33538049;
constexpr Uint64 kLogDegreeBound25 = 10;
constexpr Uint64 kDegreeBound25 = 1L << kLogDegreeBound25;

// RLWE parameters for an 80-bit modulus.
// The modulus represented in decimal is 646119422561999443726337.
constexpr absl::uint128 kModulus80 =
    absl::MakeUint128(35026, 3764636248688824321);
constexpr Uint64 kLogDegreeBound80 = 11;
constexpr Uint64 kDegreeBound80 = 1L << kLogDegreeBound80;

constexpr Uint64 kMaxNumCoeffs = 1L << 15;
constexpr Uint64 kMaxLogNumCoeffs = 15;
constexpr Uint64 kMaxVariance = 256;

}  // namespace rlwe

#endif  // RLWE_CONSTANTS_H_
