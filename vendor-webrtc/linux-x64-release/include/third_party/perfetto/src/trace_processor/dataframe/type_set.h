/*
 * Copyright (C) 2025 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef SRC_TRACE_PROCESSOR_DATAFRAME_TYPE_SET_H_
#define SRC_TRACE_PROCESSOR_DATAFRAME_TYPE_SET_H_

#include <array>
#include <cstddef>
#include <cstdint>
#include <optional>
#include <string>
#include <tuple>
#include <type_traits>
#include <variant>

#include "perfetto/base/logging.h"

namespace perfetto::trace_processor::dataframe {

// TypeSet: memory-efficient type hierarchy system with explicit ownership.
//
// The TypeSet template allows for representing hierarchical relationships
// between sets of types, supporting implicit upcasting and safe downcasting
// while using minimal memory (4 bytes per instance). It's designed for use with
// empty struct types as type tags.
//
// Think of it like std::variant but optimized for working with type tags
// instead of values.
//
// Example:
//   struct A {};
//   struct B {};
//   struct C {};
//
//   using ABCSet = TypeSet<A, B, C>;  // Can hold A, B, or C
//   using ABSet = TypeSet<A, B>;      // Can hold A or B
//
//   // Create a typeset containing type A
//   ABSet ab(A{});
//
//   // Check which type it contains
//   if (ab.Is<A>()) { /* ... */ }
//
//   // Implicit upcast to a more general set (ABSet -> ABCSet)
//   ABCSet abc = ab.Upcast<ABCSet>();
//
//   // Safe downcast with runtime checking
//   std::optional<ABSet> maybe_ab = abc.TryDowncast<A, B>();
//   if (maybe_ab) { /* downcast succeeded */ }
//
// Limitations:
// - All types in a TypeSet must be unique
// - Designed for use with empty structs as type tags
template <typename... Ts>
class TypeSet {
 public:
  template <std::size_t Index>
  using GetTypeAtIndex =
      typename std::tuple_element_t<Index, std::tuple<Ts...>>;

  static constexpr std::size_t kSize = sizeof...(Ts);

  // Checks if a type is contained in this TypeSet
  template <typename T>
  static constexpr bool Contains() {
    return (std::is_same_v<T, Ts> || ...);
  }

  TypeSet() = delete;

  template <typename T, typename = std::enable_if_t<Contains<T>()>>
  TypeSet(T) : type_idx_(GetTypeIndex<T>()) {}

  uint32_t index() const { return type_idx_; }

  template <typename T>
  constexpr bool Is() const {
    return type_idx_ == GetTypeIndex<T>();
  }

  // Checks if the current type is contained in another TypeSet.
  //
  // Example:
  //   TypeSet<A, B, C> abc(A{});
  //   abc.IsAnyOf<TypeSet<A, B>>();    // true
  //   abc.IsAnyOf<TypeSet<A, B, C>>(); // true
  //   abc.IsAnyOf<TypeSet<C>>();       // false
  template <typename OtherTypeSet>
  bool IsAnyOf() const {
    // Create a lookup table at compile time of which types are in OtherTypeSet
    constexpr std::array kTypeInOtherSet = {
        OtherTypeSet::template Contains<Ts>()...,
    };
    // O(1) lookup at runtime using our current type index
    return kTypeInOtherSet[type_idx_];
  }

  // Implicitly converts to a TypeSet that includes all current types.
  template <typename OtherTypeSet>
  OtherTypeSet Upcast() const {
    return UpcastImpl(static_cast<OtherTypeSet*>(nullptr));
  }

  // Attempts to convert to a more specific TypeSet.
  template <typename TargetTypeSet>
  std::optional<TargetTypeSet> TryDowncast() const {
    if (uint32_t t = GetMappedIndex<TargetTypeSet>();
        t != static_cast<uint32_t>(-1)) {
      return TargetTypeSet(t);
    }
    return std::nullopt;
  }

