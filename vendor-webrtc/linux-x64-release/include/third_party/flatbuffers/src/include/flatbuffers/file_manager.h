/*
 * Copyright 2023 Google Inc. All rights reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef FLATBUFFERS_FILE_MANAGER_H_
#define FLATBUFFERS_FILE_MANAGER_H_

#include <set>
#include <string>

#include "flatbuffers/util.h"

namespace flatbuffers {

// A File interface to write data to file by default or
// save only file names
class FileManager {
 public:
  FileManager() = default;
  virtual ~FileManager() = default;

  virtual bool SaveFile(const std::string &absolute_file_name,
                        const std::string &content) = 0;

  virtual bool LoadFile(const std::string &absolute_file_name,
                        std::string *buf) = 0;

 private:
  // Copying is not supported.
  FileManager(const FileManager &) = delete;
  FileManager &operator=(const FileManager &) = delete;
};

}  // namespace flatbuffers

#endif  // FLATBUFFERS_FILE_MANAGER_H_
