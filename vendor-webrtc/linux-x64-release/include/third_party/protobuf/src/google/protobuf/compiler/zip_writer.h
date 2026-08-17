// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Author: kenton@google.com (Kenton Varda)
#ifndef GOOGLE_PROTOBUF_COMPILER_ZIP_WRITER_H__
#define GOOGLE_PROTOBUF_COMPILER_ZIP_WRITER_H__

#include <cstdint>
#include <vector>

#include "google/protobuf/stubs/common.h"
#include "google/protobuf/io/zero_copy_stream.h"

namespace google {
namespace protobuf {
namespace compiler {

class ZipWriter {
 public:
  ZipWriter(io::ZeroCopyOutputStream* raw_output);
  ~ZipWriter();

  bool Write(const std::string& filename, const std::string& contents);
  bool WriteDirectory();

 private:
  struct FileInfo {
    std::string name;
    uint32_t offset;
    uint32_t size;
    uint32_t crc32;
  };

  io::ZeroCopyOutputStream* raw_output_;
  std::vector<FileInfo> files_;
};

}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_ZIP_WRITER_H__
