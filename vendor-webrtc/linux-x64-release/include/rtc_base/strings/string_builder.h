/*
 *  Copyright 2018 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_STRINGS_STRING_BUILDER_H_
#define RTC_BASE_STRINGS_STRING_BUILDER_H_

#include <cstdio>
#include <string>
#include <utility>

#include "absl/strings/str_cat.h"
#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "rtc_base/string_encode.h"

namespace webrtc {

// This is a minimalistic string builder class meant to cover the most cases of
// when you might otherwise be tempted to use a stringstream (discouraged for
// anything except logging). It uses a fixed-size buffer provided by the caller
// and concatenates strings and numbers into it, allowing the results to be
// read via `str()`.
class SimpleStringBuilder {
 public:
  explicit SimpleStringBuilder(ArrayView<char> buffer);
  SimpleStringBuilder(const SimpleStringBuilder&) = delete;
  SimpleStringBuilder& operator=(const SimpleStringBuilder&) = delete;

  SimpleStringBuilder& operator<<(char ch);
  SimpleStringBuilder& operator<<(absl::string_view str);
  SimpleStringBuilder& operator<<(int i);
  SimpleStringBuilder& operator<<(unsigned i);
  SimpleStringBuilder& operator<<(long i);                // NOLINT
  SimpleStringBuilder& operator<<(long long i);           // NOLINT
  SimpleStringBuilder& operator<<(unsigned long i);       // NOLINT
  SimpleStringBuilder& operator<<(unsigned long long i);  // NOLINT
  SimpleStringBuilder& operator<<(float f);
  SimpleStringBuilder& operator<<(double f);
  SimpleStringBuilder& operator<<(long double f);

  // Returns a pointer to the built string. The name `str()` is borrowed for
  // compatibility reasons as we replace usage of stringstream throughout the
  // code base.
  const char* str() const { return buffer_.data(); }

  // Returns the length of the string. The name `size()` is picked for STL
  // compatibility reasons.
  size_t size() const { return size_; }

// Allows appending a printf style formatted string.
#if defined(__GNUC__)
  __attribute__((__format__(__printf__, 2, 3)))
#endif
  SimpleStringBuilder&
  AppendFormat(const char* fmt, ...);

 private:
  bool IsConsistent() const {
    return size_ <= buffer_.size() - 1 && buffer_[size_] == '\0';
  }

  // An always-zero-terminated fixed-size buffer that we write to. The fixed
  // size allows the buffer to be stack allocated, which helps performance.
  // Having a fixed size is furthermore useful to avoid unnecessary resizing
  // while building it.
  const ArrayView<char> buffer_;

  // Represents the number of characters written to the buffer.
  // This does not include the terminating '\0'.
  size_t size_ = 0;
};

// A string builder that supports dynamic resizing while building a string.
// The class is based around an instance of std::string and allows moving
// ownership out of the class once the string has been built.
// Note that this class uses the heap for allocations, so SimpleStringBuilder
// might be more efficient for some use cases.
class StringBuilder {
 public:
  StringBuilder() {}
  explicit StringBuilder(absl::string_view s) : str_(s) {}

  // TODO(tommi): Support construction from StringBuilder?
  StringBuilder(const StringBuilder&) = delete;
  StringBuilder& operator=(const StringBuilder&) = delete;

  StringBuilder& operator<<(const absl::string_view str) {
    str_.append(str.data(), str.length());
    return *this;
  }

  StringBuilder& operator<<(char c) = delete;

  StringBuilder& operator<<(int i) {
    str_ += absl::StrCat(i);
    return *this;
  }

  StringBuilder& operator<<(unsigned i) {
    str_ += absl::StrCat(i);
    return *this;
  }

  StringBuilder& operator<<(long i) {  // NOLINT
    str_ += absl::StrCat(i);
    return *this;
  }

  StringBuilder& operator<<(long long i) {  // NOLINT
    str_ += absl::StrCat(i);
    return *this;
  }

  StringBuilder& operator<<(unsigned long i) {  // NOLINT
    str_ += absl::StrCat(i);
    return *this;
  }

  StringBuilder& operator<<(unsigned long long i) {  // NOLINT
    str_ += absl::StrCat(i);
    return *this;
  }

  StringBuilder& operator<<(float f) {
    str_ += absl::StrCat(f);
    return *this;
  }

  StringBuilder& operator<<(double f) {
    str_ += absl::StrCat(f);
    return *this;
  }

  const std::string& str() const { return str_; }

  void Clear() { str_.clear(); }

  size_t size() const { return str_.size(); }

  std::string Release() {
    std::string ret = std::move(str_);
    str_.clear();
    return ret;
  }

  // Allows appending a printf style formatted string.
  StringBuilder& AppendFormat(const char* fmt, ...)
#if defined(__GNUC__)
      __attribute__((__format__(__printf__, 2, 3)))
#endif
      ;

 private:
  std::string str_;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::SimpleStringBuilder;
using ::webrtc::StringBuilder;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // RTC_BASE_STRINGS_STRING_BUILDER_H_
