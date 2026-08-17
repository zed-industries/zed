/*
 *  Copyright 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_RTC_ERROR_H_
#define API_RTC_ERROR_H_

#include <stdint.h>

#include <optional>
#include <string>
#include <type_traits>
#include <utility>  // For std::move.

#include "absl/strings/str_cat.h"
#include "absl/strings/string_view.h"
#include "rtc_base/checks.h"
#include "rtc_base/logging.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

// Enumeration to represent distinct classes of errors that an application
// may wish to act upon differently. These roughly map to DOMExceptions or
// RTCError "errorDetailEnum" values in the web API, as described in the
// comments below.
enum class RTCErrorType {
  // No error.
  NONE,

  // An operation is valid, but currently unsupported.
  // Maps to OperationError DOMException.
  UNSUPPORTED_OPERATION,

  // A supplied parameter is valid, but currently unsupported.
  // Maps to OperationError DOMException.
  UNSUPPORTED_PARAMETER,

  // General error indicating that a supplied parameter is invalid.
  // Maps to InvalidAccessError or TypeError DOMException depending on context.
  INVALID_PARAMETER,

  // Slightly more specific than INVALID_PARAMETER; a parameter's value was
  // outside the allowed range.
  // Maps to RangeError DOMException.
  INVALID_RANGE,

  // Slightly more specific than INVALID_PARAMETER; an error occurred while
  // parsing string input.
  // Maps to SyntaxError DOMException.
  SYNTAX_ERROR,

  // The object does not support this operation in its current state.
  // Maps to InvalidStateError DOMException.
  INVALID_STATE,

  // An attempt was made to modify the object in an invalid way.
  // Maps to InvalidModificationError DOMException.
  INVALID_MODIFICATION,

  // An error occurred within an underlying network protocol.
  // Maps to NetworkError DOMException.
  NETWORK_ERROR,

  // Some resource has been exhausted; file handles, hardware resources, ports,
  // etc.
  // Maps to OperationError DOMException.
  RESOURCE_EXHAUSTED,

  // The operation failed due to an internal error.
  // Maps to OperationError DOMException.
  INTERNAL_ERROR,

  // An error occured that has additional data.
  // The additional data is specified in
  // https://w3c.github.io/webrtc-pc/#rtcerror-interface
  // Maps to RTCError DOMException.
  OPERATION_ERROR_WITH_DATA,
};

// Detail information, showing what further information should be present.
// https://w3c.github.io/webrtc-pc/#rtcerrordetailtype-enum
enum class RTCErrorDetailType {
  NONE,
  DATA_CHANNEL_FAILURE,
  DTLS_FAILURE,
  FINGERPRINT_FAILURE,
  SCTP_FAILURE,
  SDP_SYNTAX_ERROR,
  HARDWARE_ENCODER_NOT_AVAILABLE,
  HARDWARE_ENCODER_ERROR,
};

// Outputs the error as a friendly string. Update this method when adding a new
// error type.
//
// Only intended to be used for logging/diagnostics. The returned char* points
// to literal strings that live for the whole duration of the program.
RTC_EXPORT absl::string_view ToString(RTCErrorType error);
RTC_EXPORT absl::string_view ToString(RTCErrorDetailType error);

template <typename Sink>
void AbslStringify(Sink& sink, RTCErrorType error) {
  sink.Append(ToString(error));
}

template <typename Sink>
void AbslStringify(Sink& sink, RTCErrorDetailType error_detail) {
  sink.Append(ToString(error_detail));
}

// Roughly corresponds to RTCError in the web api. Holds an error type, a
// message, and possibly additional information specific to that error.
//
// Doesn't contain anything beyond a type and message now, but will in the
// future as more errors are implemented.
class RTC_EXPORT RTCError {
 public:
  // Constructors.

  // Creates a "no error" error.
  RTCError() {}
  explicit RTCError(RTCErrorType type) : type_(type) {}

  RTCError(RTCErrorType type, absl::string_view message)
      : type_(type), message_(message) {}

  // In many use cases, it is better to use move than copy,
  // but copy and assignment are provided for those cases that need it.
  // Note that this has extra overhead because it copies strings.
  RTCError(const RTCError& other) = default;
  RTCError(RTCError&&) = default;
  RTCError& operator=(const RTCError& other) = default;
  RTCError& operator=(RTCError&&) = default;

  // Identical to default constructed error.
  //
  // Preferred over the default constructor for code readability.
  static RTCError OK();

  // Error type.
  RTCErrorType type() const { return type_; }
  void set_type(RTCErrorType type) { type_ = type; }

  // Human-readable message describing the error. Shouldn't be used for
  // anything but logging/diagnostics, since messages are not guaranteed to be
  // stable.
  const char* message() const;

  void set_message(absl::string_view message);

  RTCErrorDetailType error_detail() const { return error_detail_; }
  void set_error_detail(RTCErrorDetailType detail) { error_detail_ = detail; }
  std::optional<uint16_t> sctp_cause_code() const { return sctp_cause_code_; }
  void set_sctp_cause_code(uint16_t cause_code) {
    sctp_cause_code_ = cause_code;
  }

  // Convenience method for situations where you only care whether or not an
  // error occurred.
  bool ok() const { return type_ == RTCErrorType::NONE; }

  template <typename Sink>
  friend void AbslStringify(Sink& sink, const RTCError& error) {
    sink.Append(ToString(error.type_));
    if (!error.message_.empty()) {
      sink.Append(" with message: \"");
      sink.Append(error.message_);
      sink.Append("\"");
    }
  }

 private:
  RTCErrorType type_ = RTCErrorType::NONE;
  std::string message_;
  RTCErrorDetailType error_detail_ = RTCErrorDetailType::NONE;
  std::optional<uint16_t> sctp_cause_code_;
};

// Helper macro that can be used by implementations to create an error with a
// message and log it. `message` should be a string literal or movable
// std::string.
#define LOG_AND_RETURN_ERROR_EX(type, message, severity)                     \
  {                                                                          \
    RTC_DCHECK(type != RTCErrorType::NONE);                                  \
    RTC_LOG(severity) << message << " (" << ::webrtc::ToString(type) << ")"; \
    return ::webrtc::RTCError(type, message);                                \
  }

#define LOG_AND_RETURN_ERROR(type, message) \
  LOG_AND_RETURN_ERROR_EX(type, message, LS_ERROR)

// RTCErrorOr<T> is the union of an RTCError object and a T object. RTCErrorOr
// models the concept of an object that is either a usable value, or an error
// Status explaining why such a value is not present. To this end RTCErrorOr<T>
// does not allow its RTCErrorType value to be RTCErrorType::NONE. This is
// enforced by a debug check in most cases.
//
// The primary use-case for RTCErrorOr<T> is as the return value of a function
// which may fail. For example, CreateRtpSender will fail if the parameters
// could not be successfully applied at the media engine level, but if
// successful will return a unique_ptr to an RtpSender.
//
// Example client usage for a RTCErrorOr<std::unique_ptr<T>>:
//
//  RTCErrorOr<std::unique_ptr<Foo>> result = FooFactory::MakeNewFoo(arg);
//  if (result.ok()) {
//    std::unique_ptr<Foo> foo = result.ConsumeValue();
//    foo->DoSomethingCool();
//  } else {
//    RTC_LOG(LS_ERROR) << result.error();
//  }
//
// Example factory implementation returning RTCErrorOr<std::unique_ptr<T>>:
//
//  RTCErrorOr<std::unique_ptr<Foo>> FooFactory::MakeNewFoo(int arg) {
//    if (arg <= 0) {
//      return RTCError(RTCErrorType::INVALID_RANGE, "Arg must be positive");
//    } else {
//      return std::unique_ptr<Foo>(new Foo(arg));
//    }
//  }
//
template <typename T>
class RTCErrorOr {
  // Used to convert between RTCErrorOr<Foo>/RtcErrorOr<Bar>, when an implicit
  // conversion from Foo to Bar exists.
  template <typename U>
  friend class RTCErrorOr;

 public:
  typedef T element_type;

  // Constructs a new RTCErrorOr with RTCErrorType::INTERNAL_ERROR error. This
  // is marked 'explicit' to try to catch cases like 'return {};', where people
  // think RTCErrorOr<std::vector<int>> will be initialized with an empty
  // vector, instead of a RTCErrorType::INTERNAL_ERROR error.
  RTCErrorOr() : error_(RTCErrorType::INTERNAL_ERROR) {}

  // Constructs a new RTCErrorOr with the given non-ok error. After calling
  // this constructor, calls to value() will DCHECK-fail.
  //
  // NOTE: Not explicit - we want to use RTCErrorOr<T> as a return
  // value, so it is convenient and sensible to be able to do 'return
  // RTCError(...)' when the return type is RTCErrorOr<T>.
  //
  // REQUIRES: !error.ok(). This requirement is DCHECKed.
  RTCErrorOr(RTCError&& error) : error_(std::move(error)) {  // NOLINT
    RTC_DCHECK(!error_.ok());
  }

  // Constructs a new RTCErrorOr with the given value. After calling this
  // constructor, calls to value() will succeed, and calls to error() will
  // return a default-constructed RTCError.
  //
  // NOTE: Not explicit - we want to use RTCErrorOr<T> as a return type
  // so it is convenient and sensible to be able to do 'return T()'
  // when the return type is RTCErrorOr<T>.
  RTCErrorOr(const T& value) : value_(value) {}        // NOLINT
  RTCErrorOr(T&& value) : value_(std::move(value)) {}  // NOLINT

  // Delete the copy constructor and assignment operator; there aren't any use
  // cases where you should need to copy an RTCErrorOr, as opposed to moving
  // it. Can revisit this decision if use cases arise in the future.
  RTCErrorOr(const RTCErrorOr& other) = delete;
  RTCErrorOr& operator=(const RTCErrorOr& other) = delete;

  // Move constructor and move-assignment operator.
  //
  // Visual Studio doesn't support "= default" with move constructors or
  // assignment operators (even though they compile, they segfault), so define
  // them explicitly.
  RTCErrorOr(RTCErrorOr&& other)
      : error_(std::move(other.error_)), value_(std::move(other.value_)) {}
  RTCErrorOr& operator=(RTCErrorOr&& other) {
    error_ = std::move(other.error_);
    value_ = std::move(other.value_);
    return *this;
  }

  // Conversion constructor and assignment operator; T must be copy or move
  // constructible from U.
  template <typename U>
  RTCErrorOr(RTCErrorOr<U> other)  // NOLINT
      : error_(std::move(other.error_)), value_(std::move(other.value_)) {}
  template <typename U>
  RTCErrorOr& operator=(RTCErrorOr<U> other) {
    error_ = std::move(other.error_);
    value_ = std::move(other.value_);
    return *this;
  }

  // Returns a reference to our error. If this contains a T, then returns
  // default-constructed RTCError.
  const RTCError& error() const { return error_; }

  // Moves the error. Can be useful if, say "CreateFoo" returns an
  // RTCErrorOr<Foo>, and internally calls "CreateBar" which returns an
  // RTCErrorOr<Bar>, and wants to forward the error up the stack.
  RTCError MoveError() { return std::move(error_); }

  // Returns this->error().ok()
  bool ok() const { return error_.ok(); }

  // Returns a reference to our current value, or DCHECK-fails if !this->ok().
  //
  // Can be convenient for the implementation; for example, a method may want
  // to access the value in some way before returning it to the next method on
  // the stack.
  const T& value() const {
    RTC_DCHECK(ok());
    return *value_;
  }
  T& value() {
    RTC_DCHECK(ok());
    return *value_;
  }

  // Moves our current value out of this object and returns it, or DCHECK-fails
  // if !this->ok().
  T MoveValue() {
    RTC_DCHECK(ok());
    return std::move(*value_);
  }

  template <typename Sink>
  friend void AbslStringify(Sink& sink, const RTCErrorOr<T>& error_or) {
    if (error_or.ok()) {
      sink.Append("OK");
      if constexpr (std::is_convertible_v<T, absl::AlphaNum>) {
        sink.Append(" with value: ");
        sink.Append(absl::StrCat(error_or.value()));
      }
    } else {
      sink.Append(absl::StrCat(error_or.error()));
    }
  }

 private:
  RTCError error_;
  std::optional<T> value_;
};

}  // namespace webrtc

#endif  // API_RTC_ERROR_H_
