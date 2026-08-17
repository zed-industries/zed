//
// Copyright 2023 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

#ifndef GRPC_SRC_CORE_UTIL_REF_COUNTED_STRING_H
#define GRPC_SRC_CORE_UTIL_REF_COUNTED_STRING_H

#include <grpc/support/port_platform.h>
#include <stddef.h>

#include <string>

#include "absl/strings/string_view.h"
#include "src/core/util/ref_counted.h"
#include "src/core/util/ref_counted_ptr.h"

namespace grpc_core {

// An immutable reference counted string.
class RefCountedString {
 public:
  static RefCountedPtr<RefCountedString> Make(absl::string_view src);

  // Not copyable.
  RefCountedString(const RefCountedString&) = delete;
  RefCountedString& operator=(const RefCountedString&) = delete;

  // Provide the same interface as RefCounted<>.
  // We reimplement this instead of inheritting to make pointer math
  // easier in Make().
  RefCountedPtr<RefCountedString> Ref() {
    IncrementRefCount();
    return RefCountedPtr<RefCountedString>(this);
  }
  void Unref() {
    if (header_.rc.Unref()) Destroy();
  }

  absl::string_view as_string_view() const {
    return absl::string_view(payload_, header_.length);
  }

  char* c_str() { return payload_; }

 private:
  // Allow RefCountedPtr<> to access IncrementRefCount().
  template <typename T>
  friend class RefCountedPtr;

  explicit RefCountedString(absl::string_view src);
  void IncrementRefCount() { header_.rc.Ref(); }
  void Destroy();

  struct Header {
    RefCount rc;
    size_t length;
  };
  Header header_;
  char payload_[];
};

// Wrapper around RefCountedPtr<RefCountedString> to give value semantics,
// especially to overloaded operators.
class RefCountedStringValue {
 public:
  RefCountedStringValue() : str_{} {}
  explicit RefCountedStringValue(absl::string_view str)
      : str_(RefCountedString::Make(str)) {}

  absl::string_view as_string_view() const {
    return str_ == nullptr ? absl::string_view() : str_->as_string_view();
  }

  const char* c_str() const { return str_ == nullptr ? "" : str_->c_str(); }

 private:
  RefCountedPtr<RefCountedString> str_;
};

inline bool operator==(const RefCountedStringValue& lhs,
                       absl::string_view rhs) {
  return lhs.as_string_view() == rhs;
}
inline bool operator==(absl::string_view lhs,
                       const RefCountedStringValue& rhs) {
  return lhs == rhs.as_string_view();
}
inline bool operator==(const RefCountedStringValue& lhs,
                       const RefCountedStringValue& rhs) {
  return lhs.as_string_view() == rhs.as_string_view();
}

inline bool operator!=(const RefCountedStringValue& lhs,
                       absl::string_view rhs) {
  return lhs.as_string_view() != rhs;
}
inline bool operator!=(absl::string_view lhs,
                       const RefCountedStringValue& rhs) {
  return lhs != rhs.as_string_view();
}
inline bool operator!=(const RefCountedStringValue& lhs,
                       const RefCountedStringValue& rhs) {
  return lhs.as_string_view() != rhs.as_string_view();
}

inline bool operator<(const RefCountedStringValue& lhs, absl::string_view rhs) {
  return lhs.as_string_view() < rhs;
}
inline bool operator<(absl::string_view lhs, const RefCountedStringValue& rhs) {
  return lhs < rhs.as_string_view();
}
inline bool operator<(const RefCountedStringValue& lhs,
                      const RefCountedStringValue& rhs) {
  return lhs.as_string_view() < rhs.as_string_view();
}

inline bool operator>(const RefCountedStringValue& lhs, absl::string_view rhs) {
  return lhs.as_string_view() > rhs;
}
inline bool operator>(absl::string_view lhs, const RefCountedStringValue& rhs) {
  return lhs > rhs.as_string_view();
}
inline bool operator>(const RefCountedStringValue& lhs,
                      const RefCountedStringValue& rhs) {
  return lhs.as_string_view() > rhs.as_string_view();
}

// A sorting functor to support heterogeneous lookups in sorted containers.
struct RefCountedStringValueLessThan {
  using is_transparent = void;
  bool operator()(const RefCountedStringValue& lhs,
                  const RefCountedStringValue& rhs) const {
    return lhs < rhs;
  }
  bool operator()(absl::string_view lhs,
                  const RefCountedStringValue& rhs) const {
    return lhs < rhs;
  }
  bool operator()(const RefCountedStringValue& lhs,
                  absl::string_view rhs) const {
    return lhs < rhs;
  }
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_UTIL_REF_COUNTED_STRING_H