  // Gets the index of type T in the TypeSet.
  template <typename T>
  static constexpr uint32_t GetTypeIndex() {
    static_assert(Contains<T>(), "Provided type not allowed in this TypeSet");
    uint32_t idx = GetTypeIndexUnchecked<T>();
    PERFETTO_CHECK(idx != static_cast<uint32_t>(-1));
    return idx;
  }

  // Equality operator for TypeSet.
  constexpr bool operator==(const TypeSet<Ts...>& other) const {
    return type_idx_ == other.type_idx_;
  }

  // Returns a string representation of the TypeSet.
  std::string ToString() const {
    std::string result = "TypeSet";
    result += "[" + std::to_string(type_idx_) + "]";
    return result;
  }

  // Gets the type at the same index in a variant as T is in this TypeSet.
  //
  // Useful if you have a std::variant shadowing the schema of the TypeSet and
  // you want to figure out what the type of the variant is matching the type
  // in the TypeSet.
  template <typename T, typename VariantType>
  struct VariantAtTypeIndexHelper {
    static_assert(Contains<T>(), "Provided type not allowed in this TypeSet");
    static_assert(
        std::variant_size_v<VariantType> == sizeof...(Ts),
        "VariantType must have the same number of types as the TypeSet");
    using type =
        typename std::variant_alternative_t<GetTypeIndex<T>(), VariantType>;
  };
  template <typename T, typename VariantType>
  using VariantTypeAtIndex =
      typename VariantAtTypeIndexHelper<T, VariantType>::type;

 private:
  // Private constructor used internally
  explicit TypeSet(uint32_t idx) : type_idx_(idx) {}

  // Common helper to create mapping from source TypeSet indices to target
  // TypeSet indices
  template <typename TargetTypeSet>
  static constexpr auto CreateTypeMapping() {
    // Create mapping array with invalid index as default
    std::array<uint32_t, sizeof...(Ts)> mapping{};

    // Fill with -1 using constexpr loop
    for (size_t i = 0; i < sizeof...(Ts); ++i) {
      mapping[i] = static_cast<uint32_t>(-1);
    }

    // Fill in mappings
    size_t idx = 0;
    ((mapping[idx++] = TargetTypeSet::template GetTypeIndexUnchecked<Ts>()),
     ...);
    return mapping;
  }

  // Get mapped index in target TypeSet (returns -1 if no mapping exists)
  template <typename TargetTypeSet>
  uint32_t GetMappedIndex() const {
    // Create mapping once at compile time
    constexpr auto mapping = CreateTypeMapping<TargetTypeSet>();

    // Get mapped index with O(1) lookup
    return mapping[type_idx_];
  }

  template <typename T>
  static constexpr uint32_t GetTypeIndexUnchecked() {
    constexpr std::array matches = {std::is_same_v<T, Ts>...};
    for (uint32_t i = 0; i < sizeof...(Ts); ++i) {
      if (matches[i]) {
        return i;
      }
    }
    return static_cast<uint32_t>(-1);
  }

  template <typename... Us>
  constexpr auto UpcastImpl(TypeSet<Us...>*) const {
    // Verify at compile-time that all types can be mapped (upcasting safety
    // check)
    static_assert(
        (TypeSet<Us...>::template Contains<Ts>() && ...),
        "Cannot upcast: target TypeSet does not contain all source types");

    // Get the mapping of our current index to target index
    uint32_t target_idx = GetMappedIndex<TypeSet<Us...>>();

    // This should never fail due to the static_assert, but let's be safe
    PERFETTO_DCHECK(target_idx != static_cast<uint32_t>(-1));
    return TypeSet<Us...>(target_idx);
  }

  // Allow other TypeSet instantiations to access private members
  template <typename...>
  friend class TypeSet;

  // Stores which type is currently active (index into Ts...)
  uint32_t type_idx_;
};

}  // namespace perfetto::trace_processor::dataframe

#endif  // SRC_TRACE_PROCESSOR_DATAFRAME_TYPE_SET_H_
