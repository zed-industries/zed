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

#ifndef FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_ONE_OF_IMPL_H_
#define FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_ONE_OF_IMPL_H_

#include <cstddef>
#include <optional>
#include <tuple>
#include <type_traits>
#include <variant>

#include "absl/random/bit_gen_ref.h"
#include "absl/random/distributions.h"
#include "./fuzztest/internal/domains/domain_base.h"
#include "./fuzztest/internal/domains/serialization_helpers.h"
#include "./fuzztest/internal/meta.h"
#include "./fuzztest/internal/serialization.h"
#include "./fuzztest/internal/status.h"
#include "./fuzztest/internal/type_support.h"

namespace fuzztest::internal {

template <typename... Inner>
class OneOfImpl
    : public domain_implementor::DomainBase<
          OneOfImpl<Inner...>,
          value_type_t<std::tuple_element_t<0, std::tuple<Inner...>>>,
          std::variant<corpus_type_t<Inner>...>> {
 public:
  using typename OneOfImpl::DomainBase::corpus_type;
  using typename OneOfImpl::DomainBase::value_type;

  // All value_types of inner domains must be the same. (Though note that they
  // can have different corpus_types!)
  static_assert(
      std::conjunction_v<std::is_same<value_type, value_type_t<Inner>>...>,
      "All domains in a OneOf must have the same value_type.");

  explicit OneOfImpl(Inner... domains) : domains_(std::move(domains)...) {}

  corpus_type Init(absl::BitGenRef prng) {
    if (auto seed = this->MaybeGetRandomSeed(prng)) return *seed;
    // TODO(b/191368509): Consider the cardinality of the subdomains to weight
    // them.
    return Switch<kNumDomains>(
        absl::Uniform(prng, size_t{}, kNumDomains), [&](auto I) {
          return corpus_type(std::in_place_index<I>,
                             std::get<I>(domains_).Init(prng));
        });
  }

  void Mutate(corpus_type& val, absl::BitGenRef prng,
              const domain_implementor::MutationMetadata& metadata,
              bool only_shrink) {
    // Switch to another domain 1% of the time when not reducing.
    if (kNumDomains > 1 && !only_shrink && absl::Bernoulli(prng, 0.01)) {
      // Choose a different index.
      size_t offset = absl::Uniform<size_t>(prng, 1, kNumDomains);
      size_t index = static_cast<size_t>(val.index());
      index += offset;
      if (index >= kNumDomains) index -= kNumDomains;
      Switch<kNumDomains>(index, [&](auto I) {
        auto& domain = std::get<I>(domains_);
        val.template emplace<I>(domain.Init(prng));
      });
    } else {
      Switch<kNumDomains>(val.index(), [&](auto I) {
        auto& domain = std::get<I>(domains_);
        domain.Mutate(std::get<I>(val), prng, metadata, only_shrink);
      });
    }
  }

  value_type GetValue(const corpus_type& v) const {
    return Switch<kNumDomains>(v.index(), [&](auto I) -> value_type {
      auto domain = std::get<I>(domains_);
      return domain.GetValue(std::get<I>(v));
    });
  }

  std::optional<corpus_type> FromValue(const value_type& v) const {
    std::optional<corpus_type> res;
    const auto try_one_corpus = [&](auto I) {
      auto corpus_value = std::get<I>(domains_).FromValue(v);
      if (!corpus_value.has_value()) return false;

      const absl::Status valid =
          std::get<I>(domains_).ValidateCorpusValue(*corpus_value);
      if (!valid.ok()) return false;

      res.emplace(std::in_place_index<I>, *std::move(corpus_value));
      return true;
    };

    ApplyIndex<kNumDomains>([&](auto... I) {
      // Try them in order, break on first success.
      (try_one_corpus(I) || ...);
    });

    return res;
  }

  auto GetPrinter() const { return VariantPrinter<Inner...>{domains_}; }

  std::optional<corpus_type> ParseCorpus(const IRObject& obj) const {
    return ParseWithDomainVariant(domains_, obj);
  }

  IRObject SerializeCorpus(const corpus_type& v) const {
    return SerializeWithDomainVariant(domains_, v);
  }

  absl::Status ValidateCorpusValue(const corpus_type& corpus_value) const {
    return Switch<kNumDomains>(corpus_value.index(), [&](auto I) {
      const absl::Status s =
          std::get<I>(domains_).ValidateCorpusValue(std::get<I>(corpus_value));
      return Prefix(s, "Invalid value for OneOf() domain");
    });
  }

 private:
  static constexpr size_t kNumDomains = sizeof...(Inner);
  static_assert(kNumDomains > 0, "OneOf requires a non-empty list.");

  std::tuple<Inner...> domains_;
};

}  // namespace fuzztest::internal

#endif  // FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_ONE_OF_IMPL_H_
