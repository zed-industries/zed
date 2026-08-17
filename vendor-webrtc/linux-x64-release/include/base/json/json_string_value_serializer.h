// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_JSON_JSON_STRING_VALUE_SERIALIZER_H_
#define BASE_JSON_JSON_STRING_VALUE_SERIALIZER_H_

#include <memory>
#include <string>
#include <string_view>

#include "base/base_export.h"
#include "base/json/json_reader.h"
#include "base/memory/raw_ptr.h"
#include "base/values.h"

class BASE_EXPORT JSONStringValueSerializer : public base::ValueSerializer {
 public:
  // |json_string| is the string that will be the destination of the
  // serialization.  The caller of the constructor retains ownership of the
  // string. |json_string| must not be null.
  explicit JSONStringValueSerializer(std::string* json_string);

  JSONStringValueSerializer(const JSONStringValueSerializer&) = delete;
  JSONStringValueSerializer& operator=(const JSONStringValueSerializer&) =
      delete;

  ~JSONStringValueSerializer() override;

  // Attempt to serialize the data structure represented by Value into
  // JSON.  If the return value is true, the result will have been written
  // into the string passed into the constructor.
  bool Serialize(base::ValueView root) override;

  // Equivalent to Serialize(root) except binary values are omitted from the
  // output.
  bool SerializeAndOmitBinaryValues(base::ValueView root);

  void set_pretty_print(bool new_value) { pretty_print_ = new_value; }
  bool pretty_print() { return pretty_print_; }

 private:
  bool SerializeInternal(base::ValueView root, bool omit_binary_values);

  // Owned by the caller of the constructor.
  raw_ptr<std::string> json_string_;
  bool pretty_print_;  // If true, serialization will span multiple lines.
};

class BASE_EXPORT JSONStringValueDeserializer : public base::ValueDeserializer {
 public:
  // This retains a reference to the contents of |json_string|, so the data
  // must outlive the JSONStringValueDeserializer. |options| is a bitmask of
  // JSONParserOptions.
  explicit JSONStringValueDeserializer(
      std::string_view json_string,
      int options = base::JSON_PARSE_CHROMIUM_EXTENSIONS);

  JSONStringValueDeserializer(const JSONStringValueDeserializer&) = delete;
  JSONStringValueDeserializer& operator=(const JSONStringValueDeserializer&) =
      delete;

  ~JSONStringValueDeserializer() override;

  // Attempts to deserialize |json_string_| into a structure of Value objects.
  // If the return value is null, then
  // (1) |error_code| will be filled with an integer error code
  //     (base::ValueDeserializer::kErrorCodeInvalidFormat) if a non-null
  //     |error_code| was given.
  // (2) |error_message| will be filled with a formatted error message,
  //     including the location of the error (if appropriate), if a non-null
  //     |error_message| was given.
  // The caller takes ownership of the returned value.
  std::unique_ptr<base::Value> Deserialize(int* error_code,
                                           std::string* error_message) override;

 private:
  // Data is owned by the caller of the constructor.
  std::string_view json_string_;
  const int options_;
};

#endif  // BASE_JSON_JSON_STRING_VALUE_SERIALIZER_H_
