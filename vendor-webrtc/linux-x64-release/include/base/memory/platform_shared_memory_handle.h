// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MEMORY_PLATFORM_SHARED_MEMORY_HANDLE_H_
#define BASE_MEMORY_PLATFORM_SHARED_MEMORY_HANDLE_H_

#include "build/build_config.h"

#if BUILDFLAG(IS_APPLE)
#include <mach/mach.h>

#include "base/apple/scoped_mach_port.h"
#elif BUILDFLAG(IS_FUCHSIA)
#include <lib/zx/vmo.h>
#elif BUILDFLAG(IS_WIN)
#include "base/win/scoped_handle.h"
#include "base/win/windows_types.h"
#elif BUILDFLAG(IS_POSIX)
#include <sys/types.h>

#include "base/files/scoped_file.h"
#endif

namespace base::subtle {

#if BUILDFLAG(IS_POSIX) && !BUILDFLAG(IS_APPLE) && !BUILDFLAG(IS_ANDROID)
// Helper structs to keep two descriptors on POSIX. It's needed to support
// ConvertToReadOnly().
struct BASE_EXPORT FDPair {
  // The main shared memory descriptor that is used for mapping. May be either
  // writable or read-only, depending on region's mode.
  int fd;
  // The read-only descriptor, valid only in kWritable mode. Replaces |fd| when
  // a region is converted to read-only.
  int readonly_fd;
};

struct BASE_EXPORT ScopedFDPair {
  ScopedFDPair();
  ScopedFDPair(ScopedFD in_fd, ScopedFD in_readonly_fd);
  ScopedFDPair(ScopedFDPair&&);
  ScopedFDPair& operator=(ScopedFDPair&&);
  ~ScopedFDPair();

  FDPair get() const;

  ScopedFD fd;
  ScopedFD readonly_fd;
};
#endif

// Platform-specific shared memory type used by the shared memory system.
#if BUILDFLAG(IS_APPLE)
using PlatformSharedMemoryHandle = mach_port_t;
using ScopedPlatformSharedMemoryHandle = apple::ScopedMachSendRight;
#elif BUILDFLAG(IS_FUCHSIA)
using PlatformSharedMemoryHandle = zx::unowned_vmo;
using ScopedPlatformSharedMemoryHandle = zx::vmo;
#elif BUILDFLAG(IS_WIN)
using PlatformSharedMemoryHandle = HANDLE;
using ScopedPlatformSharedMemoryHandle = win::ScopedHandle;
#elif BUILDFLAG(IS_ANDROID)
using PlatformSharedMemoryHandle = int;
using ScopedPlatformSharedMemoryHandle = ScopedFD;
#else
using PlatformSharedMemoryHandle = FDPair;
using ScopedPlatformSharedMemoryHandle = ScopedFDPair;
#endif

}  // namespace base::subtle

#endif  // BASE_MEMORY_PLATFORM_SHARED_MEMORY_HANDLE_H_
