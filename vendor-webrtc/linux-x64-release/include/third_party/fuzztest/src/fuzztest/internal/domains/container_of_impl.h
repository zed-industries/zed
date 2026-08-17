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

#ifndef FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_CONTAINER_OF_IMPL_H_
#define FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_CONTAINER_OF_IMPL_H_

#include <algorithm>
#include <cstddef>
#include <cstdint>
#include <functional>
#include <iterator>
#include <list>
#include <optional>
#include <string>
#include <type_traits>
#include <vector>

#include "absl/random/bit_gen_ref.h"
#include "absl/random/distributions.h"
#include "absl/status/status.h"
#include "absl/strings/str_format.h"
#include "absl/types/span.h"
#include "./fuzztest/internal/domains/container_mutation_helpers.h"
#include "./fuzztest/internal/domains/domain_base.h"
#include "./fuzztest/internal/logging.h"
#include "./fuzztest/internal/meta.h"
#include "./fuzztest/internal/serialization.h"
#include "./fuzztest/internal/status.h"
#include "./fuzztest/internal/type_support.h"

namespace fuzztest::internal {

// Used for ChoosePosition();
enum class IncludeEnd { kYes, kNo };

// Default maximum size for FuzzTest containers/lists.
inline constexpr size_t kDefaultContainerMaxSize = 5000;

// For cases where the type is a container, choose one of the elements in the
// container.
template <typename Container>
auto ChoosePosition(Container& val, IncludeEnd include_end,
                    absl::BitGenRef prng) {
  size_t i = absl::Uniform<size_t>(
      prng, 0, include_end == IncludeEnd::kYes ? val.size() + 1 : val.size());
  return std::next(val.begin(), i);
}

template <typename ContainerDomain,
          typename ValueType = ExtractTemplateParameter<0, ContainerDomain>,
          typename InnerDomain = ExtractTemplateParameter<1, ContainerDomain>>
using ContainerOfImplBaseCorpusType = std::conditional_t<
    // We use std::list as corpus type if:
    // 1) Container is vector<bool>, in order to be able to hold a reference to
    // a single bit. Otherwise that would only be possible through the
    // `reference` proxy class.
    // 2) Container is associative, in order to allow modifying the keys.
    // Otherwise the inner domain would be immutable (e.g., std::pair<const int,
    // int> for maps).
    // 3) Inner domain has custom corpus type.
    is_bitvector_v<ValueType> || is_associative_container_v<ValueType> ||
        InnerDomain::has_custom_corpus_type,
    std::list<corpus_type_t<InnerDomain>>, ValueType>;

// Common base for container domains. Provides common APIs.
template <typename Derived, typename T = ExtractTemplateParameter<0, Derived>,
          typename InnerDomainT = ExtractTemplateParameter<1, Derived>>
class ContainerOfImplBase
    : public domain_implementor::DomainBase<
          Derived, T, ContainerOfImplBaseCorpusType<Derived, T, InnerDomainT>> {
 public:
  using ContainerOfImplBase::DomainBase::has_custom_corpus_type;
  using typename ContainerOfImplBase::DomainBase::corpus_type;
  using typename ContainerOfImplBase::DomainBase::value_type;

  // Some container mutation only applies to vector or string types which do
  // not have a custom corpus type.
  static constexpr bool is_vector_or_string =
      !has_custom_corpus_type &&
      (is_vector_v<value_type> || std::is_same_v<value_type, std::string>);

  // The current implementation of container dictionary only supports
  // vector or string container value_type, whose InnerDomain is
  // an `ArbitraryImpl<T2>` where T2 is an integral type.
  static constexpr bool container_has_memory_dict =
      is_memory_dictionary_compatible<InnerDomainT>::value &&
      is_vector_or_string;

  // If `!container_has_memory_dict`, dict_type is a bool and dict
  // is not used. This conditional_t may be neccessary because some
  // value_type may not have copy constructors(for example, proto).
  // Making it a safe type(bool) to not break some targets.
  using dict_type = std::conditional_t<container_has_memory_dict,
                                       ContainerDictionary<value_type>, bool>;
  using dict_entry_type = std::conditional_t<container_has_memory_dict,
                                             DictionaryEntry<value_type>, bool>;

  ContainerOfImplBase() = default;
  explicit ContainerOfImplBase(InnerDomainT inner) : inner_(std::move(inner)) {}

  void Mutate(corpus_type& val, absl::BitGenRef prng,
              const domain_implementor::MutationMetadata& metadata,
              bool only_shrink) {
    permanent_dict_candidate_ = std::nullopt;
    FUZZTEST_INTERNAL_CHECK(
        min_size() <= val.size() && val.size() <= max_size(), "Size ",
        val.size(), " is not between ", min_size(), " and ", max_size());

    const bool can_shrink = val.size() > min_size();
    const bool can_grow = !only_shrink && val.size() < max_size();
    const bool can_change = val.size() != 0;
    const bool can_use_memory_dict = !only_shrink &&
                                     container_has_memory_dict && can_change &&
                                     metadata.cmp_tables != nullptr;
    const int action_count =
        can_shrink + can_grow + can_change + can_use_memory_dict;
    if (action_count == 0) return;
    int action = absl::Uniform(prng, 0, action_count);

    if (can_shrink) {
      if (action-- == 0) {
        if constexpr (!has_custom_corpus_type) {
          EraseRandomChunk(val, prng, min_size());
          return;
        }
        val.erase(ChoosePosition(val, IncludeEnd::kNo, prng));
        return;
      }
    }
    if (can_grow) {
      if (action-- == 0) {
        if constexpr (!has_custom_corpus_type) {
          auto element_val = inner_.Init(prng);
          InsertRandomChunk(val, prng, max_size(), element_val);
          return;
        }
        Self().GrowOne(val, prng);
        return;
      }
    }
    if (can_change) {
      if (action-- == 0) {
        // If possible, mutate a consecutive chunk.
        if constexpr (!has_custom_corpus_type) {
          const size_t changes =
              val.size() == 1 ? 1 : 1 + absl::Zipf(prng, val.size() - 1);
          const size_t change_offset =
              ChooseOffset(val.size() - changes + 1, prng);
          auto it_start = std::next(val.begin(), change_offset);
          auto it_end = std::next(it_start, changes);
          for (; it_start != it_end; it_start = std::next(it_start)) {
            Self().MutateElement(val, prng, metadata, only_shrink, it_start);
          }
          return;
        }
        Self().MutateElement(val, prng, metadata, only_shrink,
                             ChoosePosition(val, IncludeEnd::kNo, prng));
        return;
      }
    }
    if constexpr (container_has_memory_dict) {
      if (can_use_memory_dict) {
        if (action-- == 0) {
          bool mutated = MemoryDictionaryMutation(
              val, prng, metadata.cmp_tables, temporary_dict_, GetManualDict(),
              permanent_dict_, permanent_dict_candidate_, max_size());
          // If dict failed, fall back to changing an element.
          if (!mutated) {
            Self().MutateElement(val, prng, metadata, only_shrink,
                                 ChoosePosition(val, IncludeEnd::kNo, prng));
          }
          return;
        }
      }
    }
  }

  void UpdateMemoryDictionary(
      const corpus_type& val,
      domain_implementor::ConstCmpTablesPtr cmp_tables) {
    // TODO(JunyangShao): Implement dictionary propagation to container
    // elements. For now the propagation stops in container domains.
    // Because all elements share an `inner_` and will share
    // a dictionary if we propagate it, which makes the dictionary
    // not efficient.
    if constexpr (container_has_memory_dict) {
      if (cmp_tables != nullptr) {
        temporary_dict_.MatchEntriesFromTableOfRecentCompares(val, *cmp_tables);
        if (permanent_dict_candidate_.has_value() &&
            permanent_dict_.Size() < kPermanentDictMaxSize) {
          permanent_dict_.AddEntry(std::move(*permanent_dict_candidate_));
          permanent_dict_candidate_ = std::nullopt;
        }
      }
    }
  }

  // These are specific for containers only. They are not part of the common
  // Domain API.
  Derived& WithSize(size_t s) {
    max_size_ = min_size_ = s;
    return Self();
  }
  Derived& WithMinSize(size_t s) {
    FUZZTEST_INTERNAL_CHECK_PRECONDITION(
        !max_size_.has_value() || s <= *max_size_, "Minimal size ", s,
        " cannot be larger than maximal size ", *max_size_);
    min_size_ = s;
    return Self();
  }
  Derived& WithMaxSize(size_t s) {
    FUZZTEST_INTERNAL_CHECK_PRECONDITION(
        min_size_ <= s, "Maximal size ", s,
        " cannot be smaller than minimal size ", min_size_);
    max_size_ = s;
    return Self();
  }
  Derived& WithDictionary(absl::Span<const value_type> manual_dict) {
    static_assert(container_has_memory_dict,
                  "Manual Dictionary now only supports std::vector or "
                  "std::string or std::string_view.\n");
    for (const value_type& entry : manual_dict) {
      FUZZTEST_INTERNAL_CHECK(
          entry.size() <= max_size(),
          "At least one dictionary entry is larger than max container size.");
      manual_dict_.AddEntry({std::nullopt, entry});
    }
    return Self();
  }
  Derived& WithDictionary(
      std::function<std::vector<value_type>()> manual_dict_provider) {
    manual_dict_provider_ = std::move(manual_dict_provider);
    return Self();
  }

  auto GetPrinter() const {
    if constexpr (std::is_same_v<value_type, std::string> ||
                  std::is_same_v<value_type, std::vector<uint8_t>>) {
      // std::string has special handling for better output
      return StringPrinter{};
    } else {
      return ContainerPrinter<Derived, InnerDomainT>{inner_};
    }
  }

  value_type GetValue(const corpus_type& value) const {
    if constexpr (has_custom_corpus_type) {
      value_type result;
      for (const auto& v : value) {
        result.insert(result.end(), inner_.GetValue(v));
      }
      return result;
    } else {
      return value;
    }
  }

  std::optional<corpus_type> FromValue(const value_type& value) const {
    if constexpr (!has_custom_corpus_type) {
      return value;
    } else {
      corpus_type copus_value;
      for (const auto& elem : value) {
        auto inner_value = inner_.FromValue(elem);
        if (!inner_value) return std::nullopt;
        copus_value.push_back(*std::move(inner_value));
      }
      return copus_value;
    }
  }

  std::optional<corpus_type> ParseCorpus(const IRObject& obj) const {
    // Use the generic serializer when no custom corpus type is used, since it
    // is more efficient. Eg a string value can be serialized as a string
    // instead of as a sequence of char values.
    if constexpr (has_custom_corpus_type) {
      auto subs = obj.Subs();
      if (!subs) return std::nullopt;
      corpus_type res;
      for (const auto& elem : *subs) {
        if (auto parsed_elem = inner_.ParseCorpus(elem)) {
          res.insert(res.end(), std::move(*parsed_elem));
        } else {
          return std::nullopt;
        }
      }
      return res;
    } else {
      return obj.ToCorpus<corpus_type>();
    }
  }

  IRObject SerializeCorpus(const corpus_type& v) const {
    if constexpr (has_custom_corpus_type) {
      IRObject obj;
      auto& subs = obj.MutableSubs();
      for (const auto& elem : v) {
        subs.push_back(inner_.SerializeCorpus(elem));
      }
      return obj;
    } else {
      return IRObject::FromCorpus(v);
    }
  }

  absl::Status ValidateCorpusValue(const corpus_type& corpus_value) const {
    // Check size.
    if (corpus_value.size() < min_size()) {
      return absl::InvalidArgumentError(absl::StrCat(
          "Invalid size: ", corpus_value.size(), ". Min size: ", min_size()));
    }
    if (corpus_value.size() > max_size()) {
      return absl::InvalidArgumentError(absl::StrCat(
          "Invalid size: ", corpus_value.size(), ". Max size: ", max_size()));
    }
    // Check elements.
    int i = 0;
    for (const auto& elem : corpus_value) {
      const absl::Status s = inner_.ValidateCorpusValue(elem);
      if (!s.ok()) {
        return Prefix(s,
                      absl::StrCat("Invalid value in container at index ", i));
      }
      i++;
    }
    return absl::OkStatus();
  }

  InnerDomainT Inner() const { return inner_; }

  // Needed for `CopyConstraintsFrom`.
  template <typename, typename, typename>
  friend class ContainerOfImplBase;

  template <typename OtherDerived, typename OtherT, typename OtherInnerDomain>
  void CopyConstraintsFrom(const ContainerOfImplBase<OtherDerived, OtherT,
                                                     OtherInnerDomain>& other) {
    min_size_ = other.min_size_;
    max_size_ = other.max_size_;
  }

 protected:
  InnerDomainT inner_;

  size_t ChooseRandomInitialSize(absl::BitGenRef prng) {
    // The container size should not be empty (unless max_size_ = 0) because the
    // initialization should be random if possible.
    // TODO(changochen): Increase the number of generated elements.
    // Currently we make container generate zero or one element to avoid
    // infinite recursion in recursive data structures. For the example, we want
    // to build a domain for `struct X{ int leaf; vector<X> recursive`, the
    // expected generated length is `E(X) = E(leaf) + E(recursive)`. If the
    // container generate `0-10` elements when calling `Init`, then
    // `E(recursive) =  4.5 E(X)`, which will make `E(X) = Infinite`.
    // Make some smallish random seed containers.
    return absl::Uniform(prng, min_size(),
                         std::min(max_size() + 1, min_size() + 2));
  }

  size_t min_size() const { return min_size_; }
  size_t max_size() const {
    return max_size_.value_or(std::max(min_size_, kDefaultContainerMaxSize));
  }

 private:
  Derived& Self() { return static_cast<Derived&>(*this); }
  dict_type& GetManualDict() {
    if (manual_dict_provider_.has_value() &&
        *manual_dict_provider_ != nullptr) {
      std::vector<value_type> manual_dict = (*manual_dict_provider_)();
      WithDictionary({manual_dict.data(), manual_dict.size()});
      manual_dict_provider_ = std::nullopt;
    }
    return manual_dict_;
  }

  // DO NOT use directly. Use min_size() and max_size() instead.
  size_t min_size_ = 0;
  std::optional<size_t> max_size_ = std::nullopt;

  // Temporary memory dictionary. Collected from tracing the program
  // execution. It will always be empty if no execution_coverage_ is found,
  // for example when running with other fuzzer engines.
  dict_type temporary_dict_ = {};

  // Dictionary provided by the user. It has the same type requirements as
  // memory dictionaries, but it could be made more general.
  // TODO(JunyangShao): make it more general.
  dict_type manual_dict_ = {};
  std::optional<std::function<std::vector<value_type>()>> manual_dict_provider_;

  // Permanent memory dictionary. Contains entries upgraded from
  // temporary_dict_. Upgrade happens when a temporary_dict_ entry
  // leads to new coverage.
  dict_type permanent_dict_ = {};
  static constexpr size_t kPermanentDictMaxSize = 512;

  // Keep tracks of what temporary_dict_ entry was used in the last dictionary
  // mutation. Will get upgraded into permanent_dict_ if it leads to new
  // coverages.
  std::optional<dict_entry_type> permanent_dict_candidate_ = std::nullopt;
};

// Base class for associative container domains, such as Domain<std::set> and
// Domain<absl::flat_hash_map>; these container have a key_type, used for
// element access by key.
template <typename T, typename InnerDomain>
class AssociativeContainerOfImpl
    : public ContainerOfImplBase<AssociativeContainerOfImpl<T, InnerDomain>> {
  using Base = typename AssociativeContainerOfImpl::ContainerOfImplBase;

 public:
  using typename Base::corpus_type;

  static_assert(Base::has_custom_corpus_type, "Must be custom to mutate keys");

  AssociativeContainerOfImpl() = default;
  explicit AssociativeContainerOfImpl(InnerDomain inner)
      : Base(std::move(inner)) {}

  corpus_type Init(absl::BitGenRef prng) {
    if (auto seed = this->MaybeGetRandomSeed(prng)) return *seed;
    const size_t size = this->ChooseRandomInitialSize(prng);

    corpus_type val;
    Grow(val, prng, size, 10000);
    if (val.size() < this->min_size()) {
      // We tried to make a container with the minimum specified size and we
      // could not after a lot of attempts. This could be caused by an
      // unsatisfiable domain, such as one where the minimum desired size is
      // greater than the number of unique `value_type` values that exist; for
      // example, a uint8_t has only 256 possible values, so we can't create
      // a std::set<uint8_t> whose size is greater than 256, as requested here:
      //
      //    SetOf(Arbitrary<uint8_t>()).WithMinSize(300)
      //
      // Abort the test and inform the user.
      AbortInTest(absl::StrFormat(R"(

[!] Ineffective use of WithSize()/WithMinSize() detected!

The domain failed trying to find enough values that satisfy the constraints.
The minimum size requested is %u and we could only find %u elements.

Please verify that the inner domain can provide enough values.
)",
                                  this->min_size(), val.size()));
    }
    return val;
  }

