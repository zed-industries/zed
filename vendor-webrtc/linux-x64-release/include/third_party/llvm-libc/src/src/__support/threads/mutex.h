//===--- A platform independent abstraction layer for mutexes ---*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC___SUPPORT_THREADS_MUTEX_H
#define LLVM_LIBC_SRC___SUPPORT_THREADS_MUTEX_H

#include "src/__support/macros/properties/architectures.h"

// Platform independent code will include this header file which pulls
// the platfrom specific specializations using platform macros.
//
// The platform specific specializations should define a class by name
// Mutex with non-static methods having the following signature:
//
// MutexError lock();
// MutexError trylock();
// MutexError timedlock(...);
// MutexError unlock();
// MutexError reset(); // Used to reset inconsistent robust mutexes.
//
// Apart from the above non-static methods, the specializations should
// also provide few static methods with the following signature:
//
// static MutexError init(mtx_t *);
// static MutexError destroy(mtx_t *);
//
// All of the static and non-static methods should ideally be implemented
// as inline functions so that implementations of public functions can
// call them without a function call overhead.
//
// Another point to keep in mind that is that the libc internally needs a
// few global locks. So, to avoid static initialization order fiasco, we
// want the constructors of the Mutex classes to be constexprs.

#if defined(__linux__)
#include "src/__support/threads/linux/mutex.h"
#elif defined(LIBC_TARGET_ARCH_IS_GPU)
#include "src/__support/threads/gpu/mutex.h"
#endif // __linux__

#endif // LLVM_LIBC_SRC___SUPPORT_THREADS_MUTEX_H
