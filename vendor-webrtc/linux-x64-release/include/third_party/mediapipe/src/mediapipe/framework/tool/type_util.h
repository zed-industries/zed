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

#ifndef MEDIAPIPE_FRAMEWORK_TOOL_TYPE_UTIL_H_
#define MEDIAPIPE_FRAMEWORK_TOOL_TYPE_UTIL_H_

#include <cstddef>
#include <ostream>
#include <string>
#include <typeinfo>
#include <utility>

#include "absl/base/attributes.h"
#include "mediapipe/framework/demangle.h"
#include "mediapipe/framework/port.h"

namespace mediapipe {

// An identifier for a type. This class is lightweight and is meant to be passed
// by value.
// To get the TypeId for SomeType, write kTypeId<SomeType>.
class TypeId {
 public:
  size_t hash_code() const { return impl_.hash_code(); }
  std::string name() const { return impl_.name(); }
  bool operator==(const TypeId& other) const { return impl_ == other.impl_; }
  bool operator<(const TypeId& other) const { return impl_ < other.impl_; }
  friend std::ostream& operator<<(std::ostream& stream, const TypeId& id) {
    return stream << id.name();
  }

  template <typename H>
  friend H AbslHashValue(H h, const TypeId& r) {
    return H::combine(std::move(h), r.hash_code());
  }

  template <class T>
  static constexpr inline TypeId Of() {
    return TypeId{Impl::Get<T>()};
  }

 private:
  // This implementation uses no RTTI. It distinguishes types, but does not
  // know their names.
  // TODO: record compile-time type string for (some or all) types.
  template <class T>
  struct TypeTag {
    static constexpr char dummy = 0;
  };
  struct NoRttiImpl {
    template <class T>
    static constexpr inline NoRttiImpl Get() {
      return {&TypeTag<T>::dummy};
    }
    size_t hash_code() const { return reinterpret_cast<uintptr_t>(tag_); }
    std::string name() const { return "<type name missing>"; }
    bool operator==(const NoRttiImpl& other) const {
      return tag_ == other.tag_;
    }
    bool operator<(const NoRttiImpl& other) const { return tag_ < other.tag_; }

    const void* tag_;
  };

#if MEDIAPIPE_HAS_RTTI
  template <class T>
  static const std::type_info& GetTypeInfo() {
    return typeid(T);
  }
  // This implementation uses RTTI, and delegates all operations to
  // std::type_info. In order to support constexpr construction, we don't store
  // a type_info directly (which is not constexpr), but a pointer to a function
  // returning it (which is). This implementation is a bit slower than the
  // others. The only potential advantage would be the ability to match types
  // across multiple dynamic libraries, but we don't support that setup anyway.
  // This is provided for completeness.
  struct FullRttiImpl {
    template <class T>
    static constexpr inline FullRttiImpl Get() {
      return {GetTypeInfo<T>};
    }
    size_t hash_code() const { return get_().hash_code(); }
    std::string name() const { return Demangle(get_().name()); }
    bool operator==(const FullRttiImpl& other) const {
      return get_ == other.get_ || get_() == other.get_();
    }
    bool operator<(const FullRttiImpl& other) const {
      return get_().before(other.get_());
    }

    decltype(&GetTypeInfo<void>) get_;
  };

  // This implementation also stores a pointer to a std::type_info getter
  // function, but it only invokes it to get the type's name. It's equivalent to
  // NoRttiImpl for most operations, but it allows getting the type's name.
  struct FastRttiImpl {
    template <class T>
    static constexpr inline FastRttiImpl Get() {
      return {GetTypeInfo<T>};
    }
    size_t hash_code() const { return reinterpret_cast<uintptr_t>(get_); }
    std::string name() const { return Demangle(get_().name()); }
    bool operator==(const FastRttiImpl& other) const {
      return get_ == other.get_;
    }
    bool operator<(const FastRttiImpl& other) const {
      return reinterpret_cast<uintptr_t>(get_) <
             reinterpret_cast<uintptr_t>(other.get_);
    }

    decltype(&GetTypeInfo<void>) get_;
  };

  using Impl = FastRttiImpl;
#else
  using Impl = NoRttiImpl;
#endif  // MEDIAPIPE_HAS_RTTI
  constexpr explicit TypeId(Impl impl) : impl_(impl) {}

  Impl impl_;
};

template <class T>
static constexpr TypeId kTypeId = TypeId::Of<T>();

namespace tool {

// Helper method that returns a hash code of the given type.
// Superseded by TypeId.
template <typename T>
ABSL_DEPRECATED("Use TypeId directly instead.")
size_t GetTypeHash() {
  return kTypeId<T>.hash_code();
}

}  // namespace tool
}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_TOOL_TYPE_UTIL_H_
