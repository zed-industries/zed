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
// IWYU pragma: friend fuzztest/internal/domains/domain_base.h

// This header file contains implementation details of the domain type erasure.
// The classes `UntypedDomainConcept`, `TypedDomainConcept`, and `DomainModel`
// should not be used directly. Instead, use the domain interfaces `Domain<T>`
// and `UntypedDomain`.

#ifndef FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_DOMAIN_TYPE_ERASURE_H_
#define FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_DOMAIN_TYPE_ERASURE_H_

#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <memory>
#include <optional>

#include "absl/functional/function_ref.h"
#include "absl/random/bit_gen_ref.h"
#include "absl/status/status.h"
#include "absl/strings/string_view.h"
#include "./fuzztest/internal/any.h"
#include "./fuzztest/internal/domains/mutation_metadata.h"
#include "./fuzztest/internal/logging.h"
#include "./fuzztest/internal/meta.h"
#include "./fuzztest/internal/printer.h"
#include "./fuzztest/internal/serialization.h"

namespace fuzztest {

using GenericDomainValueType = internal::MoveOnlyAny;
using GenericDomainCorpusType = internal::CopyableAny;

namespace internal {

// `UntypedDomainConcept` erases value_type/corpus_type type information for all
// inputs and outputs. All the `Untyped[Name]` functions implement the same API
// as the `[Name]` functions, but marshall the inputs and output through the
// generic types.
class UntypedDomainConcept {
 public:
  virtual ~UntypedDomainConcept() = default;

  virtual std::unique_ptr<UntypedDomainConcept> UntypedClone() const = 0;
  virtual GenericDomainCorpusType UntypedInit(absl::BitGenRef) = 0;
  virtual void UntypedMutate(
      GenericDomainCorpusType& val, absl::BitGenRef prng,
      const domain_implementor::MutationMetadata& metadata,
      bool only_shrink) = 0;
  virtual void UntypedUpdateMemoryDictionary(
      const GenericDomainCorpusType& val,
      domain_implementor::ConstCmpTablesPtr cmp_tables) = 0;
  virtual std::optional<GenericDomainCorpusType> UntypedParseCorpus(
      const IRObject& obj) const = 0;
  virtual absl::Status UntypedValidateCorpusValue(
      const GenericDomainCorpusType& corpus_value) const = 0;
  virtual IRObject UntypedSerializeCorpus(
      const GenericDomainCorpusType& v) const = 0;
  virtual uint64_t UntypedCountNumberOfFields(GenericDomainCorpusType&) = 0;
  virtual uint64_t UntypedMutateSelectedField(
      GenericDomainCorpusType&, absl::BitGenRef,
      const domain_implementor::MutationMetadata&, bool, uint64_t) = 0;
  virtual GenericDomainValueType UntypedGetValue(
      const GenericDomainCorpusType& v) const = 0;
  virtual void UntypedPrintCorpusValue(
      const GenericDomainCorpusType& val, domain_implementor::RawSink out,
      domain_implementor::PrintMode mode) const = 0;
  // Prints the value from an aggregate domain in a formatted way: prints
  // `prefix`, then calls `element_formatter` for each element with `out`,
  // the element's index, and the element's string representation, and
  // finally prints `suffix`. This is what the runtime uses to print out the
  // arguments when it finds a counterexample.
  virtual void UntypedPrintFormattedAggregateValue(
      const GenericDomainCorpusType& val, domain_implementor::RawSink out,
      domain_implementor::PrintMode mode, absl::string_view prefix,
      absl::string_view suffix,
      absl::FunctionRef<void(domain_implementor::RawSink, size_t,
                             absl::string_view)>
          element_formatter) const = 0;
};

// `TypedDomainConcept<ValueType>` extends `UntypedDomainConcept` with methods
// that handle `ValueType` inputs/outputs.
template <typename ValueType>
class TypedDomainConcept : public UntypedDomainConcept {
 public:
  virtual std::unique_ptr<TypedDomainConcept> TypedClone() const = 0;
  virtual ValueType TypedGetRandomValue(absl::BitGenRef prng) = 0;
  virtual ValueType TypedGetValue(const GenericDomainCorpusType& v) const = 0;
  virtual std::optional<GenericDomainCorpusType> TypedFromValue(
      const ValueType& v) const = 0;

  std::unique_ptr<UntypedDomainConcept> UntypedClone() const final {
    return TypedClone();
  }

