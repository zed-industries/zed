// Copyright 2017 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef SRC_UNITTEST_COMMON_H_
#define SRC_UNITTEST_COMMON_H_

#include <string>
#include <vector>

#include "base/files/file_path.h"
#include "base/files/file_util.h"
#include "puffin/src/include/puffin/common.h"
#include "puffin/src/logging.h"

namespace puffin {

// Utility class to delete a file when it goes out of scope.
class ScopedPathUnlinker {
 public:
  explicit ScopedPathUnlinker(const std::string& path) : path_(path) {}
  ~ScopedPathUnlinker() {
    base::FilePath path = base::FilePath::FromUTF8Unsafe(path_);
    if (!base::DeleteFile(path)) {
      LOG(ERROR) << "Failed to delete: " << path_;
    }
  }

 private:
  const std::string path_;

  DISALLOW_COPY_AND_ASSIGN(ScopedPathUnlinker);
};

// Makes a temporary file as /tmp/puffin-XXXXXX. Both |filename| and |fd| are
// optional, but if given, they will be populated with the new temporary file's
// values.
bool MakeTempFile(std::string* filename);

extern const Buffer kDeflatesSample1;
extern const Buffer kPuffsSample1;
extern const std::vector<ByteExtent> kDeflateExtentsSample1;
extern const std::vector<BitExtent> kSubblockDeflateExtentsSample1;
extern const std::vector<ByteExtent> kPuffExtentsSample1;

extern const Buffer kDeflatesSample2;
extern const Buffer kPuffsSample2;
extern const std::vector<ByteExtent> kDeflateExtentsSample2;
extern const std::vector<BitExtent> kSubblockDeflateExtentsSample2;
extern const std::vector<ByteExtent> kPuffExtentsSample2;

extern const Buffer kProblematicCache;
extern const std::vector<BitExtent> kProblematicCacheDeflateExtents;
extern const std::vector<BitExtent> kProblematicCachePuffExtents;

}  // namespace puffin

#endif  // SRC_UNITTEST_COMMON_H_
