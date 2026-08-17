// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TESTING_LIBFUZZER_PROTO_JSON_PROTO_CONVERTER_H_
#define TESTING_LIBFUZZER_PROTO_JSON_PROTO_CONVERTER_H_

#include <sstream>
#include <string>

#include "testing/libfuzzer/proto/json.pb.h"

namespace json_proto {

class JsonProtoConverter {
 public:
  std::string Convert(const JsonValue& json_value);
  std::string Convert(const json_proto::JsonObject&);
  std::string Convert(const json_proto::ArrayValue&);

 private:
  std::stringstream data_;

  void AppendArray(const json_proto::ArrayValue&);
  void AppendNumber(const json_proto::NumberValue&);
  void AppendObject(const json_proto::JsonObject&);
  void AppendValue(const json_proto::JsonValue&);
};

}  // namespace json_proto

#endif  // TESTING_LIBFUZZER_PROTO_JSON_PROTO_CONVERTER_H_
