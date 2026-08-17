// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MEMORY_PAGE_SIZE_H_
#define BASE_MEMORY_PAGE_SIZE_H_

#include <stddef.h>

#include "base/base_export.h"

namespace base {

// Returns the number of bytes in a memory page. Do not use this to compute
// the number of pages in a block of memory for calling mincore(). On some
// platforms, e.g. iOS, mincore() uses a different page size from what is
// returned by GetPageSize().
BASE_EXPORT size_t GetPageSize();

}  // namespace base

#endif  // BASE_MEMORY_PAGE_SIZE_H_
