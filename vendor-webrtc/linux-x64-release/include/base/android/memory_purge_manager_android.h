// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ANDROID_MEMORY_PURGE_MANAGER_ANDROID_H_
#define BASE_ANDROID_MEMORY_PURGE_MANAGER_ANDROID_H_

#include "base/android/jni_android.h"

namespace base::android {

class BASE_EXPORT MemoryPurgeManagerAndroid {
 public:
  static void Initialize(JNIEnv* env);

  MemoryPurgeManagerAndroid(const MemoryPurgeManagerAndroid&) = delete;
  MemoryPurgeManagerAndroid& operator=(const MemoryPurgeManagerAndroid&) =
      delete;

  // Called by JNI
  static void PostDelayedPurgeTaskOnUiThread(int delay);

  // Called by JNI
  static bool IsOnPreFreezeMemoryTrimEnabled();
};

}  // namespace base::android

#endif  // BASE_ANDROID_MEMORY_PURGE_MANAGER_ANDROID_H_
