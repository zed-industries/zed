//===-- Convert Statfs to Statvfs -------------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_SYS_STATVFS_LINUX_STATFS_TO_STATVFS_H
#define LLVM_LIBC_SRC_SYS_STATVFS_LINUX_STATFS_TO_STATVFS_H

#include "include/llvm-libc-types/struct_statvfs.h"
#include "src/__support/CPP/optional.h"
#include "src/__support/OSUtil/syscall.h"
#include "src/__support/macros/attributes.h"
#include "src/__support/macros/config.h"
#include "src/errno/libc_errno.h"
#include <asm/statfs.h>
#include <sys/syscall.h>
namespace LIBC_NAMESPACE_DECL {

namespace statfs_utils {
#ifdef SYS_statfs64
using LinuxStatFs = statfs64;
#else
using LinuxStatFs = statfs;
#endif

// Linux kernel set an additional flag to f_flags. Libc should mask it out.
LIBC_INLINE_VAR constexpr decltype(LinuxStatFs::f_flags) ST_VALID = 0x0020;

LIBC_INLINE cpp::optional<LinuxStatFs> linux_statfs(const char *path) {
  // The kernel syscall routine checks the validity of the path before filling
  // the statfs structure. So, it is possible that the result is not initialized
  // after the syscall. Since the struct is trvial, the compiler will generate
  // pattern filling for the struct.
  LinuxStatFs result;
  // On 32-bit platforms, original statfs cannot handle large file systems.
  // In such cases, SYS_statfs64 is defined and should be used.
#ifdef SYS_statfs64
  int ret = syscall_impl<int>(SYS_statfs64, path, sizeof(result), &result);
#else
  int ret = syscall_impl<int>(SYS_statfs, path, &result);
#endif
  if (ret < 0) {
    libc_errno = -ret;
    return cpp::nullopt;
  }
  result.f_flags &= ~ST_VALID;
  return result;
}

LIBC_INLINE cpp::optional<LinuxStatFs> linux_fstatfs(int fd) {
  // The kernel syscall routine checks the validity of the path before filling
  // the statfs structure. So, it is possible that the result is not initialized
  // after the syscall. Since the struct is trvial, the compiler will generate
  // pattern filling for the struct.
  LinuxStatFs result;
  // On 32-bit platforms, original fstatfs cannot handle large file systems.
  // In such cases, SYS_fstatfs64 is defined and should be used.
#ifdef SYS_fstatfs64
  int ret = syscall_impl<int>(SYS_fstatfs64, fd, sizeof(result), &result);
#else
  int ret = syscall_impl<int>(SYS_fstatfs, fd, &result);
#endif
  if (ret < 0) {
    libc_errno = -ret;
    return cpp::nullopt;
  }
  result.f_flags &= ~ST_VALID;
  return result;
}

// must use 'struct' tag to refer to type 'statvfs' in this scope. There will be
// a function in the same namespace with the same name. For consistency, we use
// struct prefix for all statvfs/statfs related types.
LIBC_INLINE struct statvfs statfs_to_statvfs(const LinuxStatFs &in) {
  struct statvfs out;
  out.f_bsize = in.f_bsize;
  out.f_frsize = in.f_frsize;
  out.f_blocks = static_cast<decltype(out.f_blocks)>(in.f_blocks);
  out.f_bfree = static_cast<decltype(out.f_bfree)>(in.f_bfree);
  out.f_bavail = static_cast<decltype(out.f_bavail)>(in.f_bavail);
  out.f_files = static_cast<decltype(out.f_files)>(in.f_files);
  out.f_ffree = static_cast<decltype(out.f_ffree)>(in.f_ffree);
  out.f_favail = static_cast<decltype(out.f_favail)>(in.f_ffree);
  out.f_fsid = in.f_fsid.val[0];
  if constexpr (sizeof(decltype(out.f_fsid)) == sizeof(uint64_t))
    out.f_fsid |= static_cast<decltype(out.f_fsid)>(in.f_fsid.val[1]) << 32;
  out.f_flag = in.f_flags;
  out.f_namemax = in.f_namelen;
  return out;
}
} // namespace statfs_utils
} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_SYS_STATVFS_LINUX_STATFS_TO_STATVFS_H
