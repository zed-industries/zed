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

#ifndef FUZZTEST_INTERNAL_DOMAIN_TESTING_H_
#define FUZZTEST_INTERNAL_DOMAIN_TESTING_H_

#include <algorithm>
#include <cmath>
#include <cstddef>
#include <limits>
#include <optional>
#include <ostream>
#include <set>
#include <string>
#include <utility>
#include <vector>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "absl/container/flat_hash_set.h"
#include "absl/hash/hash.h"
#include "absl/random/bit_gen_ref.h"
#include "absl/random/random.h"
#include "absl/status/status.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/string_view.h"
#include "./fuzztest/internal/domains/mutation_metadata.h"
#include "./fuzztest/internal/logging.h"
#include "./fuzztest/internal/meta.h"
#include "./fuzztest/internal/serialization.h"
#include "./fuzztest/internal/test_protobuf.pb.h"
#include "google/protobuf/util/field_comparator.h"
#include "google/protobuf/util/message_differencer.h"

namespace fuzztest {

// Status matchers.

MATCHER_P(StatusIs, status_code, "") { return arg.code() == status_code; }

MATCHER_P2(StatusIs, status_code, message, "") {
  return (arg.code() == status_code) &&
         testing::Matches(message)(std::string(arg.message()));
}

MATCHER_P(IsInvalid, message, "") {
  return testing::ExplainMatchResult(
      StatusIs(absl::StatusCode::kInvalidArgument, message), arg,
      result_listener);
}

#ifndef ASSERT_OK
#define ASSERT_OK(x) ASSERT_THAT(x, StatusIs(absl::StatusCode::kOk))
#endif  // ASSERT_OK
#ifndef EXPECT_OK
#define EXPECT_OK(x) EXPECT_THAT(x, StatusIs(absl::StatusCode::kOk))
#endif  // EXPECT_OK

// Tests whether arg is in the range [a, b].
MATCHER_P2(IsInClosedRange, a, b,
           absl::StrCat(negation ? "isn't" : "is", " in the closed range [",
                        testing::PrintToString(a), ", ",
                        testing::PrintToString(b), "]")) {
  return a <= arg && arg <= b;
}

// Make sure floating hash/eq handle NaN.
struct Hash {
  template <typename T>
  size_t operator()(const T& v) const {
    if constexpr (internal::Requires<T>(
                      [](auto v) -> decltype(v && v.get()) {})) {
      // Smart pointers.
      return v ? absl::HashOf(*v) : 0;
    } else if constexpr (internal::Requires<T>(
                             [](auto v) -> decltype(std::isnan(
                                            *std::optional(v))) {})) {
      auto o = std::optional(v);
      return !o || std::isnan(*o) ? 0 : absl::Hash<T>{}(*o);
    } else if constexpr (internal::Requires<T>(
                             [](auto v) -> decltype(v.hash_function()) {})) {
      return (*this)(std::set<internal::value_type_t<T>>(v.begin(), v.end()));
    } else {
      return absl::Hash<T>{}(v);
    }
  }
};

struct Eq {
  template <typename T>
  bool operator()(const T& a, const T& b) const {
    if constexpr (internal::is_protocol_buffer_v<T>) {
      google::protobuf::util::MessageDifferencer differencer;
      google::protobuf::util::DefaultFieldComparator cmp;
      cmp.set_treat_nan_as_equal(true);
      differencer.set_field_comparator(&cmp);
      return differencer.Compare(a, b);
    } else if constexpr (internal::Requires<T>(
                             [](auto v) -> decltype(v && v.get()) {})) {
      // Smart pointers.
      return a ? b && *a == *b : !b;
    } else if constexpr (internal::Requires<T>(
                             [](auto v) -> decltype(std::isnan(
                                            *std::optional(v))) {})) {
      auto oa = std::optional(a), ob = std::optional(b);
      return a == b || (oa && ob && std::isnan(*oa) && std::isnan(*ob));
    } else {
      return a == b;
    }
  }
};

template <typename T>
using Set = absl::flat_hash_set<T, Hash, Eq>;

// The Value class keeps the corpus and value types together throughout tests to
// simplify their access and mutation.
template <typename Domain>
struct Value {
  using T = internal::value_type_t<Domain>;
  internal::corpus_type_t<Domain> corpus_value;
  T user_value;

  Value(Domain& domain, absl::BitGenRef prng)
      : corpus_value(domain.Init(prng)),
        user_value(domain.GetValue(corpus_value)) {}

