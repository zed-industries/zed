// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_FUZZTEST_SUPPORT_H_
#define BASE_TEST_FUZZTEST_SUPPORT_H_

#include <optional>
#include <string>
#include <tuple>
#include <vector>

#include "base/containers/to_vector.h"
#include "base/types/optional_ref.h"
#include "base/values.h"
#include "third_party/fuzztest/src/fuzztest/fuzztest.h"

namespace {

template <typename T>
  requires std::copy_constructible<T>
std::optional<std::tuple<T>> Wrap(base::optional_ref<const T> maybe_value) {
  return maybe_value.has_value() ? std::optional(std::tuple<T>{*maybe_value})
                                 : std::nullopt;
}

auto ArbitraryValueNull() {
  return fuzztest::ReversibleMap(
      [] { return base::Value(); },
      [](const base::Value& value) { return std::optional<std::tuple<>>{}; });
}

auto ArbitraryValueBool() {
  return fuzztest::ReversibleMap(
      [](bool boolean) { return base::Value(boolean); },
      [](const base::Value& value) { return Wrap<bool>(value.GetIfBool()); },
      fuzztest::Arbitrary<bool>());
}

auto ArbitraryValueInt() {
  return fuzztest::ReversibleMap(
      [](int number) { return base::Value(number); },
      [](const base::Value& value) { return Wrap<int>(value.GetIfInt()); },
      fuzztest::Arbitrary<int>());
}

auto ArbitraryValueDouble() {
  return fuzztest::ReversibleMap(
      [](double number) { return base::Value(number); },
      [](const base::Value& value) {
        return Wrap<double>(value.GetIfDouble());
      },
      fuzztest::Finite<double>());
}

auto ArbitraryValueString() {
  return fuzztest::ReversibleMap(
      [](std::string string) { return base::Value(std::move(string)); },
      [](const base::Value& value) {
        return Wrap<std::string>(value.GetIfString());
      },
      // TODO: Strings should not be constrained to ASCII.
      fuzztest::AsciiString());
}

auto ArbitraryValueBlob() {
  return fuzztest::ReversibleMap(
      [](std::vector<uint8_t> blob) { return base::Value(std::move(blob)); },
      [](const base::Value& value) {
        return Wrap<std::vector<uint8_t>>(value.GetIfBlob());
      },
      fuzztest::Arbitrary<std::vector<uint8_t>>());
}

auto ArbitraryValueList(fuzztest::Domain<base::Value> entry_domain) {
  return fuzztest::ReversibleMap(
      [](std::vector<base::Value> values) {
        auto list = base::Value::List::with_capacity(values.size());
        for (auto& value : values) {
          list.Append(std::move(value));
        }
        return base::Value(std::move(list));
      },
      [](const base::Value& value) {
        auto maybe_list = base::optional_ref(value.GetIfList());
        return maybe_list.has_value()
                   ? std::make_optional(std::tuple{
                         base::ToVector(*maybe_list, &base::Value::Clone)})
                   : std::nullopt;
      },
      fuzztest::ContainerOf<std::vector<base::Value>>(entry_domain));
}

auto ArbitraryValueDict(fuzztest::Domain<base::Value> value_domain) {
  return fuzztest::ReversibleMap(
      [](std::vector<std::pair<std::string, base::Value>> e) {
        return base::Value(base::Value::Dict(std::make_move_iterator(e.begin()),
                                             std::make_move_iterator(e.end())));
      },
      [](const base::Value& value) {
        auto maybe_dict = base::optional_ref(value.GetIfDict());
        return maybe_dict.has_value()
                   ? std::make_optional(std::tuple{base::ToVector(
                         *maybe_dict,
                         [](const auto& entry) {
                           return std::make_pair(entry.first,
                                                 entry.second.Clone());
                         })})
                   : std::nullopt;
      },
      fuzztest::ContainerOf<std::vector<std::pair<std::string, base::Value>>>(
          // TODO: Keys should not be constrained to ASCII.
          fuzztest::PairOf(fuzztest::AsciiString(), value_domain)));
}

fuzztest::Domain<base::Value> ArbitraryValue() {
  fuzztest::DomainBuilder builder;
  builder.Set<base::Value>(
      "value",
      fuzztest::OneOf(ArbitraryValueNull(), ArbitraryValueBool(),
                      ArbitraryValueInt(), ArbitraryValueDouble(),
                      ArbitraryValueString(), ArbitraryValueBlob(),
                      ArbitraryValueList(builder.Get<base::Value>("value")),
                      ArbitraryValueDict(builder.Get<base::Value>("value"))));
  return std::move(builder).Finalize<base::Value>("value");
}

}  // namespace

template <>
class fuzztest::internal::ArbitraryImpl<base::Value>
    : public fuzztest::Domain<base::Value> {
 public:
  ArbitraryImpl() : fuzztest::Domain<base::Value>(ArbitraryValue()) {}
};

template <>
class fuzztest::internal::ArbitraryImpl<base::Value::Dict>
    : public fuzztest::internal::ReversibleMapImpl<
          base::Value::Dict (*)(
              std::vector<std::pair<std::string, base::Value>>),
          std::optional<
              std::tuple<std::vector<std::pair<std::string, base::Value>>>> (*)(
              const base::Value::Dict&),
          fuzztest::internal::ArbitraryImpl<
              std::vector<std::pair<std::string, base::Value>>>> {
 public:
  ArbitraryImpl()
      : fuzztest::internal::ReversibleMapImpl<
            base::Value::Dict (*)(
                std::vector<std::pair<std::string, base::Value>>),
            std::optional<std::tuple<
                std::vector<std::pair<std::string, base::Value>>>> (*)(
                const base::Value::Dict&),
            fuzztest::internal::ArbitraryImpl<
                std::vector<std::pair<std::string, base::Value>>>>(
            [](std::vector<std::pair<std::string, base::Value>> e) {
              return base::Value::Dict(std::make_move_iterator(e.begin()),
                                       std::make_move_iterator(e.end()));
            },
            [](const base::Value::Dict& value) {
              return std::make_optional(
                  std::tuple{base::ToVector(value, [](const auto& entry) {
                    return std::make_pair(entry.first, entry.second.Clone());
                  })});
            },
            fuzztest::internal::ArbitraryImpl<
                std::vector<std::pair<std::string, base::Value>>>()) {}
};

#endif  // BASE_TEST_FUZZTEST_SUPPORT_H_
