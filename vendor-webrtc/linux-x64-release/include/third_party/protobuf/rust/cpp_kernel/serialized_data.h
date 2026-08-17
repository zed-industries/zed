// Protocol Buffers - Google's data interchange format
// Copyright 2024 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_RUST_CPP_KERNEL_SERIALIZED_DATA_H__
#define GOOGLE_PROTOBUF_RUST_CPP_KERNEL_SERIALIZED_DATA_H__

#include <climits>
#include <cstddef>
#include <cstdint>
#include <cstring>

#include "absl/log/absl_check.h"
#include "absl/log/absl_log.h"
#include "google/protobuf/message_lite.h"
#include "rust/cpp_kernel/rust_alloc_for_cpp_api.h"

namespace google {
namespace protobuf {
namespace rust {

// Represents serialized Protobuf wire format data.
//
// Only to be used to transfer serialized data from C++ to Rust under these
// assumptions:
// * Rust and C++ versions of this struct are ABI compatible.
// * Rust version owns and frees its data.
// * The data were allocated using the Rust allocator.
//
extern "C" struct SerializedData {
  // Owns the memory, must be freed by Rust.
  const uint8_t* data;
  size_t len;
};

inline bool SerializeMsg(const google::protobuf::MessageLite* msg, SerializedData* out) {
  ABSL_DCHECK(msg->IsInitialized());
  size_t len = msg->ByteSizeLong();
  if (len > INT_MAX) {
    ABSL_LOG(ERROR) << msg->GetTypeName()
                    << " exceeded maximum protobuf size of 2GB: " << len;
    return false;
  }
  uint8_t* bytes = static_cast<uint8_t*>(proto2_rust_alloc(len, alignof(char)));
  if (bytes == nullptr) {
    ABSL_LOG(FATAL) << "Rust allocator failed to allocate memory.";
  }
  if (!msg->SerializeWithCachedSizesToArray(bytes)) {
    return false;
  }
  out->data = bytes;
  out->len = len;
  return true;
}

}  // namespace rust
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_RUST_CPP_KERNEL_SERIALIZED_DATA_H__
