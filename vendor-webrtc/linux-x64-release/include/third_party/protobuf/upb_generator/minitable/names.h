// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef THIRD_PARTY_UPB_UPB_GENERATOR_MINITABLE_NAMES_H_
#define THIRD_PARTY_UPB_UPB_GENERATOR_MINITABLE_NAMES_H_

#include <string>

#include "absl/strings/string_view.h"

// Must be last.
#include "upb/port/def.inc"

namespace upb {
namespace generator {

// Maps: foo/bar/baz.proto -> foo/bar/baz.upb_minitable.h
UPBC_API std::string MiniTableHeaderFilename(absl::string_view proto_filename);

// These are the publicly visible symbols defined in foo.upb_minitable.h.
//   extern const upb_MiniTable <Message>;             // One for each message.
//   extern const upb_MiniTableEnum <Enum>;            // One for each enum.
//   extern const upb_MiniTableExtension <Extension>;  // One for each ext.
//   extern const upb_MiniTableFile <File>;            // One for each file.
//
//   extern const upb_MiniTable* <MessagePtr>;

UPBC_API std::string MiniTableMessageVarName(absl::string_view full_name);
UPBC_API std::string MiniTableEnumVarName(absl::string_view full_name);
UPBC_API std::string MiniTableExtensionVarName(absl::string_view full_name);
UPBC_API std::string MiniTableFileVarName(absl::string_view proto_filename);

// This is used for weak linking and tree shaking. Other translation units may
// define weak versions of this symbol that point to a dummy message, to
// gracefully degrade the behavior of the generated code when the message is not
// linked into the current binary.
UPBC_API std::string MiniTableMessagePtrVarName(absl::string_view full_name);

}  // namespace generator
}  // namespace upb

#endif  // THIRD_PARTY_UPB_UPB_GENERATOR_MINITABLE_NAMES_H_
