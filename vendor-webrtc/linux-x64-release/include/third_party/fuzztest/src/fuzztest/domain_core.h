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

// IWYU pragma: private, include "fuzztest/fuzztest.h"
// IWYU pragma: friend fuzztest/.*

#ifndef FUZZTEST_FUZZTEST_DOMAIN_CORE_H_
#define FUZZTEST_FUZZTEST_DOMAIN_CORE_H_

#include <array>
#include <cmath>
#include <cstddef>
#include <deque>
#include <initializer_list>
#include <limits>
#include <list>
#include <map>
#include <memory>
#include <optional>
#include <set>
#include <string>
#include <string_view>
#include <tuple>
#include <type_traits>
#include <unordered_map>
#include <unordered_set>
#include <utility>
#include <variant>
#include <vector>

#include "absl/container/flat_hash_map.h"
#include "absl/random/bit_gen_ref.h"
#include "absl/status/status.h"
#include "absl/strings/string_view.h"
#include "absl/types/span.h"
#include "./fuzztest/internal/any.h"
#include "./fuzztest/internal/domains/aggregate_of_impl.h"
#include "./fuzztest/internal/domains/arbitrary_impl.h"
#include "./fuzztest/internal/domains/bit_flag_combination_of_impl.h"
#include "./fuzztest/internal/domains/container_of_impl.h"
#include "./fuzztest/internal/domains/domain.h"  // IWYU pragma: export
#include "./fuzztest/internal/domains/domain_base.h"  // IWYU pragma: export
#include "./fuzztest/internal/domains/element_of_impl.h"
#include "./fuzztest/internal/domains/filter_impl.h"
#include "./fuzztest/internal/domains/flat_map_impl.h"
#include "./fuzztest/internal/domains/in_range_impl.h"
#include "./fuzztest/internal/domains/map_impl.h"
#include "./fuzztest/internal/domains/one_of_impl.h"
#include "./fuzztest/internal/domains/optional_of_impl.h"
#include "./fuzztest/internal/domains/overlap_of_impl.h"
#include "./fuzztest/internal/domains/smart_pointer_of_impl.h"
#include "./fuzztest/internal/domains/unique_elements_container_of_impl.h"
#include "./fuzztest/internal/domains/variant_of_impl.h"
#include "./fuzztest/internal/logging.h"
#include "./fuzztest/internal/meta.h"
#include "./fuzztest/internal/printer.h"  // IWYU pragma: export
#include "./fuzztest/internal/serialization.h"
#include "./fuzztest/internal/type_support.h"

namespace fuzztest {

class DomainBuilder {
 public:
  DomainBuilder()
      : domain_lookup_table_(std::make_unique<DomainLookUpTable>()) {}

  DomainBuilder(const DomainBuilder&) = delete;
  DomainBuilder& operator=(const DomainBuilder&) = delete;

  // Return a domain referred by `name`. Such a domain doesn't know what type of
  // values it can handle until we call Set on the name.
  template <typename T>
  Domain<T> Get(std::string_view name) {
    FUZZTEST_INTERNAL_CHECK(domain_lookup_table_ != nullptr,
                            "Finalize() has been called!");
    return IndirectDomain<T>(GetIndirect<T>(name));
  }

  template <typename T>
  void Set(std::string_view name, const Domain<T>& domain) {
    FUZZTEST_INTERNAL_CHECK(domain_lookup_table_ != nullptr,
                            "Finalize() has been called!");
    auto* indirect = GetIndirect<T>(name);
    FUZZTEST_INTERNAL_CHECK(!indirect->has_value(),
                            "Cannot set the same domain twice!");
    *indirect = internal::MoveOnlyAny(std::in_place_type<Domain<T>>, domain);
  }

  // Return the top level domain that is used for generating and mutating
  // values. This is also the only domain that a user should actually use. We
  // should set every named domain in the builder before we call Finalize. And
  // after calling it, the domain builder is invalidated and cannot be used
  // anymore.
  template <typename T>
  Domain<T> Finalize(std::string_view name) && {
    FUZZTEST_INTERNAL_CHECK(domain_lookup_table_ != nullptr,
                            "Finalize() has been called!");
    FUZZTEST_INTERNAL_CHECK(
        GetIndirect<T>(name)->has_value(),
        // FUZZTEST_INTERNAL_CHECK uses absl::StrCat, which does not accept
        // std::string_view in iOS builds, so convert to absl::string_view.
        "Finalize() has been called with an unknown name: ",
        absl::string_view{name.data(), name.size()});
    for (auto& iter : *domain_lookup_table_) {
      FUZZTEST_INTERNAL_CHECK(
          iter.second != nullptr && iter.second->has_value(),
          "Some domain is not set yet!");
    }
    auto domain = GetIndirect<T>(name)->template GetAs<Domain<T>>();
    return OwningDomain<T>(std::move(domain), std::move(domain_lookup_table_));
  }

