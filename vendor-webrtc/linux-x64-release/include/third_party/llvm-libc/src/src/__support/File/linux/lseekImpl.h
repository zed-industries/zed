//===-- Linux implementation of lseek -------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC___SUPPORT_FILE_LINUX_LSEEKIMPL_H
#define LLVM_LIBC_SRC___SUPPORT_FILE_LINUX_LSEEKIMPL_H

#include "hdr/types/off_t.h"
#include "src/__support/OSUtil/syscall.h" // For internal syscall function.
#include "src/__support/common.h"
#include "src/__support/error_or.h"
#include "src/__support/macros/config.h"
#include "src/errno/libc_errno.h"

#include <stdint.h>      // For uint64_t.
#include <sys/syscall.h> // For syscall numbers.

namespace LIBC_NAMESPACE_DECL {
namespace internal {

LIBC_INLINE ErrorOr<off_t> lseekimpl(int fd, off_t offset, int whence) {
  off_t result;
#ifdef SYS_lseek
  int ret = LIBC_NAMESPACE::syscall_impl<int>(SYS_lseek, fd, offset, whence);
  result = ret;
#elif defined(SYS_llseek) || defined(SYS__llseek)
  static_assert(sizeof(size_t) == 4, "size_t must be 32 bits.");
#ifdef SYS_llseek
  constexpr long LLSEEK_SYSCALL_NO = SYS_llseek;
#elif defined(SYS__llseek)
  constexpr long LLSEEK_SYSCALL_NO = SYS__llseek;
#endif
  off_t offset_64 = offset;
  int ret = LIBC_NAMESPACE::syscall_impl<int>(
      LLSEEK_SYSCALL_NO, fd, offset_64 >> 32, offset_64, &result, whence);
#else
#error "lseek, llseek and _llseek syscalls not available."
#endif
  if (ret < 0)
    return Error(-ret);
  return result;
}

} // namespace internal
} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC___SUPPORT_FILE_LINUX_LSEEKIMPL_H
