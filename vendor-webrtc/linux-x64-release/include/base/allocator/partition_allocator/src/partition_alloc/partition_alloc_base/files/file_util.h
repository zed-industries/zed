// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file contains utility functions for dealing with the local
// filesystem.

#ifndef PARTITION_ALLOC_PARTITION_ALLOC_BASE_FILES_FILE_UTIL_H_
#define PARTITION_ALLOC_PARTITION_ALLOC_BASE_FILES_FILE_UTIL_H_

#include <cstddef>
#include <cstdint>
#include <cstdio>

#include "partition_alloc/build_config.h"
#include "partition_alloc/partition_alloc_base/component_export.h"

#if PA_BUILDFLAG(IS_POSIX) || PA_BUILDFLAG(IS_FUCHSIA)
#include <sys/stat.h>
#include <unistd.h>
#endif

namespace partition_alloc::internal::base {

#if PA_BUILDFLAG(IS_POSIX) || PA_BUILDFLAG(IS_FUCHSIA)

// Read exactly |bytes| bytes from file descriptor |fd|, storing the result
// in |buffer|. This function is protected against EINTR and partial reads.
// Returns true iff |bytes| bytes have been successfully read from |fd|.
PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE)
bool ReadFromFD(int fd, char* buffer, size_t bytes);

#endif  // PA_BUILDFLAG(IS_POSIX) || PA_BUILDFLAG(IS_FUCHSIA)

}  // namespace partition_alloc::internal::base

#endif  // PARTITION_ALLOC_PARTITION_ALLOC_BASE_FILES_FILE_UTIL_H_
