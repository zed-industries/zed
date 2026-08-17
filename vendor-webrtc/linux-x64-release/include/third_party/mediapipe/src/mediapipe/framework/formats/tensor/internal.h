// Copyright 2021 The MediaPipe Authors.
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

#ifndef MEDIAPIPE_FRAMEWORK_FORMATS_TENSOR_INTERNAL_H_
#define MEDIAPIPE_FRAMEWORK_FORMATS_TENSOR_INTERNAL_H_

#include <cstdint>
#include <type_traits>

namespace mediapipe {

// Generates unique view id at compile-time using FILE and LINE.
#define TENSOR_UNIQUE_VIEW_TYPE_ID()                       \
  static inline uint64_t kId = tensor_internal::FnvHash64( \
      __FILE__, tensor_internal::FnvHash64(TENSOR_INT_TO_STRING(__LINE__)))

// Generates unique view id at compile-time using FILE and LINE and Type of the
// template view's argument.
#define TENSOR_UNIQUE_VIEW_TYPE_ID_T(T) \
  static inline uint64_t kId = tool::GetTypeHash<T>();

namespace tensor_internal {

#define TENSOR_INT_TO_STRING2(x) #x
#define TENSOR_INT_TO_STRING(x) TENSOR_INT_TO_STRING2(x)

// Compile-time hash function
// https://en.wikipedia.org/wiki/Fowler%E2%80%93Noll%E2%80%93Vo_hash_function
inline constexpr uint64_t kFnvPrime = 0x00000100000001B3;
inline constexpr uint64_t kFnvOffsetBias = 0xcbf29ce484222325;
inline constexpr uint64_t FnvHash64(uint64_t value1, uint64_t value2) {
  return (value2 ^ value1) * kFnvPrime;
}
constexpr uint64_t FnvHash64(const char* str, uint64_t hash = kFnvOffsetBias) {
  return (str[0] == 0) ? hash : FnvHash64(str + 1, FnvHash64(hash, str[0]));
}
template <typename... Ts>
struct TypeList {
  static constexpr std::size_t size{sizeof...(Ts)};
};
template <typename, typename>
struct TypeInList {};
template <typename T, typename... Ts>
struct TypeInList<T, TypeList<T, Ts...>>
    : std::integral_constant<std::size_t, 0> {};
template <typename T, typename TOther, typename... Ts>
struct TypeInList<T, TypeList<TOther, Ts...>>
    : std::integral_constant<std::size_t,
                             1 + TypeInList<T, TypeList<Ts...>>::value> {};

}  // namespace tensor_internal
}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_FORMATS_TENSOR_INTERNAL_H_
