// Copyright 2023 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LIB_PROMISE_STATUS_FLAG_H
#define GRPC_SRC_CORE_LIB_PROMISE_STATUS_FLAG_H

#include <grpc/support/port_platform.h>

#include <optional>
#include <ostream>

#include "absl/log/check.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/str_join.h"
#include "src/core/lib/promise/detail/status.h"

namespace grpc_core {

struct Failure {
  template <typename Sink>
  friend void AbslStringify(Sink& sink, Failure) {
    sink.Append("failed");
  }
};
struct Success {
  template <typename Sink>
  friend void AbslStringify(Sink& sink, Success) {
    sink.Append("ok");
  }
};

GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION inline bool IsStatusOk(Failure) {
  return false;
}
GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION inline bool IsStatusOk(Success) {
  return true;
}

template <>
struct StatusCastImpl<absl::Status, Success> {
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static absl::Status Cast(Success) {
    return absl::OkStatus();
  }
};

template <>
struct StatusCastImpl<absl::Status, Success&> {
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static absl::Status Cast(Success) {
    return absl::OkStatus();
  }
};

template <>
struct StatusCastImpl<absl::Status, const Success&> {
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static absl::Status Cast(Success) {
    return absl::OkStatus();
  }
};

template <>
struct StatusCastImpl<absl::Status, Failure> {
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static absl::Status Cast(Failure) {
    return absl::CancelledError();
  }
};

template <typename T>
struct StatusCastImpl<absl::StatusOr<T>, Failure> {
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static absl::StatusOr<T> Cast(Failure) {
    return absl::CancelledError();
  }
};

// A boolean representing whether an operation succeeded (true) or failed
// (false).
class StatusFlag {
 public:
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION StatusFlag() : value_(true) {}
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION explicit StatusFlag(bool value)
      : value_(value) {}
  // NOLINTNEXTLINE(google-explicit-constructor)
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION StatusFlag(Failure) : value_(false) {}
  // NOLINTNEXTLINE(google-explicit-constructor)
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION StatusFlag(Success) : value_(true) {}

  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION bool ok() const { return value_; }

  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION bool operator==(StatusFlag other) const {
    return value_ == other.value_;
  }
  std::string ToString() const { return value_ ? "ok" : "failed"; }

  template <typename Sink>
  friend void AbslStringify(Sink& sink, StatusFlag flag) {
    if (flag.ok()) {
      sink.Append("ok");
    } else {
      sink.Append("failed");
    }
  }

