/*
 * Copyright (C) 2010 The Android Open Source Project
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

#ifndef SRC_ANDROID_SDK_NATIVEHELPER_SCOPED_UTF_CHARS_H_
#define SRC_ANDROID_SDK_NATIVEHELPER_SCOPED_UTF_CHARS_H_

// Copied from
// https://cs.android.com/android/platform/superproject/main/+/main:libnativehelper/header_only_include/nativehelper/scoped_utf_chars.h;drc=4be05051ef76b2c24d8385732a892401eb45d911

#include <stddef.h>
#include <string.h>

#include <jni.h>

#include "src/android_sdk/nativehelper/nativehelper_utils.h"

// Protect this with __has_include to cope with `stl: "none"` users.
#if __has_include(<string_view>)
#include <string_view>
#endif

// A smart pointer that provides read-only access to a Java string's UTF chars.
// Unlike GetStringUTFChars, we throw NullPointerException rather than abort if
// passed a null jstring, and c_str will return nullptr.
// This makes the correct idiom very simple:
//
//   ScopedUtfChars name(env, java_name);
//   if (name.c_str() == nullptr) {
//     return nullptr;
//   }
//
// Also consider using `GET_UTF_OR_RETURN`, a shorthand for the 4 lines above.
class ScopedUtfChars {
 public:
  ScopedUtfChars(JNIEnv* env, jstring s) noexcept : env_(env), string_(s) {
    if (s == nullptr) {
      utf_chars_ = nullptr;
      jniThrowNullPointerException(env);
    } else {
      utf_chars_ = env->GetStringUTFChars(s, nullptr);
    }
  }

  ScopedUtfChars(ScopedUtfChars&& rhs) noexcept
      : env_(rhs.env_), string_(rhs.string_), utf_chars_(rhs.utf_chars_) {
    rhs.env_ = nullptr;
    rhs.string_ = nullptr;
    rhs.utf_chars_ = nullptr;
  }

  ~ScopedUtfChars() noexcept { release_string(); }

  ScopedUtfChars& operator=(ScopedUtfChars&& rhs) noexcept {
    if (this != &rhs) {
      // Delete the currently owned UTF chars.
      release_string();

      // Move the rhs ScopedUtfChars and zero it out.
      env_ = rhs.env_;
      string_ = rhs.string_;
      utf_chars_ = rhs.utf_chars_;
      rhs.env_ = nullptr;
      rhs.string_ = nullptr;
      rhs.utf_chars_ = nullptr;
    }
    return *this;
  }

  const char* c_str() const noexcept { return utf_chars_; }

  size_t size() const noexcept { return strlen(utf_chars_); }

  const char& operator[](size_t n) const noexcept { return utf_chars_[n]; }

#if __has_include(<string_view>)
  operator std::string_view() const noexcept { return utf_chars_; }
#endif  // SRC_ANDROID_SDK_NATIVEHELPER_SCOPED_UTF_CHARS_H_

 private:
  void release_string() noexcept {
    if (utf_chars_) {
      env_->ReleaseStringUTFChars(string_, utf_chars_);
    }
  }

  JNIEnv* env_;
  jstring string_;
  const char* utf_chars_;

  DISALLOW_COPY_AND_ASSIGN(ScopedUtfChars);
};

#endif  // SRC_ANDROID_SDK_NATIVEHELPER_SCOPED_UTF_CHARS_H_
