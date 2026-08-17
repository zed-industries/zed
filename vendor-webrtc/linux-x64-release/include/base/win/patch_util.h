// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_WIN_PATCH_UTIL_H_
#define BASE_WIN_PATCH_UTIL_H_

#include <windows.h>

#include "base/base_export.h"

namespace base {
namespace win {
namespace internal {

// Copies |length| bytes from |source| to |destination|, temporarily setting
// |destination| to writable. Returns a Windows error code or NO_ERROR if
// successful.
BASE_EXPORT DWORD ModifyCode(void* destination,
                             const void* source,
                             size_t length);

}  // namespace internal
}  // namespace win
}  // namespace base

#endif  // BASE_WIN_PATCH_UTIL_H_
