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

#ifndef DISTRIBUTED_POINT_FUNCTIONS_DPF_XOR_WRAPPER_H_
#define DISTRIBUTED_POINT_FUNCTIONS_DPF_XOR_WRAPPER_H_

#include <utility>

namespace distributed_point_functions {

// Wraps the given type, replacing additions and subtractions by XOR.
template <typename T>
class XorWrapper {
 public:
  using WrappedType = T;

  constexpr XorWrapper() : wrapped_{} {}
  explicit constexpr XorWrapper(T wrapped) : wrapped_(std::move(wrapped)) {}

  // XorWrapper is copyable and movable.
  constexpr XorWrapper(const XorWrapper&) = default;
  constexpr XorWrapper& operator=(const XorWrapper&) = default;
  constexpr XorWrapper(XorWrapper&&) = default;
  constexpr XorWrapper& operator=(XorWrapper&&) = default;

  // Assignment operators.
  constexpr XorWrapper& operator+=(const XorWrapper& rhs) {
    wrapped_ ^= rhs.value();
    return *this;
  }
  constexpr XorWrapper& operator-=(const XorWrapper& rhs) {
    wrapped_ ^= rhs.value();
    return *this;
  }

  // Returns a reference to the wrapped object.
  constexpr T& value() { return wrapped_; }
  constexpr const T& value() const { return wrapped_; }

 private:
  T wrapped_;
};

template <typename T>
constexpr XorWrapper<T> operator+(XorWrapper<T> a, const XorWrapper<T>& b) {
  a += b;
  return a;
}

template <typename T>
constexpr XorWrapper<T> operator-(XorWrapper<T> a, const XorWrapper<T>& b) {
  a -= b;
  return a;
}

// Negation does nothing in XOR sharing, since -a = 0-a.
template <typename T>
constexpr XorWrapper<T> operator-(const XorWrapper<T>& a) {
  return a;
}

template <typename T>
constexpr bool operator==(const XorWrapper<T>& a, const XorWrapper<T>& b) {
  return a.value() == b.value();
}

template <typename T>
constexpr bool operator!=(const XorWrapper<T>& a, const XorWrapper<T>& b) {
  return !(a == b);
}

}  // namespace distributed_point_functions

#endif  // DISTRIBUTED_POINT_FUNCTIONS_DPF_XOR_WRAPPER_H_
