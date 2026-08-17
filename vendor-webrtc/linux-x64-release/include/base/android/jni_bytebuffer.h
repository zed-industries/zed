// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ANDROID_JNI_BYTEBUFFER_H_
#define BASE_ANDROID_JNI_BYTEBUFFER_H_

#include <jni.h>

#include <optional>

#include "base/base_export.h"
#include "base/containers/span.h"

namespace base::android {

// Given a JNIEnv and a jobject representing a byte buffer, produce a base::span
// corresponding to that byte buffer. These crash at runtime if the passed-in
// jobject does not correspond to a java.nio.Buffer, or if the passed-in buffer
// is unaligned and the current CPU architecture may sometimes require aligned
// accesses - this requirement is enforced even if your code never actually
// *does* the types of accesses that require alignment.
//
// Usually, that is what you want, since both of those conditions are programmer
// errors.
//
// If needed, there are also variants below starting with Maybe that return
// std::nullopt in that case and do not crash.
base::span<const uint8_t> BASE_EXPORT JavaByteBufferToSpan(JNIEnv* env,
                                                           jobject buffer);

base::span<uint8_t> BASE_EXPORT JavaByteBufferToMutableSpan(JNIEnv* env,
                                                            jobject buffer);

std::optional<base::span<const uint8_t>> BASE_EXPORT
MaybeJavaByteBufferToSpan(JNIEnv* env, jobject buffer);

std::optional<base::span<uint8_t>> BASE_EXPORT
MaybeJavaByteBufferToMutableSpan(JNIEnv* env, jobject buffer);

}  // namespace base::android

#endif  // BASE_ANDROID_JNI_BYTEBUFFER_H_
