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

#ifndef FUZZTEST_FUZZTEST_INTERNAL_REGISTRATION_H_
#define FUZZTEST_FUZZTEST_INTERNAL_REGISTRATION_H_

#include <cstdio>
#include <cstdlib>
#include <functional>
#include <iostream>
#include <string>
#include <tuple>
#include <type_traits>
#include <utility>
#include <vector>

#include "absl/functional/any_invocable.h"
#include "absl/status/status.h"
#include "absl/strings/str_format.h"
#include "absl/types/span.h"
#include "./fuzztest/domain_core.h"
#include "./fuzztest/internal/domains/aggregate_of_impl.h"
#include "./fuzztest/internal/domains/domain.h"
#include "./fuzztest/internal/meta.h"
#include "./fuzztest/internal/printer.h"
#include "./fuzztest/internal/type_support.h"

namespace fuzztest {
namespace internal {

struct BasicTestInfo {
  std::string suite_name;
  std::string test_name;
  std::string file;
  int line = 0;
  bool uses_fixture = false;
};

// Use base classes to progressively add members/behavior to the registerer
// object. This way we can statically assert that certain functions are called
// in the right order.

struct NoFixture {};

// Initial base class. No custom domain, no seeds.
template <typename... Args>
struct DefaultRegistrationBase {
  static constexpr bool kHasDomain = false;
  static constexpr bool kHasSeeds = false;
  static constexpr bool kHasSeedProvider = false;
  static constexpr size_t kNumArgs = sizeof...(Args);

  static_assert((std::is_same_v<Args, std::decay_t<Args>> && ...));

  Domain<std::tuple<Args...>> GetDomains() const {
    return TupleOf(Arbitrary<Args>()...);
  }

  using SeedT = std::tuple<Args...>;
};

template <typename Fixture, typename BaseFixture, typename... Args>
DefaultRegistrationBase<std::decay_t<Args>...> DefaultRegistrationBaseImpl(
    Fixture*, void (BaseFixture::*)(Args...));

template <typename... Args>
DefaultRegistrationBase<std::decay_t<Args>...> DefaultRegistrationBaseImpl(
    NoFixture*, void (*)(Args...));

template <typename Fixture, typename TargetFunction>
using DefaultRegistrationBaseT = decltype(DefaultRegistrationBaseImpl(
    static_cast<Fixture*>(nullptr), static_cast<TargetFunction>(nullptr)));

// A custom domain was specified.
template <typename... Args>
struct RegistrationWithDomainsBase {
  static constexpr bool kHasDomain = true;
  static constexpr bool kHasSeeds = false;
  static constexpr bool kHasSeedProvider = false;
  static constexpr size_t kNumArgs = sizeof...(Args);

  Domain<std::tuple<Args...>> domains_;

  const auto& GetDomains() const { return domains_; }

  using SeedT = decltype(domains_.GetValue({}));
};

// Seeds were specified. It derived from the existing base to augment it.
template <typename Base>
struct RegistrationWithSeedsBase : Base {
  static constexpr bool kHasSeeds = true;

  explicit RegistrationWithSeedsBase(Base base) : Base(std::move(base)) {}

  std::vector<
      corpus_type_t<std::decay_t<decltype(std::declval<Base>().GetDomains())>>>
      seeds_;
};

template <typename Base, typename SeedProvider>
struct RegistrationWithSeedProviderBase : Base {
  static constexpr bool kHasSeedProvider = true;

  explicit RegistrationWithSeedProviderBase(Base base,
                                            SeedProvider seed_provider)
      : Base(std::move(base)), seed_provider_(std::move(seed_provider)) {}

  SeedProvider seed_provider_;
};

class PerIterationFixture;
struct RegistrationToken;

template <typename Fixture, typename TargetFunction,
          typename Base = DefaultRegistrationBaseT<Fixture, TargetFunction>,
          typename SeedProvider = void*>
class Registration : private Base {
  using SeedT = typename Base::SeedT;

 public:
  explicit Registration(BasicTestInfo info, TargetFunction target_function)
      : test_info_(std::move(info)), target_function_(target_function) {}

