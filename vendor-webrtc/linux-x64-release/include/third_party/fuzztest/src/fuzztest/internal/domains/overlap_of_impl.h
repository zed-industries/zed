// Copyright 2024 Google LLC
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

#ifndef FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_OVERLAP_OF_IMPL_H_
#define FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_OVERLAP_OF_IMPL_H_

#include <cstddef>
#include <optional>
#include <tuple>
#include <type_traits>
#include <variant>

#include "absl/random/bit_gen_ref.h"
#include "absl/random/distributions.h"
#include "absl/status/status.h"
#include "absl/strings/str_format.h"
#include "./fuzztest/internal/domains/domain_base.h"
#include "./fuzztest/internal/domains/serialization_helpers.h"
#include "./fuzztest/internal/logging.h"
#include "./fuzztest/internal/meta.h"
#include "./fuzztest/internal/serialization.h"
#include "./fuzztest/internal/status.h"
#include "./fuzztest/internal/type_support.h"

namespace fuzztest::internal_no_adl {
auto Utf8String();
}  // namespace fuzztest::internal_no_adl

namespace fuzztest::internal {

template <typename... Inner>
class OverlapOfImpl
    : public domain_implementor::DomainBase<
          OverlapOfImpl<Inner...>,
          value_type_t<std::tuple_element_t<0, std::tuple<Inner...>>>,
          std::variant<corpus_type_t<Inner>...>> {
 public:
  using typename OverlapOfImpl::DomainBase::corpus_type;
  using typename OverlapOfImpl::DomainBase::value_type;

  static_assert(
      std::conjunction_v<std::is_same<value_type, value_type_t<Inner>>...>,
      "All overlapping domains must have the same value_type.");

  explicit OverlapOfImpl(Inner... domains) : domains_(std::move(domains)...) {}

  corpus_type Init(absl::BitGenRef prng) {
    if (auto seed = this->MaybeGetRandomSeed(prng)) return *seed;
    while (true) {
      auto result = Switch<kNumDomains>(
          absl::Uniform(prng, size_t{}, kNumDomains), [&](auto I) {
            return corpus_type(std::in_place_index<I>,
                               std::get<I>(domains_).Init(prng));
          });
      const bool valid = ValidateCorpusValueForOtherDomains(result).ok();
      MaybeReportOnValidationResult(valid);
      if (valid) return result;
    }
  }

  void Mutate(corpus_type& val, absl::BitGenRef prng,
              const domain_implementor::MutationMetadata& metadata,
              bool only_shrink) {
    const size_t index = val.index();
    std::optional<value_type> orig_value;
    std::optional<corpus_type> mutant_corpus;
    while (true) {
      const size_t mutation_index =
          only_shrink ? index : absl::Uniform(prng, size_t{}, kNumDomains);
      if (index != mutation_index) {
        if (!orig_value.has_value()) {
          orig_value = Switch<kNumDomains>(index, [&](auto I) {
            auto& domain = std::get<I>(domains_);
            return domain.GetValue(std::get<I>(val));
          });
        }
        Switch<kNumDomains>(mutation_index, [&](auto I) {
          auto& domain = std::get<I>(domains_);
          auto inner_corpus = domain.FromValue(*orig_value);
          FUZZTEST_INTERNAL_CHECK(inner_corpus.has_value(),
                                  "Mutate() called on a user value that is not "
                                  "valid in all overlapping domains");
          domain.Mutate(*inner_corpus, prng, metadata, only_shrink);
          mutant_corpus =
              corpus_type(std::in_place_index<I>, *std::move(inner_corpus));
        });
      } else {
        mutant_corpus = val;
        Switch<kNumDomains>(index, [&](auto I) {
          auto& domain = std::get<I>(domains_);
          domain.Mutate(std::get<I>(*mutant_corpus), prng, metadata,
                        only_shrink);
        });
      }
      FUZZTEST_INTERNAL_CHECK(mutant_corpus.has_value(),
                              "mutant corpus value is missing");
      const bool valid =
          ValidateCorpusValueForOtherDomains(*mutant_corpus).ok();
      MaybeReportOnValidationResult(valid);
      if (valid) {
        val = *std::move(mutant_corpus);
        return;
      }
    }
  }

  value_type GetValue(const corpus_type& v) const {
    return Switch<kNumDomains>(v.index(), [&](auto I) -> value_type {
      auto domain = std::get<I>(domains_);
      return domain.GetValue(std::get<I>(v));
    });
  }

  std::optional<corpus_type> FromValue(const value_type& v) const {
    std::optional<corpus_type> corpus;
    // Unless the serialization domain is set, use the first inner domain to get
    // the corpus value. We could use other domains but there is little
    // difference.
    Switch<kNumDomains>(serialization_domain_index_.value_or(0), [&](auto I) {
      auto inner_corpus = std::get<I>(domains_).FromValue(v);
      if (!inner_corpus.has_value()) return;
      corpus = corpus_type(std::in_place_index<I>, *std::move(inner_corpus));
    });
    if (!corpus.has_value() || !ValidateCorpusValue(*corpus).ok()) {
      return std::nullopt;
    }
    return corpus;
  }

  auto GetPrinter() const { return VariantPrinter<Inner...>{domains_}; }

  std::optional<corpus_type> ParseCorpus(const IRObject& obj) const {
    if (!serialization_domain_index_.has_value()) {
      return ParseWithDomainVariant(domains_, obj);
    }
    return Switch<kNumDomains>(
        *serialization_domain_index_,
        [&](auto I) -> std::optional<corpus_type> {
          auto inner_corpus = std::get<I>(domains_).ParseCorpus(obj);
          if (!inner_corpus.has_value()) return std::nullopt;
          return corpus_type(std::in_place_index<I>, *std::move(inner_corpus));
        });
  }

  IRObject SerializeCorpus(const corpus_type& v) const {
    if (!serialization_domain_index_.has_value()) {
      return SerializeWithDomainVariant(domains_, v);
    }
    if (*serialization_domain_index_ == v.index()) {
      return Switch<kNumDomains>(*serialization_domain_index_, [&](auto I) {
        return std::get<I>(domains_).SerializeCorpus(std::get<I>(v));
      });
    }
    const auto user_value = Switch<kNumDomains>(v.index(), [&](auto I) {
      auto& domain = std::get<I>(domains_);
      return domain.GetValue(std::get<I>(v));
    });
    return Switch<kNumDomains>(*serialization_domain_index_, [&](auto I) {
      auto& domain = std::get<I>(domains_);
      const auto inner_corpus = domain.FromValue(user_value);
      FUZZTEST_INTERNAL_CHECK(inner_corpus.has_value(),
                              "Mutate() called on a user value that is not "
                              "valid in all overlapping domains");
      return domain.SerializeCorpus(*inner_corpus);
    });
  }

  absl::Status ValidateCorpusValue(const corpus_type& corpus_value) const {
    auto status = Switch<kNumDomains>(corpus_value.index(), [&](auto I) {
      return std::get<I>(domains_).ValidateCorpusValue(
          std::get<I>(corpus_value));
    });
    if (status.ok()) status = ValidateCorpusValueForOtherDomains(corpus_value);
    return Prefix(status, "Invalid value for the overlapping domains");
  }

 private:
  friend class OverlapOfTestPeer;
  friend auto internal_no_adl::Utf8String();

  static constexpr size_t kNumDomains = sizeof...(Inner);
  static_assert(kNumDomains > 1,
                "It requires more than one domain to overlap.");

  std::tuple<Inner...> domains_;
  std::optional<size_t> serialization_domain_index_;
  size_t num_validation_attempts_ = 0;
  size_t num_validation_failures_ = 0;

  OverlapOfImpl& WithSerializationDomain(size_t index) {
    FUZZTEST_INTERNAL_CHECK_PRECONDITION(
        index < kNumDomains,
        absl::StrFormat("Serialization domain index must be less than %d",
                        kNumDomains));
    serialization_domain_index_ = index;
    return *this;
  }

  absl::Status ValidateCorpusValueForOtherDomains(
      const corpus_type& corpus_value) const {
    const auto value = Switch<kNumDomains>(corpus_value.index(), [&](auto I) {
      auto& domain = std::get<I>(domains_);
      return domain.GetValue(std::get<I>(corpus_value));
    });
    auto status = absl::OkStatus();
    const auto validate_one_domain = [&](auto I) {
      if (I == corpus_value.index()) return true;
      const auto& domain = std::get<I>(domains_);
      auto inner_corpus = domain.FromValue(value);
      if (!inner_corpus.has_value()) {
        status = absl::InvalidArgumentError(
            "failed to convert value into domain corpus");
        return false;
      }
      status = domain.ValidateCorpusValue(*inner_corpus);
      return status.ok();
    };
    ApplyIndex<kNumDomains>(
        [&](auto... I) { (validate_one_domain(I) && ...); });
    return status;
  }

  void MaybeReportOnValidationResult(bool succeeded) {
    ++num_validation_attempts_;
    if (!succeeded) {
      ++num_validation_failures_;
      FUZZTEST_INTERNAL_CHECK_PRECONDITION(
          num_validation_attempts_ <= 100 ||
              static_cast<double>(num_validation_failures_) <=
                  .9 * static_cast<double>(num_validation_attempts_),
          absl::StrFormat(R"(

[!] Ineffective use of overlapping domains detected!

Values were not valid on all of the overlapping domains on more than 90%% of the samples.
%d out of %d have failed.
)",
                          num_validation_failures_, num_validation_attempts_));
    }
  }
};

}  // namespace fuzztest::internal

#endif  // FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_OVERLAP_OF_IMPL_H_
