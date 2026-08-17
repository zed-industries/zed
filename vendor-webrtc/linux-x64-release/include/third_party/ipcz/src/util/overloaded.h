// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_UTIL_OVERLOADED_H_
#define IPCZ_SRC_UTIL_OVERLOADED_H_

namespace ipcz {

// Overloaded is a template helper for more succint evaluation of std::variant
// values. This allows visitation to be expressed with anonymous lambdas as
// follows:
//
//     std::variant<A, B, C> v = MakeSomeVariant();
//     std::visit(
//         Overloaded{
//             [](const A& v) { return "A"; },
//             [](const B& v) { return "B"; },
//             [](const C& v) { return "C"; },
//         }, v);
//
// Usage requires an expicit case for every possible alternative held by the
// variant.
template <typename... Callables>
struct Overloaded : Callables... {
  using Callables::operator()...;
};

// A deduction guide which allows anonymous lambda types to be deduced from
// constructor arguments.
template <typename... Callables>
Overloaded(Callables...) -> Overloaded<Callables...>;

}  // namespace ipcz

#endif  // IPCZ_SRC_UTIL_OVERLOADED_H_
