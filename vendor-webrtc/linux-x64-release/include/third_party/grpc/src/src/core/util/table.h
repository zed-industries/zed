// Copyright 2021 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef GRPC_SRC_CORE_UTIL_TABLE_H
#define GRPC_SRC_CORE_UTIL_TABLE_H

#include <grpc/support/port_platform.h>
#include <stddef.h>

#include <initializer_list>
#include <new>
#include <type_traits>
#include <utility>

#include "absl/meta/type_traits.h"
#include "absl/utility/utility.h"
#include "src/core/util/bitset.h"

namespace grpc_core {

// Meta-programming detail types to aid in building up a Table
namespace table_detail {

// A tuple-like type that contains manually constructed elements.
template <typename, typename... Ts>
struct Elements;

template <typename T, typename... Ts>
struct Elements<absl::enable_if_t<!std::is_empty<T>::value>, T, Ts...>
    : Elements<void, Ts...> {
  struct alignas(T) Data {
    unsigned char bytes[sizeof(T)];
  };
  Data x;
  Elements() {}
  T* ptr() { return reinterpret_cast<T*>(x.bytes); }
  const T* ptr() const { return reinterpret_cast<const T*>(x.bytes); }
};
template <typename T, typename... Ts>
struct Elements<absl::enable_if_t<std::is_empty<T>::value>, T, Ts...>
    : Elements<void, Ts...> {
  T* ptr() { return reinterpret_cast<T*>(this); }
  const T* ptr() const { return reinterpret_cast<const T*>(this); }
};
template <>
struct Elements<void> {};

// Element accessor for Elements<>
// Provides a static method f that returns a pointer to the value of element I
// for Elements<Ts...>
template <size_t I, typename... Ts>
struct GetElem;

template <typename T, typename... Ts>
struct GetElem<0, T, Ts...> {
  static T* f(Elements<void, T, Ts...>* e) { return e->ptr(); }
  static const T* f(const Elements<void, T, Ts...>* e) { return e->ptr(); }
};

template <size_t I, typename T, typename... Ts>
struct GetElem<I, T, Ts...> {
  static auto f(Elements<void, T, Ts...>* e)
      -> decltype(GetElem<I - 1, Ts...>::f(e)) {
    return GetElem<I - 1, Ts...>::f(e);
  }
  static auto f(const Elements<void, T, Ts...>* e)
      -> decltype(GetElem<I - 1, Ts...>::f(e)) {
    return GetElem<I - 1, Ts...>::f(e);
  }
};

// CountIncludedStruct is the backing for the CountIncluded function below.
// Sets a member constant N to the number of times Needle is in Haystack.
template <typename Needle, typename... Haystack>
struct CountIncludedStruct;

template <typename Needle, typename Straw, typename... RestOfHaystack>
struct CountIncludedStruct<Needle, Straw, RestOfHaystack...> {
  static constexpr size_t N =
      static_cast<size_t>(std::is_same<Needle, Straw>::value) +
      CountIncludedStruct<Needle, RestOfHaystack...>::N;
};
template <typename Needle>
struct CountIncludedStruct<Needle> {
  static constexpr size_t N = 0;
};
// Returns the number of times Needle is in Haystack.
template <typename Needle, typename... Haystack>
constexpr size_t CountIncluded() {
  return CountIncludedStruct<Needle, Haystack...>::N;
}

// IndexOfStruct is the backing for IndexOf below.
// Set a member constant N to the index of Needle in Haystack.
// Ignored should be void always, and is used for enable_if_t.
template <typename Ignored, typename Needle, typename... Haystack>
struct IndexOfStruct;

template <typename Needle, typename Straw, typename... RestOfHaystack>
struct IndexOfStruct<absl::enable_if_t<std::is_same<Needle, Straw>::value>,
                     Needle, Straw, RestOfHaystack...> {
  // The first element is the one we're looking for. Done.
  static constexpr size_t N = 0;
};
template <typename Needle, typename Straw, typename... RestOfHaystack>
struct IndexOfStruct<absl::enable_if_t<!std::is_same<Needle, Straw>::value>,
                     Needle, Straw, RestOfHaystack...> {
  // The first element is not the one we're looking for, recurse looking at the
  // tail, and sum the number of recursions.
  static constexpr size_t N =
      1 + IndexOfStruct<void, Needle, RestOfHaystack...>::N;
};
// Return the index of Needle in Haystack.
// Guarded by CountIncluded to ensure that the return type is unambiguous.
// If you got here from a compiler error using Table, it's likely that you've
// used the type-based accessor/mutators, but the type you're using is repeated
// more than once in the Table type arguments. Consider either using the indexed
// accessor/mutator variants, or eliminating the ambiguity in type resolution.
template <typename Needle, typename... Haystack>
constexpr absl::enable_if_t<CountIncluded<Needle, Haystack...>() == 1, size_t>
IndexOf() {
  return IndexOfStruct<void, Needle, Haystack...>::N;
}

// TypeIndexStruct is the backing for TypeIndex below.
// Sets member type Type to the type at index I in Ts.
// Implemented as a simple type recursion.
template <size_t I, typename... Ts>
struct TypeIndexStruct;

template <typename T, typename... Ts>
struct TypeIndexStruct<0, T, Ts...> {
  using Type = T;
};
template <size_t I, typename T, typename... Ts>
struct TypeIndexStruct<I, T, Ts...> : TypeIndexStruct<I - 1, Ts...> {};
// TypeIndex is the type at index I in Ts.
template <size_t I, typename... Ts>
using TypeIndex = typename TypeIndexStruct<I, Ts...>::Type;

// Helper to call the destructor of p if p is non-null.
template <typename T>
void DestructIfNotNull(T* p) {
  if (p) p->~T();
}

}  // namespace table_detail

// A Table<Ts> is much like a tuple<optional<Ts>...> - a set of values that are
// optionally present. Table efficiently packs the presence bits for size, and
// provides a slightly more convenient interface.
template <typename... Ts>
class Table {
  // Helper - TypeIndex<I> is the type at index I in Ts
  template <size_t I>
  using TypeIndex = table_detail::TypeIndex<I, Ts...>;

