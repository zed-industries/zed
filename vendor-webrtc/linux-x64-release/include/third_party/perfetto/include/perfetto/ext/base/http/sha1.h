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

#ifndef INCLUDE_PERFETTO_EXT_BASE_HTTP_SHA1_H_
#define INCLUDE_PERFETTO_EXT_BASE_HTTP_SHA1_H_

#include <stddef.h>

#include <array>
#include <cstddef>
#include <cstdint>
#include <string>

namespace perfetto {
namespace base {

constexpr size_t kSHA1Length = 20;
using SHA1Digest = std::array<uint8_t, kSHA1Length>;

SHA1Digest SHA1Hash(const std::string& str);
SHA1Digest SHA1Hash(const void* data, size_t size);

}  // namespace base
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_EXT_BASE_HTTP_SHA1_H_