 private:
  // We don't need copyability of the inner domains here.
  // We use shared_ptr to hold the whole table together.
  // The domains point into each other and into themselves recursively. We must
  // keep them with pointer stability.
  using DomainLookUpTable =
      absl::flat_hash_map<std::string, std::unique_ptr<internal::MoveOnlyAny>>;

  // Return the raw pointer of the indirections.
  template <typename T>
  auto GetIndirect(std::string_view name) {
    // Cast `name` to absl::string_view for platforms where this type is not
    // the same as std::string_view.
    auto& indirection =
        (*domain_lookup_table_)[absl::string_view{name.data(), name.size()}];
    if (!indirection) {
      indirection = std::make_unique<internal::MoveOnlyAny>();
    }
    FUZZTEST_INTERNAL_CHECK(
        !indirection->has_value() || indirection->Has<Domain<T>>(),
        "The indirection must either be empty or hold a value of Domain<T>");
    return indirection.get();
  }

  // Domains that uses a layer of indirection. This allows us to create domains
  // for recursive data structures.
  template <typename T>
  class IndirectDomain
      : public domain_implementor::DomainBase<
            IndirectDomain<T>, internal::value_type_t<Domain<T>>,
            internal::corpus_type_t<Domain<T>>> {
   public:
    using typename IndirectDomain::DomainBase::corpus_type;
    using typename IndirectDomain::DomainBase::value_type;

    explicit IndirectDomain(internal::MoveOnlyAny* indirect)
        : indirect_inner_(indirect) {}

    corpus_type Init(absl::BitGenRef prng) {
      return GetInnerDomain().Init(prng);
    }

    void Mutate(corpus_type& val, absl::BitGenRef prng,
                const domain_implementor::MutationMetadata& metadata,
                bool only_shrink) {
      GetInnerDomain().Mutate(val, prng, metadata, only_shrink);
    }

    void UpdateMemoryDictionary(
        const corpus_type& val,
        domain_implementor::ConstCmpTablesPtr cmp_tables) {
      return GetInnerDomain().UpdateMemoryDictionary(val, cmp_tables);
    }

    auto GetPrinter() const { return GetInnerDomain().GetPrinter(); }

    value_type GetValue(const corpus_type& v) const {
      return GetInnerDomain().GetValue(v);
    }

    std::optional<corpus_type> FromValue(const value_type& v) const {
      return GetInnerDomain().FromValue(v);
    }

    std::optional<corpus_type> ParseCorpus(
        const internal::IRObject& obj) const {
      return GetInnerDomain().ParseCorpus(obj);
    }

    internal::IRObject SerializeCorpus(const corpus_type& v) const {
      return GetInnerDomain().SerializeCorpus(v);
    }

    absl::Status ValidateCorpusValue(const corpus_type& corpus_value) const {
      return GetInnerDomain().ValidateCorpusValue(corpus_value);
    }

   private:
    Domain<T>& GetInnerDomain() const {
      return indirect_inner_->GetAs<Domain<T>>();
    }
    internal::MoveOnlyAny* indirect_inner_;
  };

  // Same as Domain<T>, but also holds ownership of the lookup table.
  // This is for toplevel domains.
  template <typename T>
  class OwningDomain : public domain_implementor::DomainBase<
                           OwningDomain<T>, internal::value_type_t<Domain<T>>,
                           internal::corpus_type_t<Domain<T>>> {
   public:
    using typename OwningDomain::DomainBase::corpus_type;
    using typename OwningDomain::DomainBase::value_type;

    OwningDomain(const Domain<T>& inner,
                 std::unique_ptr<DomainLookUpTable> domain_lookup_table)
        : inner_(inner), domain_lookup_table_(std::move(domain_lookup_table)) {}

    corpus_type Init(absl::BitGenRef prng) { return inner_.Init(prng); }

    void Mutate(corpus_type& val, absl::BitGenRef prng,
                const domain_implementor::MutationMetadata& metadata,
                bool only_shrink) {
      inner_.Mutate(val, prng, metadata, only_shrink);
    }

    void UpdateMemoryDictionary(
        const corpus_type& val,
        domain_implementor::ConstCmpTablesPtr cmp_tables) {
      return inner_.UpdateMemoryDictionary(val, cmp_tables);
    }

    auto GetPrinter() const { return inner_.GetPrinter(); }

    value_type GetValue(const corpus_type& v) const {
      return inner_.GetValue(v);
    }

    std::optional<corpus_type> FromValue(const value_type& v) const {
      return inner_.FromValue(v);
    }

    std::optional<corpus_type> ParseCorpus(
        const internal::IRObject& obj) const {
      return inner_.ParseCorpus(obj);
    }

    internal::IRObject SerializeCorpus(const corpus_type& v) const {
      return inner_.SerializeCorpus(v);
    }

    absl::Status ValidateCorpusValue(const corpus_type& corpus_value) const {
      return inner_.ValidateCorpusValue(corpus_value);
    }

   private:
    Domain<T> inner_;
    // Domains are copy constructible, so we need shared access to this table.
    std::shared_ptr<const DomainLookUpTable> domain_lookup_table_;
  };

