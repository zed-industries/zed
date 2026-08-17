// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_FUCHSIA_FILE_UTILS_H_
#define BASE_FUCHSIA_FILE_UTILS_H_

#include <fuchsia/io/cpp/fidl.h>

#include "base/base_export.h"
#include "base/files/file_path.h"

namespace base {

// Persisted data directory, i.e. /data .
BASE_EXPORT extern const char kPersistedDataDirectoryPath[];

// Persisted cache directory, i.e. /cache .
BASE_EXPORT extern const char kPersistedCacheDirectoryPath[];

// Services directory, i.e. /svc .
BASE_EXPORT extern const char kServiceDirectoryPath[];

// Package root directory, i.e. /pkg .
BASE_EXPORT extern const char kPackageRootDirectoryPath[];

// Returns a read-only fuchsia.io.Directory for the specified |path|, or an
// invalid InterfaceHandle if the path doesn't exist or it's not a directory.
BASE_EXPORT fidl::InterfaceHandle<::fuchsia::io::Directory> OpenDirectoryHandle(
    const base::FilePath& path);

// Returns a write-capable fuchsia.io.Directory for the specified |path| or
// an invalid InterfaceHandle if the path doesn't exist or it's not a directory.
BASE_EXPORT fidl::InterfaceHandle<::fuchsia::io::Directory>
OpenWritableDirectoryHandle(const base::FilePath& path);

}  // namespace base

#endif  // BASE_FUCHSIA_FILE_UTILS_H_
