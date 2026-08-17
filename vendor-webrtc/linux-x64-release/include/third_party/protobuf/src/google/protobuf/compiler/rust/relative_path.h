// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_COMPILER_RUST_RELATIVE_PATH_H__
#define GOOGLE_PROTOBUF_COMPILER_RUST_RELATIVE_PATH_H__

#include <string>
#include <vector>

#include "absl/algorithm/container.h"
#include "absl/log/absl_check.h"
#include "absl/strings/match.h"
#include "absl/strings/string_view.h"

namespace google {
namespace protobuf {
namespace compiler {
namespace rust {

// Relative path using '/' as a separator.
class RelativePath final {
 public:
  explicit RelativePath(absl::string_view path) : path_(path) {
    ABSL_CHECK(!absl::StartsWith(path, "/"))
        << "only relative paths are supported";
    // `..` and `.` not supported, since there's no use case for that right now.
    for (absl::string_view segment : Segments()) {
      ABSL_CHECK(segment != "..") << "`..` segments are not supported";
      ABSL_CHECK(segment != ".") << "`.` segments are not supported";
    }
  }

  // Returns a path getting us from the current relative path to the `dest`
  // path.
  //
  // Supports both files and directories.
  std::string Relative(const RelativePath& dest) const;
  std::vector<absl::string_view> Segments() const;
  bool IsDirectory() const;

 private:
  absl::string_view path_;
};

}  // namespace rust
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_RUST_RELATIVE_PATH_H__
