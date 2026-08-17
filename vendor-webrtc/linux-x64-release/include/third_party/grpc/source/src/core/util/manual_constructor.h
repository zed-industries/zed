//
//
// Copyright 2016 gRPC authors.
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
//

#ifndef GRPC_SRC_CORE_UTIL_MANUAL_CONSTRUCTOR_H
#define GRPC_SRC_CORE_UTIL_MANUAL_CONSTRUCTOR_H

// manually construct a region of memory with some type

#include <grpc/support/port_platform.h>
#include <stddef.h>

#include <utility>

#include "src/core/util/construct_destruct.h"

namespace grpc_core {

// this contains templated helpers needed to implement the ManualConstructors
// in this file.
namespace manual_ctor_impl {

// is_one_of returns true it a class, Member, is present in a variadic list of
// classes, List.
template <class Member, class... List>
class is_one_of;

template <class Member, class... List>
class is_one_of<Member, Member, List...> {
 public:
  static constexpr const bool value = true;
};

template <class Member, class A, class... List>
class is_one_of<Member, A, List...> {
 public:
  static constexpr const bool value = is_one_of<Member, List...>::value;
};

template <class Member>
class is_one_of<Member> {
 public:
  static constexpr const bool value = false;
};

// max_size_of returns sizeof(Type) for the largest type in the variadic list
// of classes, Types.
template <class... Types>
class max_size_of;

template <class A>
class max_size_of<A> {
 public:
  static constexpr const size_t value = sizeof(A);
};

template <class A, class... B>
class max_size_of<A, B...> {
 public:
  static constexpr const size_t value = sizeof(A) > max_size_of<B...>::value
                                            ? sizeof(A)
                                            : max_size_of<B...>::value;
};

// max_size_of returns alignof(Type) for the largest type in the variadic list
// of classes, Types.
template <class... Types>
class max_align_of;

template <class A>
class max_align_of<A> {
 public:
  static constexpr const size_t value = alignof(A);
};

template <class A, class... B>
class max_align_of<A, B...> {
 public:
  static constexpr const size_t value = alignof(A) > max_align_of<B...>::value
                                            ? alignof(A)
                                            : max_align_of<B...>::value;
};

}  // namespace manual_ctor_impl

template <typename Type>
class ManualConstructor {
 public:
  // No constructor or destructor because one of the most useful uses of
  // this class is as part of a union, and members of a union could not have
  // constructors or destructors till C++11.  And, anyway, the whole point of
  // this class is to bypass constructor and destructor.

  Type* get() { return reinterpret_cast<Type*>(&space_); }
  const Type* get() const { return reinterpret_cast<const Type*>(&space_); }

  Type* operator->() { return get(); }
  const Type* operator->() const { return get(); }

  Type& operator*() { return *get(); }
  const Type& operator*() const { return *get(); }

  void Init() { Construct(get()); }

  // Init() constructs the Type instance using the given arguments
  // (which are forwarded to Type's constructor).
  //
  // Note that Init() with no arguments performs default-initialization,
  // not zero-initialization (i.e it behaves the same as "new Type;", not
  // "new Type();"), so it will leave non-class types uninitialized.
  template <typename... Ts>
  void Init(Ts&&... args) {
    Construct(get(), std::forward<Ts>(args)...);
  }

  // Init() that is equivalent to copy and move construction.
  // Enables usage like this:
  //   ManualConstructor<std::vector<int>> v;
  //   v.Init({1, 2, 3});
  void Init(const Type& x) { Construct(get(), x); }
  void Init(Type&& x) { Construct(get(), std::forward<Type>(x)); }

  void Destroy() { Destruct(get()); }

 private:
  alignas(Type) char space_[sizeof(Type)];
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_UTIL_MANUAL_CONSTRUCTOR_H
