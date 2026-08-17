// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_COMPILER_RUST_ENUM_H__
#define GOOGLE_PROTOBUF_COMPILER_RUST_ENUM_H__

#include <cstdint>
#include <string>
#include <utility>
#include <vector>

#include "absl/strings/string_view.h"
#include "absl/types/span.h"
#include "google/protobuf/compiler/rust/context.h"
#include "google/protobuf/descriptor.h"

namespace google {
namespace protobuf {
namespace compiler {
namespace rust {

// Generates code for a particular enum in `.pb.rs`.
void GenerateEnumDefinition(Context& ctx, const EnumDescriptor& desc);

// An enum value with a unique number and any aliases for it.
struct RustEnumValue {
  // The canonical CamelCase name in Rust.
  std::string name;
  int32_t number;
  std::vector<std::string> aliases;
};

// Returns the list of rust enum variants to produce, along with their aliases.
// Performs name normalization, deduplication, and alias determination.
// The `number` and `name` of every returned `RustEnumValue` is unique.
std::vector<RustEnumValue> EnumValues(
    absl::string_view enum_name,
    absl::Span<const std::pair<absl::string_view, int32_t>> values);

}  // namespace rust
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_RUST_ENUM_H__
