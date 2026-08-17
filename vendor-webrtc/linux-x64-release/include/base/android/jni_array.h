// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ANDROID_JNI_ARRAY_H_
#define BASE_ANDROID_JNI_ARRAY_H_

#include <jni.h>
#include <stddef.h>
#include <stdint.h>

#include <ostream>
#include <string>
#include <vector>

#include "base/android/scoped_java_ref.h"
#include "base/check_op.h"
#include "base/compiler_specific.h"
#include "base/containers/span.h"

namespace base::android {

// As |GetArrayLength| makes no guarantees about the returned value (e.g., it
// may be -1 if |array| is not a valid Java array), provide a safe wrapper
// that always returns a valid, non-negative size.
// Returns the length of Java array.
template <typename JavaArrayType>
BASE_EXPORT size_t SafeGetArrayLength(JNIEnv* env,
                                      const JavaRef<JavaArrayType>& jarray) {
  DCHECK(jarray);
  jsize length = env->GetArrayLength(jarray.obj());
  DCHECK_GE(length, 0) << "Invalid array length: " << length;
  return static_cast<size_t>(std::max(0, length));
}

// Returns a new Java byte array converted from the given bytes array.
UNSAFE_BUFFER_USAGE BASE_EXPORT ScopedJavaLocalRef<jbyteArray>
ToJavaByteArray(JNIEnv* env, const uint8_t* bytes, size_t len);

BASE_EXPORT ScopedJavaLocalRef<jbyteArray> ToJavaByteArray(
    JNIEnv* env,
    span<const uint8_t> bytes);

// Returns a new Java byte array converted from the given string. No UTF-8
// conversion is performed.
BASE_EXPORT ScopedJavaLocalRef<jbyteArray> ToJavaByteArray(
    JNIEnv* env,
    std::string_view str);

// Returns a new Java boolean array converted from the given bool array.
BASE_EXPORT ScopedJavaLocalRef<jbooleanArray> ToJavaBooleanArray(
    JNIEnv* env,
    span<const bool> bools);

// Returns a new Java boolean array converted from the given bool vector.
//
// std::vector<bool> does not convert to span, so we have a separate overload.
BASE_EXPORT ScopedJavaLocalRef<jbooleanArray> ToJavaBooleanArray(
    JNIEnv* env,
    const std::vector<bool>& bools);

// Returns a new Java int array converted from the given int array.
BASE_EXPORT ScopedJavaLocalRef<jintArray> ToJavaIntArray(
    JNIEnv* env,
    span<const int32_t> ints);

// Returns a new Java long array converted from the given int64_t array.
BASE_EXPORT ScopedJavaLocalRef<jlongArray> ToJavaLongArray(
    JNIEnv* env,
    span<const int64_t> longs);

// Returns a new Java float array converted from the given C++ float array.
BASE_EXPORT ScopedJavaLocalRef<jfloatArray> ToJavaFloatArray(
    JNIEnv* env,
    span<const float> floats);

// Returns a new Java double array converted from the given C++ double array.
BASE_EXPORT ScopedJavaLocalRef<jdoubleArray> ToJavaDoubleArray(
    JNIEnv* env,
    span<const double> doubles);

// Returns a new clazz[] with the content of |v|.
BASE_EXPORT ScopedJavaLocalRef<jobjectArray> ToJavaArrayOfObjects(
    JNIEnv* env,
    jclass clazz,
    span<const ScopedJavaLocalRef<jobject>> v);

// Returns a new Object[] with the content of |v|.
BASE_EXPORT ScopedJavaLocalRef<jobjectArray> ToJavaArrayOfObjects(
    JNIEnv* env,
    span<const ScopedJavaLocalRef<jobject>> v);
BASE_EXPORT ScopedJavaLocalRef<jobjectArray> ToJavaArrayOfObjects(
    JNIEnv* env,
    span<const ScopedJavaGlobalRef<jobject>> v);

// Returns a new Type[] with the content of |v|.
BASE_EXPORT ScopedJavaLocalRef<jobjectArray> ToTypedJavaArrayOfObjects(
    JNIEnv* env,
    span<const ScopedJavaLocalRef<jobject>> v,
    jclass type);
BASE_EXPORT ScopedJavaLocalRef<jobjectArray> ToTypedJavaArrayOfObjects(
    JNIEnv* env,
    span<const ScopedJavaGlobalRef<jobject>> v,
    jclass type);

// Returns a array of Java byte array converted from |v|.
BASE_EXPORT ScopedJavaLocalRef<jobjectArray> ToJavaArrayOfByteArray(
    JNIEnv* env,
    span<const std::string> v);

BASE_EXPORT ScopedJavaLocalRef<jobjectArray> ToJavaArrayOfByteArray(
    JNIEnv* env,
    span<const std::vector<uint8_t>> v);

BASE_EXPORT ScopedJavaLocalRef<jobjectArray> ToJavaArrayOfStrings(
    JNIEnv* env,
    span<const std::string> v);

BASE_EXPORT ScopedJavaLocalRef<jobjectArray> ToJavaArrayOfStrings(
    JNIEnv* env,
    span<const std::u16string> v);

BASE_EXPORT ScopedJavaLocalRef<jobjectArray> ToJavaArrayOfStringArray(
    JNIEnv* env,
    span<const std::vector<std::string>> v);

BASE_EXPORT ScopedJavaLocalRef<jobjectArray> ToJavaArrayOfStringArray(
    JNIEnv* env,
    span<const std::vector<std::u16string>> v);

// Converts a Java string array to a native array.
BASE_EXPORT void AppendJavaStringArrayToStringVector(
    JNIEnv* env,
    const JavaRef<jobjectArray>& array,
    std::vector<std::u16string>* out);

BASE_EXPORT void AppendJavaStringArrayToStringVector(
    JNIEnv* env,
    const JavaRef<jobjectArray>& array,
    std::vector<std::string>* out);

// Appends the Java bytes in |bytes_array| onto the end of |out|.
BASE_EXPORT void AppendJavaByteArrayToByteVector(
    JNIEnv* env,
    const JavaRef<jbyteArray>& byte_array,
    std::vector<uint8_t>* out);

// Replaces the content of |out| with the Java bytes in |byte_array|.
BASE_EXPORT void JavaByteArrayToByteVector(
    JNIEnv* env,
    const JavaRef<jbyteArray>& byte_array,
    std::vector<uint8_t>* out);

// Copy the contents of java |byte_array| into |dest|. The span must be larger
// than or equal to the array.
// Returns the number of bytes copied.
BASE_EXPORT size_t
JavaByteArrayToByteSpan(JNIEnv* env,
                        const JavaRef<jbyteArray>& byte_array,
                        span<uint8_t> dest);

// Replaces the content of |out| with the Java bytes in |byte_array|. No UTF-8
// conversion is performed.
BASE_EXPORT void JavaByteArrayToString(JNIEnv* env,
                                       const JavaRef<jbyteArray>& byte_array,
                                       std::string* out);

// Replaces the content of |out| with the Java booleans in |boolean_array|.
BASE_EXPORT void JavaBooleanArrayToBoolVector(
    JNIEnv* env,
    const JavaRef<jbooleanArray>& boolean_array,
    std::vector<bool>* out);

// Replaces the content of |out| with the Java ints in |int_array|.
BASE_EXPORT void JavaIntArrayToIntVector(JNIEnv* env,
                                         const JavaRef<jintArray>& int_array,
                                         std::vector<int>* out);

// Replaces the content of |out| with the Java longs in |long_array|.
BASE_EXPORT void JavaLongArrayToInt64Vector(
    JNIEnv* env,
    const JavaRef<jlongArray>& long_array,
    std::vector<int64_t>* out);

// Replaces the content of |out| with the Java longs in |long_array|.
BASE_EXPORT void JavaLongArrayToLongVector(
    JNIEnv* env,
    const JavaRef<jlongArray>& long_array,
    std::vector<jlong>* out);

// Replaces the content of |out| with the Java floats in |float_array|.
BASE_EXPORT void JavaFloatArrayToFloatVector(
    JNIEnv* env,
    const JavaRef<jfloatArray>& float_array,
    std::vector<float>* out);

// Replaces the content of |out| with the Java doubles in |double_array|.
BASE_EXPORT void JavaDoubleArrayToDoubleVector(
    JNIEnv* env,
    const JavaRef<jdoubleArray>& double_array,
    std::vector<double>* out);

// Assuming |array| is an byte[][] (array of byte arrays), replaces the
// content of |out| with the corresponding vector of strings. No UTF-8
// conversion is performed.
BASE_EXPORT void JavaArrayOfByteArrayToStringVector(
    JNIEnv* env,
    const JavaRef<jobjectArray>& array,
    std::vector<std::string>* out);

// Assuming |array| is an byte[][] (array of byte arrays), replaces the
// content of |out| with the corresponding vector of vector of uint8. No UTF-8
// conversion is performed.
BASE_EXPORT void JavaArrayOfByteArrayToBytesVector(
    JNIEnv* env,
    const JavaRef<jobjectArray>& array,
    std::vector<std::vector<uint8_t>>* out);

// Assuming |array| is an String[][] (array of String arrays), replaces the
// content of |out| with the corresponding vector of string vectors.
BASE_EXPORT void Java2dStringArrayTo2dStringVector(
    JNIEnv* env,
    const JavaRef<jobjectArray>& array,
    std::vector<std::vector<std::string>>* out);

// Assuming |array| is an String[][] (array of String arrays), replaces the
// content of |out| with the corresponding vector of string vectors. No UTF-8
// conversion is performed.
BASE_EXPORT void Java2dStringArrayTo2dStringVector(
    JNIEnv* env,
    const JavaRef<jobjectArray>& array,
    std::vector<std::vector<std::u16string>>* out);

// Assuming |array| is an int[][] (array of int arrays), replaces the
// contents of |out| with the corresponding vectors of ints.
BASE_EXPORT void JavaArrayOfIntArrayToIntVector(
    JNIEnv* env,
    const JavaRef<jobjectArray>& array,
    std::vector<std::vector<int>>* out);

}  // namespace base::android

#endif  // BASE_ANDROID_JNI_ARRAY_H_
