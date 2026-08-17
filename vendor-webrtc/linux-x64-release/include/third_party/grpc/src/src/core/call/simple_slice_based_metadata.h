// Copyright 2023 gRPC authors.
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

#ifndef GRPC_SRC_CORE_CALL_SIMPLE_SLICE_BASED_METADATA_H
#define GRPC_SRC_CORE_CALL_SIMPLE_SLICE_BASED_METADATA_H

#include <grpc/support/port_platform.h>

#include "absl/strings/string_view.h"
#include "src/core/call/parsed_metadata.h"
#include "src/core/lib/slice/slice.h"

namespace grpc_core {

// Models grpc metadata (as per the rules for MetadataMap) for a metadata
// element that consists of a Slice.
// Use by deriving from this class and adding `kRepeatable` and `key()`.
struct SimpleSliceBasedMetadata {
  using ValueType = Slice;
  using MementoType = Slice;
  static MementoType ParseMemento(Slice value,
                                  bool will_keep_past_request_lifetime,
                                  MetadataParseErrorFn) {
    if (will_keep_past_request_lifetime) {
      return value.TakeUniquelyOwned();
    } else {
      return value.TakeOwned();
    }
  }
  static ValueType MementoToValue(MementoType value) { return value; }
  static Slice Encode(const ValueType& x) { return x.Ref(); }
  static absl::string_view DisplayValue(const ValueType& value) {
    return value.as_string_view();
  }
  static absl::string_view DisplayMemento(const MementoType& value) {
    return value.as_string_view();
  }
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_CALL_SIMPLE_SLICE_BASED_METADATA_H
