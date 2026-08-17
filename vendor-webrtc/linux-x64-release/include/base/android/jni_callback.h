// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ANDROID_JNI_CALLBACK_H_
#define BASE_ANDROID_JNI_CALLBACK_H_

#include <jni.h>

#include <type_traits>

#include "base/android/scoped_java_ref.h"
#include "base/base_export.h"
#include "base/functional/callback_forward.h"
#include "base/functional/callback_helpers.h"
#include "third_party/jni_zero/jni_zero.h"

namespace base::android {

using JniOnceWrappedCallbackType =
    base::OnceCallback<void(const jni_zero::JavaRef<jobject>&)>;
using JniRepeatingWrappedCallbackType =
    base::RepeatingCallback<void(const jni_zero::JavaRef<jobject>&)>;

BASE_EXPORT ScopedJavaLocalRef<jobject> ToJniCallback(
    JNIEnv* env,
    JniOnceWrappedCallbackType&& callback);
BASE_EXPORT ScopedJavaLocalRef<jobject> ToJniCallback(
    JNIEnv* env,
    JniRepeatingWrappedCallbackType&& callback);
BASE_EXPORT ScopedJavaLocalRef<jobject> ToJniCallback(
    JNIEnv* env,
    const JniRepeatingWrappedCallbackType& callback);

// Java Callbacks don't return a value so any return value by the passed in
// callback will be ignored.
template <typename R, typename Arg>
BASE_EXPORT ScopedJavaLocalRef<jobject> ToJniCallback(
    JNIEnv* env,
    base::OnceCallback<R(Arg)>&& callback) {
  return ToJniCallback(env, base::BindOnce(
                                [](base::OnceCallback<R(Arg)> captured_callback,
                                   const jni_zero::JavaRef<jobject>& j_result) {
                                  Arg result = jni_zero::FromJniType<Arg>(
                                      jni_zero::AttachCurrentThread(),
                                      j_result);
                                  std::move(captured_callback).Run(result);
                                },
                                std::move(callback)));
}

// Java Callbacks don't return a value so any return value by the passed in
// callback will be ignored.
template <typename R>
BASE_EXPORT ScopedJavaLocalRef<jobject> ToJniCallback(
    JNIEnv* env,
    base::OnceCallback<R()>&& callback) {
  return ToJniCallback(env, base::BindOnce(
                                [](base::OnceCallback<R()> captured_callback,
                                   const jni_zero::JavaRef<jobject>& j_result) {
                                  std::move(captured_callback).Run();
                                },
                                std::move(callback)));
}

// Java Callbacks don't return a value so any return value by the passed in
// callback will be ignored.
template <typename R, typename Arg>
BASE_EXPORT ScopedJavaLocalRef<jobject> ToJniCallback(
    JNIEnv* env,
    const base::RepeatingCallback<R(Arg)>& callback) {
  return ToJniCallback(
      env, base::BindRepeating(
               [](const base::RepeatingCallback<R(Arg)>& captured_callback,
                  const jni_zero::JavaRef<jobject>& j_result) {
                 Arg result = jni_zero::FromJniType<Arg>(
                     jni_zero::AttachCurrentThread(), j_result);
                 captured_callback.Run(result);
               },
               callback));
}

// Java Callbacks don't return a value so any return value by the passed in
// callback will be ignored.
template <typename R>
BASE_EXPORT ScopedJavaLocalRef<jobject> ToJniCallback(
    JNIEnv* env,
    const base::RepeatingCallback<R()>& callback) {
  return ToJniCallback(
      env, base::BindRepeating(
               [](const base::RepeatingCallback<R()>& captured_callback,
                  const jni_zero::JavaRef<jobject>& j_result) {
                 captured_callback.Run();
               },
               callback));
}
}  // namespace base::android

#endif  // BASE_ANDROID_JNI_CALLBACK_H_
