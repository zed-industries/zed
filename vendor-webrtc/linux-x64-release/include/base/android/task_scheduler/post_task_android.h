// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ANDROID_TASK_SCHEDULER_POST_TASK_ANDROID_H_
#define BASE_ANDROID_TASK_SCHEDULER_POST_TASK_ANDROID_H_

#include "base/android/jni_weak_ref.h"
#include "base/base_export.h"

namespace base {

// C++ interface for PostTask.java
class BASE_EXPORT PostTaskAndroid {
 public:
  PostTaskAndroid() = delete;
  PostTaskAndroid(const PostTaskAndroid&) = delete;
  PostTaskAndroid& operator=(const PostTaskAndroid&) = delete;

  // Routes tasks posted via the Java PostTask APIs through the C++ PostTask
  // APIs. Invoked once the C++ PostTask APIs are fully initialized.
  static void SignalNativeSchedulerReady();

  // Called to resets all the task runners for after each unit test.
  static void ResetTaskRunnerForTesting();
};

}  // namespace base

#endif  // BASE_ANDROID_TASK_SCHEDULER_POST_TASK_ANDROID_H_