 public:
  // Construct a table with no values set.
  Table() = default;
  // Destruct - forwards to the Destruct member with an integer sequence so we
  // can destruct field-wise.
  ~Table() { Destruct(absl::make_index_sequence<sizeof...(Ts)>()); }

  // Copy another table
  Table(const Table& rhs) {
    // Since we know all fields are clear initially, pass false for or_clear.
    Copy<false>(absl::make_index_sequence<sizeof...(Ts)>(), rhs);
  }

  // Copy another table
  Table& operator=(const Table& rhs) {
    // Since we may not be all clear, pass true for or_clear to have Copy()
    // clear newly emptied fields.
    Copy<true>(absl::make_index_sequence<sizeof...(Ts)>(), rhs);
    return *this;
  }

  // Move from another table
  Table(Table&& rhs) noexcept {
    // Since we know all fields are clear initially, pass false for or_clear.
    Move<false>(absl::make_index_sequence<sizeof...(Ts)>(),
                std::forward<Table>(rhs));
  }

  // Move from another table
  Table& operator=(Table&& rhs) noexcept {
    // Since we may not be all clear, pass true for or_clear to have Move()
    // clear newly emptied fields.
    Move<true>(absl::make_index_sequence<sizeof...(Ts)>(),
               std::forward<Table>(rhs));
    return *this;
  }

  // Check if this table has a value for type T.
  // Only available if there exists only one T in Ts.
  template <typename T>
  bool has() const {
    return has<index_of<T>()>();
  }

  // Check if this table has index I.
  template <size_t I>
  absl::enable_if_t<(I < sizeof...(Ts)), bool> has() const {
    return present_bits_.is_set(I);
  }

