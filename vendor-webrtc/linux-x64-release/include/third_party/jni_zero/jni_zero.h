// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef JNI_ZERO_JNI_ZERO_H_
#define JNI_ZERO_JNI_ZERO_H_

#include <jni.h>

#include "third_party/jni_zero/java_refs.h"
#include "third_party/jni_zero/jni_export.h"
#include "third_party/jni_zero/jni_methods.h"
#include "third_party/jni_zero/jni_wrappers.h"
#include "third_party/jni_zero/logging.h"
#include "third_party/jni_zero/type_conversions.h"

namespace jni_zero {


// Commonly needed jclasses:
extern JNI_ZERO_COMPONENT_BUILD_EXPORT jclass g_object_class;
extern JNI_ZERO_COMPONENT_BUILD_EXPORT jclass g_string_class;
// Singletons for empty things.
extern JNI_ZERO_COMPONENT_BUILD_EXPORT LeakedJavaGlobalRef<jstring>
    g_empty_string;
extern JNI_ZERO_COMPONENT_BUILD_EXPORT LeakedJavaGlobalRef<jobject>
    g_empty_list;
extern JNI_ZERO_COMPONENT_BUILD_EXPORT LeakedJavaGlobalRef<jobject> g_empty_map;
}  // namespace jni_zero

#endif  // JNI_ZERO_JNI_ZERO_H_
