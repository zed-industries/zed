// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef JNI_ZERO_BENCHMARK_H_
#define JNI_ZERO_BENCHMARK_H_

#include "third_party/jni_zero/jni_zero.h"

template <>
std::int32_t jni_zero::FromJniType<std::int32_t>(
    JNIEnv* env,
    const jni_zero::JavaRef<jobject>& j_integer);

template <>
jni_zero::ScopedJavaLocalRef<jobject> jni_zero::ToJniType<std::int32_t>(
    JNIEnv* env,
    const std::int32_t& input);

#endif  // JNI_ZERO_BENCHMARK_H_
