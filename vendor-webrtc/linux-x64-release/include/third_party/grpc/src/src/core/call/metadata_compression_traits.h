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

#ifndef GRPC_SRC_CORE_CALL_METADATA_COMPRESSION_TRAITS_H
#define GRPC_SRC_CORE_CALL_METADATA_COMPRESSION_TRAITS_H

#include <grpc/support/port_platform.h>
#include <stddef.h>

namespace grpc_core {

///////////////////////////////////////////////////////////////////////////////
// Compression traits.
//
// Each metadata trait exposes exactly one compression trait.
// This type directs how transports might choose to compress the metadata.
// Adding a value here typically involves editing all transports to support the
// trait, and so should not be done lightly.

// No compression.
struct NoCompressionCompressor {};

// Expect a single value for this metadata key, but we don't know apriori its
// value.
// It's ok if it changes over time, but it should be mostly stable.
// This is used for things like user-agent, which is expected to be the same
// for all requests.
struct StableValueCompressor {};

// Expect a single value for this metadata key, and we know apriori its value.
template <typename T, T value>
struct KnownValueCompressor {};

// Values are incompressable, but expect the key to be in most requests and try
// and compress that.
struct FrequentKeyWithNoValueCompressionCompressor {};

// Expect a small set of values for this metadata key.
struct SmallSetOfValuesCompressor {};

// Expect integral values up to N for this metadata key.
template <size_t N>
struct SmallIntegralValuesCompressor {};

// Specialty compressor for grpc-timeout metadata.
struct TimeoutCompressor {};

// Specialty compressors for HTTP/2 pseudo headers.
struct HttpSchemeCompressor {};
struct HttpMethodCompressor {};
struct HttpStatusCompressor {};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_CALL_METADATA_COMPRESSION_TRAITS_H
