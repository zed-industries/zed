// Copyright 2021 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LIB_PROMISE_ARENA_PROMISE_H
#define GRPC_SRC_CORE_LIB_PROMISE_ARENA_PROMISE_H

#include <grpc/support/port_platform.h>
#include <stdlib.h>

#include <cstddef>
#include <memory>
#include <type_traits>
#include <utility>

#include "absl/meta/type_traits.h"
#include "src/core/lib/promise/context.h"
#include "src/core/lib/promise/poll.h"
#include "src/core/lib/resource_quota/arena.h"
#include "src/core/util/construct_destruct.h"

namespace grpc_core {

namespace arena_promise_detail {

struct ArgType {
  alignas(std::max_align_t) char buffer[sizeof(void*)];
};

template <typename T>
T*& ArgAsPtr(ArgType* arg) {
  static_assert(sizeof(ArgType) >= sizeof(T**),
                "Must have ArgType of at least one pointer size");
  return *reinterpret_cast<T**>(arg);
}

template <typename T>
struct Vtable {
  // Poll the promise, once.
  Poll<T> (*poll_once)(ArgType* arg);
  // Destroy the underlying callable object if there is one.
  // Since we don't delete (the arena owns the memory) but we may need to call a
  // destructor, we expose this for when the ArenaPromise object is destroyed.
  void (*destroy)(ArgType* arg);
};

template <typename T>
struct VtableAndArg {
  const Vtable<T>* vtable;
  ArgType arg;
};

// Implementation of Vtable for an empty object.
// Used when an empty ArenaPromise is created, or when the ArenaPromise is moved
// from. Since in either case these objects should not be polled, we simply
// crash if it is.
template <typename T>
struct Null {
  static const Vtable<T> vtable;

  static Poll<T> PollOnce(ArgType*) {
    abort();
    GPR_UNREACHABLE_CODE(return Pending{});
  }

  static void Destroy(ArgType*) {}
};

template <typename T>
const Vtable<T> Null<T>::vtable = {PollOnce, Destroy};

// Implementation of ImplInterface for a callable object.
template <typename T, typename Callable>
struct AllocatedCallable {
  static const Vtable<T> vtable;

  static Poll<T> PollOnce(ArgType* arg) {
    return poll_cast<T>((*ArgAsPtr<Callable>(arg))());
  }

  static void Destroy(ArgType* arg) { Destruct(ArgAsPtr<Callable>(arg)); }
};

template <typename T, typename Callable>
const Vtable<T> AllocatedCallable<T, Callable>::vtable = {PollOnce, Destroy};

// Implementation of ImplInterface for a small callable object (one that fits
// within the ArgType arg)
template <typename T, typename Callable>
struct Inlined {
  static const Vtable<T> vtable;

  static Poll<T> PollOnce(ArgType* arg) {
    return poll_cast<T>((*reinterpret_cast<Callable*>(arg))());
  }

  static void Destroy(ArgType* arg) {
    Destruct(reinterpret_cast<Callable*>(arg));
  }
};

template <typename T, typename Callable>
const Vtable<T> Inlined<T, Callable>::vtable = {PollOnce, Destroy};

// If a callable object is empty we can substitute any instance of that callable
// for the one we call (for how could we tell the difference)?
// Since this corresponds to a lambda with no fields, and we expect these to be
// reasonably common, we can elide the arena allocation entirely and simply poll
// a global shared instance.
// (this comes up often when the promise only accesses context data from the
// containing activity).
template <typename T, typename Callable>
struct SharedCallable {
  static const Vtable<T> vtable;

