// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_JSON_INTERNAL_PARSER_H__
#define GOOGLE_PROTOBUF_JSON_INTERNAL_PARSER_H__

#include <string>

#include "google/protobuf/json/internal/lexer.h"
#include "google/protobuf/message.h"
#include "google/protobuf/util/type_resolver.h"

namespace google {
namespace protobuf {
namespace json_internal {
// Internal version of google::protobuf::util::JsonStreamToMessage; see json_util.h for
// details.
absl::Status JsonStreamToMessage(io::ZeroCopyInputStream* input,
                                 Message* message,
                                 json_internal::ParseOptions options);
// Internal version of google::protobuf::util::JsonToBinaryStream; see json_util.h for
// details.
absl::Status JsonToBinaryStream(google::protobuf::util::TypeResolver* resolver,
                                const std::string& type_url,
                                io::ZeroCopyInputStream* json_input,
                                io::ZeroCopyOutputStream* binary_output,
                                json_internal::ParseOptions options);
}  // namespace json_internal
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_JSON_INTERNAL_PARSER_H__
