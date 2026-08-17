// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_JSON_JSON_FILE_VALUE_SERIALIZER_H_
#define BASE_JSON_JSON_FILE_VALUE_SERIALIZER_H_

#include <stddef.h>

#include <memory>
#include <string>

#include "base/base_export.h"
#include "base/files/file_path.h"
#include "base/json/json_reader.h"
#include "base/values.h"

class BASE_EXPORT JSONFileValueSerializer : public base::ValueSerializer {
 public:
  JSONFileValueSerializer() = delete;

  // |json_file_path_| is the path of a file that will be destination of the
  // serialization. The serializer will attempt to create the file at the
  // specified location.
  explicit JSONFileValueSerializer(const base::FilePath& json_file_path);

  JSONFileValueSerializer(const JSONFileValueSerializer&) = delete;
  JSONFileValueSerializer& operator=(const JSONFileValueSerializer&) = delete;

  ~JSONFileValueSerializer() override;

  // DO NOT USE except in unit tests to verify the file was written properly.
  // We should never serialize directly to a file since this will block the
  // thread. Instead, serialize to a string and write to the file you want on
  // the thread pool.
  //
  // Attempt to serialize the data structure represented by Value into
  // JSON.  If the return value is true, the result will have been written
  // into the file whose name was passed into the constructor.
  bool Serialize(base::ValueView root) override;

  // Equivalent to Serialize(root) except binary values are omitted from the
  // output.
  bool SerializeAndOmitBinaryValues(base::ValueView root);

 private:
  bool SerializeInternal(base::ValueView root, bool omit_binary_values);

  const base::FilePath json_file_path_;
};

class BASE_EXPORT JSONFileValueDeserializer : public base::ValueDeserializer {
 public:
  JSONFileValueDeserializer() = delete;

  // |json_file_path_| is the path of a file that will be source of the
  // deserialization. |options| is a bitmask of JSONParserOptions.
  explicit JSONFileValueDeserializer(
      const base::FilePath& json_file_path,
      int options = base::JSON_PARSE_CHROMIUM_EXTENSIONS);

  JSONFileValueDeserializer(const JSONFileValueDeserializer&) = delete;
  JSONFileValueDeserializer& operator=(const JSONFileValueDeserializer&) =
      delete;

  ~JSONFileValueDeserializer() override;

  // Attempts to deserialize the data structure encoded in the file passed to
  // the constructor into a structure of Value objects. If the return value is
  // null, then
  // (1) |error_code| will be filled with an integer error code (either a
  //     JsonFileError or base::ValueDeserializer::kErrorCodeInvalidFormat) if a
  //     non-null |error_code| was given.
  // (2) |error_message| will be filled with a formatted error message,
  //     including the location of the error (if appropriate), if a non-null
  //     |error_message| was given.
  // The caller takes ownership of the returned value.
  std::unique_ptr<base::Value> Deserialize(int* error_code,
                                           std::string* error_message) override;

  // This enum is designed to safely overlap with JSONParser::JsonParseError.
  //
  // These values are persisted to logs. Entries should not be renumbered and
  // numeric values should never be reused.
  enum JsonFileError {
    JSON_NO_ERROR = 0,
    JSON_ACCESS_DENIED = kErrorCodeFirstMetadataError,
    JSON_CANNOT_READ_FILE,
    JSON_FILE_LOCKED,
    JSON_NO_SUCH_FILE
  };

  // File-specific error messages that can be returned.
  static const char kAccessDenied[];
  static const char kCannotReadFile[];
  static const char kFileLocked[];
  static const char kNoSuchFile[];

  // Convert an error code into an error message.  |error_code| is assumed to
  // be a JsonFileError.
  static const char* GetErrorMessageForCode(int error_code);

  // Returns the size (in bytes) of JSON string read from disk in the last
  // successful |Deserialize()| call.
  size_t get_last_read_size() const { return last_read_size_; }

 private:
  // A wrapper for ReadFileToString which returns a non-zero JsonFileError if
  // there were file errors.
  int ReadFileToString(std::string* json_string);

  const base::FilePath json_file_path_;
  const int options_;
  size_t last_read_size_ = 0u;
};

#endif  // BASE_JSON_JSON_FILE_VALUE_SERIALIZER_H_