 private:
  friend Base;

  void GrowOne(corpus_type& val, absl::BitGenRef prng) {
    constexpr size_t kFailuresAllowed = 100;
    Grow(val, prng, 1, kFailuresAllowed);
  }

  // Try to grow `val` by `n` elements.
  void Grow(corpus_type& val, absl::BitGenRef prng, size_t n,
            size_t failures_allowed) {
    // Try a few times to insert a new element (correctly assuming the
    // initialization yields a random element if possible). We might get
    // duplicates. But don't try forever because we might be at the limit of the
    // container. Eg a set<char> with 256 elements can't grow anymore.
    //
    // Use the real value to make sure we are not adding invalid elements to the
    // list. The insertion in `real_value` will do the deduping for us.
    auto real_value = this->GetValue(val);
    const size_t final_size = real_value.size() + n;
    while (real_value.size() < final_size) {
      auto new_element = this->inner_.Init(prng);
      if (real_value.insert(this->inner_.GetValue(new_element)).second) {
        val.push_back(std::move(new_element));
      } else {
        // Just stop if we reached the allowed failures.
        // Let the caller decide what to do.
        if (failures_allowed-- == 0) return;
      }
    }
  }

  // Try to mutate the element in `it`.
  void MutateElement(corpus_type& val, absl::BitGenRef prng,
                     const domain_implementor::MutationMetadata& metadata,
                     bool only_shrink, typename corpus_type::iterator it) {
    size_t failures_allowed = 100;
    // Try a few times to mutate the element.
    // If the mutation reduces the number of elements in the container it means
    // we made the key collide with an existing element. Don't try forever as
    // there might not be any other value that we can mutate to.
    // Eg a set<char> with 256 elements can't mutate any of its elements.
    //
    // Use the real value to make sure we are not adding invalid elements to the
    // list. The insertion in `real_value` will do the deduping for us.
    corpus_type original_element_list;
    original_element_list.splice(original_element_list.end(), val, it);
    auto real_value = this->GetValue(val);

    while (failures_allowed > 0) {
      auto new_element = original_element_list.front();
      this->inner_.Mutate(new_element, prng, metadata, only_shrink);
      if (real_value.insert(this->inner_.GetValue(new_element)).second) {
        val.push_back(std::move(new_element));
        return;
      } else {
        --failures_allowed;
      }
    }
    // Give up and put the element back.
    val.splice(val.end(), original_element_list);
  }
};

