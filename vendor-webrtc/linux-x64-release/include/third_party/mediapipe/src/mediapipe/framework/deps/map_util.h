// Copyright 2019 The MediaPipe Authors.
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
//
// This file provides utility functions for use with STL map-like data
// structures, such as std::map and hash_map. Some functions will also work with
// sets, such as ContainsKey().

#ifndef MEDIAPIPE_DEPS_MAP_UTIL_H_
#define MEDIAPIPE_DEPS_MAP_UTIL_H_

#include <stddef.h>

#include <iterator>
#include <memory>
#include <string>
#include <type_traits>
#include <utility>

#include "absl/log/absl_check.h"
#include "mediapipe/framework/port/logging.h"

namespace mediapipe {

// A note on terminology: `m` and `M` represent a map and its type.
//
// Returns a const reference to the value associated with the given key if it
// exists. Crashes otherwise.
//
// This is intended as a replacement for operator[] as an rvalue (for reading)
// when the key is guaranteed to exist.
//
// operator[] for lookup is discouraged for several reasons (note that these
// reasons may apply to only some map types):
//  * It has a side-effect of inserting missing keys
//  * It is not thread-safe (even when it is not inserting, it can still
//      choose to resize the underlying storage)
//  * It invalidates iterators (when it chooses to resize)
//  * It default constructs a value object even if it doesn't need to
//
// This version assumes the key is printable, and includes it in the fatal log
// message.
template <typename M>
const typename M::value_type::second_type& FindOrDie(
    const M& m, const typename M::value_type::first_type& key) {
  auto it = m.find(key);
  ABSL_CHECK(it != m.end()) << "Map key not found: " << key;
  return it->second;
}

// Same as above, but returns a non-const reference.
template <typename M>
typename M::value_type::second_type& FindOrDie(
    M& m,  // NOLINT
    const typename M::value_type::first_type& key) {
  auto it = m.find(key);
  ABSL_CHECK(it != m.end()) << "Map key not found: " << key;
  return it->second;
}

// Returns a const reference to the value associated with the given key if it
// exists, otherwise returns a const reference to the provided default value.
//
// WARNING: If a temporary object is passed as the default "value,"
// this function will return a reference to that temporary object,
// which will be destroyed at the end of the statement. A common
// example: if you have a map with std::string values, and you pass a char*
// as the default "value," either use the returned value immediately
// or store it in a std::string (not std::string&).
template <typename M>
const typename M::value_type::second_type& FindWithDefault(
    const M& m, const typename M::value_type::first_type& key,
    const typename M::value_type::second_type& value) {
  auto it = m.find(key);
  if (it != m.end()) {
    return it->second;
  }
  return value;
}

// Returns a pointer to the const value associated with the given key if it
// exists, or null otherwise.
template <typename M>
const typename M::value_type::second_type* FindOrNull(
    const M& m, const typename M::value_type::first_type& key) {
  auto it = m.find(key);
  if (it == m.end()) {
    return nullptr;
  }
  return &it->second;
}

// Returns a pointer to the non-const value associated with the given key if it
// exists, or null otherwise.
template <typename M>
typename M::value_type::second_type* FindOrNull(
    M& m,  // NOLINT
    const typename M::value_type::first_type& key) {
  auto it = m.find(key);
  if (it == m.end()) {
    return nullptr;
  }
  return &it->second;
}

// Returns true if and only if the given m contains the given key.
template <typename M, typename Key>
bool ContainsKey(const M& m, const Key& key) {
  return m.find(key) != m.end();
}

// Inserts the given key and value into the given m if and only if the
// given key did NOT already exist in the m. If the key previously
// existed in the m, the value is not changed. Returns true if the
// key-value pair was inserted; returns false if the key was already present.
template <typename M>
bool InsertIfNotPresent(M* m, const typename M::value_type& vt) {
  return m->insert(vt).second;
}

// Same as above except the key and value are passed separately.
template <typename M>
bool InsertIfNotPresent(M* m, const typename M::value_type::first_type& key,
                        const typename M::value_type::second_type& value) {
  return InsertIfNotPresent(m, {key, value});
}

// Saves the reverse mapping into reverse. Returns true if values could all be
// inserted.
template <typename M, typename ReverseM>
bool ReverseMap(const M& m, ReverseM* reverse) {
  ABSL_CHECK(reverse != nullptr);
  for (const auto& kv : m) {
    if (!InsertIfNotPresent(reverse, kv.second, kv.first)) {
      return false;
    }
  }
  return true;
}

}  // namespace mediapipe

#endif  // MEDIAPIPE_DEPS_MAP_UTIL_H_
