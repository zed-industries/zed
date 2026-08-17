//===-- Wrapper over SYS_statx syscall ------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_SYS_STAT_LINUX_KERNEL_STATX_H
#define LLVM_LIBC_SRC_SYS_STAT_LINUX_KERNEL_STATX_H

#include "src/__support/OSUtil/syscall.h" // For internal syscall function.
#include "src/__support/common.h"
#include "src/__support/macros/config.h"

#include <stdint.h>
#include <sys/stat.h>
#include <sys/syscall.h> // For syscall numbers.

// It is safe to include this kernel header as it is designed to be
// included from user programs without causing any name pollution.
#include <linux/kdev_t.h>

namespace {

// The type definitions in the internal namespace match kernel's definition of
// the statx_timestamp and statx types in linux/stat.h. We define equivalent
// types here instead of including that header file to avoid name mixup between
// linux/stat.h and the libc's stat.h.
struct statx_timestamp {
  int64_t tv_sec;
  uint32_t tv_nsec;
  int32_t __reserved;
};

struct statx_buf {
  uint32_t stx_mask;       // What results were written
  uint32_t stx_blksize;    // Preferred general I/O size
  uint64_t stx_attributes; // Flags conveying information about the file
  uint32_t stx_nlink;      // Number of hard links
  uint32_t stx_uid;        // User ID of owner
  uint32_t stx_gid;        // Group ID of owner
  uint16_t stx_mode;       // File mode
  uint16_t __spare0[1];
  uint64_t stx_ino;                 // Inode number
  uint64_t stx_size;                // File size
  uint64_t stx_blocks;              // Number of 512-byte blocks allocated
  uint64_t stx_attributes_mask;     // Mask to show what's supported in
                                    // stx_attributes
  struct statx_timestamp stx_atime; // Last access time
  struct statx_timestamp stx_btime; // File creation time
  struct statx_timestamp stx_ctime; // Last attribute change time
  struct statx_timestamp stx_mtime; // Last data modification time
  uint32_t stx_rdev_major;          // Device ID of special file
  uint32_t stx_rdev_minor;
  uint32_t stx_dev_major; // ID of device containing file
  uint32_t stx_dev_minor;
  uint64_t stx_mnt_id;
  uint64_t __spare2;
  uint64_t __spare3[12]; // Spare space for future expansion
};

// The below mask value is based on the definition of a similarly
// named macro in linux/stat.h. When this flag is passed for the
// mask argument to the statx syscall, all fields except the
// stx_btime field will be filled in.
constexpr unsigned int STATX_BASIC_STATS_MASK = 0x7FF;

} // Anonymous namespace

namespace LIBC_NAMESPACE_DECL {

LIBC_INLINE int statx(int dirfd, const char *__restrict path, int flags,
                      struct stat *__restrict statbuf) {
  // We make a statx syscall and copy out the result into the |statbuf|.
  ::statx_buf xbuf;
  int ret = LIBC_NAMESPACE::syscall_impl<int>(SYS_statx, dirfd, path, flags,
                                              ::STATX_BASIC_STATS_MASK, &xbuf);
  if (ret < 0)
    return -ret;

  statbuf->st_dev = MKDEV(xbuf.stx_dev_major, xbuf.stx_dev_minor);
  statbuf->st_ino = static_cast<decltype(statbuf->st_ino)>(xbuf.stx_ino);
  statbuf->st_mode = xbuf.stx_mode;
  statbuf->st_nlink = xbuf.stx_nlink;
  statbuf->st_uid = xbuf.stx_uid;
  statbuf->st_gid = xbuf.stx_gid;
  statbuf->st_rdev = MKDEV(xbuf.stx_rdev_major, xbuf.stx_rdev_minor);
  statbuf->st_size = xbuf.stx_size;
  statbuf->st_atim.tv_sec = xbuf.stx_atime.tv_sec;
  statbuf->st_atim.tv_nsec = xbuf.stx_atime.tv_nsec;
  statbuf->st_mtim.tv_sec = xbuf.stx_mtime.tv_sec;
  statbuf->st_mtim.tv_nsec = xbuf.stx_mtime.tv_nsec;
  statbuf->st_ctim.tv_sec = xbuf.stx_ctime.tv_sec;
  statbuf->st_ctim.tv_nsec = xbuf.stx_ctime.tv_nsec;
  statbuf->st_blksize = xbuf.stx_blksize;
  statbuf->st_blocks =
      static_cast<decltype(statbuf->st_blocks)>(xbuf.stx_blocks);

  return 0;
}

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_SYS_STAT_LINUX_KERNEL_STATX_H
