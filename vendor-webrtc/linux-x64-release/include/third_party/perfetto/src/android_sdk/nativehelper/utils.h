/*
 * Copyright (C) 2023 The Android Open Source Project
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

/**
 * JNI utils for external use.
 *
 * This file may only be included by C++ code.
 */

#ifndef SRC_ANDROID_SDK_NATIVEHELPER_UTILS_H_
#define SRC_ANDROID_SDK_NATIVEHELPER_UTILS_H_

// Copied from
// https://cs.android.com/android/platform/superproject/main/+/main:libnativehelper/header_only_include/nativehelper/utils.h;drc=4be05051ef76b2c24d8385732a892401eb45d911

#include <jni.h>

#include <string>

#include "src/android_sdk/nativehelper/scoped_local_ref.h"
#include "src/android_sdk/nativehelper/scoped_utf_chars.h"

namespace android {
namespace jnihelp {

// Implementation details. DO NOT use directly.
namespace internal {

[[maybe_unused]] static const char* GetCStr(const char* str) {
  return str;
}
[[maybe_unused]] static const char* GetCStr(const std::string& str) {
  return str.c_str();
}

}  // namespace internal

// A class that implicitly casts to the default values of various JNI types.
// Used for returning from a JNI method when an exception occurs, where we don't
// care about the return value.
class JniDefaultValue {
 public:
  operator jboolean() const { return JNI_FALSE; }
  operator jbyte() const { return 0; }
  operator jchar() const { return 0; }
  operator jshort() const { return 0; }
  operator jint() const { return 0; }
  operator jlong() const { return 0; }
  operator jfloat() const { return 0; }
  operator jdouble() const { return 0; }
  operator jobject() const { return nullptr; }
  operator jclass() const { return nullptr; }
  operator jstring() const { return nullptr; }
  operator jarray() const { return nullptr; }
  operator jobjectArray() const { return nullptr; }
  operator jbooleanArray() const { return nullptr; }
  operator jbyteArray() const { return nullptr; }
  operator jcharArray() const { return nullptr; }
  operator jshortArray() const { return nullptr; }
  operator jintArray() const { return nullptr; }
  operator jlongArray() const { return nullptr; }
  operator jfloatArray() const { return nullptr; }
  operator jdoubleArray() const { return nullptr; }
  operator jthrowable() const { return nullptr; }
};

// Gets `ScopedUtfChars` from a `jstring` expression.
//
// Throws `NullPointerException` and returns the default value if the given
// `jstring` is a null pointer.
//
// Examples:
//
// - If the function returns a value:
//
// jobject MyJniMethod(JNIEnv* env, jstring j_str) {
//   ScopedUtfChars str = GET_UTF_OR_RETURN(env, j_str);
//   // Safely use `str` here...
// }
//
// - If the function returns void:
//
// void MyJniMethod(JNIEnv* env, jstring j_str) {
//   ScopedUtfChars str = GET_UTF_OR_RETURN_VOID(env, j_str);
//   // Safely use `str` here...
// }
//
// The idiomatic way to construct an `std::string` using this macro (an
// additional string copy is performed):
//
// jobject MyJniMethod(JNIEnv* env, jstring j_str) {
//   std::string str(GET_UTF_OR_RETURN(env, j_str));
//   // Safely use `str` here...
// }
#define GET_UTF_OR_RETURN(env, expr) \
  GET_UTF_OR_RETURN_IMPL_((env), (expr), android::jnihelp::JniDefaultValue())
#define GET_UTF_OR_RETURN_VOID(env, expr) GET_UTF_OR_RETURN_IMPL_((env), (expr))

#define GET_UTF_OR_RETURN_IMPL_(env, expr, ...)                    \
  ({                                                               \
    ScopedUtfChars __or_return_scoped_utf_chars(env, expr);        \
    if (__or_return_scoped_utf_chars.c_str() == nullptr) {         \
      /* Return with a pending exception from `ScopedUtfChars`. */ \
      return __VA_ARGS__;                                          \
    }                                                              \
    std::move(__or_return_scoped_utf_chars);                       \
  })

// Creates `ScopedLocalRef<jstring>` from a `const char*` or `std::string`
// expression using NewStringUTF.
//
// Throws `OutOfMemoryError` and returns the default value if the system runs
// out of memory.
//
// Examples:
//
// - If the function returns a value:
//
// jobject MyJniMethod(JNIEnv* env) {
//   std::string str = "foo";
//   ScopedLocalRef<jstring> j_str = CREATE_UTF_OR_RETURN(env, str);
//   // Safely use `j_str` here...
// }
//
// - If the function returns void:
//
// void MyJniMethod(JNIEnv* env) {
//   std::string str = "foo";
//   ScopedLocalRef<jstring> j_str = CREATE_UTF_OR_RETURN_VOID(env, str);
//   // Safely use `j_str` here...
// }
#define CREATE_UTF_OR_RETURN(env, expr) \
  CREATE_UTF_OR_RETURN_IMPL_((env), (expr), android::jnihelp::JniDefaultValue())
#define CREATE_UTF_OR_RETURN_VOID(env, expr) \
  CREATE_UTF_OR_RETURN_IMPL_((env), (expr))

#define CREATE_UTF_OR_RETURN_IMPL_(env, expr, ...)                             \
  ({                                                                           \
    const char* __or_return_c_str;                                             \
    ScopedLocalRef<jstring> __or_return_local_ref(                             \
        env,                                                                   \
        env->NewStringUTF(__or_return_c_str =                                  \
                              android::jnihelp::internal::GetCStr(expr)));     \
    /* `*__or_return_c_str` may be freed here, but we only compare the pointer \
     * against nullptr. DO NOT DEREFERENCE `*__or_return_c_str` after this     \
     * point. */                                                               \
    /* `NewStringUTF` returns nullptr when OOM or the input is nullptr, but    \
     * only throws an exception when OOM. */                                   \
    if (__or_return_local_ref == nullptr && __or_return_c_str != nullptr) {    \
      /* Return with a pending exception from `NewStringUTF`. */               \
      return __VA_ARGS__;                                                      \
    }                                                                          \
    std::move(__or_return_local_ref);                                          \
  })

}  // namespace jnihelp
}  // namespace android

#endif  // SRC_ANDROID_SDK_NATIVEHELPER_UTILS_H_
