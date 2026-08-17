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

#ifndef FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_UNIQUE_ELEMENTS_CONTAINER_OF_IMPL_H_
#define FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_UNIQUE_ELEMENTS_CONTAINER_OF_IMPL_H_

#include <cstddef>
#include <optional>

#include "absl/container/flat_hash_set.h"
#include "absl/random/bit_gen_ref.h"
#include "absl/status/status.h"
#include "./fuzztest/internal/domains/container_of_impl.h"
#include "./fuzztest/internal/domains/domain_base.h"
#include "./fuzztest/internal/meta.h"
#include "./fuzztest/internal/serialization.h"

namespace fuzztest::internal {

template <typename InnerDomain>
using UniqueDomain =
    AssociativeContainerOfImpl<absl::flat_hash_set<value_type_t<InnerDomain>>,
                               InnerDomain>;

// UniqueElementsContainerImpl supports producing containers of type `T`, with
// elements of type `E` from domain `InnerDomain inner`, with a guarantee that
// each element of the container has a unique value from `InnerDomain`. The
// guarantee is provided by using a `absl::flat_hash_set<E>` under the hood.
template <typename T, typename InnerDomain>
class UniqueElementsContainerImpl
    : public domain_implementor::DomainBase<
          UniqueElementsContainerImpl<T, InnerDomain>, T,
          corpus_type_t<UniqueDomain<InnerDomain>>> {
 public:
  using typename UniqueElementsContainerImpl::DomainBase::corpus_type;
  using typename UniqueElementsContainerImpl::DomainBase::value_type;

  UniqueElementsContainerImpl() = default;
  explicit UniqueElementsContainerImpl(InnerDomain inner)
      : inner_domain_(inner), unique_domain_(std::move(inner)) {}

  corpus_type Init(absl::BitGenRef prng) {
    if (auto seed = this->MaybeGetRandomSeed(prng)) return *seed;
    return unique_domain_.Init(prng);
  }

  void Mutate(corpus_type& val, absl::BitGenRef prng,
              const domain_implementor::MutationMetadata& metadata,
              bool only_shrink) {
    unique_domain_.Mutate(val, prng, metadata, only_shrink);
  }

  value_type GetValue(const corpus_type& v) const {
    // Converts directly via `inner_domain_` instead of via `unique_domain_` to
    // preserve the order of elements in sequence containers.
    value_type result;
    for (const auto& inner_corpus_val : v) {
      result.insert(result.end(), inner_domain_.GetValue(inner_corpus_val));
    }
    return result;
  }

  std::optional<corpus_type> FromValue(const value_type& v) const {
    // Converts directly via `inner_domain_` instead of via `unique_domain_` to
    // preserve the order of elements in sequence containers.
    corpus_type result;
    for (const auto& inner_user_val : v) {
      auto inner_corpus_val = inner_domain_.FromValue(inner_user_val);
      if (!inner_corpus_val) return std::nullopt;
      result.insert(result.end(), *std::move(inner_corpus_val));
    }
    return result;
  }

  auto GetPrinter() const { return unique_domain_.GetPrinter(); }

  std::optional<corpus_type> ParseCorpus(const IRObject& obj) const {
    return unique_domain_.ParseCorpus(obj);
  }

  IRObject SerializeCorpus(const corpus_type& v) const {
    return unique_domain_.SerializeCorpus(v);
  }

  absl::Status ValidateCorpusValue(const corpus_type& corpus_value) const {
    absl::Status status = unique_domain_.ValidateCorpusValue(corpus_value);
    if (!status.ok()) return status;
    auto unique_values = unique_domain_.GetValue(corpus_value);
    if (unique_values.size() != corpus_value.size()) {
      return absl::InvalidArgumentError(
          "The container doesn't have unique elements");
    }
    return absl::OkStatus();
  }

  auto& WithSize(size_t s) { return WithMinSize(s).WithMaxSize(s); }
  auto& WithMinSize(size_t s) {
    unique_domain_.WithMinSize(s);
    return *this;
  }
  auto& WithMaxSize(size_t s) {
    unique_domain_.WithMaxSize(s);
    return *this;
  }

 private:
  InnerDomain inner_domain_;
  UniqueDomain<InnerDomain> unique_domain_;
};

}  // namespace fuzztest::internal

#endif  // FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_UNIQUE_ELEMENTS_CONTAINER_OF_IMPL_H_
