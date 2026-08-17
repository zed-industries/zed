// Copyright 2020 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef MEDIAPIPE_UTIL_ANDROID_JNI_COMMON_LOGGING_H_
#define MEDIAPIPE_UTIL_ANDROID_JNI_COMMON_LOGGING_H_

#include <android/log.h>
#include <stdlib.h>

#define JNI_COMMON_LOG(priority, ...) \
  __android_log_print(ANDROID_LOG_##priority, __FILE__, __VA_ARGS__)

#define JNI_COMMON_CHECK(condition)                                        \
  if (!(condition)) {                                                      \
    JNI_COMMON_LOG(ERROR, "CHECK FAILED at %s:%d: %s", __FILE__, __LINE__, \
                   #condition);                                            \
    abort();                                                               \
  }

#define JNI_COMMON_CHECK_WITH_LOG(condition, message, ...)              \
  if (!(condition)) {                                                   \
    __android_log_print(ANDROID_LOG_ERROR, __FILE__,                    \
                        "CHECK FAILED at %s:%d: %s " message, __FILE__, \
                        __LINE__, #condition, ##__VA_ARGS__);           \
    abort();                                                            \
  }

#endif  // MEDIAPIPE_UTIL_ANDROID_JNI_COMMON_LOGGING_H_
