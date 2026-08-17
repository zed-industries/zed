// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef THIRD_PARTY_UPB_UPB_GENERATOR_MINITABLE_NAMES_INTERNAL_H_
#define THIRD_PARTY_UPB_UPB_GENERATOR_MINITABLE_NAMES_INTERNAL_H_

#include <string>

#include "absl/strings/string_view.h"

namespace upb {
namespace generator {

// Like the public MiniTableHeaderFilename, but also handles the case where we
// are generating a bootstrap header.
std::string MiniTableHeaderFilename(absl::string_view proto_filename,
                                    bool bootstrap);

// Global static variables used in the generated .c file.
constexpr char kEnumsInit[] = "enums_layout";
constexpr char kExtensionsInit[] = "extensions_layout";
constexpr char kMessagesInit[] = "messages_layout";

// Per-message static variables used in the generated .c file.
std::string MiniTableFieldsVarName(absl::string_view msg_full_name);
std::string MiniTableSubMessagesVarName(absl::string_view msg_full_name);

}  // namespace generator
}  // namespace upb

#endif  // THIRD_PARTY_UPB_UPB_GENERATOR_MINITABLE_NAMES_INTERNAL_H_