  GenericDomainValueType UntypedGetValue(
      const GenericDomainCorpusType& v) const final {
    return GenericDomainValueType(std::in_place_type<ValueType>,
                                  TypedGetValue(v));
  }
};

// `DomainModel<D>` is a wrapper around a concrete domain `D`. It implements the
// concept classes `UntypedDomainConcept` and `TypedDomainConcept<ValueType>` by
// delegating the calls to `Untyped[Name]` and `Typed[Name]` functions to the
// corresponding `[Name]` functions on the wrapped domain.
template <typename D>
class DomainModel final : public TypedDomainConcept<value_type_t<D>> {
 public:
  using ValueType = value_type_t<D>;
  using CorpusType = corpus_type_t<D>;

  explicit DomainModel(const D& domain) : domain_(domain) {}
  explicit DomainModel(D&& domain) : domain_(std::forward<D>(domain)) {}

  std::unique_ptr<TypedDomainConcept<ValueType>> TypedClone() const final {
    return std::make_unique<DomainModel>(*this);
  }

  GenericDomainCorpusType UntypedInit(absl::BitGenRef prng) final {
    return GenericDomainCorpusType(std::in_place_type<CorpusType>,
                                   domain_.Init(prng));
  }

  void UntypedMutate(GenericDomainCorpusType& val, absl::BitGenRef prng,
                     const domain_implementor::MutationMetadata& metadata,
                     bool only_shrink) final {
    domain_.Mutate(val.GetAs<CorpusType>(), prng, metadata, only_shrink);
  }

  void UntypedUpdateMemoryDictionary(
      const GenericDomainCorpusType& val,
      domain_implementor::ConstCmpTablesPtr cmp_tables) final {
    domain_.UpdateMemoryDictionary(val.GetAs<CorpusType>(), cmp_tables);
  }

  ValueType TypedGetRandomValue(absl::BitGenRef prng) final {
    return domain_.GetRandomValue(prng);
  }

  ValueType TypedGetValue(const GenericDomainCorpusType& v) const final {
    return domain_.GetValue(v.GetAs<CorpusType>());
  }

  std::optional<GenericDomainCorpusType> TypedFromValue(
      const ValueType& v) const final {
    if (auto c = domain_.FromValue(v)) {
      return GenericDomainCorpusType(std::in_place_type<CorpusType>,
                                     *std::move(c));
    } else {
      return std::nullopt;
    }
  }

  std::optional<GenericDomainCorpusType> UntypedParseCorpus(
      const IRObject& obj) const final {
    if (auto res = domain_.ParseCorpus(obj)) {
      return GenericDomainCorpusType(std::in_place_type<CorpusType>,
                                     *std::move(res));
    } else {
      return std::nullopt;
    }
  }

  IRObject UntypedSerializeCorpus(
      const GenericDomainCorpusType& v) const final {
    return domain_.SerializeCorpus(v.template GetAs<CorpusType>());
  }

  absl::Status UntypedValidateCorpusValue(
      const GenericDomainCorpusType& corpus_value) const final {
    return domain_.ValidateCorpusValue(corpus_value.GetAs<CorpusType>());
  }

  uint64_t UntypedCountNumberOfFields(GenericDomainCorpusType& v) final {
    return domain_.CountNumberOfFields(v.GetAs<CorpusType>());
  }

  uint64_t UntypedMutateSelectedField(
      GenericDomainCorpusType& v, absl::BitGenRef prng,
      const domain_implementor::MutationMetadata& metadata, bool only_shrink,
      uint64_t selected_field_index) final {
    return domain_.MutateSelectedField(v.GetAs<CorpusType>(), prng, metadata,
                                       only_shrink, selected_field_index);
  }

  void UntypedPrintCorpusValue(const GenericDomainCorpusType& val,
                               domain_implementor::RawSink out,
                               domain_implementor::PrintMode mode) const final {
    PrintValue(domain_, val.GetAs<CorpusType>(), out, mode);
  }

  void UntypedPrintFormattedAggregateValue(
      const GenericDomainCorpusType& val, domain_implementor::RawSink out,
      domain_implementor::PrintMode mode, absl::string_view prefix,
      absl::string_view suffix,
      absl::FunctionRef<void(domain_implementor::RawSink, size_t,
                             absl::string_view)>
          element_formatter) const final {
    auto printer = domain_.GetPrinter();
    if constexpr (Requires<decltype(printer)>(
                      [&](auto t) -> decltype(t.PrintFormattedAggregateValue(
                                      val.GetAs<CorpusType>(), out, mode,
                                      prefix, suffix, element_formatter)) {})) {
      printer.PrintFormattedAggregateValue(val.GetAs<CorpusType>(), out, mode,
                                           prefix, suffix, element_formatter);
    } else {
      FUZZTEST_INTERNAL_CHECK(false,
                              "PrintFormattedAggregateValue() can only be "
                              "called on AggregatePrinter!");
    }
  }

 private:
  D domain_;
};

}  //  namespace internal
}  //  namespace fuzztest

#endif  // FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_DOMAIN_TYPE_ERASURE_H_
