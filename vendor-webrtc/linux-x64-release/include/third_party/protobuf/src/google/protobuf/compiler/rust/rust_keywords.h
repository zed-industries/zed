// Protocol Buffers - Google's data interchange format
// Copyright 2024 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_COMPILER_RUST_RUST_KEYWORDS_H__
#define GOOGLE_PROTOBUF_COMPILER_RUST_RUST_KEYWORDS_H__

#include "absl/strings/string_view.h"

namespace google {
namespace protobuf {
namespace compiler {
namespace rust {

// Returns true if the provided name is legal to use as a raw identifier name
// by prefixing with r#
// https://doc.rust-lang.org/reference/identifiers.html#raw-identifiers
bool IsLegalRawIdentifierName(absl::string_view str_without_r_prefix);

// Returns true if the provided str is a Rust 2021 Edition keyword and cannot be
// used as an identifier. These symbols can be used with an r# prefix unless
// IsLegalRawIdentifierName returns false. This function should always match the
// behavior for the corresponding Edition that our emitted crates use.
bool IsRustKeyword(absl::string_view str);

}  // namespace rust
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_RUST_RUST_KEYWORDS_H__
