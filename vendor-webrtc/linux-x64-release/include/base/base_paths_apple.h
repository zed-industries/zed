// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_BASE_PATHS_APPLE_H_
#define BASE_BASE_PATHS_APPLE_H_

#include "base/files/file_path.h"

namespace base::apple::internal {

// Returns the absolute path to the executable.
base::FilePath GetExecutablePath();

// Returns true if the module for |address| is found. |path| will contain
// the path to the module. Note that |path| may not be absolute.
[[nodiscard]] bool GetModulePathForAddress(base::FilePath* path,
                                           const void* address);

}  // namespace base::apple::internal

#endif  // BASE_BASE_PATHS_APPLE_H_
