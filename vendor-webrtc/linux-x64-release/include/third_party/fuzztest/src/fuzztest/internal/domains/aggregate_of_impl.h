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

#ifndef FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_AGGREGATE_OF_IMPL_H_
#define FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_AGGREGATE_OF_IMPL_H_

#include <array>
#include <cstddef>
#include <optional>
#include <tuple>
#include <type_traits>
#include <utility>

#include "absl/random/bit_gen_ref.h"
#include "absl/random/distributions.h"
#include "./fuzztest/internal/domains/domain_base.h"
#include "./fuzztest/internal/domains/serialization_helpers.h"
#include "./fuzztest/internal/meta.h"
#include "./fuzztest/internal/serialization.h"
#include "./fuzztest/internal/status.h"
#include "./fuzztest/internal/type_support.h"

namespace fuzztest::internal {

enum class RequireCustomCorpusType { kNo, kYes };

// For user defined types (structs) we require a custom corpus_type
// (std::tuple), because the serializer does not support structs, only tuples.
template <typename T, RequireCustomCorpusType require_custom, typename... Inner>
using AggregateOfImplCorpusType =
    std::conditional_t<require_custom == RequireCustomCorpusType::kYes ||
                           (Inner::has_custom_corpus_type || ...),
                       std::tuple<corpus_type_t<Inner>...>, T>;

template <typename T, RequireCustomCorpusType require_custom, typename... Inner>
class AggregateOfImpl
    : public domain_implementor::DomainBase<
          AggregateOfImpl<T, require_custom, Inner...>, T,
          AggregateOfImplCorpusType<T, require_custom, Inner...>> {
 public:
  using AggregateOfImpl::DomainBase::has_custom_corpus_type;
  using typename AggregateOfImpl::DomainBase::corpus_type;
  using typename AggregateOfImpl::DomainBase::value_type;

  AggregateOfImpl() = default;
  explicit AggregateOfImpl(std::in_place_t, Inner... inner)
      : inner_(std::move(inner)...) {}

  corpus_type Init(absl::BitGenRef prng) {
    if (auto seed = this->MaybeGetRandomSeed(prng)) return *seed;
    return std::apply(
        [&](auto&... inner) { return corpus_type{inner.Init(prng)...}; },
        inner_);
  }

  void Mutate(corpus_type& val, absl::BitGenRef prng,
              const domain_implementor::MutationMetadata& metadata,
              bool only_shrink) {
    std::integral_constant<int, sizeof...(Inner)> size;
    auto bound = internal::BindAggregate(val, size);
    // Filter the tuple to only the mutable fields.
    // The const ones can't be mutated.
    // Eg in `std::pair<const int, int>` for maps.
    static constexpr auto to_mutate =
        GetMutableSubtuple<decltype(internal::BindAggregate(std::declval<T&>(),
                                                            size))>();
    static constexpr size_t actual_size =
        std::tuple_size_v<decltype(to_mutate)>;
    if constexpr (actual_size > 0) {
      int offset = absl::Uniform<int>(prng, 0, actual_size);
      Switch<actual_size>(offset, [&](auto I) {
        std::get<to_mutate[I]>(inner_).Mutate(std::get<to_mutate[I]>(bound),
                                              prng, metadata, only_shrink);
      });
    }
  }

  void UpdateMemoryDictionary(
      const corpus_type& val,
      domain_implementor::ConstCmpTablesPtr cmp_tables) {
    // Copy codes from Mutate that does the mutable domain filtering things.
    std::integral_constant<int, sizeof...(Inner)> size;
    auto bound = internal::BindAggregate(val, size);
    static constexpr auto to_mutate =
        GetMutableSubtuple<decltype(internal::BindAggregate(std::declval<T&>(),
                                                            size))>();
    static constexpr size_t actual_size =
        std::tuple_size_v<decltype(to_mutate)>;
    // Apply UpdateMemoryDictionary to every mutable domain.
    if constexpr (actual_size > 0) {
      ApplyIndex<actual_size>([&](auto... I) {
        (std::get<to_mutate[I]>(inner_).UpdateMemoryDictionary(
             std::get<to_mutate[I]>(bound), cmp_tables),
         ...);
      });
    }
  }

  auto GetPrinter() const {
    return AggregatePrinter<AggregateOfImpl, Inner...>{
        *this, inner_, GetTypeNameIfUserDefined<T>()};
  }

  value_type GetValue(const corpus_type& value) const {
    if constexpr (has_custom_corpus_type) {
      if constexpr (DetectBindableFieldCount<value_type>() ==
                    DetectBraceInitCount<value_type>()) {
        return ApplyIndex<sizeof...(Inner)>([&](auto... I) {
          return T{std::get<I>(inner_).GetValue(std::get<I>(value))...};
        });
      } else {
        // Right now the only other possibility is that the bindable field count
        // is one less than the brace init field count. In that case, that extra
        // field is used to initialize an empty base class. We'll need to update
        // this if that ever changes.
        return ApplyIndex<sizeof...(Inner)>([&](auto... I) {
          return T{{}, std::get<I>(inner_).GetValue(std::get<I>(value))...};
        });
      }
    } else {
      return value;
    }
  }

  std::optional<corpus_type> FromValue(const value_type& value) const {
    if constexpr (has_custom_corpus_type) {
      return ApplyIndex<sizeof...(Inner)>([&](auto... I) {
        auto bound = internal::BindAggregate(
            value, std::integral_constant<int, sizeof...(Inner)>{});
        return [](auto... optional_values) -> std::optional<corpus_type> {
          if ((optional_values.has_value() && ...)) {
            return std::tuple(*std::move(optional_values)...);
          } else {
            return std::nullopt;
          }
        }(std::get<I>(inner_).FromValue(std::get<I>(bound))...);
      });
    } else {
      return value;
    }
  }

  // Use the generic serializer when no custom corpus type is used, since it is
  // more efficient. Eg a string value can be serialized as a string instead of
  // as a sequence of char values.
  std::optional<corpus_type> ParseCorpus(const IRObject& obj) const {
    if constexpr (has_custom_corpus_type) {
      return ParseWithDomainTuple(inner_, obj);
    } else {
      return obj.ToCorpus<corpus_type>();
    }
  }

  IRObject SerializeCorpus(const corpus_type& v) const {
    if constexpr (has_custom_corpus_type) {
      return SerializeWithDomainTuple(inner_, v);
    } else {
      return IRObject::FromCorpus(v);
    }
  }

  absl::Status ValidateCorpusValue(const corpus_type& corpus_value) const {
    absl::Status result = absl::OkStatus();
    ApplyIndex<sizeof...(Inner)>([&](auto... I) {
      (
          [&] {
            if (!result.ok()) return;
            const absl::Status s = std::get<I>(inner_).ValidateCorpusValue(
                std::get<I>(corpus_value));
            result = Prefix(s, "Invalid value in aggregate");
          }(),
          ...);
    });
    return result;
  }

 private:
  template <typename Tuple>
  static constexpr auto GetMutableSubtuple() {
    return ApplyIndex<std::tuple_size_v<Tuple>>([](auto... I) {
      constexpr auto is_const = [](auto I2) {
        return std::is_const_v<
            std::remove_reference_t<std::tuple_element_t<I2, Tuple>>>;
      };
      std::array<int, (!is_const(I) + ... + 0)> res{};
      int pos = 0;
      ((is_const(I) ? I : res[pos++] = I), ...);
      return res;
    });
  }

  std::tuple<Inner...> inner_;
};

}  // namespace fuzztest::internal

#endif  // FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_AGGREGATE_OF_IMPL_H_
