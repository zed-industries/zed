//
//
// Copyright 2016 gRPC authors.
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
//
//

#ifndef GRPC_SRC_CORE_LIB_SLICE_PERCENT_ENCODING_H
#define GRPC_SRC_CORE_LIB_SLICE_PERCENT_ENCODING_H

// Percent encoding and decoding of slices.
// Transforms arbitrary strings into safe-for-transmission strings by using
// variants of percent encoding (RFC 3986).
// Two major variants are supplied: one that strictly matches URL encoding,
//   and another which applies percent encoding only to non-http2 header
//   bytes (the 'compatible' variant)

#include <grpc/support/port_platform.h>

#include "src/core/lib/slice/slice.h"

namespace grpc_core {

enum class PercentEncodingType {
  // Flags [A-Za-z0-9-_.~] as unreserved bytes for the percent encoding routines
  URL,
  // Flags ascii7 non-control characters excluding '%' as unreserved bytes for
  // the percent encoding routines
  Compatible
};

// Percent-encode a slice, returning the new slice (this cannot fail):
// unreserved_bytes is a bitfield indicating which bytes are considered
// unreserved and thus do not need percent encoding
Slice PercentEncodeSlice(Slice slice, PercentEncodingType type);
// Percent-decode a slice, permissively.
// If a % triplet can not be decoded, pass it through verbatim.
// This cannot fail.
Slice PermissivePercentDecodeSlice(Slice slice_in);

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_SLICE_PERCENT_ENCODING_H
