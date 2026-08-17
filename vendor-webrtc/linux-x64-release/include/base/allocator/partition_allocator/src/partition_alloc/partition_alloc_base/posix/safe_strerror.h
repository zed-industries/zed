// Copyright 2011 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_PARTITION_ALLOC_BASE_POSIX_SAFE_STRERROR_H_
#define PARTITION_ALLOC_PARTITION_ALLOC_BASE_POSIX_SAFE_STRERROR_H_

#include <cstddef>
#include <string>

#include "partition_alloc/partition_alloc_base/component_export.h"

namespace partition_alloc::internal::base {

// BEFORE using anything from this file, first look at PLOG and friends in
// logging.h and use them instead if applicable.
//
// This file declares safe, portable alternatives to the POSIX strerror()
// function. strerror() is inherently unsafe in multi-threaded apps and should
// never be used. Doing so can cause crashes. Additionally, the thread-safe
// alternative strerror_r varies in semantics across platforms. Use these
// functions instead.

// Thread-safe strerror function with dependable semantics that never fails.
// It will write the string form of error "err" to buffer buf of length len.
// If there is an error calling the OS's strerror_r() function then a message to
// that effect will be printed into buf, truncating if necessary. The final
// result is always null-terminated. The value of errno is never changed.
//
// Use this instead of strerror_r().
PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE)
void safe_strerror_r(int err, char* buf, size_t len);

// Calls safe_strerror_r with a buffer of suitable size and returns the result
// in a C++ string.
//
// Use this instead of strerror(). Note though that safe_strerror_r will be
// more robust in the case of heap corruption errors, since it doesn't need to
// allocate a string.
PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE) std::string safe_strerror(int err);

}  // namespace partition_alloc::internal::base

#endif  // PARTITION_ALLOC_PARTITION_ALLOC_BASE_POSIX_SAFE_STRERROR_H_