  // Registers domains for a property function as a `TupleOf` individual
  // domains. This is useful when the domains are specified indirectly, e.g.,
  // when they are returned from a helper function. For example:
  //
  // auto StringAndIndex(int size) {
  //   return TupleOf(String().WithSize(size), InRange(0, size - 1));
  // }
  //
  // void MyProperty(std::string s, int i) { ... }
  // FUZZ_TEST(MySuite, MyProperty).WithDomains(StringAndIndex(10));
  template <typename... NewDomains>
  auto WithDomains(AggregateOfImpl<std::tuple<value_type_t<NewDomains>...>,
                                   RequireCustomCorpusType::kNo, NewDomains...>
                       domain) && {
    static_assert(!Registration::kHasDomain,
                  "WithDomains can only be called once.");
    static_assert(!Registration::kHasSeeds,
                  "WithDomains can not be called after WithSeeds.");
    static_assert(!Registration::kHasSeedProvider,
                  "WithDomains can not be called after WithSeedProvider.");
    static_assert(
        Base::kNumArgs == sizeof...(NewDomains),
        "Number of domains specified in .WithDomains() does not match "
        "the number of function parameters.");
    using NewBase = RegistrationWithDomainsBase<value_type_t<NewDomains>...>;
    return Registration<Fixture, TargetFunction, NewBase, SeedProvider>(
        std::move(test_info_), target_function_, NewBase{std::move(domain)});
  }

  // Registers a domain for each parameter of the property function. This is the
  // recommended approach when domains are explicitly listed as part of the fuzz
  // test definition. For example:
  //
  // void MyProperty(std::string s, int n) { ... }
  // FUZZ_TEST(MySuite, MyProperty).WithDomains(String(), Positive<int>());
  template <typename... NewDomains>
  auto WithDomains(NewDomains&&... domains) && {
    return std::move(*this).WithDomains(
        TupleOf(std::forward<NewDomains>(domains)...));
  }

  void ReportBadSeed(const SeedT& seed, const absl::Status& status) {
    absl::FPrintF(stderr,
                  "\n[!] Skipping WithSeeds() value in %s.%s:\n%s:%d: %s:\n{",
                  test_info_.suite_name, test_info_.test_name, test_info_.file,
                  test_info_.line, status.ToString());

    // We use a direct call to PrintUserValue because we don't have a
    // corpus_type object to pass to PrintValue.
    bool first = true;
    const auto print_one_arg = [&](auto I) {
      using value_type = std::decay_t<std::tuple_element_t<I, SeedT>>;
      AutodetectTypePrinter<value_type>().PrintUserValue(
          std::get<I>(seed), &std::cerr,
          domain_implementor::PrintMode::kHumanReadable);
      if (!first) absl::FPrintF(stderr, ", ");
      first = false;
    };
    ApplyIndex<Base::kNumArgs>([&](auto... I) { (print_one_arg(I), ...); });
    absl::FPrintF(stderr, "}\n\n");
  }

  auto WithSeeds(absl::Span<const SeedT> seeds) && {
    if constexpr (!Registration::kHasSeeds) {
      return Registration<Fixture, TargetFunction,
                          RegistrationWithSeedsBase<Base>, SeedProvider>(
                 std::move(test_info_), target_function_,
                 RegistrationWithSeedsBase<Base>(std::move(*this)))
          .WithSeeds(seeds);
    } else {
      const auto& domains = this->GetDomains();
      for (const auto& seed : seeds) {
        auto corpus_value = domains.FromValue(seed);
        if (!corpus_value) {
          const absl::Status status = absl::InvalidArgumentError(
              "Could not turn value into corpus type");
          ReportBadSeed(seed, status);
          continue;
        }
        const absl::Status status = domains.ValidateCorpusValue(*corpus_value);
        if (!status.ok()) {
          ReportBadSeed(seed, status);
          continue;
        }
        this->seeds_.push_back(*std::move(corpus_value));
      }
      return std::move(*this);
    }
  }

  // Registers a (free) function that returns a vector of seeds. This can be
  // used when the seeds need to be initialized dynamically. For example, if the
  // seeds depend on any global variables, this is a way to resolve the static
  // initialization order fiasco.
  auto WithSeeds(
      absl::AnyInvocable<std::vector<SeedT>() const> seed_provider) && {
    static_assert(
        !Registration::kHasSeedProvider,
        "WithSeeds that registers a seed provider can only be called once.");
    return Registration<
        Fixture, TargetFunction,
        RegistrationWithSeedProviderBase<Base, decltype(seed_provider)>,
        decltype(seed_provider)>(
        std::move(test_info_), target_function_,
        RegistrationWithSeedProviderBase<Base, decltype(seed_provider)>(
            std::move(*this), std::move(seed_provider)));
  }

