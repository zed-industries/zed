// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef _JNI_ZERO_STUB_CONVERSIONS_H_
#define _JNI_ZERO_STUB_CONVERSIONS_H_

#include <cstdint>
#include <map>
#include <optional>
#include <string>
#include <vector>

#include "third_party/jni_zero/default_conversions.h"
#include "third_party/jni_zero/jni_zero.h"
#include "third_party/jni_zero/test/sample_for_tests.h"

// This file contains empty conversion functions needed by the compile tests.

#define EMPTY_TYPE_CONVERSIONS(T)                                          \
  template <>                                                              \
  T FromJniType<T>(JNIEnv * env, const JavaRef<jobject>& j_object) {       \
    return {};                                                             \
  }                                                                        \
  template <>                                                              \
  ScopedJavaLocalRef<jobject> ToJniType<T>(JNIEnv * env, const T& input) { \
    return nullptr;                                                        \
  }

#define EMPTY_PRIMITIVE_ARRAY_CONVERSIONS(T)             \
  template <>                                            \
  std::vector<T> FromJniArray<std::vector<T>>(           \
      JNIEnv * env, const JavaRef<jobject>& j_object) {  \
    return {};                                           \
  }                                                      \
  template <>                                            \
  ScopedJavaLocalRef<jarray> ToJniArray<std::vector<T>>( \
      JNIEnv * env, const std::vector<T>& vec) {         \
    return nullptr;                                      \
  }

#define EMPTY_OBJECT_ARRAY_CONVERSIONS(T)                      \
  template <>                                                  \
  std::vector<T> FromJniArray<std::vector<T>>(                 \
      JNIEnv * env, const JavaRef<jobject>& j_object) {        \
    return {};                                                 \
  }                                                            \
  template <>                                                  \
  ScopedJavaLocalRef<jobjectArray> ToJniArray<std::vector<T>>( \
      JNIEnv * env, const std::vector<T>& vec, jclass clazz) { \
    return nullptr;                                            \
  }

#define EMPTY_LIST_CONVERSIONS(T)                        \
  template <>                                            \
  ScopedJavaLocalRef<jobject> ToJniList<std::vector<T>>( \
      JNIEnv * env, const std::vector<T>& vec) {         \
    return nullptr;                                      \
  }

#define EMPTY_COLLECTION_CONVERSIONS(T)                 \
  template <>                                           \
  std::vector<T> FromJniCollection<std::vector<T>>(     \
      JNIEnv * env, const JavaRef<jobject>& j_object) { \
    return {};                                          \
  }

namespace jni_zero {

// These conversion functions are normally provided by the embedding app.
EMPTY_TYPE_CONVERSIONS(tests::CPPClass)
EMPTY_TYPE_CONVERSIONS(std::string)
EMPTY_TYPE_CONVERSIONS(std::u16string)
EMPTY_TYPE_CONVERSIONS(std::optional<std::string>)
template <>
ScopedJavaLocalRef<jobject> ToJniType<const char>(JNIEnv* env,
                                                  const char* input) {
  return {};
}

template <>
tests::CPPClass* FromJniType<tests::CPPClass*>(JNIEnv* env,
                                               const JavaRef<jobject>& j_obj) {
  return nullptr;
}

}  // namespace jni_zero
#endif
