//
//
// Copyright 2017 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LIB_COMPRESSION_COMPRESSION_INTERNAL_H
#define GRPC_SRC_CORE_LIB_COMPRESSION_COMPRESSION_INTERNAL_H

#include <grpc/impl/compression_types.h>
#include <grpc/support/port_platform.h>
#include <stdint.h>

#include <initializer_list>
#include <optional>

#include "absl/strings/string_view.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/slice/slice.h"
#include "src/core/util/bitset.h"

namespace grpc_core {

// Given a string naming a compression algorithm, return the corresponding enum
// or nullopt on error.
std::optional<grpc_compression_algorithm> ParseCompressionAlgorithm(
    absl::string_view algorithm);
// Convert a compression algorithm to a string. Returns nullptr if a name is not
// known.
const char* CompressionAlgorithmAsString(grpc_compression_algorithm algorithm);
// Retrieve the default compression algorithm from channel args, return nullopt
// if not found.
std::optional<grpc_compression_algorithm>
DefaultCompressionAlgorithmFromChannelArgs(const ChannelArgs& args);

// A set of grpc_compression_algorithm values.
class CompressionAlgorithmSet {
 public:
  // Construct from a uint32_t bitmask - bit 0 => algorithm 0, bit 1 =>
  // algorithm 1, etc.
  static CompressionAlgorithmSet FromUint32(uint32_t value);
  // Locate in channel args and construct from the found value.
  static CompressionAlgorithmSet FromChannelArgs(const ChannelArgs& args);
  // Parse a string of comma-separated compression algorithms.
  static CompressionAlgorithmSet FromString(absl::string_view str);
  // Construct an empty set.
  CompressionAlgorithmSet();
  // Construct from a std::initializer_list of grpc_compression_algorithm
  // values.
  CompressionAlgorithmSet(
      std::initializer_list<grpc_compression_algorithm> algorithms);

  // Given a compression level, choose an appropriate algorithm from this set.
  grpc_compression_algorithm CompressionAlgorithmForLevel(
      grpc_compression_level level) const;
  // Return true if this set contains algorithm, false otherwise.
  bool IsSet(grpc_compression_algorithm algorithm) const;
  // Add algorithm to this set.
  void Set(grpc_compression_algorithm algorithm);

  // Return a comma separated string of the algorithms in this set.
  absl::string_view ToString() const;
  Slice ToSlice() const;

  // Return a bitmask of the algorithms in this set.
  uint32_t ToLegacyBitmask() const;

  bool operator==(const CompressionAlgorithmSet& other) const {
    return set_ == other.set_;
  }

 private:
  BitSet<GRPC_COMPRESS_ALGORITHMS_COUNT> set_;
};

grpc_compression_options CompressionOptionsFromChannelArgs(
    const ChannelArgs& args);

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_COMPRESSION_COMPRESSION_INTERNAL_H
