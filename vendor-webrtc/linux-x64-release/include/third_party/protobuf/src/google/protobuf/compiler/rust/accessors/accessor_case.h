// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_COMPILER_RUST_ACCESSORS_ACCESSOR_CASE_H__
#define GOOGLE_PROTOBUF_COMPILER_RUST_ACCESSORS_ACCESSOR_CASE_H__

#include "absl/strings/string_view.h"

namespace google {
namespace protobuf {
namespace compiler {
namespace rust {

// GenerateAccessorMsgImpl is reused for all three types of $Msg$, $Msg$Mut and
// $Msg$View; this enum signifies which case we are handling so corresponding
// adjustments can be made (for example: to not emit any mutation accessors
// on $Msg$View).
enum class AccessorCase {
  OWNED,
  MUT,
  VIEW,
};

// Returns the `self` receiver type for a subfield view accessor.
absl::string_view ViewReceiver(AccessorCase accessor_case);

// Returns the lifetime of a subfield view accessor.
// Views are Copy, and so the full `'msg` lifetime can be used.
// Any `&self` or `&mut self` accessors need to use the lifetime of that
// borrow, which is referenced via `'_`.
// See b/314989133 for _mut accessors.
absl::string_view ViewLifetime(AccessorCase accessor_case);

}  // namespace rust
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_RUST_ACCESSORS_ACCESSOR_CASE_H__
