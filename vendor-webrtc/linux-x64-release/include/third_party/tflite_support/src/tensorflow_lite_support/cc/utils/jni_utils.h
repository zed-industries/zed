/* Copyright 2020 The TensorFlow Authors. All Rights Reserved.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
==============================================================================*/

#ifndef TENSORFLOW_LITE_SUPPORT_CC_UTILS_JNI_UTILS_H_
#define TENSORFLOW_LITE_SUPPORT_CC_UTILS_JNI_UTILS_H_

#include <jni.h>

#include <functional>
#include <string>
#include <vector>

#include "absl/status/status.h"  // from @com_google_absl
#include "absl/strings/string_view.h"  // from @com_google_absl
#include "tensorflow_lite_support/cc/port/configuration_proto_inc.h"
#include "tensorflow_lite_support/cc/port/statusor.h"

namespace tflite {
namespace support {
namespace utils {

const char kIllegalArgumentException[] = "java/lang/IllegalArgumentException";
const char kIllegalStateException[] = "java/lang/IllegalStateException";
const char kNullPointerException[] = "java/lang/NullPointerException";
const char kIndexOutOfBoundsException[] = "java/lang/IndexOutOfBoundsException";
const char kIOException[] = "java/io/IOException";
const char kRuntimeException[] = "java/lang/RuntimeException";
const char kUnsupportedOperationException[] =
    "java/lang/UnsupportedOperationException";
const char kAssertionError[] = "java/lang/AssertionError";

constexpr int kInvalidPointer = 0;

// Check if t is nullptr, throw IllegalStateException if it is.
// Used to verify different types of jobjects are correctly created from jni.
template <typename T>
T CheckNotNull(JNIEnv* env, T&& t) {
  if (t == nullptr) {
    env->ThrowNew(env->FindClass(kIllegalStateException), "");
    return nullptr;
  }
  return std::forward<T>(t);
}

// Converts an interable (specified by iterators, `begin` and `end`) into
// a Java ArrayList using a converter, which processes a single element in the
// interable before adding it to the ArrayList.
template <typename Iterator>
jobject ConvertVectorToArrayList(
    JNIEnv* env, const Iterator& begin, const Iterator& end,
    std::function<jobject(typename std::iterator_traits<Iterator>::value_type)>
        converter) {
  jclass array_list_class = env->FindClass("java/util/ArrayList");
  jmethodID array_list_ctor =
      env->GetMethodID(array_list_class, "<init>", "(I)V");
  jint initial_capacity = static_cast<jint>(std::distance(begin, end));
  jobject array_list_object =
      env->NewObject(array_list_class, array_list_ctor, initial_capacity);
  jmethodID array_list_add_method =
      env->GetMethodID(array_list_class, "add", "(Ljava/lang/Object;)Z");

  for (auto it = begin; it != end; ++it) {
    env->CallBooleanMethod(array_list_object, array_list_add_method,
                           converter(*it));
  }
  return array_list_object;
}

// Converts delegate Java int type to delegate proto type.
tflite::support::StatusOr<tflite::proto::Delegate> ConvertToProtoDelegate(
    jint delegate);

std::string JStringToString(JNIEnv* env, jstring jstr);

std::vector<std::string> StringListToVector(JNIEnv* env, jobject list_object);

// Gets a mapped file buffer from a java object representing a file.
absl::string_view GetMappedFileBuffer(JNIEnv* env, const jobject& file_buffer);

// Creates a Java byte array object based on the input data.
jbyteArray CreateByteArray(JNIEnv* env, const jbyte* data, int num_bytes);

void ThrowException(JNIEnv* env, const char* clazz, const char* fmt, ...);

void ThrowExceptionWithMessage(JNIEnv* env, const char* clazz,
                               const char* message);

const char* GetExceptionClassNameForStatusCode(absl::StatusCode status_code);

}  // namespace utils
}  // namespace support
}  // namespace tflite
#endif  // TENSORFLOW_LITE_SUPPORT_CC_UTILS_JNI_UTILS_H_
