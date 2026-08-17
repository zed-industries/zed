// Copyright 2022 gRPC authors.
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

#ifndef GRPC_SRC_CORE_UTIL_NO_DESTRUCT_H
#define GRPC_SRC_CORE_UTIL_NO_DESTRUCT_H

#include <grpc/support/port_platform.h>

#include <type_traits>
#include <utility>

#include "src/core/util/construct_destruct.h"

namespace grpc_core {

// NoDestruct<T> is a wrapper around an object of type T that:
// - stores the value inline - no heap allocation
// - is non-copyable
// - is eagerly constructed (i.e. the constructor is called when NoDestruct is
//   constructed)
// - *NEVER* calls ~T()
// It's useful in cases where no ordering can be assumed between destructors of
// objects that need to refer to each other - such as at program destruction
// time.
// Examples:
//  // globally available object:
//  static NoDestruct<Foo> g_foo(1, "foo", 2.0);
//  // used as:
//  g_foo->DoSomething();
//  // singleton function:
//  Bar* BarSingleton() {
//   static NoDestruct<Bar> bar(1, "bar", 2.0);
//   return &*bar;
//  }
// The globally available version is constructed at program startup, and the
// singleton version is constructed at the first call to BarSingleton().
// Neither Foo nor Bar instance will be destructed.
template <typename T>
class NoDestruct {
 public:
  template <typename... Args>
  explicit NoDestruct(Args&&... args) {
    static_assert(std::is_trivially_destructible<NoDestruct<T>>::value,
                  "NoDestruct must be trivially destructible");
    Construct(reinterpret_cast<T*>(&space_), std::forward<Args>(args)...);
  }
  NoDestruct(const NoDestruct&) = delete;
  NoDestruct& operator=(const NoDestruct&) = delete;
  ~NoDestruct() = default;

  T* operator->() { return get(); }
  const T* operator->() const { return get(); }
  T& operator*() { return *get(); }
  const T& operator*() const { return *get(); }

  T* get() { return reinterpret_cast<T*>(&space_); }
  const T* get() const { return reinterpret_cast<const T*>(&space_); }

 private:
  alignas(T) char space_[sizeof(T)];
};

// Helper for when a program desires a single *process wide* instance of a
// default constructed T to be always available.
// The instance is constructed eagerly at program startup, so it's essentially
// free to load the pointer to the instance.
template <typename T>
class NoDestructSingleton {
 public:
  static T* Get() { return &*value_; }

 private:
  NoDestructSingleton() = delete;
  ~NoDestructSingleton() = delete;

  static NoDestruct<T> value_;
};

template <typename T>
NoDestruct<T> NoDestructSingleton<T>::value_;

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_UTIL_NO_DESTRUCT_H
