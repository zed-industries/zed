// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef THIRD_PARTY_UPB_UPB_GENERATOR_REFLECTION_NAMES_H_
#define THIRD_PARTY_UPB_UPB_GENERATOR_REFLECTION_NAMES_H_

#include <string>

#include "absl/strings/string_view.h"

// Must be last.
#include "upb/port/def.inc"

namespace upb {
namespace generator {

// These are the publicly visible symbols defined in foo.upbdefs.h.
//   const upb_MessageDef* <GetMessage>(upb_DefPool *s);
//   extern const _upb_DefPool_Init <File>;

UPBC_API std::string ReflectionGetMessageSymbol(absl::string_view full_name);
UPBC_API std::string ReflectionFileSymbol(absl::string_view filename);

}  // namespace generator
}  // namespace upb

#include "upb/port/undef.inc"

#endif  // THIRD_PARTY_UPB_UPB_GENERATOR_REFLECTION_NAMES_H_
