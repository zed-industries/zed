// Copyright 2015 The Bazel Authors. All rights reserved.
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

#ifndef INCLUDED_THIRD_PARTY_IJAR_MAPPED_FILE_H
#define INCLUDED_THIRD_PARTY_IJAR_MAPPED_FILE_H

#include "third_party/ijar/common.h"

namespace devtools_ijar {

struct MappedInputFileImpl;
struct MappedOutputFileImpl;

// A memory mapped input file.
class MappedInputFile {
 private:
  MappedInputFileImpl *impl_;

 protected:
  const char* errmsg_;
  bool opened_;
  u1* buffer_;
  size_t length_;

 public:
  MappedInputFile(const char* name);
  virtual ~MappedInputFile();

  // If opening the file succeeded or not.
  bool Opened() const { return opened_; }

  // Description of the last error that happened.
  const char* Error() const { return errmsg_; }

  // The mapped contents of the file.
  u1* Buffer() const { return buffer_ ; }

  // The length of the file.
  size_t Length() const { return length_; }

  // Unmap a given number of bytes from the beginning of the file.
  void Discard(size_t bytes);
  int Close();
};

class MappedOutputFile {
 private:
  MappedOutputFileImpl *impl_;

 protected:
  const char* errmsg_;
  bool opened_;
  u1* buffer_;
  size_t estimated_size_;

 public:
  MappedOutputFile(const char* name, size_t estimated_size);
  virtual ~MappedOutputFile();

  // If opening the file succeeded or not.
  bool Opened() const { return opened_; }

  // Description of the last error that happened.
  const char* Error() const { return errmsg_; }

  // The mapped contents of the file.
  u1* Buffer() const { return buffer_; }
  int Close(size_t size);
};

}  // namespace devtools_ijar
#endif
