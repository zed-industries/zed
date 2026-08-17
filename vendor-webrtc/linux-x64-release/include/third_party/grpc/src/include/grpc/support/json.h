//
// Copyright 2015 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

#ifndef GRPC_SUPPORT_JSON_H
#define GRPC_SUPPORT_JSON_H

#include <grpc/support/port_platform.h>
#include <stdint.h>

#include <map>
#include <string>
#include <type_traits>
#include <utility>
#include <variant>
#include <vector>

#include "absl/strings/str_cat.h"

namespace grpc_core {
namespace experimental {

// A JSON value, which can be any one of null, boolean, number, string,
// object, or array.
class Json {
 public:
  // The JSON type.
  enum class Type {
    kNull,     // No payload.  Default type when using the zero-arg ctor.
    kBoolean,  // Use boolean() for payload.
    kNumber,   // Numbers are stored in string form to avoid precision
               // and integer capacity issues.  Use string() for payload.
    kString,   // Use string() for payload.
    kObject,   // Use object() for payload.
    kArray,    // Use array() for payload.
  };

  using Object = std::map<std::string, Json>;
  using Array = std::vector<Json>;

  // Factory method for kBoolean.
  static Json FromBool(bool b) {
    Json json;
    json.value_ = b;
    return json;
  }

  // Factory methods for kNumber.
  static Json FromNumber(const std::string& str) {
    Json json;
    json.value_ = NumberValue{str};
    return json;
  }
  static Json FromNumber(const char* str) {
    Json json;
    json.value_ = NumberValue{std::string(str)};
    return json;
  }
  static Json FromNumber(std::string&& str) {
    Json json;
    json.value_ = NumberValue{std::move(str)};
    return json;
  }
  template <typename T>
  static std::enable_if_t<std::is_arithmetic_v<T>, Json> FromNumber(T value) {
    Json json;
    json.value_ = NumberValue{absl::StrCat(value)};
    return json;
  }

  // Factory methods for kString.
  static Json FromString(const std::string& str) {
    Json json;
    json.value_ = str;
    return json;
  }
  static Json FromString(const char* str) {
    Json json;
    json.value_ = std::string(str);
    return json;
  }
  static Json FromString(std::string&& str) {
    Json json;
    json.value_ = std::move(str);
    return json;
  }

  // Factory methods for kObject.
  static Json FromObject(const Object& object) {
    Json json;
    json.value_ = object;
    return json;
  }
  static Json FromObject(Object&& object) {
    Json json;
    json.value_ = std::move(object);
    return json;
  }

  // Factory methods for kArray.
  static Json FromArray(const Array& array) {
    Json json;
    json.value_ = array;
    return json;
  }
  static Json FromArray(Array&& array) {
    Json json;
    json.value_ = std::move(array);
    return json;
  }

  Json() = default;

  // Copyable.
  Json(const Json& other) = default;
  Json& operator=(const Json& other) = default;

  // Moveable.
  Json(Json&& other) noexcept : value_(std::move(other.value_)) {
    other.value_ = std::monostate();
  }
  Json& operator=(Json&& other) noexcept {
    value_ = std::move(other.value_);
    other.value_ = std::monostate();
    return *this;
  }

  // Returns the JSON type.
  Type type() const {
    struct ValueFunctor {
      Json::Type operator()(const std::monostate&) { return Type::kNull; }
      Json::Type operator()(bool) { return Type::kBoolean; }
      Json::Type operator()(const NumberValue&) { return Type::kNumber; }
      Json::Type operator()(const std::string&) { return Type::kString; }
      Json::Type operator()(const Object&) { return Type::kObject; }
      Json::Type operator()(const Array&) { return Type::kArray; }
    };
    return std::visit(ValueFunctor(), value_);
  }

  // Payload accessor for kBoolean.
  // Must not be called for other types.
  bool boolean() const { return std::get<bool>(value_); }

  // Payload accessor for kNumber or kString.
  // Must not be called for other types.
  const std::string& string() const {
    const NumberValue* num = std::get_if<NumberValue>(&value_);
    if (num != nullptr) return num->value;
    return std::get<std::string>(value_);
  }

  // Payload accessor for kObject.
  // Must not be called for other types.
  const Object& object() const { return std::get<Object>(value_); }

  // Payload accessor for kArray.
  // Must not be called for other types.
  const Array& array() const { return std::get<Array>(value_); }

  bool operator==(const Json& other) const { return value_ == other.value_; }
  bool operator!=(const Json& other) const { return !(*this == other); }

 private:
  struct NumberValue {
    std::string value;

    bool operator==(const NumberValue& other) const {
      return value == other.value;
    }
  };
  using Value = std::variant<std::monostate,  // kNull
                             bool,            // kBoolean
                             NumberValue,     // kNumber
                             std::string,     // kString
                             Object,          // kObject
                             Array>;          // kArray

  explicit Json(Value value) : value_(std::move(value)) {}

  Value value_;
};

}  // namespace experimental
}  // namespace grpc_core

#endif  // GRPC_SUPPORT_JSON_H
