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

#ifndef GRPC_PYTHON_OPENCENSUS_RPC_ENCODING_H
#define GRPC_PYTHON_OPENCENSUS_RPC_ENCODING_H

#include <grpc/support/port_platform.h>
#include <stdint.h>
#include <string.h>

// TODO(xuanwn): Use absl::endian/absl::byteswap when abseil-cpp is upgraded
// to 202507.
#include "absl/base/internal/endian.h"
#include "absl/strings/string_view.h"

namespace grpc_observability {

// TODO(xuanwn): Reuse c++ rpc_encoding file.
// RpcServerStatsEncoding encapsulates the logic for encoding and decoding of
// rpc server stats messages. Rpc server stats consists of a uint64_t time
// value (server latency in nanoseconds).
class RpcServerStatsEncoding {
 public:
  // Size of encoded RPC server stats.
  static constexpr size_t kRpcServerStatsSize = 10;
  // Error value.
  static constexpr size_t kEncodeDecodeFailure = 0;

  // Deserializes rpc server stats from the incoming 'buf' into *time.  Returns
  // number of bytes decoded. If the buffer is of insufficient size (it must be
  // at least kRpcServerStatsSize bytes) or the encoding version or field ID are
  // unrecognized, *time will be set to 0 and it will return
  // kEncodeDecodeFailure. Inlined for performance reasons.
  static size_t Decode(absl::string_view buf, uint64_t* time) {
    if (buf.size() < kRpcServerStatsSize) {
      *time = 0;
      return kEncodeDecodeFailure;
    }

    uint8_t version = buf[kVersionIdOffset];
    uint32_t fieldID = buf[kServerElapsedTimeOffset];
    if (version != kVersionId || fieldID != kServerElapsedTimeField) {
      *time = 0;
      return kEncodeDecodeFailure;
    }
    *time = absl::little_endian::Load64(
        &buf[kServerElapsedTimeOffset + kFieldIdSize]);
    return kRpcServerStatsSize;
  }

  // Serializes rpc server stats into the provided buffer.  It returns the
  // number of bytes written to the buffer. If the buffer is smaller than
  // kRpcServerStatsSize bytes it will return kEncodeDecodeFailure. Inlined for
  // performance reasons.
  static size_t Encode(uint64_t time, char* buf, size_t buf_size) {
    if (buf_size < kRpcServerStatsSize) {
      return kEncodeDecodeFailure;
    }

    buf[kVersionIdOffset] = kVersionId;
    buf[kServerElapsedTimeOffset] = kServerElapsedTimeField;
    absl::little_endian::Store64(&buf[kServerElapsedTimeOffset + kFieldIdSize],
                                 time);
    return kRpcServerStatsSize;
  }

 private:
  // Size of Version ID.
  static constexpr size_t kVersionIdSize = 1;
  // Size of Field ID.
  static constexpr size_t kFieldIdSize = 1;

  // Offset and value for currently supported version ID.
  static constexpr size_t kVersionIdOffset = 0;
  static constexpr size_t kVersionId = 0;

  enum FieldIdValue {
    kServerElapsedTimeField = 0,
  };

  enum FieldSize {
    kServerElapsedTimeSize = 8,
  };

  enum FieldIdOffset {
    kServerElapsedTimeOffset = kVersionIdSize,
  };

  RpcServerStatsEncoding() = delete;
  RpcServerStatsEncoding(const RpcServerStatsEncoding&) = delete;
  RpcServerStatsEncoding(RpcServerStatsEncoding&&) = delete;
  RpcServerStatsEncoding operator=(const RpcServerStatsEncoding&) = delete;
  RpcServerStatsEncoding operator=(RpcServerStatsEncoding&&) = delete;
};

}  // namespace grpc_observability

#endif  // GRPC_PYTHON_OPENCENSUS_RPC_ENCODING_H
