// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef JNI_ZERO_TYPE_CONVERSIONS_H_
#define JNI_ZERO_TYPE_CONVERSIONS_H_

#include <jni.h>

#include <optional>
#include <type_traits>

#include "third_party/jni_zero/java_refs.h"

#define JNI_ZERO_ENABLE_TYPE_CONVERSIONS 1

namespace jni_zero {

#define JNI_ZERO_CONVERSION_FAILED_MSG(name)                               \
  "Failed to find a " name                                                 \
  " specialization for the given type. Did you forget to include the "     \
  "header file that declares it?\n"                                        \
  "If this error originates from a generated _jni.h file, make sure that " \
  "the header that declares the specialization is #included before the "   \
  "_jni.h one."

namespace internal {
template <typename T>
concept IsJavaRef = std::is_base_of_v<JavaRef<jobject>, T>;

template <typename T>
concept HasReserve = requires(T t) { t.reserve(0); };

template <typename T>
concept HasPushBack = requires(T t, T::value_type v) { t.push_back(v); };

template <typename T>
concept HasInsert = requires(T t, T::value_type v) { t.insert(v); };

template <typename T>
concept IsMap = requires(T t) {
  typename T::key_type;
  typename T::mapped_type;
};

template <typename T>
concept IsContainer = requires(T t) {
  requires !IsMap<T>;
  typename T::value_type;
  t.begin();
  t.end();
  t.size();
};

template <typename T>
concept IsObjectContainer =
    IsContainer<T> && !std::is_arithmetic_v<typename T::value_type>;

template <typename T>
concept IsOptional = !std::is_arithmetic_v<T> &&
                     std::same_as<T, std::optional<typename T::value_type>>;

template <typename T>
concept IsPrimitive = std::is_arithmetic<T>::value;

template <typename T>
concept HasSpecificSpecialization = requires(T t) {
  requires IsMap<T> || IsObjectContainer<T> || IsOptional<T> || IsPrimitive<T>;
};

// Used to allow for the c++ type to be non-primitive even if the java type is
// primitive, when doing type conversions. primitive<->primitive conversions use
// static_cast while From/ToJniType is used when the c++ type is non-primitive.
template <typename CppType, typename JavaType>
struct PrimitiveConvert {
  static constexpr CppType FromJniType(JNIEnv* env, JavaType v) {
    if constexpr (std::is_arithmetic_v<CppType> || std::is_enum_v<CppType>) {
      return static_cast<CppType>(v);
    } else {
      return FromJniType<CppType>(env, v);
    }
  }

  static constexpr JavaType ToJniType(JNIEnv* env, CppType v) {
    if constexpr (std::is_arithmetic_v<CppType> || std::is_enum_v<CppType>) {
      return static_cast<JavaType>(v);
    } else {
      return ToJniType<JavaType>(env, v);
    }
  }
};

}  // namespace internal

template <typename T>
inline T FromJniType(JNIEnv* env, const JavaRef<jobject>& obj) {
  static_assert(sizeof(T) == 0, JNI_ZERO_CONVERSION_FAILED_MSG("FromJniType"));
}

template <typename T>
  requires(!internal::HasSpecificSpecialization<T>)
inline ScopedJavaLocalRef<jobject> ToJniType(JNIEnv* env, const T& obj) {
  static_assert(sizeof(T) == 0, JNI_ZERO_CONVERSION_FAILED_MSG("ToJniType"));
}

template <typename T>
  requires internal::IsPrimitive<T>
inline ScopedJavaLocalRef<jobject> ToJniType(JNIEnv* env, T obj) {
  static_assert(sizeof(T) == 0, JNI_ZERO_CONVERSION_FAILED_MSG("ToJniType"));
}

// Allow conversions using pointers by wrapping non-pointer conversions.
// Cannot live in default_conversions.h because we want code to be able to
// specialize it.
template <typename T>
inline ScopedJavaLocalRef<jobject> ToJniType(JNIEnv* env, T* value) {
  if (!value) {
    return nullptr;
  }
  return ToJniType(env, *value);
}

#undef JNI_ZERO_CONVERSION_FAILED_MSG
#define JNI_ZERO_CONVERSION_FAILED_MSG(name)                             \
  "Failed to find a " name                                               \
  " specialization for the given type.\n"                                \
  "If this error is from a generated _jni.h file, ensure that the type " \
  "conforms to the container concepts defined in "                       \
  "jni_zero/type_conversions.h.\n"                                       \
  "If this error is from a non-generated call, ensure that there "       \
  "exists an #include for jni_zero/default_conversions.h."

// Convert from an stl container to a Java array. Uses ToJniType() on each
// element.
template <typename T>
inline ScopedJavaLocalRef<jobjectArray> ToJniArray(JNIEnv* env,
                                                   const T& obj,
                                                   jclass array_class) {
  static_assert(sizeof(T) == 0, JNI_ZERO_CONVERSION_FAILED_MSG("ToJniArray"));
}

// Convert from a Java array to an stl container of primitive types.
template <typename T>
inline ScopedJavaLocalRef<jarray> ToJniArray(JNIEnv* env, const T& obj) {
  static_assert(sizeof(T) == 0, JNI_ZERO_CONVERSION_FAILED_MSG("ToJniArray"));
}

// Convert from a Java array to an stl container. Uses FromJniType() on each
// element for non-primitive types.
template <typename T>
inline T FromJniArray(JNIEnv* env, const JavaRef<jobject>& obj) {
  static_assert(sizeof(T) == 0, JNI_ZERO_CONVERSION_FAILED_MSG("FromJniArray"));
}

// Convert from an stl container to a Java List<> by using ToJniType() on each
// element.
template <typename T>
inline ScopedJavaLocalRef<jobject> ToJniList(JNIEnv* env, const T& obj) {
  static_assert(sizeof(T) == 0, JNI_ZERO_CONVERSION_FAILED_MSG("ToJniList"));
}

// Convert from a Java Collection<> to an stl container by using FromJniType()
// on each element.
template <typename T>
inline T FromJniCollection(JNIEnv* env, const JavaRef<jobject>& obj) {
  static_assert(sizeof(T) == 0,
                JNI_ZERO_CONVERSION_FAILED_MSG("FromJniCollection"));
}
#undef JNI_ZERO_CONVERSION_FAILED_MSG

}  // namespace jni_zero

#endif  // JNI_ZERO_TYPE_CONVERSIONS_H_
