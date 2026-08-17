// Copyright 2016 The Bazel Authors. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef THIRD_PARTY_IJAR_ZLIB_CLIENT_H_
#define THIRD_PARTY_IJAR_ZLIB_CLIENT_H_

#include <limits.h>

#include <limits>
#include <stdexcept>

#include "third_party/ijar/common.h"

namespace devtools_ijar {
// Try to compress a file entry in memory using the deflate algorithm.
// It will compress buf (of size length) unless the compressed size is bigger
// than the input size. The result will overwrite the content of buf and the
// final size is returned.
size_t TryDeflate(u1* buf, size_t length);

u4 ComputeCrcChecksum(u1* buf, size_t length);

struct DecompressedFile {
  u1* uncompressed_data;
  u4 uncompressed_size;
  u4 compressed_size;
};

class Decompressor {
 public:
  Decompressor();
  ~Decompressor();
  DecompressedFile* UncompressFile(const u1* buffer, size_t bytes_avail);
  char* GetError();

 private:
  // Administration of memory reserved for decompressed data. We use the same
  // buffer for each file to avoid some malloc()/free() calls and free the
  // memory only in the dtor. C-style memory management is used so that we
  // can call realloc.
  u1* uncompressed_data_;
  size_t uncompressed_data_allocated_;
  // last error
  char errmsg[4 * PATH_MAX];

  int error(const char* fmt, ...);

  // Buffer size is initially INITIAL_BUFFER_SIZE. It doubles in size every
  // time it is found too small, until it reaches MAX_BUFFER_SIZE. If that is
  // not enough, we bail out. We only decompress class files, so they should
  // be smaller than 64K anyway, but we give a little leeway.
  // MAX_BUFFER_SIZE must be bigger than the size of the biggest file in the
  // ZIP. It is set to 2GB here because no one has audited the code for 64-bit
  // cleanliness.
  static const size_t INITIAL_BUFFER_SIZE = 256 * 1024;  // 256K
  static const size_t MAX_BUFFER_SIZE = std::numeric_limits<int32_t>::max();
};
}  // namespace devtools_ijar

#endif  // THIRD_PARTY_IJAR_ZLIB_CLIENT_H_