 private:
  bool value_;
};

GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION inline bool operator==(StatusFlag flag,
                                                            Failure) {
  return !flag.ok();
}
GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION inline bool operator==(Failure,
                                                            StatusFlag flag) {
  return !flag.ok();
}
GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION inline bool operator==(StatusFlag flag,
                                                            Success) {
  return flag.ok();
}
GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION inline bool operator==(Success,
                                                            StatusFlag flag) {
  return flag.ok();
}

GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION inline bool operator!=(StatusFlag flag,
                                                            Failure) {
  return flag.ok();
}
GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION inline bool operator!=(Failure,
                                                            StatusFlag flag) {
  return flag.ok();
}
GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION inline bool operator!=(StatusFlag flag,
                                                            Success) {
  return !flag.ok();
}
GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION inline bool operator!=(Success,
                                                            StatusFlag flag) {
  return !flag.ok();
}

GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION inline bool IsStatusOk(
    const StatusFlag& flag) {
  return flag.ok();
}

template <>
struct StatusCastImpl<absl::Status, StatusFlag> {
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static absl::Status Cast(
      StatusFlag flag) {
    return flag.ok() ? absl::OkStatus() : absl::CancelledError();
  }
};

template <>
struct StatusCastImpl<absl::Status, StatusFlag&> {
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static absl::Status Cast(
      StatusFlag flag) {
    return flag.ok() ? absl::OkStatus() : absl::CancelledError();
  }
};

template <>
struct StatusCastImpl<absl::Status, const StatusFlag&> {
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static absl::Status Cast(
      StatusFlag flag) {
    return flag.ok() ? absl::OkStatus() : absl::CancelledError();
  }
};

template <>
struct StatusCastImpl<StatusFlag, Success> {
  static StatusFlag Cast(Success) { return StatusFlag(true); }
};

template <>
struct StatusCastImpl<StatusFlag, Failure> {
  static StatusFlag Cast(Failure) { return StatusFlag(false); }
};

template <>
struct FailureStatusCastImpl<StatusFlag, Failure> {
  static StatusFlag Cast(Failure) { return StatusFlag(false); }
};

template <typename T>
struct FailureStatusCastImpl<absl::StatusOr<T>, StatusFlag> {
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static absl::StatusOr<T> Cast(
      StatusFlag flag) {
    DCHECK(!flag.ok());
    return absl::CancelledError();
  }
};

template <typename T>
struct FailureStatusCastImpl<absl::StatusOr<T>, StatusFlag&> {
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static absl::StatusOr<T> Cast(
      StatusFlag flag) {
    DCHECK(!flag.ok());
    return absl::CancelledError();
  }
};

template <typename T>
struct FailureStatusCastImpl<absl::StatusOr<T>, const StatusFlag&> {
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static absl::StatusOr<T> Cast(
      StatusFlag flag) {
    DCHECK(!flag.ok());
    return absl::CancelledError();
  }
};

// A value if an operation was successful, or a failure flag if not.
template <typename T>
class ValueOrFailure {
 public:
  // NOLINTNEXTLINE(google-explicit-constructor)
  ValueOrFailure(T value) : value_(std::move(value)) {}
  // NOLINTNEXTLINE(google-explicit-constructor)
  ValueOrFailure(Failure) {}
  // NOLINTNEXTLINE(google-explicit-constructor)
  ValueOrFailure(StatusFlag status) { CHECK(!status.ok()); }

  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static ValueOrFailure FromOptional(
      std::optional<T> value) {
    return ValueOrFailure{std::move(value)};
  }

  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION bool ok() const {
    return value_.has_value();
  }
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION StatusFlag status() const {
    return StatusFlag(ok());
  }

  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION const T& value() const {
    return value_.value();
  }
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION T& value() { return value_.value(); }
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION const T& operator*() const {
    return *value_;
  }
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION T& operator*() { return *value_; }
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION const T* operator->() const {
    return &*value_;
  }
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION T* operator->() { return &*value_; }

  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION bool operator==(
      const ValueOrFailure& other) const {
    return value_ == other.value_;
  }

  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION bool operator!=(
      const ValueOrFailure& other) const {
    return value_ != other.value_;
  }

  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION bool operator==(const T& other) const {
    return value_ == other;
  }

  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION bool operator!=(const T& other) const {
    return value_ != other;
  }

 private:
  std::optional<T> value_;
};

template <typename T>
inline std::ostream& operator<<(std::ostream& os,
                                const ValueOrFailure<T>& value) {
  if (value.ok()) {
    return os << "Success(" << *value << ")";
  } else {
    return os << "Failure";
  }
}

template <typename Sink, typename T>
void AbslStringify(Sink& sink, const ValueOrFailure<T>& value) {
  if (value.ok()) {
    sink.Append("Success(");
    sink.Append(absl::StrCat(*value));
    sink.Append(")");
  } else {
    sink.Append("Failure");
  }
}

template <typename Sink, typename... Ts>
void AbslStringify(Sink& sink, const ValueOrFailure<std::tuple<Ts...>>& value) {
  if (value.ok()) {
    sink.Append("Success(");
    sink.Append(absl::StrCat("(", absl::StrJoin(*value, ", "), ")"));
    sink.Append(")");
  } else {
    sink.Append("Failure");
  }
}

template <typename T>
GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION inline bool IsStatusOk(
    const ValueOrFailure<T>& value) {
  return value.ok();
}

template <typename T>
GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION inline T TakeValue(
    ValueOrFailure<T>&& value) {
  return std::move(value.value());
}

template <typename T>
GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION inline T TakeValue(
    absl::StatusOr<T>&& value) {
  return std::move(*value);
}

template <typename T>
struct StatusCastImpl<absl::StatusOr<T>, ValueOrFailure<T>> {
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static absl::StatusOr<T> Cast(
      ValueOrFailure<T> value) {
    return value.ok() ? absl::StatusOr<T>(std::move(value.value()))
                      : absl::CancelledError();
  }
};

template <typename T>
struct StatusCastImpl<ValueOrFailure<T>, Failure> {
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static ValueOrFailure<T> Cast(Failure) {
    return ValueOrFailure<T>(Failure{});
  }
};

template <typename T>
struct StatusCastImpl<ValueOrFailure<T>, StatusFlag&> {
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static ValueOrFailure<T> Cast(
      StatusFlag f) {
    CHECK(!f.ok());
    return ValueOrFailure<T>(Failure{});
  }
};

template <typename T>
struct StatusCastImpl<ValueOrFailure<T>, StatusFlag> {
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static ValueOrFailure<T> Cast(
      StatusFlag f) {
    CHECK(!f.ok());
    return ValueOrFailure<T>(Failure{});
  }
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_PROMISE_STATUS_FLAG_H