  std::unique_ptr<DomainLookUpTable> domain_lookup_table_;
};

// This namespace is here only as a way to disable ADL (argument-dependent
// lookup). Names should be used from the fuzztest:: namespace.
namespace internal_no_adl {

// Arbitrary<T>() represents any value of type T.
//
// Example usage:
//
//   Arbitrary<int>()  // Any `int` value.
//
// The Arbitrary<T>() domain is implemented for all native C++ types and for
// protocol buffers. E.g.,:
//
//   Arbitrary<absl::flat_hash_map<uint32_t, MyProtoMessage>>()
//
template <typename T>
auto Arbitrary() {
  return internal::ArbitraryImpl<T>{};
}

// ElementOf(values) represents the input domain composed of the explicitly
// listed `values`.
//
// Example usage:
//
//   ElementOf({0xDEADBEEF, 0xBADDCAFE, 0xFEEDFACE})
//
template <typename T>
auto ElementOf(std::initializer_list<T> values) {
  return internal::ElementOfImpl<T>(values);
}

template <typename T>
auto ElementOf(std::vector<T> values) {
  return internal::ElementOfImpl<T>(std::move(values));
}

template <typename T, std::size_t N>
auto ElementOf(std::array<T, N> values) {
  return internal::ElementOfImpl<T>(
      std::vector<T>(values.begin(), values.end()));
}

template <typename T>
auto Just(T val) {
  return internal::ElementOfImpl<T>({std::move(val)});
}

template <int&... ExplicitArgumentBarrier, typename... Inner>
auto OneOf(Inner... domains) {
  return internal::OneOfImpl<Inner...>(std::move(domains)...);
}

// TODO(xinhaoyuan): Documentation.
template <int&... ExplicitArgumentBarrier, typename... Inner>
auto OverlapOf(Inner... domains) {
  auto MaybeWrapDomain =
      [](auto domain) -> Domain<internal::value_type_t<decltype(domain)>> {
    return domain;
  };
  return internal::OverlapOfImpl<Domain<internal::value_type_t<Inner>>...>(
      MaybeWrapDomain(std::move(domains))...);
}

// Filter(predicate, inner) combinator creates a domain that filters out values
// with `predicate` from an `inner` domain.
//
// IMPORTANT: Use this only to filter out a small subset of the original domain,
// otherwise fuzzing won't be effective.
//
// Example usage:
//
//   Filter([](int x) { return x != kSentinel; }, Arbitrary<int>())
//
template <int&... ExplicitArgumentBarrier, typename Inner, typename Pred>
auto Filter(Pred predicate, Inner inner) {
  return internal::FilterImpl<internal::value_type_t<Inner>>(
      std::move(predicate), std::move(inner));
}

// InRange(min, max) represents any value between [min, max], closed interval.
// Supports integral and floating point types.
//
// Example usage:
//
//   InRange(0.0, 1.0)  // Any probability value.
//
template <typename T>
auto InRange(T min, T max) {
  return internal::InRangeImpl<T>(min, max);
}

// Finite() represents any floating point number, that is not infinite or NaN.
//
// Example usage:
//
//   Finite<double>()  // Any finite double.
//
template <typename T>
auto Finite() {
  static_assert(std::is_floating_point_v<T>,
                "Finite<T>() can only be used with floating point types!");
  return Filter([](T f) { return std::isfinite(f); }, Arbitrary<T>());
}

// Positive() represents any number greater than zero.
//
// Example usage:
//
//   Positive<float>()  // Any positive float value.
//
template <typename T>
auto Positive() {
  if constexpr (std::is_floating_point_v<T>) {
    return InRange<T>(std::numeric_limits<T>::denorm_min(),
                      std::numeric_limits<T>::max());
  } else {
    return InRange<T>(T{1}, std::numeric_limits<T>::max());
  }
}

// NonNegative() represents any number not smaller than zero.
//
// Example usage:
//
//   NonNegative<float>()
//
template <typename T>
auto NonNegative() {
  return InRange<T>(T{}, std::numeric_limits<T>::max());
}

// Negative() represents any number less than zero.
//
// Example usage:
//
//   Negative<float>()  // Any negative float value.
//
template <typename T>
auto Negative() {
  static_assert(!std::is_unsigned_v<T>,
                "Negative<T>() can only be used with signed T-s! For char, "
                "consider using signed char.");
  if constexpr (std::is_floating_point_v<T>) {
    return InRange<T>(std::numeric_limits<T>::lowest(),
                      -std::numeric_limits<T>::denorm_min());
  } else {
    return InRange<T>(std::numeric_limits<T>::min(), T{-1});
  }
}

// NonPositive() represents any number not greater than zero.
//
// Example usage:
//
//   NonPositive<float>()
//
template <typename T>
auto NonPositive() {
  static_assert(!std::is_unsigned_v<T>,
                "NonPositive<T>() can only be used with signed T-s! For char, "
                "consider using signed char.");
  return InRange<T>(std::numeric_limits<T>::lowest(), T{});
}

// NonZero() represents any number except zero.
//
// Example usage:
//
//   NonZero<float>()        // Any non-zero float value.
//
// If the type is unsigned, then the domain represents positive values. For
// example:
//
//   NonZero<unsigned int>() // Any positive int value.
//
template <typename T>
auto NonZero() {
  if constexpr (std::is_signed_v<T>) {
    return OneOf(Negative<T>(), Positive<T>());
  } else {
    return Positive<T>();
  }
}

// BitFlagCombinationOf(flags) represents any combination of binary `flags` via
// bitwise operations.
//
// Example usage:
//
//   enum Options {
//     kFirst  = 1 << 0,
//     kSecond = 1 << 1,
//     kThird  = 1 << 2,
//   };
//
//   BitFlagCombinationOf({kFirst, kThird})
//
// will includes {0, kFirst, kThird,  kFirst | kThird}.
template <typename T>
auto BitFlagCombinationOf(std::initializer_list<T> flags) {
  return internal::BitFlagCombinationOfImpl<T>(
      absl::MakeSpan(flags.begin(), flags.end()));
}

template <typename T>
auto BitFlagCombinationOf(const std::vector<T>& flags) {
  return internal::BitFlagCombinationOfImpl<T>(absl::MakeSpan(flags));
}

// ContainerOf<T>(inner) combinator creates a domain for a container T (eg, a
// std::vector, std::set, etc) where elements are created from `inner`.
//
// Example usage:
//
//   ContainerOf<std::vector<int>>(InRange(1, 2021))
//
// The domain also supports customizing the minimum and maximum size via the
// `WithSize`, `WithMinSize` and `WithMaxSize` functions. Eg:
//
//   ContainerOf<std::vector<int>>(Arbitrary<int>()).WithMaxSize(5)
//
template <typename T, int&... ExplicitArgumentBarrier, typename Inner>
auto ContainerOf(Inner inner) {
  static_assert(
      std::is_same_v<internal::DropConst<typename T::value_type>,
                     internal::DropConst<internal::value_type_t<Inner>>>);
  return internal::ContainerOfImpl<T, Inner>(std::move(inner));
}

// If the container type `T` is a class template whose first template parameter
// is the type of values stored in the container, and whose other template
// parameters, if any, are optional, the template parameters of `T` may be
// omitted, in which case `ContainerOf` will use the `value_type` of the
// `elements_domain` as the first template parameter for T. For example:
//
//   ContainerOf<std::vector>(Positive<int>()).WithSize(3);
//
template <template <typename, typename...> class T,
          int&... ExplicitArgumentBarrier, typename Inner,
          typename C = T<internal::value_type_t<Inner>>>
auto ContainerOf(Inner inner) {
  static_assert(
      std::is_same_v<internal::DropConst<typename C::value_type>,
                     internal::DropConst<internal::value_type_t<Inner>>>);
  return internal::ContainerOfImpl<C, Inner>(std::move(inner));
}

// ASCII character domains.
inline auto NonZeroChar() { return Positive<char>(); }
inline auto AsciiChar() { return InRange<char>(0, 127); }
inline auto PrintableAsciiChar() { return InRange<char>(32, 126); }
inline auto NumericChar() { return InRange<char>('0', '9'); }
inline auto LowerChar() { return InRange<char>('a', 'z'); }
inline auto UpperChar() { return InRange<char>('A', 'Z'); }
inline auto AlphaChar() { return OneOf(LowerChar(), UpperChar()); }
inline auto AlphaNumericChar() { return OneOf(AlphaChar(), NumericChar()); }

// String() is shorthand for Arbitrary<std::string>().
inline auto String() { return Arbitrary<std::string>(); }

// StringOf(inner) combinator creates a `std::string` domain with characters of
// the `inner` domain.
//
// Example usage:
//
//   StringOf(AsciiChar())
//
template <int&... ExplicitArgumentBarrier, typename Inner>
inline auto StringOf(Inner inner) {
  return ContainerOf<std::string>(std::move(inner));
}

// AsciiString() represents `std::string`-s composed of ASCII characters.
inline auto AsciiString() { return StringOf(AsciiChar()); }

// PrintableAsciiString() represents `std::string`-s composed of printable ASCII
// characters.
inline auto PrintableAsciiString() { return StringOf(PrintableAsciiChar()); }

// StructOf<T>(inner...) combinator creates a user-defined type `T` domain with
// fields of the `inner...` domains.
//
// Example usage:
//
//   struct Thing {
//     int id;
//     std::string name;
//   };
//
//   StructOf<MyType>(InRange(0, 10), Arbitrary<std::string>())
//
template <typename T, int&... ExplicitArgumentBarrier, typename... Inner>
auto StructOf(Inner... inner) {
  return internal::AggregateOfImpl<T, internal::RequireCustomCorpusType::kYes,
                                   Inner...>(std::in_place,
                                             std::move(inner)...);
}

// PairOf(inner1, inner2) combinator creates a `std::pair` domain where the
// first element is of `inner1` domain, and the second element is of `inner2`
// domain.
//
// Example usage:
//
//   PairOf(InRange(0, 10), Arbitrary<std::string>())
//
template <int&... ExplicitArgumentBarrier, typename Inner1, typename Inner2>
auto PairOf(Inner1 inner1, Inner2 inner2) {
  return internal::AggregateOfImpl<
      std::pair<internal::value_type_t<Inner1>, internal::value_type_t<Inner2>>,
      internal::RequireCustomCorpusType::kNo, Inner1, Inner2>(
      std::in_place, std::move(inner1), std::move(inner2));
}

// TupleOf(inner...) combinator creates a `std::tuple` domain with elements of
// `inner...` domains.
//
// Example usage:
//
//   TupleOf(InRange(0, 10), Arbitrary<std::string>())
//
template <int&... ExplicitArgumentBarrier, typename... Inner>
auto TupleOf(Inner... inner) {
  return internal::AggregateOfImpl<std::tuple<internal::value_type_t<Inner>...>,
                                   internal::RequireCustomCorpusType::kNo,
                                   Inner...>(std::in_place,
                                             std::move(inner)...);
}

// VariantOf(inner...) combinator creates a `std::variant` domain with elements
// of `inner...` domains.
//
// Example usage:
//
//   VariantOf(InRange(0, 10), Arbitrary<std::string>())
//
// VariantOf<T>(inner...) allows specifying a custom variant type `T`.
//
// Example usage:
//
//   VariantOf<absl::variant<int,std::string>>(InRange(0, 10),
//                                             Arbitrary<std::string>())
//
// `T` can be an instantiation of `std::variant` or any other class that matches
// its API. E.g., `absl::variant`.
template <typename T, int&... ExplicitArgumentBarrier, typename... Inner>
auto VariantOf(Inner... inner) {
  return internal::VariantOfImpl<T, Inner...>(std::in_place,
                                              std::move(inner)...);
}

template <int&... ExplicitArgumentBarrier, typename... Inner>
auto VariantOf(Inner... inner) {
  return VariantOf<std::variant<internal::value_type_t<Inner>...>>(
      std::move(inner)...);
}

// OptionalOf(inner) combinator creates a `std::optional` domain with the
// underlying value of the `inner` domain.
//
// Example usage:
//
//   OptionalOf(InRange(0, 10))
//
//
// OptionalOf<OptionalT>(inner) allows specifying a custom optional type
// `OptionalT`.
//
// Example usage:
//
//   OptionalOf<absl::optional<int>>(InRange(0, 10))
//
// `OptionalT` can be an instantiation of `std::optional` or any other class
// that matches its API. E.g., `absl::optional`.

template <typename OptionalT, int&... ExplicitArgumentBarrier, typename Inner>
auto OptionalOf(Inner inner) {
  return internal::OptionalOfImpl<OptionalT>(std::move(inner));
}

template <int&... ExplicitArgumentBarrier, typename Inner>
auto OptionalOf(Inner inner) {
  return OptionalOf<std::optional<internal::value_type_t<Inner>>>(
      std::move(inner));
}

// NullOpt<T>() creates an optional<T> domain with a single value std::nullopt.
template <typename T>
auto NullOpt() {
  return OptionalOf(internal::ArbitraryImpl<T>()).SetAlwaysNull();
}

// NonNull(inner) excludes `std::nullopt` from `inner` which needs to be
// an optional domain.
//
// Example usage:
//
//   NonNull(OptionalOf(InRange(0, 10)))
//
template <int&... ExplicitArgumentBarrier, typename Inner>
auto NonNull(Inner inner) {
  return inner.SetWithoutNull();
}

// SmartPointerOf<Ptr>(inner) combinator creates a domain for a smart pointer
// type `Ptr` to the object of `inner` domain.
//
// Example usage:
//
//   SmartPointerOf<std::unique_ptr<int>>(InRange(0, 10))
//
// The inner object will be created via `new` through the `inner` domain.
//
// Compatible with std::unique_ptr, std::shared_ptr, and other smart pointers of
// similar API. For std::unique_ptr and std::shared_ptr, use UniquePtrOf() and
// SharedPtrOf() instead.
template <typename Ptr, int&... ExplicitArgumentBarrier, typename Inner>
auto SmartPointerOf(Inner inner) {
  return internal::SmartPointerOfImpl<Ptr>(std::move(inner));
}

// UniquePtrOf(inner) combinator creates a `std::unique_ptr` domain with the
// pointed object of the `inner` domain.
//
// Example usage:
//
//   UniquePtrOf(InRange(0, 10))
//
template <int&... ExplicitArgumentBarrier, typename Inner>
auto UniquePtrOf(Inner inner) {
  return SmartPointerOf<std::unique_ptr<internal::value_type_t<Inner>>>(
      std::move(inner));
}

// SharedPtrOf(inner) combinator creates a `std::shared_ptr` domain with the
// pointed object of the `inner` domain.
//
// Example usage:
//
//   SharedPtrOf(InRange(0, 10))
//
template <int&... ExplicitArgumentBarrier, typename Inner>
auto SharedPtrOf(Inner inner) {
  return SmartPointerOf<std::shared_ptr<internal::value_type_t<Inner>>>(
      std::move(inner));
}

// Map(mapper, inner...) combinator creates a domain that uses the `mapper`
// function to map the values created by the `inner...` domains.
// Example usage:
//
//   Map([](int i) { return 2 * i; }, Arbitrary<int>())
//
// Note: `Map` doesn't support seeds. See `ReversibleMap` if you needs seeds.
template <int&... ExplicitArgumentBarrier, typename Mapper, typename... Inner>
auto Map(Mapper mapper, Inner... inner) {
  return internal::MapImpl<Mapper, Inner...>(std::move(mapper),
                                             std::move(inner)...);
}

// ReversibleMap(mapper, inv_mapper, inner...) combinator creates a domain that
// uses the `mapper` function to map the values created by the `inner...`
// domains, and it uses the `inv_mapper` function to map the domain values back
// into the `inner...` domains. This enables seed support via the `.WithSeeds()`
// methods.
//
// The return type of `inv_mapper` should be `std::optional<std::tuple<T...>>`,
// where `T...` are the input types of `mapper`. Note that `std::tuple` is
// necessary even if `mapper` has a single parameter.
//
// Example:
//
//   ReversibleMap(
//       [](double real, double imag) {
//         return std::complex<double>{real, imag};
//       },
//       [](std::complex<double> z) {
//         return std::optional{std::tuple{z.real(), z.imag()}};
//       },
//       Arbitrary<double>(), Arbitrary<double>())
//
// The function `mapper` doesn't necessarily need to be one-to-one. If it isn't
// and it maps several inner domain tuples to the same value `y`, then
// `inv_mapper` can map `y` back to any of these inner domain tuples.
//
// Example:
//
//   ReversibleMap([](int a, int b) { return std::max(a, b); },
//                 [](int c) {
//                   return std::optional{std::tuple{c, c}};
//                 },
//                 Arbitrary<int>(), Arbitrary<int>())
//
// Importantly, if `inv_mapper` maps `y` to `std::tuple{x...}` and all values
// `x...` are in the respective inner domains, then `mapper` must map `x...` to
// `y`. By slightly abusing the notation, we can write this as
// `mapper(inv_mapper(y)) == y`.
//
// To ensure this property, `inv_mapper` may need to return `std::nullopt`.
//
// Example:
//
//   ReversibleMap(
//       [](int a, int b) {
//         return a > b ? std::pair{a, b} : std::pair{b, a};
//       },
//       [](std::pair<int, int> p) {
//         auto [a, b] = p;
//         return a >= b ? std::optional{std::tuple{a, b}} : std::nullopt;
//       },
//       Arbitrary<int>(), Arbitrary<int>())
//
// In this example, `mapper` always returns a pair `(a, b)` such that `a >= b`.
// Thus, when `a < b`, `inv_mapper` must return `std::nullopt` because there is
// no possible value it could return so that
//
//   `mapper(inv_mapper(std::pair{a, b})) == std::pair{a, b}`.
//
template <int&... ExplicitArgumentBarrier, typename Mapper, typename InvMapper,
          typename... Inner>
auto ReversibleMap(Mapper mapper, InvMapper inv_mapper, Inner... inner) {
  return internal::ReversibleMapImpl<Mapper, InvMapper, Inner...>(
      std::move(mapper), std::move(inv_mapper), std::move(inner)...);
}

// `FlatMap(flat_mapper, inner...)` combinator creates a domain that uses the
// `flat_mapper` function to map the values created by the `inner...` domains.
// Unlike `Map`, however, `FlatMap` maps these into a new _domain_, instead of
// into a value. This domain can then be used as an argument to a fuzz test, or
// to another domain. This can be useful for generating values that depend on
// each other. Example usage:
//
//   // Generate domain of two equal-sized strings
//   FlatMap(
//     [](int size) {
//       return PairOf(Arbitrary<std::string>().WithSize(size),
//                     Arbitrary<std::string>().WithSize(size)); },
//     InRange(0, 10));
//
// Note: `FlatMap` doesn't support seeds.
template <int&... ExplicitArgumentBarrier, typename FlatMapper,
          typename... Inner>
auto FlatMap(FlatMapper flat_mapper, Inner... inner) {
  return internal::FlatMapImpl<FlatMapper, Inner...>(std::move(flat_mapper),
                                                     std::move(inner)...);
}

// VectorOf(inner) combinator creates a `std::vector` domain with elements of
// the `inner` domain.
//
// Example usage:
//
//   VectorOf(InRange(1, 2021))  // std::vector of years.
//
template <int&... ExplicitArgumentBarrier, typename Inner>
auto VectorOf(Inner inner) {
  return ContainerOf<std::vector<internal::value_type_t<Inner>>>(
      std::move(inner));
}

// DequeOf(inner) combinator creates a `std::deque` domain with elements of the
// `inner` domain.
//
// Example usage:
//
//   DequeOf(InRange(1, 2021))
//
template <int&... ExplicitArgumentBarrier, typename Inner>
auto DequeOf(Inner inner) {
  return ContainerOf<std::deque<internal::value_type_t<Inner>>>(
      std::move(inner));
}

// ListOf(inner) combinator creates a `std::list` domain with elements of the
// `inner` domain.
//
// Example usage:
//
//   ListOf(InRange(1, 2021))
//
template <int&... ExplicitArgumentBarrier, typename Inner>
auto ListOf(Inner inner) {
  return ContainerOf<std::list<internal::value_type_t<Inner>>>(
      std::move(inner));
}

// SetOf(inner) combinator creates a `std::set` domain with elements of the
// `inner` domain.
//
// Example usage:
//
//   SetOf(InRange(1, 2021))
//
template <int&... ExplicitArgumentBarrier, typename Inner>
auto SetOf(Inner inner) {
  return ContainerOf<std::set<internal::value_type_t<Inner>>>(std::move(inner));
}

// MapOf(key_domain, value_domain) combinator creates a `std::map` domain with
// keys from the `key_domain` domain, and values from the `value_domain` domain.
//
// Example usage:
//
//   MapOf(InRange(1, 100), Arbitrary<std::string>())
//
template <int&... ExplicitArgumentBarrier, typename KeyDomain,
          typename ValueDomain>
auto MapOf(KeyDomain key_domain, ValueDomain value_domain) {
  return ContainerOf<std::map<internal::value_type_t<KeyDomain>,
                              internal::value_type_t<ValueDomain>>>(
      PairOf(std::move(key_domain), std::move(value_domain)));
}

// UnorderedSetOf(inner) combinator creates a `std::unordered_set` domain with
// elements of the `inner` domain.
//
// Example usage:
//
//   UnorderedSetOf(InRange(1, 2021))
//
template <int&... ExplicitArgumentBarrier, typename Inner>
auto UnorderedSetOf(Inner inner) {
  return ContainerOf<std::unordered_set<internal::value_type_t<Inner>>>(
      std::move(inner));
}

// UnorderedMapOf(key_domain, value_domain) combinator creates a
// `std::unordered_map` domain with keys from the `key_domain` domain, and
// values from the `value_domain` domain.
//
// Example usage:
//
//   UnorderedMapOf(InRange(1, 100), Arbitrary<std::string>())
//
template <int&... ExplicitArgumentBarrier, typename KeyDomain,
          typename ValueDomain>
auto UnorderedMapOf(KeyDomain key_domain, ValueDomain value_domain) {
  return ContainerOf<std::unordered_map<internal::value_type_t<KeyDomain>,
                                        internal::value_type_t<ValueDomain>>>(
      PairOf(std::move(key_domain), std::move(value_domain)));
}

// ArrayOf(inner...) combinator creates a `std::array` domain with N elements,
// one for each of the N domains of `inner...`. ArrayOf<N>(inner) is an overload
// for the case where a single domain `inner` generates values for all the `N`
// elements in the `std::array`.
//
// Example usage:
//
//   ArrayOf(InRange(1, 2021), InRange(1, 12))
//
// Generates `std::array<int, 2>` instances, where the values might represent
// year and month.
//
//   ArrayOf<3>(InRange(0.0, 1.0))
//
// Generates `std::array<double, 3>` instances, where the values might represent
// corrdinates in a unit cube.
//
template <int&... ExplicitArgumentBarrier, typename... Inner>
auto ArrayOf(Inner... inner) {
  static_assert(sizeof...(Inner) > 0,
                "ArrayOf can only be used to create a non-empty std::array.");
  // All value_types of inner domains must be the same, though they can have
  // different corpus_types.
  using value_type =
      internal::value_type_t<std::tuple_element_t<0, std::tuple<Inner...>>>;
  static_assert(std::conjunction_v<
                    std::is_same<value_type, internal::value_type_t<Inner>>...>,
                "All domains in a ArrayOf must have the same value_type.");
  return internal::AggregateOfImpl<std::array<value_type, sizeof...(Inner)>,
                                   internal::RequireCustomCorpusType::kNo,
                                   Inner...>(std::in_place,
                                             std::move(inner)...);
}

template <int N, int&... ExplicitArgumentBarrier, typename Inner>
auto ArrayOf(const Inner& inner) {
  static_assert(N > 0,
                "ArrayOf can only be used to create a non-empty std::array.");
  // Use the above specialization, passing (copying) `inner` N times.
  return internal::ApplyIndex<N>(
      [&](auto... I) { return ArrayOf(((void)I, inner)...); });
}

// UniqueElementsContainerOf(inner) combinator is similar to:
//
//   Map([](absl::flat_hash_set<value_type> unique_values) {
//        return T(unique_values.begin(), unique_values.end());
//     }, UnorderedSetOf(inner))
//
// Where `value_type` is the type of values produced by domain `inner`.
// UniqueElementsContainerOf creates an intermediate domain similar to:
//
//   ContainerOf<absl::flat_hash_set>(inner)
//
// That domain produces collections of instances of `value_type`, where each
// value in the collection is unique; this means that `value_type` must meet the
// type constraints necessary to create an instance of:
//
//   absl::flat_hash_set<value_type>
//
// Example usage:
//
//   UniqueElementsContainerOf<std::vector<int>>(InRange(1, 2021))
//
// The domain supports customizing the minimum and maximum size via the same
// functions as other container combinators: `WithSize`, `WithMinSize` and
// `WithMaxSize`. For example:
//
//   UniqueElementsContainerOf<std::vector<int>>(Arbitrary<int>()).WithSize(5)
//
template <typename T, int&... ExplicitArgumentBarrier, typename Inner>
auto UniqueElementsContainerOf(Inner inner) {
  static_assert(
      std::is_same_v<internal::DropConst<typename T::value_type>,
                     internal::DropConst<internal::value_type_t<Inner>>>);
  return internal::UniqueElementsContainerImpl<T, Inner>(std::move(inner));
}

// UniqueElementsVectorOf(inner) combinator is a shorthand for:
//
//   UniqueElementsContainerOf<std::vector<ElementT>>(inner)
//
// Where `ElementT` is the type of value produced by the domain `inner`.
//
// Example usage:
//
//   UniqueElementsVectorOf(InRange(1, 2021))
//
template <typename Inner>
auto UniqueElementsVectorOf(Inner inner) {
  return UniqueElementsContainerOf<std::vector<internal::value_type_t<Inner>>>(
      std::move(inner));
}

// ConstructorOf<T>(inner...) combinator creates a user-defined type `T` domain
// by passing values from the `inner...` domains to T's constructor.
//
// Example usage:
//
//   class Thing {
//    public:
//     Thing(int a, std::string b);
//   };
//
//   ConstructorOf<Thing>(InRange(0, 5), Arbitrary<std::string>())
//
template <typename T, int&... ExplicitArgumentBarrier, typename... Inner>
auto ConstructorOf(Inner... inner) {
  // TODO(sbenzaquen): Consider using a custom impl instead of Map for better
  // diagnostics.
  return internal::NamedMap(
      internal::GetTypeName<T>(),
      [](auto&&... args) { return T(std::forward<decltype(args)>(args)...); },
      std::move(inner)...);
}

// NonEmpty(inner) is shorthand for domain.WithMinSize(1).
//
// To represent any non-empty container one can use NonEmpty(), e.g.,
// NonEmpty(Arbitrary<std::vector<int>>()) or NonEmpty(VectorOf(String())).
//
// Example usage:
//
//   NonEmpty(String())
//
template <int&... ExplicitArgumentBarrier, typename Inner>
auto NonEmpty(Inner inner) {
  return inner.WithMinSize(1);
}

}  // namespace internal_no_adl

// Inject the names from internal_no_adl into fuzztest, without allowing for
// ADL. Note that an `inline` namespace would not have this effect (ie it would
// still allow ADL to trigger).
using namespace internal_no_adl;  // NOLINT

}  // namespace fuzztest

#endif  // FUZZTEST_FUZZTEST_DOMAIN_CORE_H_
