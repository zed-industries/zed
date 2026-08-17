// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_FUNCTIONAL_OVERLOADED_H_
#define BASE_FUNCTIONAL_OVERLOADED_H_

namespace base {

// std::visit() needs to be called with a functor object, such as
//
//  struct Visitor {
//    std::string operator()(const PackageA& source) {
//      return "PackageA";
//    }
//
//    std::string operator()(const PackageB& source) {
//      return "PackageB";
//    }
//  };
//
//  std::variant<PackageA, PackageB> var = PackageA();
//  return std::visit(Visitor(), var);
//
// `Overloaded` enables the above code to be written as:
//
//  std::visit(
//     Overloaded{
//         [](const PackageA& pack) { return "PackageA"; },
//         [](const PackageB& pack) { return "PackageB"; },
//     }, var);
//
// Note: Overloads must be implemented for all the variant options. Otherwise,
// there will be a compilation error.
//
// This struct inherits operator() method from all its base classes.
// Introduces operator() method from all its base classes into its definition.
template <typename... Callables>
struct Overloaded : Callables... {
  using Callables::operator()...;
};

// Uses template argument deduction so that the `Overloaded` struct can be used
// without specifying its template argument. This allows anonymous lambdas
// passed into the `Overloaded` constructor.
template <typename... Callables>
Overloaded(Callables...) -> Overloaded<Callables...>;

}  // namespace base

#endif  // BASE_FUNCTIONAL_OVERLOADED_H_
