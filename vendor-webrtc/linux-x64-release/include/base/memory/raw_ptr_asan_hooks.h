// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MEMORY_RAW_PTR_ASAN_HOOKS_H_
#define BASE_MEMORY_RAW_PTR_ASAN_HOOKS_H_

#include "partition_alloc/buildflags.h"

#if PA_BUILDFLAG(USE_ASAN_BACKUP_REF_PTR)

#include "base/memory/raw_ptr.h"

namespace base::internal {

const RawPtrHooks* GetRawPtrAsanHooks();

}

#endif  // PA_BUILDFLAG(USE_ASAN_BACKUP_REF_PTR)

#endif  // BASE_MEMORY_RAW_PTR_ASAN_HOOKS_H_
