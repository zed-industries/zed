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

#ifndef FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_DOMAIN_H_
#define FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_DOMAIN_H_

#include <cstddef>
#include <cstdint>
#include <memory>
#include <optional>

#include "absl/functional/function_ref.h"
#include "absl/random/bit_gen_ref.h"
#include "absl/status/status.h"
#include "absl/strings/string_view.h"
#include "./fuzztest/internal/domains/domain_base.h"
#include "./fuzztest/internal/domains/domain_type_erasure.h"  // IWYU pragma: export
#include "./fuzztest/internal/printer.h"
#include "./fuzztest/internal/serialization.h"
#include "./fuzztest/internal/table_of_recent_compares.h"

namespace fuzztest {
namespace internal {

// Generic printer used by `Domain<T>` and `UntypedDomain` that delegates
// printing to the actual printers through `UntypedDomainConcept`.
struct GenericPrinter {
  const UntypedDomainConcept& domain;

  void PrintCorpusValue(const GenericDomainCorpusType& val,
                        domain_implementor::RawSink out,
                        domain_implementor::PrintMode mode) const {
    domain.UntypedPrintCorpusValue(val, out, mode);
  }

  void PrintFormattedAggregateValue(
      const GenericDomainCorpusType& val, domain_implementor::RawSink out,
      domain_implementor::PrintMode mode, absl::string_view prefix,
      absl::string_view suffix,
      absl::FunctionRef<void(domain_implementor::RawSink, size_t,
                             absl::string_view)>
          element_formatter) const {
    domain.UntypedPrintFormattedAggregateValue(val, out, mode, prefix, suffix,
                                               element_formatter);
  }
};

}  // namespace internal

// `Domain<T>` is the type-erased domain interface.
//
// It can be constructed from any object derived from `DomainBase` that
// implements the domain methods for the value type `T`.
template <typename T>
class Domain {
 public:
  // Domains deal with three different types:
  //
  // 1) The "user value type" is the user-facing type that serves as the basis
  //    for values represented by the domain. E.g., the user value type of:
  //
  //      Domain<std::string> d = InRegexp("a+");
  //
  //    is `std::string`, as the `InRegexp()` domain represents strings.
  //
  using value_type = T;

  // 2) The "corpus value type" is the internal type the domain works on. E.g.,
  //    for `InRegexp()` this is a data structure that represents paths through
  //    the Deterministic Finite Automaton (DFA) representing the given regular
  //    expression.
  //
  //    In the typed-erased interface `Domain<T>`, we use a generic type-erased
  //    type (similar to `std::any`) to store any corpus value.
  //
  using corpus_type = GenericDomainCorpusType;

  // 3) Finally, the `IRObject` is an intermediate representation for
  //    serialization. Corpus values are serialized by first transforming them
  //    into an `IRObject` (then to a string), and parsed the other way around.
  //    Note that while `value_type` and `corpus_type` can be many different
  //    types, there's only a single `IRObject` type.

  // TODO(b/303324603): Get rid of this:
  static constexpr bool has_custom_corpus_type = true;

  // Intentionally not marked as explicit to allow implicit conversion from the
  // inner domain implementations.
  template <typename Inner, typename CorpusType>
  Domain(const domain_implementor::DomainBase<Inner, T, CorpusType>& inner)
      : inner_(std::make_unique<internal::DomainModel<Inner>>(
            static_cast<const Inner&>(inner))) {}
  template <typename Inner, typename CorpusType>
  Domain(domain_implementor::DomainBase<Inner, T, CorpusType>&& inner)
      : inner_(std::make_unique<internal::DomainModel<Inner>>(
            static_cast<Inner&&>(inner))) {}

  Domain(const Domain& other) { *this = other; }
  Domain& operator=(const Domain& other) {
    inner_ = other.inner_->TypedClone();
    return *this;
  }
  // No default constructor or move operations to avoid a null state.

  // `GetRandomValue()` returns a random user value from the domain. This is
  // useful e.g., for generation-based black-box fuzzing, when coverage-guided
  // fuzzing is not possible, or for other use cases when manually sampling the
  // domain makes sense (e.g., getting random values for benchmarking). These
  // are the only uses cases when the users should use domains directly, and
  // this is the only method that the users should call.
  //
  // Important notes:
  //
  // -  In general, `GetRandomValue()` doesn't provide any guarantees on the
  // distribution of the returned values.
  //
  // -  For a fixed version of FuzzTest, `GetRandomValue()` will return the same
  // value when called with `prng`s that produce the same sequence of varieties.
  // However, the returned value may change between different FuzzTest versions.
  //
  // -  We strongly recommend against relying on fixed PRNG seeding in tests and
  // for reproducing fuzzing bugs. When used for black-box fuzzing, we recommend
  // saving the generated value as a reproducer, so that reproduction works even
  // between different versions of FuzzTest.
  //
  // -  For `prng`, we recommend using Abseil generators (e.g., `absl::BitGen`),
  // which actively prevent accidental usage of fixed PRNG seeding:
  // https://abseil.io/docs/cpp/guides/random#seed-stability.
  value_type GetRandomValue(absl::BitGenRef prng) {
    return inner_->TypedGetRandomValue(prng);
  }

