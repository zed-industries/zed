//===-- String to integer conversion utils ----------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_SYS_WAIT_WAIT4IMPL_H
#define LLVM_LIBC_SRC_SYS_WAIT_WAIT4IMPL_H

#include "src/__support/OSUtil/syscall.h" // For internal syscall function.
#include "src/__support/common.h"
#include "src/__support/error_or.h"
#include "src/__support/macros/config.h"
#include "src/errno/libc_errno.h"

#include <signal.h>
#include <sys/syscall.h> // For syscall numbers.
#include <sys/wait.h>

namespace LIBC_NAMESPACE_DECL {
namespace internal {

// The implementation of wait here is very minimal. We will add more
// functionality and standard compliance in future.

LIBC_INLINE ErrorOr<pid_t> wait4impl(pid_t pid, int *wait_status, int options,
                                     struct rusage *usage) {
#if SYS_wait4
  pid = LIBC_NAMESPACE::syscall_impl<pid_t>(SYS_wait4, pid, wait_status,
                                            options, usage);
#elif defined(SYS_waitid)
  int idtype = P_PID;
  if (pid == -1) {
    idtype = P_ALL;
  } else if (pid < -1) {
    idtype = P_PGID;
    pid *= -1;
  } else if (pid == 0) {
    idtype = P_PGID;
  }

  options |= WEXITED;

  siginfo_t info;
  pid = LIBC_NAMESPACE::syscall_impl<pid_t>(SYS_waitid, idtype, pid, &info,
                                            options, usage);
  if (pid >= 0)
    pid = info.si_pid;

  if (wait_status) {
    switch (info.si_code) {
    case CLD_EXITED:
      *wait_status = W_EXITCODE(info.si_status, 0);
      break;
    case CLD_DUMPED:
      *wait_status = info.si_status | WCOREFLAG;
      break;
    case CLD_KILLED:
      *wait_status = info.si_status;
      break;
    case CLD_TRAPPED:
    case CLD_STOPPED:
      *wait_status = W_STOPCODE(info.si_status);
      break;
    case CLD_CONTINUED:
      // Set wait_status to a value that the caller can check via WIFCONTINUED.
      // glibc has a non-POSIX macro definition __W_CONTINUED for this value.
      *wait_status = 0xffff;
      break;
    default:
      *wait_status = 0;
      break;
    }
  }
#else
#error "wait4 and waitid syscalls not available."
#endif
  if (pid < 0)
    return Error(-pid);
  return pid;
}

} // namespace internal
} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_SYS_WAIT_WAIT4IMPL_H
