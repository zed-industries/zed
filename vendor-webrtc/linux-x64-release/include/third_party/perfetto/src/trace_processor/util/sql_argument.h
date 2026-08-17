/*
 * Copyright (C) 2022 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef SRC_TRACE_PROCESSOR_UTIL_SQL_ARGUMENT_H_
#define SRC_TRACE_PROCESSOR_UTIL_SQL_ARGUMENT_H_

#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "perfetto/base/logging.h"
#include "perfetto/base/status.h"
#include "perfetto/ext/base/string_view.h"
#include "perfetto/trace_processor/basic_types.h"
#include "src/trace_processor/containers/null_term_string_view.h"

namespace perfetto::trace_processor::sql_argument {

// Possible types which can be specified in SQL.
// This differs from SqlValue::Type by allowing specifying richer
// types (e.g. kBool, kInt, kUint and kLong all map to
// SqlValue::Type::kLong). This allows more accurate type checking
// and, when lots of values are stored, reduced memory usage.
enum class Type {
  kBool,
  kLong,
  kDouble,
  kString,

  // Deprecated types.
  // TODO(b/380259828): Remove.
  kInt,
  kUint,
  kProto,
  kFloat,
  kBytes,
};

// Represents the definition of an argument from SQL. See
// |ParseArgumentDefinitions| for more details.
class ArgumentDefinition {
 public:
  ArgumentDefinition(std::string dollar_name, Type type)
      : dollar_name_(std::move(dollar_name)), type_(type) {
    PERFETTO_DCHECK(!dollar_name_.empty() && dollar_name_[0] == '$');
  }

  NullTermStringView dollar_name() const {
    return NullTermStringView(dollar_name_);
  }

  NullTermStringView name() const {
    return NullTermStringView(dollar_name_.c_str() + 1,
                              dollar_name_.size() - 1);
  }

  Type type() const { return type_; }

  bool operator==(const ArgumentDefinition& other) const {
    return dollar_name_ == other.dollar_name_ && type_ == other.type_;
  }

 private:
  std::string dollar_name_;
  Type type_;
};

// Returns whether the given |name| is considered valid.
//
// Names are valid if they only contain alphanumeric characters or underscores.
bool IsValidName(base::StringView name);

// Parses a string containing a type from SQL and converts it to a Type enum
// value.
// Returns std::nullopt if |type| did not correspond to any of the enum values.
std::optional<Type> ParseType(base::StringView type);

// Converts an argument type to a string for printing (e.g. in error messages
// etc).
const char* TypeToHumanFriendlyString(sql_argument::Type type);

// Converts an argument type to the equivalent SqlValue::Type.
SqlValue::Type TypeToSqlValueType(sql_argument::Type type);

// Parses a string containing argument definitions from SQL and converts it into
// a typed list of ArgumentDefinition structs
//
// An argument definition is a variable name followed by a type. Variable names
// only contain alphanumeric characters or underscores. Types must be one of
// the types corresponding to the Type enum.
//
// The expected form of |args| is comma-separated list of argument definitions.
// For example: foo BYTES, bar PROTO, baz INT, foobar STRING
base::Status ParseArgumentDefinitions(const std::string& args,
                                      std::vector<ArgumentDefinition>& out);

// Serialises the given argument list into a string.
std::string SerializeArguments(const std::vector<ArgumentDefinition>& args);

}  // namespace perfetto::trace_processor::sql_argument

#endif  // SRC_TRACE_PROCESSOR_UTIL_SQL_ARGUMENT_H_