  // The methods below are used by the FuzzTest framework and custom domain
  // implementations.

  // `Init()` generates a random value of `corpus_type`.
  //
  // Used to create initial values for fuzzing.The generated value can often be
  // a "special value" (e.g., 0, MAX_INT, NaN, infinity, empty vector, etc.).
  // For basic, fixed-size data types (e.g., `optional<int>`), `Init()` might
  // give any value. For variable-size data types (e.g., containers, linked
  // lists, trees, etc.), `Init()` typically returns a smaller-sized value.
  // Larger-sized values however can be created through calls to `Mutate()`.
  //
  // ENSURES: That `Init()` is non-deterministic, i.e., it doesn't always return
  // the same value. This is because `Mutate()` often relies on `Init()` giving
  // different values (e.g., when growing a `std::set<T>` and adding new `T`
  // values).
  corpus_type Init(absl::BitGenRef prng) { return inner_->UntypedInit(prng); }

  // Mutate() makes a relatively small modification on `val` of `corpus_type`.
  //
  // Used during coverage-guided fuzzing. When `only_shrink` is true,
  // the mutated value is always "simpler" (e.g., smaller). This is used for
  // input minimization ("shrinking").
  //
  // ENSURES: That the mutated value is not the same as the original.
  void Mutate(corpus_type& val, absl::BitGenRef prng,
              const domain_implementor::MutationMetadata& metadata,
              bool only_shrink) {
    return inner_->UntypedMutate(val, prng, metadata, only_shrink);
  }

  // Mutates `corpus_value` using `prng`, `only_shirnk` and the default mutation
  // metadata. This is a temporary wrapper that redirects the call to the real
  // interface with an explicit argument for metadata.
  void Mutate(corpus_type& corpus_value, absl::BitGenRef prng,
              bool only_shrink) {
    return Mutate(corpus_value, prng, {}, only_shrink);
  }

  // The methods below are responsible for transforming between the above
  // described three types that domains deal with. Here's a quick overview:
  //
  //        +------ GetValue() <----+     +---- ParseCorpus() <---+
  //        |                       |     |                       |
  //        v                       |     v                       |
  //
  //   value_type                 corpus_type                  IRObject
  //
  //        |                       ^     |                       ^
  //        |                       |     |                       |
  //        +----> FromValue() -----+     +-> SerializeCorpus() --+

  // Turns `corpus_value` into the user value.
  //
  // Used before passing the user value to the property function.
  value_type GetValue(const corpus_type& corpus_value) const {
    return inner_->TypedGetValue(corpus_value);
  }

  // Turns `user_value` back to a corpus value **without validation**.
  //
  // This is necessary to support `WithSeeds()` for a domain: `WithSeeds()`
  // takes user values. In order to mutate the provided seeds, they need to be
  // turned into corpus values first. Some domains might not support this
  // method.
  //
  // Note that validation must be done with `ValidateCorpusValue()` after
  // calling this function.
  std::optional<corpus_type> FromValue(const value_type& user_value) const {
    return inner_->TypedFromValue(user_value);
  }

  // Turns an `IRObject` value `obj` into corpus value **without validation**.
  //
  // Validation must be done with `ValidateCorpusValue()` after parsing.
  //
  // TODO(lszekeres): Return StatusOr<corpus_type>.
  std::optional<corpus_type> ParseCorpus(const internal::IRObject& obj) const {
    return inner_->UntypedParseCorpus(obj);
  }

  // Turns `corpus_value` to an `IRObject`.
  internal::IRObject SerializeCorpus(const corpus_type& corpus_value) const {
    return inner_->UntypedSerializeCorpus(corpus_value);
  }

  // Checks the validity of `corpus_value`, e.g., if it matches the domain's
  // constraints.
  //
  // After creating a corpus value, either via `ParseCorpus()` or via
  // `FromValue()`, this method should be used to determine if the corpus value
  // is valid.
  absl::Status ValidateCorpusValue(const corpus_type& corpus_value) const {
    return inner_->UntypedValidateCorpusValue(corpus_value);
  }

  // Returns the printer to be used to print values.
  auto GetPrinter() const { return internal::GenericPrinter{*inner_}; }

