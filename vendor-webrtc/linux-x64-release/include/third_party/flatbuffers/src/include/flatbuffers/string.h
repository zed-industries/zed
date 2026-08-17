/*
 * Copyright 2021 Google Inc. All rights reserved.
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

#ifndef FLATBUFFERS_STRING_H_
#define FLATBUFFERS_STRING_H_

#include "flatbuffers/base.h"
#include "flatbuffers/vector.h"

namespace flatbuffers {

struct String : public Vector<char> {
  const char *c_str() const { return reinterpret_cast<const char *>(Data()); }
  std::string str() const { return std::string(c_str(), size()); }

  // clang-format off
  #ifdef FLATBUFFERS_HAS_STRING_VIEW
  flatbuffers::string_view string_view() const {
    return flatbuffers::string_view(c_str(), size());
  }

  /* implicit */
  operator flatbuffers::string_view() const {
    return flatbuffers::string_view(c_str(), size());
  }
  #endif // FLATBUFFERS_HAS_STRING_VIEW
  // clang-format on

  bool operator<(const String &o) const {
    return StringLessThan(this->data(), this->size(), o.data(), o.size());
  }
};

// Convenience function to get std::string from a String returning an empty
// string on null pointer.
static inline std::string GetString(const String *str) {
  return str ? str->str() : "";
}

// Convenience function to get char* from a String returning an empty string on
// null pointer.
static inline const char *GetCstring(const String *str) {
  return str ? str->c_str() : "";
}

#ifdef FLATBUFFERS_HAS_STRING_VIEW
// Convenience function to get string_view from a String returning an empty
// string_view on null pointer.
static inline flatbuffers::string_view GetStringView(const String *str) {
  return str ? str->string_view() : flatbuffers::string_view();
}
#endif  // FLATBUFFERS_HAS_STRING_VIEW

}  // namespace flatbuffers

#endif  // FLATBUFFERS_STRING_H_
