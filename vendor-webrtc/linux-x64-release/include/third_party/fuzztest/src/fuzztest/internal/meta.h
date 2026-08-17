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

#ifndef FUZZTEST_FUZZTEST_INTERNAL_META_H_
#define FUZZTEST_FUZZTEST_INTERNAL_META_H_

#include <array>
#include <complex>
#include <cstdlib>
#include <memory>
#include <optional>
#include <tuple>
#include <type_traits>
#include <utility>
#include <variant>
#include <vector>

#include "absl/numeric/int128.h"

namespace google::protobuf {
template <typename T>
struct is_proto_enum;
}  // namespace google::protobuf

namespace fuzztest::internal {

template <size_t... I, typename F>
constexpr auto ApplyIndexImpl(F f, std::index_sequence<I...>) {
  return f(std::integral_constant<size_t, I>{}...);
}

// Invoke `f(std::integral_constant<size_t, 0>{}, ...,
//           std::integral_constant<size_t, N - 1>{});
//
// Useful with a generic lambda like:
//   ApplyIndex<N>([&](auto... I) {
//     (std::get<I>(....), ...);
//   }
// This allows users to expand an N without needing their own helper
// function/specialization generally required when working with
// std::index_sequence.
template <size_t N, typename F>
constexpr auto ApplyIndex(F f) {
  return ApplyIndexImpl(f, std::make_index_sequence<N>{});
}

// Calls f(std::integral_constant<size_t, i>{}).
// Requires `0 <= i && i < Max`
// This function raises a runtime int into a static int. Useful for dealing with
// tuples, variants, etc.
template <size_t Max, typename F>
decltype(auto) Switch(size_t i, F f) {
  using Call = decltype(f(std::integral_constant<size_t, 0>())) (*)(F&);
  return ApplyIndex<Max>([](auto... I) {
    static constexpr Call kFuncs[] = {
        [](F& f) { return std::move(f)(decltype(I){}); }...};
    return kFuncs;
  })[i](f);
}

// Type dependent "true"/"false".
// Useful for SFINAE and static_asserts where we need a type dependent
// expression that happens to be constant.
template <typename T>
constexpr std::false_type always_false = {};
template <typename T>
constexpr std::true_type always_true = {};

// "backport" of C++20's `requires` expression.
// It can be used as:
//   if constexpr (Requires<T>([](auto x) -> decltype(x.foo()) {})) {
// with similar behavior as C++20's
//   if constexpr (requires{ t.foo(); }) {
// Much more verbose than `requires`, but still much more compact than the
// SFINAE alternative.
template <typename... T, typename F>
constexpr bool Requires(F) {
  return std::is_invocable_v<F, T...>;
}

// Simple typeid implementation that does not require RTTI.
// Only supports comparisons for equality.
using TypeId = const void*;
template <typename T>
inline constexpr TypeId type_id = &type_id<T>;

// Some simple type traits
template <typename T>
inline constexpr bool is_monostate_v = std::is_class_v<T>&& std::is_empty_v<T>&&
    std::is_default_constructible_v<T>;

template <typename T>
inline constexpr bool is_variant_v = false;

template <typename... T>
inline constexpr bool is_variant_v<std::variant<T...>> = true;

template <typename T>
inline constexpr bool is_pair_v = false;

template <typename T, typename U>
inline constexpr bool is_pair_v<std::pair<T, U>> = true;

template <typename... T>
inline constexpr bool is_tuple_v = false;

template <typename... T>
inline constexpr bool is_tuple_v<std::tuple<T...>> = true;

template <typename T>
inline constexpr bool is_array_v = false;

template <typename T, size_t N>
inline constexpr bool is_array_v<std::array<T, N>> = true;

template <typename T>
inline constexpr bool is_bitvector_v = false;

template <>
inline constexpr bool is_bitvector_v<std::vector<bool>> = true;

template <typename T>
inline constexpr bool is_vector_v = false;

template <typename T>
inline constexpr bool is_vector_v<std::vector<T>> = true;

template <typename T>
inline constexpr bool is_unique_ptr_v = false;

template <typename T>
inline constexpr bool is_unique_ptr_v<std::unique_ptr<T>> = true;

template <typename T>
inline constexpr bool is_shared_ptr_v = false;

template <typename T>
inline constexpr bool is_shared_ptr_v<std::shared_ptr<T>> = true;

template <typename T>
inline constexpr bool is_std_complex_v = false;

template <typename T>
inline constexpr bool is_std_complex_v<std::complex<T>> = true;

template <typename T>
using MakeUnsignedT = typename std::conditional_t<
    std::is_same_v<T, absl::int128> || std::is_same_v<T, absl::uint128>,
    std::enable_if<true, absl::uint128>, std::make_unsigned<T>>::type;

// Protocol buffers are handled through duck typing to avoid a strong dependency
// on the library.
template <typename T>
constexpr bool IsProtocolBufferImpl(typename T::Message*) {
  // Check the basics:
  //  - There is a base class called Message.
  //  - It has GetReflection/GetDescriptor methods.
  //  - They interact as we expect with each other.
  //
  //  We can add more specifics if this is still too ambiguous.
  using Message = typename T::Message;
  return std::is_base_of_v<Message, T> &&
         std::is_convertible_v<const T*, const Message*> &&
         Requires<const Message*>(
             [](auto* f) -> decltype(f->GetReflection()->HasField(
                             *f, f->GetDescriptor()->field(0))) {});
}

template <typename T>
constexpr bool IsProtocolBufferImpl(...) {
  return false;
}

template <typename T>
inline constexpr bool is_protocol_buffer_v = IsProtocolBufferImpl<T>(nullptr);

template <typename T>
constexpr bool IsProtocolBufferEnumImpl(
    std::enable_if_t<google::protobuf::is_proto_enum<T>::value, bool>) {
  return true;
}

template <typename T, typename = void>
constexpr bool IsProtocolBufferEnumImpl(...) {
  return false;
}

template <typename T>
inline constexpr bool is_protocol_buffer_enum_v =
    IsProtocolBufferEnumImpl<T>(true);

template <typename T>
inline constexpr bool has_size_v =
    Requires<T>([](auto v) -> decltype(v.size()) {});

template <typename T>
inline constexpr bool is_dynamic_container_v =
    Requires<T>([](auto v) -> decltype(v.insert(v.end(), *v.begin())) {});

template <typename T>
inline constexpr bool is_associative_container_v = is_dynamic_container_v<T>&&
Requires<T>([](auto v) -> decltype(v.find(
                           std::declval<typename decltype(v)::key_type>())) {});

template <typename T, typename = void>
struct is_memory_dictionary_compatible : std::false_type {};

template <typename T>
struct is_memory_dictionary_compatible<
    T, std::enable_if_t<T::is_memory_dictionary_compatible_v>>
    : std::true_type {};

// `BindAggregate` uses structured bindings to create a tuple of references to
// the fields on the input object.
// It has to be provided with the number of fields contained in the structure.
// We special case `0` because we can't do structured bindings there, and it is
// unnecessary anyway.
template <typename T>
auto BindAggregate(T&, std::integral_constant<int, 0>) {
  return std::tie();
}
// Implement the overloads from 1 to 80.
// There is no "variadic" way of doing this.
#define FUZZTEST_INTERNAL_BIND_AGGREGATE_(n, ...)                \
  template <typename T>                                          \
  auto BindAggregate(T& value, std::integral_constant<int, n>) { \
    auto& [__VA_ARGS__] = value;                                 \
    return std::tie(__VA_ARGS__);                                \
  }

FUZZTEST_INTERNAL_BIND_AGGREGATE_(1, x1)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(2, x1, x2)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(3, x1, x2, x3)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(4, x1, x2, x3, x4)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(5, x1, x2, x3, x4, x5)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(6, x1, x2, x3, x4, x5, x6)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(7, x1, x2, x3, x4, x5, x6, x7)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(8, x1, x2, x3, x4, x5, x6, x7, x8)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(9, x1, x2, x3, x4, x5, x6, x7, x8, x9)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(10, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(11, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(12, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(13, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(14, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(15, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(16, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(17, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(18, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(19, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(20, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(21, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(22, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(23, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(24, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(25, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(26, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(27, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(28, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27, x28)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(29, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27, x28,
                                  x29)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(30, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27, x28,
                                  x29, x30)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(31, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27, x28,
                                  x29, x30, x31)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(32, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27, x28,
                                  x29, x30, x31, x32)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(33, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27, x28,
                                  x29, x30, x31, x32, x33)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(34, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27, x28,
                                  x29, x30, x31, x32, x33, x34)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(35, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27, x28,
                                  x29, x30, x31, x32, x33, x34, x35)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(36, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27, x28,
                                  x29, x30, x31, x32, x33, x34, x35, x36)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(37, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27, x28,
                                  x29, x30, x31, x32, x33, x34, x35, x36, x37)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(38, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27, x28,
                                  x29, x30, x31, x32, x33, x34, x35, x36, x37,
                                  x38)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(39, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27, x28,
                                  x29, x30, x31, x32, x33, x34, x35, x36, x37,
                                  x38, x39)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(40, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27, x28,
                                  x29, x30, x31, x32, x33, x34, x35, x36, x37,
                                  x38, x39, x40)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(41, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27, x28,
                                  x29, x30, x31, x32, x33, x34, x35, x36, x37,
                                  x38, x39, x40, x41)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(42, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27, x28,
                                  x29, x30, x31, x32, x33, x34, x35, x36, x37,
                                  x38, x39, x40, x41, x42)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(43, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27, x28,
                                  x29, x30, x31, x32, x33, x34, x35, x36, x37,
                                  x38, x39, x40, x41, x42, x43)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(44, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27, x28,
                                  x29, x30, x31, x32, x33, x34, x35, x36, x37,
                                  x38, x39, x40, x41, x42, x43, x44)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(45, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27, x28,
                                  x29, x30, x31, x32, x33, x34, x35, x36, x37,
                                  x38, x39, x40, x41, x42, x43, x44, x45)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(46, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27, x28,
                                  x29, x30, x31, x32, x33, x34, x35, x36, x37,
                                  x38, x39, x40, x41, x42, x43, x44, x45, x46)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(47, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27, x28,
                                  x29, x30, x31, x32, x33, x34, x35, x36, x37,
                                  x38, x39, x40, x41, x42, x43, x44, x45, x46,
                                  x47)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(48, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27, x28,
                                  x29, x30, x31, x32, x33, x34, x35, x36, x37,
                                  x38, x39, x40, x41, x42, x43, x44, x45, x46,
                                  x47, x48)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(49, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27, x28,
                                  x29, x30, x31, x32, x33, x34, x35, x36, x37,
                                  x38, x39, x40, x41, x42, x43, x44, x45, x46,
                                  x47, x48, x49)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(50, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27, x28,
                                  x29, x30, x31, x32, x33, x34, x35, x36, x37,
                                  x38, x39, x40, x41, x42, x43, x44, x45, x46,
                                  x47, x48, x49, x50)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(51, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27, x28,
                                  x29, x30, x31, x32, x33, x34, x35, x36, x37,
                                  x38, x39, x40, x41, x42, x43, x44, x45, x46,
                                  x47, x48, x49, x50, x51)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(52, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27, x28,
                                  x29, x30, x31, x32, x33, x34, x35, x36, x37,
                                  x38, x39, x40, x41, x42, x43, x44, x45, x46,
                                  x47, x48, x49, x50, x51, x52)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(53, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27, x28,
                                  x29, x30, x31, x32, x33, x34, x35, x36, x37,
                                  x38, x39, x40, x41, x42, x43, x44, x45, x46,
                                  x47, x48, x49, x50, x51, x52, x53)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(54, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27, x28,
                                  x29, x30, x31, x32, x33, x34, x35, x36, x37,
                                  x38, x39, x40, x41, x42, x43, x44, x45, x46,
                                  x47, x48, x49, x50, x51, x52, x53, x54)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(55, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27, x28,
                                  x29, x30, x31, x32, x33, x34, x35, x36, x37,
                                  x38, x39, x40, x41, x42, x43, x44, x45, x46,
                                  x47, x48, x49, x50, x51, x52, x53, x54, x55)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(56, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27, x28,
                                  x29, x30, x31, x32, x33, x34, x35, x36, x37,
                                  x38, x39, x40, x41, x42, x43, x44, x45, x46,
                                  x47, x48, x49, x50, x51, x52, x53, x54, x55,
                                  x56)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(57, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27, x28,
                                  x29, x30, x31, x32, x33, x34, x35, x36, x37,
                                  x38, x39, x40, x41, x42, x43, x44, x45, x46,
                                  x47, x48, x49, x50, x51, x52, x53, x54, x55,
                                  x56, x57)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(58, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27, x28,
                                  x29, x30, x31, x32, x33, x34, x35, x36, x37,
                                  x38, x39, x40, x41, x42, x43, x44, x45, x46,
                                  x47, x48, x49, x50, x51, x52, x53, x54, x55,
                                  x56, x57, x58)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(59, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27, x28,
                                  x29, x30, x31, x32, x33, x34, x35, x36, x37,
                                  x38, x39, x40, x41, x42, x43, x44, x45, x46,
                                  x47, x48, x49, x50, x51, x52, x53, x54, x55,
                                  x56, x57, x58, x59)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(60, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27, x28,
                                  x29, x30, x31, x32, x33, x34, x35, x36, x37,
                                  x38, x39, x40, x41, x42, x43, x44, x45, x46,
                                  x47, x48, x49, x50, x51, x52, x53, x54, x55,
                                  x56, x57, x58, x59, x60)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(61, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27, x28,
                                  x29, x30, x31, x32, x33, x34, x35, x36, x37,
                                  x38, x39, x40, x41, x42, x43, x44, x45, x46,
                                  x47, x48, x49, x50, x51, x52, x53, x54, x55,
                                  x56, x57, x58, x59, x60, x61)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(62, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27, x28,
                                  x29, x30, x31, x32, x33, x34, x35, x36, x37,
                                  x38, x39, x40, x41, x42, x43, x44, x45, x46,
                                  x47, x48, x49, x50, x51, x52, x53, x54, x55,
                                  x56, x57, x58, x59, x60, x61, x62)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(63, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27, x28,
                                  x29, x30, x31, x32, x33, x34, x35, x36, x37,
                                  x38, x39, x40, x41, x42, x43, x44, x45, x46,
                                  x47, x48, x49, x50, x51, x52, x53, x54, x55,
                                  x56, x57, x58, x59, x60, x61, x62, x63)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(64, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27, x28,
                                  x29, x30, x31, x32, x33, x34, x35, x36, x37,
                                  x38, x39, x40, x41, x42, x43, x44, x45, x46,
                                  x47, x48, x49, x50, x51, x52, x53, x54, x55,
                                  x56, x57, x58, x59, x60, x61, x62, x63, x64)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(65, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27, x28,
                                  x29, x30, x31, x32, x33, x34, x35, x36, x37,
                                  x38, x39, x40, x41, x42, x43, x44, x45, x46,
                                  x47, x48, x49, x50, x51, x52, x53, x54, x55,
                                  x56, x57, x58, x59, x60, x61, x62, x63, x64,
                                  x65)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(66, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27, x28,
                                  x29, x30, x31, x32, x33, x34, x35, x36, x37,
                                  x38, x39, x40, x41, x42, x43, x44, x45, x46,
                                  x47, x48, x49, x50, x51, x52, x53, x54, x55,
                                  x56, x57, x58, x59, x60, x61, x62, x63, x64,
                                  x65, x66)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(67, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27, x28,
                                  x29, x30, x31, x32, x33, x34, x35, x36, x37,
                                  x38, x39, x40, x41, x42, x43, x44, x45, x46,
                                  x47, x48, x49, x50, x51, x52, x53, x54, x55,
                                  x56, x57, x58, x59, x60, x61, x62, x63, x64,
                                  x65, x66, x67)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(68, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27, x28,
                                  x29, x30, x31, x32, x33, x34, x35, x36, x37,
                                  x38, x39, x40, x41, x42, x43, x44, x45, x46,
                                  x47, x48, x49, x50, x51, x52, x53, x54, x55,
                                  x56, x57, x58, x59, x60, x61, x62, x63, x64,
                                  x65, x66, x67, x68)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(69, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27, x28,
                                  x29, x30, x31, x32, x33, x34, x35, x36, x37,
                                  x38, x39, x40, x41, x42, x43, x44, x45, x46,
                                  x47, x48, x49, x50, x51, x52, x53, x54, x55,
                                  x56, x57, x58, x59, x60, x61, x62, x63, x64,
                                  x65, x66, x67, x68, x69)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(70, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27, x28,
                                  x29, x30, x31, x32, x33, x34, x35, x36, x37,
                                  x38, x39, x40, x41, x42, x43, x44, x45, x46,
                                  x47, x48, x49, x50, x51, x52, x53, x54, x55,
                                  x56, x57, x58, x59, x60, x61, x62, x63, x64,
                                  x65, x66, x67, x68, x69, x70)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(71, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27, x28,
                                  x29, x30, x31, x32, x33, x34, x35, x36, x37,
                                  x38, x39, x40, x41, x42, x43, x44, x45, x46,
                                  x47, x48, x49, x50, x51, x52, x53, x54, x55,
                                  x56, x57, x58, x59, x60, x61, x62, x63, x64,
                                  x65, x66, x67, x68, x69, x70, x71)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(72, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27, x28,
                                  x29, x30, x31, x32, x33, x34, x35, x36, x37,
                                  x38, x39, x40, x41, x42, x43, x44, x45, x46,
                                  x47, x48, x49, x50, x51, x52, x53, x54, x55,
                                  x56, x57, x58, x59, x60, x61, x62, x63, x64,
                                  x65, x66, x67, x68, x69, x70, x71, x72)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(73, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27, x28,
                                  x29, x30, x31, x32, x33, x34, x35, x36, x37,
                                  x38, x39, x40, x41, x42, x43, x44, x45, x46,
                                  x47, x48, x49, x50, x51, x52, x53, x54, x55,
                                  x56, x57, x58, x59, x60, x61, x62, x63, x64,
                                  x65, x66, x67, x68, x69, x70, x71, x72, x73)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(
    74, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15, x16,
    x17, x18, x19, x20, x21, x22, x23, x24, x25, x26, x27, x28, x29, x30, x31,
    x32, x33, x34, x35, x36, x37, x38, x39, x40, x41, x42, x43, x44, x45, x46,
    x47, x48, x49, x50, x51, x52, x53, x54, x55, x56, x57, x58, x59, x60, x61,
    x62, x63, x64, x65, x66, x67, x68, x69, x70, x71, x72, x73, x74)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(
    75, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15, x16,
    x17, x18, x19, x20, x21, x22, x23, x24, x25, x26, x27, x28, x29, x30, x31,
    x32, x33, x34, x35, x36, x37, x38, x39, x40, x41, x42, x43, x44, x45, x46,
    x47, x48, x49, x50, x51, x52, x53, x54, x55, x56, x57, x58, x59, x60, x61,
    x62, x63, x64, x65, x66, x67, x68, x69, x70, x71, x72, x73, x74, x75)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(
    76, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15, x16,
    x17, x18, x19, x20, x21, x22, x23, x24, x25, x26, x27, x28, x29, x30, x31,
    x32, x33, x34, x35, x36, x37, x38, x39, x40, x41, x42, x43, x44, x45, x46,
    x47, x48, x49, x50, x51, x52, x53, x54, x55, x56, x57, x58, x59, x60, x61,
    x62, x63, x64, x65, x66, x67, x68, x69, x70, x71, x72, x73, x74, x75, x76)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(77, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27, x28,
                                  x29, x30, x31, x32, x33, x34, x35, x36, x37,
                                  x38, x39, x40, x41, x42, x43, x44, x45, x46,
                                  x47, x48, x49, x50, x51, x52, x53, x54, x55,
                                  x56, x57, x58, x59, x60, x61, x62, x63, x64,
                                  x65, x66, x67, x68, x69, x70, x71, x72, x73,
                                  x74, x75, x76, x77)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(78, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27, x28,
                                  x29, x30, x31, x32, x33, x34, x35, x36, x37,
                                  x38, x39, x40, x41, x42, x43, x44, x45, x46,
                                  x47, x48, x49, x50, x51, x52, x53, x54, x55,
                                  x56, x57, x58, x59, x60, x61, x62, x63, x64,
                                  x65, x66, x67, x68, x69, x70, x71, x72, x73,
                                  x74, x75, x76, x77, x78)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(79, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27, x28,
                                  x29, x30, x31, x32, x33, x34, x35, x36, x37,
                                  x38, x39, x40, x41, x42, x43, x44, x45, x46,
                                  x47, x48, x49, x50, x51, x52, x53, x54, x55,
                                  x56, x57, x58, x59, x60, x61, x62, x63, x64,
                                  x65, x66, x67, x68, x69, x70, x71, x72, x73,
                                  x74, x75, x76, x77, x78, x79)
FUZZTEST_INTERNAL_BIND_AGGREGATE_(80, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
                                  x11, x12, x13, x14, x15, x16, x17, x18, x19,
                                  x20, x21, x22, x23, x24, x25, x26, x27, x28,
                                  x29, x30, x31, x32, x33, x34, x35, x36, x37,
                                  x38, x39, x40, x41, x42, x43, x44, x45, x46,
                                  x47, x48, x49, x50, x51, x52, x53, x54, x55,
                                  x56, x57, x58, x59, x60, x61, x62, x63, x64,
                                  x65, x66, x67, x68, x69, x70, x71, x72, x73,
                                  x74, x75, x76, x77, x78, x79, x80)

#undef FUZZTEST_INTERNAL_BIND_AGGREGATE_

// For N > 80, use std::get<I>.
// Some aggregate types like std::tuple provide this protocol, but so can user
// defined types.
template <typename T, int N>
auto BindAggregate(T& value, std::integral_constant<int, N>) {
  static_assert(N > 80);
  if constexpr (std::is_aggregate_v<T> &&
                !Requires<T>([](auto v) -> decltype(std::get<0>(v)) {})) {
    static_assert(always_false<T>,
                  "Aggregate types are only supported up to 80 fields.");
  }
  // For the rest use the tuple API
  return ApplyIndex<N>(
      [&](auto... I) { return std::tie(std::get<I>(value)...); });
}

// We disable `T` because `{T&&}` would work for non aggregates as a move
// construction.
template <typename T>
struct AnythingBut {
  template <typename U, std::enable_if_t<!std::is_same_v<U, T>, int> = 0>
  operator U&&() const {
    std::abort();
  }
};

// Try to call ApplyIndex from I all the way down to 0.
template <size_t I, typename F>
constexpr std::optional<int> ApplyIndexFor(F f) {
  if constexpr (ApplyIndex<I>(f)) {
    return I;
  }
  if constexpr (I == 0) {
    return std::nullopt;
  } else {
    return ApplyIndexFor<I - 1>(f);
  }
}

// Try applying aggregate initialization to `T` with different number of
// arguments. We start at 80 and go down until one works.
template <typename T>
constexpr std::optional<int> DetectBraceInitCount() {
  constexpr auto can_init_impl =
      [](auto... I) -> decltype(T{(I, AnythingBut<T>{})...}) {};
  constexpr auto can_init = [](auto... I) {
    return std::is_invocable_v<decltype(can_init_impl), decltype(I)...>;
  };
  if constexpr (!std::is_aggregate_v<T>) {
    return std::nullopt;
  }

  return ApplyIndexFor<80>(can_init);
}

// This allows us to determine whether T has a base class. If it does, it is
// initialized with a one more field than we can bind to it.
template <typename T>
struct AnythingButBaseOf {
  template <typename U, std::enable_if_t<!std::is_base_of_v<U, T>, int> = 0>
  operator U&&() const {
    std::abort();
  }
};

// Detect the number of fields bindable with `auto& [...] = t;`.
// This can be less than the number of fields used to initialize `T` if `T`
// inherits from an empty base class. Multiple inheritance is not supported.
template <typename T>
constexpr std::optional<int> DetectBindableFieldCount() {
  // Classes which inherit from a base are initialized with an extra first field
  // corresponding to the base class. We know from DetectBraceInitCount() that
  // `T` is initializable with N fields which are not a `T`. If `T` is not
  // initializable when we insist that the first field is not a base of `T`,
  // then we know that T has a base class and needs to be adjusted accordingly.
  // We don't worry about whether the potential base classes of T are empty: if
  // they are not, then binding will fail later anyways with a good error
  // message, and there is no way around this.
  constexpr std::optional<int> brace_init_count_opt = DetectBraceInitCount<T>();
  constexpr int brace_init_count = brace_init_count_opt.value_or(0);

  // Detect if the first initialization field is a base class.
  constexpr auto no_base_impl = [](auto... I)
      -> decltype(T{AnythingButBaseOf<T>{}, (I, AnythingBut<T>{})...}) {};
  constexpr auto no_base = [](auto... I) {
    return std::is_invocable_v<decltype(no_base_impl), decltype(I)...>;
  };

  if constexpr (brace_init_count < 1) {
    return brace_init_count_opt;
  } else if constexpr (ApplyIndex<brace_init_count - 1>(no_base)) {
    return brace_init_count;
  }

  // Detect if the second initialization field is a base class.
  constexpr auto no_two_bases_impl =
      [](auto... I) -> decltype(T{AnythingBut<T>{}, AnythingButBaseOf<T>{},
                                  (I, AnythingBut<T>{})...}) {};
  constexpr auto no_two_bases = [](auto... I) {
    return std::is_invocable_v<decltype(no_two_bases_impl), decltype(I)...>;
  };

  // Check for multiple inheritance. This condition also triggers if we have a
  // struct whose first field is its base class. The latter case should be
  // somewhat rare, because:
  // - We don't currently support structs with a non-empty parent class
  //   (these cannot be decomposed in `BindAggregate()`).
  // - It is rare to have a struct with a member that is an empty class, as
  //   would be the case if the parent were empty.
  if constexpr (brace_init_count >= 2) {
    static_assert(ApplyIndex<brace_init_count - 2>(no_two_bases),
                  "Multiple inheritance is not currently supported, nor are "
                  "structs whose first fields are their base class.");
  }
  // We have exactly one base class.
  return brace_init_count - 1;
}

template <typename T>
constexpr std::optional<int> DetectAggregateSize() {
  if constexpr (is_pair_v<T> || is_tuple_v<T> || is_array_v<T>) {
    return std::tuple_size_v<T>;
  } else {
    return DetectBindableFieldCount<T>();
  }
}

// Detect the number and types of the fields.
template <typename T, int N = *DetectAggregateSize<std::remove_cv_t<T>>()>
auto DetectBindAggregate(T& v) {
  return BindAggregate(v, std::integral_constant<int, N>{});
}

template <typename T>
inline constexpr bool is_bindable_aggregate_v =
    Requires<T&>([](auto& v) -> decltype(DetectBindAggregate(v)) {});

template <int I, typename T>
struct ExtractTemplateParameterImpl;

template <int I, template <typename...> class C, typename... T>
struct ExtractTemplateParameterImpl<I, C<T...>> {
  using type = std::tuple_element_t<I, std::tuple<T...>>;
};

// Evaluates to the Ith template parameter of the class template
// instantiation `T`. Eg:
//   `ExtractTemplateParameter<1, Foo<int, double, char>>`
// evaluates to `double`.
template <int I, typename T>
using ExtractTemplateParameter =
    typename ExtractTemplateParameterImpl<I, T>::type;

template <typename T>
struct DropConstImpl {
  using type = std::remove_const_t<T>;
};

template <template <typename...> class C, typename... T>
struct DropConstImpl<C<T...>> {
  using type = C<typename DropConstImpl<T>::type...>;
};

// Drop `const` recursively from class templates.
// Eg:
//   `DropConst<std::vector<std::pair<const K, std::tuple<const int>>>`
// evaluates to
//   `std::vector<std::pair<K, std::tuple<int>>>`
template <typename T>
using DropConst = typename DropConstImpl<T>::type;

// MakeDependentType<T, U...> evaluates to `T` but makes the declaration type
// dependent on templates `U...`.
// This allows delaying name lookup for the second phase when the types involved
// are fully defined.
// See https://en.cppreference.com/w/cpp/language/two-phase_lookup
template <typename T, typename... Dependent>
using MakeDependentType = std::enable_if_t<(always_true<Dependent> && ...), T>;

// We use this to get a nice compiler error when `T` and `U` don't match instead
// of just "false".
template <typename T, typename U>
constexpr void CheckIsSame() {
  static_assert(std::is_same_v<T, U>);
}

template <typename Domain>
using value_type_t = typename Domain::value_type;

template <typename Domain>
using corpus_type_t = typename Domain::corpus_type;

}  // namespace fuzztest::internal

#endif  // FUZZTEST_FUZZTEST_INTERNAL_META_H_