  // Try to update the dynamic memory dictionary.
  // If it propagates to a domain that's compatible with dynamic
  // dictionary, it will try to match and save dictionary entries from
  // dynamic data collected by SanCov.
  //
  // TODO(b/303324603): Using an extension mechanism, expose this method in
  // the interface only for user value types `T` for which it makes sense.
  void UpdateMemoryDictionary(
      const corpus_type& corpus_value,
      const internal::TablesOfRecentCompares* cmp_tables) {
    return inner_->UntypedUpdateMemoryDictionary(corpus_value, cmp_tables);
  }

  // Return the field counts of `corpus_value` if `corpus_value` is
  // a `ProtobufDomainImpl::corpus_type`. Otherwise propagate it
  // to inner domains and returns the sum of inner results. The corpus value is
  // taken as mutable reference to allow memoization.
  //
  // TODO(b/303324603): Using an extension mechanism, expose this method in
  // the interface only for user value types `T` for which it makes sense.
  uint64_t CountNumberOfFields(corpus_type& corpus_value) {
    return inner_->UntypedCountNumberOfFields(corpus_value);
  }

  // Mutate the selected protobuf field using `selected_field_index`.
  // Return value is the same as CountNumberOfFields.
  //
  // TODO(b/303324603): Using an extension mechanism, expose this method in
  // the interface only for user value types `T` for which it makes sense.
  uint64_t MutateSelectedField(
      corpus_type& corpus_value, absl::BitGenRef prng,
      const domain_implementor::MutationMetadata& metadata, bool only_shrink,
      uint64_t selected_field_index) {
    return inner_->UntypedMutateSelectedField(
        corpus_value, prng, metadata, only_shrink, selected_field_index);
  }

 private:
  friend class DomainBuilder;
  friend class UntypedDomain;

  // The wrapped inner domain.
  std::unique_ptr<internal::TypedDomainConcept<T>> inner_;
};

// `UntypedDomain` is the version of the domain interface where the user value
// type is also type-erased. For now, this only has the minimal interface needed
// by the FuzzTest runtime.
class UntypedDomain {
 public:
  using value_type = GenericDomainValueType;
  using corpus_type = GenericDomainCorpusType;
  static constexpr bool has_custom_corpus_type = true;

  // Intentionally not marked as explicit to allow implicit conversion from the
  // inner domain implementations.
  template <typename Inner, typename ValueType, typename CorpusType>
  UntypedDomain(
      const domain_implementor::DomainBase<Inner, ValueType, CorpusType>& inner)
      : inner_(std::make_unique<internal::DomainModel<Inner>>(
            static_cast<const Inner&>(inner))) {}
  template <typename Inner, typename ValueType, typename CorpusType>
  UntypedDomain(
      domain_implementor::DomainBase<Inner, ValueType, CorpusType>&& inner)
      : inner_(std::make_unique<internal::DomainModel<Inner>>(
            static_cast<Inner&&>(inner))) {}

  UntypedDomain(const UntypedDomain& other) { *this = other; }
  UntypedDomain& operator=(const UntypedDomain& other) {
    inner_ = other.inner_->UntypedClone();
    return *this;
  }
  // No default constructor or move operations to avoid a null state.

  // Allows implicit conversion from `Domain<ValueType>` to `UntypedDomain`.
  template <typename ValueType>
  UntypedDomain(const Domain<ValueType>& domain)
      : inner_(domain.inner_->UntypedClone()) {}

  corpus_type Init(absl::BitGenRef prng) { return inner_->UntypedInit(prng); }

  void Mutate(corpus_type& corpus_value, absl::BitGenRef prng,
              const domain_implementor::MutationMetadata& metadata,
              bool only_shrink) {
    return inner_->UntypedMutate(corpus_value, prng, metadata, only_shrink);
  }

  void Mutate(corpus_type& corpus_value, absl::BitGenRef prng,
              bool only_shrink) {
    return Mutate(corpus_value, prng, {}, only_shrink);
  }

  value_type GetValue(const corpus_type& corpus_value) const {
    return inner_->UntypedGetValue(corpus_value);
  }

  std::optional<corpus_type> ParseCorpus(const internal::IRObject& obj) const {
    return inner_->UntypedParseCorpus(obj);
  }

  internal::IRObject SerializeCorpus(const corpus_type& corpus_value) const {
    return inner_->UntypedSerializeCorpus(corpus_value);
  }

  absl::Status ValidateCorpusValue(const corpus_type& corpus_value) const {
    return inner_->UntypedValidateCorpusValue(corpus_value);
  }

  auto GetPrinter() const { return internal::GenericPrinter{*inner_}; }

  void UpdateMemoryDictionary(
      const corpus_type& corpus_value,
      domain_implementor::ConstCmpTablesPtr cmp_tables) {
    return inner_->UntypedUpdateMemoryDictionary(corpus_value, cmp_tables);
  }

 private:
  // The wrapped inner domain.
  std::unique_ptr<internal::UntypedDomainConcept> inner_;
};

}  // namespace fuzztest

#endif  // FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_DOMAIN_H_
