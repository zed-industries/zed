/*
 * Copyright 2021 Google LLC
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

#ifndef DISTRIBUTED_POINT_FUNCTIONS_DPF_TUPLE_H_
#define DISTRIBUTED_POINT_FUNCTIONS_DPF_TUPLE_H_

#include <stddef.h>

#include <tuple>
#include <utility>

namespace distributed_point_functions {

// A Tuple class with added element-wise addition, subtraction, and negation
// operators.
template <typename... T>
class Tuple {
 public:
  using Base = std::tuple<T...>;

  Tuple() {}
  Tuple(T... elements) : value_(elements...) {}
  explicit Tuple(Base t) : value_(std::move(t)) {}

  // Copy constructor.
  Tuple(const Tuple& t) = default;
  Tuple& operator=(const Tuple& t) = default;

  // Getters for the base tuple type.
  Base& value() { return value_; }
  const Base& value() const { return value_; }

 private:
  Base value_;
};

namespace dpf_internal {

// Implementation of addition and negation. See
// https://stackoverflow.com/a/50815143.
// We declare the templates here, but define them at the end of this header
// because the definitions need to make use of operator+ and operator-.
template <typename... T, std::size_t... I>
constexpr Tuple<T...> add(const Tuple<T...>& lhs, const Tuple<T...>& rhs,
                          std::index_sequence<I...>);

template <typename... T, std::size_t... I>
constexpr Tuple<T...> negate(const Tuple<T...>& t, std::index_sequence<I...>);

}  // namespace dpf_internal

template <typename... T>
constexpr Tuple<T...> operator+(const Tuple<T...>& lhs,
                                const Tuple<T...>& rhs) {
  return dpf_internal::add(lhs, rhs, std::make_index_sequence<sizeof...(T)>{});
}

template <typename... T>
constexpr Tuple<T...>& operator+=(Tuple<T...>& lhs, const Tuple<T...>& rhs) {
  lhs = lhs + rhs;
  return lhs;
}

template <typename... T>
constexpr Tuple<T...> operator-(const Tuple<T...>& t) {
  return dpf_internal::negate(t, std::make_index_sequence<sizeof...(T)>{});
}

template <typename... T>
constexpr Tuple<T...> operator-(const Tuple<T...>& lhs,
                                const Tuple<T...>& rhs) {
  return lhs + (-rhs);
}

template <typename... T>
constexpr Tuple<T...>& operator-=(Tuple<T...>& lhs, const Tuple<T...>& rhs) {
  lhs = lhs - rhs;
  return lhs;
}

// Equality and inequality operators.
template <typename... T>
constexpr bool operator==(const Tuple<T...>& lhs, const Tuple<T...>& rhs) {
  return lhs.value() == rhs.value();
}

template <typename... T>
constexpr bool operator!=(const Tuple<T...>& lhs, const Tuple<T...>& rhs) {
  return lhs.value() != rhs.value();
}

namespace dpf_internal {
template <typename... T, std::size_t... I>
constexpr Tuple<T...> add(const Tuple<T...>& lhs, const Tuple<T...>& rhs,
                          std::index_sequence<I...>) {
  return Tuple<T...>{std::get<I>(lhs.value()) + std::get<I>(rhs.value())...};
}

template <typename... T, std::size_t... I>
constexpr Tuple<T...> negate(const Tuple<T...>& t, std::index_sequence<I...>) {
  return Tuple<T...>{
      // Explicitly cast to T to avoid -Wnarrowing warnings for small integers.
      T(-std::get<I>(t.value()))...};
}
}  // namespace dpf_internal

}  // namespace distributed_point_functions

#endif  // DISTRIBUTED_POINT_FUNCTIONS_DPF_TUPLE_H_
