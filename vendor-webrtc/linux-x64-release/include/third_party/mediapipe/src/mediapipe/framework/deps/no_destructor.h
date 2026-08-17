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

#ifndef MEDIAPIPE_DEPS_NO_DESTRUCTOR_H_
#define MEDIAPIPE_DEPS_NO_DESTRUCTOR_H_

#include <type_traits>
#include <utility>

namespace mediapipe {

// NoDestructor<T> is a wrapper around an object of type T that
// * stores the object of type T inline inside NoDestructor<T>
// * eagerly forwards constructor arguments to it (i.e. acts like T in terms
//   of construction)
// * provides access to the object of type T like a pointer via ->, *, and get()
//   (note that const NoDestructor<T> works like a pointer to const T)
// * never calls T's destructor for the object
//   (hence NoDestructor<T> objects created on the stack or as member variables
//   will lead to memory and/or resource leaks)
//
// One key use case of NoDestructor (which in itself is not lazy) is optimizing
// the following pattern of safe on-demand construction of an object with
// non-trivial constructor in static storage without destruction ever happening:
//   const std::string& MyString() {
//     static std::string* x = new std::string("foo");  // note the "static"
//     return *x;
//   }
// By using NoDestructor we do not need to involve heap allocation and
// corresponding pointer following (and hence extra CPU cache usage/needs)
// on each access:
//   const std::string& MyString() {
//     static NoDestructor<std::string> x("foo");
//     return *x;
//   }
// Since C++11 this static-in-a-function pattern results in exactly-once,
// thread-safe, on-demand construction of an object, and very fast access
// thereafter (the cost is a few extra cycles).
// NoDestructor makes accesses even faster by storing the object inline in
// static storage.
//
// Note that:
// * Since destructor is never called, the object lives on during program exit
//   and can be safely accessed by any threads that have not been joined.
// * This static-in-a-function NoDestructor usage pattern should be preferred
//   to uses of gtl::LazyStaticPtr in new code.
//
// Also note that
//   static NoDestructor<NonPOD> ptr(whatever);
// can safely replace
//   static NonPOD* ptr = new NonPOD(whatever);
// or
//   static NonPOD obj(whatever);
// at file-level scope when the safe static-in-a-function pattern is infeasible
// to use for some good reason.
// All three of the NonPOD patterns above suffer from the same issue that
// initialization of that object happens non-thread-safely at
// a globally-undefined point during initialization of static-storage objects,
// but NoDestructor<> usage provides both the safety of having the object alive
// during program exit sequence and the performance of not doing extra memory
// dereference on access.
//
template <typename T>
class NoDestructor {
 public:
  typedef T element_type;

  // Forwards arguments to the T's constructor: calls T(args...).
  template <typename... Ts,
            // Disable this overload when it might collide with copy/move.
            typename std::enable_if<
                !std::is_same<void(typename std::decay<Ts>::type...),
                              void(NoDestructor)>::value,
                int>::type = 0>
  explicit NoDestructor(Ts&&... args) {
    new (space_) T(std::forward<Ts>(args)...);
  }

  // Forwards copy and move construction for T. Enables usage like this:
  //   static NoDestructor<std::array<std::string, 3>> x{{{"1", "2", "3"}}};
  //   static NoDestructor<std::vector<int>> x{{1, 2, 3}};
  explicit NoDestructor(const T& x) { new (space_) T(x); }
  explicit NoDestructor(T&& x) { new (space_) T(std::move(x)); }

  // No copying.
  NoDestructor(const NoDestructor&) = delete;
  NoDestructor& operator=(const NoDestructor&) = delete;

  // Pretend to be a smart pointer to T with deep constness.
  // Never returns a null pointer.
  T& operator*() { return *get(); }
  T* operator->() { return get(); }
  T* get() { return reinterpret_cast<T*>(space_); }
  const T& operator*() const { return *get(); }
  const T* operator->() const { return get(); }
  const T* get() const { return reinterpret_cast<const T*>(space_); }

 private:
  alignas(T) char space_[sizeof(T)];
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_DEPS_NO_DESTRUCTOR_H_
