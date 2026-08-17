// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TRAITS_BAG_H_
#define BASE_TRAITS_BAG_H_

#include <initializer_list>
#include <optional>
#include <tuple>
#include <type_traits>
#include <utility>

#include "base/parameter_pack.h"

// A bag of Traits (structs / enums / etc...) can be an elegant alternative to
// the builder pattern and multiple default arguments for configuring things.
// Traits are terser than the builder pattern and can be evaluated at compile
// time, however they require the use of variadic templates which complicates
// matters. This file contains helpers that make Traits easier to use.
//
// WARNING: Trait bags are currently too heavy for non-constexpr usage in prod
// code due to template bloat, although adding NOINLINE to template constructors
// configured via trait bags can help.
//
// E.g.
//   struct EnableFeatureX {};
//   struct UnusedTrait {};
//   enum Color { RED, BLUE };
//
//   struct ValidTraits {
//      ValidTraits(EnableFeatureX);
//      ValidTraits(Color);
//   };
//   ...
//   DoSomethingAwesome();                 // Use defaults (Color::BLUE &
//                                         // feature X not enabled)
//   DoSomethingAwesome(EnableFeatureX(),  // Turn feature X on
//                      Color::RED);       // And make it red.
//   DoSomethingAwesome(UnusedTrait(),     // Compile time error.
//                      Color::RED);
//
// DoSomethingAwesome might be defined as:
//
//   template <class... ArgTypes>
//     requires trait_helpers::AreValidTraits<ValidTraits, ArgTypes...>
//   constexpr void DoSomethingAwesome(ArgTypes... args)
//      : enable_feature_x(
//            trait_helpers::HasTrait<EnableFeatureX, ArgTypes...>()),
//        color(trait_helpers::GetEnum<Color, EnumTraitA::BLUE>(args...)) {}

namespace base {
namespace trait_helpers {

// Represents a trait that has been removed by a predicate.
struct EmptyTrait {};

// Predicate used to remove any traits from the given list of types by
// converting them to EmptyTrait. E.g.
//
// template <typename... Args>
// void MyFunc(Args... args) {
//   DoSomethingWithTraits(
//       base::trait_helpers::Exclude<UnwantedTrait1,
//                                    UnwantedTrait2>::Filter(args)...);
// }
//
// NB It's possible to actually remove the unwanted trait from the pack, but
// that requires constructing a filtered tuple and applying it to the function,
// which isn't worth the complexity over ignoring EmptyTrait.
template <typename... TraitsToExclude>
struct Exclude {
  template <typename T>
  static constexpr auto Filter(T t) {
    if constexpr (ParameterPack<TraitsToExclude...>::template HasType<
                      T>::value) {
      return EmptyTrait();
    } else {
      return t;
    }
  }
};

// CallFirstTag is an argument tag that helps to avoid ambiguous overloaded
// functions. When the following call is made:
//    func(CallFirstTag(), arg...);
// the compiler will give precedence to an overload candidate that directly
// takes CallFirstTag. Another overload that takes CallSecondTag will be
// considered iff the preferred overload candidates were all invalids and
// therefore discarded.
struct CallSecondTag {};
struct CallFirstTag : CallSecondTag {};

// A trait filter class |TraitFilterType| implements the protocol to get a value
// of type |ArgType| from an argument list and convert it to a value of type
// |TraitType|. If the argument list contains an argument of type |ArgType|, the
// filter class will be instantiated with that argument. If the argument list
// contains no argument of type |ArgType|, the filter class will be instantiated
// using the default constructor if available; a compile error is issued
// otherwise. The filter class must have the conversion operator TraitType()
// which returns a value of type TraitType.

// |InvalidTrait| is used to return from GetTraitFromArg when the argument is
// not compatible with the desired trait.
struct InvalidTrait {};

// Returns an object of type |TraitFilterType| constructed from |arg| if
// compatible, or |InvalidTrait| otherwise.
template <class TraitFilterType, class ArgType>
  requires std::constructible_from<TraitFilterType, ArgType>
constexpr TraitFilterType GetTraitFromArg(CallFirstTag, ArgType arg) {
  return TraitFilterType(arg);
}

template <class TraitFilterType, class ArgType>
constexpr InvalidTrait GetTraitFromArg(CallSecondTag, ArgType arg) {
  return InvalidTrait();
}

// Returns an object of type |TraitFilterType| constructed from a compatible
// argument in |args...|, or default constructed if none of the arguments are
// compatible. This is the implementation of GetTraitFromArgList() with a
// disambiguation tag.
template <class TraitFilterType, class... ArgTypes>
  requires(std::constructible_from<TraitFilterType, ArgTypes> || ...)
constexpr TraitFilterType GetTraitFromArgListImpl(CallFirstTag,
                                                  ArgTypes... args) {
  return std::get<TraitFilterType>(std::make_tuple(
      GetTraitFromArg<TraitFilterType>(CallFirstTag(), args)...));
}

template <class TraitFilterType, class... ArgTypes>
constexpr TraitFilterType GetTraitFromArgListImpl(CallSecondTag,
                                                  ArgTypes... args) {
  static_assert(std::is_constructible_v<TraitFilterType>,
                "The traits bag is missing a required trait.");
  return TraitFilterType();
}

// Constructs an object of type |TraitFilterType| from a compatible argument in
// |args...|, or using the default constructor, and returns its associated trait
// value using conversion to |TraitFilterType::ValueType|. If there are more
// than one compatible argument in |args|, generates a compile-time error.
template <class TraitFilterType, class... ArgTypes>
constexpr typename TraitFilterType::ValueType GetTraitFromArgList(
    ArgTypes... args) {
  static_assert(
      count({std::is_constructible_v<TraitFilterType, ArgTypes>...}, true) <= 1,
      "The traits bag contains multiple traits of the same type.");
  return GetTraitFromArgListImpl<TraitFilterType>(CallFirstTag(), args...);
}

// Helper class to implemnent a |TraitFilterType|.
template <typename T, typename _ValueType = T>
struct BasicTraitFilter {
  using ValueType = _ValueType;

