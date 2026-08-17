// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Utility functions to convert between protobuf binary format and proto3 JSON
// format.
#ifndef GOOGLE_PROTOBUF_UTIL_JSON_UTIL_H__
#define GOOGLE_PROTOBUF_UTIL_JSON_UTIL_H__

#include "absl/base/attributes.h"
#include "google/protobuf/json/json.h"

namespace google {
namespace protobuf {
namespace util {
using JsonParseOptions = ::google::protobuf::json::ParseOptions;
using JsonPrintOptions = ::google::protobuf::json::PrintOptions;

using ::google::protobuf::json::BinaryToJsonStream;
using ::google::protobuf::json::BinaryToJsonString;

using ::google::protobuf::json::JsonStringToMessage;
using ::google::protobuf::json::JsonToBinaryStream;

using ::google::protobuf::json::JsonToBinaryString;
using ::google::protobuf::json::MessageToJsonString;
}  // namespace util
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_UTIL_JSON_UTIL_H__
