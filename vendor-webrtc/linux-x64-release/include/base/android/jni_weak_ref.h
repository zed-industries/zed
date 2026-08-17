// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ANDROID_JNI_WEAK_REF_H_
#define BASE_ANDROID_JNI_WEAK_REF_H_

#include "base/android/scoped_java_ref.h"
#include "third_party/jni_zero/jni_zero.h"

using JavaObjectWeakGlobalRef = jni_zero::ScopedJavaGlobalWeakRef;

#endif  // BASE_ANDROID_JNI_WEAK_REF_H_
