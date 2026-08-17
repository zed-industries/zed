// Copyright 2021 The Crashpad Authors
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
// limitations under the License.

#ifndef CRASHPAD_UTIL_IOS_IOS_INTERMEDIATE_DUMP_INTERFACE_H_
#define CRASHPAD_UTIL_IOS_IOS_INTERMEDIATE_DUMP_INTERFACE_H_

#include "base/files/file_path.h"
#include "util/file/file_reader.h"
#include "util/file/filesystem.h"
#include "util/file/string_file.h"
#include "util/misc/initialization_state_dcheck.h"

namespace crashpad {
namespace internal {

//! \brief The base class for reading data into an IOSIntermediateDumpReader.
class IOSIntermediateDumpInterface {
 public:
  virtual FileReaderInterface* FileReader() const = 0;
  virtual FileOffset Size() const = 0;
};

//! \brief An intermediate dump backed by a FilePath. FilePath is unlinked
//!     immediately upon initialization to ensure files are only processed once
//!     in the event a crash is introduced by this intermediate dump.
class IOSIntermediateDumpFilePath : public IOSIntermediateDumpInterface {
 public:
  bool Initialize(const base::FilePath& path);

  // IOSIntermediateDumpInterface:
  FileReaderInterface* FileReader() const override;
  FileOffset Size() const override;

 private:
  ScopedFileHandle handle_;
  std::unique_ptr<WeakFileHandleFileReader> reader_;
  InitializationStateDcheck initialized_;
};

//! \brief An intermediate dump backed by a byte array.
class IOSIntermediateDumpByteArray : public IOSIntermediateDumpInterface {
 public:
  IOSIntermediateDumpByteArray(const void* data, size_t size);

  // IOSIntermediateDumpInterface
  FileReaderInterface* FileReader() const override;
  FileOffset Size() const override;

 private:
  std::unique_ptr<StringFile> string_file_;
};

}  // namespace internal
}  // namespace crashpad

#endif  // CRASHPAD_UTIL_IOS_IOS_INTERMEDIATE_DUMP_INTERFACE_H_