  // If the value_type is not copy constructible we have to copy the corpus and
  // regenerate the value.
  Value(const Value& other, Domain& domain)
      : corpus_value(other.corpus_value),
        user_value(domain.GetValue(corpus_value)) {}

  Value(const Domain& domain, T user_value)
      : corpus_value([&]() {
          auto corpus_value = domain.FromValue(user_value);
          FUZZTEST_INTERNAL_CHECK_PRECONDITION(corpus_value.has_value(),
                                               "Invalid user_value!");
          return *corpus_value;
        }()),
        user_value(std::move(user_value)) {}

  void Mutate(Domain& domain, absl::BitGenRef prng,
              const domain_implementor::MutationMetadata& metadata,
              bool only_shrink) {
    domain.Mutate(corpus_value, prng, metadata, only_shrink);
    user_value = domain.GetValue(corpus_value);
  }

  void RandomizeByRepeatedMutation(
      Domain& domain, absl::BitGenRef prng,
      const domain_implementor::MutationMetadata& metadata = {},
      bool only_shrink = false) {
    static constexpr int kMutations = 1000;
    for (int i = 0; i < kMutations; ++i) {
      domain.Mutate(corpus_value, prng, metadata, only_shrink);
    }
    user_value = domain.GetValue(corpus_value);
  }

  // Make the Value hashable/comparable to put them in sets/maps.
  template <typename H>
  friend H AbslHashValue(H state, const Value& self) {
    const auto& v = self.user_value;
    if constexpr (internal::Requires<T>(
                      [](auto v) -> decltype(std::isnan(*std::optional(v))) {
                      })) {
      auto o = std::optional(v);
      if (o && std::isnan(*o)) o = 0;
      return H::combine(std::move(state), o);
    } else if constexpr (internal::Requires<T>(
                             [](auto v) -> decltype(v.hash_function()) {})) {
      return H::combine(std::move(state), std::set<internal::value_type_t<T>>(
                                              v.begin(), v.end()));
    } else {
      return H::combine(std::move(state), v);
    }
  }

  friend bool operator==(const Value& a, const Value& b) {
    if constexpr (internal::Requires<T>(
                      [](auto v) -> decltype(std::isnan(*std::optional(v))) {
                      })) {
      auto oa = std::optional(a.user_value), ob = std::optional(b.user_value);
      return a.user_value == b.user_value ||
             (oa && ob && std::isnan(*oa) && std::isnan(*ob));
    } else {
      return a.user_value == b.user_value;
    }
  }

  friend bool operator!=(const Value& a, const Value& b) { return !(a == b); }

  friend bool operator==(const Value& a, const T& b) {
    return a.user_value == b;
  }
  friend bool operator!=(const Value& a, const T& b) {
    return a.user_value != b;
  }
  friend bool operator<(const Value& a, const T& b) { return a.user_value < b; }
  friend bool operator>(const Value& a, const T& b) { return a.user_value > b; }
  friend bool operator<=(const Value& a, const T& b) {
    return a.user_value <= b;
  }
  friend bool operator>=(const Value& a, const T& b) {
    return a.user_value >= b;
  }
  friend std::ostream& operator<<(std::ostream& s, const Value& v) {
    return s << testing::PrintToString(v.user_value);
  }

