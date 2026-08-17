// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef JNI_ZERO_COMMON_APIS_H_
#define JNI_ZERO_COMMON_APIS_H_

#include <jni.h>

#include "third_party/jni_zero/java_refs.h"
#include "third_party/jni_zero/jni_export.h"

namespace jni_zero {
// Wraps Collection.toArray().
JNI_ZERO_COMPONENT_BUILD_EXPORT ScopedJavaLocalRef<jobjectArray>
CollectionToArray(JNIEnv* env, const JavaRef<jobject>& collection);
// Wraps Arrays.asList().
JNI_ZERO_COMPONENT_BUILD_EXPORT ScopedJavaLocalRef<jobject> ArrayToList(
    JNIEnv* env,
    const JavaRef<jobjectArray>& array);
// Serializes a Map to an array of key/values.
JNI_ZERO_COMPONENT_BUILD_EXPORT ScopedJavaLocalRef<jobjectArray> MapToArray(
    JNIEnv* env,
    const JavaRef<jobject>& map);
JNI_ZERO_COMPONENT_BUILD_EXPORT ScopedJavaLocalRef<jobject> ArrayToMap(
    JNIEnv* env,
    const JavaRef<jobjectArray>& array);

JNI_ZERO_COMPONENT_BUILD_EXPORT ScopedJavaLocalRef<jobject>
ListGet(JNIEnv* env, const JavaRef<jobject>& list, jint idx);

JNI_ZERO_COMPONENT_BUILD_EXPORT ScopedJavaLocalRef<jobject> ListSet(
    JNIEnv* env,
    const JavaRef<jobject>& list,
    jint idx,
    const JavaRef<jobject>& value);

JNI_ZERO_COMPONENT_BUILD_EXPORT void ListAdd(JNIEnv* env,
                                             const JavaRef<jobject>& list,
                                             const JavaRef<jobject>& value);

JNI_ZERO_COMPONENT_BUILD_EXPORT ScopedJavaLocalRef<jobject>
MapGet(JNIEnv* env, const JavaRef<jobject>& map, const JavaRef<jobject>& key);

JNI_ZERO_COMPONENT_BUILD_EXPORT ScopedJavaLocalRef<jobject> SetMapAt(
    JNIEnv* env,
    const JavaRef<jobject>& map,
    const JavaRef<jobject>& key,
    const JavaRef<jobject>& value);

JNI_ZERO_COMPONENT_BUILD_EXPORT jint
CollectionSize(JNIEnv* env, const JavaRef<jobject>& collection);

JNI_ZERO_COMPONENT_BUILD_EXPORT jint MapSize(JNIEnv* env,
                                             const JavaRef<jobject>& map);
JNI_ZERO_COMPONENT_BUILD_EXPORT bool FromJavaBoolean(
    JNIEnv* env,
    const JavaRef<jobject>& j_bool);

JNI_ZERO_COMPONENT_BUILD_EXPORT ScopedJavaLocalRef<jobject> ToJavaBoolean(
    JNIEnv* env,
    bool val);

JNI_ZERO_COMPONENT_BUILD_EXPORT int32_t
FromJavaInteger(JNIEnv* env, const JavaRef<jobject>& j_int);

JNI_ZERO_COMPONENT_BUILD_EXPORT ScopedJavaLocalRef<jobject> ToJavaInteger(
    JNIEnv* env,
    int32_t val);

JNI_ZERO_COMPONENT_BUILD_EXPORT int64_t
FromJavaLong(JNIEnv* env, const JavaRef<jobject>& j_long);

JNI_ZERO_COMPONENT_BUILD_EXPORT ScopedJavaLocalRef<jobject> ToJavaLong(
    JNIEnv* env,
    int64_t val);
}  // namespace jni_zero

#endif  // JNI_ZERO_COMMON_APIS_H_
