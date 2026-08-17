// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef JNI_ZERO_JNI_METHODS_H_
#define JNI_ZERO_JNI_METHODS_H_

#include <jni.h>

#include <atomic>
#include <string>

#include "third_party/jni_zero/java_refs.h"
#include "third_party/jni_zero/jni_export.h"

namespace jni_zero {
// Attaches the current thread to the VM (if necessary) and return the JNIEnv*.
JNI_ZERO_COMPONENT_BUILD_EXPORT JNIEnv* AttachCurrentThread();

// Same to AttachCurrentThread except that thread name will be set to
// |thread_name| if it is the first call. Otherwise, thread_name won't be
// changed. AttachCurrentThread() doesn't regard underlying platform thread
// name, but just resets it to "Thread-???". This function should be called
// right after new thread is created if it is important to keep thread name.
JNI_ZERO_COMPONENT_BUILD_EXPORT JNIEnv* AttachCurrentThreadWithName(
    const std::string& thread_name);

// Detaches the current thread from VM if it is attached.
JNI_ZERO_COMPONENT_BUILD_EXPORT void DetachFromVM();

// Initializes the global JVM.
JNI_ZERO_COMPONENT_BUILD_EXPORT void InitVM(JavaVM* vm);

// Returns true if the global JVM has been initialized.
JNI_ZERO_COMPONENT_BUILD_EXPORT bool IsVMInitialized();

// Returns the global JVM, or nullptr if it has not been initialized.
JNI_ZERO_COMPONENT_BUILD_EXPORT JavaVM* GetVM();

// Do not allow any future native->java calls.
// This is necessary in gtest DEATH_TESTS to prevent
// GetJavaStackTraceIfPresent() from accessing a defunct JVM (due to fork()).
// https://crbug.com/1484834
JNI_ZERO_COMPONENT_BUILD_EXPORT void DisableJvmForTesting();

JNI_ZERO_COMPONENT_BUILD_EXPORT void SetExceptionHandler(
    void (*callback)(JNIEnv*));

// Returns true if an exception is pending in the provided JNIEnv*.
JNI_ZERO_COMPONENT_BUILD_EXPORT bool HasException(JNIEnv* env);

// If an exception is pending in the provided JNIEnv*, this function clears it
// and returns true.
JNI_ZERO_COMPONENT_BUILD_EXPORT bool ClearException(JNIEnv* env);

// If there's any pending exception, this function will call the set exception
// handler, or if none are set, it will fatally LOG.
JNI_ZERO_COMPONENT_BUILD_EXPORT void CheckException(JNIEnv* env);

// Sets a function to call instead of just using JNIEnv.FindClass. Useful for
// chrome's "splits" which need to be resolved in special ClassLoaders. The
// class name parameter (first string) will be given in package.dot.Format. The
// second parameter is the split name, which will just be an empty string if not
// used.
JNI_ZERO_COMPONENT_BUILD_EXPORT void SetClassResolver(
    jclass (*resolver)(JNIEnv*, const char*, const char*));

// Finds the class named |class_name| and returns it.
// Use this method instead of invoking directly the JNI FindClass method (to
// prevent leaking local references).
// This method triggers a fatal assertion if the class could not be found.
// Use HasClass if you need to check whether the class exists.
JNI_ZERO_COMPONENT_BUILD_EXPORT ScopedJavaLocalRef<jclass>
GetClass(JNIEnv* env, const char* class_name, const char* split_name);
JNI_ZERO_COMPONENT_BUILD_EXPORT ScopedJavaLocalRef<jclass> GetClass(
    JNIEnv* env,
    const char* class_name);

// This class is a wrapper for JNIEnv Get(Static)MethodID.
class JNI_ZERO_COMPONENT_BUILD_EXPORT MethodID {
 public:
  enum Type {
    TYPE_STATIC,
    TYPE_INSTANCE,
  };

  // Returns the method ID for the method with the specified name and signature.
  // This method triggers a fatal assertion if the method could not be found.
  template <Type type>
  static jmethodID Get(JNIEnv* env,
                       jclass clazz,
                       const char* method_name,
                       const char* jni_signature);

  // The caller is responsible to zero-initialize |atomic_method_id|.
  // It's fine to simultaneously call this on multiple threads referencing the
  // same |atomic_method_id|.
  template <Type type>
  static jmethodID LazyGet(JNIEnv* env,
                           jclass clazz,
                           const char* method_name,
                           const char* jni_signature,
                           std::atomic<jmethodID>* atomic_method_id);
};
}  // namespace jni_zero

#endif  // JNI_ZERO_JNI_METHODS_H
