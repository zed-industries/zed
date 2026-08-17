// Copyright 2011 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Use std::tuple as tuple type. This file contains helper functions for
// working with std::tuples.
// The functions DispatchToMethod and DispatchToFunction take a function pointer
// or instance and method pointer, and unpack a tuple into arguments to the
// call.
//
// Example usage:
//   // These two methods of creating a Tuple are identical.
//   std::tuple<int, const char*> tuple_a(1, "wee");
//   std::tuple<int, const char*> tuple_b = std::make_tuple(1, "wee");
//
//   void SomeFunc(int a, const char* b) { }
//   DispatchToFunction(&SomeFunc, tuple_a);  // SomeFunc(1, "wee")
//   DispatchToFunction(
//       &SomeFunc, std::make_tuple(10, "foo"));    // SomeFunc(10, "foo")
//
//   struct { void SomeMeth(int a, int b, int c) { } } foo;
//   DispatchToMethod(&foo, &Foo::SomeMeth, std::make_tuple(1, 2, 3));
//   // foo->SomeMeth(1, 2, 3);

#ifndef BASE_TUPLE_H_
#define BASE_TUPLE_H_

#include <stddef.h>

#include <tuple>
#include <utility>

#include "build/build_config.h"

namespace base {

// Dispatchers ----------------------------------------------------------------
//
// Helper functions that call the given method on an object, with the unpacked
// tuple arguments.  Notice that they all have the same number of arguments,
// so you need only write:
//   DispatchToMethod(object, &Object::method, args);
// This is very useful for templated dispatchers, since they don't need to know
// what type |args| is.

// Non-Static Dispatchers with no out params.

template <typename ObjT, typename Method, typename Tuple, size_t... Ns>
inline void DispatchToMethodImpl(const ObjT& obj,
                                 Method method,
                                 Tuple&& args,
                                 std::index_sequence<Ns...>) {
  (obj->*method)(std::get<Ns>(std::forward<Tuple>(args))...);
}

template <typename ObjT, typename Method, typename Tuple>
inline void DispatchToMethod(const ObjT& obj, Method method, Tuple&& args) {
  constexpr size_t size = std::tuple_size_v<std::decay_t<Tuple>>;
  DispatchToMethodImpl(obj, method, std::forward<Tuple>(args),
                       std::make_index_sequence<size>());
}

// Static Dispatchers with no out params.

template <typename Function, typename Tuple, size_t... Ns>
inline void DispatchToFunctionImpl(Function function,
                                   Tuple&& args,
                                   std::index_sequence<Ns...>) {
  (*function)(std::get<Ns>(std::forward<Tuple>(args))...);
}

template <typename Function, typename Tuple>
inline void DispatchToFunction(Function function, Tuple&& args) {
  constexpr size_t size = std::tuple_size_v<std::decay_t<Tuple>>;
  DispatchToFunctionImpl(function, std::forward<Tuple>(args),
                         std::make_index_sequence<size>());
}

// Dispatchers with out parameters.

template <typename ObjT,
          typename Method,
          typename InTuple,
          typename OutTuple,
          size_t... InNs,
          size_t... OutNs>
inline void DispatchToMethodImpl(const ObjT& obj,
                                 Method method,
                                 InTuple&& in,
                                 OutTuple* out,
                                 std::index_sequence<InNs...>,
                                 std::index_sequence<OutNs...>) {
  (obj->*method)(std::get<InNs>(std::forward<InTuple>(in))...,
                 &std::get<OutNs>(*out)...);
}

template <typename ObjT, typename Method, typename InTuple, typename OutTuple>
inline void DispatchToMethod(const ObjT& obj,
                             Method method,
                             InTuple&& in,
                             OutTuple* out) {
  constexpr size_t in_size = std::tuple_size_v<std::decay_t<InTuple>>;
  constexpr size_t out_size = std::tuple_size_v<OutTuple>;
  DispatchToMethodImpl(obj, method, std::forward<InTuple>(in), out,
                       std::make_index_sequence<in_size>(),
                       std::make_index_sequence<out_size>());
}

}  // namespace base

#endif  // BASE_TUPLE_H_
