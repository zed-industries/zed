/*
 * Copyright 2023 Google LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef DISTRIBUTED_POINT_FUNCTIONS_DPF_INTERNAL_ANY_SPAN_H_
#define DISTRIBUTED_POINT_FUNCTIONS_DPF_INTERNAL_ANY_SPAN_H_

// A class that can serve the purpose of both absl::Span<T> and absl::Span<T*>
// at the same time. Introduces the run-time overhead of a std::variant check.
//
// Note that this class DOES NOT provide common container features, such as
// iterators. It is not intended to be used by users of this library. Any
// function that takes a MaybeDerefSpan<T> should be called with either an
// absl::Span<T> or an absl::Span<T*>.

#include <type_traits>

#include "absl/meta/type_traits.h"
#include "absl/types/span.h"
#include "absl/types/variant.h"

namespace distributed_point_functions {
namespace dpf_internal {

template <typename T>
class MaybeDerefSpan {
 private:
  template <typename U>
  using EnableIfValueIsConst =
      typename absl::enable_if_t<std::is_const<T>::value, U>;

  template <typename U>
  using EnableIfValueIsConvertibleToSpan = typename absl::enable_if_t<
      absl::disjunction<std::is_convertible<U, absl::Span<T>>,
                        std::is_convertible<U, absl::Span<T* const>>>::value,
      U>;

 public:
  // Implicit constructors from the underlying absl::Span.
  MaybeDerefSpan(absl::Span<T> span)
      : span_(span) {}  // NOLINT(runtime/explicit)
  MaybeDerefSpan(absl::Span<T* const> span)
      : span_(span) {}  // NOLINT(runtime/explicit)

  // Implicit constructor of a const MaybeDerefSpan from a non-const one.
  template <typename T2 = T, typename = EnableIfValueIsConst<T2>>
  MaybeDerefSpan(
      const MaybeDerefSpan<typename std::remove_const<T>::type>& other)
      : span_(absl::ConvertVariantTo<decltype(span_)>(other.span_)) {
  }  // NOLINT(runtime/explicit)

  // Implicit constructor of a const MaybeDerefSpan from anything that is
  // convertible to one of the underlying spans.
  template <typename V, typename = EnableIfValueIsConst<V>,
            typename = EnableIfValueIsConvertibleToSpan<V>>
  MaybeDerefSpan(const V& span)
      : span_(absl::MakeConstSpan(span)) {}  // NOLINT(runtime/explicit)

  inline constexpr T& operator[](size_t index) const {
    if (absl::holds_alternative<absl::Span<T* const>>(span_)) {
      return *absl::get<absl::Span<T* const>>(span_)[index];
    }
    return absl::get<absl::Span<T>>(span_)[index];
  }

  inline constexpr size_t size() const {
    return absl::visit([](auto v) { return v.size(); }, span_);
  }

 private:
  template <typename U>
  friend class MaybeDerefSpan;

  absl::variant<absl::Span<T>, absl::Span<T* const>> span_;
};

}  // namespace dpf_internal
}  // namespace distributed_point_functions

#endif  // DISTRIBUTED_POINT_FUNCTIONS_DPF_INTERNAL_ANY_SPAN_H_