 private:
  // We don't test the printers here, just that we return one.
  // The printers themselves are tested in type_support_test.cc
  using Printer = decltype(std::declval<const Domain&>().GetPrinter());
};

template <typename Domain>
void VerifyRoundTripThroughConversion(const Value<Domain>& v,
                                      const Domain& domain) {
  {
    auto corpus_value = domain.FromValue(v.user_value);
    ASSERT_TRUE(corpus_value) << v;
    ASSERT_OK(domain.ValidateCorpusValue(*corpus_value));
    auto new_v = domain.GetValue(*corpus_value);
    EXPECT_TRUE(Eq{}(v.user_value, new_v))
        << "v=" << v << " new_v=" << testing::PrintToString(new_v);
  }
  {
    auto serialized = domain.SerializeCorpus(v.corpus_value).ToString();
    auto parsed = internal::IRObject::FromString(serialized);
    ASSERT_TRUE(parsed);
    auto parsed_corpus = domain.ParseCorpus(*parsed);
    ASSERT_TRUE(parsed_corpus)
        << serialized << " value = " << testing::PrintToString(v.user_value);
    ASSERT_OK(domain.ValidateCorpusValue(*parsed_corpus));
    EXPECT_TRUE(Eq{}(v.user_value, domain.GetValue(*parsed_corpus)));
  }
}

template <typename Container, typename Domain>
void VerifyRoundTripThroughConversion(const Container& values,
                                      const Domain& domain) {
  for (const auto& v : values) {
    VerifyRoundTripThroughConversion(v, domain);
  }
}

template <typename Domain>
Value(Domain&, absl::BitGenRef) -> Value<Domain>;

template <typename Domain>
auto GenerateValues(Domain domain, int num_seeds = 10, int num_mutations = 100,
                    const domain_implementor::MutationMetadata& metadata = {},
                    bool only_shrink = false) {
  absl::BitGen bitgen;

  absl::flat_hash_set<Value<Domain>> seeds;
  // Make sure we can make some unique seeds.
  // Randomness might create duplicates so keep going until we got them.
  while (seeds.size() < num_seeds) {
    seeds.insert(Value(domain, bitgen));
  }

  auto values = seeds;

  for (const auto& seed : seeds) {
    auto value = seed;

    absl::flat_hash_set<Value<Domain>> mutations = {value};
    // As above, we repeat until we find enough unique ones.
    while (mutations.size() < num_mutations) {
      const auto previous = value;
      value.Mutate(domain, bitgen, metadata, only_shrink);
      // Make sure that it changed in some way.
      mutations.insert(value);
      EXPECT_NE(previous, value) << "Value=" << value << " Prev=" << previous;
    }
    values.merge(mutations);
  }

  return values;
}

template <typename Domain>
auto GenerateNonUniqueValues(
    Domain domain, int num_seeds = 10, int num_mutations = 100,
    const domain_implementor::MutationMetadata& metadata = {},
    bool only_shrink = false) {
  absl::BitGen bitgen;

  std::vector<Value<Domain>> seeds;
  while (seeds.size() < num_seeds) {
    seeds.push_back(Value(domain, bitgen));
  }

  auto values = seeds;

  for (const auto& seed : seeds) {
    auto value = seed;
    std::vector<Value<Domain>> mutations = {value};
    while (mutations.size() < num_mutations) {
      value.Mutate(domain, bitgen, metadata, only_shrink);
      mutations.push_back(value);
    }
    values.insert(values.end(), mutations.begin(), mutations.end());
  }

  return values;
}

template <typename Domain>
auto GenerateInitialValues(Domain domain, int n) {
  std::vector<Value<Domain>> values;
  absl::BitGen bitgen;
  values.reserve(n);
  for (int i = 0; i < n; ++i) {
    values.push_back(Value(domain, bitgen));
  }
  return values;
}

template <typename Values, typename Pred>
void CheckValues(const Values& values, Pred pred) {
  for (const auto& value : values) {
    ASSERT_TRUE(pred(value.user_value)) << "Incorrect value: " << value;
  }
}

template <typename Domain>
auto MutateUntilFoundN(
    Domain domain, size_t n,
    const domain_implementor::MutationMetadata& metadata = {},
    bool only_shrink = false) {
  absl::flat_hash_set<Value<Domain>> seen;
  absl::BitGen bitgen;
  Value val(domain, bitgen);
  while (seen.size() < n) {
    seen.insert(Value(val, domain));
    val.Mutate(domain, bitgen, metadata, only_shrink);
  }
  return seen;
}

template <typename Domain, typename IsTerminal, typename IsCloser,
          typename T = internal::value_type_t<Domain>>
absl::Status TestShrink(
    Domain domain, const absl::flat_hash_set<Value<Domain>>& values,
    IsTerminal is_terminal, IsCloser is_closer_to_zero,
    const domain_implementor::MutationMetadata& metadata = {}) {
  absl::BitGen bitgen;

  for (auto value : values) {
    while (!is_terminal(value.user_value)) {
      auto previous_value = value;
      value.Mutate(domain, bitgen, metadata, true);
      if (value == previous_value) {
        return absl::InternalError(
            absl::StrCat("Mutate failed to produce a new value starting from ",
                         testing::PrintToString(previous_value.user_value)));
      }
      if (!is_terminal(value.user_value)) {
        if (!is_closer_to_zero(previous_value.user_value, value.user_value)) {
          return absl::InternalError(absl::StrCat(
              "While shrinking, the value of ",
              testing::PrintToString(value.user_value),
              " was not closer to zero than the previous value of ",
              testing::PrintToString(previous_value.user_value)));
        }
      }
    }
  }
  return absl::OkStatus();
}

template <typename Domain, typename Values, typename Pred>
void TestShrink(Domain domain, const Values& values, Pred pred) {
  absl::BitGen bitgen;

  for (const auto& value : values) {
    auto other_value = value;
    other_value.Mutate(domain, bitgen, {}, true);
    ASSERT_TRUE(pred(value.user_value, other_value.user_value))
        << value << " " << other_value;
  }
}

template <typename T>
bool AreSame(const T& prev, const T& next) {
  if constexpr (std::is_arithmetic_v<T>) {
    return prev == next || (std::isnan(prev) && std::isnan(next));
  } else if constexpr (std::is_enum_v<T> ||
                       std::is_same_v<std::optional<int>, T> ||
                       std::is_same_v<std::pair<int, int>, T> ||
                       std::is_same_v<std::pair<const int, int>, T>) {
    return prev == next;
  } else {
    // For container types.
    if (prev.size() != next.size()) return false;
    for (auto prev_i = prev.begin(), next_i = next.begin();
         prev_i != prev.end(); ++prev_i, ++next_i) {
      if (!AreSame(*prev_i, *next_i)) return false;
    }
    return true;
  }
}

template <typename T>
bool TowardsZero(const T& prev, const T& next) {
  if constexpr (std::is_same_v<std::byte, T> ||
                std::is_same_v<const std::byte, T>) {
    return TowardsZero(std::to_integer<unsigned char>(prev),
                       std::to_integer<unsigned char>(next));
  } else if constexpr (std::numeric_limits<T>::is_integer ||
                       std::is_arithmetic_v<T> || std::is_enum_v<T>) {
    if constexpr (std::is_floating_point_v<T>) {
      // Ignore non-finites. Those never move towards zero.
      if (!std::isfinite(prev) || !std::isfinite(next)) return true;
    }
    return (prev == next && prev == 0) ||
           (prev < 0 && prev < next && next <= 0) ||
           (prev >= 0 && 0 <= next && next < prev);
  } else if constexpr (std::is_same_v<std::optional<int>, T>) {
    return (prev && next && TowardsZero(*prev, *next)) || (prev && !next) ||
           (!prev && !next);
  } else if constexpr (std::is_same_v<std::pair<int, int>, T> ||
                       std::is_same_v<std::pair<const int, int>, T> ||
                       std::is_same_v<std::pair<const std::string, int>, T>) {
    return TowardsZero(prev.first, next.first) ||
           TowardsZero(prev.second, next.second);
  } else {
    // For container types.
    if (prev.empty() && next.empty()) return true;
    if (prev.size() != next.size()) return prev.size() > next.size();
    // Something inside shrunk.
    if constexpr (internal::Requires<T>([](auto v) ->
                                        typename decltype(v)::key_type {})) {
      const auto get_key = [](auto it) -> decltype(auto) {
        if constexpr (std::is_same_v<typename T::key_type,
                                     typename T::value_type>) {
          return *it;
        } else {
          return it->first;
        }
      };
      // There can be at most one element whose key doesn't match. Compare that
      // one separately after the loop.
      std::optional<typename T::const_iterator> missing_prev, missing_next;
      // For associative containers, do a find.
      // Sequential iteration will not work for unordered ones.
      for (auto it = prev.begin(); it != prev.end(); ++it) {
        auto rhs = next.find(get_key(it));
        if (rhs != next.end()) {
          if (!TowardsZero(*it, *rhs) && !Eq{}(*it, *rhs)) return false;
        } else {
          if (missing_prev.has_value()) return false;
          missing_prev = it;
        }
      }
      for (auto it = next.begin(); it != next.end(); ++it) {
        if (prev.find(get_key(it)) == prev.end()) {
          if (missing_next.has_value()) return false;
          missing_next = it;
        }
      }

      if (missing_prev.has_value() != missing_next.has_value()) return false;
      if (missing_prev.has_value()) {
        return TowardsZero(**missing_prev, **missing_next);
      }
      return true;
    } else {
      for (auto prev_i = prev.begin(), next_i = next.begin();
           prev_i != prev.end(); ++prev_i, ++next_i) {
        if (TowardsZero(*prev_i, *next_i)) return true;
      }
      return false;
    }
  }
}

template <typename ScalarVisitor, typename RepeatedVisitor>
void VisitTestProtobuf(ScalarVisitor scalar_visitor,
                       RepeatedVisitor repeated_visitor) {
  scalar_visitor(
      "b", [](auto& val) { return val.has_b(); },
      [](auto& val) { return val.b(); });
  scalar_visitor(
      "i32", [](auto& val) { return val.has_i32(); },
      [](auto& val) { return val.i32(); });
  scalar_visitor(
      "u32", [](auto& val) { return val.has_u32(); },
      [](auto& val) { return val.u32(); });
  scalar_visitor(
      "i64", [](auto& val) { return val.has_i64(); },
      [](auto& val) { return val.i64(); });
  scalar_visitor(
      "u64", [](auto& val) { return val.has_u64(); },
      [](auto& val) { return val.u64(); });
  scalar_visitor(
      "f", [](auto& val) { return val.has_f(); },
      [](auto& val) { return val.f(); });
  scalar_visitor(
      "d", [](auto& val) { return val.has_d(); },
      [](auto& val) { return val.d(); });
  scalar_visitor(
      "str", [](auto& val) { return val.has_str(); },
      [](auto& val) { return val.str(); });
  scalar_visitor(
      "oneof_i32", [](auto& val) { return val.has_oneof_i32(); },
      [](auto& val) { return val.oneof_i32(); });
  scalar_visitor(
      "oneof_i64", [](auto& val) { return val.has_oneof_i64(); },
      [](auto& val) { return val.oneof_i64(); });
  scalar_visitor(
      "subproto_i32",
      [](auto& val) { return val.subproto().has_subproto_i32(); },
      [](auto& val) { return val.subproto().subproto_i32(); });
  scalar_visitor(
      "e", [](auto& val) { return val.has_e(); },
      [](auto& val) { return val.e(); });

  repeated_visitor("rep_b", [](auto& val) { return val.rep_b(); });
  repeated_visitor("rep_i32", [](auto& val) { return val.rep_i32(); });
  repeated_visitor("rep_u32", [](auto& val) { return val.rep_u32(); });
  repeated_visitor("rep_i64", [](auto& val) { return val.rep_i64(); });
  repeated_visitor("rep_u64", [](auto& val) { return val.rep_u64(); });
  repeated_visitor("rep_f", [](auto& val) { return val.rep_f(); });
  repeated_visitor("rep_d", [](auto& val) { return val.rep_d(); });
  repeated_visitor("rep_str", [](auto& val) { return val.rep_str(); });
  repeated_visitor("map_field", [](auto& val) {
    std::vector<std::pair<int, int>> v;
    for (const auto& p : val.map_field()) v.push_back(p);
    std::sort(v.begin(), v.end());
    return v;
  });
  repeated_visitor("rep_subproto", [](auto& val) {
    std::vector<std::optional<int>> v;
    for (const auto& s : val.rep_subproto()) {
      v.push_back(s.has_subproto_i32() ? std::optional(s.subproto_i32())
                                       : std::nullopt);
    }
    return v;
  });
  repeated_visitor("rep_e", [](auto& val) { return val.rep_e(); });
}

inline bool TowardsZero(const internal::TestProtobuf& prev,
                        const internal::TestProtobuf& next) {
  absl::string_view error_field;
  const auto verify_towards_zero = [&](absl::string_view name, auto has,
                                       auto get) {
    auto pv = has(prev) ? std::optional(get(prev)) : std::nullopt;
    auto nv = has(next) ? std::optional(get(next)) : std::nullopt;

    if (pv && nv && !TowardsZero(*pv, *nv) && !AreSame(*pv, *nv)) {
      error_field = name;
    }
    if (!pv && nv) {
      error_field = name;
    }
  };

  const auto verify_repeated_towards_zero = [&](absl::string_view name,
                                                auto get) {
    auto pv = get(prev);
    auto nv = get(next);
    if (!TowardsZero(pv, nv) && !AreSame(pv, nv)) {
      error_field = name;
    }
  };

  VisitTestProtobuf(verify_towards_zero, verify_repeated_towards_zero);
  if (error_field.empty()) {
    return true;
  } else {
    ADD_FAILURE() << "Failed on field: " << error_field;
    return false;
  }
}

// Returns the number of iterations needed to hit `num_cases`, with the
// probability of hitting a case given as `hit_probability`, so that the
// probability of failure is upper-bounded by `10^(-confidence_level)`.
inline int IterationsToHitAll(int num_cases, double hit_probability,
                              int confidence_level = 15) {
  return static_cast<int>(
      -(confidence_level * std::log(10) + std::log(num_cases)) /
      std::log(1.0 - hit_probability));
}

}  // namespace fuzztest

#endif  // FUZZTEST_INTERNAL_DOMAIN_TESTING_H_
