// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_FILES_FILE_ERROR_OR_H_
#define BASE_FILES_FILE_ERROR_OR_H_

#include "base/files/file.h"
#include "base/types/expected.h"

namespace base {

// Helper for methods which perform file system operations and which may fail.
// Objects of this type can take on EITHER a base::File::Error value OR a result
// value of the specified type. For example:
//
// base::FileErrorOr<int64_t> GetSize() {
//   if (failed_to_get_size)
//     return base::unexpected(base::File::Error::FILE_ERROR_FAILED);
//
//   return size;
// }
//
template <class ValueType>
using FileErrorOr = expected<ValueType, File::Error>;

}  // namespace base

#endif  // BASE_FILES_FILE_ERROR_OR_H_
