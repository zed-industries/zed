/*
 *  Copyright 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
// Do not include this file directly. It's intended to be used only by the JNI
// generation script. We are exporting types in strange namespaces in order to
// be compatible with the generated code targeted for Chromium.

#ifndef SDK_ANDROID_SRC_JNI_JNI_GENERATOR_HELPER_H_
#define SDK_ANDROID_SRC_JNI_JNI_GENERATOR_HELPER_H_

#include <jni.h>

#include <atomic>

#include "third_party/jni_zero/jni_zero_internal.h"

#define JNI_REGISTRATION_EXPORT __attribute__((visibility("default")))

#if defined(WEBRTC_ARCH_X86)
// Dalvik JIT generated code doesn't guarantee 16-byte stack alignment on
// x86 - use force_align_arg_pointer to realign the stack at the JNI
// boundary. crbug.com/655248
#define JNI_GENERATOR_EXPORT \
  __attribute__((force_align_arg_pointer)) extern "C" JNIEXPORT JNICALL
#else
#define JNI_GENERATOR_EXPORT extern "C" JNIEXPORT JNICALL
#endif

// Re-export helpers in the old jni_generator namespace.
// TODO(b/319078685): Remove once all uses of the jni_generator has been
// updated.
namespace jni_generator {
using jni_zero::internal::kJniStackMarkerValue;

// TODO(b/319078685): Remove JniJavaCallContextUnchecked once all uses of the
// jni_generator has been updated.
struct JniJavaCallContextUnchecked {
  inline JniJavaCallContextUnchecked() {
// TODO(ssid): Implement for other architectures.
#if defined(__arm__) || defined(__aarch64__)
    // This assumes that this method does not increment the stack pointer.
    asm volatile("mov %0, sp" : "=r"(sp));
#else
    sp = 0;
#endif
  }

  // Force no inline to reduce code size.
  template <jni_zero::MethodID::Type type>
  void Init(JNIEnv* env,
            jclass clazz,
            const char* method_name,
            const char* jni_signature,
            std::atomic<jmethodID>* atomic_method_id) {
    env1 = env;

    // Make sure compiler doesn't optimize out the assignment.
    memcpy(&marker, &kJniStackMarkerValue, sizeof(kJniStackMarkerValue));
    // Gets PC of the calling function.
    pc = reinterpret_cast<uintptr_t>(__builtin_return_address(0));

    method_id = jni_zero::MethodID::LazyGet<type>(
        env, clazz, method_name, jni_signature, atomic_method_id);
  }

  ~JniJavaCallContextUnchecked() {
    // Reset so that spurious marker finds are avoided.
    memset(&marker, 0, sizeof(marker));
  }

  uint64_t marker;
  uintptr_t sp;
  uintptr_t pc;

  JNIEnv* env1;
  jmethodID method_id;
};

// TODO(b/319078685): Remove JniJavaCallContextChecked once all uses of the
// jni_generator has been updated.
// Context about the JNI call with exception unchecked to be stored in stack.
struct JniJavaCallContextChecked {
  // Force no inline to reduce code size.
  template <jni_zero::MethodID::Type type>
  void Init(JNIEnv* env,
            jclass clazz,
            const char* method_name,
            const char* jni_signature,
            std::atomic<jmethodID>* atomic_method_id) {
    base.Init<type>(env, clazz, method_name, jni_signature, atomic_method_id);
    // Reset `pc` to correct caller.
    base.pc = reinterpret_cast<uintptr_t>(__builtin_return_address(0));
  }

  ~JniJavaCallContextChecked() { jni_zero::CheckException(base.env1); }

  JniJavaCallContextUnchecked base;
};

static_assert(sizeof(JniJavaCallContextChecked) ==
                  sizeof(JniJavaCallContextUnchecked),
              "Stack unwinder cannot work with structs of different sizes.");

}  // namespace jni_generator

#endif  // SDK_ANDROID_SRC_JNI_JNI_GENERATOR_HELPER_H_
