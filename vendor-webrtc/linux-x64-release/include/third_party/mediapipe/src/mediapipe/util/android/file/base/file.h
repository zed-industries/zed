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

#ifndef MEDIAPIPE_ANDROID_FILE_BASE_FILE_H_
#define MEDIAPIPE_ANDROID_FILE_BASE_FILE_H_

#include <sys/stat.h>
#include <sys/types.h>

#include <string>

namespace mediapipe {
namespace file {

class Options;

bool IsAbsolutePath(const std::string& path);
Options CreationMode(mode_t permissions);

class Options {
 public:
  Options() = default;

  void set_permissions(mode_t permissions) { permissions_ = permissions; }

  mode_t permissions() const { return permissions_; }

 private:
  mode_t permissions_ = S_IRWXU | S_IRWXG | S_IRWXO;
};

inline Options Defaults() { return Options(); }

}  // namespace file

class File {
 public:
  // Return the "basename" for "fname".  I.e. strip out everything
  // up to and including the last "/" in the name.
  static std::string Basename(const std::string& fname);

  static std::string StripBasename(const std::string& fname);

  static bool IsLocalFile(const std::string& fname);

  static bool Exists(const char* name);
  static bool Exists(const std::string& name) { return Exists(name.c_str()); }

  static const std::string CanonicalizeFileName(const char* fname);
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_ANDROID_FILE_BASE_FILE_H_
