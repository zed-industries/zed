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

#ifndef FUZZTEST_FUZZTEST_INTERNAL_FIXTURE_DRIVER_H_
#define FUZZTEST_FUZZTEST_INTERNAL_FIXTURE_DRIVER_H_

#include <functional>
#include <iostream>
#include <memory>
#include <optional>
#include <string>
#include <string_view>
#include <tuple>
#include <type_traits>
#include <utility>
#include <vector>

#include "absl/functional/any_invocable.h"
#include "absl/status/status.h"
#include "absl/strings/str_format.h"
#include "absl/types/span.h"
#include "./fuzztest/internal/any.h"
#include "./fuzztest/internal/domains/domain.h"
#include "./fuzztest/internal/logging.h"
#include "./fuzztest/internal/meta.h"
#include "./fuzztest/internal/printer.h"
#include "./fuzztest/internal/registration.h"
#include "./fuzztest/internal/type_support.h"

namespace fuzztest::internal {

// The interface for test fixtures with setup and teardown methods that need to
// be explicitly called by the testing framework.
class FixtureWithExplicitSetUp {
 public:
  virtual ~FixtureWithExplicitSetUp() = default;

  virtual void SetUp() = 0;
  virtual void TearDown() = 0;

  static void SetUpTestSuite() {}
  static void TearDownTestSuite() {}
};

// Marker interfaces for specifying the fixture's instantiation semantics:
//
// -  Per-iteration semantics: The fixture object is instantiated and discarded
//    once per fuzz test iteration.
//
// -  Per-fuzz-test semantics: The fixture object is instantiated and discarded
//    once per fuzz test. The same  object is reused in all fuzz test
//    iterations.
class PerIterationFixture : public FixtureWithExplicitSetUp {};
class PerFuzzTestFixture : public FixtureWithExplicitSetUp {};

class UntypedFixtureDriver {
 public:
  virtual ~UntypedFixtureDriver() = 0;

  // DESIGN - FuzzTest runtime and fixture driver should follow the protocol
  // below to run a fuzz test and its iterations:
  //
  // For any fuzz test, runtime calls `RunFuzzTest(run_test)` {
  //   Fixture driver sets up the per-test fixture;
  //   Fixture driver calls `run_test` {
  //     Runtime runs fuzzing loop - for each iteration, runtime calls
  //     `RunFuzzTestIteration(run_iteration)` {
  //       Fixture driver sets up per-iteration fixture;
  //       Fixture driver calls `run_iteration` {
  //         Runtime calls `Test(...)` {
  //           Fixture driver runs the property function;
  //         }
  //       }
  //       Fixture driver tears down the per-iteration fixture;
  //     }
  //   }
  //   Fixture driver tears down the per-test fixture;
  // }
  //
  // The reason for the nested callbacks is to support complex setup e.g.
  // where the fixutre needs to run the test in a thread whose lifetime
  // is limited within the setup.

  // SetUp-Run-TearDowns a fuzz test using a move-only callback which should be
  // called once and only once.
  virtual void RunFuzzTest(absl::AnyInvocable<void() &&> run_test);

  // SetUp-Run-TearDowns a test iteration using a move-only callback which
  // should be called once and only once.
  virtual void RunFuzzTestIteration(
      absl::AnyInvocable<void() &&> run_iteration);

  // We take by rvalue ref to allow moving from it if necessary, but we want to
  // delay destroying the value until after instrumentation is turned off in the
  // caller.
  virtual void Test(MoveOnlyAny&& args_untyped) const = 0;

  virtual std::vector<GenericDomainCorpusType> GetSeeds() const = 0;
  virtual UntypedDomain GetDomains() const = 0;
};

// Typed subinterface with functionality that depends on knowing `ValueType`.
// `SeedProvider` is the type of the function returning dynamically initialized
// seeds.
template <typename ValueType, typename SeedProvider>
class TypedFixtureDriver : public UntypedFixtureDriver {
 public:
  TypedFixtureDriver(Domain<ValueType> domain,
                     std::vector<GenericDomainCorpusType> seeds,
                     const SeedProvider& seed_provider)
      : domain_(std::move(domain)),
        seeds_(std::move(seeds)),
        seed_provider_(seed_provider) {}

