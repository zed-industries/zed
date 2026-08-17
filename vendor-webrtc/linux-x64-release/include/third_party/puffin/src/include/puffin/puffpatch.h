// Copyright 2017 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef SRC_INCLUDE_PUFFIN_PUFFPATCH_H_
#define SRC_INCLUDE_PUFFIN_PUFFPATCH_H_

#include "base/files/file.h"
#include "base/files/file_path.h"
#include "puffin/common.h"
#include "puffin/stream.h"

namespace puffin {

extern const char kMagic[];
extern const size_t kMagicLength;
constexpr uint64_t kDefaultPuffCacheSize = 50 * 1024 * 1024;  // 50 MB

// Status codes for Puffin APIs.
//
// Client code should only rely on the distinction between P_OK and the other
// status codes.
//
enum Status {
  P_OK = 1,  // Successful operation.

  P_UNKNOWN_ERROR = 2,  // Error other than listed below.

  P_READ_OPEN_ERROR = 3,  // Could not open input file for reading.
  P_READ_ERROR = 4,       // Could not read from opened input file.

  P_WRITE_OPEN_ERROR = 5,  // Could not open output file for writing.
  P_WRITE_ERROR = 6,       // Could not write to opened output file.

  P_BAD_PUFFIN_MAGIC = 7,    // Puffin patch has bad magic.
  P_BAD_PUFFIN_VERSION = 8,  // Puffin patch has wrong version.
  P_BAD_PUFFIN_HEADER = 9,   // Puffin patch has corrupt header.
  P_BAD_PUFFIN_PATCH_TYPE =
      10,  // Puffin patch provided an unsupported patch type.
  P_BAD_PUFFIN_CORRUPT = 11,  // Zucchini patch has corrupt data.
  P_BAD_ZUCC_CORRUPT = 12,    // Zucchini patch has corrupt data.
  P_BAD_ZUCC_OLD_IMAGE = 13,  // Old zucchini image invalid.
  P_BAD_ZUCC_NEW_IMAGE = 14,  // New image invalid.

  P_BAD_TRANSFORM = 15,  // Transform mis-specified.

  P_STREAM_ERROR = 20,            // Unexpected error from file_stream.h
  P_STREAM_NOT_CONSUMED = 21,     // Stream has extra data, is expected to be
                                  // used up.
  P_SERIALIZATION_FAILED = 22,    //
  P_DESERIALIZATION_FAILED = 23,  //
  P_INPUT_NOT_RECOGNIZED = 24,    // Unrecognized input (not a crx)

  P_UNABLE_TO_GENERATE_PUFFPATCH = 25,  // Generic failure generating patch.
};

// Applies the Puffin patch to deflate stream |src| to create deflate stream
// |dst|. This function is used in the client and internally uses bspatch to
// apply the patch. The input streams are of type |shared_ptr| because
// |PuffPatch| needs to wrap these streams into another ones and we don't want
// to loose the ownership of the input streams. Optionally one can cache the
// puff buffers individually if non-zero value is passed |max_cache_size|.
//
// |src|           IN  Source deflate stream.
// |dst|           IN  Destination deflate stream.
// |patch|         IN  The input patch.
// |patch_length|  IN  The length of the patch.
// |max_cache_size|IN  The maximum amount of memory to cache puff buffers.
Status PuffPatch(UniqueStreamPtr src,
                 UniqueStreamPtr dst,
                 const uint8_t* patch,
                 size_t patch_length,
                 size_t max_cache_size = 0);

Status ApplyPuffPatch(const base::FilePath& input_path,
                      const base::FilePath& patch_path,
                      const base::FilePath& output_path);

Status ApplyPuffPatch(base::File input_file,
                      base::File patch_file,
                      base::File output_file);

}  // namespace puffin

#endif  // SRC_INCLUDE_PUFFIN_PUFFPATCH_H_
