// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_JSON_INTERNAL_UNPARSER_H__
#define GOOGLE_PROTOBUF_JSON_INTERNAL_UNPARSER_H__

#include <string>

#include "absl/strings/string_view.h"
#include "google/protobuf/json/internal/writer.h"
#include "google/protobuf/message.h"
#include "google/protobuf/util/type_resolver.h"

namespace google {
namespace protobuf {
namespace json_internal {
// Internal version of google::protobuf::util::MessageToJsonString; see json_util.h for
// details.
absl::Status MessageToJsonString(const Message& message, std::string* output,
                                 json_internal::WriterOptions options);
// Internal version of google::protobuf::util::BinaryToJsonStream; see json_util.h for
// details.
absl::Status BinaryToJsonStream(google::protobuf::util::TypeResolver* resolver,
                                const std::string& type_url,
                                io::ZeroCopyInputStream* binary_input,
                                io::ZeroCopyOutputStream* json_output,
                                json_internal::WriterOptions options);
}  // namespace json_internal
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_JSON_INTERNAL_UNPARSER_H__
