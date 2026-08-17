/*
 * Copyright (C) 2007 The Android Open Source Project
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

/** JNI utils for nativehelper-internal use. */

#ifndef SRC_ANDROID_SDK_NATIVEHELPER_NATIVEHELPER_UTILS_H_
#define SRC_ANDROID_SDK_NATIVEHELPER_NATIVEHELPER_UTILS_H_

// Copied from
// https://cs.android.com/android/platform/superproject/main/+/main:libnativehelper/header_only_include/nativehelper/nativehelper_utils.h;drc=4be05051ef76b2c24d8385732a892401eb45d911

#include <jni.h>

#if defined(__cplusplus)

#if !defined(DISALLOW_COPY_AND_ASSIGN)
// DISALLOW_COPY_AND_ASSIGN disallows the copy and operator= functions. It goes
// in the private: declarations in a class.
#define DISALLOW_COPY_AND_ASSIGN(TypeName) \
  TypeName(const TypeName&) = delete;      \
  void operator=(const TypeName&) = delete
#endif  // !defined(DISALLOW_COPY_AND_ASSIGN)

// This seems a header-only include. Provide NPE throwing.
static inline int jniThrowNullPointerException(JNIEnv* env) {
  if (env->ExceptionCheck()) {
    // Drop any pending exception.
    env->ExceptionClear();
  }

  jclass e_class = env->FindClass("java/lang/NullPointerException");
  if (e_class == nullptr) {
    return -1;
  }

  if (env->ThrowNew(e_class, nullptr) != JNI_OK) {
    env->DeleteLocalRef(e_class);
    return -1;
  }

  env->DeleteLocalRef(e_class);
  return 0;
}

#endif  // defined(__cplusplus)

#endif  // SRC_ANDROID_SDK_NATIVEHELPER_NATIVEHELPER_UTILS_H_
