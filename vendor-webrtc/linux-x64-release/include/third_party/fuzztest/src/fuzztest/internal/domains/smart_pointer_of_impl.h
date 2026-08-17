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

#ifndef FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_SMART_POINTER_OF_IMPL_H_
#define FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_SMART_POINTER_OF_IMPL_H_

#include <optional>
#include <type_traits>
#include <utility>
#include <variant>

#include "absl/random/bit_gen_ref.h"
#include "absl/random/distributions.h"
#include "absl/status/status.h"
#include "absl/strings/str_format.h"
#include "absl/strings/string_view.h"
#include "./fuzztest/internal/domains/domain.h"
#include "./fuzztest/internal/domains/domain_base.h"
#include "./fuzztest/internal/domains/serialization_helpers.h"
#include "./fuzztest/internal/meta.h"
#include "./fuzztest/internal/printer.h"
#include "./fuzztest/internal/serialization.h"
#include "./fuzztest/internal/status.h"

namespace fuzztest::internal {

template <typename T>
class SmartPointerOfImpl
    : public domain_implementor::DomainBase<
          SmartPointerOfImpl<T>, T,
          std::variant<std::monostate, GenericDomainCorpusType>> {
  using Inner = Domain<typename T::element_type>;
  using InnerFn = const Inner& (*)();

 public:
  using typename SmartPointerOfImpl::DomainBase::corpus_type;
  using typename SmartPointerOfImpl::DomainBase::value_type;

  static_assert(
      Requires<T>(
          [](auto x) -> std::enable_if_t<std::is_pointer_v<decltype(x.get())>,
                                         decltype(!x, *x)> {}),
      "T must be a smart pointer type.");

  // Since we allow for recursion in this domain, we want to delay the
  // construction of the inner domain. Otherwise we would have an infinite
  // recursion of domains being created.
  explicit SmartPointerOfImpl(InnerFn fn) : inner_(fn) {}
  explicit SmartPointerOfImpl(Inner inner) : inner_(std::move(inner)) {}

  corpus_type Init(absl::BitGenRef prng) {
    if (auto seed = this->MaybeGetRandomSeed(prng)) return *seed;
    // Init will always have an empty smart pointer to reduce nesting.
    // Otherwise it is very easy to get a stack overflow during Init() when
    // there is recursion in the domains.
    return corpus_type();
  }

  void Mutate(corpus_type& val, absl::BitGenRef prng,
              const domain_implementor::MutationMetadata& metadata,
              bool only_shrink) {
    const bool has_value = val.index() == 1;
    if (!has_value) {
      // Only add a value if we are not shrinking.
      if (!only_shrink) val.template emplace<1>(GetOrMakeInner().Init(prng));
    } else if (absl::Bernoulli(prng, 1. / 100)) {
      // 1/100 chance of returning an empty.
      val.template emplace<0>();
    } else {
      GetOrMakeInner().Mutate(std::get<1>(val), prng, metadata, only_shrink);
    }
  }

  auto GetPrinter() const { return Printer{GetOrMakeInnerConst()}; }

  value_type GetValue(const corpus_type& v) const {
    if (v.index() == 0) return value_type();
    return value_type(new auto(GetOrMakeInnerConst().GetValue(std::get<1>(v))));
  }

  std::optional<corpus_type> FromValue(const value_type& v) const {
    if (!v) return corpus_type(std::in_place_index<0>);
    if (auto inner_value = GetOrMakeInnerConst().FromValue(*v)) {
      return corpus_type(std::in_place_index<1>, *std::move(inner_value));
    } else {
      return std::nullopt;
    }
  }

  std::optional<corpus_type> ParseCorpus(const IRObject& obj) const {
    return ParseWithDomainOptional(GetOrMakeInnerConst(), obj);
  }

  IRObject SerializeCorpus(const corpus_type& v) const {
    return SerializeWithDomainOptional(GetOrMakeInnerConst(), v);
  }

  absl::Status ValidateCorpusValue(const corpus_type& corpus_value) const {
    if (corpus_value.index() == 0) return absl::OkStatus();
    const absl::Status s =
        GetOrMakeInnerConst().ValidateCorpusValue(std::get<1>(corpus_value));
    return Prefix(s, "Invalid value for smart pointer domain");
  }

 private:
  struct Printer {
    const Inner& inner;

    void PrintCorpusValue(const corpus_type& v, domain_implementor::RawSink out,
                          domain_implementor::PrintMode mode) const {
      if (v.index() == 0) {
        absl::Format(out, "nullptr");
        return;
      }
      if (mode == domain_implementor::PrintMode::kSourceCode) {
        absl::string_view maker =
            is_unique_ptr_v<value_type>   ? "std::make_unique"
            : is_shared_ptr_v<value_type> ? "std::make_shared"
                                          : "<MAKE_SMART_POINTER>";
        absl::Format(out, "%s<%s>", maker,
                     GetTypeName<typename value_type::element_type>());
      }
      absl::Format(out, "(");
      PrintValue(inner, std::get<1>(v), out, mode);
      absl::Format(out, ")");
    }
  };

  Inner& GetOrMakeInner() {
    if (inner_.index() == 0) {
      inner_.template emplace<1>(std::get<0>(inner_)());
    }
    return std::get<1>(inner_);
  }

  const Inner& GetOrMakeInnerConst() const {
    return inner_.index() == 0 ? std::get<0>(inner_)() : std::get<1>(inner_);
  }

  // We don't construct it eagerly to avoid an infinite recursion during
  // default construction. We only construct the sub domain on demand.
  std::variant<InnerFn, Inner> inner_;
};

}  // namespace fuzztest::internal

#endif  // FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_SMART_POINTER_OF_IMPL_H_
