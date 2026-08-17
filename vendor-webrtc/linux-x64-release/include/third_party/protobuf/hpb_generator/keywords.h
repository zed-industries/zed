// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef PROTOBUF_COMPILER_HBP_GENERATOR_KEYWORDS_H
#define PROTOBUF_COMPILER_HBP_GENERATOR_KEYWORDS_H

#include <string>

#include "absl/strings/string_view.h"

namespace google::protobuf::hpb_generator {

// Resolves proto field name conflict with C++ reserved keywords.
std::string ResolveKeywordConflict(absl::string_view name);

}  // namespace protobuf
}  // namespace google::hpb_generator

#endif  // PROTOBUF_COMPILER_HBP_GENERATOR_KEYWORDS_H