  TypedFixtureDriver(const TypedFixtureDriver&) = delete;
  TypedFixtureDriver& operator=(const TypedFixtureDriver&) = delete;
  TypedFixtureDriver(TypedFixtureDriver&& other) = delete;
  TypedFixtureDriver& operator=(TypedFixtureDriver&& other) = delete;

  std::vector<GenericDomainCorpusType> GetSeeds() const final {
    std::vector<GenericDomainCorpusType> seeds = GetSeedsFromSeedProvider();
    seeds.reserve(seeds.size() + seeds_.size());
    seeds.insert(seeds.end(), seeds_.begin(), seeds_.end());
    return seeds;
  }

  UntypedDomain GetDomains() const final { return domain_; }

 protected:
  const SeedProvider& seed_provider() const { return seed_provider_; }

  std::vector<GenericDomainCorpusType> GetSeedsFromUserValues(
      absl::Span<const ValueType> values) const {
    std::vector<GenericDomainCorpusType> seeds;
    seeds.reserve(values.size());
    for (const ValueType& val : values) {
      std::optional<GenericDomainCorpusType> corpus_value =
          domain_.FromValue(val);
      if (!corpus_value.has_value()) {
        const absl::Status status =
            absl::InvalidArgumentError("Could not turn value into corpus type");
        ReportBadSeed(val, status);
        continue;
      }

      const absl::Status status = domain_.ValidateCorpusValue(*corpus_value);
      if (!status.ok()) {
        ReportBadSeed(val, status);
        continue;
      }

      seeds.push_back(*std::move(corpus_value));
    }
    return seeds;
  }

 private:
  void ReportBadSeed(const ValueType& seed, const absl::Status& status) const {
    absl::FPrintF(GetStderr(), "\n[!] Skipping WithSeeds() value: %s:\n{",
                  status.ToString());
    AutodetectTypePrinter<ValueType>().PrintUserValue(
        seed, &std::cerr, domain_implementor::PrintMode::kHumanReadable);
    absl::FPrintF(GetStderr(), "}\n\n");
  }

  virtual std::vector<GenericDomainCorpusType> GetSeedsFromSeedProvider()
      const = 0;

