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

#ifndef FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_FILTER_IMPL_H_
#define FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_FILTER_IMPL_H_

#include <cstdint>
#include <functional>
#include <optional>

#include "absl/random/bit_gen_ref.h"
#include "absl/status/status.h"
#include "absl/strings/str_format.h"
#include "./fuzztest/internal/domains/domain.h"
#include "./fuzztest/internal/domains/domain_base.h"
#include "./fuzztest/internal/logging.h"
#include "./fuzztest/internal/serialization.h"

namespace fuzztest::internal {

template <typename T>
class FilterImpl
    : public domain_implementor::DomainBase<FilterImpl<T>, T,
                                            GenericDomainCorpusType> {
 public:
  using typename FilterImpl::DomainBase::corpus_type;
  using typename FilterImpl::DomainBase::value_type;

  FilterImpl() = default;
  explicit FilterImpl(std::function<bool(const T&)> predicate, Domain<T> inner)
      : predicate_(std::move(predicate)), inner_(std::move(inner)) {}

  corpus_type Init(absl::BitGenRef prng) {
    if (auto seed = this->MaybeGetRandomSeed(prng)) return *seed;
    while (true) {
      auto v = inner_.Init(prng);
      if (RunFilter(v)) return v;
    }
  }

  void Mutate(corpus_type& val, absl::BitGenRef prng,
              const domain_implementor::MutationMetadata& metadata,
              bool only_shrink) {
    corpus_type original_val = val;
    while (true) {
      inner_.Mutate(val, prng, metadata, only_shrink);
      if (RunFilter(val)) return;
      val = original_val;
    }
  }

  value_type GetValue(const corpus_type& v) const { return inner_.GetValue(v); }

  std::optional<corpus_type> FromValue(const value_type& v) const {
    if (!predicate_(v)) return std::nullopt;
    return inner_.FromValue(v);
  }

  auto GetPrinter() const { return inner_.GetPrinter(); }

  std::optional<corpus_type> ParseCorpus(const IRObject& obj) const {
    return inner_.ParseCorpus(obj);
  }

  IRObject SerializeCorpus(const corpus_type& v) const {
    return inner_.SerializeCorpus(v);
  }

  absl::Status ValidateCorpusValue(const corpus_type& corpus_value) const {
    if (const auto inner_validate_status =
            inner_.ValidateCorpusValue(corpus_value);
        !inner_validate_status.ok()) {
      return Prefix(inner_validate_status,
                    "Invalid corpus value for the inner domain in Filter()");
    }
    if (predicate_(GetValue(corpus_value))) return absl::OkStatus();
    return absl::InvalidArgumentError(
        "Value does not match Filter() predicate.");
  }

 private:
  bool RunFilter(const corpus_type& v) {
    ++num_values_;
    bool res = predicate_(GetValue(v));
    if (!res) {
      ++num_skips_;
      FUZZTEST_INTERNAL_CHECK_PRECONDITION(
          num_skips_ <= 100 || static_cast<double>(num_skips_) <=
                                   .9 * static_cast<double>(num_values_),
          absl::StrFormat(R"(

[!] Ineffective use of Filter() detected!

Filter predicate failed on more than 90%% of the samples.
%d out of %d have failed.

Please use Filter() only to skip unlikely values. To filter out a significant
chunk of the input domain, consider defining a custom domain by construction.
See more details in the User Guide.
)",
                          num_skips_, num_values_));
    }
    return res;
  }

  std::function<bool(const T&)> predicate_;
  Domain<T> inner_;
  uint64_t num_values_ = 0;
  uint64_t num_skips_ = 0;
};

}  // namespace fuzztest::internal

#endif  // FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_FILTER_IMPL_H_
