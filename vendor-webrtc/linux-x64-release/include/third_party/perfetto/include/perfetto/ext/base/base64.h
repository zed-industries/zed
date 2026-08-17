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

#ifndef INCLUDE_PERFETTO_EXT_BASE_BASE64_H_
#define INCLUDE_PERFETTO_EXT_BASE_BASE64_H_

#include <optional>
#include <string>

#include "perfetto/ext/base/string_view.h"
#include "perfetto/ext/base/utils.h"  // For ssize_t.

namespace perfetto {
namespace base {

// Returns the length of the destination string (included '=' padding).
// Does NOT include the size of the string null terminator.
inline size_t Base64EncSize(size_t src_size) {
  return (src_size + 2) / 3 * 4;
}

// Returns the upper bound on the length of the destination buffer.
// The actual decoded length might be <= the number returned here.
inline size_t Base64DecSize(size_t src_size) {
  return (src_size + 3) / 4 * 3;
}

// Does NOT null-terminate |dst|.
ssize_t Base64Encode(const void* src,
                     size_t src_size,
                     char* dst,
                     size_t dst_size);

std::string Base64Encode(const void* src, size_t src_size);

inline std::string Base64Encode(StringView sv) {
  return Base64Encode(sv.data(), sv.size());
}

// Returns -1 in case of failure.
ssize_t Base64Decode(const char* src,
                     size_t src_size,
                     uint8_t* dst,
                     size_t dst_size);

std::optional<std::string> Base64Decode(const char* src, size_t src_size);

inline std::optional<std::string> Base64Decode(StringView sv) {
  return Base64Decode(sv.data(), sv.size());
}

}  // namespace base
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_EXT_BASE_BASE64_H_
