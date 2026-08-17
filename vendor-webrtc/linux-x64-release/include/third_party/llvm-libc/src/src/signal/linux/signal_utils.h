//===-- Internal header for Linux signals -----------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_SIGNAL_LINUX_SIGNAL_UTILS_H
#define LLVM_LIBC_SRC_SIGNAL_LINUX_SIGNAL_UTILS_H

#include "hdr/types/sigset_t.h"
#include "src/__support/OSUtil/syscall.h" // For internal syscall function.
#include "src/__support/common.h"
#include "src/__support/macros/config.h"

#include <signal.h> // sigaction
#include <stddef.h>
#include <sys/syscall.h>          // For syscall numbers.

namespace LIBC_NAMESPACE_DECL {

// The POSIX definition of struct sigaction and the sigaction data structure
// expected by the rt_sigaction syscall differ in their definition. So, we
// define the equivalent of the what the kernel expects to help with making
// the rt_sigaction syscall.
//
// NOTE: Though the kernel definition does not have a union to include the
// handler taking siginfo_t * argument, one can set sa_handler to sa_sigaction
// if SA_SIGINFO is set in sa_flags.
struct KernelSigaction {
  LIBC_INLINE KernelSigaction &operator=(const struct sigaction &sa) {
    sa_flags = sa.sa_flags;
    sa_restorer = sa.sa_restorer;
    sa_mask = sa.sa_mask;
    if (sa_flags & SA_SIGINFO) {
      sa_sigaction = sa.sa_sigaction;
    } else {
      sa_handler = sa.sa_handler;
    }
    return *this;
  }

  LIBC_INLINE operator struct sigaction() const {
    struct sigaction sa;
    sa.sa_flags = static_cast<int>(sa_flags);
    sa.sa_mask = sa_mask;
    sa.sa_restorer = sa_restorer;
    if (sa_flags & SA_SIGINFO)
      sa.sa_sigaction = sa_sigaction;
    else
      sa.sa_handler = sa_handler;
    return sa;
  }

  union {
    void (*sa_handler)(int);
    void (*sa_sigaction)(int, siginfo_t *, void *);
  };
  unsigned long sa_flags;
  void (*sa_restorer)(void);
  // Our public definition of sigset_t matches that of the kernel's definition.
  // So, we can use the public sigset_t type here.
  sigset_t sa_mask;
};

static constexpr size_t BITS_PER_SIGWORD = sizeof(unsigned long) * 8;

LIBC_INLINE constexpr sigset_t full_set() { return sigset_t{{-1UL}}; }

LIBC_INLINE constexpr sigset_t empty_set() { return sigset_t{{0}}; }

// Set the bit corresponding to |signal| in |set|. Return true on success
// and false on failure. The function will fail if |signal| is greater than
// NSIG or negative.
LIBC_INLINE constexpr bool add_signal(sigset_t &set, int signal) {
  if (signal > NSIG || signal <= 0)
    return false;
  size_t n = size_t(signal) - 1;
  size_t word = n / BITS_PER_SIGWORD;
  size_t bit = n % BITS_PER_SIGWORD;
  set.__signals[word] |= (1UL << bit);
  return true;
}

// Reset the bit corresponding to |signal| in |set|. Return true on success
// and false on failure. The function will fail if |signal| is greater than
// NSIG or negative.
LIBC_INLINE constexpr bool delete_signal(sigset_t &set, int signal) {
  if (signal > NSIG || signal <= 0)
    return false;
  size_t n = size_t(signal) - 1;
  size_t word = n / BITS_PER_SIGWORD;
  size_t bit = n % BITS_PER_SIGWORD;
  set.__signals[word] &= ~(1UL << bit);
  return true;
}

LIBC_INLINE int block_all_signals(sigset_t &set) {
  sigset_t full = full_set();
  return LIBC_NAMESPACE::syscall_impl<int>(SYS_rt_sigprocmask, SIG_BLOCK, &full,
                                           &set, sizeof(sigset_t));
}

LIBC_INLINE int restore_signals(const sigset_t &set) {
  return LIBC_NAMESPACE::syscall_impl<int>(SYS_rt_sigprocmask, SIG_SETMASK,
                                           &set, nullptr, sizeof(sigset_t));
}

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_SIGNAL_LINUX_SIGNAL_UTILS_H
