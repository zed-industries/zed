/*
 * Copyright (C) 2010 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef SRC_ANDROID_SDK_NATIVEHELPER_SCOPED_LOCAL_FRAME_H_
#define SRC_ANDROID_SDK_NATIVEHELPER_SCOPED_LOCAL_FRAME_H_

// Copied from
// https://cs.android.com/android/platform/superproject/main/+/main:libnativehelper/header_only_include/nativehelper/scoped_local_frame.h;drc=4be05051ef76b2c24d8385732a892401eb45d911

#include <jni.h>

#include "src/android_sdk/nativehelper/nativehelper_utils.h"

class ScopedLocalFrame {
 public:
  explicit ScopedLocalFrame(JNIEnv* env) : mEnv(env) {
    mEnv->PushLocalFrame(128);
  }

  ~ScopedLocalFrame() { mEnv->PopLocalFrame(nullptr); }

 private:
  JNIEnv* const mEnv;

  DISALLOW_COPY_AND_ASSIGN(ScopedLocalFrame);
};

#endif  // SRC_ANDROID_SDK_NATIVEHELPER_SCOPED_LOCAL_FRAME_H_
