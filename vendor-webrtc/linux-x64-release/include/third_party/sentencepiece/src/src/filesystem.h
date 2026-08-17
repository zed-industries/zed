// Copyright 2016 Google Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.!

#ifndef FILESYSTEM_H_
#define FILESYSTEM_H_

#include <stdio.h>

#include <fstream>
#include <memory>
#include <string>

#include "absl/strings/string_view.h"
#include "common.h"
#include "sentencepiece_processor.h"

namespace sentencepiece {
namespace filesystem {
class ReadableFile {
 public:
  ReadableFile() {}
  explicit ReadableFile(absl::string_view filename, bool is_binary = false) {}
  virtual ~ReadableFile() {}

  virtual util::Status status() const = 0;
  virtual bool ReadLine(std::string *line) = 0;
  virtual bool ReadAll(std::string *line) = 0;
};

class WritableFile {
 public:
  WritableFile() {}
  explicit WritableFile(absl::string_view filename, bool is_binary = false) {}
  virtual ~WritableFile() {}

  virtual util::Status status() const = 0;
  virtual bool Write(absl::string_view text) = 0;
  virtual bool WriteLine(absl::string_view text) = 0;
};

std::unique_ptr<ReadableFile> NewReadableFile(absl::string_view filename,
                                              bool is_binary = false);
std::unique_ptr<WritableFile> NewWritableFile(absl::string_view filename,
                                              bool is_binary = false);

}  // namespace filesystem
}  // namespace sentencepiece
#endif  // FILESYSTEM_H_
