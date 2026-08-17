/*
 * Copyright (C) 2019 The Android Open Source Project
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

#ifndef INCLUDE_PERFETTO_BASE_STATUS_H_
#define INCLUDE_PERFETTO_BASE_STATUS_H_

#include <optional>
#include <string>
#include <string_view>
#include <vector>

#include "perfetto/base/compiler.h"
#include "perfetto/base/export.h"
#include "perfetto/base/logging.h"

namespace perfetto {
namespace base {

// Represents either the success or the failure message of a function.
// This can used as the return type of functions which would usually return an
// bool for success or int for errno but also wants to add some string context
// (ususally for logging).
//
// Similar to absl::Status, an optional "payload" can also be included with more
// context about the error. This allows passing additional metadata about the
// error (e.g. location of errors, potential mitigations etc).
class PERFETTO_EXPORT_COMPONENT Status {
 public:
  Status() : ok_(true) {}
  explicit Status(std::string msg) : ok_(false), message_(std::move(msg)) {
    PERFETTO_CHECK(!message_.empty());
  }

  // Copy operations.
  Status(const Status&) = default;
  Status& operator=(const Status&) = default;

  // Move operations. The moved-from state is valid but unspecified.
  Status(Status&&) noexcept = default;
  Status& operator=(Status&&) = default;

  bool ok() const { return ok_; }

  // When ok() is false this returns the error message. Returns the empty string
  // otherwise.
  const std::string& message() const { return message_; }
  const char* c_message() const { return message_.c_str(); }

  //////////////////////////////////////////////////////////////////////////////
  // Payload Management APIs
  //////////////////////////////////////////////////////////////////////////////

  // Payloads can be attached to error statuses to provide additional context.
  //
  // Payloads are (key, value) pairs, where the key is a string acting as a
  // unique "type URL" and the value is an opaque string. The "type URL" should
  // be unique, follow the format of a URL and, ideally, documentation on how to
  // interpret its associated data should be available.
  //
  // To attach a payload to a status object, call `Status::SetPayload()`.
  // Similarly, to extract the payload from a status, call
  // `Status::GetPayload()`.
  //
  // Note: the payload APIs are only meaningful to call when the status is an
  // error. Otherwise, all methods are noops.

  // Gets the payload for the given |type_url| if one exists.
  //
  // Will always return std::nullopt if |ok()|.
  std::optional<std::string_view> GetPayload(std::string_view type_url) const;

  // Sets the payload for the given key. The key should
  //
  // Will always do nothing if |ok()|.
  void SetPayload(std::string_view type_url, std::string value);

  // Erases the payload for the given string and returns true if the payload
  // existed and was erased.
  //
  // Will always do nothing if |ok()|.
  bool ErasePayload(std::string_view type_url);

 private:
  struct Payload {
    std::string type_url;
    std::string payload;
  };

  bool ok_ = false;
  std::string message_;
  std::vector<Payload> payloads_;
};

// Returns a status object which represents the Ok status.
inline Status OkStatus() {
  return Status();
}

Status ErrStatus(const char* format, ...) PERFETTO_PRINTF_FORMAT(1, 2);

}  // namespace base
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_BASE_STATUS_H_