  Domain<ValueType> domain_;
  std::vector<GenericDomainCorpusType> seeds_;
  const SeedProvider& seed_provider_;
};

// ForceVectorForStringView is a temporary hack for reliably
// finding buffer overflows. ASAN cannot detect small overflows in
// std::string-s. See related bug at
// https://bugs.llvm.org/show_bug.cgi?id=26380. As a temporary
// workaround, we enable finding overflows by copying the contents
// of the original string into a separate temporary heap buffer.
// TODO(b/194687521): Remove this when we (or ASAN) detect overflows
// in strings.
struct ForceVector {
  operator std::string_view() const { return {value.data(), value.size()}; }
  std::vector<char> value;
};

template <typename Dest, typename Src>
decltype(auto) ForceVectorForStringView(Src&& src) {
  // We only do this when Src is a std::string. If it's a string view it is
  // handled by the string view domain itself.
  if constexpr (std::is_same_v<void(std::decay_t<Dest>, std::decay_t<Src>),
                               void(std::string_view, std::string)>) {
    return ForceVector{std::vector<char>(src.begin(), src.end())};
  } else {
    return std::forward<Src>(src);
  }
}

// The base class for fixture drivers.
//
// A fixture driver is used for maintaining a fixture -- constructing it,
// setting it up, tearing it down, and destructing it -- during a fuzz test. It
// also acts as a proxy to the fixture's target function.
//
// The type parameters are:
//   - `DomainT`        -- the type of the domain. Eg `Domain<std::tuple<int>>`.
//   - `Fixture`        -- the type of the test fixture.
//   - `TargetFunction` -- the type of the fixture's target function.
//   - `SeedProvider`   -- the type of the function returning dynamically
//                         initialized seeds.
template <typename DomainT, typename Fixture, typename TargetFunction,
          typename SeedProvider>
class FixtureDriver;

// Specialization for `TargetFunction = void(BaseFixture::*)(Args...)`
//
// The new type parameters are:
//   - `BaseFixture` -- the class from which `Fixture` is derived and which has
//                      the target function.
//   - `Args...` -- the types of the target function's parameters.
template <typename DomainT, typename Fixture, typename BaseFixture,
          typename SeedProvider, typename... Args>
class FixtureDriver<DomainT, Fixture, void (BaseFixture::*)(Args...),
                    SeedProvider>
    : public TypedFixtureDriver<value_type_t<DomainT>, SeedProvider> {
 public:
  static_assert(std::is_base_of_v<BaseFixture, Fixture>);

  using TargetFunction = void (BaseFixture::*)(Args...);

  explicit FixtureDriver(TargetFunction target_function, DomainT domain,
                         std::vector<GenericDomainCorpusType> seeds,
                         const SeedProvider& seed_provider)
      : FixtureDriver::TypedFixtureDriver(std::move(domain), std::move(seeds),
                                          seed_provider),
        target_function_(target_function) {}

  void Test(MoveOnlyAny&& args_untyped) const override {
    FUZZTEST_INTERNAL_CHECK_PRECONDITION(
        fixture_ != nullptr,
        "fixture is nullptr. Did you forget to instantiate it in one of the "
        "SetUp methods?");
    std::apply(
        [&](auto&&... args) {
          (fixture_.get()->*target_function_)(
              ForceVectorForStringView<Args>(std::move(args))...);
        },
        args_untyped.GetAs<value_type_t<DomainT>>());
  }

  std::vector<GenericDomainCorpusType> GetSeedsFromSeedProvider() const final {
    if (this->seed_provider() == nullptr) return {};
    if constexpr (std::is_invocable_v<SeedProvider, Fixture*>) {
      static_assert(std::is_same_v<std::invoke_result_t<SeedProvider, Fixture*>,
                                   std::vector<value_type_t<DomainT>>>);
      FUZZTEST_INTERNAL_CHECK_PRECONDITION(
          fixture_ != nullptr,
          "fixture is nullptr. Did you forget to instantiate it in one of the "
          "SetUp methods?");
      return this->GetSeedsFromUserValues(
          std::invoke(this->seed_provider(), fixture_.get()));
    } else if constexpr (std::is_invocable_v<SeedProvider>) {
      static_assert(std::is_same_v<std::invoke_result_t<SeedProvider>,
                                   std::vector<value_type_t<DomainT>>>);
      return this->GetSeedsFromUserValues(std::invoke(this->seed_provider()));
    } else {
      return {};
    }
  }

 protected:
  // The fixture managed by the fixture driver.
  std::unique_ptr<Fixture> fixture_;

 private:
  TargetFunction target_function_;
};

// Specialization for `Fixture = NoFixture`.
// This is used for FUZZ_TEST invocations that do not require a fixture.
// TargetFunction must be `void(*)(Args...)`
//
// The new type parameters are:
//   - `Args...` -- the types of the target function's parameters.
template <typename DomainT, typename SeedProvider, typename... Args>
class FixtureDriver<DomainT, NoFixture, void (*)(Args...), SeedProvider>
    : public TypedFixtureDriver<value_type_t<DomainT>, SeedProvider> {
 public:
  using TargetFunction = void (*)(Args...);

  explicit FixtureDriver(TargetFunction target_function, DomainT domain,
                         std::vector<GenericDomainCorpusType> seeds,
                         const SeedProvider& seed_provider)
      : FixtureDriver::TypedFixtureDriver(std::move(domain), std::move(seeds),
                                          seed_provider),
        target_function_(target_function) {}

  void Test(MoveOnlyAny&& args_untyped) const override {
    std::apply(
        [&](auto&&... args) {
          target_function_(ForceVectorForStringView<Args>(std::move(args))...);
        },
        args_untyped.GetAs<value_type_t<DomainT>>());
  }

  std::vector<GenericDomainCorpusType> GetSeedsFromSeedProvider() const final {
    if (this->seed_provider() == nullptr) return {};
    if constexpr (std::is_invocable_v<SeedProvider>) {
      static_assert(std::is_same_v<std::invoke_result_t<SeedProvider>,
                                   std::vector<value_type_t<DomainT>>>);
      return this->GetSeedsFromUserValues(std::invoke(this->seed_provider()));
    } else {
      return {};
    }
  }

 private:
  TargetFunction target_function_;
};

template <typename DomainT, typename Fixture, typename TargetFunction,
          typename SeedProvider, typename = void>
class FixtureDriverImpl;

// The fixture driver for "NoFixture", which is the tag used for the FUZZ_TEST
// macro that uses no fixtures. No fixture is created.
template <typename DomainT, typename TargetFunction, typename SeedProvider>
class FixtureDriverImpl<DomainT, NoFixture, TargetFunction, SeedProvider> final
    : public FixtureDriver<DomainT, NoFixture, TargetFunction, SeedProvider> {
 public:
  using FixtureDriverImpl::FixtureDriver::FixtureDriver;
};

// The fixture driver for default-constructible classes that act like fixtures:
// their setup is in the constructor, teardown is in the destructor, and they
// have a target function. Such fixtures are instantiated and destructed once
// per fuzz test.
template <typename DomainT, typename Fixture, typename TargetFunction,
          typename SeedProvider>
class FixtureDriverImpl<
    DomainT, Fixture, TargetFunction, SeedProvider,
    std::enable_if_t<std::conjunction_v<
        std::is_default_constructible<Fixture>,
        std::negation<std::is_base_of<FixtureWithExplicitSetUp, Fixture>>>>>
    final
    : public FixtureDriver<DomainT, Fixture, TargetFunction, SeedProvider> {
 public:
  using FixtureDriverImpl::FixtureDriver::FixtureDriver;

  void RunFuzzTest(absl::AnyInvocable<void() &&> run_test) override {
    this->fixture_ = std::make_unique<Fixture>();
    if constexpr (Requires<Fixture>(
                      [](auto&& x) -> decltype(x.FuzzTestRunner(
                                       std::move(run_test))) {})) {
      this->fixture_->FuzzTestRunner(std::move(run_test));
    } else {
      std::move(run_test)();
    }
    this->fixture_ = nullptr;
  }

  void RunFuzzTestIteration(
      absl::AnyInvocable<void() &&> run_iteration) override {
    if constexpr (Requires<Fixture>(
                      [](auto&& x) -> decltype(x.FuzzTestIterationRunner(
                                       std::move(run_iteration))) {})) {
      this->fixture_->FuzzTestIterationRunner(std::move(run_iteration));
    } else {
      std::move(run_iteration)();
    }
  }
};

// The fixture driver for test fixtures with explicit setup that assume the
// "per-iteration" semantics.
template <typename DomainT, typename Fixture, typename BaseFixture,
          typename SeedProvider, typename... Args>
class FixtureDriverImpl<
    DomainT, Fixture, void (BaseFixture::*)(Args...), SeedProvider,
    std::enable_if_t<
        std::conjunction_v<std::is_default_constructible<Fixture>,
                           std::is_base_of<BaseFixture, Fixture>,
                           std::is_base_of<PerIterationFixture, Fixture>>>>
    final : public FixtureDriver<DomainT, Fixture,
                                 void (BaseFixture::*)(Args...), SeedProvider> {
 public:
  using FixtureDriverImpl::FixtureDriver::FixtureDriver;

  void RunFuzzTest(absl::AnyInvocable<void() &&> run_test) override {
    std::move(run_test)();
  }

  void RunFuzzTestIteration(
      absl::AnyInvocable<void() &&> run_iteration) override {
    this->fixture_ = std::make_unique<Fixture>();
    this->fixture_->SetUp();
    std::move(run_iteration)();
    this->fixture_->TearDown();
    this->fixture_ = nullptr;
  }
};

// The fixture driver for test fixtures with explicit setup that assume the
// "per-fuzz-test" semantics.
template <typename DomainT, typename Fixture, typename BaseFixture,
          typename SeedProvider, typename... Args>
class FixtureDriverImpl<
    DomainT, Fixture, void (BaseFixture::*)(Args...), SeedProvider,
    std::enable_if_t<
        std::conjunction_v<std::is_default_constructible<Fixture>,
                           std::is_base_of<BaseFixture, Fixture>,
                           std::is_base_of<PerFuzzTestFixture, Fixture>>>>
    final : public FixtureDriver<DomainT, Fixture,
                                 void (BaseFixture::*)(Args...), SeedProvider> {
 public:
  using FixtureDriverImpl::FixtureDriver::FixtureDriver;

  void RunFuzzTest(absl::AnyInvocable<void() &&> run_test) override {
    this->fixture_ = std::make_unique<Fixture>();
    this->fixture_->SetUp();
    std::move(run_test)();
    this->fixture_->TearDown();
    this->fixture_ = nullptr;
  }

  void RunFuzzTestIteration(
      absl::AnyInvocable<void() &&> run_iteration) override {
    std::move(run_iteration)();
  }
};

}  // namespace fuzztest::internal

#endif  // FUZZTEST_FUZZTEST_INTERNAL_FIXTURE_DRIVER_H_
