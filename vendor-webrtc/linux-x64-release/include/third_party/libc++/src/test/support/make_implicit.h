//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef SUPPORT_MAKE_IMPLICIT_H
#define SUPPORT_MAKE_IMPLICIT_H

// "make_implicit<Tp>(Args&&... args)" is a function to construct 'Tp'
// from 'Args...' using implicit construction.

#include <utility>

template <class T, class... Args>
T make_implicit(Args&&... args) {
  return {std::forward<Args>(args)...};
}

#endif // SUPPORT_MAKE_IMPLICIT_H