  constexpr BasicTraitFilter(ValueType v) : value(v) {}

  constexpr operator ValueType() const { return value; }

  ValueType value = {};
};

template <typename ArgType, ArgType DefaultValue>
struct EnumTraitFilter : public BasicTraitFilter<ArgType> {
  constexpr EnumTraitFilter() : BasicTraitFilter<ArgType>(DefaultValue) {}
  constexpr EnumTraitFilter(ArgType arg) : BasicTraitFilter<ArgType>(arg) {}
};

template <typename ArgType>
struct OptionalEnumTraitFilter
    : public BasicTraitFilter<ArgType, std::optional<ArgType>> {
  constexpr OptionalEnumTraitFilter()
      : BasicTraitFilter<ArgType, std::optional<ArgType>>(std::nullopt) {}
  constexpr OptionalEnumTraitFilter(ArgType arg)
      : BasicTraitFilter<ArgType, std::optional<ArgType>>(arg) {}
};

// Tests whether multiple given argtument types are all valid traits according
// to the provided ValidTraits. To use, define a ValidTraits
template <typename ArgType>
struct RequiredEnumTraitFilter : public BasicTraitFilter<ArgType> {
  constexpr RequiredEnumTraitFilter(ArgType arg)
      : BasicTraitFilter<ArgType>(arg) {}
};

// Note EmptyTrait is always regarded as valid to support filtering.
template <class ValidTraits, class T>
concept IsValidTrait =
    std::constructible_from<ValidTraits, T> || std::same_as<T, EmptyTrait>;

// Tests whether a given trait type is valid or invalid by testing whether it is
// convertible to the provided ValidTraits type. To use, define a ValidTraits
// type like this:
//
// struct ValidTraits {
//   ValidTraits(MyTrait);
//   ...
// };
//
// You can 'inherit' valid traits like so:
//
// struct MoreValidTraits {
//   MoreValidTraits(ValidTraits);  // Pull in traits from ValidTraits.
//   MoreValidTraits(MyOtherTrait);
//   ...
// };
template <class ValidTraits, class... ArgTypes>
concept AreValidTraits = (IsValidTrait<ValidTraits, ArgTypes> && ...);

// Helper to make getting an enum from a trait more readable.
template <typename Enum, typename... Args>
static constexpr Enum GetEnum(Args... args) {
  return GetTraitFromArgList<RequiredEnumTraitFilter<Enum>>(args...);
}

// Helper to make getting an enum from a trait with a default more readable.
template <typename Enum, Enum DefaultValue, typename... Args>
static constexpr Enum GetEnum(Args... args) {
  return GetTraitFromArgList<EnumTraitFilter<Enum, DefaultValue>>(args...);
}

// Helper to make getting an optional enum from a trait with a default more
// readable.
template <typename Enum, typename... Args>
static constexpr std::optional<Enum> GetOptionalEnum(Args... args) {
  return GetTraitFromArgList<OptionalEnumTraitFilter<Enum>>(args...);
}

// Helper to make checking for the presence of a trait more readable.
template <typename Trait, typename... Args>
struct HasTrait : ParameterPack<Args...>::template HasType<Trait> {
  static_assert(count({std::is_constructible_v<Trait, Args>...}, true) <= 1,
                "The traits bag contains multiple traits of the same type.");
};

// If you need a template vararg constructor to delegate to a private
// constructor, you may need to add this to the private constructor to ensure
// it's not matched by accident.
struct NotATraitTag {};

}  // namespace trait_helpers
}  // namespace base

#endif  // BASE_TRAITS_BAG_H_
