// Copyright 2019 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Declares DeletingFile.

#ifndef MEDIAPIPE_FRAMEWORK_FORMATS_DELETING_FILE_H_
#define MEDIAPIPE_FRAMEWORK_FORMATS_DELETING_FILE_H_

#include <string>

namespace mediapipe {

// A DeletingFile conveys the path to a file and takes care of cleanup
// (generally deletion of a file if it is a local temporary).
class DeletingFile {
 public:
  DeletingFile(const DeletingFile&) = delete;
  DeletingFile& operator=(const DeletingFile&) = delete;

  // DeletingFile is movable. The moved-from object remains in valid but
  // unspecified state and will not perform any operations on destruction.
  DeletingFile(DeletingFile&& other);
  DeletingFile& operator=(DeletingFile&& other);

  // Provide the path to the file and whether the file should be deleted
  // when this object is destroyed.
  DeletingFile(const std::string& path, bool delete_on_destruction);

  // Takes care of cleaning up the file (deletes it if
  // delete_on_destruction was true on construction, otherwise leaves
  // it alone).
  virtual ~DeletingFile();

  // Return the path to the file.
  const std::string& Path() const;

 private:
  std::string path_;
  bool delete_on_destruction_;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_FORMATS_DELETING_FILE_H_
