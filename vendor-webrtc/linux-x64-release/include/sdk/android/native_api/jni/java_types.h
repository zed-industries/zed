/*
 *  Copyright 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

// Android's FindClass() is tricky because the app-specific ClassLoader is not
// consulted when there is no app-specific frame on the stack (i.e. when called
// from a thread created from native C++ code). These helper functions provide a
// workaround for this.
// http://developer.android.com/training/articles/perf-jni.html#faq_FindClass

#ifndef SDK_ANDROID_NATIVE_API_JNI_JAVA_TYPES_H_
#define SDK_ANDROID_NATIVE_API_JNI_JAVA_TYPES_H_

#include <jni.h>

#include <map>
#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "api/array_view.h"
#include "api/sequence_checker.h"
#include "rtc_base/checks.h"
#include "sdk/android/native_api/jni/scoped_java_ref.h"

// Abort the process if `jni` has a Java exception pending.
// This macros uses the comma operator to execute ExceptionDescribe
// and ExceptionClear ignoring their return values and sending ""
// to the error stream.
#define CHECK_EXCEPTION(jni)        \
  RTC_CHECK(!jni->ExceptionCheck()) \
      << (jni->ExceptionDescribe(), jni->ExceptionClear(), "")

namespace webrtc {

// ---------------
// -- Utilities --
// ---------------

// Provides a convenient way to iterate over a Java Iterable using the
// C++ range-for loop.
// E.g. for (jobject value : Iterable(jni, j_iterable)) { ... }
// Note: Since Java iterators cannot be duplicated, the iterator class is not
// copyable to prevent creating multiple C++ iterators that refer to the same
// Java iterator.
class Iterable {
 public:
  Iterable(JNIEnv* jni, const jni_zero::JavaRef<jobject>& iterable);
  Iterable(Iterable&& other);

  ~Iterable();

  Iterable(const Iterable&) = delete;
  Iterable& operator=(const Iterable&) = delete;

  class Iterator {
   public:
    // Creates an iterator representing the end of any collection.
    Iterator();
    // Creates an iterator pointing to the beginning of the specified
    // collection.
    Iterator(JNIEnv* jni, const jni_zero::JavaRef<jobject>& iterable);

    // Move constructor - necessary to be able to return iterator types from
    // functions.
    Iterator(Iterator&& other);

    ~Iterator();

    Iterator(const Iterator&) = delete;
    Iterator& operator=(const Iterator&) = delete;

    // Move assignment should not be used.
    Iterator& operator=(Iterator&&) = delete;

    // Advances the iterator one step.
    Iterator& operator++();

    // Removes the element the iterator is pointing to. Must still advance the
    // iterator afterwards.
    void Remove();

    // Provides a way to compare the iterator with itself and with the end
    // iterator.
    // Note: all other comparison results are undefined, just like for C++ input
    // iterators.
    bool operator==(const Iterator& other);
    bool operator!=(const Iterator& other) { return !(*this == other); }
    jni_zero::ScopedJavaLocalRef<jobject>& operator*();

   private:
    bool AtEnd() const;

    JNIEnv* jni_ = nullptr;
    jni_zero::ScopedJavaLocalRef<jobject> iterator_;
    jni_zero::ScopedJavaLocalRef<jobject> value_;
    SequenceChecker thread_checker_;
  };

  Iterable::Iterator begin() { return Iterable::Iterator(jni_, iterable_); }
  Iterable::Iterator end() { return Iterable::Iterator(); }

 private:
  JNIEnv* jni_;
  jni_zero::ScopedJavaLocalRef<jobject> iterable_;
};

// Returns true if `obj` == null in Java.
bool IsNull(JNIEnv* jni, const jni_zero::JavaRef<jobject>& obj);

// Returns the name of a Java enum.
std::string GetJavaEnumName(JNIEnv* jni,
                            const jni_zero::JavaRef<jobject>& j_enum);

Iterable GetJavaMapEntrySet(JNIEnv* jni,
                            const jni_zero::JavaRef<jobject>& j_map);
ScopedJavaLocalRef<jobject> GetJavaMapEntryKey(
    JNIEnv* jni,
    const jni_zero::JavaRef<jobject>& j_entry);
ScopedJavaLocalRef<jobject> GetJavaMapEntryValue(
    JNIEnv* jni,
    const jni_zero::JavaRef<jobject>& j_entry);

// --------------------------------------------------------
// -- Methods for converting Java types to native types. --
// --------------------------------------------------------

int64_t JavaToNativeLong(JNIEnv* env, const jni_zero::JavaRef<jobject>& j_long);

std::optional<bool> JavaToNativeOptionalBool(
    JNIEnv* jni,
    const jni_zero::JavaRef<jobject>& boolean);
std::optional<double> JavaToNativeOptionalDouble(
    JNIEnv* jni,
    const jni_zero::JavaRef<jobject>& j_double);
std::optional<int32_t> JavaToNativeOptionalInt(
    JNIEnv* jni,
    const jni_zero::JavaRef<jobject>& integer);

// Given a (UTF-16) jstring return a new UTF-8 native string.
std::string JavaToNativeString(JNIEnv* jni,
                               const jni_zero::JavaRef<jstring>& j_string);

template <typename T, typename Convert>
std::vector<T> JavaToNativeVector(
    JNIEnv* env,
    const jni_zero::JavaRef<jobjectArray>& j_container,
    Convert convert) {
  std::vector<T> container;
  const size_t size = env->GetArrayLength(j_container.obj());
  container.reserve(size);
  for (size_t i = 0; i < size; ++i) {
    container.emplace_back(convert(
        env, jni_zero::ScopedJavaLocalRef<jobject>(
                 env, env->GetObjectArrayElement(j_container.obj(), i))));
  }
  CHECK_EXCEPTION(env) << "Error during JavaToNativeVector";
  return container;
}

template <typename T, typename Java_T = jobject, typename Convert>
std::vector<T> JavaListToNativeVector(JNIEnv* env,
                                      const jni_zero::JavaRef<jobject>& j_list,
                                      Convert convert) {
  std::vector<T> native_list;
  if (!j_list.is_null()) {
    for (ScopedJavaLocalRef<jobject>& j_item : Iterable(env, j_list)) {
      native_list.emplace_back(
          convert(env, static_java_ref_cast<Java_T>(env, j_item)));
    }
    CHECK_EXCEPTION(env) << "Error during JavaListToNativeVector";
  }
  return native_list;
}

template <typename Key, typename T, typename Convert>
std::map<Key, T> JavaToNativeMap(JNIEnv* env,
                                 const jni_zero::JavaRef<jobject>& j_map,
                                 Convert convert) {
  std::map<Key, T> container;
  for (auto const& j_entry : GetJavaMapEntrySet(env, j_map)) {
    container.emplace(convert(env, GetJavaMapEntryKey(env, j_entry),
                              GetJavaMapEntryValue(env, j_entry)));
  }
  return container;
}

// Converts Map<String, String> to std::map<std::string, std::string>.
std::map<std::string, std::string> JavaToNativeStringMap(
    JNIEnv* env,
    const jni_zero::JavaRef<jobject>& j_map);

// --------------------------------------------------------
// -- Methods for converting native types to Java types. --
// --------------------------------------------------------

ScopedJavaLocalRef<jobject> NativeToJavaBoolean(JNIEnv* env, bool b);
ScopedJavaLocalRef<jobject> NativeToJavaDouble(JNIEnv* env, double d);
ScopedJavaLocalRef<jobject> NativeToJavaInteger(JNIEnv* jni, int32_t i);
ScopedJavaLocalRef<jobject> NativeToJavaLong(JNIEnv* env, int64_t u);
ScopedJavaLocalRef<jstring> NativeToJavaString(JNIEnv* jni, const char* str);
ScopedJavaLocalRef<jstring> NativeToJavaString(JNIEnv* jni,
                                               const std::string& str);

ScopedJavaLocalRef<jobject> NativeToJavaDouble(
    JNIEnv* jni,
    const std::optional<double>& optional_double);
ScopedJavaLocalRef<jobject> NativeToJavaInteger(
    JNIEnv* jni,
    const std::optional<int32_t>& optional_int);
ScopedJavaLocalRef<jstring> NativeToJavaString(
    JNIEnv* jni,
    const std::optional<std::string>& str);

// Helper function for converting std::vector<T> into a Java array.
template <typename T, typename Convert>
ScopedJavaLocalRef<jobjectArray> NativeToJavaObjectArray(
    JNIEnv* env,
    const std::vector<T>& container,
    jclass clazz,
    Convert convert) {
  jni_zero::ScopedJavaLocalRef<jobjectArray> j_container(
      env, env->NewObjectArray(container.size(), clazz, nullptr));
  int i = 0;
  for (const T& element : container) {
    env->SetObjectArrayElement(j_container.obj(), i,
                               convert(env, element).obj());
    ++i;
  }
  return j_container;
}

ScopedJavaLocalRef<jbyteArray> NativeToJavaByteArray(
    JNIEnv* env,
    ArrayView<int8_t> container);
ScopedJavaLocalRef<jintArray> NativeToJavaIntArray(
    JNIEnv* env,
    ArrayView<int32_t> container);

std::vector<int8_t> JavaToNativeByteArray(
    JNIEnv* env,
    const jni_zero::JavaRef<jbyteArray>& jarray);
std::vector<int32_t> JavaToNativeIntArray(
    JNIEnv* env,
    const jni_zero::JavaRef<jintArray>& jarray);
std::vector<float> JavaToNativeFloatArray(
    JNIEnv* env,
    const jni_zero::JavaRef<jfloatArray>& jarray);

ScopedJavaLocalRef<jobjectArray> NativeToJavaBooleanArray(
    JNIEnv* env,
    const std::vector<bool>& container);
ScopedJavaLocalRef<jobjectArray> NativeToJavaDoubleArray(
    JNIEnv* env,
    const std::vector<double>& container);
ScopedJavaLocalRef<jobjectArray> NativeToJavaIntegerArray(
    JNIEnv* env,
    const std::vector<int32_t>& container);
ScopedJavaLocalRef<jobjectArray> NativeToJavaLongArray(
    JNIEnv* env,
    const std::vector<int64_t>& container);
ScopedJavaLocalRef<jobjectArray> NativeToJavaStringArray(
    JNIEnv* env,
    const std::vector<std::string>& container);

// This is a helper class for NativeToJavaList(). Use that function instead of
// using this class directly.
class JavaListBuilder {
 public:
  explicit JavaListBuilder(JNIEnv* env);
  ~JavaListBuilder();
  void add(const jni_zero::JavaRef<jobject>& element);
  jni_zero::ScopedJavaLocalRef<jobject> java_list() { return j_list_; }

 private:
  JNIEnv* env_;
  jni_zero::ScopedJavaLocalRef<jobject> j_list_;
};

template <typename C, typename Convert>
ScopedJavaLocalRef<jobject> NativeToJavaList(JNIEnv* env,
                                             const C& container,
                                             Convert convert) {
  JavaListBuilder builder(env);
  for (const auto& e : container)
    builder.add(convert(env, e));
  return builder.java_list();
}

// This is a helper class for NativeToJavaMap(). Use that function instead of
// using this class directly.
class JavaMapBuilder {
 public:
  explicit JavaMapBuilder(JNIEnv* env);
  ~JavaMapBuilder();
  void put(const jni_zero::JavaRef<jobject>& key,
           const jni_zero::JavaRef<jobject>& value);
  jni_zero::ScopedJavaLocalRef<jobject> GetJavaMap() { return j_map_; }

 private:
  JNIEnv* env_;
  jni_zero::ScopedJavaLocalRef<jobject> j_map_;
};

template <typename C, typename Convert>
ScopedJavaLocalRef<jobject> NativeToJavaMap(JNIEnv* env,
                                            const C& container,
                                            Convert convert) {
  JavaMapBuilder builder(env);
  for (const auto& e : container) {
    const auto key_value_pair = convert(env, e);
    builder.put(key_value_pair.first, key_value_pair.second);
  }
  return builder.GetJavaMap();
}

template <typename C>
ScopedJavaLocalRef<jobject> NativeToJavaStringMap(JNIEnv* env,
                                                  const C& container) {
  JavaMapBuilder builder(env);
  for (const auto& e : container) {
    const auto key_value_pair = std::make_pair(
        NativeToJavaString(env, e.first), NativeToJavaString(env, e.second));
    builder.put(key_value_pair.first, key_value_pair.second);
  }
  return builder.GetJavaMap();
}

// Return a `jlong` that will correctly convert back to `ptr`.  This is needed
// because the alternative (of silently passing a 32-bit pointer to a vararg
// function expecting a 64-bit param) picks up garbage in the high 32 bits.
jlong NativeToJavaPointer(const void* ptr);

// ------------------------
// -- Deprecated methods --
// ------------------------

// Deprecated. Use JavaToNativeString.
inline std::string JavaToStdString(JNIEnv* jni,
                                   const jni_zero::JavaRef<jstring>& j_string) {
  return JavaToNativeString(jni, j_string);
}

// Deprecated. Use scoped jobjects instead.
inline std::string JavaToStdString(JNIEnv* jni, jstring j_string) {
  return JavaToStdString(jni, jni_zero::JavaParamRef<jstring>(jni, j_string));
}

// Deprecated. Use JavaListToNativeVector<std::string, jstring> instead.
// Given a List of (UTF-16) jstrings
// return a new vector of UTF-8 native strings.
std::vector<std::string> JavaToStdVectorStrings(
    JNIEnv* jni,
    const jni_zero::JavaRef<jobject>& list);

// Deprecated. Use JavaToNativeStringMap instead.
// Parses Map<String, String> to std::map<std::string, std::string>.
inline std::map<std::string, std::string> JavaToStdMapStrings(
    JNIEnv* jni,
    const jni_zero::JavaRef<jobject>& j_map) {
  return JavaToNativeStringMap(jni, j_map);
}

// Deprecated. Use scoped jobjects instead.
inline std::map<std::string, std::string> JavaToStdMapStrings(JNIEnv* jni,
                                                              jobject j_map) {
  return JavaToStdMapStrings(jni, jni_zero::JavaParamRef<jobject>(jni, j_map));
}

}  // namespace webrtc

#endif  // SDK_ANDROID_NATIVE_API_JNI_JAVA_TYPES_H_
