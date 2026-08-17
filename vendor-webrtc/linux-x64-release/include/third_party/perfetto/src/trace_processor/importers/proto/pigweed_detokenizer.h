/*
 * Copyright (C) 2024 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_PIGWEED_DETOKENIZER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_PIGWEED_DETOKENIZER_H_

#include <string_view>
#include <variant>

#include "perfetto/ext/base/flat_hash_map.h"
#include "perfetto/ext/base/status_or.h"
#include "perfetto/protozero/field.h"

namespace perfetto::trace_processor::pigweed {

// We only distinguish between the int types that we need to; we need
// to know different lengths for unsigned due to varint encoding.
// Strings are not supported.
enum ArgType { kSignedInt, kUnsigned32, kUnsigned64, kFloat };

// Representation of an arg in a formatting string: where it is,
// its contents, and its type.
struct Arg {
  ArgType type;
  std::string format;
  size_t begin;
  size_t end;
};

// A parsed format string from the database.
class FormatString {
 public:
  FormatString() = default;
  FormatString(std::string template_str);

  const std::string& template_str() const { return template_str_; }

  const std::vector<Arg>& args() const { return args_; }

 private:
  std::string template_str_;
  std::vector<Arg> args_;
};

// A string that we have detokenized, along with any information we gathered
// along the way.
class DetokenizedString {
 public:
  explicit DetokenizedString(const uint32_t token, FormatString str_template);

  explicit DetokenizedString(
      const uint32_t token,
      FormatString str_template,
      std::vector<std::variant<int64_t, uint64_t, double>> args,
      std::vector<std::string> args_formatted);

  // The fully formatted string.
  std::string Format() const;

  // The printf template used to format the string.
  const std::string& template_str() const {
    return format_string_.template_str();
  }

  // The ID of the template used to format the string.
  uint32_t token() const { return token_; }

  // Numerical args in the string, in order.
  const std::vector<std::variant<int64_t, uint64_t, double>>& args() const {
    return args_;
  }

 private:
  uint32_t token_;
  FormatString format_string_;
  // We don't bother holding 32 bit versions, just promote them.
  std::vector<std::variant<int64_t, uint64_t, double>> args_;
  std::vector<std::string> args_formatted_;
};

class PigweedDetokenizer {
 public:
  explicit PigweedDetokenizer(base::FlatHashMap<uint32_t, FormatString> tokens);
  base::StatusOr<DetokenizedString> Detokenize(
      const protozero::ConstBytes& bytes) const;

 private:
  base::FlatHashMap<uint32_t, FormatString> tokens_;
};

PigweedDetokenizer CreateNullDetokenizer();

base::StatusOr<PigweedDetokenizer> CreateDetokenizer(
    const protozero::ConstBytes& blob);

}  // namespace perfetto::trace_processor::pigweed

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_PIGWEED_DETOKENIZER_H_
