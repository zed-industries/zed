// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_COMPILER_RUST_ACCESSORS_HELPERS_H__
#define GOOGLE_PROTOBUF_COMPILER_RUST_ACCESSORS_HELPERS_H__

#include <string>

#include "google/protobuf/compiler/rust/context.h"
#include "google/protobuf/descriptor.h"

namespace google {
namespace protobuf {
namespace compiler {
namespace rust {

// Returns the field's default value as a Rust literal / identifier.
//
// Both strings and bytes are represented as a byte string literal, i.e. in the
// format `b"default value here"`. It is the caller's responsibility to convert
// the byte literal to an actual string, if needed.
std::string DefaultValue(Context& ctx, const FieldDescriptor& field);

}  // namespace rust
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_RUST_ACCESSORS_HELPERS_H__
