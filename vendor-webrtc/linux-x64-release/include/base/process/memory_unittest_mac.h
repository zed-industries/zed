// Copyright 2010 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file contains helpers for the process_util_unittest to allow it to fully
// test the Mac code.

#ifndef BASE_PROCESS_MEMORY_UNITTEST_MAC_H_
#define BASE_PROCESS_MEMORY_UNITTEST_MAC_H_

#include <stddef.h>
#include <sys/types.h>

#include "build/build_config.h"

namespace base {

// Allocates memory via system allocators. Alas, they take a _signed_ size for
// allocation.
void* AllocateViaCFAllocatorSystemDefault(ssize_t size);
void* AllocateViaCFAllocatorMalloc(ssize_t size);
void* AllocateViaCFAllocatorMallocZone(ssize_t size);

}  // namespace base

#endif  // BASE_PROCESS_MEMORY_UNITTEST_MAC_H_
