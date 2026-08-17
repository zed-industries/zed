//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef SUPPORT_UNWRAP_CONTAINER_ADAPTOR_H
#define SUPPORT_UNWRAP_CONTAINER_ADAPTOR_H

// Allows accessing the underlying container of the given adaptor.
template <class Adaptor>
struct UnwrapAdaptor : Adaptor {
  UnwrapAdaptor() = default;

  UnwrapAdaptor(Adaptor&& adaptor) : Adaptor(std::move(adaptor)) {}
  // `c` is a protected member variable of the base class.
  decltype(auto) get_container() {
    return (UnwrapAdaptor::c); // Put into parentheses to make sure the function returns a reference.
  }

  // TODO: make this work pre-C++20.
  decltype(auto) get_comparator()
  requires requires {
    UnwrapAdaptor::c;
  } {
    return (UnwrapAdaptor::comp); // Put into parentheses to make sure the function returns a reference.
  }
};

#endif // SUPPORT_UNWRAP_CONTAINER_ADAPTOR_H
