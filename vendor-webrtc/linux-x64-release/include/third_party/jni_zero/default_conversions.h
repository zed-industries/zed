// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef JNI_ZERO_DEFAULT_CONVERSIONS_H_
#define JNI_ZERO_DEFAULT_CONVERSIONS_H_

#include <optional>
#include <type_traits>
#include <vector>

#include "third_party/jni_zero/common_apis.h"
#include "third_party/jni_zero/jni_zero.h"

namespace jni_zero {

// Allow conversions using std::optional by wrapping non-optional conversions.
template <internal::IsOptional T>
inline T FromJniType(JNIEnv* env, const JavaRef<jobject>& j_object) {
  if (!j_object) {
    return std::nullopt;
  }
  return FromJniType<typename T::value_type>(env, j_object);
}

template <internal::IsOptional T>
inline ScopedJavaLocalRef<jobject> ToJniType(JNIEnv* env, const T& opt_value) {
  if (!opt_value) {
    return nullptr;
  }
  return ToJniType(env, opt_value.value());
}

// Convert Java array -> container type using FromJniType() on each element.
template <internal::IsObjectContainer ContainerType>
inline ContainerType FromJniArray(JNIEnv* env,
                                  const JavaRef<jobject>& j_object) {
  jobjectArray j_array = static_cast<jobjectArray>(j_object.obj());
  using ElementType = std::remove_const_t<typename ContainerType::value_type>;
  constexpr bool has_push_back = internal::HasPushBack<ContainerType>;
  constexpr bool has_insert = internal::HasInsert<ContainerType>;
  static_assert(has_push_back || has_insert, "Template type not supported.");
  jsize array_jsize = env->GetArrayLength(j_array);

  ContainerType ret;
  if constexpr (internal::HasReserve<ContainerType>) {
    size_t array_size = static_cast<size_t>(array_jsize);
    ret.reserve(array_size);
  }
  for (jsize i = 0; i < array_jsize; ++i) {
    jobject j_element = env->GetObjectArrayElement(j_array, i);
    // Do not call FromJni for jobject->jobject.
    if constexpr (std::is_base_of_v<JavaRef<jobject>, ElementType>) {
      if constexpr (has_push_back) {
        ret.emplace_back(env, j_element);
      } else if constexpr (has_insert) {
        ret.emplace(env, j_element);
      }
    } else {
      auto element = ScopedJavaLocalRef<jobject>::Adopt(env, j_element);
      if constexpr (has_push_back) {
        ret.push_back(FromJniType<ElementType>(env, element));
      } else if constexpr (has_insert) {
        ret.insert(FromJniType<ElementType>(env, element));
      }
    }
  }
  return ret;
}

// Convert container type -> Java array using ToJniType() on each element.
template <internal::IsObjectContainer ContainerType>
inline ScopedJavaLocalRef<jobjectArray>
ToJniArray(JNIEnv* env, const ContainerType& collection, jclass clazz) {
  using ElementType = std::remove_const_t<typename ContainerType::value_type>;
  size_t array_size = collection.size();
  jsize array_jsize = static_cast<jsize>(array_size);
  jobjectArray j_array = env->NewObjectArray(array_jsize, clazz, nullptr);
  CheckException(env);

  jsize i = 0;
  for (auto& value : collection) {
    // Do not call ToJni for jobject->jobject.
    if constexpr (std::is_base_of_v<JavaRef<jobject>, ElementType>) {
      env->SetObjectArrayElement(j_array, i, value.obj());
    } else {
      ScopedJavaLocalRef<jobject> element = ToJniType(env, value);
      env->SetObjectArrayElement(j_array, i, element.obj());
    }
    ++i;
  }
  return ScopedJavaLocalRef<jobjectArray>(env, j_array);
}

#define DECLARE_PRIMITIVE_ARRAY_CONVERSIONS(T)                                 \
  template <>                                                                  \
  JNI_ZERO_COMPONENT_BUILD_EXPORT std::vector<T> FromJniArray<std::vector<T>>( \
      JNIEnv * env, const JavaRef<jobject>& j_object);                         \
  template <>                                                                  \
  JNI_ZERO_COMPONENT_BUILD_EXPORT ScopedJavaLocalRef<jarray>                   \
  ToJniArray<std::vector<T>>(JNIEnv * env, const std::vector<T>& vec);

DECLARE_PRIMITIVE_ARRAY_CONVERSIONS(int64_t)
DECLARE_PRIMITIVE_ARRAY_CONVERSIONS(int32_t)
DECLARE_PRIMITIVE_ARRAY_CONVERSIONS(int16_t)
DECLARE_PRIMITIVE_ARRAY_CONVERSIONS(uint16_t)
DECLARE_PRIMITIVE_ARRAY_CONVERSIONS(uint8_t)
DECLARE_PRIMITIVE_ARRAY_CONVERSIONS(bool)
DECLARE_PRIMITIVE_ARRAY_CONVERSIONS(float)
DECLARE_PRIMITIVE_ARRAY_CONVERSIONS(double)

#undef DECLARE_PRIMITIVE_ARRAY_CONVERSIONS

// Specialization for ByteArrayView.
template <>
inline ByteArrayView FromJniArray<ByteArrayView>(
    JNIEnv* env,
    const JavaRef<jobject>& j_object) {
  jbyteArray j_array = static_cast<jbyteArray>(j_object.obj());
  return ByteArrayView(env, j_array);
}

template <internal::IsObjectContainer ContainerType>
inline ContainerType FromJniCollection(JNIEnv* env,
                                       const JavaRef<jobject>& j_collection) {
  ScopedJavaLocalRef<jobjectArray> arr = CollectionToArray(env, j_collection);
  return FromJniArray<ContainerType>(env, arr);
}

// Convert container type -> Java array using ToJniType() on each element.
template <internal::IsObjectContainer ContainerType>
inline ScopedJavaLocalRef<jobject> ToJniList(JNIEnv* env,
                                             const ContainerType& collection) {
  if (collection.empty()) {
    return g_empty_list.AsLocalRef(env);
  }
  ScopedJavaLocalRef<jobjectArray> arr =
      ToJniArray(env, collection, g_object_class);
  return ArrayToList(env, arr);
}

// Convert Map -> stl map type using FromJniType() on each key & value.
template <internal::IsMap ContainerType>
inline ContainerType FromJniType(JNIEnv* env,
                                 const JavaRef<jobject>& j_object) {
  using KeyType = ContainerType::key_type;
  using ValueType = ContainerType::mapped_type;

  ScopedJavaLocalRef<jobjectArray> j_array = MapToArray(env, j_object);
  jsize array_jsize = env->GetArrayLength(j_array.obj());

  ContainerType ret;
  if constexpr (internal::HasReserve<ContainerType>) {
    size_t array_size = static_cast<size_t>(array_jsize);
    ret.reserve(array_size / 2);
  }
  for (jsize i = 0; i < array_jsize; i += 2) {
    // No need to call CheckException() since we know the array is of the
    // correct size, since we are the ones who created it.
    jobject j_key = env->GetObjectArrayElement(j_array.obj(), i);
    jobject j_value = env->GetObjectArrayElement(j_array.obj(), i + 1);
    // Do not call FromJni for jobject->jobject.
    if constexpr (internal::IsJavaRef<KeyType> &&
                  internal::IsJavaRef<ValueType>) {
      ret.emplace(std::piecewise_construct, std::forward_as_tuple(env, j_key),
                  std::forward_as_tuple(env, j_value));
    } else if constexpr (internal::IsJavaRef<KeyType>) {
      auto value = ScopedJavaLocalRef<jobject>::Adopt(env, j_value);
      ret.emplace(std::piecewise_construct, std::forward_as_tuple(env, j_key),
                  FromJniType<ValueType>(env, value));
    } else if constexpr (internal::IsJavaRef<ValueType>) {
      auto key = ScopedJavaLocalRef<jobject>::Adopt(env, j_key);
      ret.emplace(std::piecewise_construct, FromJniType<KeyType>(env, key),
                  std::forward_as_tuple(env, j_value));
    } else {
      auto key = ScopedJavaLocalRef<jobject>::Adopt(env, j_key);
      auto value = ScopedJavaLocalRef<jobject>::Adopt(env, j_value);
      ret.emplace(FromJniType<KeyType>(env, key),
                  FromJniType<ValueType>(env, value));
    }
  }
  return ret;
}

// Convert stl map -> Map type using ToJniType() on each key & value.
template <internal::IsMap ContainerType>
inline ScopedJavaLocalRef<jobject> ToJniType(JNIEnv* env,
                                             const ContainerType& map) {
  using KeyType = ContainerType::key_type;
  using ValueType = ContainerType::mapped_type;
  jsize map_jsize = static_cast<jsize>(map.size());
  if (map_jsize == 0) {
    return g_empty_map.AsLocalRef(env);
  }
  jobjectArray j_array =
      env->NewObjectArray(map_jsize * 2, g_object_class, nullptr);
  CheckException(env);

  jsize i = 0;
  for (auto const& [key, value] : map) {
    // Do not call ToJni for jobject->jobject.
    if constexpr (internal::IsJavaRef<KeyType>) {
      env->SetObjectArrayElement(j_array, i, key.obj());
    } else {
      ScopedJavaLocalRef<jobject> j_key = ToJniType(env, key);
      env->SetObjectArrayElement(j_array, i, j_key.obj());
    }
    ++i;

    if constexpr (internal::IsJavaRef<ValueType>) {
      env->SetObjectArrayElement(j_array, i, value.obj());
    } else {
      ScopedJavaLocalRef<jobject> j_value = ToJniType(env, value);
      env->SetObjectArrayElement(j_array, i, j_value.obj());
    }
    ++i;
  }
  auto array = ScopedJavaLocalRef<jobjectArray>::Adopt(env, j_array);
  return ArrayToMap(env, array);
}

template <>
inline bool FromJniType<bool>(JNIEnv* env, const JavaRef<jobject>& j_bool) {
  return FromJavaBoolean(env, j_bool);
}

template <>
inline ScopedJavaLocalRef<jobject> ToJniType<bool>(JNIEnv* env, bool val) {
  return ToJavaBoolean(env, val);
}

template <>
inline int32_t FromJniType<int32_t>(JNIEnv* env,
                                    const JavaRef<jobject>& j_int) {
  return FromJavaInteger(env, j_int);
}

template <>
inline ScopedJavaLocalRef<jobject> ToJniType<int32_t>(JNIEnv* env,
                                                      int32_t val) {
  return ToJavaInteger(env, val);
}

template <>
inline int64_t FromJniType<int64_t>(JNIEnv* env,
                                    const JavaRef<jobject>& j_long) {
  return FromJavaLong(env, j_long);
}

template <>
inline ScopedJavaLocalRef<jobject> ToJniType<int64_t>(JNIEnv* env,
                                                      int64_t val) {
  return ToJavaLong(env, val);
}

}  // namespace jni_zero
#endif  // JNI_ZERO_DEFAULT_CONVERSIONS_H_
