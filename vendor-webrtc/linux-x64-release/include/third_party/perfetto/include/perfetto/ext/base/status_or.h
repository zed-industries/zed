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

#ifndef INCLUDE_PERFETTO_EXT_BASE_STATUS_OR_H_
#define INCLUDE_PERFETTO_EXT_BASE_STATUS_OR_H_

#include <optional>

#include "perfetto/base/status.h"

namespace perfetto {
namespace base {

// Union of a object of type |T| with a |base::Status|. Useful for cases where
// a |T| indicates a successful result of an operation and |base::Status|
// represents an error.
//
// This class is modelled closely on absl::Status and should essentially 1:1
// match it's API.
template <typename T>
class StatusOr {
 public:
  // Matches naming of declarations in similar types e.g. std::optional,
  // std::variant.
  using value_type = T;

  // Intentionally implicit to allow idomatic usage (e.g. returning value/status
  // from base::StatusOr returning function).
  StatusOr(base::Status status) : StatusOr(std::move(status), std::nullopt) {
    if (status.ok()) {
      // Matches what Abseil's approach towards OkStatus being passed to
      // absl::StatusOr<T>.
      PERFETTO_FATAL("base::OkStatus passed to StatusOr: this is not allowd");
    }
  }
  StatusOr(T value) : StatusOr(base::OkStatus(), std::move(value)) {}

  bool ok() const { return status_.ok(); }
  const base::Status& status() const { return status_; }

  T& value() {
    PERFETTO_DCHECK(status_.ok());
    return *value_;
  }
  const T& value() const { return *value_; }

  T& operator*() { return value(); }
  const T& operator*() const { return value(); }

  T* operator->() { return &value(); }
  const T* operator->() const { return &value(); }

 private:
  StatusOr(base::Status status, std::optional<T> value)
      : status_(std::move(status)), value_(std::move(value)) {
    PERFETTO_DCHECK(!status_.ok() || value_.has_value());
  }

  base::Status status_;
  std::optional<T> value_;
};

// Deduction guide to make returning StatusOr less verbose.
template <typename T>
StatusOr(T) -> StatusOr<T>;

}  // namespace base
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_EXT_BASE_STATUS_OR_H_
