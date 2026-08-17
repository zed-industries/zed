/*
 * Copyright (C) 2019 The Android Open Source Project
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

#ifndef INCLUDE_PERFETTO_EXT_BASE_HASH_H_
#define INCLUDE_PERFETTO_EXT_BASE_HASH_H_

#include <stddef.h>
#include <stdint.h>
#include <string>
#include <string_view>
#include <type_traits>
#include <utility>

namespace perfetto {
namespace base {

// A helper class which computes a 64-bit hash of the input data.
// The algorithm used is FNV-1a as it is fast and easy to implement and has
// relatively few collisions.
// WARNING: This hash function should not be used for any cryptographic purpose.
class Hasher {
 public:
  // Creates an empty hash object
  constexpr Hasher() = default;

  // Hashes a numeric value.
  template <
      typename T,
      typename std::enable_if<std::is_arithmetic<T>::value, bool>::type = true>
  void Update(T data) {
    Update(reinterpret_cast<const char*>(&data), sizeof(data));
  }

  constexpr void Update(char c) { return Update(&c, 1); }

  // Using the loop instead of "Update(str, strlen(str))" to avoid looping twice
  constexpr void Update(const char* str) {
    for (const auto* p = str; *p; ++p)
      Update(*p);
  }

  // Hashes a byte array.
  constexpr void Update(const char* data, size_t size) {
    for (size_t i = 0; i < size; i++) {
      result_ ^= static_cast<uint8_t>(data[i]);
      // Note: Arithmetic overflow of unsigned integers is well defined in C++
      // standard unlike signed integers.
      // https://stackoverflow.com/a/41280273
      result_ *= kFnv1a64Prime;
    }
  }

  // Allow hashing anything that has `data` and `size` and has the kHashable
  // trait (e.g., base::StringView).
  template <typename T, typename = std::enable_if_t<T::kHashable>>
  constexpr void Update(const T& t) {
    if constexpr (std::is_member_function_pointer_v<decltype(&T::data)>) {
      Update(t.data(), t.size());
    } else {
      Update(t.data, t.size);
    }
  }

  constexpr void Update(std::string_view s) { Update(s.data(), s.size()); }

  constexpr uint64_t digest() const { return result_; }

  // Usage:
  // uint64_t hashed_value = Hash::Combine(33, false, "ABC", 458L, 3u, 'x');
  template <typename... Ts>
  static constexpr uint64_t Combine(Ts&&... args) {
    Hasher hasher;
    hasher.UpdateAll(std::forward<Ts>(args)...);
    return hasher.digest();
  }

  // Creates a hasher with `args` already hashed.
  //
  // Usage:
  // Hasher partial = Hash::CreatePartial(33, false, "ABC", 458L);
  template <typename... Ts>
  static constexpr Hasher CreatePartial(Ts&&... args) {
    Hasher hasher;
    hasher.UpdateAll(std::forward<Ts>(args)...);
    return hasher;
  }

  // `hasher.UpdateAll(33, false, "ABC")` is shorthand for:
  // `hasher.Update(33); hasher.Update(false); hasher.Update("ABC");`
  constexpr void UpdateAll() {}

  template <typename T, typename... Ts>
  constexpr void UpdateAll(T&& arg, Ts&&... args) {
    Update(arg);
    UpdateAll(std::forward<Ts>(args)...);
  }

 private:
  static constexpr uint64_t kFnv1a64OffsetBasis = 0xcbf29ce484222325;
  static constexpr uint64_t kFnv1a64Prime = 0x100000001b3;

  uint64_t result_ = kFnv1a64OffsetBasis;
};

// This is for using already-hashed key into std::unordered_map and avoid the
// cost of re-hashing. Example:
// unordered_map<uint64_t, Value, AlreadyHashed> my_map.
template <typename T>
struct AlreadyHashed {
  size_t operator()(const T& x) const { return static_cast<size_t>(x); }
};

// base::Hash uses base::Hasher for integer values and falls base to std::hash
// for other types. This is needed as std::hash for integers is just the
// identity function and Perfetto uses open-addressing hash table, which are
// very sensitive to hash quality and are known to degrade in performance
// when using std::hash.
template <typename T>
struct Hash {
  // Version for ints, using base::Hasher.
  template <typename U = T>
  auto operator()(const U& x) ->
      typename std::enable_if<std::is_arithmetic<U>::value, size_t>::type
      const {
    Hasher hash;
    hash.Update(x);
    return static_cast<size_t>(hash.digest());
  }

  // Version for non-ints, falling back to std::hash.
  template <typename U = T>
  auto operator()(const U& x) ->
      typename std::enable_if<!std::is_arithmetic<U>::value, size_t>::type
      const {
    return std::hash<U>()(x);
  }
};

}  // namespace base
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_EXT_BASE_HASH_H_
