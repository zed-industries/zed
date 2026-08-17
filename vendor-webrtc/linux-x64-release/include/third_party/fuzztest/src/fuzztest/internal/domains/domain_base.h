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

// IWYU pragma: private, include "fuzztest/fuzztest.h"
// IWYU pragma: friend fuzztest/.*

#ifndef FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_DOMAIN_BASE_H_
#define FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_DOMAIN_BASE_H_

#include <cstdint>
#include <cstdlib>
#include <functional>
#include <iostream>
#include <optional>
#include <type_traits>
#include <vector>

#include "absl/random/bit_gen_ref.h"
#include "absl/random/distributions.h"
#include "absl/status/status.h"
#include "absl/strings/str_format.h"
#include "./fuzztest/internal/domains/domain_type_erasure.h"
#include "./fuzztest/internal/domains/mutation_metadata.h"  // IWYU pragma: export
#include "./fuzztest/internal/meta.h"
#include "./fuzztest/internal/printer.h"
#include "./fuzztest/internal/serialization.h"
#include "./fuzztest/internal/type_support.h"

namespace fuzztest::domain_implementor {

// `DomainBase` is the base class for all domain implementations.
//
// The type parameters are as follows:
//
// - `Derived`: Instead of virtual inheritance, the domain hierarchy uses the
//   Curiously Recurring Template Pattern (CRTP). A class `X` deriving from
//   `DomainBase` needs to instantiate the template with `Derived = X`.
// - `ValueType`: The type of the user-facing values the domain represents.
//   If the deriving class is itself a template parameterized by its user value
//   type (as the first type parameter), you may omit this parameter.
// - `CorpusType`: The type of the corpus values that the domain implementation
//   uses internally. Typically this is the same as `ValueType` (which is the
//   default), but sometimes the domain may need to maintain different or
//   additional information to be able to efficiently perform value mutation.
//
// The domain implementation can access `ValueType` and `CorpusType` through
// the type members `DomainBase::value_type` and `DomainBase::corpus_type`.
//
// To implement a domain, a deriving class should implement at least the
// following methods. See the documentation of the interface class `Domain<T>`
// for a detailed description of each method.
//
// - corpus_type Init(absl::BitGenRef prng)
//
//   The method that returns an initial value. To support seeding, this function
//   may first call `MaybeGetRandomSeed()` and return its non-nullopt result.
//   TODO(b/303324603): Move the call to `MaybeGetRandomSeed()` to a wrapper.
//
// - void Mutate(corpus_type& val, absl::BitGenRef prng,
//               const MutationMetadata& metadata, bool only_shrink)
//
//   The method that mutates the corpus value.
//
// - absl::Status ValidateCorpusValue(const corpus_type& val) const
//
//   The method to validate that the corpus value satisfies the domain's
//   constraints.
//
// - auto GetPrinter() const
//
//   The method that returns the domain's printer---a class that provides one of
//   the following two methods for printing user/corpus values.
//   TODO(b/303324603): Add more documentation about printers.
//
//     void PrintUserValue(const value_type& val,
//                         RawSink out,
//                         PrintMode mode) const
//
//     void PrintCorpusValue(const corpus_type& val,
//                           RawSink out,
//                           PrintMode mode) const
//
// In addition, if the domain has a custom `CorpusType`, it will need to
// override (by shadowing) the methods `GetValue()`, `FromValue()`,
// `ParseCorpus()`, and `SerializeCorpus()`. See the default implementations of
// these methods below.
template <typename Derived,
          typename ValueType =
              fuzztest::internal::ExtractTemplateParameter<0, Derived>,
          typename CorpusType = ValueType>
class DomainBase {
 public:
  using value_type = ValueType;
  using corpus_type = CorpusType;
  static constexpr bool has_custom_corpus_type =
      !std::is_same_v<ValueType, CorpusType>;

  DomainBase() {
    // Check that the interface of `Derived` matches the requirements for a
    // domain implementation. We check these inside the constructor of
    // `DomainBase`, where `Derived` is already fully defined. If we try to
    // check them at class scope we would see an incomplete `Derived` class and
    // the checks would not work.
    fuzztest::internal::CheckIsSame<
        ValueType, fuzztest::internal::value_type_t<Derived>>();
    fuzztest::internal::CheckIsSame<
        CorpusType, fuzztest::internal::corpus_type_t<Derived>>();
    static_assert(has_custom_corpus_type == Derived::has_custom_corpus_type);

    // Check that `Derived` type-checks against the type-erased `Domain<T>`
    // interface by forcing the `DomainModel` template specialization.
    if (Derived* domain = nullptr) {
      (void)fuzztest::internal::DomainModel<Derived>{*domain};
    }
  }

  // Returns a random user value from the domain. In general, doesn't provide
  // guarantees on the distribution of the returned values. Provides the same
  // stability/reproducibility guarantees as `prng`.
  ValueType GetRandomValue(absl::BitGenRef prng) {
    return derived().GetValue(derived().GetRandomCorpusValue(prng));
  }