  // Return the value for type T, or nullptr if it is un-set.
  // Only available if there exists only one T in Ts.
  template <typename T>
  T* get() {
    return get<index_of<T>()>();
  }

  // Return the value for type T, or nullptr if it is un-set.
  // Only available if there exists only one T in Ts.
  template <typename T>
  const T* get() const {
    return get<index_of<T>()>();
  }

  // Return the value for index I, or nullptr if it is un-set.
  template <size_t I>
  TypeIndex<I>* get() {
    if (has<I>()) return element_ptr<I>();
    return nullptr;
  }

  // Return the value for index I, or nullptr if it is un-set.
  template <size_t I>
  const TypeIndex<I>* get() const {
    if (has<I>()) return element_ptr<I>();
    return nullptr;
  }

  // Return the value for type T, default constructing it if it is un-set.
  template <typename T>
  T* get_or_create() {
    return get_or_create<index_of<T>()>();
  }

  // Return the value for index I, default constructing it if it is un-set.
  template <size_t I>
  TypeIndex<I>* get_or_create() {
    auto* p = element_ptr<I>();
    if (!set_present<I>(true)) {
      new (p) TypeIndex<I>();
    }
    return element_ptr<I>();
  }

  // Set the value for type T - using Args as construction arguments.
  template <typename T, typename... Args>
  T* set(Args&&... args) {
    return set<index_of<T>()>(std::forward<Args>(args)...);
  }

  // Set the value for index I - using Args as construction arguments.
  template <size_t I, typename... Args>
  TypeIndex<I>* set(Args&&... args) {
    auto* p = element_ptr<I>();
    if (set_present<I>(true)) {
      TypeIndex<I> replacement(std::forward<Args>(args)...);
      *p = std::move(replacement);
    } else {
      new (p) TypeIndex<I>(std::forward<Args>(args)...);
    }
    return p;
  }

  template <size_t I>
  TypeIndex<I>* set(TypeIndex<I>&& value) {
    auto* p = element_ptr<I>();
    if (set_present<I>(true)) {
      *p = std::forward<TypeIndex<I>>(value);
    } else {
      new (p) TypeIndex<I>(std::forward<TypeIndex<I>>(value));
    }
    return p;
  }

  // Clear the value for type T, leaving it un-set.
  template <typename T>
  void clear() {
    clear<index_of<T>()>();
  }

  // Clear the value for index I, leaving it un-set.
  template <size_t I>
  void clear() {
    if (set_present<I>(false)) {
      using T = TypeIndex<I>;
      element_ptr<I>()->~T();
    }
  }

  // Iterate through each set field in the table
  template <typename F>
  void ForEach(F f) const {
    ForEachImpl(std::move(f), absl::make_index_sequence<sizeof...(Ts)>());
  }

  // Iterate through each set field in the table if it exists in Vs, in the
  // order of Vs.
  template <typename F, typename... Vs>
  void ForEachIn(F f) const {
    ForEachImpl(std::move(f),
                absl::index_sequence<table_detail::IndexOf<Vs, Ts...>()...>());
  }

  // Iterate through each set field in the table if it exists in Vs, in the
  // order of Vs. For each existing field, call the filter function. If the
  // function returns true, keep the field. Otherwise, remove the field.
  template <typename F, typename... Vs>
  void FilterIn(F f) {
    FilterInImpl(std::move(f),
                 absl::index_sequence<table_detail::IndexOf<Vs, Ts...>()...>());
  }

  // Count the number of set fields in the table
  size_t count() const { return present_bits_.count(); }

  // Check if the table is completely empty
  bool empty() const { return present_bits_.none(); }

  // Clear all elements in the table.
  void ClearAll() { ClearAllImpl(absl::make_index_sequence<sizeof...(Ts)>()); }

 private:
  // Bit field for which elements of the table are set (true) or un-set (false,
  // the default) -- one bit for each type in Ts.
  using PresentBits = BitSet<sizeof...(Ts)>;
  // The tuple-like backing structure for Table.
  using Elements = table_detail::Elements<void, Ts...>;
  // Extractor from Elements
  template <size_t I>
  using GetElem = table_detail::GetElem<I, Ts...>;

