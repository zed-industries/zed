// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ANDROID_BINDER_BOX_H_
#define BASE_ANDROID_BINDER_BOX_H_

#include <jni.h>

#include <vector>

#include "base/android/binder.h"
#include "base/android/scoped_java_ref.h"
#include "base/base_export.h"

namespace base::android {

// Creates a new binder box containing `binders` and returns a Java reference to
// it. The Java reference (which itself is an android.os.IBinder) may be passed
// to another process and unpacked there by UnpackBinderBox().
//
// The point of this thing is to conveniently pass native binders through Java
// code (e.g. across Java AIDL) without actually taking Java references to them.
// This is desirable because by design AIBinder_toJavaBinder actually leaks
// IBinder references for an indeterminate period of time, which is unacceptable
// for native binder users who want deterministic control of their binder's
// refcounts.
BASE_EXPORT ScopedJavaLocalRef<jobject> PackBinderBox(
    JNIEnv* env,
    std::vector<BinderRef> binders);

// Retrieves a collection of binders stashed in a binder box.
BASE_EXPORT BinderStatusOr<std::vector<BinderRef>> UnpackBinderBox(
    JNIEnv* env,
    const JavaRef<jobject>& box);

}  // namespace base::android

#endif  // BASE_ANDROID_BINDER_BOX_H_