template <typename Derived, typename T = ExtractTemplateParameter<0, Derived>,
          typename InnerDomain = ExtractTemplateParameter<1, Derived>>
class SequenceContainerOfImplBase
    : public ContainerOfImplBase<Derived, T, InnerDomain> {
  using Base = typename SequenceContainerOfImplBase::ContainerOfImplBase;

 public:
  using typename Base::corpus_type;

  SequenceContainerOfImplBase() = default;
  explicit SequenceContainerOfImplBase(InnerDomain inner)
      : Base(std::move(inner)) {}

  corpus_type Init(absl::BitGenRef prng) {
    if (auto seed = this->MaybeGetRandomSeed(prng)) return *seed;
    const size_t size = this->ChooseRandomInitialSize(prng);
    corpus_type val;
    while (val.size() < size) {
      val.insert(val.end(), this->inner_.Init(prng));
    }
    return val;
  }

  uint64_t CountNumberOfFields(corpus_type& val) {
    uint64_t total_weight = 0;
    for (auto& i : val) {
      total_weight += this->inner_.CountNumberOfFields(i);
    }
    return total_weight;
  }

  uint64_t MutateSelectedField(
      corpus_type& val, absl::BitGenRef prng,
      const domain_implementor::MutationMetadata& metadata, bool only_shrink,
      uint64_t selected_field_index) {
    uint64_t field_counter = 0;
    for (auto& i : val) {
      field_counter += this->inner_.MutateSelectedField(
          i, prng, metadata, only_shrink, selected_field_index - field_counter);
      if (field_counter >= selected_field_index) break;
    }
    return field_counter;
  }

 private:
  friend Base;

  void GrowOne(corpus_type& val, absl::BitGenRef prng) {
    val.insert(ChoosePosition(val, IncludeEnd::kYes, prng),
               this->inner_.Init(prng));
  }

  void MutateElement(corpus_type&, absl::BitGenRef prng,
                     const domain_implementor::MutationMetadata& metadata,
                     bool only_shrink, typename corpus_type::iterator it) {
    this->inner_.Mutate(*it, prng, metadata, only_shrink);
  }
};

template <typename T, typename InnerDomain>
class SequenceContainerOfImpl : public SequenceContainerOfImplBase<
                                    SequenceContainerOfImpl<T, InnerDomain>> {
  using Base = typename SequenceContainerOfImpl::SequenceContainerOfImplBase;

 public:
  using Base::Base;
};

template <typename T, typename InnerDomain>
using ContainerOfImpl =
    std::conditional_t<is_associative_container_v<T>,
                       AssociativeContainerOfImpl<T, InnerDomain>,
                       SequenceContainerOfImpl<T, InnerDomain>>;

}  // namespace fuzztest::internal

#endif  // FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_CONTAINER_OF_IMPL_H_
