// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_BLOB_BLOB_UTILS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_BLOB_BLOB_UTILS_H_

#include <stdint.h>
#include <limits>

#include "third_party/blink/public/common/common_export.h"

namespace blink {

class BlobUtils {
 public:
  // Get the preferred capacity a mojo::DataPipe being used to read a blob.
  static uint32_t BLINK_COMMON_EXPORT
  GetDataPipeCapacity(uint64_t target_blob_size);

  // Get the preferred chunk size to use when reading a blob to copy
  // into a mojo::DataPipe.
  static uint32_t BLINK_COMMON_EXPORT GetDataPipeChunkSize();

  // Constant used to represent when a blob's size is unknown.
  static constexpr uint64_t BLINK_COMMON_EXPORT kUnknownSize =
      std::numeric_limits<uint64_t>::max();
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_BLOB_BLOB_UTILS_H_
