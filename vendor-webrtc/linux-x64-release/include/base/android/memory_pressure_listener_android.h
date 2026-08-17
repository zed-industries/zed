// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ANDROID_MEMORY_PRESSURE_LISTENER_ANDROID_H_
#define BASE_ANDROID_MEMORY_PRESSURE_LISTENER_ANDROID_H_

#include "base/android/jni_android.h"

namespace base {
namespace android {

// Implements the C++ counter part of MemoryPressureListener.java
class BASE_EXPORT MemoryPressureListenerAndroid {
 public:
  static void Initialize(JNIEnv* env);

  MemoryPressureListenerAndroid(const MemoryPressureListenerAndroid&) = delete;
  MemoryPressureListenerAndroid& operator=(
      const MemoryPressureListenerAndroid&) = delete;

  // Called by JNI.
  static void OnMemoryPressure(int memory_pressure_type);
};

}  // namespace android
}  // namespace base

#endif  // BASE_ANDROID_MEMORY_PRESSURE_LISTENER_ANDROID_H_
