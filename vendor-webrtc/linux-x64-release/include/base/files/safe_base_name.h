// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_FILES_SAFE_BASE_NAME_H_
#define BASE_FILES_SAFE_BASE_NAME_H_

#include <optional>

#include "base/base_export.h"
#include "base/compiler_specific.h"
#include "base/files/file_path.h"

namespace base {

// Represents the last path component of a FilePath object, either a file or a
// directory. This type does not allow absolute paths or references to parent
// directories and is considered safe to be passed over IPC. See
// FilePath::BaseName().
// Usage examples:
// std::optional<SafeBaseName> a
//     (SafeBaseName::Create(FILE_PATH_LITERAL("file.txt")));
// FilePath dir(FILE_PATH_LITERAL("foo")); dir.Append(*a);
class BASE_EXPORT SafeBaseName {
 public:
  // TODO(crbug.com/40205226): Change to only be exposed to Mojo.
  SafeBaseName() = default;

  // Factory method that returns a valid SafeBaseName or std::nullopt.
  static std::optional<SafeBaseName> Create(const FilePath&);

  // Same as above, but takes a StringViewType for convenience.
  static std::optional<SafeBaseName> Create(FilePath::StringViewType);
  const FilePath& path() const LIFETIME_BOUND { return path_; }

  // Convenience functions.
  const std::string AsUTF8Unsafe() const { return path_.AsUTF8Unsafe(); }
  const FilePath::StringType& value() const LIFETIME_BOUND {
    return path_.value();
  }
  [[nodiscard]] bool empty() const { return path_.empty(); }

  bool operator==(const SafeBaseName& that) const;

 private:
  // Constructs a new SafeBaseName from the given FilePath.
  explicit SafeBaseName(const FilePath&);
  FilePath path_;
};

}  // namespace base

#endif  // BASE_FILES_SAFE_BASE_NAME_H_