  // Registers a member function of `Fixture` (or any of its base classes
  // `BaseFixture`) that returns a vector of seeds. This is useful if the seeds
  // not only need to be initialized dynamically (e.g., to avoid the static
  // initialization order fiasco), but they also depend on the fixture's
  // internal state.
  //
  // NOTE: `Fixture` cannot be a fixture that uses the "per-iteration" semantics
  // (see GoogleTest fixture adapters for details).
  template <typename BaseFixture>
  auto WithSeeds(std::vector<SeedT> (BaseFixture::*seed_provider)()) && {
    static_assert(std::is_base_of_v<BaseFixture, Fixture>,
                  "Seed provider that is a member function must be a member of "
                  "the fixture.");
    static_assert(!std::is_base_of_v<PerIterationFixture, Fixture>,
                  "Seed provider cannot be a member of a fixture that uses "
                  "\"per-iteration\" semantics.");
    static_assert(
        !Registration::kHasSeedProvider,
        "WithSeeds that registers a seed provider can only be called once.");
    using NewSeedProvider =
        absl::AnyInvocable<std::vector<SeedT>(BaseFixture*) const>;
    using NewBase = RegistrationWithSeedProviderBase<Base, NewSeedProvider>;
    return Registration<Fixture, TargetFunction, NewBase, NewSeedProvider>(
        std::move(test_info_), target_function_,
        NewBase(std::move(*this), std::mem_fn(seed_provider)));
  }

 private:
  std::vector<GenericDomainCorpusType> seeds() const {
    if constexpr (Base::kHasSeeds) {
      return this->seeds_;
    } else {
      return {};
    }
  }

  SeedProvider seed_provider() {
    if constexpr (Base::kHasSeedProvider) {
      return std::move(this->seed_provider_);
    } else {
      return {};
    }
  }

  template <typename, typename, typename, typename>
  friend class Registration;
  friend struct RegistrationToken;

  explicit Registration(BasicTestInfo info, TargetFunction target_function,
                        Base base)
      : Base(std::move(base)),
        test_info_(std::move(info)),
        target_function_(target_function) {}

  BasicTestInfo test_info_;
  TargetFunction target_function_;
};

}  // namespace internal

// Returns a registration for a fuzz test based on `Fixture` and
// `target_function`. `target_function` must be a pointer to a member function
// of `Fixture` or its base class.
//
// This is an advanced API; in almost all cases you should prefer registring
// your fixture-based fuzz tests with the FUZZ_TEST_F macro. Unlike the macro,
// this function allows customizing `suite_name`, `test_name`, `file`, and
// `line`. For example, it is suitable when you want to register fuzz tests with
// dynamically generated test names.
//
// Note: If `Fixture` is a GoogleTest fixture, make sure that all fuzz tests
// registered with `suite_name` also use `Fixture`. Otherwise, you may encounter
// undefined behavior when it comes to test suite setup and teardown.
template <typename Fixture, typename TargetFunction>
auto GetRegistrationWithFixture(std::string suite_name, std::string test_name,
                                std::string file, int line,
                                TargetFunction target_function) {
  return ::fuzztest::internal::Registration<Fixture, TargetFunction>(
      ::fuzztest::internal::BasicTestInfo{std::move(suite_name),
                                          std::move(test_name), std::move(file),
                                          line, true},
      target_function);
}

// Returns a registration for a fuzz test based on `target_function`, which
// should be a function pointer.
//
// This is an advanced API; in almost all cases you should prefer registring
// your fuzz tests with the FUZZ_TEST macro. Unlike the macro, this function
// allows customizing `suite_name`, `test_name`, `file`, and `line`. For
// example, it is suitable when you want to register fuzz tests with dynamically
// generated test names.
template <typename TargetFunction>
auto GetRegistration(std::string suite_name, std::string test_name,
                     std::string file, int line,
                     TargetFunction target_function) {
  return ::fuzztest::internal::Registration<::fuzztest::internal::NoFixture,
                                            TargetFunction>(
      ::fuzztest::internal::BasicTestInfo{std::move(suite_name),
                                          std::move(test_name), std::move(file),
                                          line, false},
      target_function);
}

}  // namespace fuzztest

#endif  // FUZZTEST_FUZZTEST_INTERNAL_REGISTRATION_H_
