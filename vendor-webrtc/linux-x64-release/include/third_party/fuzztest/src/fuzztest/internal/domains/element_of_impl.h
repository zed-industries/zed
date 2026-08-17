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

#ifndef FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_ELEMENT_OF_IMPL_H_
#define FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_ELEMENT_OF_IMPL_H_

#include <cstddef>
#include <optional>
#include <string>
#include <string_view>
#include <vector>

#include "absl/random/bit_gen_ref.h"
#include "absl/random/distributions.h"
#include "absl/time/time.h"
#include "./fuzztest/internal/domains/domain_base.h"
#include "./fuzztest/internal/logging.h"
#include "./fuzztest/internal/serialization.h"
#include "./fuzztest/internal/type_support.h"

namespace fuzztest::internal {

enum class ElementOfImplCorpusType : size_t;

template <typename T>
class ElementOfImpl
    : public domain_implementor::DomainBase<ElementOfImpl<T>, T,
                                            ElementOfImplCorpusType> {
 public:
  using typename ElementOfImpl::DomainBase::corpus_type;
  using typename ElementOfImpl::DomainBase::value_type;

  explicit ElementOfImpl(std::vector<T> values) : values_(values) {
    FUZZTEST_INTERNAL_CHECK_PRECONDITION(
        !values.empty(), "ElementOf requires a non empty list.");
  }

  corpus_type Init(absl::BitGenRef prng) {
    if (auto seed = this->MaybeGetRandomSeed(prng)) return *seed;
    return corpus_type{absl::Uniform<size_t>(prng, 0, values_.size())};
  }

  void Mutate(corpus_type& val, absl::BitGenRef prng,
              const domain_implementor::MutationMetadata&, bool only_shrink) {
    if (values_.size() <= 1) return;
    if (only_shrink) {
      size_t index = static_cast<size_t>(val);
      if (index == 0) return;
      index = absl::Uniform<size_t>(prng, 0, index);
      val = static_cast<corpus_type>(index);
      return;
    }
    // Choose a different index.
    size_t offset = absl::Uniform<size_t>(prng, 1, values_.size());
    size_t index = static_cast<size_t>(val);
    index += offset;
    if (index >= values_.size()) index -= values_.size();
    val = static_cast<corpus_type>(index);
  }

  value_type GetValue(corpus_type value) const {
    return values_[static_cast<size_t>(value)];
  }

  std::optional<corpus_type> FromValue(const value_type& v) const {
    // For simple scalar types we try to find them in the list.
    // Otherwise, we fail unconditionally because we might not be able to
    // effectively compare the values.
    // Checking for `operator==` is not enough. You will have false positives
    // where `operator==` exists but it either doens't compile or it gives the
    // wrong answer.
    // TODO(b/298068402): Improve this.
    if constexpr (std::is_enum_v<value_type> ||
                  std::is_arithmetic_v<value_type> ||
                  std::is_same_v<std::string, value_type> ||
                  std::is_same_v<std::string_view, value_type> ||
                  std::is_same_v<absl::Duration, value_type> ||
                  std::is_same_v<absl::Time, value_type>) {
      auto it = std::find(values_.begin(), values_.end(), v);
      return it == values_.end() ? std::nullopt
                                 : std::optional(static_cast<corpus_type>(
                                       it - values_.begin()));
    }
    return std::nullopt;
  }

  auto GetPrinter() const { return AutodetectTypePrinter<T>(); }

  std::optional<corpus_type> ParseCorpus(const IRObject& obj) const {
    return obj.ToCorpus<corpus_type>();
  }

  IRObject SerializeCorpus(const corpus_type& v) const {
    return IRObject::FromCorpus(v);
  }

  absl::Status ValidateCorpusValue(const corpus_type& corpus_value) const {
    if (static_cast<size_t>(corpus_value) < values_.size()) {
      return absl::OkStatus();
    }
    return absl::InvalidArgumentError("Invalid ElementOf() value");
  }

 private:
  std::vector<T> values_;
};

}  // namespace fuzztest::internal

#endif  // FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_ELEMENT_OF_IMPL_H_
