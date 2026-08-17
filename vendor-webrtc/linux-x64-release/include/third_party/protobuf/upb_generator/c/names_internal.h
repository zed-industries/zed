// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef THIRD_PARTY_UPB_UPB_GENERATOR_C_NAMES_INTERNAL_H_
#define THIRD_PARTY_UPB_UPB_GENERATOR_C_NAMES_INTERNAL_H_

#include <string>

#include "absl/strings/string_view.h"

namespace upb {
namespace generator {

// Like the public CApiHeaderFilename, but also handles the case where we are
// generating a bootstrap header.
std::string CApiHeaderFilename(absl::string_view proto_filename,
                               bool bootstrap);

}  // namespace generator
}  // namespace upb

#endif  // THIRD_PARTY_UPB_UPB_GENERATOR_C_NAMES_INTERNAL_H_
