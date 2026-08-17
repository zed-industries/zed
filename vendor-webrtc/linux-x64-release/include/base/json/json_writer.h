// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_JSON_JSON_WRITER_H_
#define BASE_JSON_JSON_WRITER_H_

#include <stddef.h>

#include <cstdint>
#include <optional>
#include <string>
#include <string_view>
#include <variant>

#include "base/base_export.h"
#include "base/json/json_common.h"
#include "base/memory/raw_ptr.h"
#include "base/values.h"

namespace base {

enum JsonOptions {
  // This option instructs the writer that if a Binary value is encountered,
  // the value (and key if within a dictionary) will be omitted from the
  // output, and success will be returned. Otherwise, if a binary value is
  // encountered, failure will be returned.
  OPTIONS_OMIT_BINARY_VALUES = 1 << 0,

  // This option instructs the writer to write doubles that have no fractional
  // part as a normal integer (i.e., without using exponential notation
  // or appending a '.0') as long as the value is within the range of a
  // 64-bit int.
  OPTIONS_OMIT_DOUBLE_TYPE_PRESERVATION = 1 << 1,

  // Return a slightly nicer formatted json string (pads with whitespace to
  // help with readability).
  OPTIONS_PRETTY_PRINT = 1 << 2,
};

// Given a root node, generates and returns a JSON string.
//
// Returns `std::nullopt` if
//    * the nesting depth exceeds `max_depth`, or
//    * the JSON contains binary values.
BASE_EXPORT std::optional<std::string> WriteJson(
    ValueView node,
    size_t max_depth = internal::kAbsoluteMaxDepth);

// Given a root node, generates and returns a JSON string.
// The string is formatted according to `options` which is a bitmask of
// `JsonOptions`.
//
// Returns `std::nullopt` if
//    * the nesting depth exceeds `max_depth,` or
//    * the JSON contains binary values
//      (unless `JsonOptions::OPTIONS_OMIT_BINARY_VALUES` is passed).
BASE_EXPORT std::optional<std::string> WriteJsonWithOptions(
    ValueView node,
    uint32_t options,
    size_t max_depth = internal::kAbsoluteMaxDepth);

class BASE_EXPORT JSONWriter {
 public:
  using enum JsonOptions;

  JSONWriter(const JSONWriter&) = delete;
  JSONWriter& operator=(const JSONWriter&) = delete;

  // Given a root node, generates a JSON string and puts it into |json|.
  // The output string is overwritten and not appended.
  //
  // TODO(tc): Should we generate json if it would be invalid json (e.g.,
  // |node| is not a dictionary/list Value or if there are inf/-inf float
  // values)? Return true on success and false on failure.
  //
  // Deprecated: use the standalone method `WriteJson()` instead.
  static bool Write(ValueView node,
                    std::string* json,
                    size_t max_depth = internal::kAbsoluteMaxDepth);

  // Same as above but with |options| which is a bunch of JSONWriter::Options
  // bitwise ORed together. Return true on success and false on failure.
  //
  // Deprecated: use the standalone method `WriteJsonWithOptions()` instead.
  static bool WriteWithOptions(ValueView node,
                               int options,
                               std::string* json,
                               size_t max_depth = internal::kAbsoluteMaxDepth);

 private:
  JSONWriter(int options,
             std::string* json,
             size_t max_depth = internal::kAbsoluteMaxDepth);

  // Called recursively to build the JSON string. When completed,
  // |json_string_| will contain the JSON.
  bool BuildJSONString(std::monostate node, size_t depth);
  bool BuildJSONString(bool node, size_t depth);
  bool BuildJSONString(int node, size_t depth);
  bool BuildJSONString(double node, size_t depth);
  bool BuildJSONString(std::string_view node, size_t depth);
  bool BuildJSONString(const Value::BlobStorage& node, size_t depth);
  bool BuildJSONString(const Value::Dict& node, size_t depth);
  bool BuildJSONString(const Value::List& node, size_t depth);

  // Adds space to json_string_ for the indent level.
  void IndentLine(size_t depth);

  bool omit_binary_values_;
  bool omit_double_type_preservation_;
  bool pretty_print_;

  // Where we write JSON data as we generate it.
  raw_ptr<std::string> json_string_;

  // Maximum depth to write.
  const size_t max_depth_;

  // The number of times the writer has recursed (current stack depth).
  size_t stack_depth_;
};

}  // namespace base

#endif  // BASE_JSON_JSON_WRITER_H_