  static Poll<T> PollOnce(ArgType* arg) {
    return (*reinterpret_cast<Callable*>(arg))();
  }
};

template <typename T, typename Callable>
const Vtable<T> SharedCallable<T, Callable>::vtable = {PollOnce,
                                                       Null<T>::Destroy};

// Redirector type: given a callable type, expose a Make() function that creates
// the appropriate underlying implementation.
template <typename T, typename Callable, typename Ignored = void>
struct ChooseImplForCallable;

template <typename T, typename Callable>
struct ChooseImplForCallable<
    T, Callable,
    absl::enable_if_t<!std::is_empty<Callable>::value &&
                      (sizeof(Callable) > sizeof(ArgType))>> {
  static void Make(Callable&& callable, VtableAndArg<T>* out) {
    out->vtable = &AllocatedCallable<T, Callable>::vtable;
    ArgAsPtr<Callable>(&out->arg) = GetContext<Arena>()->template New<Callable>(
        std::forward<Callable>(callable));
  }
};

template <typename T, typename Callable>
struct ChooseImplForCallable<
    T, Callable,
    absl::enable_if_t<!std::is_empty<Callable>::value &&
                      (sizeof(Callable) <= sizeof(ArgType))>> {
  static void Make(Callable&& callable, VtableAndArg<T>* out) {
    out->vtable = &Inlined<T, Callable>::vtable;
    Construct(reinterpret_cast<Callable*>(&out->arg),
              std::forward<Callable>(callable));
  }
};

template <typename T, typename Callable>
struct ChooseImplForCallable<
    T, Callable, absl::enable_if_t<std::is_empty<Callable>::value>> {
  static void Make(Callable&&, VtableAndArg<T>* out) {
    out->vtable = &SharedCallable<T, Callable>::vtable;
  }
};

// Wrap ChooseImplForCallable with a friend approachable syntax.
template <typename T, typename Callable>
void MakeImplForCallable(Callable&& callable, VtableAndArg<T>* out) {
  ChooseImplForCallable<T, Callable>::Make(std::forward<Callable>(callable),
                                           out);
}

}  // namespace arena_promise_detail

// A promise for which the state memory is allocated from an arena.
template <typename T>
class ArenaPromise {
 public:
  // Construct an empty, uncallable, invalid ArenaPromise.
  ArenaPromise() = default;

  // Construct an ArenaPromise that will call the given callable when polled.
  template <typename Callable,
            typename Ignored =
                absl::enable_if_t<!std::is_same<Callable, ArenaPromise>::value>>
  // NOLINTNEXTLINE(google-explicit-constructor)
  ArenaPromise(Callable&& callable) {
    arena_promise_detail::MakeImplForCallable(std::forward<Callable>(callable),
                                              &vtable_and_arg_);
  }

  // ArenaPromise is not copyable.
  ArenaPromise(const ArenaPromise&) = delete;
  ArenaPromise& operator=(const ArenaPromise&) = delete;
  // ArenaPromise is movable.
  ArenaPromise(ArenaPromise&& other) noexcept
      : vtable_and_arg_(other.vtable_and_arg_) {
    other.vtable_and_arg_.vtable = &arena_promise_detail::Null<T>::vtable;
  }
  ArenaPromise& operator=(ArenaPromise&& other) noexcept {
    vtable_and_arg_.vtable->destroy(&vtable_and_arg_.arg);
    vtable_and_arg_ = other.vtable_and_arg_;
    other.vtable_and_arg_.vtable = &arena_promise_detail::Null<T>::vtable;
    return *this;
  }

  // Destruction => call Destroy on the underlying impl object.
  ~ArenaPromise() { vtable_and_arg_.vtable->destroy(&vtable_and_arg_.arg); }

  // Expose the promise interface: a call operator that returns Poll<T>.
  Poll<T> operator()() {
    return vtable_and_arg_.vtable->poll_once(&vtable_and_arg_.arg);
  }

  bool has_value() const {
    return vtable_and_arg_.vtable != &arena_promise_detail::Null<T>::vtable;
  }

 private:
  // Underlying impl object.
  arena_promise_detail::VtableAndArg<T> vtable_and_arg_ = {
      &arena_promise_detail::Null<T>::vtable, {}};
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_PROMISE_ARENA_PROMISE_H
