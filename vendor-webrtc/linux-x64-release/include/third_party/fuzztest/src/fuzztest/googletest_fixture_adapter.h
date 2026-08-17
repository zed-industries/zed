// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef FUZZTEST_FUZZTEST_GOOGLETEST_FIXTURE_ADAPTER_H_
#define FUZZTEST_FUZZTEST_GOOGLETEST_FIXTURE_ADAPTER_H_

#include <type_traits>

#include "gtest/gtest.h"
#include "./fuzztest/internal/fixture_driver.h"

namespace fuzztest {
namespace internal {

template <typename Fixture, typename InstantiationType, typename = void>
class GoogleTestFixtureAdapter;

template <typename Fixture, typename InstantiationType>
class GoogleTestFixtureAdapter<
    Fixture, InstantiationType,
    std::enable_if_t<std::conjunction_v<
        std::is_base_of<::testing::Test, Fixture>,
        std::is_base_of<FixtureWithExplicitSetUp, InstantiationType>>>>
    : public Fixture, public virtual InstantiationType {
 public:
  void SetUp() override { Fixture::SetUp(); }
  void TearDown() override { Fixture::TearDown(); }

  static void SetUpTestSuite() { Fixture::SetUpTestSuite(); }
  static void TearDownTestSuite() { Fixture::TearDownTestSuite(); }

 private:
  void TestBody() override {}
};

}  // namespace internal

// Adapts the GoogleTest fixture class `Fixture` so that it can be used with the
// FUZZ_TEST_F macro.
//
// The fixture is adapted using the "per-iteration" semantics: it will be
// instantiated, set up, torn down, and discarded once per fuzz test iteration.
// That is, each call to the fixture's property function will be on a fresh
// fixture object.
//
// It is always safe to use this adapter, since the "per-iteration" semantics
// observes the GoogleTest invariant that a fixture object is never reused in
// multiple tests or multiple runs of the same test. However, if initializing
// the fixture is expensive, the resulting fuzz test may be slow and
// ineffective.
//
// Note: Only use GoogleTest fixtures in fuzz tests if they are also used in
// unit tests. For more details, see
// https://github.com/google/fuzztest/blob/main/doc/fixtures.md.
//
// Example:
//
// class SumVecTest : public testing::Test {
//  public:
//   SumVecTest() : vec_{1, 2, 3} {}
//
//  protected:
//   std::vector<int> vec_;
// };
//
// class SumVecFuzzTest
//     : public fuzztest::PerIterationFixtureAdapter<SumVecTest> {
//  public:
//   void SumsLastEntry(int last_entry) {
//     int previous_sum = SumVec(vec_);
//     vec_.push_back(last_entry);
//     EXPECT_EQ(SumVec(vec_), previous_sum + last_entry);
//   }
// };
//
// FUZZ_TEST_F(SumVecFuzzTest, SumsLastEntry);
//
template <typename Fixture>
using PerIterationFixtureAdapter =
    ::fuzztest::internal::GoogleTestFixtureAdapter<
        Fixture, ::fuzztest::internal::PerIterationFixture>;

// Adapts the GoogleTest fixture class `Fixture` so that it can be used with the
// FUZZ_TEST_F macro.
//
// The fixture is adapted using the "per-fuzz-test" semantics: it will be
// instantiated, set up, torn down, and discarded once per fuzz test, and reused
// across all test iterations. That is, each call to the fixture's property
// function will be on the same fixture object.
//
// Use this adapter when initializing the fixture is too expensive to be
// performed in each fuzz test iteration. However, note that the "per-fuzz-test"
// semantics breaks the GoogleTest invariant that a fixture object is never
// reused in multiple tests or multiple runs of the same test. In particular,
// make sure that your property function resets the fixture so that each fuzz
// test iteration starts in the same state.
//
// Note: Only use GoogleTest fixtures in fuzz tests if they are also used in
// unit tests. For more details, see
// https://github.com/google/fuzztest/doc/fuzztest-fixtures.md.
//
// Example:
//
// class EchoServerTest : public testing::Test {
//  public:
//   EchoServerTest() { server_.Start("localhost:9999"); }
//   ~EchoServerTest() override { server_.Stop(); }

//  private:
//   EchoServer server_;
// };
//
// class EchoServerFuzzTest
//     : public fuzztest::PerFuzzTestFixtureAdapter<EchoServerTest> {
//  public:
//   void ReturnsTheSameString(const std::string& request) {
//     std::string response;
//     SendRequest("localhost:9999", request, &response);
//     EXPECT_EQ(response, request);
//   }
// };
//
// FUZZ_TEST_F(EchoServerFuzzTest, ReturnsTheSameString);
//
template <typename Fixture>
using PerFuzzTestFixtureAdapter =
    ::fuzztest::internal::GoogleTestFixtureAdapter<
        Fixture, ::fuzztest::internal::PerFuzzTestFixture>;

}  // namespace fuzztest

#endif  // FUZZTEST_FUZZTEST_GOOGLETEST_FIXTURE_ADAPTER_H_