  // Given a T, return the unambiguous index of it within Ts.
  template <typename T>
  static constexpr size_t index_of() {
    return table_detail::IndexOf<T, Ts...>();
  }

  // Given an index, return a point to the (maybe uninitialized!) data value at
  // index I.
  template <size_t I>
  TypeIndex<I>* element_ptr() {
    return GetElem<I>::f(&elements_);
  }

  // Given an index, return a point to the (maybe uninitialized!) data value at
  // index I.
  template <size_t I>
  const TypeIndex<I>* element_ptr() const {
    return GetElem<I>::f(&elements_);
  }

  // Set the present bit to value (if true - value is present/set, if false,
  // value is un-set). Returns the old value so that calling code can note
  // transition edges.
  template <size_t I>
  bool set_present(bool value) {
    bool out = present_bits_.is_set(I);
    present_bits_.set(I, value);
    return out;
  }

  // Set the value of index I to the value held in rhs index I if it is set.
  // If it is unset, if or_clear is true, then clear our value, otherwise do
  // nothing.
  template <bool or_clear, size_t I>
  void CopyIf(const Table& rhs) {
    if (auto* p = rhs.get<I>()) {
      set<I>(*p);
    } else if (or_clear) {
      clear<I>();
    }
  }

  // Set the value of index I to the value moved from rhs index I if it was set.
  // If it is unset, if or_clear is true, then clear our value, otherwise do
  // nothing.
  template <bool or_clear, size_t I>
  void MoveIf(Table&& rhs) {
    if (auto* p = rhs.get<I>()) {
      set<I>(std::move(*p));
    } else if (or_clear) {
      clear<I>();
    }
  }

  // Call (*f)(value) if that value is in the table.
  template <size_t I, typename F>
  void CallIf(F* f) const {
    if (auto* p = get<I>()) {
      (*f)(*p);
    }
  }

  // Call (*f)(value) if that value is in the table.
  // If the value is present in the table and (*f)(value) returns false, remove
  // the value from the table.
  template <size_t I, typename F>
  void FilterIf(F* f) {
    if (auto* p = get<I>()) {
      if (!(*f)(*p)) {
        clear<I>();
      }
    }
  }

  // For each field (element I=0, 1, ...) if that field is present, call its
  // destructor.
  template <size_t... I>
  void Destruct(absl::index_sequence<I...>) {
    (table_detail::DestructIfNotNull(get<I>()), ...);
  }

  // For each field (element I=0, 1, ...) copy that field into this table -
  // or_clear as per CopyIf().
  template <bool or_clear, size_t... I>
  void Copy(absl::index_sequence<I...>, const Table& rhs) {
    (CopyIf<or_clear, I>(rhs), ...);
  }

  // For each field (element I=0, 1, ...) move that field into this table -
  // or_clear as per MoveIf().
  template <bool or_clear, size_t... I>
  void Move(absl::index_sequence<I...>, Table&& rhs) {
    (MoveIf<or_clear, I>(std::forward<Table>(rhs)), ...);
  }

  // For each field (element I=0, 1, ...) if that field is present, call f.
  template <typename F, size_t... I>
  void ForEachImpl(F f, absl::index_sequence<I...>) const {
    (CallIf<I>(&f), ...);
  }

  // For each field (element I=0, 1, ...) if that field is present, call f. If
  // f returns false, remove the field from the table.
  template <typename F, size_t... I>
  void FilterInImpl(F f, absl::index_sequence<I...>) {
    (FilterIf<I>(&f), ...);
  }

  template <size_t... I>
  void ClearAllImpl(absl::index_sequence<I...>) {
    (clear<I>(), ...);
  }

  // Bit field indicating which elements are set.
  PresentBits present_bits_;
  // The memory to store the elements themselves.
  Elements elements_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_UTIL_TABLE_H
