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

#ifndef SRC_TRACE_PROCESSOR_UTIL_BUILD_ID_H_
#define SRC_TRACE_PROCESSOR_UTIL_BUILD_ID_H_

#include <cstddef>
#include <cstdint>
#include <functional>
#include <string>
#include <utility>

#include "perfetto/ext/base/hash.h"
#include "perfetto/ext/base/string_view.h"

namespace perfetto::trace_processor {

// Represents the unique identifier of an executable, shared library, or module.
// For example for ELF files this is the id stored in the .note.gnu.build-id
// section. Sometimes a breakpad module id is used.
// This class abstracts away the details of where this id comes from and how it
// is converted to a StringId which is the representation used by tables in
// trace_processor.
class BuildId {
 public:
  // Allow hashing with base::Hash.
  static constexpr bool kHashable = true;
  size_t size() const { return raw_.size(); }
  const char* data() const { return raw_.data(); }

  static BuildId FromHex(base::StringView data);

  static BuildId FromRaw(base::StringView data) {
    return BuildId(data.ToStdString());
  }
  static BuildId FromRaw(std::string data) { return BuildId(std::move(data)); }
  static BuildId FromRaw(const uint8_t* data, size_t size) {
    return BuildId(std::string(reinterpret_cast<const char*>(data), size));
  }

  BuildId(const BuildId&) = default;
  BuildId(BuildId&&) = default;

  BuildId& operator=(const BuildId&) = default;
  BuildId& operator=(BuildId&&) = default;

  bool operator==(const BuildId& o) const { return raw_ == o.raw_; }

  bool operator!=(const BuildId& o) const { return !(*this == o); }

  bool operator<(const BuildId& o) const { return raw_ < o.raw_; }

  std::string ToHex() const;

  const std::string& raw() const { return raw_; }

 private:
  explicit BuildId(std::string data) : raw_(std::move(data)) {}
  std::string raw_;
};

}  // namespace perfetto::trace_processor

template <>
struct std::hash<perfetto::trace_processor::BuildId> {
  std::size_t operator()(
      const perfetto::trace_processor::BuildId& o) const noexcept {
    return perfetto::base::Hasher::Combine(o);
  }
};

#endif  // SRC_TRACE_PROCESSOR_UTIL_BUILD_ID_H_