  // Default GetValue and FromValue functions for !has_custom_corpus_type
  // domains.
  ValueType GetValue(const ValueType& v) const {
    static_assert(!has_custom_corpus_type);
    return v;
  }
  std::optional<ValueType> FromValue(const ValueType& v) const {
    static_assert(!has_custom_corpus_type);
    return v;
  }

  std::optional<CorpusType> ParseCorpus(const internal::IRObject& obj) const {
    static_assert(!has_custom_corpus_type);
    return obj.ToCorpus<CorpusType>();
  }

  internal::IRObject SerializeCorpus(const CorpusType& v) const {
    static_assert(!has_custom_corpus_type);
    return internal::IRObject::FromCorpus(v);
  }

  void UpdateMemoryDictionary(const CorpusType& val, ConstCmpTablesPtr) {}

  uint64_t CountNumberOfFields(const CorpusType&) { return 0; }

  uint64_t MutateSelectedField(CorpusType&, absl::BitGenRef,
                               const MutationMetadata&, bool, uint64_t) {
    return 0;
  }

  // Stores `seeds` to be occasionally sampled from during value initialization.
  // When called multiple times, appends to the previously added seeds.
  //
  // Note: Beware of the weird corner case when calling `.WithSeeds({0})`. This
  // will result in an ambiguous call, since `{0}` can be interpreted as
  // `std::function`. Use `.WithSeeds(std::vector{0})` instead.
  std::enable_if_t<std::is_copy_constructible_v<CorpusType>, Derived&>
  WithSeeds(const std::vector<ValueType>& seeds) {
    seeds_.reserve(seeds_.size() + seeds.size());
    for (const ValueType& seed : seeds) {
      std::optional<CorpusType> corpus_value = derived().FromValue(seed);
      if (!corpus_value.has_value()) {
        ReportBadSeedAndExit(
            seed,
            absl::InvalidArgumentError(
                "Seed could not be converted to the internal corpus value"));
      }

      absl::Status valid = derived().ValidateCorpusValue(*corpus_value);
      if (!valid.ok()) ReportBadSeedAndExit(seed, valid);

      seeds_.push_back(*std::move(corpus_value));
    }
    return derived();
  }

  // The type of a function that generates a vector of seeds.
  using SeedProvider = std::function<std::vector<ValueType>()>;

  // Stores `seed_provider` to be called lazily the first time the seeds are
  // needed. The generated seeds are appended to any explicitly stored seeds.
  //
  // This overload can be used when the seeds need to be initialized
  // dynamically. For example, if the seeds depend on any global variables, this
  // is a way to resolve the static initialization order fiasco.
  std::enable_if_t<std::is_copy_constructible_v<CorpusType>, Derived&>
  WithSeeds(SeedProvider seed_provider) {
    seed_provider_ = std::move(seed_provider);
    return derived();
  }

 protected:
  // `Derived::Init()` can use this to sample seeds for this domain.
  std::optional<CorpusType> MaybeGetRandomSeed(absl::BitGenRef prng) {
    if (seed_provider_ != nullptr) {
      WithSeeds(std::invoke(seed_provider_));
      seed_provider_ = nullptr;
    }

    static constexpr double kProbabilityToReturnSeed = 0.5;
    if (seeds_.empty() || !absl::Bernoulli(prng, kProbabilityToReturnSeed)) {
      return std::nullopt;
    }
    return seeds_[fuzztest::internal::ChooseOffset(seeds_.size(), prng)];
  }

  // Default implementation of GetRandomCorpusValue() without guarantees on the
  // distribution of the returned values. If possible, the derived domain should
  // override this with a better implementation.
  CorpusType GetRandomCorpusValue(absl::BitGenRef prng) {
    auto corpus_val = derived().Init(prng);
    // Mutate a random number of times (including zero times). This eliminates
    // potential cyclicity issues (e.g., even number of mutations cancel out)
    // and allows returning an initial value with small but positive
    // probability.
    constexpr int kMaxMutations = 1000;
    for (int i = absl::Uniform(prng, 0, kMaxMutations); i > 0; --i) {
      derived().Mutate(corpus_val, prng, MutationMetadata{},
                       /*only_shrink=*/false);
    }
    return corpus_val;
  }

 private:
  Derived& derived() { return static_cast<Derived&>(*this); }
  const Derived& derived() const { return static_cast<const Derived&>(*this); }

  static void ReportBadSeedAndExit(const ValueType& seed,
                                   const absl::Status& status) {
    // This may run during fuzz test registration (i.e., global variable
    // initialization), so we can't use `GetStderr()`.
    absl::FPrintF(stderr, "[!] Invalid seed value (%s):\n\n{",
                  status.ToString());
    fuzztest::internal::AutodetectTypePrinter<ValueType>().PrintUserValue(
        seed, &std::cerr, PrintMode::kHumanReadable);
    absl::FPrintF(stderr, "}\n");
    std::exit(1);
  }

  std::vector<CorpusType> seeds_;
  SeedProvider seed_provider_;
};

}  //  namespace fuzztest::domain_implementor

#endif  // FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_DOMAIN_BASE_H_
