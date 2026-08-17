/* Copyright 2021 The TensorFlow Authors. All Rights Reserved.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
==============================================================================*/

#ifndef TENSORFLOW_LITE_SUPPORT_METADATA_CC_UTILS_ZIP_MEM_FILE_H_
#define TENSORFLOW_LITE_SUPPORT_METADATA_CC_UTILS_ZIP_MEM_FILE_H_

#include <cstdlib>

#include "absl/strings/string_view.h"  // from @com_google_absl
#include "third_party/zlib/contrib/minizip/ioapi.h"

namespace tflite {
namespace metadata {

// In-memory read-only zip file implementation.
//
// Adapted from [1], with a few key differences:
// * backed by an `absl::string_view` instead of malloc-ed C buffers,
// * supports opening the file for reading through `unzOpen2_64`.
//
// This class is NOT thread-safe.
//
// [1]:
// https://github.com/google/libkml/blob/master/third_party/zlib-1.2.3/contrib/minizip/iomem_simple.c
class ZipReadOnlyMemFile {
 public:
  // Constructs an in-memory read-only zip file from a buffer. Does not copy or
  // take ownership over the provided buffer: the caller is responsible for
  // ensuring the buffer outlives this object.
  ZipReadOnlyMemFile(const char* buffer, size_t size);
  // Provides access to the `zlib_filefunc64_def` implementation for the
  // in-memory zip file.
  zlib_filefunc64_def& GetFileFunc64Def();

 private:
  // The string view backing the in-memory file.
  absl::string_view data_;
  // The current offset in the file.
  ZPOS64_T offset_;
  // The `zlib_filefunc64_def` implementation for this in-memory zip file.
  zlib_filefunc64_def zlib_filefunc64_def_;

  // Convenience function to access the current data size.
  size_t Size() const { return data_.size(); }

  // The file function implementations used in the `zlib_filefunc64_def`.
  static voidpf OpenFile(voidpf opaque, const void* filename, int mode);
  static uLong ReadFile(voidpf opaque, voidpf stream, void* buf, uLong size);
  static uLong WriteFile(voidpf opaque, voidpf stream, const void* buf,
                         uLong size);
  static ZPOS64_T TellFile(voidpf opaque, voidpf stream);
  static long SeekFile  // NOLINT
      (voidpf opaque, voidpf stream, ZPOS64_T offset, int origin);
  static int CloseFile(voidpf opaque, voidpf stream);
  static int ErrorFile(voidpf opaque, voidpf stream);
};

}  // namespace metadata
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_METADATA_CC_UTILS_ZIP_MEM_FILE_H_
