// Copyright 2023 The Centipede Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Abstraction of metadata collected from executions that does not
// contribute to coverage but can be useful in mutation.
//
// This library is for both engine and runner.

#ifndef THIRD_PARTY_CENTIPEDE_EXECUTION_METADATA_H_
#define THIRD_PARTY_CENTIPEDE_EXECUTION_METADATA_H_

#include <functional>

#include "./centipede/shared_memory_blob_sequence.h"
#include "./common/defs.h"

namespace fuzztest::internal {

struct ExecutionMetadata {
  // Appends a CMP entry comparing `a` and `b` to the metadata. Returns false if
  // the entry cannot be appended. Return true otherwise.
  bool AppendCmpEntry(ByteSpan a, ByteSpan b);

  // Enumerates through all CMP entries in the metadata by calling
  // `callback` on each of them. Returns false if there are invalid
  // entries. Returns true otherwise.
  bool ForEachCmpEntry(std::function<void(ByteSpan, ByteSpan)> callback) const;

  // Writes the contents to `outputs_blobseq` with header `tag`. Returns true
  // iff successful.
  bool Write(Blob::SizeAndTagT tag, BlobSequence &outputs_blobseq) const;

  // Reads the contents from `blob`.
  //
  // Note that the method does not check the blob tag, it should be checked by
  // the method users.
  void Read(Blob blob);

  // CMP entries are stored in one large ByteArray to minimize RAM consumption.
  // One CMP arg pair is stored as
  //  * `size` (1-byte value)
  //  * `value0` (`size` bytes)
  //  * `value1` (`size` bytes)
  ByteArray cmp_data;
};

}  // namespace fuzztest::internal

#endif  // THIRD_PARTY_CENTIPEDE_EXECUTION_METADATA_H_
