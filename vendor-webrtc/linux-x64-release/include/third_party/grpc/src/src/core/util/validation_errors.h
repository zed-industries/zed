// Copyright 2020 gRPC authors.
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

#ifndef GRPC_SRC_CORE_UTIL_VALIDATION_ERRORS_H
#define GRPC_SRC_CORE_UTIL_VALIDATION_ERRORS_H

#include <grpc/support/port_platform.h>
#include <stddef.h>

#include <map>
#include <string>
#include <utility>
#include <vector>

#include "absl/status/status.h"
#include "absl/strings/string_view.h"

namespace grpc_core {

// Tracks errors that occur during validation of a data structure (e.g.,
// a JSON object or protobuf message).  Errors are tracked based on
// which field they are associated with.  If at least one error occurs
// during validation, the validation failed.
//
// Example usage:
//
// absl::StatusOr<std::string> GetFooBar(const Json::Object& json) {
//   ValidationErrors errors;
//   {
//     ValidationErrors::ScopedField field("foo");
//     auto it = json.object().find("foo");
//     if (it == json.object().end()) {
//       errors.AddError("field not present");
//     } else if (it->second.type() != Json::Type::kObject) {
//       errors.AddError("must be a JSON object");
//     } else {
//       const Json& foo = it->second;
//       ValidationErrors::ScopedField field(".bar");
//       auto it = foo.object().find("bar");
//       if (it == json.object().end()) {
//         errors.AddError("field not present");
//       } else if (it->second.type() != Json::Type::kString) {
//         errors.AddError("must be a JSON string");
//       } else {
//         return it->second.string();
//       }
//     }
//   }
//   return errors.status(absl::StatusCode::kInvalidArgument,
//                        "errors validating foo.bar");
// }
class ValidationErrors {
 public:
  // Default maximum number of errors to track per scope.
  static constexpr size_t kMaxErrorCount = 20;

  // Pushes a field name onto the stack at construction and pops it off
  // of the stack at destruction.
  class ScopedField {
   public:
    ScopedField(ValidationErrors* errors, absl::string_view field_name)
        : errors_(errors) {
      errors_->PushField(field_name);
    }

    // Not copyable.
    ScopedField(const ScopedField& other) = delete;
    ScopedField& operator=(const ScopedField& other) = delete;

    // Movable.
    ScopedField(ScopedField&& other) noexcept
        : errors_(std::exchange(other.errors_, nullptr)) {}
    ScopedField& operator=(ScopedField&& other) noexcept {
      if (errors_ != nullptr) errors_->PopField();
      errors_ = std::exchange(other.errors_, nullptr);
      return *this;
    }

    ~ScopedField() {
      if (errors_ != nullptr) errors_->PopField();
    }

   private:
    ValidationErrors* errors_;
  };

  ValidationErrors() : ValidationErrors(kMaxErrorCount) {}

  // Creates a tracker that collects at most `max_error_count` errors per field.
  explicit ValidationErrors(size_t max_error_count)
      : max_error_count_(max_error_count) {}

  // Records that we've encountered an error associated with the current
  // field.
  void AddError(absl::string_view error) GPR_ATTRIBUTE_NOINLINE;

  // Returns true if the current field has errors.
  bool FieldHasErrors() const GPR_ATTRIBUTE_NOINLINE;

  // Returns the resulting status of parsing.
  // If there are no errors, this will return an Ok status instead of using the
  // prefix argument.
  absl::Status status(absl::StatusCode code, absl::string_view prefix) const;

  // Returns the resulting error message
  // If there are no errors, this will return an empty string.
  std::string message(absl::string_view prefix) const;

  // Returns true if there are no errors.
  bool ok() const { return field_errors_.empty(); }

  size_t size() const { return field_errors_.size(); }

 private:
  // Pushes a field name onto the stack.
  void PushField(absl::string_view ext) GPR_ATTRIBUTE_NOINLINE;
  // Pops a field name off of the stack.
  void PopField() GPR_ATTRIBUTE_NOINLINE;

  // Errors that we have encountered so far, keyed by field name.
  // TODO(roth): If we don't actually have any fields for which we
  // report more than one error, simplify this data structure.
  std::map<std::string /*field_name*/, std::vector<std::string>> field_errors_;
  // Stack of field names indicating the field that we are currently
  // validating.
  std::vector<std::string> fields_;

  size_t max_error_count_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_UTIL_VALIDATION_ERRORS_H
