// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Utility functions to convert between protobuf binary format and proto3 JSON
// format.
#ifndef GOOGLE_PROTOBUF_JSON_JSON_H__
#define GOOGLE_PROTOBUF_JSON_JSON_H__

#include <string>

#include "absl/status/status.h"
#include "absl/strings/string_view.h"
#include "google/protobuf/message.h"
#include "google/protobuf/util/type_resolver.h"

// Must be included last.
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {
namespace json {
struct ParseOptions {
  // Whether to ignore unknown JSON fields during parsing
  bool ignore_unknown_fields = false;

  // If true, when a lowercase enum value fails to parse, try convert it to
  // UPPER_CASE and see if it matches a valid enum.
  // WARNING: This option exists only to preserve legacy behavior. Avoid using
  // this option. If your enum needs to support different casing, consider using
  // allow_alias instead.
  bool case_insensitive_enum_parsing = false;
};

struct PrintOptions {
  // Whether to add spaces, line breaks and indentation to make the JSON output
  // easy to read.
  bool add_whitespace = false;
  // Whether to always print fields which do not support presence if they would
  // otherwise be omitted, namely:
  // - Implicit presence fields set to their 0 value
  // - Empty lists and maps
  bool always_print_fields_with_no_presence = false;
  // Whether to always print enums as ints. By default they are rendered as
  // strings.
  bool always_print_enums_as_ints = false;
  // Whether to preserve proto field names
  bool preserve_proto_field_names = false;
  // If set, int64 values that can be represented exactly as a double are
  // printed without quotes.
  bool unquote_int64_if_possible = false;
};

// Converts from protobuf message to JSON and appends it to |output|. This is a
// simple wrapper of BinaryToJsonString(). It will use the DescriptorPool of the
// passed-in message to resolve Any types.
//
// Please note that non-OK statuses are not a stable output of this API and
// subject to change without notice.
PROTOBUF_EXPORT absl::Status MessageToJsonString(const Message& message,
                                                 std::string* output,
                                                 const PrintOptions& options);

inline absl::Status MessageToJsonString(const Message& message,
                                        std::string* output) {
  return MessageToJsonString(message, output, PrintOptions());
}

// Converts from JSON string to protobuf message. This works equivalently to
// JsonToBinaryStream(). It will use the DescriptorPool of the passed-in
// message to resolve Any types.
//
// Please note that non-OK statuses are not a stable output of this API and
// subject to change without notice.
PROTOBUF_EXPORT absl::Status JsonStringToMessage(absl::string_view input,
                                                 Message* message,
                                                 const ParseOptions& options);

inline absl::Status JsonStringToMessage(absl::string_view input,
                                        Message* message) {
  return JsonStringToMessage(input, message, ParseOptions());
}

// Converts from JSON stream to protobuf message. Similar to JsonStringToMessage
// but with input stream.
//
// Please note that non-OK statuses are not a stable output of this API and
// subject to change without notice.
PROTOBUF_EXPORT absl::Status JsonStreamToMessage(io::ZeroCopyInputStream* input,
                                                 Message* message,
                                                 const ParseOptions& options);

inline absl::Status JsonStreamToMessage(io::ZeroCopyInputStream* input,
                                        Message* message) {
  return JsonStreamToMessage(input, message, ParseOptions());
}

// Converts protobuf binary data to JSON.
// The conversion will fail if:
//   1. TypeResolver fails to resolve a type.
//   2. input is not valid protobuf wire format, or conflicts with the type
//      information returned by TypeResolver.
// Note that unknown fields will be discarded silently.
//
// Please note that non-OK statuses are not a stable output of this API and
// subject to change without notice.
PROTOBUF_EXPORT absl::Status BinaryToJsonStream(
    google::protobuf::util::TypeResolver* resolver, const std::string& type_url,
    io::ZeroCopyInputStream* binary_input,
    io::ZeroCopyOutputStream* json_output, const PrintOptions& options);

inline absl::Status BinaryToJsonStream(google::protobuf::util::TypeResolver* resolver,
                                       const std::string& type_url,
                                       io::ZeroCopyInputStream* binary_input,
                                       io::ZeroCopyOutputStream* json_output) {
  return BinaryToJsonStream(resolver, type_url, binary_input, json_output,
                            PrintOptions());
}

PROTOBUF_EXPORT absl::Status BinaryToJsonString(
    google::protobuf::util::TypeResolver* resolver, const std::string& type_url,
    const std::string& binary_input, std::string* json_output,
    const PrintOptions& options);

inline absl::Status BinaryToJsonString(google::protobuf::util::TypeResolver* resolver,
                                       const std::string& type_url,
                                       const std::string& binary_input,
                                       std::string* json_output) {
  return BinaryToJsonString(resolver, type_url, binary_input, json_output,
                            PrintOptions());
}

// Converts JSON data to protobuf binary format.
// The conversion will fail if:
//   1. TypeResolver fails to resolve a type.
//   2. input is not valid JSON format, or conflicts with the type
//      information returned by TypeResolver.
//
// Please note that non-OK statuses are not a stable output of this API and
// subject to change without notice.
PROTOBUF_EXPORT absl::Status JsonToBinaryStream(
    google::protobuf::util::TypeResolver* resolver, const std::string& type_url,
    io::ZeroCopyInputStream* json_input,
    io::ZeroCopyOutputStream* binary_output, const ParseOptions& options);

inline absl::Status JsonToBinaryStream(
    google::protobuf::util::TypeResolver* resolver, const std::string& type_url,
    io::ZeroCopyInputStream* json_input,
    io::ZeroCopyOutputStream* binary_output) {
  return JsonToBinaryStream(resolver, type_url, json_input, binary_output,
                            ParseOptions());
}

PROTOBUF_EXPORT absl::Status JsonToBinaryString(
    google::protobuf::util::TypeResolver* resolver, const std::string& type_url,
    absl::string_view json_input, std::string* binary_output,
    const ParseOptions& options);

inline absl::Status JsonToBinaryString(google::protobuf::util::TypeResolver* resolver,
                                       const std::string& type_url,
                                       absl::string_view json_input,
                                       std::string* binary_output) {
  return JsonToBinaryString(resolver, type_url, json_input, binary_output,
                            ParseOptions());
}
}  // namespace json
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_JSON_JSON_H__
