/*
 * Copyright (C) 2021 The Android Open Source Project
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

#ifndef INCLUDE_PERFETTO_TRACING_STRING_HELPERS_H_
#define INCLUDE_PERFETTO_TRACING_STRING_HELPERS_H_

#include "perfetto/base/export.h"
#include "perfetto/base/logging.h"

#include <cstddef>
#include <string>

namespace perfetto {

// A wrapper for marking strings that can't be determined to be static at build
// time, but are in fact static.
class PERFETTO_EXPORT_COMPONENT StaticString {
 public:
  // Implicit constructor for string literals.
  template <size_t N>
  constexpr StaticString(const char (&str)[N]) : value(str) {}

  // Implicit constructor for null strings.
  constexpr StaticString(std::nullptr_t) : value(nullptr) {}

  constexpr explicit StaticString(const char* str) : value(str) {}

  operator bool() const { return !!value; }

  const char* value;
};

// A explicit wrapper for marking strings as dynamic to ensure that perfetto
// doesn't try to cache the pointer value.
class PERFETTO_EXPORT_COMPONENT DynamicString {
 public:
  explicit DynamicString(const std::string& str)
      : value(str.data()), length(str.length()) {}
  explicit DynamicString(const char* str) : value(str) {
    PERFETTO_DCHECK(str);
    length = strlen(str);
  }
  DynamicString(const char* str, size_t len) : value(str), length(len) {}
  constexpr DynamicString() : value(nullptr), length(0) {}

  operator bool() const { return !!value; }

  const char* value;
  size_t length;
};

namespace internal {

template <size_t N>
constexpr const char* GetStaticString(const char (&string)[N]) {
  return string;
}

constexpr std::nullptr_t GetStaticString(std::nullptr_t) {
  return nullptr;
}

constexpr const char* GetStaticString(perfetto::StaticString string) {
  return string.value;
}

}  // namespace internal
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_TRACING_STRING_HELPERS_H_
