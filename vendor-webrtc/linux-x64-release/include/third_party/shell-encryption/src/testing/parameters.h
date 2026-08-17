/*
 * Copyright 2020 Google LLC.
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

#ifndef RLWE_TESTING_INSTANCES_H_
#define RLWE_TESTING_INSTANCES_H_

#include <array>
#include <tuple>

#include <gmock/gmock.h>
#include "absl/numeric/int128.h"
#include "constants.h"
#include "context.h"
#include "integral_types.h"
#include "montgomery.h"

namespace rlwe {
namespace testing {

// ModularInt types for typed tests. A typed test can be defined as follows in
// test files.
//
// template <typename ModularInt>
// class TemplatedTest : public ::testing::Test {};
// TYPED_TEST_SUITE(TemplatedTest, rlwe::testing::ModularIntTypes);
//
// When a new type is added, one must also specify parameters for testing below.
typedef ::testing::Types<
    rlwe::MontgomeryInt<Uint16>, rlwe::MontgomeryInt<Uint32>,
    rlwe::MontgomeryInt<Uint64>, rlwe::MontgomeryInt<absl::uint128>>
    ModularIntTypes;

// Parameters for testing. These parameters must be specialized for each of the
// ModularIntTypes above. By default, they contain an empty array.
// In a typed test, one can iterate over all the context parameters, and create
// a context as follows.
//
// for (const auto& params : rlwe::testing::ContextParameters<TypeParam>::value)
// {
//   ASSERT_OK_AND_ASSIGN(auto context,
//   rlwe::RlweContext<TypeParam>::Create(params));
//   // Your test code here.
// }
template <typename ModularInt>
struct ContextParameters {
  static std::vector<typename RlweContext<ModularInt>::Parameters> Value() {
    return {};
  }
};

template <>
struct ContextParameters<MontgomeryInt<Uint16>> {
  static std::vector<RlweContext<MontgomeryInt<Uint16>>::Parameters> Value() {
      return {
          RlweContext<MontgomeryInt<Uint16>>::Parameters{
              .modulus = kNewhopeModulus,
              .log_n = 10,
              .log_t = 1,
              .variance = 8},
      };
  }
};

template <>
struct ContextParameters<MontgomeryInt<Uint32>> {
  static std::vector<RlweContext<MontgomeryInt<Uint32>>::Parameters> Value() {
      return {
          RlweContext<MontgomeryInt<Uint32>>::Parameters{
              .modulus = kModulus25, .log_n = 10, .log_t = 1, .variance = 8},
          RlweContext<MontgomeryInt<Uint32>>::Parameters{
              .modulus = kModulus29, .log_n = 11, .log_t = 5, .variance = 8},
      };
  }
};

template <>
struct ContextParameters<MontgomeryInt<Uint64>> {
  static std::vector<RlweContext<MontgomeryInt<Uint64>>::Parameters> Value() {
      return {
          RlweContext<MontgomeryInt<Uint64>>::Parameters{
              .modulus = kModulus59, .log_n = 11, .log_t = 10, .variance = 8},
      };
  }
};

template <>
struct ContextParameters<MontgomeryInt<absl::uint128>> {
  static std::vector<RlweContext<MontgomeryInt<absl::uint128>>::Parameters> Value() {
      return {
          RlweContext<MontgomeryInt<absl::uint128>>::Parameters{
              .modulus = kModulus80, .log_n = 11, .log_t = 11, .variance = 8},
      };
  }
};

// Parameters for testing of modulus switching. These parameters must be
// specialized for (some of the) ModularIntTypes above. By default, they contain
// empty arrays of tuples. This is the case for
// rlwe::MontgomeryInt<rlwe::Uint16> and rlwe::MontgomeryInt<rlwe::Uint32>.
//
// In a typed test, one can iterate over all the parameters, and create
// context's as follows.
//
// for (const auto& [params1, params2] :
//      rlwe::testing::ContextParametersModulusSwitching<TypeParam>::value)
// {
//   ASSERT_OK_AND_ASSIGN(auto context1,
//   rlwe::RlweContext<TypeParam>::Create(params1));
//   ASSERT_OK_AND_ASSIGN(auto context2,
//   rlwe::RlweContext<TypeParam>::Create(params2));
//   // Your test code here.
// }
template <typename ModularInt>
struct ContextParametersModulusSwitching {
  using Params = typename RlweContext<ModularInt>::Parameters;
  static std::vector<std::tuple<Params, Params>> Value() {
    return {};
  }
};

template <>
struct ContextParametersModulusSwitching<MontgomeryInt<Uint64>> {
  using Params = typename RlweContext<MontgomeryInt<Uint64>>::Parameters;
  static std::vector<std::tuple<Params, Params>> Value() {
    return {
      std::make_tuple(
          Params{.modulus = 17592186028033ULL,
                 .log_n = 10,
                 .log_t = 4,
                 .variance = 8},
          Params{
              .modulus = 1589249ULL, .log_n = 10, .log_t = 4, .variance = 8})};
  }
};

template <>
struct ContextParametersModulusSwitching<MontgomeryInt<absl::uint128>> {
  using Params = typename RlweContext<MontgomeryInt<absl::uint128>>::Parameters;
  static std::vector<std::tuple<Params, Params>> Value() {
    return {
      std::make_tuple(
          Params{.modulus = absl::MakeUint128(4611686018427387903ULL,
                                              18446744073709355009ULL),
                 .log_n = 10,
                 .log_t = 2,
                 .variance = 8},
          Params{.modulus = 35184371961857ULL,
                 .log_n = 10,
                 .log_t = 2,
                 .variance = 8})};
  }
};

}  // namespace testing
}  // namespace rlwe

#endif  // RLWE_TESTING_INSTANCES_H_
