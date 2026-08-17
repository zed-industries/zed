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

#ifndef FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_SERIALIZATION_HELPERS_H_
#define FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_SERIALIZATION_HELPERS_H_

#include <cstddef>
#include <optional>
#include <tuple>
#include <utility>
#include <variant>
#include <vector>

#include "absl/types/span.h"
#include "./fuzztest/internal/domains/domain.h"
#include "./fuzztest/internal/meta.h"
#include "./fuzztest/internal/serialization.h"

namespace fuzztest::internal {

// Helper serialization functions for common patterns: optional/variant/tuple.

template <typename... Domain>
IRObject SerializeWithDomainVariant(
    const std::tuple<Domain...>& domains,
    const std::variant<corpus_type_t<Domain>...>& v) {
  IRObject obj;
  auto& subs = obj.MutableSubs();
  subs.push_back(IRObject::FromCorpus(v.index()));
  Switch<sizeof...(Domain)>(v.index(), [&](auto I) {
    subs.push_back(std::get<I>(domains).SerializeCorpus(std::get<I>(v)));
  });
  return obj;
}

template <typename... Domain,
          typename ReturnT = std::variant<corpus_type_t<Domain>...>>
std::optional<ReturnT> ParseWithDomainVariant(
    const std::tuple<Domain...>& domains, const IRObject& obj) {
  auto subs = obj.Subs();
  if (!subs || subs->size() != 2) return std::nullopt;

  auto alternative = (*subs)[0].GetScalar<size_t>();
  if (!alternative || *alternative >= sizeof...(Domain)) return std::nullopt;

  return Switch<sizeof...(Domain)>(
      *alternative, [&](auto I) -> std::optional<ReturnT> {
        auto inner_corpus = std::get<I>(domains).ParseCorpus((*subs)[1]);
        if (!inner_corpus) return std::nullopt;
        return ReturnT(std::in_place_index<I>, *std::move(inner_corpus));
      });
}

template <typename T>
auto SerializeWithDomainOptional(
    const Domain<T>& domain,
    const std::variant<std::monostate, GenericDomainCorpusType>& v) {
  IRObject obj;
  auto& subs = obj.MutableSubs();
  subs.push_back(IRObject(v.index()));
  if (v.index() == 1) {
    subs.push_back(domain.SerializeCorpus(std::get<1>(v)));
  }
  return obj;
}

template <typename T, typename ReturnT =
                          std::variant<std::monostate, GenericDomainCorpusType>>
std::optional<ReturnT> ParseWithDomainOptional(const Domain<T>& domain,
                                               const IRObject& obj) {
  auto subs = obj.Subs();
  if (!subs || subs->empty()) return std::nullopt;
  auto index = (*subs)[0].GetScalar<size_t>();
  if (index == 0) {
    if (subs->size() != 1) return std::nullopt;
    return ReturnT();
  } else if (index == 1) {
    if (subs->size() != 2) return std::nullopt;
    auto inner_corpus = domain.ParseCorpus((*subs)[1]);
    if (!inner_corpus) return std::nullopt;
    return ReturnT(std::in_place_index<1>, *std::move(inner_corpus));
  } else {
    return std::nullopt;
  }
}

template <typename... Domain>
auto SerializeWithDomainTuple(
    const std::tuple<Domain...>& domains,
    const std::tuple<corpus_type_t<Domain>...>& corpus) {
  IRObject obj;
  auto& subs = obj.MutableSubs();
  ApplyIndex<sizeof...(Domain)>([&](auto... I) {
    (subs.push_back(std::get<I>(domains).SerializeCorpus(std::get<I>(corpus))),
     ...);
  });
  return obj;
}

// Parse a corpus given a tuple of domains, skipping the first `skip` subs.
template <typename... Domain>
std::optional<std::tuple<corpus_type_t<Domain>...>> ParseWithDomainTuple(
    const std::tuple<Domain...>& domains, const IRObject& obj, int skip = 0) {
  auto subs = obj.Subs();
  if (!subs || subs->size() != sizeof...(Domain) + skip) return std::nullopt;
  return ApplyIndex<sizeof...(Domain)>([&](auto... I) {
    return [](auto... opts) {
      return (!opts || ...)
                 ? std::nullopt
                 : std::optional(std::tuple<corpus_type_t<Domain>...>{
                       *std::move(opts)...});
    }(std::get<I>(domains).ParseCorpus((*subs)[I + skip])...);
  });
}

}  // namespace fuzztest::internal

#endif  // FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_SERIALIZATION_HELPERS_H_
